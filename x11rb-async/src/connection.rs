// This code is dual licensed under MIT OR Apache 2.0.

//! Generic connection-related traits.

use x11rb::connection::RequestKind;
use x11rb_protocol::DiscardMode;

use crate::errors::{ConnectionError, ParseError, ReplyOrIdError};
use crate::x11_utils::{ExtensionInformation, TryParse, TryParseFd, X11Error};
use crate::{
    BufWithFds, Cookie, CookieWithFds, Event, RawEventAndSeqNumber, RawFdContainer, ReplyOrError,
    SequenceNumber, Setup, VoidCookie,
};

use std::boxed::Box;
use std::future::Future;
use std::io::IoSlice;
use std::pin::Pin;

pub(crate) type Fut<'a, T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>;

/// A connection to an X11 server for sending requests.
pub trait RequestConnection {
    /// Type used as buffer to store raw replies or events before
    /// they are parsed.
    type Buf: AsRef<[u8]> + std::fmt::Debug + Send + Sync + 'static;

    /// Send a request with a reply to the server.
    ///
    /// This is the `async` analog of [`x11rb::RequestConnection::send_request_with_reply`].
    fn send_request_with_reply<'this, 'bufs, 'sl, 'future, R>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<RawFdContainer>,
    ) -> Fut<'future, Cookie<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        R: TryParse + Send;

    /// Send a request with a reply containing file descriptors to the server.
    ///
    /// This is the `async` analog of [`x11rb::RequestConnection::send_request_with_reply_with_fds`].
    fn send_request_with_reply_with_fds<'this, 'bufs, 'sl, 'future, R>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<RawFdContainer>,
    ) -> Fut<'future, CookieWithFds<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        R: TryParseFd + Send;

    /// Send a request without a reply to the server.
    ///
    /// This is the `async` analog of [`x11rb::RequestConnection::send_request_without_reply`].
    fn send_request_without_reply<'this, 'bufs, 'sl, 'future>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<RawFdContainer>,
    ) -> Fut<'future, VoidCookie<'this, Self>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future;

    /// The reply for this request shoiuld be discarded.
    fn discard_reply(&self, sequence: SequenceNumber, kind: RequestKind, mode: DiscardMode);

    /// Prefetch information about an extension.
    ///
    /// This is the `async` analog of [`x11rb::RequestConnection::prefetch_extension_information`].
    fn prefetch_extension_information(&self, name: &'static str) -> Fut<'_, (), ConnectionError>;

    /// Get information about an extension.
    ///
    /// This is the `async` analog of [`x11rb::RequestConnection::extension_information`].
    fn extension_information(
        &self,
        name: &'static str,
    ) -> Fut<'_, Option<ExtensionInformation>, ConnectionError>;

    /// Wait for the reply to a request.
    ///
    /// This is the `async` analog of [`x11rb::RequestConnection::wait_for_reply_or_raw_error`].
    fn wait_for_reply_or_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<Self::Buf>, ConnectionError>;

    /// Wait for the reply to a request.
    ///
    /// This is the `async` analog of [`x11rb::RequestConnection::wait_for_reply`].
    fn wait_for_reply(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError>;

    /// Wait for the reply to a request with file descriptors.
    fn wait_for_reply_with_fds_raw(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<BufWithFds<Self::Buf>, Self::Buf>, ConnectionError>;

    /// Check whether a request has errored.
    ///
    /// This is the `async` analog of [`x11rb::RequestConnection::check_for_raw_error`].
    fn check_for_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError>;

    /// Prefetches the maximum request length.
    fn prefetch_maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    /// Get the maximum request length.
    fn maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = usize> + Send + '_>>;

    /// Parse a generic error.
    fn parse_error(&self, error: &[u8]) -> Result<X11Error, ParseError>;

    /// Parse a generic event.
    fn parse_event(&self, event: &[u8]) -> Result<Event, ParseError>;
}

/// An asynchronous connection to an X11 server.
pub trait Connection: RequestConnection {
    /// Wait for a raw/unparsed event from the X11 server.
    ///
    /// This is the `async` analog of [`x11rb::Connection::wait_for_raw_event`].
    fn wait_for_raw_event(&self) -> Fut<'_, RawEventAndSeqNumber<Self::Buf>, ConnectionError>;

    /// Poll for a raw/unparsed event from the X11 server.
    fn poll_for_raw_event(
        &self,
    ) -> Result<Option<RawEventAndSeqNumber<Self::Buf>>, ConnectionError>;

    /// Flush the output buffer.
    fn flush(&self) -> Fut<'_, (), ConnectionError>;

    /// Get the setup information of the connection.
    fn setup(&self) -> &Setup;

    /// Generate a new X11 identifier.
    ///
    /// This is the `async` analog of [`x11rb::Connection::generate_id`].
    fn generate_id(&self) -> Fut<'_, u32, ReplyOrIdError>;
}
