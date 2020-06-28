use std::io::Result as IOResult;
use std::net::{TcpListener, TcpStream};
use std::os::unix::net::UnixStream;
use smol::{Async, Task};

mod connection;

async fn handle_client_impl(client: Async<TcpStream>) -> IOResult<()> {
    let server = UnixStream::connect("/tmp/.X11-unix/X0")?;

    // We will spawn two tasks:
    // - Read from client, forward to server
    // - Read from server, forward to client
    //
    // Each task needs its own copy of the socket.
    let server2 = server.try_clone()?;

    // There is no Async::try_clone(), so we have to destroy the Async first
    let client = client.into_inner()?;
    let client2 = client.try_clone()?;

    let server = Async::new(server)?;
    let server2 = Async::new(server2)?;
    let client = Async::new(client)?;
    let client2 = Async::new(client2)?;

    let connection = connection::Connection::new();

    let future1 = connection.forward_client(client, server);
    let future2 = connection.forward_server(server2, client2);

    futures_util::try_join!(future1, future2)?;

    Ok(())
}

async fn handle_client(client: Async<TcpStream>) {
    handle_client_impl(client).await.expect("Error in client handling")
}

fn spawn_command_line(display: &str) {
    std::env::set_var("DISPLAY", display);
    let mut args = std::env::args_os().skip(1);
    if let Some(command) = args.next() {
        std::process::Command::new(command)
            .args(args)
            .spawn()
            .unwrap();
    }
}

fn main() -> IOResult<()> {
    smol::run(async {
        let listener = Async::<TcpListener>::bind("127.0.0.1:6004")?;
        spawn_command_line(":4");
        loop {
            let (socket, _addr) = listener.accept().await?;
            Task::spawn(handle_client(socket)).detach();
        }
    })
}
