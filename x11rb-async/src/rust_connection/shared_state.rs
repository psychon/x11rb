//! The state of the connection that is shared with the reading future

use event_listener::Event;
use futures_lite::future;
use std::convert::Infallible;
use std::io;
use std::mem;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex as StdMutex, MutexGuard as StdMutexGuard,
};
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

    /// Flag that indicates that the future for drive() was dropped and we no longer read input.
    driver_dropped: AtomicBool,
}

impl<S: Stream> SharedState<S> {
    pub(super) fn new(stream: S) -> Self {
        Self {
            inner: Default::default(),
            stream,
            new_input: Event::new(),
            driver_dropped: AtomicBool::new(false),
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
    pub(super) async fn wait_for_incoming<R, F>(&self, mut get_reply: F) -> Result<R, io::Error>
    where
        F: FnMut(&mut ProtoConnection) -> Option<R>,
    {
        loop {
            // See if we can find the reply in the connection.
            if let Some(reply) = get_reply(&mut self.lock_connection()) {
                return Ok(reply);
            }

            // Register a listener for the reply.
            let listener = self.new_input.listen();

            // Maybe a packet was delivered while we were registering the listener.
            if let Some(reply) = get_reply(&mut self.lock_connection()) {
                return Ok(reply);
            }

            // Maybe the future from drive() was dropped?
            // We only check this down here and not before the listener since this is unlikely
            if self.driver_dropped.load(Ordering::SeqCst) {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Driving future was dropped",
                ));
            }

            // Wait for the next packet.
            listener.await;
        }
    }

    /// Read incoming packets from the stream and put them into the inner connection.
    pub(super) async fn drive(
        &self,
        _break_on_drop: BreakOnDrop<S>,
    ) -> Result<Infallible, ConnectionError> {
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
                    let _num_notified = self.new_input.notify_additional(usize::MAX);
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
        let original_length = out_packets.len();
        loop {
            // If the necessary packet size is larger than our buffer, just fill straight
            // into the buffer.
            if self.inner.remaining_capacity() >= self.read_buffer.len() {
                tracing::trace!(
                    "Trying to read large packet with {} bytes remaining",
                    self.inner.remaining_capacity()
                );
                match stream.read(self.inner.buffer(), fd_storage) {
                    Ok(0) => {
                        tracing::error!("Large read returned zero");
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "The X11 server closed the connection",
                        ));
                    }
                    Ok(n) => {
                        tracing::trace!("Read {} bytes directly into large packet", n);
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
                        tracing::error!("Buffer read returned zero");
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "The X11 server closed the connection",
                        ));
                    }
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                    Err(e) => return Err(e),
                };
                tracing::trace!("Read {} bytes into read buffer", nread);

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
        tracing::trace!(
            "Read {} complete packet(s)",
            out_packets.len() - original_length
        );

        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct BreakOnDrop<S>(pub(super) Arc<SharedState<S>>);

impl<S> Drop for BreakOnDrop<S> {
    fn drop(&mut self) {
        // Mark the connection as broken
        self.0.driver_dropped.store(true, Ordering::SeqCst);

        // Wake up everyone that might be waiting
        let _num_notified = self.0.new_input.notify_additional(usize::MAX);
    }
}
