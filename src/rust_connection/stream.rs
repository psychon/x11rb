use std::io::{IoSlice, Result};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, IntoRawSocket, RawSocket};

use super::fd_read_write::{ReadFD, WriteFD};
use super::xauth::Family;
use crate::utils::RawFdContainer;

/// A wrapper around a `TcpStream` or `UnixStream`.
#[derive(Debug)]
pub struct Stream {
    inner: StreamInner,
}

#[derive(Debug)]
enum StreamInner {
    TcpStream(TcpStream),
    #[cfg(unix)]
    UnixStream(UnixStream),
}

impl Stream {
    /// Try to connect to the X11 server described by the given arguments.
    pub fn connect(host: &str, protocol: Option<&str>, display: u16) -> Result<Self> {
        const TCP_PORT_BASE: u16 = 6000;

        if (protocol.is_none() || protocol != Some("unix")) && !host.is_empty() && host != "unix" {
            let stream = TcpStream::connect((host, TCP_PORT_BASE + display))?;
            Self::from_tcp_stream(stream)
        } else {
            // On non-unix, this variable is not mutated.
            #[allow(unused_mut)]
            let mut error = None;

            #[cfg(unix)]
            {
                if protocol.is_none() || protocol == Some("unix") {
                    let file_name = format!("/tmp/.X11-unix/X{}", display);

                    // TODO: Try abstract socket (file name with prepended '\0')
                    // Not supported on Rust right now: https://github.com/rust-lang/rust/issues/42048

                    match UnixStream::connect(file_name) {
                        Ok(stream) => {
                            return Self::from_unix_stream(stream);
                        }
                        Err(err) => error = Some(err),
                    }
                }
            }

            if protocol.is_none() && host.is_empty() {
                let stream = TcpStream::connect(("localhost", TCP_PORT_BASE + display))?;
                Self::from_tcp_stream(stream)
            } else {
                use crate::errors::ConnectError;
                use std::io::{Error, ErrorKind};
                Err(error.unwrap_or_else(|| {
                    Error::new(ErrorKind::Other, ConnectError::DisplayParsingError)
                }))
            }
        }
    }

