use std::net::TcpStream;
use std::io::{IoSlice, Write, Read};
use std::error::Error;
use std::convert::TryFrom;
use std::iter::repeat;
use std::cell::RefCell;

use crate::utils::CSlice;
use crate::connection::{Connection, Cookie, SequenceNumber};
use crate::generated::xproto::{Setup, Setuprequest};
use crate::x11_utils::GenericEvent;
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};

struct ConnectionInner {
    stream: TcpStream,
    request: SequenceNumber,
    next_id: u32,
    max_id: u32
}

impl ConnectionInner {
    fn connect(mut stream: TcpStream) -> Result<(ConnectionInner, Setup), Box<dyn Error>> {
        Self::write_setup(&mut stream)?;
        let setup = Self::read_setup(&mut stream)?;
        let (base, mask) = (setup.resource_id_base, setup.resource_id_mask);
        let result = ConnectionInner {
            stream,
            request: 0,
            next_id: base,
            max_id: base | mask
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
        let request = Setuprequest {
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
        Ok(Setup::try_from(&setup[..])?)
    }

    fn read_packet(&mut self) -> Result<CSlice, Box<dyn Error>> {
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

        // HACK FIXME turn the Vec into a CSlice, because the current API requires it
        unsafe {
            let ptr: *mut u8 = libc::malloc(buffer.len()) as _;
            let slice = std::slice::from_raw_parts_mut(ptr, buffer.len());
            slice[..].clone_from_slice(&buffer[..]);
            Ok(CSlice::new(slice.as_ptr(), buffer.len()))
        }
    }

    fn wait_for_reply(&mut self, sequence: SequenceNumber) -> Result<CSlice, ConnectionErrorOrX11Error> {
        // FIXME: Actually use the sequence number and handle things correctly.
        // FIXME: Idea: Have this always return a CSlice and have the caller (Cookie and the multi
        // reply cookie) tell apart reply and error.
        let _ = sequence;
        Ok(self.read_packet().map_err(|_| ConnectionError::UnknownError)?)
    }

    fn generate_id(&mut self) -> u32 {
        // FIXME: use the XC_MISC extension to get a new range when the old one is used up
        assert!(self.next_id < self.max_id);
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub struct RustConnection {
    inner: RefCell<ConnectionInner>,
    setup: Setup
}

impl RustConnection {
    pub fn connect() -> Result<(RustConnection, usize), Box<dyn Error>> {
        let screen = 0;
        let stream = TcpStream::connect("127.0.0.1:6001")?;
        let (inner, setup) = ConnectionInner::connect(stream)?;
        let conn = RustConnection {
            inner: RefCell::new(inner),
            setup
        };
        Ok((conn, screen))
    }

    fn send_request(&self, bufs: &[IoSlice], has_reply: bool) -> SequenceNumber {
        let mut inner = self.inner.borrow_mut();
        inner.request += 1;
        let seqno = inner.request;
        let _ = has_reply;
        // FIXME: We must always be able to read when we write
        let written = inner.stream.write_vectored(bufs).unwrap();
        assert_eq!(written, bufs.iter().map(|s| s.len()).sum(), "FIXME: Implement partial write handling");
        seqno
    }
}

impl Connection for RustConnection {
    fn send_request_with_reply<R>(&self, bufs: &[IoSlice]) -> Cookie<Self, R>
        where R: TryFrom<CSlice, Error=ParseError>
    {
        Cookie::new(self, self.send_request(bufs, true))
    }

    fn send_request_without_reply(&self, bufs: &[IoSlice]) -> SequenceNumber {
        self.send_request(bufs, false)
    }

    fn discard_reply(&self, sequence: SequenceNumber) {
        let _ = sequence;
        unimplemented!();
    }

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<CSlice, ConnectionErrorOrX11Error> {
        self.inner.borrow_mut().wait_for_reply(sequence)
    }

    fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError> {
        unimplemented!();
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
}
