// This code is dual licensed under MIT OR Apache 2.0.

//! A connection implemented entirely in Rust.

mod nb_connect;

use crate::connection::{Connection, Fut, RequestConnection};
use crate::errors::{ConnectError, ConnectionError, ParseError, ReplyOrIdError};
use crate::util::AsIoError;
use crate::{
    Cookie, CookieWithFds, RawEventAndSeqNumber, RawFdContainer, ReplyOrError, SequenceNumber,
    Setup, VoidCookie,
};

use async_io::Async;
use async_lock::Mutex;
use futures_lite::future::poll_fn;
use futures_lite::prelude::*;

use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

#[cfg(unix)]
use std::os::unix::io::AsRawFd as AsRaw;
#[cfg(windows)]
use std::os::windows::io::AsRawSocket as AsRaw;

#[doc(inline)]
pub use x11rb::rust_connection::{DefaultStream, Stream};

use x11rb::connection::{Connection as _, RequestConnection as _};
use x11rb::rust_connection::{PollMode, RustConnection as InnerConnection};

scoped_tls::scoped_thread_local!(
    // Pass the context down to the connection.
    static CURRENT_CONTEXT: Waker
);

/// A connection implemented entirely in pure Rust.
pub struct RustConnection<S: Stream = DefaultStream> {
    /// The inner connection.
    inner: Arc<InnerConnection<WrappedStream<S>>>,

    /// Prevent more than one task from reading at once.
    read_lock: Mutex<()>,

    /// Prevent more than one task from writing at once.
    write_lock: Mutex<()>,
}

/// Wraps a stream registered in the reactor.
///
/// `CURRENT_CONTEXT` should be set when calling this; otherwise a panic may occur.
struct WrappedStream<S: Stream> {
    /// Register the stream into the reactor.
    stream: Async<S>,
}

impl<S: Stream> Stream for WrappedStream<S> {
    fn poll(&self, mode: PollMode) -> io::Result<()> {
        // Get the current context.
        CURRENT_CONTEXT.with(|waker| {
            let mut ctx = Context::from_waker(waker);

            // Poll the stream.
            let poll = match mode {
                PollMode::Readable => self.stream.poll_readable(&mut ctx),
                PollMode::Writable => self.stream.poll_writable(&mut ctx),
                PollMode::ReadAndWritable => {
                    // Poll both at once.
                    let readable = self.stream.poll_readable(&mut ctx);
                    let writable = self.stream.poll_writable(&mut ctx);

                    // Combine the polls.
                    match (readable, writable) {
                        (Poll::Ready(Ok(())), Poll::Ready(Ok(()))) => Poll::Ready(Ok(())),
                        (Poll::Ready(Err(e)), _) | (_, Poll::Ready(Err(e))) => Poll::Ready(Err(e)),
                        _ => Poll::Pending,
                    }
                }
            };

            // Convert the result.
            match poll {
                Poll::Ready(res) => res,
                Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
            }
        })
    }

    // Forward remaining methods down lower, it's non-blocking anyways.

