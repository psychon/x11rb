//! Implements the `Stream` trait for `RustConnection`.

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(unix)]
use std::os::unix::io::{AsRawFd as AsRaw, RawFd};

#[cfg(windows)]
use std::os::windows::io::{AsRawSocket as AsRaw, RawSocket};

use async_io::Async;
use futures_lite::future;

use x11rb::rust_connection::{
    DefaultStream as X11rbDefaultStream, PollMode, Stream as X11rbStream,
};
use x11rb::utils::RawFdContainer;

/// A stream that bytes can be read from or written to.
pub trait StreamBase<'a>: X11rbStream {
    /// The future returned by `readable`.
    type Readable: Future<Output = io::Result<()>> + Send + 'a;

    /// The future returned by `writable`.
    type Writable: Future<Output = io::Result<()>> + Send + 'a;

    /// Wait until the stream is readable.
    fn readable(&'a self) -> Self::Readable;

    /// Wait until the stream is writable.
    fn writable(&'a self) -> Self::Writable;
}

/// A stream that bytes can be read from or written to.
pub trait Stream: for<'a> StreamBase<'a> {}
impl<S: for<'a> StreamBase<'a>> Stream for S {}

/// The default stream type.
pub type DefaultStream = StreamAdaptor<X11rbDefaultStream>;

/// An adaptor that implements a `Stream` for a type that implements `X11rbStream`.
pub struct StreamAdaptor<S> {
    inner: Async<S>,
}

impl<S: AsRaw> StreamAdaptor<S> {
    /// Create a new `StreamAdaptor` from a stream.
    pub fn new(stream: S) -> io::Result<Self> {
        Async::new(stream).map(|inner| Self { inner })
    }
}

impl<S> StreamAdaptor<S> {
    /// Get a reference to the inner stream.
    pub fn get_ref(&self) -> &S {
        self.inner.get_ref()
    }

    /// Get a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut S {
        self.inner.get_mut()
    }

    /// Consume this adaptor and return the inner stream.
    pub fn into_inner(self) -> io::Result<S> {
        self.inner.into_inner()
    }
}

impl<S: AsRaw> AsRaw for StreamAdaptor<S> {
    #[cfg(unix)]
    fn as_raw_fd(&self) -> RawFd {
        self.inner.get_ref().as_raw_fd()
    }

    #[cfg(windows)]
    fn as_raw_socket(&self) -> RawSocket {
        self.inner.get_ref().as_raw_socket()
    }
}

/// A future for reading from a [`StreamAdaptor`].
#[derive(Debug)]
pub struct Readable<'a, S>(async_io::Readable<'a, S>);

impl<S> Unpin for Readable<'_, S> {}

impl<S> Future for Readable<'_, S> {
    type Output = io::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

/// A future for writing to a [`StreamAdaptor`].
#[derive(Debug)]
pub struct Writable<'a, S>(async_io::Writable<'a, S>);

impl<S> Unpin for Writable<'_, S> {}

impl<S> Future for Writable<'_, S> {
    type Output = io::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

impl<'a, S: 'a + X11rbStream + Sync> StreamBase<'a> for StreamAdaptor<S> {
    type Readable = Readable<'a, S>;
    type Writable = Writable<'a, S>;

    fn readable(&'a self) -> Self::Readable {
        Readable(self.inner.readable())
    }

    fn writable(&'a self) -> Self::Writable {
        Writable(self.inner.writable())
    }
}

impl<S: X11rbStream> X11rbStream for StreamAdaptor<S> {
    fn poll(&self, mode: PollMode) -> io::Result<()> {
        use async_io::block_on;

        // Block on the necessary futures.
        match mode {
            PollMode::Readable => block_on(self.inner.readable()),
            PollMode::Writable => block_on(self.inner.writable()),
            PollMode::ReadAndWritable => {
                block_on(future::or(self.inner.readable(), self.inner.writable()))
            }
        }
    }

    fn read(&self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> io::Result<usize> {
        self.get_ref().read(buf, fd_storage)
    }

    fn write(&self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> io::Result<usize> {
        self.get_ref().write(buf, fds)
    }

    fn read_exact(&self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> io::Result<()> {
        self.get_ref().read_exact(buf, fd_storage)
    }

    fn write_vectored(
        &self,
        bufs: &[io::IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> io::Result<usize> {
        self.get_ref().write_vectored(bufs, fds)
    }
}
