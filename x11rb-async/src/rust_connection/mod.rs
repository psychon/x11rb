// This code is dual licensed under MIT OR Apache 2.0.

//! A connection implemented entirely in Rust.

mod nb_connect;

use crate::connection::{Connection, Fut, RequestConnection};
use crate::errors::{ConnectError, ConnectionError, ParseError, ReplyOrIdError};
use crate::{
    Cookie, CookieWithFds, RawEventAndSeqNumber, RawFdContainer, ReplyOrError, SequenceNumber,
    Setup, VoidCookie,
};

use async_io::Async;
use async_lock::{Mutex, MutexGuard};
use concurrent_queue::ConcurrentQueue;
use event_listener::Event;
use futures_lite::future;
use futures_lite::prelude::*;
use x11rb_protocol::DiscardMode;

use std::convert::Infallible;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(unix)]
use std::os::unix::io::AsRawFd as AsRaw;
#[cfg(windows)]
use std::os::windows::io::AsRawSocket as AsRaw;

#[doc(inline)]
pub use x11rb::rust_connection::{DefaultStream as X11rbDefaultStream, Stream as X11rbStream};

use x11rb::connection::{BufWithFds, Connection as _, RequestConnection as _, RequestKind};
use x11rb::rust_connection::{PollMode, RustConnection as InnerConnection};

use x11rb_protocol::connection::ReplyFdKind;
use x11rb_protocol::id_allocator::IdAllocator;

