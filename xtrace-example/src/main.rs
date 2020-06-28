//! xtrace implemented with x11rb.
//!
//! This example shows a capability of x11rb that is most likely not useful to many projects: You
//! can write a transparent "proxy" that intercepts X11 traffic.
//!
//! Specifically, this program accepts X11 clients on `DISPLAY=":4"` via TCP and connects them to
//! `DISPLAY=":0"` via a Unix stream. All traffic between the client and the server is forwarded
//! and additionally decoded.
//!
//! Plus, just because I wanted to do a simple experiment: All of this is written in `async`hronous
//! code using `smol` as the runtime.
//!
//!
//! # Structure of the code
//!
//! The main file sets up the listener and spawns a task for each connection. That task, called
//! `handle_client` handles everything concerning this one client.
//!
//! In the `forwarder` module, there is a generic utility function `forward_with_callback` that
//! forwards everything that is read from an `AsyncRead` to an `AsyncWrite`. A callback is called
//! on the data that is forwarded. The callback is used to decode and print the traffic.
//!
//! The `connection` module then chunks the traffic into X11 packets, i.e. it determines the length
//! of each packet that comes across. This requires some decoding of length fields, but no output
//! is generated. The code here also determines the types of packets (connection setup, requests,
//! replies, errors, and events).
//!
//! Finally, the code in the `connection_inner` actually decodes the packets. For each X11
//! connection, some state is kept:
//! - The number of requests the client already sent (sequence numbers). These numbers are needed
//!   to map replies to their corresponding requests.
//! - The X11 extensions that are known to be supported. This cache is filled from `QueryExtension`
//!   requests and replies that "fly by".
//! - A list of pending requests for which a reply is still expected.
//!
//! The code in `connection_inner` then handles decoding X11 packets, updating the state of the
//! connection, and generating output via `println!`.

#![forbid(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_results
)]
#![deny(
    // #[derive] generates an #[allow] for this
    unused_qualifications,
)]

use smol::{Async, Task};
use std::io::Result as IOResult;
use std::net::{TcpListener, TcpStream};
use std::os::unix::net::UnixStream;

pub(crate) mod connection;
pub(crate) mod connection_inner;
pub(crate) mod forwarder;

/// Handle an incoming client connection.
///
/// This function sets up a connection to the server
async fn handle_client_impl(client: Async<TcpStream>) -> IOResult<()> {
    let server = UnixStream::connect("/tmp/.X11-unix/X0")?;

    // We will have two futures:
    // - Read from client, forward to server
    // - Read from server, forward to client
    //
    // Each futures needs its own copy of the socket. Feel free to suggest a better approach to do
    // this.
    let server2 = server.try_clone()?;

    // There is no Async::try_clone(), so we have to destroy the Async first
    let client = client.into_inner()?;
    let client2 = client.try_clone()?;

    let server = Async::new(server)?;
    let server2 = Async::new(server2)?;
    let client = Async::new(client)?;
    let client2 = Async::new(client2)?;

    let connection = connection::Connection::default();
    let future1 = connection.forward_client(client, server);
    let future2 = connection.forward_server(server2, client2);

    // try_join polls both futures and returns an error once one of them returns an error.
    // (It also returns a result once both futures are done, but that should not matter here).
    futures_util::try_join!(future1, future2)?;

    Ok(())
}

/// Handle an incoming client connection.
///
/// This function simply calls `handle_client_impl` and handles the return value.
async fn handle_client(client: Async<TcpStream>) {
    use std::io::ErrorKind;

    match handle_client_impl(client).await {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => println!("Something disconnected"),
        Err(e) => eprintln!("Error in client forwarding: {:?}", e),
    }
}

/// Spawn the command line as a program / command.
fn spawn_command_line(display: &str) {
    std::env::set_var("DISPLAY", display);
    let mut args = std::env::args_os().skip(1);
    if let Some(command) = args.next() {
        let command = std::process::Command::new(command).args(args).spawn();
        match command {
            Ok(child) => println!("Started child process with ID {}", child.id()),
            Err(e) => eprintln!("Error while starting child process: {}", e),
        }
    } else {
        println!("You can now start programs with DISPLAY=\"{}\"", display);
        println!(
            "Hint: Alternatively, you could run {} [your-command]",
            std::env::args().next().unwrap(),
        );
    }
}

fn main() -> IOResult<()> {
    smol::run(async {
        // Port 6004 is DISPLAY=:4 (as TCP)
        let listener = Async::<TcpListener>::bind("127.0.0.1:6004")?;
        spawn_command_line(":4");
        loop {
            let (socket, _addr) = listener.accept().await?;
            Task::spawn(handle_client(socket)).detach();
        }
    })
}
