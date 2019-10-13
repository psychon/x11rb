use std::io::IoSlice;
use std::convert::TryFrom;
use std::ops::Deref;
use std::cell::RefCell;

use super::connection::{Connection, Cookie, SequenceNumber};
use super::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};
use super::generated::xproto::{Setup, QueryExtensionReply, poly_segment, Segment};
use super::utils::Buffer;
use super::x11_utils::GenericEvent;

#[derive(Debug)]
struct SavedRequest {
    has_reply: bool,
    data: Vec<u8>
}

impl SavedRequest {
    fn new(has_reply: bool, data: &[IoSlice]) -> SavedRequest {
        let data = data.iter()
            .flat_map(|slice| slice.deref())
            .copied()
            .collect::<Vec<_>>();
        SavedRequest { has_reply, data }
    }
}

#[derive(Debug, Default)]
struct FakeConnection(RefCell<Vec<SavedRequest>>);

impl FakeConnection {
    fn check_requests(&self, expected: &[(bool, Vec<u8>)]) {
        let vec = self.0.borrow();
        for (expected, actual) in expected.iter().zip(vec.iter()) {
            assert_eq!(expected.0, actual.has_reply);
            assert_eq!(actual.data, expected.1);
        }
        assert_eq!(expected.len(), vec.len());
    }
}

impl Connection for FakeConnection {
    fn send_request_with_reply<R>(&self, _bufs: &[IoSlice]) -> Cookie<Self, R>
    where R: TryFrom<Buffer, Error=ParseError>
    {
        unimplemented!()
    }

    fn send_request_without_reply(&self, bufs: &[IoSlice]) -> SequenceNumber {
        self.0.borrow_mut().push(SavedRequest::new(false, bufs));
        0
    }

    fn discard_reply(&self, _sequence: SequenceNumber) {
        unimplemented!()
    }

    fn extension_information(&self, _extension_name: &'static str) -> Option<&QueryExtensionReply> {
        unimplemented!()
    }

    fn wait_for_reply(&self, _sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error> {
        unimplemented!()
    }

    fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError> {
        unimplemented!()
    }

    fn poll_for_event(&self) -> Result<Option<GenericEvent>, ConnectionError> {
        unimplemented!()
    }

    fn flush(&self) {
        unimplemented!()
    }

    fn setup(&self) -> &Setup {
        unimplemented!()
    }

    fn generate_id(&self) -> u32 {
        unimplemented!()
    }
}

#[test]
fn test_poly_segment() -> Result<(), ConnectionErrorOrX11Error> {
    let conn = FakeConnection::default();
    let drawable = 42;
    let gc = 0x1337;
    let segments = [
        Segment { x1: 1, y1: 2, x2: 3, y2: 4 },
        Segment { x1: 5, y1: 6, x2: 7, y2: 8 },
    ];
    let length: u16 = (12 + segments.len() * 8) as u16 / 4;
    poly_segment(&conn, drawable, gc, &segments)?;

    let mut expected = Vec::new();
    expected.push(super::generated::xproto::POLY_SEGMENT_REQUEST);
    expected.push(0); // padding
    expected.extend(&length.to_ne_bytes()); // length, not in the xml
    expected.extend(&drawable.to_ne_bytes());
    expected.extend(&gc.to_ne_bytes());
    // Segments
    for x in 1u16..9u16 {
        expected.extend(&x.to_ne_bytes());
    }
    conn.check_requests(&[(false, expected)]);
    Ok(())
}