    fn read(&self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> io::Result<usize> {
        self.stream.get_ref().read(buf, fd_storage)
    }

    fn read_exact(&self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> io::Result<()> {
        self.stream.get_ref().read_exact(buf, fd_storage)
    }

    fn write(&self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> io::Result<usize> {
        self.stream.get_ref().write(buf, fds)
    }

    fn write_vectored(
        &self,
        bufs: &[io::IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> io::Result<usize> {
        self.stream.get_ref().write_vectored(bufs, fds)
    }
}

impl RustConnection {
    /// Connect to the X11 server.
    pub async fn connect(dpy_name: Option<&str>) -> Result<(Self, usize), ConnectError> {
        let (stream, screen) = nb_connect::connect(dpy_name).await?;
        Ok((Self::connect_to_stream(stream, screen).await?, screen))
    }
}

impl<S: Stream + AsRaw> RustConnection<S> {
    /// Connect to the X11 server using the given stream.
    pub async fn connect_to_stream(stream: S, screen: usize) -> Result<Self, ConnectError> {
        Self::connect_to_stream_with_auth_info(stream, screen, Vec::new(), Vec::new()).await
    }

    /// Connect to the X11 server using the given stream and authentication information.
    pub async fn connect_to_stream_with_auth_info(
        stream: S,
        screen: usize,
        auth_name: Vec<u8>,
        auth_data: Vec<u8>,
    ) -> Result<Self, ConnectError> {
        // Register the stream in the reactor.
        let stream = WrappedStream {
            stream: Async::new(stream)?,
        };

        // Set up the connection.
        let (mut connect, setup_request) =
            x11rb_protocol::connect::Connect::with_authorization(auth_name, auth_data);

        // Write the setup request.
        let mut fds = Vec::new();
        let mut nwritten = 0;

        while nwritten < setup_request.len() {
            nwritten += match stream.write(&setup_request[nwritten..], &mut fds) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => 0,
                Ok(n) => n,
                Err(e) => return Err(e.into()),
            };

            // Wait for the stream to be writable.
            stream.stream.writable().await?;
        }

        // Read in the setup.
        loop {
            // Read in the setup.
            let adv = match stream.read(connect.buffer(), &mut fds) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => 0,
                Ok(n) => n,
                Err(e) => return Err(e.into()),
            };

            // Advance the connection.
            if connect.advance(adv) {
                break;
            }

            // Wait for the stream to be readable.
            stream.stream.readable().await?;
        }

        // Resolve the setup.
        let setup = connect.into_setup()?;

        // Make sure our screen is valid.
        if screen >= setup.roots.len() {
            return Err(ConnectError::InvalidScreen);
        }

        Ok(Self {
            inner: Arc::new(InnerConnection::for_connected_stream(stream, setup)?),
            read_lock: Mutex::new(()),
            write_lock: Mutex::new(()),
        })
    }

    /// Establish a new connection on an already connected stream.
    pub fn for_connected_stream(stream: S, setup: Setup) -> Result<Self, ConnectError> {
        // Register the stream in the reactor.
        let stream = WrappedStream {
            stream: Async::new(stream)?,
        };

        Ok(Self {
            inner: Arc::new(InnerConnection::for_connected_stream(stream, setup)?),
            read_lock: Mutex::new(()),
            write_lock: Mutex::new(()),
        })
    }
}

impl<S: Stream> RustConnection<S> {
    /// Read or write asynchronously with the given function.
    async fn do_io<R, E: AsIoError>(
        &self,
        mode: PollMode,
        mut f: impl FnMut(&Self) -> Result<R, E>,
    ) -> Result<R, E> {
        loop {
            // Try the operation.
            let res = poll_fn(|cx| {
                let res = CURRENT_CONTEXT.set(cx.waker(), || f(self));
                Poll::Ready(res)
            })
            .await;

            match res {
                Err(e) if e.as_io_error().map(|e| e.kind()) == Some(io::ErrorKind::WouldBlock) => {}
                res => return res,
            }

            // Wait until the stream is readable or writable.
            match mode {
                PollMode::Readable => self
                    .inner
                    .stream()
                    .stream
                    .readable()
                    .await
                    .map_err(E::from_io_error)?,
                PollMode::Writable => self
                    .inner
                    .stream()
                    .stream
                    .writable()
                    .await
                    .map_err(E::from_io_error)?,
                PollMode::ReadAndWritable => {
                    // `or` them together.
                    let read = self.inner.stream().stream.readable();
                    let write = self.inner.stream().stream.writable();

                    read.or(write).await.map_err(E::from_io_error)?;
                }
            }
        }
    }
}

impl<S: Stream + Send + Sync + 'static> RequestConnection for RustConnection<S> {
    type Buf = Vec<u8>;

    fn send_request_with_reply<'this, 'bufs, 'sl, 'future, R>(
        &'this self,
        _bufs: &'bufs [io::IoSlice<'sl>],
        _fds: Vec<RawFdContainer>,
    ) -> Fut<'future, Cookie<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        R: x11rb::x11_utils::TryParse,
    {
        Box::pin(async move {
            // Gain exclusive access to the write end of the stream.
            let _guard = self.write_lock.lock().await;

            todo!()
        })
    }

    fn send_request_with_reply_with_fds<'this, 'bufs, 'sl, 'future, R>(
        &'this self,
        _bufs: &'bufs [io::IoSlice<'sl>],
        _fds: Vec<RawFdContainer>,
    ) -> Fut<'future, CookieWithFds<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        R: x11rb::x11_utils::TryParseFd,
    {
        todo!()
    }

