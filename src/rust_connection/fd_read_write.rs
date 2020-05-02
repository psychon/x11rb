//! Types for reading/writing bytes and FDs from a source.
//!
//! This module contains variants of `std::io::Read` and `std::io::Write` that also support passing
//! file descriptors.

use std::io::{Error, ErrorKind, IoSlice, Read, Result, Write};

use crate::utils::RawFdContainer;

/// A version of [`std::io::Write`] that also allows sending file descriptors.
pub trait WriteFD {
    /// Write a buffer and some FDs into this writer, returning how many bytes were written.
    ///
    /// This function works like [`std::io::Write::write`], but also supports sending file
    /// descriptors. The `fds` argument contains the file descriptors to send. The order of file
    /// descriptors is maintained.
    ///
    /// This function does not guarantee that all file descriptors are sent together with the data.
    /// Any file descriptors that were sent are removed from the beginning of the given `Vec`.
    ///
    /// There is no guarantee that the given file descriptors are received together with the given
    /// data. File descriptors might be received earlier than their corresponding data. It is not
    /// allowed for file descriptors to be received later than the bytes that were sent at the same
    /// time.
    fn write(&mut self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize>;

    /// Write an entire buffer of data and file descriptors into this writer.
    ///
    /// This function works like [`std::io::Write::write_all`], but also supports sending file
    /// descriptors. The `fds` argument contains the file descriptors to send. The order of file
    /// descriptors is maintained.
    ///
    /// When this function returns, all file descriptors were written.
    ///
    /// This function does not guarantee that all file descriptors are sent together with the data.
    /// Any file descriptors that were sent are removed from the beginning of the given `Vec`.
    ///
    /// There is no guarantee that the given file descriptors are received together with the given
    /// data. File descriptors might be received earlier than their corresponding data. It is not
    /// allowed for file descriptors to be received later than the bytes that were sent at the same
    /// time.
    fn write_all(&mut self, mut buf: &[u8], mut fds: Vec<RawFdContainer>) -> Result<()> {
        while !buf.is_empty() || !fds.is_empty() {
            let old_fds_len = fds.len();
            match self.write(buf, &mut fds) {
                Ok(0) if fds.len() == old_fds_len => {
                    return Err(Error::new(
                        ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    ));
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Like `write`, except that it writes from a slice of buffers.
    ///
    /// This method must behave as a call to `write` with the buffers concatenated would.
    ///
    /// The default implementation calls `write` with the first nonempty buffer provided.
    fn write_vectored(
        &mut self,
        bufs: &[IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> Result<usize> {
        for buf in bufs {
            if !buf.is_empty() {
                return self.write(&**buf, fds);
            }
        }
        Ok(0)
    }

    /// Flush this output stream, ensuring that all buffered contents are written out.
    fn flush(&mut self) -> Result<()>;
}

/// Wraps a [`std::io::Write`] to implement the [`WriteFD`] trait.
///
/// Any attempts to write file descriptors will fail. Bytes are forwarded to the underlying writer.
#[derive(Debug)]
pub struct WriteFDWrapper<W: Write + std::fmt::Debug>(W);

impl<W: Write + std::fmt::Debug> WriteFDWrapper<W> {
    /// Create a new `WriteFDWrapper` for the given writer.
    pub fn new(write: W) -> Self {
        Self(write)
    }
}

impl<W: Write + std::fmt::Debug> WriteFD for WriteFDWrapper<W> {
    fn write(&mut self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
        check_no_fds(fds)?;
        self.0.write(buf)
    }

    fn write_all(&mut self, bufs: &[u8], fds: Vec<RawFdContainer>) -> Result<()> {
        check_no_fds(&fds)?;
        self.0.write_all(bufs)
    }

    fn write_vectored(
        &mut self,
        bufs: &[IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> Result<usize> {
        check_no_fds(&fds)?;
        self.0.write_vectored(bufs)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}

// Check that the given argument is an empty slice
fn check_no_fds(fds: &[RawFdContainer]) -> Result<()> {
    if !fds.is_empty() {
        Err(Error::new(ErrorKind::Other, "FD passing is unsupported"))
    } else {
        Ok(())
    }
}

/// A version of [`std::io::BufWriter`] that supports sending file descriptors.
#[derive(Debug)]
pub struct BufWriteFD<W: WriteFD + std::fmt::Debug> {
    inner: W,
    data_buf: Vec<u8>,
    fd_buf: Vec<RawFdContainer>,
}

impl<W: WriteFD + std::fmt::Debug> BufWriteFD<W> {
    /// Creates a new `BufWriteFD` with a default buffer capacity.
    pub fn new(inner: W) -> Self {
        // Chosen by checking what libxcb does
        let default = 16384;
        Self::with_capacity(default, inner)
    }

    /// Creates a new `BufWriteFD` with the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: W) -> Self {
        Self {
            inner,
            data_buf: Vec::with_capacity(capacity),
            fd_buf: Vec::new(),
        }
    }

    fn flush_buffer(&mut self) -> Result<()> {
        let mut written = 0;
        let mut ret = Ok(());
        while written < self.data_buf.len() || !self.fd_buf.is_empty() {
            match self
                .inner
                .write(&self.data_buf[written..], &mut self.fd_buf)
            {
                Ok(0) => {
                    if written == self.data_buf.len() {
                        assert!(!self.fd_buf.is_empty());
                        ret = Err(Error::new(
                            ErrorKind::WriteZero,
                            "failed to write the buffered FDs",
                        ));
                    } else {
                        ret = Err(Error::new(
                            ErrorKind::WriteZero,
                            "failed to write the buffered data",
                        ));
                    }
                    break;
                }
                Ok(n) => written += n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => {
                    ret = Err(e);
                    break;
                }
            }
        }
        if written > 0 {
            let _ = self.data_buf.drain(..written);
        }
        ret
    }
}

impl<W: WriteFD + std::fmt::Debug> WriteFD for BufWriteFD<W> {
    fn write(&mut self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
        self.fd_buf.extend(fds.drain(..));

        if self.data_buf.len() + buf.len() > self.data_buf.capacity() {
            self.flush_buffer()?;
        }
        if buf.len() >= self.data_buf.capacity() {
            self.inner.write(buf, &mut self.fd_buf)
        } else {
            self.data_buf.write(buf)
        }
    }

    fn write_vectored(
        &mut self,
        bufs: &[IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> Result<usize> {
        self.fd_buf.extend(fds.drain(..));

        let total_len: usize = bufs.iter().map(|b| b.len()).sum();
        if self.data_buf.len() + total_len > self.data_buf.capacity() {
            self.flush_buffer()?;
        }
        if total_len >= self.data_buf.capacity() {
            self.inner.write_vectored(bufs, &mut self.fd_buf)
        } else {
            self.data_buf.write_vectored(bufs)
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.flush_buffer().and_then(|_| self.inner.flush())
    }
}

/// A version of [`std::io::Read`] that also allows receiving file descriptors.
pub trait ReadFD {
    /// Read some bytes and FDs from this reader, returning how many bytes were read.
    ///
    /// This function works like [`std::io::Read::read`], but also supports the reception of file
    /// descriptors. Any received file descriptors are appended to the given `fd_storage`.
    ///
    /// This function does not guarantee that all file descriptors were sent together with the data
    /// with which they are received. However, file descriptors may not be received later than the
    /// data that was sent at the same time. Instead, file descriptors may only be received
    /// earlier.
    fn read(&mut self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> Result<usize>;

    /// Read the exact number of bytes required to fill `buf` and also some amount of FDs.
    ///
    /// This function works like [`std::io::Read::read`], but also supports the reception of file
    /// descriptors. Any received file descriptors are appended to the given `fd_storage`.
    ///
    /// This function does not guarantee that all file descriptors were sent together with the data
    /// with which they are received. However, file descriptors may not be received later than the
    /// data that was sent at the same time. Instead, file descriptors may only be received
    /// earlier.
    fn read_exact(
        &mut self,
        mut buf: &mut [u8],
        fd_storage: &mut Vec<RawFdContainer>,
    ) -> Result<()> {
        while !buf.is_empty() {
            match self.read(buf, fd_storage) {
                Ok(0) => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedEof,
                        "failed to fill the whole buffer",
                    ))
                }
                Ok(n) => buf = &mut buf[n..],
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/// Wraps a [`std::io::Read`] to implement the [`ReadFD`] trait.
///
/// No file descriptors will be received. Attempts to read bytes are forwarded to the underlying
/// reader.
#[derive(Debug)]
pub struct ReadFDWrapper<R: Read + std::fmt::Debug>(R);

impl<R: Read + std::fmt::Debug> ReadFDWrapper<R> {
    /// Create a new `ReadFDWrapper` for the given reader.
    pub fn new(read: R) -> Self {
        Self(read)
    }
}

impl<R: Read + std::fmt::Debug> ReadFD for ReadFDWrapper<R> {
    fn read(&mut self, buf: &mut [u8], _fd_storage: &mut Vec<RawFdContainer>) -> Result<usize> {
        self.0.read(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8], _fd_storage: &mut Vec<RawFdContainer>) -> Result<()> {
        self.0.read_exact(buf)
    }
}

/// A version of [`std::io::BufReader`] that supports receiving file descriptors.
#[derive(Debug)]
pub struct BufReadFD<R: ReadFD + std::fmt::Debug> {
    inner: R,
    buf: Box<[u8]>,
    // The following two variables describe the range of available data in `buf`
    start: usize,
    end: usize,
}

impl<R: ReadFD + std::fmt::Debug> BufReadFD<R> {
    /// Creates a new `BufReadFD` with a default buffer capacity.
    pub fn new(inner: R) -> Self {
        // Chosen by checking what libxcb does
        let default = 4096;
        Self::with_capacity(default, inner)
    }

    /// Creates a new `BufReadFD` with the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        Self {
            inner,
            buf: vec![0; capacity].into_boxed_slice(),
            start: 0,
            end: 0,
        }
    }
}

impl<R: ReadFD + std::fmt::Debug> ReadFD for BufReadFD<R> {
    fn read(&mut self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> Result<usize> {
        if self.start >= self.end {
            // We have no data buffered
            if buf.len() >= self.buf.len() {
                // This is a large read, bypass our buffer
                return self.inner.read(buf, fd_storage);
            }
            // Read something new from the inner reader
            self.end = self.inner.read(&mut self.buf, fd_storage)?;
            self.start = 0;
        }
        // Read data from our buffer
        let nread = (&self.buf[self.start..self.end]).read(buf)?;
        self.start += nread;
        Ok(nread)
    }
}
