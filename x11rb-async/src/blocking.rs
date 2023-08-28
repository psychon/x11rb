// This code is dual licensed under MIT OR Apache 2.0.

//! A `Connection` implementation that uses a threadpool to handle requests.

use crate::connection::{Connection, Fut, RequestConnection};
use crate::errors::{ConnectError, ConnectionError, ParseError, ReplyOrIdError};
use crate::x11_utils::X11Error;
use crate::SequenceNumber;

use std::future::Future;
use std::io::IoSlice;
use std::mem;
use std::pin::Pin;
use std::sync::Arc;

use x11rb::connection::{Connection as BlConnection, ReplyOrError, RequestKind};
use x11rb::rust_connection::{RustConnection, Stream};

#[cfg(feature = "allow-unsafe-code")]
use std::ffi::CStr;
#[cfg(feature = "allow-unsafe-code")]
use x11rb::xcb_ffi::XCBConnection;

use x11rb_protocol::DiscardMode;

/// A `Connection` implementation that uses a threadpool to handle requests.
///
/// This type wraps around an existing `x11rb` [`Connection`](x11rb::connection::Connection) type,
/// and makes it non-blocking by pushing all operations to a threadpool. This is good if, for instance,
/// you have a `Connection` type that can't trivially be integrated into a async runtime.
///
/// However, if you have the option of using a `Connection` type that is integrated into a real async
/// reactor, you should use that instead.
///
/// # Implementation
///
/// The [`blocking`] threadpool is used to handle all requests.
///
/// [`blocking`]: https://docs.rs/blocking
pub struct BlockingConnection<C> {
    inner: Arc<C>,
}

impl<C: BlConnection + Send + Sync + 'static> BlockingConnection<C> {
    /// Create a new `BlockingConnection` from an existing `Connection`.
    pub fn new(conn: Arc<C>) -> Self {
        Self { inner: conn }
    }

    /// Run the closure with a reference to the underlying connection.
    fn with_conn<T, F>(&self, f: F) -> blocking::Task<T>
    where
        F: FnOnce(&C) -> T + Send + 'static,
        T: Send + 'static,
    {
        let inner = self.inner.clone();
        blocking::unblock(move || f(&inner))
    }

    /// Get a reference to the underlying connection.
    pub fn get_ref(&self) -> &C {
        &self.inner
    }
}

impl BlockingConnection<RustConnection> {
    /// Connect to the X11 server using this connection.
    pub async fn connect(display: Option<&str>) -> Result<(Self, usize), ConnectError> {
        let display = display.map(|s| s.to_string());

        let (inner, screen) =
            blocking::unblock(move || RustConnection::connect(display.as_deref())).await?;

        Ok((Self::new(Arc::new(inner)), screen))
    }
}

impl<S: Stream + Send + Sync + 'static> BlockingConnection<RustConnection<S>> {
    /// Establish a connection over the given stream.
    pub async fn connect_to_stream(stream: S, screen: usize) -> Result<Self, ConnectError> {
        let inner =
            blocking::unblock(move || RustConnection::connect_to_stream(stream, screen)).await?;

        Ok(Self::new(Arc::new(inner)))
    }

    /// Establish a connection over the given stream using the given auth info
    pub async fn connect_to_stream_with_auth_info(
        stream: S,
        screen: usize,
        auth_name: Vec<u8>,
        auth_data: Vec<u8>,
    ) -> Result<Self, ConnectError> {
        let inner = blocking::unblock(move || {
            RustConnection::connect_to_stream_with_auth_info(stream, screen, auth_name, auth_data)
        })
        .await?;

        Ok(Self::new(Arc::new(inner)))
    }
}

#[cfg(feature = "allow-unsafe-code")]
impl BlockingConnection<XCBConnection> {
    /// Connect to the X11 server using this connection.
    pub async fn connect(display: Option<&CStr>) -> Result<(Self, usize), ConnectError> {
        let display = display.map(|s| s.to_owned());

        let (inner, screen) =
            blocking::unblock(move || XCBConnection::connect(display.as_deref())).await?;

        Ok((Self::new(Arc::new(inner)), screen))
    }
}

