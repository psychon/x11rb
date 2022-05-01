use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

use x11rb_protocol::{
    errors::ConnectError,
    parse_display::{parse_display, ConnectAddress},
};

use crate::Connection;

type WriteType = tokio_util::compat::Compat<tokio::net::tcp::OwnedWriteHalf>;

async fn try_connect(address: ConnectAddress<'_>) -> Result<Connection<WriteType>, ConnectError> {
    match address {
        ConnectAddress::Hostname(host, port) => {
            let stream = TcpStream::connect((host.to_string(), port)).await?;
            // TODO: Do what x11rb::rust_connection::stream::peer_addr() does and call
            // x11rb_protocol::xauth::get_auth
            //let peer = stream.peer_addr()?;
            let (read, write) = stream.into_split();
            let (conn, reader) = Connection::connect_streams_with_auth_data(
                read.compat(),
                write.compat_write(),
                vec![],
                vec![],
            )
            .await?;
            tokio::spawn(reader);
            Ok(conn)
        }
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "only TCP connections are currently implemented",
        )
        .into()),
    }
}
// TODO: A write buffer?!

pub async fn connect(
    dpy_name: Option<&str>,
) -> Result<(Connection<WriteType>, usize), ConnectError> {
    let display = parse_display(dpy_name).ok_or(ConnectError::DisplayParsingError)?;
    let screen = display.screen.into();

    let mut error = None;
    for addr in display.connect_instruction() {
        match try_connect(addr).await {
            Ok(connection) => {
                return Ok((connection, screen));
            }
            Err(e) => error = Some(e),
        }
    }

    Err(error.unwrap_or(ConnectError::DisplayParsingError))
}
