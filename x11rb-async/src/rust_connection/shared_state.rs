//! The state of the connection that is shared with the reading future

use event_listener::Event;
use futures_lite::future;
use std::convert::Infallible;
use std::io;
use std::mem;
use std::sync::{Mutex as StdMutex, MutexGuard as StdMutexGuard};
use x11rb::errors::ConnectionError;
use x11rb_protocol::connection::Connection as ProtoConnection;
use x11rb_protocol::packet_reader::PacketReader as ProtoPacketReader;
use x11rb_protocol::RawFdContainer;

use super::Stream;

/// State shared between the `RustConnection` and the future polling for new packets.
#[derive(Debug)]
pub(super) struct SharedState<S> {
    /// The underlying connection manager.
    ///
    /// This is never held across an `.await` point, so it's fine to use a standard library mutex.
    inner: StdMutex<ProtoConnection>,

    /// The stream for communicating with the X11 server.
    pub(super) stream: S,

    /// Listener for when new data is available on the stream.
    new_input: Event,
}

impl<S: Stream> SharedState<S> {
    pub(super) fn new(stream: S) -> Self {
        Self {
            inner: Default::default(),
            stream,
            new_input: Event::new(),
        }
    }

    /// Lock the inner connection and return a mutex guard for it.
    pub(super) fn lock_connection(&self) -> StdMutexGuard<'_, ProtoConnection> {
        self.inner.lock().unwrap()
    }

    /// Wait for an incoming packet.
    ///
    /// The given function get_reply should check whether the needed package was already received
    /// and put into the inner connection. It should return `None` if nothing is present yet and
    /// new incoming X11 packets should be awaited.
    pub(super) async fn wait_for_incoming<R, F>(&self, mut get_reply: F) -> R
    where
        F: FnMut(&mut ProtoConnection) -> Option<R>,
    {
        loop {
            // See if we can find the reply in the connection.
            if let Some(reply) = get_reply(&mut self.lock_connection()) {
                return reply;
            }

            // Register a listener for the reply.
            let listener = self.new_input.listen();

            // Maybe a packet was delivered while we were registering the listener.
            if let Some(reply) = get_reply(&mut self.lock_connection()) {
                return reply;
            }

            // Wait for the next packet.
            listener.await;
        }
    }

    /// Read incoming packets from the stream and put them into the inner connection.
    pub(super) async fn drive(&self) -> Result<Infallible, ConnectionError> {
        let mut packet_reader = PacketReader {
            read_buffer: vec![0; 4096].into_boxed_slice(),
            inner: ProtoPacketReader::new(),
        };
        let mut fds = vec![];
        let mut packets = vec![];

        loop {
            for _ in 0..50 {
                // Try to read packets from the stream.
                packet_reader.try_read_packets(&self.stream, &mut packets, &mut fds)?;
                let packet_count = packets.len();

                // Now, actually enqueue the packets.
                {
                    let mut inner = self.inner.lock().unwrap();
                    inner.enqueue_fds(mem::take(&mut fds));
                    packets
                        .drain(..)
                        .for_each(|packet| inner.enqueue_packet(packet));
                }

                if packet_count > 0 {
                    // Notify any listeners that there is new data.
                    self.new_input.notify_additional(std::usize::MAX);
                } else {
                    // Wait for more data.
                    self.stream.readable().await?;
                }
            }

            // In the case of a large influx of packets, don't starve other tasks.
            future::yield_now().await;
        }
    }
}

#[derive(Debug)]
struct PacketReader {
    /// The read buffer to store incoming bytes in.
    read_buffer: Box<[u8]>,

    /// The inner reader that breaks these bytes into packets.
    inner: ProtoPacketReader,
}

impl PacketReader {
    /// Try to read packets from the stream.
    fn try_read_packets(
        &mut self,
        stream: &impl Stream,
        out_packets: &mut Vec<Vec<u8>>,
        fd_storage: &mut Vec<RawFdContainer>,
    ) -> io::Result<()> {
        loop {
            // If the necessary packet size is larger than our buffer, just fill straight
            // into the buffer.
            if self.inner.remaining_capacity() >= self.read_buffer.len() {
                match stream.read(self.inner.buffer(), fd_storage) {
                    Ok(0) => {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "The X11 server closed the connection",
                        ));
                    }
                    Ok(n) => {
                        if let Some(packet) = self.inner.advance(n) {
                            out_packets.push(packet);
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                    Err(e) => return Err(e),
                }
            } else {
                // read into our buffer
                let nread = match stream.read(&mut self.read_buffer, fd_storage) {
                    Ok(0) => {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "The X11 server closed the connection",
                        ));
                    }
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                    Err(e) => return Err(e),
                };

                // begin reading that data into packets
                let mut src = &self.read_buffer[..nread];
                while !src.is_empty() {
                    let dest = self.inner.buffer();
                    let amt_to_read = std::cmp::min(src.len(), dest.len());

                    dest[..amt_to_read].copy_from_slice(&src[..amt_to_read]);

                    // reborrow src
                    src = &src[amt_to_read..];

                    // advance by the given amount
                    if let Some(packet) = self.inner.advance(amt_to_read) {
                        out_packets.push(packet);
                    }
                }
            }
        }

        Ok(())
    }
}
