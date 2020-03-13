//! Generic connection-related types and definitions.
//!
//! This module contains the `Connection` trait and related definitions. The code in this module is
//! used by each concrete implementation of the X11 protocol.
//!
//!
//! # Handling X11 errors
//!
//! The X11 server can answer requests with an error packet for various reasons, e.g. because an
//! invalid window ID was given. There are three options what can be done with errors:
//!
//! - Errors can appear as X11 events in `wait_for_event()` (in XCB, this is called "unchecked")
//! - Errors can be checked for locally after a request was sent (in XCB, this is called "checked")
//! - Errors can be completely ignored (the closest analog in XCB would be `xcb_discard_reply()`)
//!
//! There is an additional difference between requests with and without replies.
//!
//! ## Requests without a reply
//!
//! For requests that do not have a reply, you get an instance of `VoidCookie` after sending the
//! request. The different behaviors can be achieved via interacting with this cookie as foolows:
//!
//! | What?           | How?                       |
//! | --------------- | -------------------------- |
//! | Treat as events | Just drop the cookie       |
//! | Check locally   | `VoidCookie::check`        |
//! | Ignore          | `VoidCookie::ignore_error` |
//!
//! ## Requests with a reply
//!
//! For requests with a reply, an additional option is what should happen to the reply. You can get
//! the reply, but any errors are still treated as events. This allows to centralise X11 error
//! handling a bit in case you only want to log errors.
//!
//! The following things can be done with the `Cookie` that you get after sending a request with an
//! error.
//!
//! | Reply  | Errors locally/ignored             | Errors as events          |
//! | ------ | ---------------------------------- | ------------------------- |
//! | Get    | `Cookie::reply`                    | `Cookie::reply_unchecked` |
//! | Ignore | `Cookie::discard_reply_and_errors` | Just drop the cookie      |

use std::convert::{TryFrom, TryInto};
use std::io::IoSlice;

use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError, ReplyError, ReplyOrIdError};
use crate::utils::RawFdContainer;
use crate::x11_utils::{GenericError, GenericEvent};
use crate::xproto::{QueryExtensionReply, Setup};

/// Number type used for referring to things that were sent to the server in responses from the
/// server.
///
/// Each request sent to the X11 server is implicitly assigned a monotonically increasing sequence
/// number. Replies, events, and errors contain the sequence number of the last request that the
/// server received. This allows to map replies to their requests and to figure out which request
/// caused an error.
pub type SequenceNumber = u64;

// Used to avoid too-complex types.
pub type BufWithFds<B> = (B, Vec<RawFdContainer>);
pub type EventAndSeqNumber<B> = (SequenceNumber, GenericEvent<B>);

/// A connection to an X11 server for sending requests.
///
/// This trait only contains functions that are used by other parts of this library. This means
/// that users of this library will most likely not need these functions, unless they want to
/// implement their own X11 connection.
pub trait RequestConnection {
    /// Type used as buffer to store raw replies or events before
    /// they are parsed.
    type Buf: AsRef<[u8]> + std::fmt::Debug + Send + Sync + 'static;

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
    fn send_request_with_reply<R>(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<Cookie<'_, Self, R>, ConnectionError>
    where
        R: for<'a> TryFrom<&'a [u8], Error = ParseError>;

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
    fn send_request_with_reply_with_fds<R>(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<CookieWithFds<'_, Self, R>, ConnectionError>
    where
        R: for<'a> TryFrom<(&'a [u8], Vec<RawFdContainer>), Error = ParseError>;

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
    fn send_request_without_reply(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<VoidCookie<'_, Self>, ConnectionError>;

    /// A reply to an error should be discarded.
    ///
    /// This method is automatically called by the `Drop` implementation on `Cookie` so that any
    /// replies that are received later can be ignored.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn discard_reply(&self, sequence: SequenceNumber, kind: RequestKind, mode: DiscardMode);

    /// Prefetches information about an extension.
    ///
    /// If the information of a extension is not cached yet, this function sends a
    /// `QueryExtension` request, but it does not wait for the reply.
    ///
    /// You can use `extension_information()` to get the reply of such request.
    ///
    /// Using this function can help to reduce round-trip latency, but you can use
    /// `extension_information()` directly without calling this function first.
    fn prefetch_extension_information(
        &self,
        extension_name: &'static str,
    ) -> Result<(), ConnectionError>;

    /// Get information about an extension.
    ///
    /// To send a request for some extension, the `QueryExtensionReply` for the extension is
    /// necessary. This function provides this information.
    ///
    /// The returned object is guaranteed to have a non-zero `present` field. Extensions that are
    /// not present are instead returned as `None`.
    fn extension_information(
        &self,
        extension_name: &'static str,
    ) -> Result<Option<QueryExtensionReply>, ConnectionError>;

    /// Wait for the reply to a request.
    ///
    /// The given sequence number identifies the request for which replies are expected. If the X11
    /// server answered the request with an error, that error is returned as an `Err`.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn wait_for_reply_or_error(
        &self,
        sequence: SequenceNumber,
    ) -> Result<Self::Buf, ReplyError<Self::Buf>>;

    /// Wait for the reply to a request.
    ///
    /// The given sequence number identifies the request for which replies are expected. If the X11
    /// server answered the request with an error, this function returns `None` and the error is
    /// instead returned by `wait_for_event()` or `poll_for_event()`.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn wait_for_reply(
        &self,
        sequence: SequenceNumber,
    ) -> Result<Option<Self::Buf>, ConnectionError>;

