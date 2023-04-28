use futures_lite::future::Ready;
use std::io::IoSlice;
use std::sync::{Arc, Mutex};

use x11rb::errors::ConnectionError;
use x11rb::protocol::xproto::Setup;
use x11rb::rust_connection::{PollMode, Stream as SyncStream};
use x11rb::utils::RawFdContainer;
use x11rb_async::connection::{Connection, RequestConnection};
use x11rb_async::rust_connection::{RustConnection, Stream as AsyncStream, StreamBase};

#[derive(Debug, Default)]
struct FakeStream(Arc<Mutex<Vec<u8>>>);

impl SyncStream for FakeStream {
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

fn send_request_via_connection(
    stream: impl AsyncStream + Send + Sync,
    length: usize,
    fds: Vec<RawFdContainer>,
) -> Option<ConnectionError> {
    let (conn, _driver) = RustConnection::for_connected_stream(stream, make_setup()).unwrap();

    // Send a large request. This should be larger than the write buffer size, which is 16384 bytes.
    let mut request = vec![0; length];

    // Set the length field correctly (x11rb-async expects this)
    request[2..4].copy_from_slice(&u16::try_from(length / 4).unwrap().to_ne_bytes());

    async_io::block_on(conn.send_request_without_reply(&[IoSlice::new(&request)], fds)).err()
}

#[test]
fn connection_sends_large_request() {
    let data = Default::default();

    // Send a large request. This should be larger than the write buffer size, which is 16384 bytes.
    let length = 16384 * 2;
    let res = send_request_via_connection(FakeStream(Arc::clone(&data)), length, Vec::new());
    assert!(res.is_none(), "{:?}", res);

    // Check that all the data was sent
    assert_eq!(data.lock().unwrap().len(), length);
}

#[test]
#[cfg(unix)]
fn retry_for_left_over_fds() {
    // FakeStream ignores FDs. This should result in some error and not be silently ignored.
    // Right now that error is WriteZero. That is still better than no error at all.

    let fds = {
        let (fd0, fd1) = rustix::io::pipe().unwrap();
        vec![RawFdContainer::new(fd0), RawFdContainer::new(fd1)]
    };

    // Send a large request. This should be larger than the write buffer size, which is 16384 bytes.
    let length = 16384 * 2;
    match send_request_via_connection(FakeStream(Default::default()), length, fds) {
        Some(ConnectionError::IoError(e)) => {
            assert_eq!(e.kind(), std::io::ErrorKind::Other);
            assert_eq!(e.to_string(), "Left over FDs after sending the request");
        }
        e => panic!("Unexpected error: {:?}", e),
    }
}
