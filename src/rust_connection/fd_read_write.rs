//! Types for reading/writing bytes and FDs from a source.
//!
//! This module contains variants of `std::io::Read` and `std::io::Write` that also support passing
//! file descriptors.

use std::io::{Error, ErrorKind, IoSlice, Read, Result, Write};

use crate::utils::RawFdContainer;

pub trait Poll {
    /// Waits or checks for level-triggered read and/or write events
    /// on the stream.
    ///
    /// The first returned boolean specifies whether the stream can
    /// be read from and the second boolean specifies whether the
    /// stream can be written into.
    ///
    /// This function is allowed to return `(false, false)`. Additionally,
    /// a read or write might return `WouldBlock` even after this function
    /// reported that the stream is readable or writable.
    ///
    /// # Panics
    ///
    /// This function shall panic if `read` and `write` are both false.
    fn poll(&mut self, read: bool, write: bool) -> Result<(bool, bool)>;
}

/// A version of [`std::io::Write`] that also allows sending file descriptors and that
/// imposes some requirements regarding blocking behavior.
pub trait WriteFD: Poll {
    /// Write a buffer and some FDs into this writer without blocking, returning how many
    /// bytes were written.
    ///
    /// This function works like [`std::io::Write::write`], but also supports sending file
    /// descriptors. The `fds` argument contains the file descriptors to send. The order of file
    /// descriptors is maintained. Whereas implementation of [`std::io::Write::write`] are
    /// allowed to block or not to block, this function must never block and return
    /// `ErrorKind::WouldBlock` if needed.
    ///
    /// This function does not guarantee that all file descriptors are sent together with the data.
    /// Any file descriptors that were sent are removed from the beginning of the given `Vec`.
    ///
    /// There is no guarantee that the given file descriptors are received together with the given
    /// data. File descriptors might be received earlier than their corresponding data. It is not
    /// allowed for file descriptors to be received later than the bytes that were sent at the same
    /// time.
    fn write(&mut self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize>;

    /// Like `write`, except that it writes from a slice of buffers. Like `write`, this
    /// method must never block.
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
    ///
    /// This operation is also non-blocking. `ErrorKind::WouldBlock` shall be returned
    /// if the buffer cannot be completely flushed.
    ///
    /// Note that if the writer is actually buffered, `poll` shall not consider the write
    /// buffer, it should only check the inner stream. This allows to use `poll` to check
    /// whether `flush` can be called.
    fn flush(&mut self) -> Result<()>;
}

/// A version of [`std::io::BufWriter`] that supports sending file descriptors.
#[derive(Debug)]
pub struct BufWriteFD<W: WriteFD> {
    inner: W,
    data_buf: Vec<u8>,
    fd_buf: Vec<RawFdContainer>,
}

impl<W: WriteFD> BufWriteFD<W> {
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

    /// Gets a mutable reference to the underlying FD writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Gets a reference to the underlying FD writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
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

