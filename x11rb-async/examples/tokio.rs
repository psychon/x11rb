use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use x11rb_async::Connection;
use x11rb_protocol::protocol::xproto::{GetInputFocusRequest, GetWindowAttributesRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (read, write) = TcpStream::connect(("127.0.0.1".to_string(), 6000 + 4))
        .await?
        .into_split();

    let (conn, reader) = Connection::connect_streams_with_auth_data(
        read.compat(),
        write.compat_write(),
        vec![],
        vec![],
    )
    .await?;
    tokio::spawn(reader);
    println!("Root window: {:?}", conn.setup().roots[0].root);

    let request = GetInputFocusRequest;
    let reply = conn.send_request_with_reply(request).await?.reply().await;
    println!("Reply: {:?}", reply);

    let request = GetWindowAttributesRequest { window: 1234 };
    conn.send_request_with_reply(request).await?;

    println!("Waiting for event");
    println!("Event: {:?}", conn.wait_for_event().await);

    Ok(())
}
