use std::io::{IoSlice, Result};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, IntoRawSocket, RawSocket};

use super::xauth::Family;
use crate::utils::RawFdContainer;

/// The kind of operation that one want to poll for.
#[derive(Debug, Clone, Copy)]
pub enum PollMode {
    /// Check if the stream is readable, i.e. there is pending data to be read.
    Readable,

    /// Check if the stream is writable, i.e. some data could be successfully written to it.
    Writable,

    /// Check for both readability and writability.
    ReadAndWritable,
}

impl PollMode {
    /// Does this poll mode include readability?
    pub fn readable(self) -> bool {
        match self {
            PollMode::Readable | PollMode::ReadAndWritable => true,
            PollMode::Writable => false,
        }
    }

    /// Does this poll mode include writability?
    pub fn writable(self) -> bool {
        match self {
            PollMode::Writable | PollMode::ReadAndWritable => true,
            PollMode::Readable => false,
        }
    }
}

/// A trait used to implement the raw communication with the X11 server.
///
/// None of the functions of this trait shall return [`std::io::ErrorKind::Interrupted`].
/// If a system call fails with this error, the implementation should try again.
pub trait Stream {
    /// Waits for level-triggered read and/or write events on the stream.
    ///
    /// This function does not return what caused it to complete the poll.
    /// Instead, callers should try to read or write and check for
    /// [`std::io::ErrorKind::WouldBlock`].
    ///
    /// This function is allowed to spuriously return even if the stream
    /// is neither readable nor writable. However, it shall not do it
    /// continuously, which would cause a 100% CPU usage.
    ///
    /// # Multithreading
    ///
    /// If `Self` is `Send + Sync` and `poll` is used concurrently from more than
    /// one thread, all threads should wake when the stream becomes readable (when
    /// `read` is `true`) or writable (when `write` is `true`).
    fn poll(&self, mode: PollMode) -> Result<()>;

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
    ///
    /// # Multithreading
    ///
    /// If `Self` is `Send + Sync` and `read` is used concurrently from more than one thread:
    ///
    /// * Both the data and the file descriptors shall be read in order, but possibly
    /// interleaved across threads.
    /// * Neither the data nor the file descriptors shall be duplicated.
    /// * The returned value shall always be the actual number of bytes read into `buf`.
    fn read(&self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> Result<usize>;

    /// Read the exact number of bytes required to fill `buf` and also some amount of FDs.
    ///
    /// Unlike `read`, this method always blocks until `buf` has been filled or there is an
    /// error.
    ///
    /// This function works like [`std::io::Read::read`], but also supports the reception of file
    /// descriptors. Any received file descriptors are appended to the given `fd_storage`.
    ///
    /// This function does not guarantee that all file descriptors were sent together with the data
    /// with which they are received. However, file descriptors may not be received later than the
    /// data that was sent at the same time. Instead, file descriptors may only be received
    /// earlier.
    ///
    /// # Multithreading
    ///
    /// Same as `read`. In any case, if this function returns without error, `buf.len()` bytes
    /// should have been read into `buf`.
    fn read_exact(&self, mut buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> Result<()> {
        while !buf.is_empty() {
            self.poll(PollMode::Readable)?;
            match self.read(buf, fd_storage) {
                Ok(0) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "failed to fill the whole buffer",
                    ))
                }
                Ok(n) => buf = &mut buf[n..],
                // Spurious wakeup from poll
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

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
    ///
    /// # Multithreading
    ///
    /// If `Self` is `Send + Sync` and `write` is used concurrently from more than one thread:
    ///
    /// * Both the data and the file descriptors shall be written in order, but possibly
    /// interleaved across threads.
    /// * Neither the data nor the file descriptors shall be duplicated.
    /// * The returned value shall always be the actual number of bytes written from `buf`.
    fn write(&self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize>;

    /// Like `write`, except that it writes from a slice of buffers. Like `write`, this
    /// method must never block.
    ///
    /// This method must behave as a call to `write` with the buffers concatenated would.
    ///
    /// The default implementation calls `write` with the first nonempty buffer provided.
    ///
    /// # Multithreading
    ///
    /// Same as `write`.
    fn write_vectored(&self, bufs: &[IoSlice<'_>], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
        for buf in bufs {
            if !buf.is_empty() {
                return self.write(&**buf, fds);
            }
        }
        Ok(0)
    }
}

/// A wrapper around a `TcpStream` or `UnixStream`.
///
/// Use by default in `RustConnection` as stream.
#[derive(Debug)]
pub struct DefaultStream {
    inner: DefaultStreamInner,
}

#[derive(Debug)]
enum DefaultStreamInner {
    TcpStream(TcpStream),
    #[cfg(unix)]
    UnixStream(UnixStream),
}

impl DefaultStream {
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
            inner: DefaultStreamInner::TcpStream(stream),
        })
    }

    /// Creates a new `Stream` from an already connected `UnixStream`.
    ///
    /// The stream will be set in non-blocking mode.
    #[cfg(unix)]
    pub fn from_unix_stream(stream: UnixStream) -> Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self {
            inner: DefaultStreamInner::UnixStream(stream),
        })
    }

    /// Get the peer's address in a format suitable for xauth.
    ///
    /// The returned values can be directly given to `super::xauth::get_auth` as `family` and
    /// `address`.
    pub(crate) fn peer_addr(&self) -> Result<(Family, Vec<u8>)> {
        match self.inner {
            DefaultStreamInner::TcpStream(ref stream) => {
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
                            return Ok((Family::INTERNET6, ip.octets().to_vec()));
                        }
                    }
                };

                // Handle the v4 address
                if !ip.is_loopback() {
                    return Ok((Family::INTERNET, ip.octets().to_vec()));
                } else {
                    // This is only reached for loopback addresses. The code below handles this.
                }
            }
            #[cfg(unix)]
            DefaultStreamInner::UnixStream(_) => {
                // Fall through to the code below.
            }
        };

        // If we get to here: This is a local connection. Use the host name as address.
        let hostname = gethostname::gethostname()
            .to_str()
            .map(|name| name.as_bytes().to_vec())
            .unwrap_or_else(Vec::new);
        Ok((Family::LOCAL, hostname))
    }
}

