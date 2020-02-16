//! A pure-rust implementation of a connection to an X11 server.

use std::io::{IoSlice, Write, Read};
use std::error::Error;
use std::convert::TryInto;
use std::iter::repeat;
use std::collections::VecDeque;

use crate::utils::Buffer;
use crate::connection::{SequenceNumber, RequestKind, DiscardMode};
use crate::generated::xproto::{Setup, SetupRequest, SetupFailed, SetupAuthenticate, GET_INPUT_FOCUS_REQUEST};
use crate::x11_utils::{GenericEvent, Serialize};
use crate::errors::ParseError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct SentRequest {
    seqno: SequenceNumber,
    kind: RequestKind,
    discard_mode: Option<DiscardMode>,
}

#[derive(Debug)]
pub(crate) struct ConnectionInner<Stream>
where Stream: Read + Write
{
    // The underlying byte stream that connects us to the X11 server
    stream: Stream,

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

impl<Stream> ConnectionInner<Stream>
where Stream: Read + Write
{
    pub(crate) fn connect(mut stream: Stream, auth_name: Vec<u8>, auth_data: Vec<u8>)
    -> Result<(Self, Setup), Box<dyn Error>> {
        Self::write_setup(&mut stream, auth_name, auth_data)?;
        let setup = Self::read_setup(&mut stream)?;
        let result = ConnectionInner {
            stream,
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

    fn write_setup(stream: &mut Stream, auth_name: Vec<u8>, auth_data: Vec<u8>)
    -> Result<(), Box<dyn Error>> {
        let request = SetupRequest {
            byte_order: Self::byte_order(),
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: auth_name,
            authorization_protocol_data: auth_data,
        };
        stream.write_all(&request.serialize())?;
        Ok(())
    }

    fn read_setup(stream: &mut Stream) -> Result<Setup, Box<dyn Error>> {
        let mut setup: Vec<_> = repeat(0).take(8).collect();
        stream.read_exact(&mut setup)?;
        let length = u16::from_ne_bytes([setup[6], setup[7]]);
        setup.extend(repeat(0).take(length as usize * 4));
        stream.read_exact(&mut setup[8..])?;
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

    fn read_packet(&mut self) -> Result<Buffer, Box<dyn Error>> {
        let mut buffer: Vec<_> = repeat(0).take(32).collect();
        self.stream.read_exact(&mut buffer)?;
        let extra_length = match buffer[0] {
            1 => { // reply
                4 * u32::from_ne_bytes([buffer[4], buffer[5], buffer[6], buffer[7]])
            },
            35 | 163 => panic!("XGE events not yet supported"),
            _ => 0
        };
        buffer.extend(repeat(0).take(extra_length as usize));
        self.stream.read_exact(&mut buffer[32..])?;

        Ok(Buffer::from_vec(buffer))
    }

    fn send_sync(&mut self) -> Result<(), Box<dyn Error>> {
        let length = 1u16.to_ne_bytes();
        let written = self.stream.write(&[GET_INPUT_FOCUS_REQUEST, 0 /* pad */, length[0], length[1]])?;
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
        let written = self.stream.write_vectored(bufs)?;
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

    fn read_packet_and_enqueue(&mut self) -> Result<(), Box<dyn Error>> {
        let packet = self.read_packet()?;
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

        Ok(())
    }

    pub(crate) fn wait_for_reply_or_error(&mut self, sequence: SequenceNumber) -> Result<Buffer, Box<dyn Error>> {
        loop {
            for (index, (seqno, _packet)) in self.pending_replies.iter().enumerate() {
                if *seqno == sequence {
                    return Ok(self.pending_replies.remove(index).unwrap().1)
                }
            }
            self.read_packet_and_enqueue()?;
        }
    }

    pub(crate) fn check_for_reply_or_error(&mut self, sequence: SequenceNumber) -> Result<Option<Buffer>, Box<dyn Error>> {
        if self.next_reply_expected < sequence {
            self.send_sync()?;
        }

        loop {
            for (index, (seqno, _packet)) in self.pending_replies.iter().enumerate() {
                if *seqno == sequence {
                    return Ok(Some(self.pending_replies.remove(index).unwrap().1))
                }
            }
            if self.last_sequence_read > sequence {
                return Ok(None);
            }
            self.read_packet_and_enqueue()?;
        }
    }

    pub(crate) fn wait_for_reply(&mut self, sequence: SequenceNumber) -> Result<Option<Buffer>, Box<dyn Error>> {
        let reply = self.wait_for_reply_or_error(sequence)?;
        if reply[0] == 0 {
            self.pending_events.push_back(reply);
            Ok(None)
        } else {
            Ok(Some(reply))
        }
    }

    pub(crate) fn wait_for_event(&mut self) -> Result<GenericEvent, Box<dyn Error>> {
        loop {
            let event = self.pending_events.pop_front();
            if let Some(event) = event {
                return Ok(event.try_into()?);
            }
            self.read_packet_and_enqueue()?;
        }
    }

    pub(crate) fn poll_for_event(&mut self) -> Result<Option<GenericEvent>, Box<dyn Error>> {
        // FIXME: Check if something can be read from the wire
        self.pending_events.pop_front()
            .map(TryInto::try_into)
            .transpose()
            .map_err(Into::into)
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
