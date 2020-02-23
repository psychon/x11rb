//! A pure-rust implementation of a connection to an X11 server.

use std::io::{Read, Write, IoSlice};
use std::error::Error;
use std::convert::{TryFrom, TryInto};
use std::sync::Mutex;

use crate::utils::{Buffer, RawFdContainer};
use crate::connection::{RequestConnection, Connection, SequenceNumber, RequestKind, DiscardMode};
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::extension_information::ExtensionInformation;
use crate::generated::xproto::{Setup, QueryExtensionReply};
use crate::generated::bigreq;
use crate::x11_utils::{GenericEvent, GenericError};
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};

mod inner;
mod id_allocator;
mod parse_display;
mod stream;
mod xauth;

use inner::PollReply;

/// A connection to an X11 server implemented in pure rust
#[derive(Debug)]
pub struct RustConnection<R: Read = stream::Stream, W: Write = stream::Stream> {
    inner: Mutex<inner::ConnectionInner<R, W>>,
    id_allocator: Mutex<id_allocator::IDAllocator>,
    setup: Setup,
    extension_information: ExtensionInformation,
    maximum_request_bytes: Mutex<Option<usize>>,
}

impl RustConnection<stream::Stream, stream::Stream> {
    /// Establish a new connection.
    ///
    /// If no `dpy_name` is provided, the value from `$DISPLAY` is used.
    pub fn connect(dpy_name: Option<&str>) -> Result<(Self, usize), Box<dyn Error>> {
        // Parse display information
        let parsed_display = parse_display::parse_display(dpy_name).ok_or(ConnectionError::DisplayParsingError)?;

        // Establish connection
        let protocol = parsed_display.protocol.as_ref().map(|s| &**s);
        let stream = stream::Stream::connect(&*parsed_display.host, protocol, parsed_display.display)?;
        let screen = parsed_display.screen.into();

        let (family, address) = stream.peer_addr()?;
        let (auth_name, auth_data) = xauth::get_auth(family, &address, parsed_display.display)
            // Ignore all errors while determining auth; instead we just try without auth info.
            .unwrap_or(None)
            .unwrap_or_else(|| (Vec::new(), Vec::new()));

        let write = stream.try_clone()?;
        Ok((Self::connect_to_stream_with_auth_info(stream, write, screen, auth_name, auth_data)?, screen))
    }
}

impl<R: Read, W: Write> RustConnection<R, W> {
    /// Establish a new connection to the given streams.
    ///
    /// `read` is used for reading data from the X11 server and `write` is used for writing.
    /// `screen` is the number of the screen that should be used. This function checks that a
    /// screen with that number exists.
    pub fn connect_to_stream(read: R, write: W, screen: usize) -> Result<Self, Box<dyn Error>> {
        Self::connect_to_stream_with_auth_info(read, write, screen, Vec::new(), Vec::new())
    }

    /// Establish a new connection to the given streams.
    ///
    /// `read` is used for reading data from the X11 server and `write` is used for writing.
    /// `screen` is the number of the screen that should be used. This function checks that a
    /// screen with that number exists.
    ///
    /// The parameters `auth_name` and `auth_data` are used for the members
    /// `authorization_protocol_name` and `authorization_protocol_data` of the `SetupRequest` that
    /// is sent to the X11 server.
    pub fn connect_to_stream_with_auth_info(read: R, write: W, screen: usize, auth_name: Vec<u8>, auth_data: Vec<u8>)
    -> Result<Self, Box<dyn Error>> {
        let (inner, setup) = inner::ConnectionInner::connect(read, write, auth_name, auth_data)?;

        // Check that we got a valid screen number
        if screen >= setup.roots.len() {
            return Err(Box::new(ConnectionError::InvalidScreen));
        }

        // Success! Set up our state
        let allocator = id_allocator::IDAllocator::new(setup.resource_id_base, setup.resource_id_mask);
        let conn = RustConnection {
            inner: Mutex::new(inner),
            id_allocator: Mutex::new(allocator),
            setup,
            extension_information: Default::default(),
            maximum_request_bytes: Mutex::new(None),
        };
        Ok(conn)
    }

    fn send_request(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>, kind: RequestKind) -> Result<SequenceNumber, ConnectionError> {
        if !fds.is_empty() {
            return Err(ConnectionError::FDPassingFailed);
        }
        self.inner.lock().unwrap().send_request(bufs, kind).or(Err(ConnectionError::UnknownError))
    }
}

