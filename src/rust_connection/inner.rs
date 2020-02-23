//! A pure-rust implementation of a connection to an X11 server.

use std::io::{IoSlice, Write, Read};
use std::error::Error;
use std::convert::TryInto;
use std::collections::VecDeque;

use crate::utils::Buffer;
use crate::connection::{SequenceNumber, RequestKind, DiscardMode};
use crate::generated::xproto::{Setup, SetupRequest, SetupFailed, SetupAuthenticate, GET_INPUT_FOCUS_REQUEST};
use crate::x11_utils::{GenericEvent, Serialize};
use crate::errors::ParseError;

#[derive(Debug, Clone)]
pub(crate) enum PollReply {
    /// It is not clear yet what the result will be; try again.
    TryAgain,
    /// There will be no reply; polling is done.
    NoReply,
    /// Here is the result of the polling; polling is done.
    Reply(Buffer),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct SentRequest {
    seqno: SequenceNumber,
    kind: RequestKind,
    discard_mode: Option<DiscardMode>,
}

#[derive(Debug)]
pub(crate) struct ConnectionInner<R, W>
where R: Read, W: Write
{
    // The underlying byte stream that connects us to the X11 server
    read: R,
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
    pending_events: VecDeque<Buffer>,
    // Replies that were read, but not yet returned to the API user
    pending_replies: VecDeque<(SequenceNumber, Buffer)>,
}

impl<R, W> ConnectionInner<R, W>
where R: Read, W: Write
{
    pub(crate) fn connect(mut read: R, mut write: W, auth_name: Vec<u8>, auth_data: Vec<u8>)
    -> Result<(Self, Setup), Box<dyn Error>> {
        Self::write_setup(&mut write, auth_name, auth_data)?;
        let setup = Self::read_setup(&mut read)?;
        let result = ConnectionInner {
            read,
            write,
            last_sequence_written: 0,
            next_reply_expected: 0,
            last_sequence_read: 0,
            sent_requests: VecDeque::new(),
            pending_events: VecDeque::new(),
            pending_replies: VecDeque::new(),
        };
        Ok((result, setup))
    }

    #[cfg(target_endian = "little")]
    fn byte_order() -> u8 {
        0x6c
    }

    #[cfg(target_endian = "big")]
    fn byte_order() -> u8 {
        0x42
    }

    fn write_setup(write: &mut W, auth_name: Vec<u8>, auth_data: Vec<u8>)
    -> Result<(), Box<dyn Error>> {
        let request = SetupRequest {
            byte_order: Self::byte_order(),
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: auth_name,
            authorization_protocol_data: auth_data,
        };
        write.write_all(&request.serialize())?;
        Ok(())
    }

    fn read_setup(read: &mut R) -> Result<Setup, Box<dyn Error>> {
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
            0 => Err(Box::new(SetupError::SetupFailed((&setup[..]).try_into()?))),
            // Success
            1 => Ok((&setup[..]).try_into()?),
            // 2 is SetupAuthenticate
            2 => Err(Box::new(SetupError::SetupAuthenticate((&setup[..]).try_into()?))),
            // Uhm... no other cases are defined
            _ => Err(Box::new(ParseError::ParseError))
        }
    }

    pub(crate) fn read_packet(&mut self) -> Result<Buffer, Box<dyn Error>> {
        let mut buffer = vec![0; 32];
        self.read.read_exact(&mut buffer)?;

        use crate::generated::xproto::GE_GENERIC_EVENT;
        const REPLY: u8 = 1;
        const SENT_GE_GENERIC_EVENT: u8 = GE_GENERIC_EVENT | 0x80;
        let extra_length = match buffer[0] {
            REPLY | GE_GENERIC_EVENT | SENT_GE_GENERIC_EVENT => {
                4 * u32::from_ne_bytes([buffer[4], buffer[5], buffer[6], buffer[7]])
            },
            _ => 0
        } as usize;
        // Use `Vec::reserve_exact` because this will be the final
        // length of the vector.
        buffer.reserve_exact(extra_length);
        buffer.resize(32 + extra_length, 0);
        self.read.read_exact(&mut buffer[32..])?;

        Ok(Buffer::from_vec(buffer))
    }

    fn send_sync(&mut self) -> Result<(), Box<dyn Error>> {
        let length = 1u16.to_ne_bytes();
        let written = self.write.write(&[GET_INPUT_FOCUS_REQUEST, 0 /* pad */, length[0], length[1]])?;
        assert_eq!(written, 4);

        self.last_sequence_written += 1;
        self.next_reply_expected = self.last_sequence_written;
        self.sent_requests.push_back(SentRequest {
            seqno: self.last_sequence_written,
            kind: RequestKind::HasResponse,
            discard_mode: Some(DiscardMode::DiscardReplyAndError),
        });

        Ok(())
    }