#[cfg(unix)]
impl AsRawFd for DefaultStream {
    fn as_raw_fd(&self) -> RawFd {
        match self.inner {
            DefaultStreamInner::TcpStream(ref stream) => stream.as_raw_fd(),
            DefaultStreamInner::UnixStream(ref stream) => stream.as_raw_fd(),
        }
    }
}

#[cfg(unix)]
impl IntoRawFd for DefaultStream {
    fn into_raw_fd(self) -> RawFd {
        match self.inner {
            DefaultStreamInner::TcpStream(stream) => stream.into_raw_fd(),
            DefaultStreamInner::UnixStream(stream) => stream.into_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl AsRawSocket for DefaultStream {
    fn as_raw_socket(&self) -> RawSocket {
        match self.inner {
            DefaultStreamInner::TcpStream(ref stream) => stream.as_raw_socket(),
        }
    }
}

#[cfg(windows)]
impl IntoRawSocket for DefaultStream {
    fn into_raw_socket(self) -> RawSocket {
        match self.inner {
            DefaultStreamInner::TcpStream(stream) => stream.into_raw_socket(),
        }
    }
}

#[cfg(unix)]
fn do_write(
    stream: &DefaultStream,
    bufs: &[nix::sys::uio::IoVec<&[u8]>],
    fds: &mut Vec<RawFdContainer>,
) -> Result<usize> {
    use nix::sys::socket::{sendmsg, ControlMessage, MsgFlags};

    fn sendmsg_wrapper(
        fd: RawFd,
        iov: &[nix::sys::uio::IoVec<&[u8]>],
        cmsgs: &[ControlMessage<'_>],
        flags: MsgFlags,
        addr: Option<&nix::sys::socket::SockAddr>,
    ) -> Result<usize> {
        loop {
            match sendmsg(fd, iov, cmsgs, flags, addr) {
                Ok(n) => return Ok(n),
                // try again
                Err(nix::Error::EINTR) => {}
                Err(e) => return Err(e.into()),
            }
        }
    }

    let fd = stream.as_raw_fd();

    let res = if !fds.is_empty() {
        let fds = fds.iter().map(|fd| fd.as_raw_fd()).collect::<Vec<_>>();
        let cmsgs = [ControlMessage::ScmRights(&fds[..])];
        sendmsg_wrapper(fd, bufs, &cmsgs, MsgFlags::empty(), None)?
    } else {
        sendmsg_wrapper(fd, bufs, &[], MsgFlags::empty(), None)?
    };

    // We successfully sent all FDs
    fds.clear();

    Ok(res)
}

impl Stream for DefaultStream {
    fn poll(&self, mode: PollMode) -> Result<()> {
        #[cfg(unix)]
        {
            use nix::poll::{poll, PollFd, PollFlags};

            let mut poll_flags = PollFlags::empty();
            if mode.readable() {
                poll_flags |= PollFlags::POLLIN;
            }
            if mode.writable() {
                poll_flags |= PollFlags::POLLOUT;
            }
            let fd = self.as_raw_fd();
            let mut poll_fds = [PollFd::new(fd, poll_flags)];
            loop {
                match poll(&mut poll_fds, -1) {
                    Ok(_) => break,
                    Err(nix::Error::EINTR) => {}
                    Err(e) => return Err(e.into()),
                }
            }
            // Let the errors (POLLERR) be handled when trying to read or write.
            Ok(())
        }
        #[cfg(windows)]
        {
            use winapi::um::winsock2::{POLLRDNORM, POLLWRNORM, SOCKET, WSAPOLLFD};
            use winapi_wsapoll::wsa_poll;

            let raw_socket = self.as_raw_socket();
            let mut events = 0;
            if mode.readable() {
                events |= POLLRDNORM;
            }
            if mode.writable() {
                events |= POLLWRNORM;
            }
            let mut poll_fds = [WSAPOLLFD {
                fd: raw_socket as SOCKET,
                events,
                revents: 0,
            }];
            let _ = wsa_poll(&mut poll_fds, -1)?;
            // Let the errors (POLLERR) be handled when trying to read or write.
            Ok(())
        }
    }

    fn read(&self, buf: &mut [u8], fd_storage: &mut Vec<RawFdContainer>) -> Result<usize> {
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
            let msg = loop {
                match recvmsg(fd, &iov[..], Some(&mut cmsg), MsgFlags::empty()) {
                    Ok(msg) => break msg,
                    // try again
                    Err(nix::Error::EINTR) => {}
                    Err(e) => return Err(e.into()),
                }
            };

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
            loop {
                let read_result = match self.inner {
                    DefaultStreamInner::TcpStream(ref stream) => {
                        // Use `impl Read for &TcpStream` to avoid needing a mutable `TcpStream`.
                        (&mut &*stream).read(buf)
                    }
                };
                match read_result {
                    Ok(n) => return Ok(n),
                    // try again
                    Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
        }
    }

    fn write(&self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
        #[cfg(unix)]
        {
            do_write(self, &[nix::sys::uio::IoVec::from_slice(buf)], fds)
        }
        #[cfg(not(unix))]
        {
            use std::io::{Error, ErrorKind, Write};
            if !fds.is_empty() {
                return Err(Error::new(ErrorKind::Other, "FD passing is unsupported"));
            }
            loop {
                let write_result = match self.inner {
                    DefaultStreamInner::TcpStream(ref stream) => {
                        // Use `impl Write for &TcpStream` to avoid needing a mutable `TcpStream`.
                        (&mut &*stream).write(buf)
                    }
                };
                match write_result {
                    Ok(n) => return Ok(n),
                    // try again
                    Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
        }
    }

    fn write_vectored(&self, bufs: &[IoSlice<'_>], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
        #[cfg(unix)]
        {
            let bufs = bufs
                .iter()
                .map(|b| nix::sys::uio::IoVec::from_slice(&**b))
                .collect::<Vec<_>>();
            do_write(self, &bufs, fds)
        }
        #[cfg(not(unix))]
        {
            use std::io::{Error, ErrorKind, Write};
            if !fds.is_empty() {
                return Err(Error::new(ErrorKind::Other, "FD passing is unsupported"));
            }
            loop {
                let write_result = match self.inner {
                    DefaultStreamInner::TcpStream(ref stream) => {
                        // Use `impl Write for &TcpStream` to avoid needing a mutable `TcpStream`.
                        (&mut &*stream).write_vectored(bufs)
                    }
                };
                match write_result {
                    Ok(n) => return Ok(n),
                    // try again
                    Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
        }
    }
}
