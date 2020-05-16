//! Read X11 packets from a reader

use std::convert::TryInto;
use std::io::Result;

use super::fd_read_write::ReadFD;
use crate::utils::RawFdContainer;

/// Minimal length of an X11 packet
const MINIMAL_PACKET_LENGTH: usize = 32;

/// A wrapper around a reader that reads X11 packet.
#[derive(Debug)]
pub(crate) struct PacketReader<R: ReadFD> {
    inner: R,

    // A packet that was partially read. The `Vec` is the partial packet and the `usize` describes
    // up to where the packet was already read.
    pending_packet: Option<(Vec<u8>, usize)>,
}

impl<R: ReadFD> PacketReader<R> {
    /// Create a new `PacketReader` that reads from the given stream.
    pub(crate) fn new(inner: R) -> Self {
        Self {
            inner,
            pending_packet: None,
        }
    }

    /// Try to read a packet from the inner reader.
    pub(crate) fn read_packet(&mut self, fd_storage: &mut Vec<RawFdContainer>) -> Result<Vec<u8>> {
        if self.pending_packet.is_none() {
            self.pending_packet = Some((vec![0; MINIMAL_PACKET_LENGTH], 0));
        }

        // Get mutable reference to the pending packet
        let (packet, already_read) = self.pending_packet.as_mut().unwrap();

        // Until the packet was fully read...
        while packet.len() != *already_read {
            // ...continue reading the packet
            let nread = self.inner.read(&mut packet[*already_read..], fd_storage)?;
            *already_read += nread;

            // Do we still need to compute the length field? (length == MINIMAL_PACKET_LENGTH)
            if let Ok(array) = packet[..].try_into() {
                // Yes, then compute the packet length and resize the `Vec` to its final size.
                let extra = extra_length(array);
                packet.reserve_exact(extra);
                packet.resize(MINIMAL_PACKET_LENGTH + extra, 0);
            }
        }

        // Check that we really read the whole packet
        let initial_packet = &packet[0..MINIMAL_PACKET_LENGTH].try_into().unwrap();
        let extra = extra_length(&initial_packet);
        assert_eq!(packet.len(), MINIMAL_PACKET_LENGTH + extra);

        // Packet successfully read
        Ok(self.pending_packet.take().unwrap().0)
    }
}

// Compute the length beyond `MINIMAL_PACKET_LENGTH` of an X11 packet.
fn extra_length(buffer: &[u8; MINIMAL_PACKET_LENGTH]) -> usize {
    use crate::protocol::xproto::GE_GENERIC_EVENT;

    let response_type = buffer[0];

    const REPLY: u8 = 1;
    if response_type == REPLY || response_type & 0x7f == GE_GENERIC_EVENT {
        let length_field = buffer[4..8].try_into().unwrap();
        let length_field = u32::from_ne_bytes(length_field) as usize;
        4 * length_field
    } else {
        // Fixed size packet: error or event that is not GE_GENERIC_EVENT
        0
    }
}