    pub(crate) fn send_request(&mut self, bufs: &[IoSlice], kind: RequestKind) -> Result<SequenceNumber, Box<dyn Error>> {
        if self.next_reply_expected + SequenceNumber::from(u16::max_value()) <= self.last_sequence_written {
            // Send a GetInputFocus request so that we can reliably reconstruct sequence numbers in
            // received packets.
            self.send_sync()?;
        }

        self.last_sequence_written += 1;
        let seqno = self.last_sequence_written;

        let sent_request = SentRequest {
            seqno,
            kind,
            discard_mode: None,
        };
        self.sent_requests.push_back(sent_request);

        // FIXME: We must always be able to read when we write
        let written = self.write.write_vectored(bufs)?;
        assert_eq!(written, bufs.iter().map(|s| s.len()).sum(), "FIXME: Implement partial write handling");
        Ok(seqno)
    }

    pub(crate) fn discard_reply(&mut self, seqno: SequenceNumber, mode: DiscardMode) {
        if let Some(entry) = self.sent_requests.iter_mut().find(|r| r.seqno == seqno) {
            entry.discard_mode = Some(mode);
        }
        match mode {
            DiscardMode::DiscardReplyAndError => self.pending_replies.retain(|r| r.0 != seqno),
            DiscardMode::DiscardReply => {
                if let Some(index) = self.pending_replies.iter().position(|r| r.0 == seqno) {
                    while self.pending_replies.get(index).filter(|r| r.0 == seqno).is_some() {
                        if let Some((_, packet)) = self.pending_replies.remove(index) {
                            if packet[0] == 0 {
                                // This is an error
                                self.pending_events.push_back(packet);
                            }
                        }
                    }
                }
            }
        }
    }

    // Extract the sequence number from a packet read from the X11 server. Thus, the packet must be
    // a reply, an event, or an error. All of these have a u16 sequence number in bytes 2 and 3...
    // except for KeymapNotify events.
    fn extract_sequence_number(&mut self, buffer: &Buffer) -> Option<SequenceNumber> {
        use crate::generated::xproto::KEYMAP_NOTIFY_EVENT;
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

    pub(crate) fn enqueue_packet(&mut self, packet: Buffer) {
        let kind = packet[0];

        // extract_sequence_number() updates our state and is thus important to call even when we
        // do not need the sequence number
        let seqno = self.extract_sequence_number(&packet).unwrap_or(self.last_sequence_read);

        // Remove all entries for older requests
        while let Some(request) = self.sent_requests.front() {
            if request.seqno >= seqno {
                break;
            }
            let _ = self.sent_requests.pop_front();
        }
        let request = self.sent_requests.front()
            .filter(|r| r.seqno == seqno);

        if kind == 0 {
            // It is an error. Let's see where we have to send it to.
            if let Some(request) = request {
                match request.discard_mode {
                    Some(DiscardMode::DiscardReplyAndError) => { /* This error should be ignored */ },
                    Some(DiscardMode::DiscardReply) => self.pending_events.push_back(packet),
                    None => self.pending_replies.push_back((seqno, packet)),
                }
            } else {
                // Unexpected error, send to main loop
                self.pending_events.push_back(packet);
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
            self.pending_events.push_back(packet);
        }
    }

    pub(crate) fn poll_for_reply_or_error(&mut self, sequence: SequenceNumber) -> Option<Buffer> {
        for (index, (seqno, _packet)) in self.pending_replies.iter().enumerate() {
            if *seqno == sequence {
                return Some(self.pending_replies.remove(index).unwrap().1)
            }
        }
        None
    }

    pub(crate) fn prepare_check_for_reply_or_error(&mut self, sequence: SequenceNumber) -> Result<(), Box<dyn Error>> {
        if self.next_reply_expected < sequence {
            self.send_sync()?;
        }
        assert!(self.next_reply_expected >= sequence);
        Ok(())
    }

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

    pub(crate) fn poll_for_reply(&mut self, sequence: SequenceNumber) -> PollReply {
        if let Some(reply) = self.poll_for_reply_or_error(sequence) {
            if reply[0] == 0 {
                self.pending_events.push_back(reply);
                PollReply::NoReply
            } else {
                PollReply::Reply(reply)
            }
        } else {
            PollReply::TryAgain
        }
    }

    pub(crate) fn poll_for_event(&mut self) -> Option<GenericEvent> {
        self.pending_events.pop_front()
            .map(|event| event.try_into().unwrap())
    }
}

// FIXME: Clean up this error stuff... somehow
#[derive(Debug, Clone, PartialEq, Eq)]
enum SetupError {
    SetupFailed(SetupFailed),
    SetupAuthenticate(SetupAuthenticate),
}

impl Error for SetupError {}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn display(f: &mut std::fmt::Formatter, prefix: &str, value: &[u8]) -> std::fmt::Result {
            match std::str::from_utf8(value).ok() {
                Some(value) => write!(f, "{}: '{}'", prefix, value),
                None => write!(f, "{}: {:?} [message is not utf8]", prefix, value),
            }
        }

        match self {
            SetupError::SetupFailed(err) => display(f, "X11 setup failed", &err.reason),
            SetupError::SetupAuthenticate(err) => display(f, "X11 authentication failed", &err.reason),
        }
    }
}
