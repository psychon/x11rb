//! Read X11 packets from a reader

use std::convert::TryInto;
use std::fmt::Debug;
use std::io::Result;

use super::fd_read_write::{BufReadFD, ReadFD};
use crate::utils::RawFdContainer;

/// A wrapper around a reader that reads X11 packet.
#[derive(Debug)]
pub(crate) struct PacketReader<R: ReadFD + Debug> {
    pub(crate) inner: BufReadFD<R>,

    // A packet that was partially read. The `Vec` is the partial packet and the `usize` describes
    // up to where the packet was already read.
    pending_packet: Option<(Vec<u8>, usize)>,
}

impl<R: ReadFD + Debug> PacketReader<R> {
    /// Create a new `PacketReader` that reads from the given stream.
    pub(crate) fn new(inner: BufReadFD<R>) -> Self {
        Self {
            inner,
            pending_packet: None,
        }
    }

    /// Deconstruct this `PacketReader`, returning the inner reader.
    ///
    /// This fails if there is currently a pending packet. In this case, `self` is returned as an
    /// error.
    pub(crate) fn into_inner(self) -> std::result::Result<BufReadFD<R>, Self> {
        if self.pending_packet.is_none() {
            Ok(self.inner)
        } else {
            Err(self)
        }
    }

    /// Gets a mutable reference to the underlying FD reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    #[allow(dead_code)] // This function exists for completeness with `get_ref`.
    pub(crate) fn get_mut(&mut self) -> &mut BufReadFD<R> {
        &mut self.inner
    }

    /// Gets a reference to the underlying FD reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub(crate) fn get_ref(&self) -> &BufReadFD<R> {
        &self.inner
    }

    /// Try to read an X11 packet from the inner reader.
    pub(crate) fn read_x11_packet(
        &mut self,
        fd_storage: &mut Vec<RawFdContainer>,
    ) -> Result<Vec<u8>> {
        self.read_packet(fd_storage, 32, compute_extra_length)
    }

    /// Try to read an X11 Setup packet from the inner reader.
    pub(crate) fn read_setup_packet(
        &mut self,
        fd_storage: &mut Vec<RawFdContainer>,
    ) -> Result<Vec<u8>> {
        self.read_packet(fd_storage, 8, compute_setup_length)
    }

    /// Try to read a packet from the inner reader.
    fn read_packet(
        &mut self,
        fd_storage: &mut Vec<RawFdContainer>,
        minimal_length: usize,
        extra_length: impl Fn(&[u8]) -> usize,
    ) -> Result<Vec<u8>> {
        if self.pending_packet.is_none() {
            self.pending_packet = Some((vec![0; minimal_length], 0));
        }

        // Get mutable reference to the pending packet
        let (packet, already_read) = self.pending_packet.as_mut().unwrap();

        // Until the packet was fully read...
        while packet.len() != *already_read {
            // ...continue reading the packet
            let nread = self.inner.read(&mut packet[*already_read..], fd_storage)?;
            *already_read += nread;

            // Do we still need to compute the length field? (length == minimal_length)
            if packet.len() == minimal_length {
                 // Yes, then compute the packet length and resize the `Vec` to its final size.
                 let extra = extra_length(&packet[..]);
                 packet.reserve_exact(extra);
                 packet.resize(minimal_length + extra, 0);
             }
        }

        // Check that we really read the whole packet
        let extra = extra_length(&packet[0..minimal_length]);
        assert_eq!(packet.len(), minimal_length + extra);

        // Packet successfully read
        Ok(self.pending_packet.take().unwrap().0)
    }
}

// Compute the length beyond the fixed 32 bytes of an X11 packet.
fn compute_extra_length(buffer: &[u8]) -> usize {
    use crate::protocol::xproto::GE_GENERIC_EVENT;
    assert_eq!(buffer.len(), 32);

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

// Compute the length beyond the fixed 8 bytes of an X11 connection setup packet.
fn compute_setup_length(buffer: &[u8]) -> usize {
    assert_eq!(buffer.len(), 8);
    4 * usize::from(u16::from_ne_bytes([buffer[6], buffer[7]]))
}
