use std::io::Result as IOResult;
use futures_io::{AsyncRead, AsyncWrite};

use crate::forwarder::forward_with_callback;

#[derive(Debug, Default)]
pub struct Connection {
}

impl Connection {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn forward_client(
        &self,
        client: impl AsyncRead + Unpin,
        server: impl AsyncWrite + Unpin,
    ) -> IOResult<()> {
        forward_with_callback(client, server, |packet| self.parse_client_packet(packet)).await
    }

    pub async fn forward_server(
        &self,
        server: impl AsyncRead + Unpin,
        client: impl AsyncWrite + Unpin,
    ) -> IOResult<()> {
        forward_with_callback(server, client, |packet| self.parse_server_packet(packet)).await
    }

    fn parse_client_packet(&self, packet: &[u8]) -> Option<usize> {
        if packet.is_empty() {
            Some(1)
        } else {
            None
        }
    }

    fn parse_server_packet(&self, packet: &[u8]) -> Option<usize> {
        if packet.is_empty() {
            Some(1)
        } else {
            None
        }
    }
}
