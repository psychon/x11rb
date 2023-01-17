// This code is dual licensed under MIT OR Apache 2.0.

//! Connect to the server asynchronously.

use crate::errors::ConnectError;

use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpStream};

#[cfg(unix)]
use std::os::unix::net::UnixStream;

use async_io::Async;
use tinyvec::TinyVec;
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
async fn resolve_addr(host: &str) -> io::Result<impl Iterator<Item = IpAddr>> {
    type Ips = TinyVec<[Option<IpAddr>; 1]>;

    let one = |i| TinyVec::from([Some(i)]).into_iter().flatten();

    // Avoid using the threadpool by trying to parse the address as IPv4 or IPv6.
    if let Ok(ipv4) = host.parse::<Ipv4Addr>() {
        return Ok(one(ipv4.into()));
    }

    if let Ok(ipv6) = host.parse::<Ipv6Addr>() {
        return Ok(one(ipv6.into()));
    }

    // Resolve the hostname using the blocking threadpool.
    let host = format!("{}:0", host);
    blocking::unblock(move || {
        use std::net::ToSocketAddrs;

        let hosts = host
            .to_socket_addrs()?
            .map(|addr| Some(addr.ip()))
            .collect::<Ips>();
        Ok(hosts.into_iter().flatten())
    })
    .await
}