    /// Wait for the reply to a request that has FDs.
    ///
    /// The given sequence number identifies the request for which replies are expected.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn wait_for_reply_with_fds(
        &self,
        sequence: SequenceNumber,
    ) -> Result<BufWithFds<Self::Buf>, ReplyError<Self::Buf>>;

    /// Check whether a request that does not have a reply caused an X11 error.
    ///
    /// The given sequence number identifies the request for which the check should be performed.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn check_for_error(
        &self,
        sequence: SequenceNumber,
    ) -> Result<Option<GenericError<Self::Buf>>, ConnectionError>;

    /// Prefetches the maximum request length.
    ///
    /// If the maximum request length is not cached yet, this function sends a `BigRequests::Enable`
    /// request, but it does not wait for the reply.
    ///
    /// You can use `maximum_request_bytes()` to get the result of this request.
    ///
    /// Using this function can help to reduce round-trip latency, but you can use
    /// `maximum_request_bytes()` directly without calling this function first.
    ///
    /// Since this uses the `BigRequests` extension, the information about that extension needs to
    /// available. Otherwise, this has to wait for the reply when calling
    /// `extension_information()`.
    ///
    /// To prefetch the necessary information, you can do the following:
    /// ```no_run
    /// use x11rb::bigreq;
    /// use x11rb::connection::RequestConnection;
    /// use x11rb::errors::ConnectionError;
    /// # fn do_it(conn: impl RequestConnection) -> Result<(), ConnectionError> {
    /// // conn is a RequestConnection
    /// conn.prefetch_extension_information(bigreq::X11_EXTENSION_NAME)?;
    /// # Ok(())
    /// # }
    /// ```
    fn prefetch_maximum_request_bytes(&self) -> Result<(), ConnectionError>;

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
    /// use x11rb::connection::{BufWithFds, RequestConnection, SequenceNumber};
    /// use x11rb::cookie::{Cookie, CookieWithFds, VoidCookie};
    /// use x11rb::errors::{ParseError, ConnectionError};
    /// use x11rb::utils::RawFdContainer;
    ///
    /// struct MyConnection();
    ///
    /// impl RequestConnection for MyConnection {
    ///     type Buf = Vec<u8>;
    ///
    ///     // [snip, other functions here]
    ///     # fn discard_reply(&self, sequence: SequenceNumber,
    ///     #                  kind: x11rb::connection::RequestKind,
    ///     #                  mode: x11rb::connection::DiscardMode) {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn prefetch_extension_information(
    ///     #     &self,
    ///     #     extension_name: &'static str,
    ///     # ) -> Result<(), ConnectionError> {
    ///     #     unimplemented!()
    ///     # }
    ///     # fn extension_information(&self, ext: &'static str)
    ///     # -> Result<Option<x11rb::xproto::QueryExtensionReply>, ConnectionError> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn wait_for_reply_or_error(&self, sequence: SequenceNumber)
    ///     # -> Result<Vec<u8>, x11rb::errors::ReplyError<Vec<u8>>> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn wait_for_reply(&self, sequence: SequenceNumber)
    ///     # -> Result<Option<Vec<u8>>, x11rb::errors::ConnectionError> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn wait_for_reply_with_fds(&self, sequence: SequenceNumber)
    ///     # -> Result<BufWithFds<Vec<u8>>, x11rb::errors::ReplyError<Vec<u8>>> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn check_for_error(&self, sequence: SequenceNumber)
    ///     # ->Result<Option<x11rb::x11_utils::GenericError<Vec<u8>>>, ConnectionError> {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn maximum_request_bytes(&self) -> usize {
    ///     #    unimplemented!()
    ///     # }
    ///     # fn prefetch_maximum_request_bytes(&self) -> Result<(), ConnectionError> {
    ///     #    unimplemented!()
    ///     # }
    ///
    ///     fn send_request_with_reply<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
    ///     -> Result<Cookie<Self, R>, ConnectionError>
    ///     where R: for<'a> TryFrom<&'a [u8], Error=ParseError> {
    ///         Ok(Cookie::new(self, self.send_request(bufs, fds, true, false)?))
    ///     }
    ///
    ///     fn send_request_with_reply_with_fds<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
    ///     -> Result<CookieWithFds<Self, R>, ConnectionError>
    ///     where R: for<'a> TryFrom<(&'a [u8], Vec<RawFdContainer>), Error=ParseError> {
    ///         Ok(CookieWithFds::new(self, self.send_request(bufs, fds, true, true)?))
    ///     }
    ///
    ///     fn send_request_without_reply(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>)
    ///     -> Result<VoidCookie<Self>, ConnectionError> {
    ///         Ok(VoidCookie::new(self, self.send_request(bufs, fds, false, false)?))
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
    fn compute_length_field<'b>(
        &self,
        request_buffers: &'b [IoSlice<'b>],
        storage: &'b mut (Vec<IoSlice<'b>>, [u8; 8]),
    ) -> Result<&'b [IoSlice<'b>], ConnectionError> {
        // Compute the total length of the request
        let length: usize = request_buffers.iter().map(|buf| buf.len()).sum();
        assert_eq!(
            length % 4,
            0,
            "The length of X11 requests must be a multiple of 4, got {}",
            length
        );
        let wire_length = length / 4;

        let first_buf = &request_buffers[0];

        // If the length fits into an u16, just return the request as-is
        if let Ok(wire_length) = TryInto::<u16>::try_into(wire_length) {
            // Check that the request contains the correct length field
            let length_field = u16::from_ne_bytes([first_buf[2], first_buf[3]]);
            assert_eq!(
                wire_length, length_field,
                "Length field contains incorrect value"
            );
            return Ok(request_buffers);
        }

        // Check that the total length is not too large
        if length > self.maximum_request_bytes() {
            return Err(ConnectionError::MaximumRequestLengthExceeded);
        }

        // Okay, we need to use big requests.
        let wire_length: u32 = wire_length
            .try_into()
            .expect("X11 request larger than 2^34 bytes?!?");
        let wire_length = wire_length.to_ne_bytes();

        // Now construct the new IoSlices

        // Replacement for the first four bytes of the request
        storage.1.copy_from_slice(&[
            // First part of the request
            first_buf[0],
            first_buf[1],
            // length field zero indicates big requests
            0,
            0,
            // New bytes: extended length
            wire_length[0],
            wire_length[1],
            wire_length[2],
            wire_length[3],
        ]);
        storage.0.push(IoSlice::new(&storage.1));

        // The remaining part of the first buffer of the request
        storage.0.push(IoSlice::new(&first_buf[4..]));

        // and the rest of the request
        storage.0.extend(
            request_buffers[1..]
                .iter()
                .map(std::ops::Deref::deref)
                .map(IoSlice::new),
        );

        Ok(&storage.0[..])
    }
}