impl<C: BlConnection + Send + Sync + 'static> RequestConnection for BlockingConnection<C> {
    type Buf = C::Buf;

    fn check_for_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError> {
        Box::pin(self.with_conn(move |conn| conn.check_for_raw_error(sequence)))
    }

    fn discard_reply(&self, sequence: SequenceNumber, kind: RequestKind, mode: DiscardMode) {
        // Doesn't block.
        self.inner.discard_reply(sequence, kind, mode);
    }

    fn extension_information(
        &self,
        name: &'static str,
    ) -> Fut<'_, Option<x11rb::x11_utils::ExtensionInformation>, ConnectionError> {
        Box::pin(self.with_conn(move |conn| conn.extension_information(name)))
    }

    fn prefetch_extension_information(&self, name: &'static str) -> Fut<'_, (), ConnectionError> {
        Box::pin(self.with_conn(move |conn| conn.prefetch_extension_information(name)))
    }

    fn maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = usize> + Send + '_>> {
        Box::pin(self.with_conn(|conn| conn.maximum_request_bytes()))
    }

    fn prefetch_maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(self.with_conn(|conn| conn.prefetch_maximum_request_bytes()))
    }

    fn parse_error(&self, error: &[u8]) -> Result<X11Error, ParseError> {
        // Doesn't block.
        self.inner.parse_error(error)
    }

    fn parse_event(&self, event: &[u8]) -> Result<x11rb::protocol::Event, ParseError> {
        // Doesn't block.
        self.inner.parse_event(event)
    }

    fn send_request_with_reply<'this, 'bufs, 'sl, 're, 'future, R>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<x11rb_protocol::RawFdContainer>,
    ) -> Fut<'future, crate::Cookie<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        're: 'future,
        R: x11rb::x11_utils::TryParse + Send + 're,
    {
        let mut buf = Vec::with_capacity(bufs.iter().map(|b| b.len()).sum());
        for b in bufs {
            buf.extend_from_slice(b);
        }

        Box::pin(async move {
            let res = self
                .with_conn(move |conn| {
                    let slices = [IoSlice::new(&buf)];
                    let cookie = conn.send_request_with_reply::<R>(&slices, fds)?;

                    let sequence = {
                        let sequence = cookie.sequence_number();
                        mem::forget(cookie);
                        sequence
                    };

                    Ok::<_, ConnectionError>(sequence)
                })
                .await?;

            Ok(crate::Cookie::new(self, res))
        })
    }

    fn send_request_with_reply_with_fds<'this, 'bufs, 'sl, 're, 'future, R>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<x11rb_protocol::RawFdContainer>,
    ) -> Fut<'future, crate::CookieWithFds<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        're: 'future,
        R: x11rb::x11_utils::TryParseFd + Send + 're,
    {
        let mut buf = Vec::with_capacity(bufs.iter().map(|b| b.len()).sum());
        for b in bufs {
            buf.extend_from_slice(b);
        }

        Box::pin(async move {
            let res = self
                .with_conn(move |conn| {
                    let slices = [IoSlice::new(&buf)];
                    let cookie = conn.send_request_with_reply_with_fds::<R>(&slices, fds)?;

                    let sequence = {
                        let sequence = cookie.sequence_number();
                        mem::forget(cookie);
                        sequence
                    };

                    Ok::<_, ConnectionError>(sequence)
                })
                .await?;

            Ok(crate::CookieWithFds::new(self, res))
        })
    }

    fn send_request_without_reply<'this, 'bufs, 'sl, 'future>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<x11rb_protocol::RawFdContainer>,
    ) -> Fut<'future, crate::VoidCookie<'this, Self>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
    {
        let mut buf = Vec::with_capacity(bufs.iter().map(|b| b.len()).sum());
        for b in bufs {
            buf.extend_from_slice(b);
        }

        Box::pin(async move {
            let res = self
                .with_conn(move |conn| {
                    let slices = [IoSlice::new(&buf)];
                    let cookie = conn.send_request_without_reply(&slices, fds)?;

                    let sequence = {
                        let sequence = cookie.sequence_number();
                        mem::forget(cookie);
                        sequence
                    };

                    Ok::<_, ConnectionError>(sequence)
                })
                .await?;

            Ok(crate::VoidCookie::new(self, res))
        })
    }

    fn wait_for_reply(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError> {
        Box::pin(self.with_conn(move |conn| conn.wait_for_reply(sequence)))
    }

    fn wait_for_reply_or_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<Self::Buf>, ConnectionError> {
        Box::pin(self.with_conn(move |conn| conn.wait_for_reply_or_raw_error(sequence)))
    }

    fn wait_for_reply_with_fds_raw(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<x11rb::connection::BufWithFds<Self::Buf>, Self::Buf>, ConnectionError>
    {
        Box::pin(self.with_conn(move |conn| conn.wait_for_reply_with_fds_raw(sequence)))
    }
}

impl<C: BlConnection + Send + Sync + 'static> Connection for BlockingConnection<C> {
    fn poll_for_raw_event_with_sequence(
        &self,
    ) -> Result<Option<x11rb_protocol::RawEventAndSeqNumber<Self::Buf>>, ConnectionError> {
        // Doesn't block.
        self.inner.poll_for_raw_event_with_sequence()
    }

    fn wait_for_raw_event_with_sequence(
        &self,
    ) -> Fut<'_, x11rb_protocol::RawEventAndSeqNumber<Self::Buf>, ConnectionError> {
        Box::pin(self.with_conn(|conn| conn.wait_for_raw_event_with_sequence()))
    }

    fn generate_id(&self) -> Fut<'_, u32, ReplyOrIdError> {
        Box::pin(self.with_conn(|conn| conn.generate_id()))
    }

    fn flush(&self) -> Fut<'_, (), ConnectionError> {
        Box::pin(self.with_conn(|conn| conn.flush()))
    }

    fn setup(&self) -> &x11rb::protocol::xproto::Setup {
        self.inner.setup()
    }
}
