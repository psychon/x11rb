use crate::asyncchannel::{OneshotSender, UnboundedSender};
use std::collections::HashMap;
use x11rb::connection::SequenceNumber;
use x11rb::protocol::xproto::KEYMAP_NOTIFY_EVENT;

#[derive(Debug)]
pub(crate) struct ConnectionState {
    replies: HashMap<SequenceNumber, OneshotSender<Vec<u8>>>,
    default_read_packets: UnboundedSender<Vec<u8>>,

    pub(crate) last_sequence_send: SequenceNumber,
    last_sequence_read: SequenceNumber,
    next_reply_expected: SequenceNumber,
}

impl ConnectionState {
    pub(crate) fn new(default_read_packets: UnboundedSender<Vec<u8>>) -> Self {
        Self {
            replies: Default::default(),
            last_sequence_send: 0,
            last_sequence_read: 0,
            next_reply_expected: 0,
            default_read_packets,
        }
    }

    pub(crate) fn need_sync(&self) -> bool {
        self.next_reply_expected + SequenceNumber::from(u16::max_value()) <= self.last_sequence_send
    }

    pub(crate) fn sending_reply_request(&mut self, sender: OneshotSender<Vec<u8>>) {
        self.last_sequence_send += 1;
        self.next_reply_expected = self.last_sequence_send;
        self.replies.insert(self.last_sequence_send, sender);
    }

    fn extract_sequence_number(&mut self, packet: &[u8]) -> Option<SequenceNumber> {
        if packet[0] == KEYMAP_NOTIFY_EVENT {
            return None;
        }
        let number = u16::from_ne_bytes([packet[2], packet[3]]);
        let high_bytes = self.last_sequence_read & !SequenceNumber::from(u16::max_value());
        let mut full_number = SequenceNumber::from(number) | high_bytes;
        if full_number < self.last_sequence_read {
            full_number += SequenceNumber::from(u16::max_value()) + 1;
        }

        self.last_sequence_read = full_number;
        Some(full_number)
    }

    pub(crate) fn received_packet(&mut self, packet: Vec<u8>) {
        if let Some(seqno) = self.extract_sequence_number(&packet) {
            // TODO: Support requests with multiple replies
            if let Some(sender) = self.replies.remove(&seqno) {
                sender.send(packet);
                return;
            }
        }

        // We have no special target for this packet, so use the generic one
        self.default_read_packets.send(packet);
    }
}
