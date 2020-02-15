//! A pure-rust implementation of a connection to an X11 server.

use std::net::TcpStream;
use std::io::IoSlice;
use std::error::Error;
use std::convert::TryFrom;
use std::cell::RefCell;

use crate::utils::{Buffer, RawFdContainer};
use crate::connection::{RequestConnection, Connection, SequenceNumber, RequestKind, DiscardMode};
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::extension_information::ExtensionInformation;
use crate::generated::xproto::{Setup, QueryExtensionReply};
use crate::x11_utils::{GenericEvent, GenericError};
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};

mod inner;
mod id_allocator;

/// A connection to an X11 server implemented in pure rust
#[derive(Debug)]
pub struct RustConnection {
    inner: RefCell<inner::ConnectionInner<TcpStream>>,
    id_allocator: RefCell<id_allocator::IDAllocator>,
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
        let (inner, setup) = inner::ConnectionInner::connect(stream)?;
        let allocator = id_allocator::IDAllocator::new(setup.resource_id_base, setup.resource_id_mask);
        let conn = RustConnection {
            inner: RefCell::new(inner),
            id_allocator: RefCell::new(allocator),
            setup,
            extension_information: Default::default()
        };
        Ok((conn, screen))
    }

    fn send_request(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>, has_reply: bool) -> Result<SequenceNumber, ConnectionError> {
        self.inner.borrow_mut().send_request(bufs, fds, has_reply).or(Err(ConnectionError::UnknownError))
    }
}

impl RequestConnection for RustConnection {
    fn send_request_with_reply<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<Cookie<Self, R>, ConnectionError>
        where R: TryFrom<Buffer, Error=ParseError>
    {
        let mut storage = Default::default();
        let bufs = self.compute_length_field(bufs, &mut storage)?;

        Ok(Cookie::new(self, self.send_request(bufs, fds, true)?))
    }

    fn send_request_with_reply_with_fds<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<CookieWithFds<Self, R>, ConnectionError>
        where R: TryFrom<(Buffer, Vec<RawFdContainer>), Error=ParseError>
    {
        let _ = (bufs, fds);
        unimplemented!()
    }

    fn send_request_without_reply(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<VoidCookie<Self>, ConnectionError> {
        // FIXME: Shouldn't this call compute_length_field? (Or rather: the implementation from
        // send_request_with_reply() should be moved to send_request())
        Ok(VoidCookie::new(self, self.send_request(bufs, fds, false)?))
    }

    fn discard_reply(&self, sequence: SequenceNumber, kind: RequestKind, mode: DiscardMode) {
        let _ = (sequence, kind, mode);
        unimplemented!();
    }

    fn extension_information(&self, extension_name: &'static str) -> Option<QueryExtensionReply> {
        self.extension_information.extension_information(self, extension_name)
    }

    fn wait_for_reply_or_error(&self, sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error> {
        Ok(self.inner.borrow_mut().wait_for_reply(sequence).map_err(|_| ConnectionError::UnknownError)?)
    }

    fn wait_for_reply(&self, _sequence: SequenceNumber) -> Result<Option<Buffer>, ConnectionError> {
        unimplemented!();
    }

    fn check_for_error(&self, _sequence: SequenceNumber) -> Result<Option<GenericError>, ConnectionError> {
        unimplemented!();
    }

    fn wait_for_reply_with_fds(&self, _sequence: SequenceNumber) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
        unimplemented!();
    }

    fn maximum_request_bytes(&self) -> usize {
        unimplemented!()
    }
}

impl Connection for RustConnection {
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
        self.id_allocator.borrow_mut().generate_id()
    }
}