/// Streams that support non-blocking functionality.
pub trait Stream<'a>: X11rbStream {
    /// The future type returned by `readable`.
    type Readable: Future<Output = io::Result<()>> + Send + 'a;

    /// The future type returned by `writable`.
    type Writable: Future<Output = io::Result<()>> + Send + 'a;

    /// Wait for the stream to become readable.
    fn readable(&'a self) -> Self::Readable;

    /// Wait for the stream to become writable.
    fn writable(&'a self) -> Self::Writable;
}

/// The `Stream` trait, but it works for all lifetimes.
pub trait Unistream: for<'a> Stream<'a> {}
impl<T: for<'a> Stream<'a>> Unistream for T {}

/// A wrapper around `DefaultStream` that implements `Stream`.
///
/// ## Implementation
///
/// This is implemented by registering the stream into the reactor.
pub struct DefaultStream {
    /// Wrap `DefaultStream` in `Async`.
    stream: Async<X11rbDefaultStream>,
}

impl X11rbStream for DefaultStream {
    fn poll(&self, mode: PollMode) -> io::Result<()> {
        // Use `async-io::block_on` to wait for readiness.
        async_io::block_on(async move {
            match mode {
                PollMode::Readable => self.stream.readable().await,
                PollMode::Writable => self.stream.writable().await,
                PollMode::ReadAndWritable => {
                    // Wait for either to become ready.
                    let readable = self.stream.readable();
                    let writable = self.stream.writable();

                    // Combine the polls.
                    readable.or(writable).await
                }
            }
        })
    }

    // The other impls are non-blocking, so we can just forward them.

    fn read(&self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> io::Result<usize> {
        self.stream.get_ref().read(buf, fd_storage)
    }

    fn write(&self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> io::Result<usize> {
        self.stream.get_ref().write(buf, fds)
    }

    fn read_exact(
        &self,
        mut buf: &mut [u8],
        fd_storage: &mut Vec<RawFdContainer>,
    ) -> io::Result<()> {
        self.stream.get_ref().read_exact(&mut buf, fd_storage)
    }

    fn write_vectored(
        &self,
        bufs: &[io::IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> io::Result<usize> {
        self.stream.get_ref().write_vectored(bufs, fds)
    }
}

impl<'a> Stream<'a> for DefaultStream {
    type Readable = DefaultReadable<'a>;
    type Writable = DefaultWritable<'a>;

    fn readable(&'a self) -> Self::Readable {
        DefaultReadable(self.stream.readable())
    }

    fn writable(&'a self) -> Self::Writable {
        DefaultWritable(self.stream.writable())
    }
}

/// A future that means "wait for the stream to become readable".
pub struct DefaultReadable<'a>(async_io::Readable<'a, X11rbDefaultStream>);

impl Future for DefaultReadable<'_> {
    type Output = io::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

/// A future that means "wait for the stream to become writable".
pub struct DefaultWritable<'a>(async_io::Writable<'a, X11rbDefaultStream>);

impl Future for DefaultWritable<'_> {
    type Output = io::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

#[cfg(unix)]
impl AsRaw for DefaultStream {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.stream.get_ref().as_raw_fd()
    }
}

#[cfg(windows)]
impl AsRaw for DefaultStream {
    fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
        self.stream.get_ref().as_raw_socket()
    }
}

const BUFFER_SIZE: usize = 16834;

/// A connection implemented entirely in pure Rust.
pub struct RustConnection<S: Unistream = DefaultStream> {
    /// The underlying connection.
    inner: Box<InnerConnection<WrapperStream<S>>>,

    /// Only one task should be calling `drain()` at a time.
    drain_lock: Mutex<()>,

    /// Our buffer for writing.
    write_buffer: Mutex<WriteBuffer>,

    /// ID allocation mechanism.
    id_allocator: Mutex<IdAllocator>,

    /// Queue of pending events.
    event_queue: ConcurrentQueue<(Vec<u8>, u64)>,

    /// Notify when a new input is available.
    new_input: Event,
}

/// A `Stream` wrapper around another `Stream`.
struct WrapperStream<S>(S);

impl<S: Unistream> X11rbStream for WrapperStream<S> {
    fn poll(&self, _mode: PollMode) -> io::Result<()> {
        // Bubble back up to the user.
        Err(io::Error::from(io::ErrorKind::WouldBlock))
    }

    // Other calls are non-blocking, just bubble up from there.

    fn read(&self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> io::Result<usize> {
        self.0.read(buf, fd_storage)
    }

    fn read_exact(
        &self,
        mut buf: &mut [u8],
        fd_storage: &mut Vec<RawFdContainer>,
    ) -> io::Result<()> {
        self.0.read_exact(&mut buf, fd_storage)
    }

    fn write(&self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> io::Result<usize> {
        self.0.write(buf, fds)
    }

    fn write_vectored(
        &self,
        bufs: &[io::IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> io::Result<usize> {
        self.0.write_vectored(bufs, fds)
    }
}

struct WriteBuffer {
    /// The buffer of bytes.
    buffer: Vec<u8>,

    /// The file descriptors that are associated with this buffer.
    fds: Vec<RawFdContainer>,
}

impl RustConnection {
    /// Connect to the X11 server.
    pub async fn connect(dpy_name: Option<&str>) -> Result<(Self, usize), ConnectError> {
        let (stream, screen) = nb_connect::connect(dpy_name).await?;
        let stream = DefaultStream {
            stream: Async::new(stream)?,
        };

        Ok((Self::connect_to_stream(stream, screen).await?, screen))
    }
}

impl<S: Unistream + AsRaw> RustConnection<S> {
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
            stream.writable().await?;
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
            stream.readable().await?;
        }

        // Resolve the setup.
        let setup = connect.into_setup()?;

        // Make sure our screen is valid.
        if screen >= setup.roots.len() {
            return Err(ConnectError::InvalidScreen);
        }

        Self::for_connected_stream(stream, setup)
    }

    /// Establish a new connection on an already connected stream.
    pub fn for_connected_stream(stream: S, setup: Setup) -> Result<Self, ConnectError> {
        Ok(Self {
            id_allocator: Mutex::new(IdAllocator::new(
                setup.resource_id_base,
                setup.resource_id_mask,
            )?),
            inner: Box::new(InnerConnection::for_connected_stream(
                WrapperStream(stream),
                setup,
            )?),
            drain_lock: Mutex::new(()),
            write_buffer: Mutex::new(WriteBuffer {
                buffer: Vec::with_capacity(BUFFER_SIZE),
                fds: Vec::new(),
            }),
            event_queue: ConcurrentQueue::unbounded(),
            new_input: Event::new(),
        })
    }
}

impl<S: Unistream> RustConnection<S> {
    /// Run the connection logic.
    ///
    /// This future will read data from the stream and distribute the results to other calls.
    /// It is meant to be run as a background task, and will never return.
    pub async fn connection(&self) -> Result<Infallible, ConnectionError> {
        // Make sure we can only run one connection task at a time.
        let drain_lock = self.drain_lock.lock().await;

        // Run the connection logic.
        self.drive(drain_lock).await
    }

    /// Read from the I/O stream and buffer any events we receive.
    async fn drive(&self, drain_lock: MutexGuard<'_, ()>) -> Result<Infallible, ConnectionError> {
        let _guard = drain_lock;

        loop {
            let mut found_event = 0usize;

            // Calling `poll` will read in data from the stream and buffer it.
            // This function also handles replies and the like.
            // It will also yield an event; push that to our queue.
            'outer: loop {
                // Only read up to 200 events at a time
                for _ in 0..200 {
                    let event = self.inner.poll_for_raw_event_with_sequence()?;

                    if let Some(event) = event {
                        // Unbounded queue, this never fails.
                        self.event_queue.push(event).ok();
                        found_event = found_event.saturating_add(1);
                    } else {
                        break 'outer;
                    }
                }

                // Yield to give time to other tasks.
                future::yield_now().await;
            }

            // Wake up all tasks waiting on a read event.
            // This will wake up all threads waiting on an event; however, I expect this
            // number to be relatively low; it will be less of a thundering herd and
            // more of a gentle group stroll.
            self.new_input.notify_additional(std::usize::MAX);

            // Wait until there is a packet available on the stream.
            self.inner.stream().0.readable().await?;

            // Yield to the other task.
            future::yield_now().await;
        }
    }

    /// Run a future as a driven future.
    async fn driven<R, E: From<ConnectionError>>(
        &self,
        fut: impl Future<Output = Result<R, E>>,
    ) -> Result<R, E> {
        if let Some(lock) = self.drain_lock.try_lock() {
            // If we aren't driven right now, drive while we run the future.
            future::or(fut, async move {
                match self.drive(lock).await {
                    Ok(x) => match x {},
                    Err(e) => Err(E::from(e)),
                }
            })
            .await
        } else {
            // We are already driven, just run the future.
            fut.await
        }
    }

    /// Send a request.
    async fn send_request(
        &self,
        bufs: &[io::IoSlice<'_>],
        mut fds: Vec<RawFdContainer>,
        kind: ReplyFdKind,
    ) -> Result<SequenceNumber, ConnectionError> {
        // Compute the request.
        let mut storage = Default::default();
        let bufs = x11rb::connection::compute_length_field(&*self.inner, bufs, &mut storage)?;

        // Lock the buffer.
        let mut buffer = self.write_buffer.lock().await;

        loop {
            // Logically send the request.
            match self.inner.external_send_request(kind) {
                Some(seq) => {
                    // Write the request to the buffer.
                    self.write_all_vectored(bufs, &mut fds, buffer).await?;
                    return Ok(seq);
                }

                None => {
                    // Synchronize and try agan.
                    buffer = self.send_sync(buffer).await?;
                }
            }
        }
    }

    /// Send a request that catches us up to the current sequence number.
    async fn send_sync<'a>(
        &self,
        buffer: MutexGuard<'a, WriteBuffer>,
    ) -> Result<MutexGuard<'a, WriteBuffer>, ConnectionError> {
        let length = 1u16.to_ne_bytes();
        let request = [
            x11rb_protocol::protocol::xproto::GET_INPUT_FOCUS_REQUEST,
            0,
            length[0],
            length[1],
        ];

        // Send this request.
        let seq = self
            .inner
            .external_send_request(ReplyFdKind::ReplyWithoutFDs)
            .expect("This request should not be blocked by syncs");
        self.inner.discard_reply(
            seq,
            RequestKind::HasResponse,
            DiscardMode::DiscardReplyAndError,
        );

        // Write the entire packet.
        let iov = &[io::IoSlice::new(&request)];
        let mut fds = Vec::new();
        self.write_all_vectored(iov, &mut fds, buffer).await
    }

    /// Flush the write buffer.
    ///
    /// This function does not drain any queues.
    async fn flush_impl<'a>(
        &self,
        mut buffer: MutexGuard<'a, WriteBuffer>,
    ) -> Result<MutexGuard<'a, WriteBuffer>, ConnectionError> {
        // If we don't have anything to write, we are done.
        if buffer.buffer.is_empty() && buffer.fds.is_empty() {
            return Ok(buffer);
        }

        // Write the entire buffer.
        let mut position = 0;
        write_with(&self.inner.stream().0, {
            let buffer = &mut *buffer;
            move |stream| {
                while position < buffer.buffer.len() {
                    let n = stream.write(&buffer.buffer[position..], &mut buffer.fds)?;
                    if n == 0 {
                        return Err(io::Error::from(io::ErrorKind::WriteZero).into());
                    }

                    position += n;
                }

                Ok(())
            }
        })
        .await?;

        if !buffer.fds.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Some file descriptors were not sent",
            )
            .into());
        }

        buffer.buffer.clear();

        Ok(buffer)
    }

    async fn write_all_vectored<'a>(
        &self,
        mut iov: &[io::IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
        mut buffer: MutexGuard<'a, WriteBuffer>,
    ) -> Result<MutexGuard<'a, WriteBuffer>, ConnectionError> {
        // Get the total length of the buffers.
        let mut total_length = iov.iter().map(|x| x.len()).sum::<usize>();

        // If our write buffer is too small, flush it first.
        if buffer.buffer.len() + total_length > buffer.buffer.capacity() {
            buffer = self.flush_impl(buffer).await?;
        }

        // If it fits in our write buffer, write it there.
        if total_length < buffer.buffer.capacity() {
            for buf in iov {
                buffer.buffer.extend_from_slice(buf);
            }

            buffer.fds.append(fds);

            return Ok(buffer);
        }

        // Otherwise, write directly to the stream.
        let mut partial: &[u8] = &[];
        write_with(&self.inner.stream().0, |stream| {
            while total_length > 0 && !partial.is_empty() {
                // If the partial buffer is non-empty, write it.
                if !partial.is_empty() {
                    let n = stream.write(partial, fds)?;
                    if n == 0 {
                        return Err(io::Error::from(io::ErrorKind::WriteZero).into());
                    }

                    partial = &partial[n..];
                    total_length -= n;
                } else {
                    // Write the iov.
                    let mut n = stream.write_vectored(iov, fds)?;
                    if n == 0 {
                        return Err(io::Error::from(io::ErrorKind::WriteZero).into());
                    }

                    // Calculate how much we have left to go.
                    total_length -= n;
                    while n > 0 {
                        if n >= iov[0].len() {
                            n -= iov[0].len();
                            iov = &iov[1..];
                        } else {
                            partial = &iov[0][n..];
                            n = 0;
                        }
                    }
                }
            }

            Ok(())
        })
        .await?;

        Ok(buffer)
    }

    async fn wait_for_reply_with_fds_impl(
        &self,
        seq: SequenceNumber,
    ) -> Result<ReplyOrError<BufWithFds<Vec<u8>>, Vec<u8>>, ConnectionError> {
        // Flush the request.
        self.flush_impl(self.write_buffer.lock().await).await?;

        // Wait for the reply.
        loop {
            // Check if we have a reply.
            match self.inner.wait_for_reply_with_fds_raw(seq) {
                Ok(reply) => return Ok(reply),
                Err(ConnectionError::IoError(ref io)) if io.kind() == io::ErrorKind::WouldBlock => {
                }
                Err(err) => return Err(err),
            }

            // Wait for the stream to become readable.
            let listener = self.new_input.listen();

            // Another reply may have been received in the meantime.
            match self.inner.wait_for_reply_with_fds_raw(seq) {
                Ok(reply) => return Ok(reply),
                Err(ConnectionError::IoError(ref io)) if io.kind() == io::ErrorKind::WouldBlock => {
                }
                Err(err) => return Err(err),
            }

            listener.await;
        }
    }

    async fn wait_for_reply_impl(
        &self,
        seq: SequenceNumber,
    ) -> Result<Option<Vec<u8>>, ConnectionError> {
        // Flush the request.
        self.flush_impl(self.write_buffer.lock().await).await?;

        // Wait for the reply.
        loop {
            // Check if we have a reply.
            match self.inner.wait_for_reply(seq) {
                Ok(reply) => return Ok(reply),
                Err(ConnectionError::IoError(ref io)) if io.kind() == io::ErrorKind::WouldBlock => {
                }
                Err(err) => return Err(err),
            }

            // Wait for the stream to become readable.
            let listener = self.new_input.listen();

            // Another reply may have been received in the meantime.
            match self.inner.wait_for_reply(seq) {
                Ok(reply) => return Ok(reply),
                Err(ConnectionError::IoError(ref io)) if io.kind() == io::ErrorKind::WouldBlock => {
                }
                Err(err) => return Err(err),
            }

            listener.await;
        }
    }
}

