//! Generic connection-related types and definitions.
//!
//! This module contains the `Connection` trait and related definitions. The code in this module is
//! used by each concrete implementation of the X11 protocol.

use std::io::IoSlice;
use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::utils::{Buffer, RawFdContainer};
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};
use crate::x11_utils::GenericEvent;
use crate::generated::xproto::{Setup, ListFontsWithInfoReply, QueryExtensionReply, ConnectionExt};

/// Number type used for referring to things that were sent to the server in responses from the
/// server.
///
/// Each request sent to the X11 server is implicitly assigned a monotonically increasing sequence
/// number. Replies, events, and errors contain the sequence number of the last request that the
/// server received. This allows to map replies to their requests and to figure out which request
/// caused an error.
pub type SequenceNumber = u64;

/// A connection to an X11 server.
pub trait Connection: Sized {
    /// Send a request with a reply to the server.
    ///
    /// The `bufs` parameter describes the raw bytes that should be sent. The returned cookie
    /// allows to get the response.
    ///
    /// The `fds` parameter contains a list of file descriptors that should be sent with the
    /// request. Ownership of these FDs is transferred to the connection. This means that the
    /// connection will close the FDs after they were sent.
    ///
    /// Users of this library will most likely not want to use this function directly. Instead, the
    /// generated code will take the supplied arguments, construct byte buffers, and call this
    /// method.
    ///
    /// The provided buffers must contain at least a single element and the first buffer must have
    /// at least four bytes. The length field must be set correctly, unless the request is larger
    /// than 2^18 bytes, because in this case, the length field would overflow. The connection
    /// automatically uses the BIG-REQUESTS extension for such large requests.
    ///
    /// In any case, the request may not be larger than the server's maximum request length.
    fn send_request_with_reply<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
        -> Result<Cookie<Self, R>, ConnectionError>
        where R: TryFrom<Buffer, Error=ParseError>;

    /// Send a request with a reply containing file descriptors to the server.
    ///
    /// The `bufs` parameter describes the raw bytes that should be sent. The returned cookie
    /// allows to get the response.
    ///
    /// The `fds` parameter contains a list of file descriptors that should be sent with the
    /// request. Ownership of these FDs is transferred to the connection. This means that the
    /// connection will close the FDs after they were sent.
    ///
    /// Users of this library will most likely not want to use this function directly. Instead, the
    /// generated code will take the supplied arguments, construct byte buffers, and call this
    /// method.
    ///
    /// The provided buffers must contain at least a single element and the first buffer must have
    /// at least four bytes. The length field must be set correctly, unless the request is larger
    /// than 2^18 bytes, because in this case, the length field would overflow. The connection
    /// automatically uses the BIG-REQUESTS extension for such large requests.
    ///
    /// In any case, the request may not be larger than the server's maximum request length.
    fn send_request_with_reply_with_fds<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
        -> Result<CookieWithFds<Self, R>, ConnectionError>
        where R: TryFrom<(Buffer, Vec<RawFdContainer>), Error=ParseError>;

