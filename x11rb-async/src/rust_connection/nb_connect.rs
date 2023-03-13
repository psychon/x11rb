//! Connect to the server using a non-blocking strategy.

use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpStream};

#[cfg(unix)]
use std::os::unix::net::UnixStream;

use async_io::Async;

use futures_lite::pin;
use futures_lite::prelude::*;
use futures_lite::stream;

use x11rb::errors::ConnectError;
use x11rb::rust_connection::DefaultStream;
use x11rb_protocol::parse_display::{ConnectAddress, ParsedDisplay};

/// Connect to a `DefaultStream` asynchronously from a display string.
pub(super) async fn connect(addrs: &ParsedDisplay) -> Result<(DefaultStream, usize), ConnectError> {
    let screen = addrs.screen as usize;
    let mut err = None;

    let connections = stream::iter(addrs.connect_instruction()).then(connect_to_addr);

    // Pinning the stream lets us use `find_map` below without boxing.
    pin!(connections);

    connections
        .find_map(|result| match result {
            Ok(stream) => Some(stream),
            Err(e) => {
                err = Some(e);
                None
            }
        })
        .await
        .map(|stream| (stream, screen))
        .ok_or_else(|| {
            let io_err =
                err.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "no address resolved"));

            ConnectError::IoError(io_err)
        })
}

/// Connect to a `DefaultStream` asynchronously.
async fn connect_to_addr(addr: ConnectAddress<'_>) -> io::Result<DefaultStream> {
    match addr {
        ConnectAddress::Hostname(host, port) => {
            let mut err = None;

            // Resolve the hostname.
            let streams = resolve_host(host)
                .await?
                .then(|ip_addr| Async::<TcpStream>::connect((ip_addr, port)))
                .map(|result| {
                    result.and_then(|stream| DefaultStream::from_tcp_stream(stream.into_inner()?))
                });

            pin!(streams);

            streams
                .find_map(|result| match result {
                    Ok(stream) => Some(stream),
                    Err(e) => {
                        err = Some(e);
                        None
                    }
                })
                .await
                .ok_or_else(|| {
                    err.unwrap_or_else(|| {
                        io::Error::new(io::ErrorKind::Other, "no address resolved")
                    })
                })
        }

        #[cfg(unix)]
        ConnectAddress::Socket(path) => {
            let stream = Async::<UnixStream>::connect(path).await?;
            DefaultStream::from_unix_stream(stream.into_inner()?)
        }

        #[cfg(not(unix))]
        ConnectAddress::Socket(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Unix sockets are not supported on this platform",
        )),

        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "unexpected address type",
        )),
    }
}

/// Resolve the address asynchronously.
async fn resolve_host(host: &str) -> io::Result<impl Stream<Item = IpAddr>> {
    enum Either<A, B> {
        Left(A),
        Right(B),
    }

    impl<A: Stream + Unpin, B: Stream<Item = A::Item> + Unpin> Stream for Either<A, B> {
        type Item = A::Item;

        fn poll_next(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<Self::Item>> {
            match self.get_mut() {
                Either::Left(a) => a.poll_next(cx),
                Either::Right(b) => b.poll_next(cx),
            }
        }
    }

    use Either::{Left, Right};

    // We can avoid using the threadpool if we can resolve the host synchronously.
    if let Ok(ipv4) = host.parse::<Ipv4Addr>() {
        return Ok(Left(stream::once(IpAddr::V4(ipv4))));
    }

    if let Ok(ipv6) = host.parse::<Ipv6Addr>() {
        return Ok(Left(stream::once(IpAddr::V6(ipv6))));
    }

    // Resolve the host using the threadpool.
    let host = format!("{}:0", host);
    let iter = blocking::unblock(move || {
        use std::net::ToSocketAddrs;

        host.to_socket_addrs()
            .map(|socket| socket.map(|socket| socket.ip()))
    })
    .await?;

    // Push the remaining addresses into a stream.
    Ok(Right(blocking::Unblock::with_capacity(1, iter)))
}