impl<S: Unistream + Send + Sync + 'static> RequestConnection for RustConnection<S> {
    type Buf = Vec<u8>;

    fn send_request_with_reply<'this, 'bufs, 'sl, 're, 'future, R>(
        &'this self,
        bufs: &'bufs [io::IoSlice<'sl>],
        fds: Vec<RawFdContainer>,
    ) -> Fut<'future, Cookie<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        're: 'future,
        R: x11rb::x11_utils::TryParse + Send + 're,
    {
        Box::pin(self.driven(async move {
            let seq = self
                .send_request(bufs, fds, ReplyFdKind::ReplyWithoutFDs)
                .await?;

            Ok(Cookie::new(self, seq))
        }))
    }

    fn send_request_with_reply_with_fds<'this, 'bufs, 'sl, 're, 'future, R>(
        &'this self,
        bufs: &'bufs [io::IoSlice<'sl>],
        fds: Vec<RawFdContainer>,
    ) -> Fut<'future, CookieWithFds<'this, Self, R>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
        're: 'future,
        R: x11rb::x11_utils::TryParseFd + Send + 're,
    {
        Box::pin(self.driven(async move {
            let seq = self
                .send_request(bufs, fds, ReplyFdKind::ReplyWithFDs)
                .await?;

            Ok(CookieWithFds::new(self, seq))
        }))
    }

    fn send_request_without_reply<'this, 'bufs, 'sl, 'future>(
        &'this self,
        bufs: &'bufs [io::IoSlice<'sl>],
        fds: Vec<RawFdContainer>,
    ) -> Fut<'future, VoidCookie<'this, Self>, ConnectionError>
    where
        'this: 'future,
        'bufs: 'future,
        'sl: 'future,
    {
        Box::pin(self.driven(async move {
            let seq = self.send_request(bufs, fds, ReplyFdKind::NoReply).await?;

            Ok(VoidCookie::new(self, seq))
        }))
    }

    fn check_for_raw_error(
        &self,
        seq: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError> {
        Box::pin(self.driven(async move {
            let mut buffer = self.write_buffer.lock().await;

            // Prepare for a reply.
            if self.inner.external_prepare_for_reply(seq) {
                // Send a synchronization packet.
                buffer = self.send_sync(buffer).await?;
            }

            // Flush the request.
            self.flush_impl(buffer).await?;

            loop {
                // Check for results.
                match self.inner.check_for_raw_error(seq) {
                    Ok(result) => return Ok(result),
                    Err(ConnectionError::IoError(e)) if e.kind() == io::ErrorKind::WouldBlock => {}
                    Err(e) => return Err(e.into()),
                }

                // Wait for the stream to become readable.
                let listener = self.new_input.listen();

                // Another reply may have been received in the meantime.
                match self.inner.check_for_raw_error(seq) {
                    Ok(result) => return Ok(result),
                    Err(ConnectionError::IoError(e)) if e.kind() == io::ErrorKind::WouldBlock => {}
                    Err(e) => return Err(e.into()),
                }

                listener.await;
            }
        }))
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
        // This shouldn't block.
        self.inner.parse_error(error)
    }

    fn parse_event(&self, event: &[u8]) -> Result<x11rb::protocol::Event, ParseError> {
        // This shouldn't block.
        self.inner.parse_event(event)
    }

    fn prefetch_extension_information(&self, name: &'static str) -> Fut<'_, (), ConnectionError> {
        Box::pin(self.driven(async move { todo!() }))
    }

    fn extension_information(
        &self,
        name: &'static str,
    ) -> Fut<'_, Option<x11rb::x11_utils::ExtensionInformation>, ConnectionError> {
        Box::pin(async move { todo!() })
    }

    fn maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = usize> + Send + '_>> {
        Box::pin(async move { todo!() })
    }

    fn prefetch_maximum_request_bytes(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move { todo!() })
    }

    fn wait_for_reply(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, Option<Self::Buf>, ConnectionError> {
        Box::pin(self.driven(self.wait_for_reply_impl(sequence)))
    }

    fn wait_for_reply_or_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<Self::Buf>, ConnectionError> {
        Box::pin(self.driven(async move {
            match self.wait_for_reply_with_fds_impl(sequence).await? {
                ReplyOrError::Reply((buf, _)) => Ok(ReplyOrError::Reply(buf)),
                ReplyOrError::Error(buf) => Ok(ReplyOrError::Error(buf)),
            }
        }))
    }

    fn wait_for_reply_with_fds_raw(
        &self,
        sequence: SequenceNumber,
    ) -> Fut<'_, ReplyOrError<BufWithFds<Self::Buf>, Self::Buf>, ConnectionError> {
        Box::pin(self.driven(self.wait_for_reply_with_fds_impl(sequence)))
    }
}