    fn send_request_without_reply<'this, 'bufs, 'sl, 'future>(
        &'this self,
        _bufs: &'bufs [io::IoSlice<'sl>],
        _fds: Vec<RawFdContainer>,
    ) -> Fut<'future, VoidCookie<'this, Self>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
    {
        todo!()
    }

    fn check_for_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError> {
        Box::pin(async move {
            // Gain exclusive access to the read end of the stream.
            let _guard = self.read_lock.lock().await;

            self.do_io(PollMode::Readable, |conn| {
                conn.inner.check_for_raw_error(sequence)
            })
            .await
        })
    }

    fn discard_reply(
        &self,
        sequence: SequenceNumber,
        kind: x11rb::connection::RequestKind,
        mode: x11rb_protocol::DiscardMode,
    ) {
        self.inner.discard_reply(sequence, kind, mode)
    }

    fn parse_error(&self, error: &[u8]) -> Result<x11rb::x11_utils::X11Error, ParseError> {
        self.inner.parse_error(error)
    }

    fn parse_event(&self, event: &[u8]) -> Result<x11rb::protocol::Event, ParseError> {
        self.inner.parse_event(event)
    }

    fn prefetch_extension_information(&self, name: &'static str) -> Fut<'_, (), ConnectionError> {
        Box::pin(async move {
            // Gain exclusive access to the write end of the stream.
            let _guard = self.write_lock.lock().await;

            self.do_io(PollMode::Writable, move |conn| {
                conn.inner.prefetch_extension_information(name)
            })
            .await
        })
    }

    fn extension_information(
        &self,
        name: &'static str,
    ) -> Fut<'_, Option<x11rb::x11_utils::ExtensionInformation>, ConnectionError> {
        Box::pin(async move {
            // Gain exclusive access to the write end of the stream.
            let _guard = self.write_lock.lock().await;

            self.do_io(PollMode::ReadAndWritable, |conn| {
                conn.inner.extension_information(name)
            })
            .await
        })
    }

    fn maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = usize> + Send + '_>> {
        Box::pin(async move {
            // Gain exclusive access to both ends.
            let _guard1 = self.write_lock.lock().await;
            let _guard2 = self.read_lock.lock().await;

            // No real way to do this asynchronously.
            let inner = self.inner.clone();
            blocking::unblock(move || inner.maximum_request_bytes()).await
        })
    }

    fn prefetch_maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            // Gain exclusive access to the write end.
            let _guard1 = self.write_lock.lock().await;

            // No real way to do this asynchronously.
            let inner = self.inner.clone();
            blocking::unblock(move || inner.prefetch_maximum_request_bytes()).await
        })
    }

    fn wait_for_reply(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError> {
        Box::pin(async move {
            // Gain exclusive access to the read end of the stream.
            let _guard = self.read_lock.lock().await;

            self.do_io(PollMode::Readable, |conn| {
                conn.inner.wait_for_reply(sequence)
            })
            .await
        })
    }

    fn wait_for_reply_or_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<Self::Buf>, ConnectionError> {
        Box::pin(async move {
            // Gain exclusive access to the read end of the stream.
            let _guard = self.read_lock.lock().await;

            self.do_io(PollMode::Readable, |conn| {
                conn.inner.wait_for_reply_or_raw_error(sequence)
            })
            .await
        })
    }

    fn wait_for_reply_with_fds_raw(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<x11rb::connection::BufWithFds<Self::Buf>, Self::Buf>, ConnectionError>
    {
        Box::pin(async move {
            // Gain exclusive access to the read end of the stream.
            let _guard = self.read_lock.lock().await;

            self.do_io(PollMode::Readable, |conn| {
                conn.inner.wait_for_reply_with_fds_raw(sequence)
            })
            .await
        })
    }
}

impl<S: Stream + Send + Sync + 'static> Connection for RustConnection<S> {
    fn poll_for_raw_event_with_sequence(
        &self,
    ) -> Result<Option<RawEventAndSeqNumber<Self::Buf>>, ConnectionError> {
        // Try to gain access to the read end of the stream.
        let _guard = match self.read_lock.try_lock() {
            Some(guard) => guard,
            None => return Ok(None),
        };

        // This never blocks.
        self.inner.poll_for_raw_event_with_sequence()
    }

    fn wait_for_raw_event_with_sequence(
        &self,
    ) -> Fut<'_, x11rb_protocol::RawEventAndSeqNumber<Self::Buf>, ConnectionError> {
        Box::pin(async move {
            // Gain exclusive access to the read end of the stream.
            let _guard = self.read_lock.lock().await;

            self.do_io(PollMode::Readable, |conn| {
                conn.inner.wait_for_raw_event_with_sequence()
            })
            .await
        })
    }

    fn setup(&self) -> &Setup {
        // Never blocks.
        self.inner.setup()
    }

    fn flush(&self) -> Fut<'_, (), ConnectionError> {
        Box::pin(async move {
            // Gain exclusive access to the write end of the stream.
            let _guard = self.write_lock.lock().await;

            self.do_io(PollMode::Writable, |conn| conn.inner.flush())
                .await
        })
    }

    fn generate_id(&self) -> Fut<'_, u32, ReplyOrIdError> {
        Box::pin(async move {
            // Gain exclusive access to both ends of the stream.
            let _guard1 = self.read_lock.lock().await;
            let _guard2 = self.write_lock.lock().await;

            self.do_io(PollMode::ReadAndWritable, |conn| conn.inner.generate_id())
                .await
        })
    }
}
