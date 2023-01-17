// This code is dual licensed under MIT OR Apache 2.0.

//! Connect to the server asynchronously.

use crate::errors::ConnectError;

use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpStream};

#[cfg(unix)]
use std::os::unix::net::UnixStream;

use async_io::Async;
use x11rb::rust_connection::DefaultStream;
use x11rb_protocol::parse_display::{ConnectAddress, ParsedDisplay};

/// Connect to a `DefaultStream` from a display string.
pub(super) async fn connect(addrs: &ParsedDisplay) -> Result<(DefaultStream, usize), ConnectError> {
    let screen = addrs.screen.into();

    let mut err = None;

    for addr in addrs.connect_instruction() {
        match connect_addr(addr).await {
            Ok(stream) => return Ok((stream, screen)),
            Err(e) => err = Some(e),
        }
    }

    Err(match err {
        Some(e) => ConnectError::IoError(e),
        None => ConnectError::DisplayParsingError,
    })
}

/// Connect to a `DefaultStream` asynchronously.
///
/// This function is very inefficient. Consider improving it later.
async fn connect_addr(addr: ConnectAddress<'_>) -> io::Result<DefaultStream> {
    match addr {
        ConnectAddress::Hostname(host, port) => {
            let mut err = None;

            // Resolve the hostname.
            for host in resolve_addr(host).await? {
                // Try to connect to each address.
                match Async::<TcpStream>::connect((host, port)).await {
                    Ok(stream) => return DefaultStream::from_tcp_stream(stream.into_inner()?),
                    Err(e) => err = Some(e),
                }
            }

            // If we get here, we failed to connect to all addresses.
            Err(err.unwrap_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "failed to connect to any address")
            }))
        }

        #[cfg(unix)]
        ConnectAddress::Socket(path) => {
            let stream = Async::<UnixStream>::connect(path).await?;
            DefaultStream::from_unix_stream(stream.into_inner()?)
        }

        #[cfg(not(unix))]
        ConnectAddress::Socket(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Unix sockets are not supported",
        )),

        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "unsupported address type",
        )),
    }
}

/// Resolve the address asynchronously.
async fn resolve_addr(host: &str) -> io::Result<impl ExactSizeIterator<Item = IpAddr>> {
    // Avoid using the threadpool by trying to parse the address as IPv4 or IPv6.
    if let Ok(ipv4) = host.parse::<Ipv4Addr>() {
        return Ok(IpAddrIter::from(ipv4));
    }

    if let Ok(ipv6) = host.parse::<Ipv6Addr>() {
        return Ok(IpAddrIter::from(ipv6));
    }

    // Resolve the hostname using the blocking threadpool.
    let host = host.to_string();
    blocking::unblock(move || {
        use std::net::ToSocketAddrs;

        let hosts = host
            .to_socket_addrs()?
            .map(|addr| addr.ip())
            .collect::<Vec<_>>();
        Ok(IpAddrIter::Multiple(hosts.into_iter()))
    })
    .await
}

/// Iterator that returns one or returns several IPs.
enum IpAddrIter {
    /// A single IP.
    Single(Option<IpAddr>),

    /// Multiple IPs.
    Multiple(std::vec::IntoIter<IpAddr>),
}

impl Iterator for IpAddrIter {
    type Item = IpAddr;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IpAddrIter::Single(ip) => ip.take(),
            IpAddrIter::Multiple(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            IpAddrIter::Single(ip) => {
                if ip.is_some() {
                    (1, Some(1))
                } else {
                    (0, Some(0))
                }
            }
            IpAddrIter::Multiple(iter) => iter.size_hint(),
        }
    }
}

impl ExactSizeIterator for IpAddrIter {}

impl From<Ipv4Addr> for IpAddrIter {
    fn from(ip: Ipv4Addr) -> Self {
        IpAddrIter::Single(Some(IpAddr::V4(ip)))
    }
}

impl From<Ipv6Addr> for IpAddrIter {
    fn from(ip: Ipv6Addr) -> Self {
        IpAddrIter::Single(Some(IpAddr::V6(ip)))
    }
}
