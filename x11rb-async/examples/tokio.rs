use x11rb_protocol::protocol::xproto::{GetInputFocusRequest, GetWindowAttributesRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen) = x11rb_async::tokio::connect(None).await?;
    println!("Root window: {:?}", conn.setup().roots[screen].root);

    let request = GetInputFocusRequest;
    let reply = conn.send_request_with_reply(request).await?.reply().await;
    println!("Reply: {:?}", reply);

    let request = GetWindowAttributesRequest { window: 1234 };
    conn.send_request_with_reply(request).await?;

    println!("Waiting for event");
    println!("Event: {:?}", conn.wait_for_event().await);

    Ok(())
}