impl<S: Unistream + Send + Sync + 'static> Connection for RustConnection<S> {
    fn poll_for_raw_event_with_sequence(
        &self,
    ) -> Result<Option<RawEventAndSeqNumber<Self::Buf>>, ConnectionError> {
        // See if there is a raw event in the queue.
        if let Ok(event) = self.event_queue.pop() {
            return Ok(Some(event));
        }

        // Poll the stream exactly once.
        future::block_on(self.driven(async move {
            future::yield_now().await;
            Ok::<_, ConnectionError>(())
        }))?;

        // Check for a raw event.
        Ok(self.event_queue.pop().ok())
    }

    fn wait_for_raw_event_with_sequence(
        &self,
    ) -> Fut<'_, x11rb_protocol::RawEventAndSeqNumber<Self::Buf>, ConnectionError> {
        Box::pin(self.driven(async move {
            loop {
                // See if there is a raw event in the queue.
                if let Ok(event) = self.event_queue.pop() {
                    return Ok(event);
                }

                // Register a listener for a new raw event.
                let new_event = self.new_input.listen();

                // Yield once to allow the event queue to be filled.
                future::yield_now().await;

                // While we were registering the listener, a new event might have been added to the
                // queue. Check for that.
                if let Ok(event) = self.event_queue.pop() {
                    return Ok(event);
                }

                // Wait for a new event.
                new_event.await;
            }
        }))
    }

    fn setup(&self) -> &Setup {
        // Never blocks.
        self.inner.setup()
    }

    fn flush(&self) -> Fut<'_, (), ConnectionError> {
        Box::pin(self.driven(async move {
            let buffer = self.write_buffer.lock().await;
            self.flush_impl(buffer).await?;
            Ok(())
        }))
    }

    fn generate_id(&self) -> Fut<'_, u32, ReplyOrIdError> {
        Box::pin(async move {
            // Try to generate an ID.
            let mut id_allocator = self.id_allocator.lock().await;

            if let Some(id) = id_allocator.generate_id() {
                return Ok(id);
            }

            // Reset the ID range.
            // TODO

            Err(ReplyOrIdError::IdsExhausted)
        })
    }
}

