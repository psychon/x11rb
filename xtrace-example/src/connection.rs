use std::convert::{TryInto, TryFrom};
use std::io::Result as IOResult;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use futures_io::{AsyncRead, AsyncWrite};

use x11rb::protocol::xproto::GE_GENERIC_EVENT;

use crate::forwarder::forward_with_callback;
use crate::connection_inner::ConnectionInner;

#[derive(Debug, Default)]
pub struct Connection {
    read_client_setup: AtomicBool,
    read_server_setup: AtomicBool,
    connection_inner: Mutex<ConnectionInner>,
}

impl Connection {
    /// Create a new instance of this struct.
    pub fn new() -> Self {
        Default::default()
    }

    /// Handle forwarding the client's data to the server.
    pub async fn forward_client(
        &self,
        client: impl AsyncRead + Unpin,
        server: impl AsyncWrite + Unpin,
    ) -> IOResult<()> {
        forward_with_callback(client, server, |packet| self.parse_client_packet(packet)).await
    }

    /// Handle forwarding the server's data to the client.
    pub async fn forward_server(
        &self,
        server: impl AsyncRead + Unpin,
        client: impl AsyncWrite + Unpin,
    ) -> IOResult<()> {
        forward_with_callback(server, client, |packet| self.parse_server_packet(packet)).await
    }

    /// Handle a packet from the client.
    ///
    /// Returns `None` if a complete packet was read. Otherwise returns the number of additional
    /// bytes that are needed.
    fn parse_client_packet(&self, packet: &[u8]) -> Option<usize> {
        if self.read_client_setup.load(Ordering::Relaxed) {
            let length_field = match packet.get(2..4) {
                None => return Some(packet.len() - 4),
                Some(length_field) => u16::from_ne_bytes(length_field.try_into().unwrap()),
            };
            let length_field = if length_field != 0 {
                usize::from(length_field) * 4
            } else {
                // Big requests
                let length_field = match packet.get(4..8) {
                    None => return Some(packet.len() - 8),
                    Some(length_field) => u32::from_ne_bytes(length_field.try_into().unwrap()),
                };
                usize::try_from(length_field).unwrap() * 4
            };
            if packet.len() < length_field {
                // Need more data
                Some(length_field - packet.len())
            } else {
                self.connection_inner.lock().unwrap().client_request(packet);
                None
            }
        } else {
            let minimum_length = 12;
            if packet.len() < minimum_length {
                return Some(minimum_length - packet.len());
            }
            println!("TODO: Check that the byte order is correct: {:?}", packet[0]);
            // There is no simple length field to use, so we let the inner handle this.
            self.connection_inner.lock().unwrap().client_setup(packet);
            todo!("Figure out request length");
            if false {
                // Got complete setup request
                self.read_client_setup.store(true, Ordering::Relaxed);
            }
            None
        }
    }

    /// Handle a packet from the server.
    ///
    /// Returns `None` if a complete packet was read. Otherwise returns the number of additional
    /// bytes that are needed.
    fn parse_server_packet(&self, packet: &[u8]) -> Option<usize> {
        if self.read_server_setup.load(Ordering::Relaxed) {
            const ERROR: u8 = 0;
            const REPLY: u8 = 1;

            // Try to figure out the length of the packet
            let has_length_field = match packet.get(0) {
                Some(&REPLY) => true,
                Some(x) if x & 0x7f == GE_GENERIC_EVENT => true,
                _ => false,
            };
            let additional_length = if has_length_field {
                if let Some(length_field) = packet.get(4..8) {
                    let length_field = u32::from_ne_bytes(length_field.try_into().unwrap());
                    let length_field = usize::try_from(length_field).unwrap();
                    assert!(length_field <= usize::max_value() / 4);
                    4 * length_field
                } else {
                    0
                }
            } else {
                0
            };
            // All packets are at least 32 bytes
            let packet_length = 32 + additional_length;
            if packet.len() < packet_length {
                // Need more data
                Some(packet_length - packet.len())
            } else {
                // Got a full packet
                let mut inner = self.connection_inner.lock().unwrap();
                match packet[0] {
                    ERROR => inner.server_error(packet),
                    REPLY => inner.server_reply(packet),
                    _ => inner.server_event(packet),
                }
                None
            }
        } else {
            // Get the length field of the server setup
            let length_field = match packet.get(7..8) {
                // Need more data
                None => return Some(8 - packet.len()),
                Some(field) => u16::from_ne_bytes(field.try_into().unwrap()),
            };
            let length_field = usize::from(length_field);
            let length = 40 + length_field * 4;
            if packet.len() < length {
                // Need more data
                Some(length - packet.len())
            } else {
                // Got the complete setup
                self.connection_inner.lock().unwrap().server_setup(packet);
                self.read_server_setup.store(true, Ordering::Relaxed);
                None
            }
        }
    }
}
