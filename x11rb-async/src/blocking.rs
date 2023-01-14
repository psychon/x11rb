// This code is dual licensed under MIT OR Apache 2.0.

//! A `Connection` implementation that uses a threadpool to handle requests.

use crate::connection::{Connection, Fut, RequestConnection};
use crate::errors::{ConnectionError, ParseError, ReplyOrIdError};
use crate::{x11_utils::X11Error, SequenceNumber};

use std::future::Future;
use std::io::IoSlice;
use std::mem;
use std::pin::Pin;
use std::sync::Arc;
use x11rb::connection::{Connection as BlConnection, ReplyOrError, RequestKind};
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
    pub fn new(conn: C) -> Self {
        Self {
            inner: Arc::new(conn),
        }
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
        Box::pin(self.with_conn(move |conn| {
            let name = name;
            conn.extension_information(name)
        }))
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

    fn send_request_with_reply<'this, 'bufs, 'sl, 'future, R>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<x11rb_protocol::RawFdContainer>,
    ) -> Fut<'future, crate::Cookie<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        R: x11rb::x11_utils::TryParse + Send,
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

    fn send_request_with_reply_with_fds<'this, 'bufs, 'sl, 'future, R>(
        &'this self,
        bufs: &'bufs [IoSlice<'sl>],
        fds: Vec<x11rb_protocol::RawFdContainer>,
    ) -> Fut<'future, crate::CookieWithFds<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        R: x11rb::x11_utils::TryParseFd + Send,
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
    fn poll_for_raw_event(
        &self,
    ) -> Result<Option<x11rb_protocol::RawEventAndSeqNumber<Self::Buf>>, ConnectionError> {
        // Doesn't block.
        self.inner.poll_for_raw_event_with_sequence()
    }

    fn wait_for_raw_event(
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