/// A connection to an X11 server.
pub trait Connection: RequestConnection {
    /// Wait for a new event from the X11 server.
    fn wait_for_event(&self) -> Result<GenericEvent<Self::Buf>, ConnectionError> {
        Ok(self.wait_for_event_with_sequence()?.1)
    }

    /// Wait for a new event from the X11 server.
    fn wait_for_event_with_sequence(&self)
        -> Result<EventAndSeqNumber<Self::Buf>, ConnectionError>;

    /// Poll for a new event from the X11 server.
    fn poll_for_event(&self) -> Result<Option<GenericEvent<Self::Buf>>, ConnectionError> {
        Ok(self.poll_for_event_with_sequence()?.map(|r| r.1))
    }

    /// Poll for a new event from the X11 server.
    fn poll_for_event_with_sequence(
        &self,
    ) -> Result<Option<EventAndSeqNumber<Self::Buf>>, ConnectionError>;

    /// Send all pending requests to the server.
    ///
    /// Implementations of this trait may buffer requests for batched sending. When this method is
    /// called, all pending requests are sent.
    ///
    /// You do not have to call this method before `wait_for_reply()`. If the request you want to
    /// wait for was not yet sent, it will be sent by `wait_for_reply()`.
    fn flush(&self) -> Result<(), ConnectionError>;

    /// Get the setup information sent by the X11 server.
    ///
    /// The setup information contains X11 server, for example the window id of the root window.
    fn setup(&self) -> &Setup;

    /// Generate a new X11 identifier.
    ///
    /// This method can, for example, be used for creating a new window. First, this method is
    /// called to generate an identifier. Next, `xproto::create_window` can be called to
    /// actually create the window.
    fn generate_id(&self) -> Result<u32, ReplyOrIdError<Self::Buf>>;
}

/// Does a request have a response?
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RequestKind {
    /// The request has no response, i.e. its type is "void".
    IsVoid,
    /// The request has a response.
    HasResponse,
}

/// Variants describing which responses to a request should be discarded.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiscardMode {
    /// Only discard the actual reply. Errors go to the main loop.
    DiscardReply,
    /// Ignore any kind of response that this request generates.
    DiscardReplyAndError,
}
