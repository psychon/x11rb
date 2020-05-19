use std::sync::Arc;

use x11rb::connection::Connection as _;
use x11rb::protocol::xproto::{
    ClientMessageData, ClientMessageEvent, ConnectionExt as _, EventMask, CLIENT_MESSAGE_EVENT,
};

// Regression test for https://github.com/psychon/x11rb/issues/231
#[test]
fn multithread_test() {
    let conn = fake_stream::connect().unwrap();
    let conn = Arc::new(conn);

    // Auxiliary thread: send requests and wait for replies
    let conn1 = conn.clone();
    let join = std::thread::spawn(move || {
        // Bug #231 sometimes caused `reply` to hang forever.
        // Send a huge amount of requests and wait for the reply
        // to check if it hangs at some point.
        for i in 1..=1_000_000 {
            let cookie = conn1.get_input_focus().unwrap();
            cookie.reply().unwrap();

            if (i % 50_000) == 0 {
                eprintln!("{}", i);
            }
        }
        eprintln!("all replies received successfully");

        let event = ClientMessageEvent {
            response_type: CLIENT_MESSAGE_EVENT,
            format: 32,
            sequence: 0,
            window: 0,
            // Just anything, we don't care
            type_: 1,
            data: ClientMessageData::from([0, 0, 0, 0, 0]),
        };

        conn1
            .send_event(false, 0u32, EventMask::NoEvent, &event)
            .unwrap();
        conn1.flush().unwrap();
    });

    // Main thread: wait for events until finished
    loop {
        let event = conn.wait_for_raw_event().unwrap();
        if event[0] == CLIENT_MESSAGE_EVENT {
            break;
        }
    }

    join.join().unwrap();
}

/// Implementations of `Read` and `Write` that do enough for the test to work.
mod fake_stream {
    use std::io::{Error, ErrorKind};
    use std::sync::mpsc::{channel, Receiver, Sender};

    use x11rb::connection::SequenceNumber;
    use x11rb::errors::ConnectError;
    use x11rb::protocol::xproto::{
        ImageOrder, Setup, CLIENT_MESSAGE_EVENT, GET_INPUT_FOCUS_REQUEST, SEND_EVENT_REQUEST,
    };
    use x11rb::rust_connection::{ReadFD, RustConnection, WriteFD};
    use x11rb::utils::RawFdContainer;

    /// Create a new `RustConnection` connected to a fake stream
    pub(crate) fn connect() -> Result<RustConnection<FakeStreamRead, FakeStreamWrite>, ConnectError>
    {
        let setup = Setup {
            status: 0,
            protocol_major_version: 0,
            protocol_minor_version: 0,
            length: 0,
            release_number: 0,
            resource_id_base: 0,
            resource_id_mask: 0xff,
            motion_buffer_size: 0,
            maximum_request_length: 0,
            image_byte_order: ImageOrder::LSBFirst,
            bitmap_format_bit_order: ImageOrder::LSBFirst,
            bitmap_format_scanline_unit: 0,
            bitmap_format_scanline_pad: 0,
            min_keycode: 0,
            max_keycode: 0,
            vendor: Vec::new(),
            pixmap_formats: Vec::new(),
            roots: Vec::new(),
        };
        let (read, write) = fake_stream();
        RustConnection::for_connected_stream(read, write, setup)
    }

    /// Get a pair of fake streams that are connected to each other
    fn fake_stream() -> (FakeStreamRead, FakeStreamWrite) {
        let (send, recv) = channel();
        let pending = Vec::new();
        let read = FakeStreamRead {
            recv,
            pending,
            nonblocking: false,
        };
        let write = FakeStreamWrite {
            send,
            seqno: 0,
            skip: 0,
        };
        (read, write)
    }

    /// A packet that still needs to be read from FakeStreamRead
    #[derive(Debug)]
    enum Packet {
        GetInputFocusReply(SequenceNumber),
        Event,
    }

    impl Packet {
        fn to_raw(&self) -> Vec<u8> {
            match self {
                Packet::GetInputFocusReply(seqno) => {
                    let seqno = (*seqno as u16).to_ne_bytes();
                    let mut reply = vec![0; 32];
                    reply[0] = 1; // This is a reply
                    reply[2..4].copy_from_slice(&seqno);
                    reply
                }
                Packet::Event => {
                    let mut reply = vec![0; 32];
                    reply[0] = CLIENT_MESSAGE_EVENT;
                    reply
                }
            }
        }
    }

    #[derive(Debug)]
    pub(crate) struct FakeStreamRead {
        recv: Receiver<Packet>,
        pending: Vec<u8>,
        nonblocking: bool,
    }

    impl ReadFD for FakeStreamRead {
        fn read(
            &mut self,
            buf: &mut [u8],
            _fd_storage: &mut Vec<RawFdContainer>,
        ) -> Result<usize, Error> {
            if self.pending.is_empty() {
                if self.nonblocking {
                    return Err(Error::new(ErrorKind::WouldBlock, "Would block"));
                }
                let packet = self.recv.recv().unwrap();
                self.pending.extend(packet.to_raw());
            }

            let len = self.pending.len().min(buf.len());
            buf[..len].copy_from_slice(&self.pending[..len]);
            self.pending.drain(..len);
            Ok(len)
        }

        fn set_nonblocking(&mut self, nonblocking: bool) -> Result<(), Error> {
            self.nonblocking = nonblocking;
            Ok(())
        }
    }

    #[derive(Debug)]
    pub(crate) struct FakeStreamWrite {
        send: Sender<Packet>,
        seqno: SequenceNumber,
        skip: usize,
    }

    impl WriteFD for FakeStreamWrite {
        fn write(&mut self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize, Error> {
            assert!(fds.is_empty());
            if self.skip > 0 {
                assert_eq!(self.skip, buf.len());
                self.skip = 0;
                return Ok(buf.len());
            }

            self.seqno += 1;
            match buf[0] {
                GET_INPUT_FOCUS_REQUEST => self
                    .send
                    .send(Packet::GetInputFocusReply(self.seqno))
                    .unwrap(),
                SEND_EVENT_REQUEST => self.send.send(Packet::Event).unwrap(),
                _ => unimplemented!(),
            }
            // Compute how much of the package was not yet received
            self.skip = usize::from(u16::from_ne_bytes([buf[2], buf[3]])) * 4 - buf.len();
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }
}
