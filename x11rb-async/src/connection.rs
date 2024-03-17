// This code is dual licensed under MIT OR Apache 2.0.

//! Generic connection-related traits.

use x11rb::connection::{BufWithFds, EventAndSeqNumber, ReplyOrError, RequestKind};
use x11rb::utils::RawFdContainer as OwnedFd;
use x11rb_protocol::protocol::xproto::Setup;
use x11rb_protocol::protocol::Event;
use x11rb_protocol::x11_utils::{
    ExtensionInformation, ReplyFDsRequest, ReplyRequest, TryParse, TryParseFd, VoidRequest,
    X11Error,
};
use x11rb_protocol::{DiscardMode, RawEventAndSeqNumber, SequenceNumber};

use crate::errors::{ConnectionError, ParseError, ReplyError, ReplyOrIdError};
use crate::{Cookie, CookieWithFds, VoidCookie};

use std::future::Future;
use std::io::IoSlice;
use std::pin::Pin;

pub(crate) type Fut<'a, T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>;

/// A connection to an X11 server for sending requests.
pub trait RequestConnection: Sync {
    /// Type used as buffer to store raw replies or events before
    /// they are parsed.
    type Buf: AsRef<[u8]> + std::fmt::Debug + Send + Sync + 'static;

    /// Send a request with a reply to the server.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::send_request_with_reply`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn send_request_with_reply<R: TryParse>(
    ///     &self,
    ///     bufs: &[IoSlice],
    ///     fds: Vec<RawFdContainer>
    /// ) -> Result<Cookie<'_, Self, R>, ConnectionError>
    /// ```
    fn send_request_with_reply<'this, 'bufs, 'sl, 're, 'future, R>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<OwnedFd>,
    ) -> Fut<'future, Cookie<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        're: 'future,
        R: TryParse + Send + 're;

    /// Send a request with a reply to the server.
    ///
    /// Rather than sending raw bytes, this method sends the trait object.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::send_trait_request_with_reply`], and
    /// is semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn send_trait_request_with_reply<R: ReplyRequest>(
    ///     &self,
    ///     request: R
    /// ) -> Result<Cookie<'_, Self, R::Reply>, ConnectionError>
    /// ```
    fn send_trait_request_with_reply<'this, 'req, 'future, R>(
        &'this self,
        request: R,
    ) -> Fut<'future, Cookie<'_, Self, R::Reply>, ConnectionError>
    where
        'this: 'future,
        'req: 'future,
        R: ReplyRequest + Send + 'req,
        R::Reply: Send,
    {
        Box::pin(async move {
            let opcode = match R::EXTENSION_NAME {
                None => 0,
                Some(extension) => {
                    self.extension_information(extension)
                        .await?
                        .ok_or(ConnectionError::UnsupportedExtension)?
                        .major_opcode
                }
            };

            let (buf, fds) = request.serialize(opcode);
            self.send_request_with_reply(&[IoSlice::new(&buf)], fds)
                .await
        })
    }

    /// Send a request with a reply containing file descriptors to the server.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::send_request_with_reply_with_fds`], and
    /// is semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn send_request_with_reply_with_fds<R: TryParseFd>(
    ///     &self,
    ///     bufs: &[IoSlice],
    ///     fds: Vec<RawFdContainer>,
    /// ) -> Result<CookieWithFds<'_, Self, R>, ConnectionError>
    /// ```
    fn send_request_with_reply_with_fds<'this, 'bufs, 'sl, 're, 'future, R>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<OwnedFd>,
    ) -> Fut<'future, CookieWithFds<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        're: 'future,
        R: TryParseFd + Send + 're;

    /// Send a request with a reply containing file descriptors to the server.
    ///
    /// Rather than sending raw bytes, this method sends the trait object.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::send_trait_request_with_reply_with_fds`],
    /// and is semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn send_trait_request_with_reply_with_fds<R: ReplyFDsRequest>(
    ///     &self,
    ///     request: R
    /// ) -> Result<CookieWithFds<'_, Self, R::Reply>, ConnectionError>
    /// ```
    fn send_trait_request_with_reply_with_fds<'this, 'req, 'future, R>(
        &'this self,
        request: R,
    ) -> Fut<'future, CookieWithFds<'_, Self, R::Reply>, ConnectionError>
    where
        'this: 'future,
        'req: 'future,
        R: ReplyFDsRequest + Send + 'req,
        R::Reply: Send,
    {
        Box::pin(async move {
            let opcode = match R::EXTENSION_NAME {
                None => 0,
                Some(extension) => {
                    self.extension_information(extension)
                        .await?
                        .ok_or(ConnectionError::UnsupportedExtension)?
                        .major_opcode
                }
            };

            let (buf, fds) = request.serialize(opcode);
            self.send_request_with_reply_with_fds(&[IoSlice::new(&buf)], fds)
                .await
        })
    }

    /// Send a request without a reply to the server.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::send_request_without_reply`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn send_request_without_reply(
    ///     &self,
    ///     bufs: &[IoSlice],
    ///     fds: Vec<RawFdContainer>,
    /// ) -> Result<VoidCookie<'_, Self>, ConnectionError>
    /// ```
    fn send_request_without_reply<'this, 'bufs, 'sl, 'future>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<OwnedFd>,
    ) -> Fut<'future, VoidCookie<'this, Self>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future;

    /// Send a request without a reply to the server.
    ///
    /// Rather than sending raw bytes, this method sends the trait object.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::send_trait_request_without_reply`],
    /// and is semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn send_trait_request_without_reply<R: VoidRequest>(
    ///     &self,
    ///     request: R
    /// ) -> Result<VoidCookie<'_, Self>, ConnectionError>
    /// ```
    fn send_trait_request_without_reply<'this, 'req, 'future, R>(
        &'this self,
        request: R,
    ) -> Fut<'future, VoidCookie<'this, Self>, ConnectionError>
    where
        'this: 'future,
        'req: 'future,
        R: VoidRequest + Send + 'req,
    {
        Box::pin(async move {
            let opcode = match R::EXTENSION_NAME {
                None => 0,
                Some(extension) => {
                    self.extension_information(extension)
                        .await?
                        .ok_or(ConnectionError::UnsupportedExtension)?
                        .major_opcode
                }
            };

            let (buf, fds) = request.serialize(opcode);
            self.send_request_without_reply(&[IoSlice::new(&buf)], fds)
                .await
        })
    }

    /// The reply for this request shoiuld be discarded.
    fn discard_reply(&self, sequence: SequenceNumber, kind: RequestKind, mode: DiscardMode);

    /// Prefetch information about an extension.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::prefetch_extension_information`], and
    /// is semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn prefetch_extension_information(&self, name: &'static str) -> Result<(), ConnectionError>
    /// ```
    fn prefetch_extension_information(&self, name: &'static str) -> Fut<'_, (), ConnectionError>;

    /// Get information about an extension.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::extension_information`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn extension_information(&self, name: &'static str)
    ///     -> Result<Option<ExtensionInformation>, ConnectionError>
    /// ```
    fn extension_information(
        &self,
        name: &'static str,
    ) -> Fut<'_, Option<ExtensionInformation>, ConnectionError>;

    /// Wait for the reply to a request.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::wait_for_reply_or_error`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn wait_for_reply_or_error(
    ///     &self,
    ///     sequence: SequenceNumber,
    /// ) -> Result<ReplyOrError<Self::Buf>, ConnectionError>
    /// ```
    fn wait_for_reply_or_error(&self, sequence: SequenceNumber) -> Fut<'_, Self::Buf, ReplyError> {
        Box::pin(async move {
            let reply_or_error = self.wait_for_reply_or_raw_error(sequence).await?;
            match reply_or_error {
                ReplyOrError::Reply(reply) => Ok(reply),
                ReplyOrError::Error(error) => Err(self.parse_error(error.as_ref())?.into()),
            }
        })
    }

    /// Wait for the reply to a request.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::wait_for_reply_or_raw_error`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn wait_for_reply_or_raw_error(
    ///     &self,
    ///     sequence: SequenceNumber,
    /// ) -> Result<ReplyOrError<Self::Buf>, ConnectionError>
    /// ```
    fn wait_for_reply_or_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<Self::Buf>, ConnectionError>;

    /// Wait for the reply to a request.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::wait_for_reply`], and is semantically
    /// equivalent to:
    ///
    /// ```no_compile
    /// async fn wait_for_reply(
    ///     &self,
    ///     sequence: SequenceNumber,
    /// ) -> Result<Option<Self::Buf>, ConnectionError>
    /// ```
    fn wait_for_reply(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError>;

    /// Wait for the reply to a request with file descriptors.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::wait_for_reply_with_fds`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn wait_for_reply_with_fds(
    ///     &self,
    ///     sequence: SequenceNumber,
    /// ) -> Result<BufWithFds<Self::Buf>, ConnectionError>
    /// ```
    fn wait_for_reply_with_fds(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, BufWithFds<Self::Buf>, ReplyError> {
        Box::pin(async move {
            let reply_or_error = self.wait_for_reply_with_fds_raw(sequence).await?;
            match reply_or_error {
                ReplyOrError::Reply(reply) => Ok(reply),
                ReplyOrError::Error(error) => Err(self.parse_error(error.as_ref())?.into()),
            }
        })
    }

    /// Wait for the reply to a request with file descriptors.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::wait_for_reply_with_fds_raw`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn wait_for_reply_with_fds_raw(
    ///     &self,
    ///     sequence: SequenceNumber,
    /// ) -> Result<ReplyOrError<BufWithFds<Self::Buf>, Self::Buf>, ConnectionError>
    /// ```
    fn wait_for_reply_with_fds_raw(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<BufWithFds<Self::Buf>, Self::Buf>, ConnectionError>;

    /// Check whether a request has errored.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::check_for_error`], and is semantically
    /// equivalent to:
    ///
    /// ```no_compile
    /// async fn check_for_error(
    ///     &self,
    ///     sequence: SequenceNumber,
    /// ) -> Result<(), ReplyError>
    /// ```
    fn check_for_error(&self, sequence: SequenceNumber) -> Fut<'_, (), ReplyError> {
        Box::pin(async move {
            let error = self.check_for_raw_error(sequence).await?;
            if let Some(error) = error {
                Err(self.parse_error(error.as_ref())?.into())
            } else {
                Ok(())
            }
        })
    }

    /// Check whether a request has errored.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::check_for_raw_error`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn check_for_raw_error(
    ///     &self,
    ///     sequence: SequenceNumber,
    /// ) -> Result<Option<Self::Buf>, ConnectionError>
    /// ```
    fn check_for_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError>;

    /// Prefetches the maximum request length.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::prefetch_maximum_request_bytes`], and
    /// is semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn prefetch_maximum_request_bytes(&self)
    /// ```
    fn prefetch_maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    /// Get the maximum request length.
    ///
    /// This is the `async` analog of [`x11rb::connection::RequestConnection::maximum_request_bytes`], and is
    /// semantically equivalent to:
    ///
    /// ```no_compile
    /// async fn maximum_request_bytes(&self) -> usize
    /// ```
    fn maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = usize> + Send + '_>>;

    /// Parse a generic error.
    fn parse_error(&self, error: &[u8]) -> Result<X11Error, ParseError>;

    /// Parse a generic event.
    fn parse_event(&self, event: &[u8]) -> Result<Event, ParseError>;
}

/// An asynchronous connection to an X11 server.
pub trait Connection: RequestConnection {
    /// Wait for a new event from the X11 server.
    ///
    /// This is the `async` analog of [`x11rb::connection::Connection::wait_for_event`], and is semantically equivalent
    /// to:
    ///
    /// ```no_compile
    /// async fn wait_for_event(&self) -> Result<Event, ConnectionError>
    /// ```
    fn wait_for_event(&self) -> Fut<'_, Event, ConnectionError> {
        Box::pin(async move { Ok(self.wait_for_event_with_sequence().await?.0) })
    }

    /// Wait for a new event from the X11 server.
    ///
    /// This is the `async` analog of [`x11rb::connection::Connection::wait_for_raw_event`], and is semantically
    /// equivalent to:
    ///
    /// ```no_compile
    /// async fn wait_for_raw_event(&self) -> Result<Self::Buf, ConnectionError>
    /// ```
    fn wait_for_raw_event(&self) -> Fut<'_, Self::Buf, ConnectionError> {
        Box::pin(async move { Ok(self.wait_for_raw_event_with_sequence().await?.0) })
    }

    /// Wait for a new event from the X11 server.
    ///
    /// This is the `async` analog of [`x11rb::connection::Connection::wait_for_event_with_sequence`], and is semantically
    /// equivalent to:
    ///
    /// ```no_compile
    /// async fn wait_for_event_with_sequence(
    ///     &self,
    ///     sequence: SequenceNumber,
    /// ) -> Result<EventAndSeqNumber, ConnectionError>
    /// ```
    fn wait_for_event_with_sequence(&self) -> Fut<'_, EventAndSeqNumber, ConnectionError> {
        Box::pin(async move {
            let (event, seq) = self.wait_for_raw_event_with_sequence().await?;
            let event = self.parse_event(event.as_ref())?;
            Ok((event, seq))
        })
    }

    /// Wait for a raw/unparsed event from the X11 server.
    ///
    /// This is the `async` analog of [`x11rb::connection::Connection::wait_for_raw_event`], and is semantically
    /// equivalent to:
    ///
    /// ```no_compile
    /// async fn wait_for_raw_event(&self) -> Result<RawEvent<Self::Buf>, ConnectionError>
    /// ```
    fn wait_for_raw_event_with_sequence(
        &self,
    ) -> Fut<'_, RawEventAndSeqNumber<Self::Buf>, ConnectionError>;

    /// Poll for a new event from the X11 server.
    fn poll_for_event(&self) -> Result<Option<Event>, ConnectionError> {
        Ok(self.poll_for_event_with_sequence()?.map(|(event, _)| event))
    }

    /// Poll for a raw/unparsed event from the X11 server.
    fn poll_for_raw_event(&self) -> Result<Option<Self::Buf>, ConnectionError> {
        Ok(self
            .poll_for_raw_event_with_sequence()?
            .map(|(event, _)| event))
    }

    /// Poll for a new event from the X11 server.
    fn poll_for_event_with_sequence(&self) -> Result<Option<EventAndSeqNumber>, ConnectionError> {
        let raw = self.poll_for_raw_event_with_sequence()?;

        match raw {
            Some((raw, seq)) => {
                let event = self.parse_event(raw.as_ref())?;
                Ok(Some((event, seq)))
            }
            None => Ok(None),
        }
    }

    /// Poll for a raw/unparsed event from the X11 server.
    fn poll_for_raw_event_with_sequence(
        &self,
    ) -> Result<Option<RawEventAndSeqNumber<Self::Buf>>, ConnectionError>;

    /// Flush the output buffer.
    fn flush(&self) -> Fut<'_, (), ConnectionError>;

    /// Get the setup information of the connection.
    fn setup(&self) -> &Setup;

    /// Generate a new X11 identifier.
    ///
    /// This is the `async` analog of [`x11rb::connection::Connection::generate_id`], and is the semantic equivalent
    /// to:
    ///
    /// ```no_compile
    /// async fn generate_id(&self) -> Result<u32, ReplyOrIdError>
    /// ```
    fn generate_id(&self) -> Fut<'_, u32, ReplyOrIdError>;
}