async fn write_with<'a, S: Stream<'a>, R, F>(stream: &'a S, mut f: F) -> Result<R, ConnectionError>
where
    F: FnMut(&S) -> Result<R, ConnectionError>,
{
    loop {
        // Try to run the operation.
        match f(stream) {
            Err(ConnectionError::IoError(ref e)) if e.kind() == io::ErrorKind::WouldBlock => {
                // The operation would block. Wait for the stream to become writable.
                stream.writable().await?;
            }
            res => return res,
        }
    }
}

trait AsIoError {
    fn as_io_error(&self) -> Option<&io::Error>;
    fn from_io_error(e: io::Error) -> Self;
}

impl AsIoError for io::Error {
    fn as_io_error(&self) -> Option<&io::Error> {
        Some(self)
    }

    fn from_io_error(e: io::Error) -> Self {
        e
    }
}

impl AsIoError for ConnectionError {
    fn as_io_error(&self) -> Option<&io::Error> {
        match self {
            Self::IoError(ref e) => Some(e),
            _ => None,
        }
    }

    fn from_io_error(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl AsIoError for ReplyOrIdError {
    fn as_io_error(&self) -> Option<&io::Error> {
        match self {
            Self::ConnectionError(ConnectionError::IoError(ref e)) => Some(e),
            _ => None,
        }
    }

    fn from_io_error(e: io::Error) -> Self {
        Self::ConnectionError(ConnectionError::IoError(e))
    }
}