    /// Creates a new `Stream` from an already connected `TcpStream`.
    ///
    /// The stream will be set in non-blocking mode.
    pub fn from_tcp_stream(stream: TcpStream) -> Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self {
            inner: StreamInner::TcpStream(stream),
        })
    }

    /// Creates a new `Stream` from an already connected `UnixStream`.
    ///
    /// The stream will be set in non-blocking mode.
    #[cfg(unix)]
    pub fn from_unix_stream(stream: UnixStream) -> Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self {
            inner: StreamInner::UnixStream(stream),
        })
    }

    /// Get the peer's address in a format suitable for xauth.
    ///
    /// The returned values can be directly given to `super::xauth::get_auth` as `family` and
    /// `address`.
    pub(crate) fn peer_addr(&self) -> Result<(Family, Vec<u8>)> {
        match self.inner {
            StreamInner::TcpStream(ref stream) => {
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
            }
            #[cfg(unix)]
            StreamInner::UnixStream(_) => {
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
        match self.inner {
            StreamInner::TcpStream(ref stream) => Ok(Self {
                inner: StreamInner::TcpStream(stream.try_clone()?),
            }),
            #[cfg(unix)]
            StreamInner::UnixStream(ref stream) => Ok(Self {
                inner: StreamInner::UnixStream(stream.try_clone()?),
            }),
        }
    }
}

#[cfg(unix)]
impl AsRawFd for Stream {
    fn as_raw_fd(&self) -> RawFd {
        match self.inner {
            StreamInner::TcpStream(ref stream) => stream.as_raw_fd(),
            StreamInner::UnixStream(ref stream) => stream.as_raw_fd(),
        }
    }
}

#[cfg(unix)]
impl IntoRawFd for Stream {
    fn into_raw_fd(self) -> RawFd {
        match self.inner {
            StreamInner::TcpStream(stream) => stream.into_raw_fd(),
            StreamInner::UnixStream(stream) => stream.into_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl AsRawSocket for Stream {
    fn as_raw_socket(&self) -> RawSocket {
        match self.inner {
            StreamInner::TcpStream(ref stream) => stream.as_raw_socket(),
        }
    }
}

#[cfg(windows)]
impl IntoRawSocket for Stream {
    fn into_raw_socket(self) -> RawSocket {
        match self.inner {
            StreamInner::TcpStream(stream) => stream.into_raw_socket(),
        }
    }
}

#[cfg(unix)]
fn do_write(fd: RawFd, bufs: &[IoSlice<'_>], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
    use nix::sys::{
        socket::{sendmsg, ControlMessage, MsgFlags},
        uio::IoVec,
    };

    let iov = bufs
        .iter()
        .map(|b| IoVec::from_slice(&**b))
        .collect::<Vec<_>>();
    let res = if !fds.is_empty() {
        let fds = fds.iter().map(|fd| fd.as_raw_fd()).collect::<Vec<_>>();
        let cmsgs = [ControlMessage::ScmRights(&fds[..])];
        sendmsg(fd, &iov, &cmsgs, MsgFlags::empty(), None)
    } else {
        sendmsg(fd, &iov, &[], MsgFlags::empty(), None)
    };
    // Nothing touched errno since sendmsg() failed
    let res = res.map_err(|_| std::io::Error::last_os_error())?;

    // We successfully sent all FDs
    fds.clear();

    Ok(res)
}

impl WriteFD for Stream {
    fn write(&mut self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
        #[cfg(unix)]
        {
            do_write(self.as_raw_fd(), &[IoSlice::new(buf)], fds)
        }
        #[cfg(not(unix))]
        {
            use std::io::{Error, ErrorKind, Write};
            if !fds.is_empty() {
                return Err(Error::new(ErrorKind::Other, "FD passing is unsupported"));
            }
            match self.inner {
                StreamInner::TcpStream(ref mut stream) => stream.write(buf),
            }
        }
    }

    fn write_vectored(
        &mut self,
        bufs: &[IoSlice<'_>],
        fds: &mut Vec<RawFdContainer>,
    ) -> Result<usize> {
        #[cfg(unix)]
        {
            do_write(self.as_raw_fd(), bufs, fds)
        }
        #[cfg(not(unix))]
        {
            use std::io::{Error, ErrorKind, Write};
            if !fds.is_empty() {
                return Err(Error::new(ErrorKind::Other, "FD passing is unsupported"));
            }
            match self.inner {
                StreamInner::TcpStream(ref mut stream) => stream.write_vectored(bufs),
            }
        }
    }

    fn flush(&mut self) -> Result<()> {
        // We do no buffering
        Ok(())
    }
}

impl ReadFD for Stream {
    fn read(&mut self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> Result<usize> {
        #[cfg(unix)]
        {
            use nix::sys::{
                socket::{recvmsg, ControlMessageOwned, MsgFlags},
                uio::IoVec,
            };

            // Chosen by checking what libxcb does
            const MAX_FDS_RECEIVED: usize = 16;
            let mut cmsg = nix::cmsg_space!([RawFd; MAX_FDS_RECEIVED]);
            let iov = [IoVec::from_mut_slice(buf)];

            let fd = self.as_raw_fd();
            let msg = recvmsg(fd, &iov[..], Some(&mut cmsg), MsgFlags::empty());
            // Nothing touched errno since recvmsg() failed
            let msg = msg.map_err(|_| std::io::Error::last_os_error())?;

            let fds_received = msg
                .cmsgs()
                .flat_map(|cmsg| match cmsg {
                    ControlMessageOwned::ScmRights(r) => r,
                    _ => Vec::new(),
                })
                .map(RawFdContainer::new);
            fd_storage.extend(fds_received);

            Ok(msg.bytes)
        }
        #[cfg(not(unix))]
        {
            use std::io::Read;
            // No FDs are read, so nothing needs to be done with fd_storage
            let _ = fd_storage;
            match self.inner {
                StreamInner::TcpStream(ref mut stream) => stream.read(buf),
            }
        }
    }
}

impl super::Poll for Stream {
    fn poll(&mut self, read: bool, write: bool) -> Result<(bool, bool)> {
        assert!(
            read || write,
            "at least one of `read` and `write` must be true",
        );

        #[cfg(unix)]
        {
            use nix::errno::Errno;
            use nix::poll::{poll, PollFd, PollFlags};
            use nix::sys::socket::{getsockopt, sockopt::SocketError};

            let mut poll_flags = PollFlags::empty();
            if read {
                poll_flags |= PollFlags::POLLIN;
            }
            if write {
                poll_flags |= PollFlags::POLLOUT;
            }
            let fd = self.as_raw_fd();
            let mut poll_fds = [PollFd::new(fd, poll_flags)];
            loop {
                match poll(&mut poll_fds, -1) {
                    Ok(_) => break,
                    Err(nix::Error::Sys(Errno::EINTR)) => {}
                    Err(_) => return Err(std::io::Error::last_os_error()),
                }
            }
            let revents = poll_fds[0].revents().unwrap_or_else(PollFlags::empty);
            if revents.contains(PollFlags::POLLERR) {
                let socket_err =
                    getsockopt(fd, SocketError).map_err(|_| std::io::Error::last_os_error())?;
                Err(std::io::Error::from_raw_os_error(socket_err))
            } else {
                let can_read = revents.contains(PollFlags::POLLIN);
                let can_write = revents.contains(PollFlags::POLLOUT);
                Ok((can_read, can_write))
            }
        }
        #[cfg(windows)]
        {
            use winapi::um::winsock2::{
                POLLERR, POLLHUP, POLLNVAL, POLLRDNORM, POLLWRNORM, SOCKET, WSAPOLLFD,
            };
            use winapi_wsapoll::wsa_poll;

            let raw_socket = self.as_raw_socket();
            let mut events = 0;
            if read {
                events |= POLLRDNORM;
            }
            if write {
                events |= POLLWRNORM;
            }
            let mut poll_fds = [WSAPOLLFD {
                fd: raw_socket as SOCKET,
                events,
                revents: 0,
            }];
            let _ = wsa_poll(&mut poll_fds, -1)?;
            if (poll_fds[0].revents & (POLLERR | POLLHUP | POLLNVAL)) != 0 {
                // Let the error be handled when trying to read or write.
                Ok((read, write))
            } else {
                let can_read = (poll_fds[0].revents & POLLRDNORM) != 0;
                let can_write = (poll_fds[0].revents & POLLWRNORM) != 0;
                Ok((can_read, can_write))
            }
        }
    }
}
