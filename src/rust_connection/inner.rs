//! A pure-rust implementation of a connection to an X11 server.

use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::{ErrorKind, IoSlice, Read, Write};

use crate::connection::{DiscardMode, RequestKind, SequenceNumber};
use crate::errors::{ConnectError, ParseError};
use crate::x11_utils::{GenericEvent, Serialize};
use crate::xproto::{Setup, SetupRequest, GET_INPUT_FOCUS_REQUEST};

#[derive(Debug, Clone)]
pub(crate) enum PollReply {
    /// It is not clear yet what the result will be; try again.
    TryAgain,
    /// There will be no reply; polling is done.
    NoReply,
    /// Here is the result of the polling; polling is done.
    Reply(Vec<u8>),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct SentRequest {
    seqno: SequenceNumber,
    kind: RequestKind,
    discard_mode: Option<DiscardMode>,
}

#[derive(Debug)]
pub(crate) struct ConnectionInner<W>
where
    W: Write,
{
    // The underlying byte stream used for writing to the X11 server. Reading is done outside of
    // this struct (for synchronisation reasons).
    write: W,

    // The sequence number of the last request that was written
    last_sequence_written: SequenceNumber,
    // Sorted(!) list with information on requests that were written, but no answer received yet.
    sent_requests: VecDeque<SentRequest>,

    // The sequence number of the next reply that is expected to come in
    next_reply_expected: SequenceNumber,

    // The sequence number of the last reply/error/event that was read
    last_sequence_read: SequenceNumber,
    // Events that were read, but not yet returned to the API user
    pending_events: VecDeque<(SequenceNumber, Vec<u8>)>,
    // Replies that were read, but not yet returned to the API user
    pending_replies: VecDeque<(SequenceNumber, Vec<u8>)>,
}

impl<W> ConnectionInner<W>
where
    W: Write,
{
    /// Create a `ConnectionInner` for the given connection.
    ///
    /// This function sends a setup request to the X11 server and waits for the reply. The received
    /// `Setup` is returned to the caller, together with an instance of `ConnectionInner`.
    ///
    /// The `read` and `write` arguments describe the connection to the X11 server. `read` is only
    /// borrowed and will be managed by the caller after this function read. Ownership of `write`
    /// is transferred to the new `ConnectionInner` instance.
    ///
    /// `auth_name` and `auth_data` describe the authentication data that will be sent to the X11
    /// server in the `SetupRequest`.
    pub(crate) fn connect(
        read: &mut impl Read,
        mut write: W,
        auth_name: Vec<u8>,
        auth_data: Vec<u8>,
    ) -> Result<(Self, Setup), ConnectError> {
        Self::write_setup(&mut write, auth_name, auth_data)?;
        let setup = read_setup(read)?;
        Ok((Self::new(write), setup))
    }

    /// Crate a `ConnectionInner` wrapping the given write stream.
    ///
    /// It is assumed that the connection was just established. This means that the next request
    /// that is sent will have sequence number one.
    pub(crate) fn new(write: W) -> Self {
        ConnectionInner {
            write,
            last_sequence_written: 0,
            next_reply_expected: 0,
            last_sequence_read: 0,
            sent_requests: VecDeque::new(),
            pending_events: VecDeque::new(),
            pending_replies: VecDeque::new(),
        }
    }

    #[cfg(target_endian = "little")]
    fn byte_order() -> u8 {
        0x6c
    }

    #[cfg(target_endian = "big")]
    fn byte_order() -> u8 {
        0x42
    }

    /// Send a `SetupRequest` to the X11 server.
    fn write_setup(
        write: &mut W,
        auth_name: Vec<u8>,
        auth_data: Vec<u8>,
    ) -> Result<(), std::io::Error> {
        let request = SetupRequest {
            byte_order: Self::byte_order(),
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: auth_name,
            authorization_protocol_data: auth_data,
        };
        write.write_all(&request.serialize())?;
        write.flush()?;
        Ok(())
    }

    /// Send a synchronisation packet to the X11 server.
    ///
    /// This function sends a `GetInputFocus` request to the X11 server and arranges for its reply
    /// to be ignored. This causes `self.next_reply_expected` to be increased.
    fn send_sync(&mut self) -> Result<(), std::io::Error> {
        let length = 1u16.to_ne_bytes();
        self.write.write_all(&[
            GET_INPUT_FOCUS_REQUEST,
            0, /* pad */
            length[0],
            length[1],
        ])?;

        self.last_sequence_written += 1;
        self.next_reply_expected = self.last_sequence_written;
        self.sent_requests.push_back(SentRequest {
            seqno: self.last_sequence_written,
            kind: RequestKind::HasResponse,
            discard_mode: Some(DiscardMode::DiscardReplyAndError),
        });

        Ok(())
    }

    /// Send a request to the X11 server.
    pub(crate) fn send_request(
        &mut self,
        bufs: &[IoSlice<'_>],
        kind: RequestKind,
    ) -> Result<SequenceNumber, std::io::Error> {
        if self.next_reply_expected + SequenceNumber::from(u16::max_value())
            <= self.last_sequence_written
        {
            // Send a GetInputFocus request so that we can reliably reconstruct sequence numbers in
            // received packets.
            self.send_sync()?;
        }

        self.last_sequence_written += 1;
        let seqno = self.last_sequence_written;

        if kind == RequestKind::HasResponse {
            self.next_reply_expected = self.last_sequence_written;
        }

        let sent_request = SentRequest {
            seqno,
            kind,
            discard_mode: None,
        };
        self.sent_requests.push_back(sent_request);

        // Now actually send the buffers
        // FIXME: We must always be able to read when we write
        let mut bufs = bufs;
        while !bufs.is_empty() {
            let mut count = self.write.write_vectored(bufs)?;
            if count == 0 {
                return Err(std::io::Error::new(
                    ErrorKind::WriteZero,
                    "failed to write anything",
                ));
            }
            while count > 0 {
                if count >= bufs[0].len() {
                    count -= bufs[0].len();
                } else {
                    let remaining = &bufs[0][count..];
                    self.write.write_all(remaining)?;
                    count = 0;
                }
                bufs = &bufs[1..];

                // Skip empty slices
                while bufs.first().map(|s| s.len()) == Some(0) {
                    bufs = &bufs[1..];
                }
            }
        }

        Ok(seqno)
    }

    /// Ignore the reply for a request that was previously sent.
    pub(crate) fn discard_reply(&mut self, seqno: SequenceNumber, mode: DiscardMode) {
        if let Some(entry) = self.sent_requests.iter_mut().find(|r| r.seqno == seqno) {
            entry.discard_mode = Some(mode);
        }
        match mode {
            DiscardMode::DiscardReplyAndError => self.pending_replies.retain(|r| r.0 != seqno),
            DiscardMode::DiscardReply => {
                if let Some(index) = self.pending_replies.iter().position(|r| r.0 == seqno) {
                    while self
                        .pending_replies
                        .get(index)
                        .filter(|r| r.0 == seqno)
                        .is_some()
                    {
                        if let Some((_, packet)) = self.pending_replies.remove(index) {
                            if packet[0] == 0 {
                                // This is an error
                                self.pending_events.push_back((seqno, packet));
                            }
                        }
                    }
                }
            }
        }
    }

    // Extract the sequence number from a packet read from the X11 server. The packet must be a
    // reply, an event, or an error. All of these have a u16 sequence number in bytes 2 and 3...
    // except for KeymapNotify events.
    fn extract_sequence_number(&mut self, buffer: &[u8]) -> Option<SequenceNumber> {
        use crate::xproto::KEYMAP_NOTIFY_EVENT;
        if buffer[0] == KEYMAP_NOTIFY_EVENT {
            return None;
        }
        // We get the u16 from the wire...
        let number = u16::from_ne_bytes([buffer[2], buffer[3]]);

        // ...and use our state to reconstruct the high bytes
        let high_bytes = self.last_sequence_read & !SequenceNumber::from(u16::max_value());
        let mut full_number = SequenceNumber::from(number) | high_bytes;
        if full_number < self.last_sequence_read {
            full_number += SequenceNumber::from(u16::max_value()) + 1;
        }

        // Update our state
        self.last_sequence_read = full_number;
        if self.next_reply_expected < full_number {
            // This is most likely an event/error that allows us to update our sequence number
            // implicitly. Normally, only requests with a reply update this (in send_request()).
            self.next_reply_expected = full_number;
        }
        Some(full_number)
    }

    /// An X11 packet was received from the connection and is now enqueued into our state.
    pub(crate) fn enqueue_packet(&mut self, packet: Vec<u8>) {
        let kind = packet[0];

        // extract_sequence_number() updates our state and is thus important to call even when we
        // do not need the sequence number
        let seqno = self
            .extract_sequence_number(&packet)
            .unwrap_or(self.last_sequence_read);

        // Remove all entries for older requests
        while let Some(request) = self.sent_requests.front() {
            if request.seqno >= seqno {
                break;
            }
            let _ = self.sent_requests.pop_front();
        }
        let request = self.sent_requests.front().filter(|r| r.seqno == seqno);

        if kind == 0 {
            // It is an error. Let's see where we have to send it to.
            if let Some(request) = request {
                match request.discard_mode {
                    Some(DiscardMode::DiscardReplyAndError) => { /* This error should be ignored */
                    }
                    Some(DiscardMode::DiscardReply) => {
                        self.pending_events.push_back((seqno, packet))
                    }
                    None => self.pending_replies.push_back((seqno, packet)),
                }
            } else {
                // Unexpected error, send to main loop
                self.pending_events.push_back((seqno, packet));
            }
        } else if kind == 1 {
            // It is a reply
            if request.filter(|r| r.discard_mode.is_some()).is_some() {
                // This reply should be discarded
            } else {
                self.pending_replies.push_back((seqno, packet));
            }
        } else {
            // It is an event
            self.pending_events.push_back((seqno, packet));
        }
    }

    /// Check if the server already sent an answer to the request with the given sequence number.
    ///
    /// This function is meant to be used for requests that have a reply. Such requests always
    /// cause a reply or an error to be sent.
    pub(crate) fn poll_for_reply_or_error(&mut self, sequence: SequenceNumber) -> Option<Vec<u8>> {
        for (index, (seqno, _packet)) in self.pending_replies.iter().enumerate() {
            if *seqno == sequence {
                return Some(self.pending_replies.remove(index).unwrap().1);
            }
        }
        None
    }

    /// Prepare for calling `poll_check_for_reply_or_error()`.
    ///
    /// To check if a request with a reply caused an error, one simply has to wait for the error or
    /// reply to be received. However, this approach does not work for requests without errors:
    /// Success is indicated by the absence of an error.
    ///
    /// Thus, this function ensures that a reply with a higher sequence number will be received.
    /// Since the X11 server handles requests in-order, if the reply to a later request is
    /// received, this means that the earlier request did not fail.
    pub(crate) fn prepare_check_for_reply_or_error(
        &mut self,
        sequence: SequenceNumber,
    ) -> Result<(), std::io::Error> {
        if self.next_reply_expected < sequence {
            self.send_sync()?;
        }
        assert!(self.next_reply_expected >= sequence);
        Ok(())
    }

    /// Check if the request with the given sequence number was already handled by the server.
    ///
    /// Before calling this function, you must call `prepare_check_for_reply_or_error()` with the
    /// sequence number.
    ///
    /// This function can be used for requests with and without a reply.
    pub(crate) fn poll_check_for_reply_or_error(&mut self, sequence: SequenceNumber) -> PollReply {
        if let Some(result) = self.poll_for_reply_or_error(sequence) {
            return PollReply::Reply(result);
        }

        if self.last_sequence_read > sequence {
            // We can be sure that there will be no reply/error
            PollReply::NoReply
        } else {
            // Hm, we cannot be sure yet. Perhaps there will still be a reply/error
            PollReply::TryAgain
        }
    }

    /// Find the reply for the request with the given sequence number.
    ///
    /// If the request caused an error, that error will be handled as an event. This means that a
    /// latter call to `poll_for_event()` will return it.
    pub(crate) fn poll_for_reply(&mut self, sequence: SequenceNumber) -> PollReply {
        if let Some(reply) = self.poll_for_reply_or_error(sequence) {
            if reply[0] == 0 {
                self.pending_events.push_back((sequence, reply));
                PollReply::NoReply
            } else {
                PollReply::Reply(reply)
            }
        } else {
            PollReply::TryAgain
        }
    }

    /// Get a pending event.
    pub(crate) fn poll_for_event_with_sequence(
        &mut self,
    ) -> Option<(SequenceNumber, GenericEvent<Vec<u8>>)> {
        self.pending_events
            .pop_front()
            .map(|(seqno, event)| (seqno, GenericEvent::new(event).unwrap()))
    }

    /// Send all pending events by flushing the output buffer.
    pub(crate) fn flush(&mut self) -> Result<(), std::io::Error> {
        self.write.flush()
    }
}

/// Read a `Setup` from the X11 server.
///
/// If the server sends a `SetupFailed` or `SetupAuthenticate` packet, these will be returned
/// as errors.
fn read_setup(read: &mut impl Read) -> Result<Setup, ConnectError> {
    let mut setup = vec![0; 8];
    read.read_exact(&mut setup)?;
    let extra_length = usize::from(u16::from_ne_bytes([setup[6], setup[7]])) * 4;
    // Use `Vec::reserve_exact` because this will be the final
    // length of the vector.
    setup.reserve_exact(extra_length);
    setup.resize(8 + extra_length, 0);
    read.read_exact(&mut setup[8..])?;
    match setup[0] {
        // 0 is SetupFailed
        0 => Err(ConnectError::SetupFailed((&setup[..]).try_into()?)),
        // Success
        1 => Ok((&setup[..]).try_into()?),
        // 2 is SetupAuthenticate
        2 => Err(ConnectError::SetupAuthenticate((&setup[..]).try_into()?)),
        // Uhm... no other cases are defined
        _ => Err(ParseError::ParseError.into()),
    }
}

#[cfg(test)]
mod test {
    use std::io::IoSlice;

    use super::{read_setup, ConnectionInner};
    use crate::connection::RequestKind;
    use crate::errors::ConnectError;
    use crate::x11_utils::Serialize;
    use crate::xproto::{Setup, SetupAuthenticate, SetupFailed};

    #[test]
    fn read_setup_success() {
        let mut setup = Setup {
            status: 1,
            protocol_major_version: 11,
            protocol_minor_version: 0,
            length: 0,
            release_number: 0,
            resource_id_base: 0,
            resource_id_mask: 0,
            motion_buffer_size: 0,
            maximum_request_length: 0,
            image_byte_order: 0,
            bitmap_format_bit_order: 0,
            bitmap_format_scanline_unit: 0,
            bitmap_format_scanline_pad: 0,
            min_keycode: 0,
            max_keycode: 0,
            vendor: vec![],
            pixmap_formats: vec![],
            roots: vec![],
        };
        setup.length = ((setup.serialize().len() - 8) / 4) as _;
        let setup_bytes = setup.serialize();

        let read = read_setup(&mut &setup_bytes[..]);
        assert_eq!(setup, read.unwrap());
    }

    #[test]
    fn read_setup_failed() {
        let mut setup = SetupFailed {
            status: 0,
            protocol_major_version: 11,
            protocol_minor_version: 0,
            length: 0,
            reason: b"whatever".to_vec(),
        };
        setup.length = ((setup.serialize().len() - 8) / 4) as _;
        let setup_bytes = setup.serialize();

        match read_setup(&mut &setup_bytes[..]) {
            Err(ConnectError::SetupFailed(read)) => assert_eq!(setup, read),
            value => panic!("Unexpected value {:?}", value),
        }
    }

    #[test]
    fn read_setup_authenticate() {
        let setup = SetupAuthenticate {
            status: 2,
            reason: b"12345678".to_vec(),
        };
        let setup_bytes = setup.serialize();

        match read_setup(&mut &setup_bytes[..]) {
            Err(ConnectError::SetupAuthenticate(read)) => assert_eq!(setup, read),
            value => panic!("Unexpected value {:?}", value),
        }
    }

    #[test]
    fn insert_sync_no_reply() -> Result<(), std::io::Error> {
        // The connection must send a sync (GetInputFocus) request every 2^16 requests (that do not
        // have a reply). Thus, this test sends more than that and tests for the sync to appear.

        let length = 1u16.to_ne_bytes();
        let no_operation = [127, 0, length[0], length[1]];
        let get_input_focus = [43, 0, length[0], length[1]];

        // Set up a connection that writes to this array
        let mut written = [0; 0x10000 * 4 + 4];
        let mut output = &mut written[..];
        let mut connection = ConnectionInner::new(&mut output);

        for num in 1..0x10000 {
            let seqno =
                connection.send_request(&[IoSlice::new(&no_operation)], RequestKind::IsVoid)?;
            assert_eq!(num, seqno);
        }
        // request 0x10000 should be a sync, hence the next one is 0x10001
        let seqno = connection.send_request(&[IoSlice::new(&no_operation)], RequestKind::IsVoid)?;
        assert_eq!(0x10001, seqno);

        let mut expected: Vec<_> = std::iter::repeat(&no_operation)
            .take(0xffff)
            .flatten()
            .copied()
            .collect();
        expected.extend_from_slice(&get_input_focus);
        expected.extend_from_slice(&no_operation);

        assert_eq!(&written[..], &expected[..]);
        Ok(())
    }

    #[test]
    fn insert_no_sync_with_reply() -> Result<(), std::io::Error> {
        // Compared to the previous test, this uses RequestKind::HasResponse, so no sync needs to
        // be inserted.

        let length = 1u16.to_ne_bytes();
        let get_input_focus = [43, 0, length[0], length[1]];

        // Set up a connection that writes to this array
        let mut written = [0; 0x10001 * 4];
        let mut output = &mut written[..];
        let mut connection = ConnectionInner::new(&mut output);

        for num in 1..=0x10001 {
            let seqno = connection
                .send_request(&[IoSlice::new(&get_input_focus)], RequestKind::HasResponse)?;
            assert_eq!(num, seqno);
        }

        let expected: Vec<_> = std::iter::repeat(&get_input_focus)
            .take(0x10001)
            .flatten()
            .copied()
            .collect();
        assert_eq!(&written[..], &expected[..]);
        Ok(())
    }

    fn partial_write_test(request: &[u8], expected_err: &str) {
        let mut written = [0x21; 2];
        let mut output = &mut written[..];
        let mut connection = ConnectionInner::new(&mut output);
        let request = [IoSlice::new(&request), IoSlice::new(&request)];
        let error = connection
            .send_request(&request, RequestKind::IsVoid)
            .unwrap_err();
        assert_eq!(expected_err, error.to_string());
    }

    #[test]
    fn partial_write_larger_slice() {
        partial_write_test(&[0; 4], "failed to write whole buffer");
    }

    #[test]
    fn partial_write_slice_border() {
        partial_write_test(&[0; 2], "failed to write anything");
    }

    #[test]
    fn full_write_trailing_empty() {
        let mut written = [0; 4];
        let mut output = &mut written[..];
        let mut connection = ConnectionInner::new(&mut output);
        let (request1, request2) = ([0; 4], [0; 0]);
        let request = [IoSlice::new(&request1), IoSlice::new(&request2)];
        let _ = connection
            .send_request(&request, RequestKind::IsVoid)
            .unwrap();
    }
}
