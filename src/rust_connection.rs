//! A pure-rust implementation of a connection to an X11 server.

use std::net::TcpStream;
use std::io::{IoSlice, Write, Read};
use std::error::Error;
use std::convert::{TryFrom, TryInto};
use std::iter::repeat;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::os::unix::io::RawFd;

use crate::utils::Buffer;
use crate::connection::{Connection, Cookie, SequenceNumber, ExtensionInformation};
use crate::generated::xproto::{Setup, SetupRequest, QueryExtensionReply};
use crate::x11_utils::GenericEvent;
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};

#[derive(Debug)]
struct ConnectionInner {
    stream: TcpStream,

    next_id: u32,
    max_id: u32,

    last_sequence_written: SequenceNumber,
    checked_requests: VecDeque<SequenceNumber>,

    last_sequence_read: SequenceNumber,
    pending_events: VecDeque<Buffer>,
    pending_replies: VecDeque<(SequenceNumber, Buffer)>,
}

impl ConnectionInner {
    fn connect(mut stream: TcpStream) -> Result<(ConnectionInner, Setup), Box<dyn Error>> {
        Self::write_setup(&mut stream)?;
        let setup = Self::read_setup(&mut stream)?;
        let (base, mask) = (setup.resource_id_base, setup.resource_id_mask);
        let result = ConnectionInner {
            stream,
            next_id: base,
            max_id: base | mask,
            last_sequence_written: 0,
            last_sequence_read: 0,
            checked_requests: VecDeque::new(),
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

    fn write_setup(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        let request = SetupRequest {
            byte_order: Self::byte_order(),
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: Vec::new(),
            authorization_protocol_data: Vec::new(),
        };
        stream.write_all(&request.to_ne_bytes())?;
        Ok(())
    }

    fn read_setup(stream: &mut TcpStream) -> Result<Setup, Box<dyn Error>> {
        let mut setup: Vec<_> = repeat(0).take(8).collect();
        stream.read_exact(&mut setup)?;
        let length = u16::from_ne_bytes([setup[6], setup[7]]);
        setup.extend(repeat(0).take(length as usize * 4));
        stream.read_exact(&mut setup[8..])?;
        Ok((&setup[..]).try_into()?)
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

    fn send_request(&mut self, bufs: &[IoSlice], fds: &[RawFd], has_reply: bool) -> Result<SequenceNumber, Box<dyn Error>> {
        assert_eq!(fds.len(), 0);

        self.last_sequence_written += 1;
        let seqno = self.last_sequence_written;

        if has_reply {
            self.checked_requests.push_back(seqno);
        }

        // FIXME: We must always be able to read when we write
        let written = self.stream.write_vectored(bufs)?;
        assert_eq!(written, bufs.iter().map(|s| s.len()).sum(), "FIXME: Implement partial write handling");
        Ok(seqno)
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
        let high_bytes = self.last_sequence_read & !(u16::max_value() as SequenceNumber);
        let mut full_number = (number as SequenceNumber) | high_bytes;
        if full_number < self.last_sequence_read {
            full_number += u16::max_value() as SequenceNumber + 1;
        }

        // Update our state
        self.last_sequence_read = full_number;
        Some(full_number)
    }

    fn read_packet_and_enqueue(&mut self) -> Result<(), Box<dyn Error>> {
        let packet = self.read_packet()?;
        let kind = packet[0];

        // extract_sequence_number() updates our state and is thus important to call even when we
        // do not need the sequence number
        let seqno = self.extract_sequence_number(&packet).unwrap_or(self.last_sequence_read);

        if kind == 0 {
            // It is an error. Let's see where we have to send it to.
            let is_checked = if let Some(&sequence) = self.checked_requests.front() {
                seqno == sequence
            } else {
                false
            };
            if is_checked {
                self.pending_replies.push_back((seqno, packet));
            } else {
                self.pending_events.push_back(packet);
            }
        } else if kind == 1 {
            // It is a reply
            self.pending_replies.push_back((seqno, packet));
        } else {
            // It is an event
            self.pending_events.push_back(packet);
        }

        Ok(())
    }

    fn wait_for_reply(&mut self, sequence: SequenceNumber) -> Result<Buffer, Box<dyn Error>> {
        // FIXME: Idea: Have this always return a Buffer and have the caller (Cookie and the multi
        // reply cookie) tell apart reply and error.
        loop {
            for (index, (seqno, _packet)) in self.pending_replies.iter().enumerate() {
                if *seqno == sequence {
                    return Ok(self.pending_replies.remove(index).unwrap().1)
                }
            }
            self.read_packet_and_enqueue()?;
        }
    }

    fn wait_for_event(&mut self) -> Result<GenericEvent, Box<dyn Error>> {
        loop {
            let event = self.pending_events.pop_front();
            if let Some(event) = event {
                return Ok(event.try_into()?);
            }
            self.read_packet_and_enqueue()?;
        }
    }

    fn poll_for_event(&mut self) -> Result<Option<GenericEvent>, Box<dyn Error>> {
        // FIXME: Check if something can be read from the wire
        self.pending_events.pop_front()
            .map(TryInto::try_into)
            .transpose()
            .map_err(Into::into)
    }

    fn generate_id(&mut self) -> u32 {
        // FIXME: use the XC_MISC extension to get a new range when the old one is used up
        assert!(self.next_id < self.max_id);
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

/// A connection to an X11 server implemented in pure rust
#[derive(Debug)]
pub struct RustConnection {
    inner: RefCell<ConnectionInner>,
    setup: Setup,
    extension_information: ExtensionInformation,
}

impl RustConnection {
    /// Establish a new connection.
    ///
    /// FIXME: This currently hardcodes the display `127.0.0.1:1`.
    pub fn connect() -> Result<(RustConnection, usize), Box<dyn Error>> {
        let screen = 0;
        let stream = TcpStream::connect("127.0.0.1:6001")?;
        let (inner, setup) = ConnectionInner::connect(stream)?;
        let conn = RustConnection {
            inner: RefCell::new(inner),
            setup,
            extension_information: Default::default()
        };
        Ok((conn, screen))
    }

    fn send_request(&self, bufs: &[IoSlice], fds: &[RawFd], has_reply: bool) -> Result<SequenceNumber, ConnectionError> {
        self.inner.borrow_mut().send_request(bufs, fds, has_reply).or(Err(ConnectionError::UnknownError))
    }
}

impl Connection for RustConnection {
    fn send_request_with_reply<R>(&self, bufs: &[IoSlice], fds: &[RawFd]) -> Result<Cookie<Self, R>, ConnectionError>
        where R: TryFrom<Buffer, Error=ParseError>
    {
        let mut storage = Default::default();
        let bufs = self.compute_length_field(bufs, &mut storage)?;

        Ok(Cookie::new(self, self.send_request(bufs, fds, true)?))
    }

    fn send_request_without_reply(&self, bufs: &[IoSlice], fds: &[RawFd]) -> Result<SequenceNumber, ConnectionError> {
        self.send_request(bufs, fds, false)
    }

    fn discard_reply(&self, sequence: SequenceNumber) {
        let _ = sequence;
        unimplemented!();
    }

    fn extension_information(&self, extension_name: &'static str) -> Option<&QueryExtensionReply> {
        self.extension_information.extension_information(self, extension_name)
    }

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error> {
        Ok(self.inner.borrow_mut().wait_for_reply(sequence).map_err(|_| ConnectionError::UnknownError)?)
    }

    fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError> {
        Ok(self.inner.borrow_mut().wait_for_event().map_err(|_| ConnectionError::UnknownError)?)
    }

    fn poll_for_event(&self) -> Result<Option<GenericEvent>, ConnectionError> {
        Ok(self.inner.borrow_mut().poll_for_event().map_err(|_| ConnectionError::UnknownError)?)
    }

    fn flush(&self) {
        // Nothing to do since we do not do any buffering
    }

    fn setup(&self) -> &Setup {
        &self.setup
    }

    fn generate_id(&self) -> u32 {
        self.inner.borrow_mut().generate_id()
    }

    fn maximum_request_bytes(&self) -> usize {
        unimplemented!()
    }
}