    /// Send a request without a reply to the server.
    ///
    /// The `bufs` parameter describes the raw bytes that should be sent. The sequence number of
    /// the request is returned, but most likely not useful to users.
    ///
    /// The `fds` parameter contains a list of file descriptors that should be sent with the
    /// request. Ownership of these FDs is transferred to the connection. This means that the
    /// connection will close the FDs after they were sent.
    ///
    /// Users of this library will most likely not want to use this function directly. Instead, the
    /// generated code will take the supplied arguments, construct byte buffers, and call this
    /// method.
    ///
    /// The provided buffers must contain at least a single element and the first buffer must have
    /// at least four bytes. The length field must be set correctly, unless the request is larger
    /// than 2^18 bytes, because in this case, the length field would overflow. The connection
    /// automatically uses the BIG-REQUESTS extension for such large requests.
    ///
    /// In any case, the request may not be larger than the server's maximum request length.
    fn send_request_without_reply(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
        -> Result<SequenceNumber, ConnectionError>;

    /// A reply to an error should be discarded.
    ///
    /// This method is automatically called by the `Drop` implementation on `Cookie` so that any
    /// replies that are received later can be ignored.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn discard_reply(&self, sequence: SequenceNumber);

    /// Get information about an extension.
    ///
    /// To send a request for some extension, the `QueryExtensionReply` for the extension is
    /// necessary. This function provides this information.
    ///
    /// The returned object is guaranteed to have a non-zero `present` field. Extensions that are
    /// not present are instead returned as `None`.
    fn extension_information(&self, extension_name: &'static str) -> Option<&QueryExtensionReply>;

    /// Wait for the reply to a request.
    ///
    /// The given sequence number identifies the request for which replies are expected.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error>;

    /// Wait for the reply to a request that has FDs.
    ///
    /// The given sequence number identifies the request for which replies are expected.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn wait_for_reply_with_fds(&self, sequence: SequenceNumber) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error>;

    /// Wait for a new event from the X11 server.
    fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError>;

    /// Poll for a new event from the X11 server.
    fn poll_for_event(&self) -> Result<Option<GenericEvent>, ConnectionError>;

    /// Send all pending requests to the server.
    ///
    /// Implementations of this trait may buffer requests for batched sending. When this method is
    /// called, all pending requests are sent.
    ///
    /// You do not have to call this method before `wait_for_reply()`. If the request you want to
    /// wait for was not yet sent, it will be sent by `wait_for_reply()`.
    fn flush(&self);

    /// Get the setup information sent by the X11 server.
    ///
    /// The setup information contains X11 server, for example the window id of the root window.
    fn setup(&self) -> &Setup;

    /// Generate a new X11 identifier.
    ///
    /// This method can, for example, be used for creating a new window. First, this method is
    /// called to generate an identifier. Next, `generated::xproto::create_window` can be called to
    /// actually create the window.
    fn generate_id(&self) -> u32;

    /// The maximum number of bytes that the X11 server accepts in a request.
    fn maximum_request_bytes(&self) -> usize;

    /// Check the request length and use BIG-REQUESTS if necessary.
    ///
    /// Users of this library will most likely not want to use this function directly.
    ///
    /// This function is provided by the trait. It examines the given request buffers and checks
    /// that the length field is set correctly.
    ///
    /// If the request has more than 2^18 bytes, this function handles using the BIG-REQUESTS
    /// extension. The request is rewritten to include the correct length field. For this case, the
    /// `storage` parameter is needed. This function uses it to store the necessary buffers.
    ///
    /// When using this function, it is recommended to allocate the `storage` parameter with
    /// `Default::default()`.
    ///
    /// Example usage:
    /// ```
    /// use std::io::IoSlice;
    /// use std::convert::TryFrom;
    /// use x11rb::connection::{Cookie, CookieWithFds, Connection, SequenceNumber};
    /// use x11rb::errors::{ParseError, ConnectionError};
    /// use x11rb::utils::{Buffer, RawFdContainer};
    ///
    /// struct MyConnection();
    ///
    /// impl Connection for MyConnection {
    ///     // [snip, other functions here]
    ///     # fn discard_reply(&self, sequence: SequenceNumber) {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn extension_information(&self, ext: &'static str)
    ///     # -> Option<&x11rb::generated::xproto::QueryExtensionReply> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn wait_for_reply(&self, sequence: SequenceNumber)
    ///     # -> Result<Buffer, x11rb::errors::ConnectionErrorOrX11Error> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn wait_for_reply_with_fds(&self, sequence: SequenceNumber)
    ///     # -> Result<(Buffer, Vec<RawFdContainer>), x11rb::errors::ConnectionErrorOrX11Error> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn wait_for_event(&self) -> Result<x11rb::x11_utils::GenericEvent, ConnectionError> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn poll_for_event(&self)
    ///     # -> Result<Option<x11rb::x11_utils::GenericEvent>, ConnectionError> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn flush(&self) {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn setup(&self) -> &x11rb::generated::xproto::Setup {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn generate_id(&self) -> u32 {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn maximum_request_bytes(&self) -> usize {
    ///     #    unimplemented!()
    ///     # }
    ///
    ///     fn send_request_with_reply<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
    ///     -> Result<Cookie<Self, R>, ConnectionError>
    ///     where R: TryFrom<Buffer, Error=ParseError> {
    ///         Ok(Cookie::new(self, self.send_request(bufs, fds, true, false)?))
    ///     }
    ///
    ///     fn send_request_with_reply_with_fds<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
    ///     -> Result<CookieWithFds<Self, R>, ConnectionError>
    ///     where R: TryFrom<(Buffer, Vec<RawFdContainer>), Error=ParseError> {
    ///         Ok(CookieWithFds::new(self, self.send_request(bufs, fds, true, true)?))
    ///     }
    ///
    ///     fn send_request_without_reply(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
    ///     -> Result<SequenceNumber, ConnectionError> {
    ///         self.send_request(bufs, fds, false, false)
    ///     }
    /// }
    ///
    /// impl MyConnection {
    ///     fn send_request(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>,
    ///                     has_reply: bool, reply_has_fds: bool)
    ///     -> Result<SequenceNumber, ConnectionError>
    ///     {
    ///         let mut storage = Default::default();
    ///         let bufs = self.compute_length_field(bufs, &mut storage)?;
    ///         unimplemented!("Now send bufs and fds to the X11 server");
    ///     }
    /// }
    /// ```
    fn compute_length_field<'b>(&self, request_buffers: &'b [IoSlice<'b>], storage: &'b mut (Vec<IoSlice<'b>>, [u8; 8])) -> Result<&'b [IoSlice<'b>], ConnectionError>
    {
        // Compute the total length of the request
        let length: usize = request_buffers.iter()
            .map(|buf| buf.len())
            .sum();
        assert_eq!(length % 4, 0, "The length of X11 requests must be a multiple of 4, got {}", length);
        let wire_length = length / 4;

        let first_buf = &request_buffers[0];

        // If the length fits into an u16, just return the request as-is
        if let Ok(wire_length) = TryInto::<u16>::try_into(wire_length) {
            // Check that the request contains the correct length field
            let length_field = u16::from_ne_bytes([first_buf[2], first_buf[3]]);
            assert_eq!(wire_length, length_field, "Length field contains incorrect value");
            return Ok(request_buffers)
        }

        // Check that the total length is not too large
        if length > self.maximum_request_bytes() {
            return Err(ConnectionError::MaximumRequestLengthExceeded);
        }

        // Okay, we need to use big requests.
        let wire_length: u32 = wire_length.try_into().expect("X11 request larger than 2^34 bytes?!?");
        let wire_length = wire_length.to_ne_bytes();

        // Now construct the new IoSlices

        // Replacement for the first four bytes of the request
        storage.1.copy_from_slice(&[
            // First part of the request
            first_buf[0], first_buf[1],
            // length field zero indicates big requests
            0, 0,
            // New bytes: extended length
            wire_length[0], wire_length[1], wire_length[2], wire_length[3]
        ]);
        storage.0.push(IoSlice::new(&storage.1));

        // The remaining part of the first buffer of the request
        storage.0.push(IoSlice::new(&first_buf[4..]));

        // and the rest of the request
        storage.0.extend(request_buffers[1..].iter().map(std::ops::Deref::deref).map(IoSlice::new));

        Ok(&storage.0[..])
    }
}

#[derive(Debug)]
struct RawCookie<'a, C>
where C: Connection
{
    connection: &'a C,
    sequence_number: SequenceNumber,
}

impl<C> RawCookie<'_, C>
where C: Connection
{
    /// Construct a new raw cookie.
    ///
    /// This function should only be used by implementations of
    /// `Connection::send_request_with_reply`.
    fn new(connection: &C, sequence_number: SequenceNumber) -> RawCookie<C> {
        RawCookie {
            connection,
            sequence_number: sequence_number
        }
    }

    /// Consume this instance and get the contained sequence number out.
    fn to_sequence_number(self) -> SequenceNumber {
        let number = self.sequence_number;
        // Prevent drop() from running
        std::mem::forget(self);
        number
    }
}

impl<C> Drop for RawCookie<'_, C>
where C: Connection
{
    fn drop(&mut self) {
        self.connection.discard_reply(self.sequence_number);
    }
}

/// A handle to a response from the X11 server.
///
/// When sending a request to the X11 server, this library returns a `Cookie`. This `Cookie` can
/// then later be used to get the response that the server sent.
#[derive(Debug)]
pub struct Cookie<'a, C, R>
where C: Connection
{
    raw_cookie: RawCookie<'a, C>,
    phantom: PhantomData<R>
}

impl<C, R> Cookie<'_, C, R>
where R: TryFrom<Buffer, Error=ParseError>,
      C: Connection
{
    /// Construct a new cookie.
    ///
    /// This function should only be used by implementations of
    /// `Connection::send_request_with_reply`.
    pub fn new(connection: &C, sequence_number: SequenceNumber) -> Cookie<C, R> {
        Cookie {
            raw_cookie: RawCookie::new(connection, sequence_number),
            phantom: PhantomData
        }
    }

    /// Get the sequence number of the request that generated this cookie.
    pub fn sequence_number(&self) -> SequenceNumber {
        self.raw_cookie.sequence_number
    }

    /// Get the raw reply that the server sent.
    pub fn raw_reply(self) -> Result<Buffer, ConnectionErrorOrX11Error> {
        let conn = self.raw_cookie.connection;
        Ok(conn.wait_for_reply(self.raw_cookie.to_sequence_number())?)
    }

    /// Get the reply that the server sent.
    pub fn reply(self) -> Result<R, ConnectionErrorOrX11Error> {
        Ok(self.raw_reply()?.try_into()?)
    }
}

/// A handle to a response containing `RawFd` from the X11 server.
///
/// When sending a request to the X11 server, this library returns a `Cookie`. This `Cookie` can
/// then later be used to get the response that the server sent.
///
/// This variant of `Cookie` represents a response that can contain `RawFd`s.
#[derive(Debug)]
pub struct CookieWithFds<'a, C, R>
where C: Connection
{
    raw_cookie: RawCookie<'a, C>,
    phantom: PhantomData<R>
}

impl<C, R> CookieWithFds<'_, C, R>
where R: TryFrom<(Buffer, Vec<RawFdContainer>), Error=ParseError>,
      C: Connection
{
    /// Construct a new cookie.
    ///
    /// This function should only be used by implementations of
    /// `Connection::send_request_with_reply`.
    pub fn new(connection: &C, sequence_number: SequenceNumber) -> CookieWithFds<C, R> {
        CookieWithFds {
            raw_cookie: RawCookie::new(connection, sequence_number),
            phantom: PhantomData
        }
    }

    /// Get the sequence number of the request that generated this cookie.
    pub fn sequence_number(&self) -> SequenceNumber {
        self.raw_cookie.sequence_number
    }

    /// Get the raw reply that the server sent.
    pub fn raw_reply(self) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
        let conn = self.raw_cookie.connection;
        Ok(conn.wait_for_reply_with_fds(self.raw_cookie.to_sequence_number())?)
    }

    /// Get the reply that the server sent.
    pub fn reply(self) -> Result<R, ConnectionErrorOrX11Error> {
        Ok(self.raw_reply()?.try_into()?)
    }
}

/// A handle to the replies to a `ListFontsWithInfo` request.
///
/// `ListFontsWithInfo` generated more than one reply, but `Cookie` only allows getting one reply.
/// This structure implements `Iterator` and allows to get all the replies.
#[derive(Debug)]
pub struct ListFontsWithInfoCookie<'a, C: Connection>(Option<RawCookie<'a, C>>);

impl<C> ListFontsWithInfoCookie<'_, C>
where C: Connection
{
    pub(crate) fn new(cookie: Cookie<C, ListFontsWithInfoReply>) -> ListFontsWithInfoCookie<C> {
        ListFontsWithInfoCookie(Some(cookie.raw_cookie))
    }

    /// Get the sequence number of the request that generated this cookie.
    pub fn sequence_number(&self) -> Option<SequenceNumber> {
        self.0.as_ref().map(|x| x.sequence_number)
    }
}

impl<C> Iterator for ListFontsWithInfoCookie<'_, C>
where C: Connection
{
    type Item = Result<ListFontsWithInfoReply, ConnectionErrorOrX11Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let cookie = match self.0.take() {
            None => return None,
            Some(cookie) => cookie
        };
        let reply = cookie.connection.wait_for_reply(cookie.sequence_number);
        let reply = match reply {
            Err(e) => return Some(Err(e)),
            Ok(v) => v
        };
        let reply: Result<ListFontsWithInfoReply, ParseError> = reply.try_into();
        let reply = reply.map_err(ConnectionErrorOrX11Error::from);
        if reply.is_ok() {
            // Is this an indicator that no more replies follow?
            if !reply.as_ref().unwrap().name.is_empty() {
                self.0 = Some(cookie);
            } else {
                return None
            }
        }
        Some(reply)
    }
}

/// Helper for implementing `Connection::extension_information()`.
#[derive(Debug, Default)]
pub struct ExtensionInformation(Mutex<HashMap<&'static str, Option<Box<QueryExtensionReply>>>>);

impl ExtensionInformation {
    /// An implementation of `Connection::extension_information()`.
    ///
    /// The given connection is used for sending a `QueryExtension` request if needed.
    pub fn extension_information<'s, C: Connection>(&'s self, conn: &C, extension_name: &'static str)
            -> Option<&'s QueryExtensionReply> {
        let mut guard = match self.0.lock() {
            Ok(guard) => guard,
            Err(_) => return None
        };
        let map = &mut *guard;
        // Insert the entry if it does not yet exist and get a reference
        let result: &Option<Box<QueryExtensionReply>> = map
            .entry(extension_name)
            .or_insert_with(|| {
                let info = conn.query_extension(extension_name.as_bytes()).ok();
                let info = info.and_then(|c| c.reply().ok());
                if let Some(info) = info {
                    // If the extension is not present, we return None, else we box it
                    if info.present == 0 {
                        None
                    } else {
                        Some(Box::new(info))
                    }
                } else {
                    // There was an error. Pretend the extension is not present.
                    None
                }
            });
        match result.as_ref() {
            None => None,
            Some(ref reply) => unsafe {
                // We only ever allocate the boxes, but never free them (or modify their
                // contents in any way). Thus, it is safe to pass out references to the
                // contents of the box. But the borrow checker does not know that our boxes
                // life as long as we do. Thus, we do some magic with raw pointers.
                //
                // If you know a way to avoid this unsafe code, please let me know!
                let raw_ptr: *const QueryExtensionReply = &***reply;
                Some(&*raw_ptr)
            }
        }
    }
}
