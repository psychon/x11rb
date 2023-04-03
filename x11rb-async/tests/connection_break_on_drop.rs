use futures_lite::future::Ready;

use x11rb::errors::ConnectionError;
use x11rb::protocol::xproto::Setup;
use x11rb::rust_connection::{PollMode, Stream};
use x11rb::utils::RawFdContainer;
use x11rb_async::connection::Connection;
use x11rb_async::rust_connection::{RustConnection, StreamBase};

#[derive(Debug)]
struct FakeStream;

impl Stream for FakeStream {
    fn poll(&self, _: PollMode) -> Result<(), std::io::Error> {
        unimplemented!()
    }
    fn read(&self, _: &mut [u8], _: &mut Vec<RawFdContainer>) -> Result<usize, std::io::Error> {
        unimplemented!()
    }
    fn write(&self, _: &[u8], _: &mut Vec<RawFdContainer>) -> Result<usize, std::io::Error> {
        unimplemented!()
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

#[test]
fn connection_breaks_on_immediate_drop() {
    let setup = Setup {
        resource_id_mask: (1 << 8) - 1,
        ..Default::default()
    };
    let (conn, driver) = RustConnection::for_connected_stream(FakeStream, setup).unwrap();

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
