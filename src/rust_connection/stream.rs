use std::net::TcpStream;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write, Result, IoSlice, IoSliceMut};

/// A wrapper around a `TcpStream` or `UnixStream`.
#[derive(Debug)]
pub enum Stream {
    TcpStream(TcpStream),
    UnixStream(UnixStream),
}

impl Stream {
    /// Try to connect to the X11 server described by the given arguments.
    pub fn connect(host: &str, protocol: Option<&str>, display: u16) -> Result<Self> {
        const TCP_PORT_BASE: u16 = 6000;

        if (protocol.is_none() || protocol != Some("unix")) && !host.is_empty() && host != "unix" {
            Ok(Stream::TcpStream(TcpStream::connect((host, TCP_PORT_BASE + display))?))
        } else {
            let mut error = None;

            if protocol.is_none() || protocol == Some("unix") {
                let file_name = format!("/tmp/.X11-unix/X{}", display);

                // TODO: Try abstract socket (file name with prepended '\0')
                // Not supported on Rust right now: https://github.com/rust-lang/rust/issues/42048

                match UnixStream::connect(file_name) {
                    Ok(stream) => return Ok(Stream::UnixStream(stream)),
                    Err(err) => error = Some(err),
                }
            }

            if protocol.is_none() && host.is_empty() {
                Ok(Stream::TcpStream(TcpStream::connect(("localhost", TCP_PORT_BASE + display))?))
            } else {
                use std::io::{Error, ErrorKind};
                use crate::errors::ConnectionError;
                Err(error.unwrap_or_else(|| Error::new(ErrorKind::Other, ConnectionError::ConnectionError)))
            }
        }
    }
}

impl Read for Stream {
    // Implementation basically copied from impl Read for TcpStream/UnixStream

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Stream::TcpStream(stream) => stream.read(buf),
            Stream::UnixStream(stream) => stream.read(buf),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        match self {
            Stream::TcpStream(stream) => stream.read_vectored(bufs),
            Stream::UnixStream(stream) => stream.read_vectored(bufs),
        }
    }

    /*
     * Initializer is unstable: https://github.com/rust-lang/rust/issues/42788
    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        match self {
            Stream::TcpStream(stream) => stream.initializer(),
            Stream::UnixStream(stream) => stream.initializer(),
        }
    }
    */
}

impl Write for Stream {
    // Implementation basically copied from impl Write for TcpStream/UnixStream

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            Stream::TcpStream(stream) => stream.write(buf),
            Stream::UnixStream(stream) => stream.write(buf),
        }
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> {
        match self {
            Stream::TcpStream(stream) => stream.write_vectored(bufs),
            Stream::UnixStream(stream) => stream.write_vectored(bufs),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            Stream::TcpStream(stream) => stream.flush(),
            Stream::UnixStream(stream) => stream.flush(),
        }
    }
}