impl<R: Read, W: Write> RequestConnection for RustConnection<R, W> {
    fn send_request_with_reply<Reply>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<Cookie<Self, Reply>, ConnectionError>
        where Reply: TryFrom<Buffer, Error=ParseError>
    {
        let mut storage = Default::default();
        let bufs = self.compute_length_field(bufs, &mut storage)?;

        Ok(Cookie::new(self, self.send_request(bufs, fds, RequestKind::HasResponse)?))
    }

    fn send_request_with_reply_with_fds<Reply>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<CookieWithFds<Self, Reply>, ConnectionError>
        where Reply: TryFrom<(Buffer, Vec<RawFdContainer>), Error=ParseError>
    {
        let mut storage = Default::default();
        let bufs = self.compute_length_field(bufs, &mut storage)?;

        let _ = (bufs, fds);
        Err(ConnectionError::FDPassingFailed)
    }

    fn send_request_without_reply(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<VoidCookie<Self>, ConnectionError> {
        let mut storage = Default::default();
        let bufs = self.compute_length_field(bufs, &mut storage)?;

        Ok(VoidCookie::new(self, self.send_request(bufs, fds, RequestKind::IsVoid)?))
    }

    fn discard_reply(&self, sequence: SequenceNumber, _kind: RequestKind, mode: DiscardMode) {
        self.inner.lock().unwrap().discard_reply(sequence, mode);
    }

    fn extension_information(&self, extension_name: &'static str) -> Option<QueryExtensionReply> {
        self.extension_information.extension_information(self, extension_name)
    }

    fn wait_for_reply_or_error(&self, sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error> {
        let mut inner = self.inner.lock().unwrap();
        loop {
            if let Some(reply) = inner.poll_for_reply_or_error(sequence) {
                if reply[0] == 0 {
                    let error: GenericError = reply.try_into()?;
                    return Err(error.into())
                } else {
                    return Ok(reply)
                }
            }
            let packet = inner.read_packet().map_err(|_| ConnectionError::UnknownError)?;
            inner.enqueue_packet(packet);
        }
    }

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<Option<Buffer>, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        loop {
            match inner.poll_for_reply(sequence) {
                PollReply::TryAgain => {},
                PollReply::NoReply => return Ok(None),
                PollReply::Reply(buffer) => return Ok(Some(buffer)),
            }
            let packet = inner.read_packet().map_err(|_| ConnectionError::UnknownError)?;
            inner.enqueue_packet(packet);
        }
    }

    fn check_for_error(&self, sequence: SequenceNumber) -> Result<Option<GenericError>, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        inner.prepare_check_for_reply_or_error(sequence).map_err(|_| ConnectionError::UnknownError)?;
        loop {
            match inner.poll_check_for_reply_or_error(sequence) {
                PollReply::TryAgain => {},
                PollReply::NoReply => return Ok(None),
                PollReply::Reply(buffer) => return Ok(buffer.try_into().ok()),
            }
            let packet = inner.read_packet().map_err(|_| ConnectionError::UnknownError)?;
            inner.enqueue_packet(packet);
        }
    }

    fn wait_for_reply_with_fds(&self, _sequence: SequenceNumber) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
        unimplemented!();
    }

    fn maximum_request_bytes(&self) -> usize {
        let mut max_bytes = self.maximum_request_bytes.lock().unwrap();
        match *max_bytes {
            None => {
                // TODO: Make it possible to prefetch extensions?
                // TODO: Make it possible to prefetch the maximum request length?
                let length = match bigreq::enable(self).ok()
                        .and_then(|cookie| cookie.reply().ok()) {
                    Some(reply) => reply.maximum_request_length,
                    None => self.setup.maximum_request_length.into(),
                };
                let length = length.try_into().unwrap_or(usize::max_value());
                let length = length * 4;
                *max_bytes = Some(length);
                length
            },
            Some(length) => length,
        }
    }
}

impl<R: Read, W: Write> Connection for RustConnection<R, W> {
    fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        loop {
            if let Some(event) = inner.poll_for_event() {
                return Ok(event);
            }
            let packet = inner.read_packet().map_err(|_| ConnectionError::UnknownError)?;
            inner.enqueue_packet(packet);
        }
    }

    fn poll_for_event(&self) -> Result<Option<GenericEvent>, ConnectionError> {
        Ok(self.inner.lock().unwrap().poll_for_event())
    }

    fn flush(&self) {
        // Nothing to do since we do not do any buffering
    }

    fn setup(&self) -> &Setup {
        &self.setup
    }

    fn generate_id(&self) -> u32 {
        self.id_allocator.lock().unwrap().generate_id(self).expect("Available XIDs exhausted")
    }
}