    fn write_helper<F, G>(
        &mut self,
        fds: &mut Vec<RawFdContainer>,
        write_buffer: F,
        write_inner: G,
        first_buffer: &[u8],
        to_write_length: usize,
    ) -> Result<usize>
    where
        F: FnOnce(&mut Vec<u8>) -> Result<usize>,
        G: FnOnce(&mut W, &mut Vec<RawFdContainer>) -> Result<usize>,
    {
        self.fd_buf.append(fds);

        // Is there enough buffer space left for this write?
        if (self.data_buf.capacity() - self.data_buf.len()) < to_write_length {
            // Not enough space, try to flush
            match self.flush_buffer() {
                Ok(_) => {}
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        let available_buf = self.data_buf.capacity() - self.data_buf.len();
                        if available_buf == 0 {
                            // Buffer filled and cannot flush anything without
                            // blocking, so return `WouldBlock`.
                            return Err(e);
                        } else {
                            let n_to_write = first_buffer.len().min(available_buf);
                            let _ = self.data_buf.write(&first_buffer[..n_to_write]).unwrap();
                            // Return `Ok` because some or all data has been buffered,
                            // so from the outside it is seen as a successful write.
                            return Ok(n_to_write);
                        }
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        if to_write_length >= self.data_buf.capacity() {
            // Write is larger than the buffer capacity, thus we just flushed the buffer. This
            // means that at this point the buffer is empty. Write directly to self.inner. No data
            // is copied into the buffer, since that would just mean that the large write gets
            // split into multiple smaller ones.
            assert!(self.data_buf.is_empty());
            write_inner(&mut self.inner, &mut self.fd_buf)
        } else {
            // At this point there is enough space available in the buffer.
            write_buffer(&mut self.data_buf)
        }
    }
}

impl<W: WriteFD> WriteFD for BufWriteFD<W> {
    fn write(&mut self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
        self.write_helper(
            fds,
            |w| w.write(buf),
            |w, fd| w.write(buf, fd),
            buf,
            buf.len(),
        )
    }

    fn write_vectored(
        &mut self,
        bufs: &[IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> Result<usize> {
        let first_nonempty = bufs
            .iter()
            .find(|b| !b.is_empty())
            .map_or(&[][..], |b| &**b);
        let total_len = bufs.iter().map(|b| b.len()).sum();
        self.write_helper(
            fds,
            |w| w.write_vectored(bufs),
            |w, fd| w.write_vectored(bufs, fd),
            first_nonempty,
            total_len,
        )
    }

    fn flush(&mut self) -> Result<()> {
        self.flush_buffer().and_then(|_| self.inner.flush())
    }
}

impl<T: WriteFD> Poll for BufWriteFD<T> {
    fn poll(&mut self, read: bool, write: bool) -> Result<(bool, bool)> {
        // Ignore buffer. Even if there is space available in the buffer, poll will block
        // until the stream is actually writable. This simplifies the implementation of
        // `write` and `write_vectored` and allows to use `poll` + `flush`.
        self.inner.poll(read, write)
    }
}

/// A version of [`std::io::Read`] that also allows receiving file descriptors and that
/// imposes some requirements regarding blocking behavior.
pub trait ReadFD: Poll {
    /// Read some bytes and FDs from this reader without blocking, returning how many bytes
    /// were read.
    ///
    /// This function works like [`std::io::Read::read`], but also supports the reception of file
    /// descriptors. Any received file descriptors are appended to the given `fd_storage`.
    /// Whereas implementation of [`std::io::Read::read`] are allowed to block or not to block,
    /// this method shall never block and return `ErrorKind::WouldBlock` if needed.
    ///
    /// This function does not guarantee that all file descriptors were sent together with the data
    /// with which they are received. However, file descriptors may not be received later than the
    /// data that was sent at the same time. Instead, file descriptors may only be received
    /// earlier.
    fn read(&mut self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> Result<usize>;

    /// Read the exact number of bytes required to fill `buf` and also some amount of FDs.
    ///
    /// Unlike `read`, this method always blocks.
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
            let _ = self.poll(true, false)?;
            match self.read(buf, fd_storage) {
                Ok(0) => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedEof,
                        "failed to fill the whole buffer",
                    ))
                }
                Ok(n) => buf = &mut buf[n..],
                // Spurious wakeup from poll
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/// A version of [`std::io::BufReader`] that supports receiving file descriptors.
#[derive(Debug)]
pub struct BufReadFD<R: ReadFD> {
    inner: R,
    buf: Box<[u8]>,
    // The following two variables describe the range of available data in `buf`
    start: usize,
    end: usize,
}

impl<R: ReadFD> BufReadFD<R> {
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

    /// Gets a mutable reference to the underlying FD reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Gets a reference to the underlying FD reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }
}

impl<R: ReadFD> ReadFD for BufReadFD<R> {
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

impl<T: ReadFD> Poll for BufReadFD<T> {
    fn poll(&mut self, read: bool, write: bool) -> Result<(bool, bool)> {
        if read && self.start < self.end {
            // Avoid blocking poll if read buffer is not empty.
            Ok((true, false))
        } else {
            self.inner.poll(read, write)
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{Error, ErrorKind, IoSlice, Result};

    use super::{BufWriteFD, Poll, WriteFD};
    use crate::utils::RawFdContainer;

    struct WouldBlockWriter();

    impl WriteFD for WouldBlockWriter {
        fn write(&mut self, _buf: &[u8], _fds: &mut Vec<RawFdContainer>) -> Result<usize> {
            Err(Error::new(ErrorKind::WouldBlock, "would block"))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    impl Poll for WouldBlockWriter {
        fn poll(&mut self, _read: bool, _write: bool) -> Result<(bool, bool)> {
            unimplemented!()
        }
    }

    // Once upon a time, this paniced because it did bufs[0]
    #[test]
    fn empty_write() {
        let mut write = BufWriteFD::new(WouldBlockWriter());
        let bufs = &[];
        let _ = write.write_vectored(bufs, &mut Vec::new()).unwrap();
    }

    // Once upon a time, BufWriteFD fell back to only writing the first buffer. This could be
    // mistaken as EOF.
    #[test]
    fn incorrect_eof() {
        let mut write = BufWriteFD::with_capacity(1, WouldBlockWriter());
        let bufs = &[IoSlice::new(&[]), IoSlice::new(b"fooo")];
        match write.write_vectored(bufs, &mut Vec::new()) {
            Ok(0) => panic!("This looks like EOF!?"),
            Ok(_) => {}
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
