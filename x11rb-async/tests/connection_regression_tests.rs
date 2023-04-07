use futures_lite::future::Ready;
use std::io::IoSlice;
use std::sync::{Arc, Mutex};

use x11rb::errors::ConnectionError;
use x11rb::protocol::xproto::Setup;
use x11rb::rust_connection::{PollMode, Stream};
use x11rb::utils::RawFdContainer;
use x11rb_async::connection::{Connection, RequestConnection};
use x11rb_async::rust_connection::{RustConnection, StreamBase};

#[derive(Debug, Default)]
struct FakeStream(Arc<Mutex<Vec<u8>>>);

impl Stream for FakeStream {
    fn poll(&self, _: PollMode) -> Result<(), std::io::Error> {
        unimplemented!()
    }
    fn read(&self, _: &mut [u8], _: &mut Vec<RawFdContainer>) -> Result<usize, std::io::Error> {
        unimplemented!()
    }
    fn write(&self, buf: &[u8], _: &mut Vec<RawFdContainer>) -> Result<usize, std::io::Error> {
        self.0.lock().unwrap().extend(buf);
        Ok(buf.len())
    }
}

impl StreamBase<'_> for FakeStream {
    type Readable = Ready<std::io::Result<()>>;
    type Writable = Ready<std::io::Result<()>>;

    fn readable(&self) -> Self::Readable {
        unimplemented!()
    }

    fn writable(&self) -> Self::Writable {
        unimplemented!()
    }
}

fn make_setup() -> Setup {
    Setup {
        resource_id_mask: (1 << 8) - 1,
        ..Default::default()
    }
}

#[test]
fn connection_breaks_on_immediate_drop() {
    let (conn, driver) =
        RustConnection::for_connected_stream(FakeStream::default(), make_setup()).unwrap();

    // Drop the driver future. This should break the connection.
    drop(driver);

    // Check that waiting for input now errors.
    match async_io::block_on(conn.wait_for_event()) {
        Err(ConnectionError::IoError(err)) => {
            assert_eq!(err.kind(), std::io::ErrorKind::Other);
            assert_eq!(
                err.into_inner().unwrap().to_string(),
                "Driving future was dropped"
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn connection_sends_large_request() {
    let data = Default::default();
    let stream = FakeStream(Arc::clone(&data));
    let (conn, _driver) = RustConnection::for_connected_stream(stream, make_setup()).unwrap();

    // Send a large request. This should be larger than the write buffer size, which is 16384 bytes.
    let length = 16384 * 2;
    let mut request = vec![0; length];

    // Set the length field correctly (x11rb-async expects this)
    request[2..4].copy_from_slice(&u16::try_from(length / 4).unwrap().to_ne_bytes());

    async_io::block_on(conn.send_request_without_reply(&[IoSlice::new(&request)], Vec::new()))
        .unwrap();

    // Check that all the data was sent
    assert_eq!(data.lock().unwrap().len(), length);
}
