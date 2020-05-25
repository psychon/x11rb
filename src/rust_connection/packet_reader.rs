//! Read X11 packets from a reader

use std::convert::TryInto;
use std::io::{Error, ErrorKind, Result};

use super::Stream;
use crate::utils::RawFdContainer;

/// Minimal length of an X11 packet
const MINIMAL_PACKET_LENGTH: usize = 32;

/// A wrapper around a reader that reads X11 packet.
#[derive(Debug)]
pub(crate) struct PacketReader {
    read_buffer: Box<[u8]>,

    // A packet that was partially read.
    pending_packet: Vec<u8>,
    // Up to where the packet is already read.
    already_read: usize,
}

impl PacketReader {
    /// Create a new `PacketReader` that reads from the given stream.
    pub(crate) fn new() -> Self {
        Self {
            // Buffer size chosen by checking what libxcb does
            read_buffer: vec![0; 4096].into_boxed_slice(),
            pending_packet: vec![0; MINIMAL_PACKET_LENGTH],
            already_read: 0,
        }
    }

    /// To be called after `nread` bytes have been writen into `pending_packet`.
    fn handle_partial_read(&mut self, nread: usize, out_packets: &mut Vec<Vec<u8>>) {
        self.already_read += nread;
        // Do we still need to compute the length field? (length == MINIMAL_PACKET_LENGTH)
        if self.already_read == MINIMAL_PACKET_LENGTH {
            // Yes, then compute the packet length and resize the `Vec` to its final size.
            let extra = extra_length(self.pending_packet[..].try_into().unwrap());
            self.pending_packet.reserve_exact(extra);
            self.pending_packet.resize(MINIMAL_PACKET_LENGTH + extra, 0);
        }

        // Has the packet been completely read?
        if self.already_read == self.pending_packet.len() {
            // Check that we really read the whole packet
            let initial_packet = &self.pending_packet[0..MINIMAL_PACKET_LENGTH]
                .try_into()
                .unwrap();
            let extra = extra_length(&initial_packet);
            assert_eq!(self.pending_packet.len(), MINIMAL_PACKET_LENGTH + extra);

            out_packets.push(std::mem::replace(
                &mut self.pending_packet,
                vec![0; MINIMAL_PACKET_LENGTH],
            ));
            self.already_read = 0;
        }
    }

    /// Block and read at least one packet from stream and read as many as
    /// possible without blocking.
    pub(crate) fn read_at_least_one_packet(
        &mut self,
        stream: &impl Stream,
        out_packets: &mut Vec<Vec<u8>>,
        fd_storage: &mut Vec<RawFdContainer>,
    ) -> Result<()> {
        while out_packets.is_empty() {
            stream.poll(true, false)?;
            self.try_read_packets(stream, out_packets, fd_storage)?;
        }
        Ok(())
    }

    /// Reads as many packets as possible from stream reader without blocking.
    pub(crate) fn try_read_packets(
        &mut self,
        stream: &impl Stream,
        out_packets: &mut Vec<Vec<u8>>,
        fd_storage: &mut Vec<RawFdContainer>,
    ) -> Result<()> {
        loop {
            if (self.pending_packet.len() - self.already_read) >= self.read_buffer.len() {
                assert_ne!(self.already_read, self.pending_packet.len());
                // Bypass the read buffer
                match stream.read(&mut self.pending_packet[self.already_read..], fd_storage) {
                    Ok(0) => {
                        return Err(Error::new(
                            ErrorKind::UnexpectedEof,
                            "The X11 server closed the connection",
                        ));
                    }
                    Ok(nread) => self.handle_partial_read(nread, out_packets),
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(e) => return Err(e),
                }
            } else {
                // Fill the read buffer
                match stream.read(&mut self.read_buffer, fd_storage) {
                    Ok(0) => {
                        return Err(Error::new(
                            ErrorKind::UnexpectedEof,
                            "The X11 server closed the connection",
                        ));
                    }
                    Ok(nread) => {
                        let mut used_from_buffer = 0;
                        // Take packets from `read_buffer`.
                        while used_from_buffer != nread {
                            let rem_read_buffer = &self.read_buffer[used_from_buffer..nread];
                            let rem_packet = &mut self.pending_packet[self.already_read..];
                            let to_copy = rem_read_buffer.len().min(rem_packet.len());
                            assert_ne!(to_copy, 0);
                            rem_packet[..to_copy].copy_from_slice(&rem_read_buffer[..to_copy]);
                            used_from_buffer += to_copy;
                            self.handle_partial_read(to_copy, out_packets);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(())
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
