use super::StreamBase;
use async_lock::{Mutex, MutexGuard};
use std::io;
use x11rb::errors::ConnectionError;
use x11rb_protocol::RawFdContainer;

#[derive(Debug, Default)]
pub(super) struct WriteBuffer(Mutex<WriteBufferInner>);

#[derive(Debug)]
pub(super) struct WriteBufferGuard<'a>(MutexGuard<'a, WriteBufferInner>);

#[derive(Debug)]
pub(super) struct WriteBufferInner {
    /// The buffer that is used for writing.
    buffer: Vec<u8>,

    /// The file descriptors that we are sending over.
    fds: Vec<RawFdContainer>,

    /// Whether the buffer has been corrupted.
    ///
    /// A lock has to be explicitly unlock()d, otherwise the buffer is marked as corrupted.
    /// This exists to detect futures that were not polled to completion and might have
    /// written only a part of their data.
    corrupted: bool,
}

impl Default for WriteBufferInner {
    fn default() -> Self {
        Self {
            buffer: Vec::with_capacity(16384),
            fds: vec![],
            corrupted: false,
        }
    }
}

impl WriteBuffer {
    /// Lock the write buffer for writing.
    ///
    /// The returned guard must be unlocked with [`unlock()`] or else the write buffer will be
    /// considered corrupted. This mechanism exists to catch futures being dropped without being
    /// polled to completion. In this situation we cannot be sure how many bytes were already
    /// written to the stream, so the complete connection is now broken.
    pub async fn lock(&self) -> Result<WriteBufferGuard<'_>, ConnectionError> {
        let mut lock = self.0.lock().await;
        if std::mem::replace(&mut lock.corrupted, true) {
            return Err(ConnectionError::IoError(io::Error::new(
                io::ErrorKind::Other,
                "The write buffer was corrupted",
            )));
        }

        Ok(WriteBufferGuard(lock))
    }
}

impl WriteBufferGuard<'_> {
    /// Unlock this guard.
    pub fn unlock(mut self) {
        self.0.corrupted = false;
    }
}

impl std::ops::Deref for WriteBufferGuard<'_> {
    type Target = WriteBufferInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for WriteBufferGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WriteBufferInner {
    /// Flush the write buffer.
    pub async fn flush<'b, S: StreamBase<'b>>(
        &mut self,
        stream: &'b S,
    ) -> Result<(), ConnectionError> {
        // If we don't have any data to write, we are done.
        if self.buffer.is_empty() && self.fds.is_empty() {
            return Ok(());
        }

        // Write the entire buffer.
        let mut position = 0;
        super::write_with(stream, {
            let (buffer, fds) = (&mut self.buffer, &mut self.fds);
            move |stream| {
                while position < buffer.len() {
                    let n = stream.write(&buffer[position..], fds)?;
                    if n == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            "failed to write whole buffer",
                        ));
                    }

                    position += n;
                }

                Ok(())
            }
        })
        .await?;

        if !self.fds.is_empty() {
            return Err(ConnectionError::IoError(io::Error::new(
                io::ErrorKind::Other,
                "failed to write all fds",
            )));
        }

        // Reset the buffer.
        self.buffer.clear();

        Ok(())
    }

    /// Write a set of buffers to the stream.
    pub async fn write_all_vectored<'b, S: StreamBase<'b>>(
        &mut self,
        stream: &'b S,
        mut bufs: &[io::IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> Result<(), ConnectionError> {
        // Get the total length of the buffers.
        let mut total_len = bufs
            .iter()
            .fold(0usize, |sum, buf| sum.saturating_add(buf.len()));

        // If our data doesn't fit, flush the buffer first.
        if self.buffer.len() + total_len > self.buffer.capacity() {
            self.flush(stream).await?;
        }

        // If our data fits now, write all of it.
        if total_len < self.buffer.capacity() {
            for buf in bufs {
                self.buffer.extend_from_slice(buf);
            }

            self.fds.append(fds);

            return Ok(());
        }

        debug_assert!(self.buffer.is_empty());

        // Otherwise, write directly to the stream.
        let mut partial: &[u8] = &[];
        super::write_with(stream, |stream| {
            while total_len > 0 && !partial.is_empty() {
                // If the partial buffer is non-empty, write it.
                if !partial.is_empty() {
                    let n = stream.write(partial, fds)?;
                    if n == 0 {
                        return Err(io::Error::from(io::ErrorKind::WriteZero));
                    }

                    partial = &partial[n..];
                    total_len -= n;
                } else {
                    // Write the iov.
                    let mut n = stream.write_vectored(bufs, fds)?;
                    if n == 0 {
                        return Err(io::Error::from(io::ErrorKind::WriteZero));
                    }

                    // Calculate how much we have left to go.
                    total_len -= n;
                    while n > 0 {
                        if n >= bufs[0].len() {
                            n -= bufs[0].len();
                            bufs = &bufs[1..];
                        } else {
                            partial = &bufs[0][n..];
                            n = 0;
                        }
                    }
                }
            }

            Ok(())
        })
        .await?;

        Ok(())
    }
}
