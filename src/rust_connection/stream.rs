use std::net::{TcpStream, SocketAddr, Ipv4Addr};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::io::{Read, Write, Result, IoSlice, IoSliceMut};

use super::xauth::Family;

/// A wrapper around a `TcpStream` or `UnixStream`.
#[derive(Debug)]
pub enum Stream {
    TcpStream(TcpStream),
    #[cfg(unix)]
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

            #[cfg(unix)]
            {
                if protocol.is_none() || protocol == Some("unix") {
                    let file_name = format!("/tmp/.X11-unix/X{}", display);

                    // TODO: Try abstract socket (file name with prepended '\0')
                    // Not supported on Rust right now: https://github.com/rust-lang/rust/issues/42048

                    match UnixStream::connect(file_name) {
                        Ok(stream) => return Ok(Stream::UnixStream(stream)),
                        Err(err) => error = Some(err),
                    }
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

impl Stream {
    /// Get the peer's address in a format suitable for xauth.
    ///
    /// The returned values can be directly given to `super::xauth::get_auth` as `family` and
    /// `address`.
    pub(crate) fn peer_addr(&self) -> Result<(Family, Vec<u8>)> {
        match self {
            Stream::TcpStream(stream) => {
                // Get the v4 address of the other end (if there is one)
                let ip = match stream.peer_addr()? {
                    SocketAddr::V4(addr) => *addr.ip(),
                    SocketAddr::V6(addr) => {
                        let ip = addr.ip();
                        if ip.is_loopback() {
                            // This is a local connection.
                            // Use LOCALHOST to cause a fall-through in the code below.
                            Ipv4Addr::LOCALHOST
                        } else if let Some(ip) = ip.to_ipv4() {
                            // Let the ipv4 code below handle this
                            ip
                        } else {
                            // Okay, this is really a v6 address
                            return Ok((Family::Internet6, ip.octets().to_vec()));
                        }
                    }
                };

                // Handle the v4 address
                if !ip.is_loopback() {
                    return Ok((Family::Internet, ip.octets().to_vec()));
                } else {
                    // This is only reached for loopback addresses. The code below handles this.
                }
            },
            #[cfg(unix)]
            Stream::UnixStream(_) => {
                // Fall through to the code below.
            }
        };

        // If we get to here: This is a local connection. Use the host name as address.
        let hostname = gethostname::gethostname()
            .to_str()
            .map(|name| name.as_bytes().to_vec())
            .unwrap_or_else(Vec::new);
        Ok((Family::Local, hostname))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// The returned `Stream` is a reference to the same stream that this object references. Both
    /// handles will read and write the same stream of data, and options set on one stream will be
    /// propagated to the other stream.
    pub fn try_clone(&self) -> Result<Stream> {
        match self {
            Stream::TcpStream(stream) => Ok(Stream::TcpStream(stream.try_clone()?)),
            #[cfg(unix)]
            Stream::UnixStream(stream) => Ok(Stream::UnixStream(stream.try_clone()?)),
        }
    }
}

impl Read for Stream {
    // Implementation basically copied from impl Read for TcpStream/UnixStream

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Stream::TcpStream(stream) => stream.read(buf),
            #[cfg(unix)]
            Stream::UnixStream(stream) => stream.read(buf),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        match self {
            Stream::TcpStream(stream) => stream.read_vectored(bufs),
            #[cfg(unix)]
            Stream::UnixStream(stream) => stream.read_vectored(bufs),
        }
    }

    /*
     * Initializer is unstable: https://github.com/rust-lang/rust/issues/42788
    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        match self {
            Stream::TcpStream(stream) => stream.initializer(),
            #[cfg(unix)]
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
            #[cfg(unix)]
            Stream::UnixStream(stream) => stream.write(buf),
        }
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> {
        match self {
            Stream::TcpStream(stream) => stream.write_vectored(bufs),
            #[cfg(unix)]
            Stream::UnixStream(stream) => stream.write_vectored(bufs),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            Stream::TcpStream(stream) => stream.flush(),
            #[cfg(unix)]
            Stream::UnixStream(stream) => stream.flush(),
        }
    }
}
