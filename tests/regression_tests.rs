use std::io::IoSlice;
use std::convert::TryFrom;
use std::ops::Deref;
use std::cell::RefCell;

use x11rb::connection::{RequestConnection, Cookie, CookieWithFds, VoidCookie, SequenceNumber, RequestKind, DiscardMode};
use x11rb::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};
use x11rb::generated::xproto::{QueryExtensionReply, ConnectionExt, Segment, KeymapNotifyEvent, ClientMessageData};
use x11rb::utils::{Buffer, RawFdContainer};
use x11rb::x11_utils::{GenericError, TryParse};

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

    fn internal_send_request(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<SequenceNumber, ConnectionError> {
        assert_eq!(fds.len(), 0);

        let mut storage = Default::default();
        let bufs = self.compute_length_field(bufs, &mut storage)?;

        self.0.borrow_mut().push(SavedRequest::new(false, bufs));
        Ok(0)
    }
}

impl RequestConnection for FakeConnection {
    fn send_request_with_reply<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<Cookie<Self, R>, ConnectionError>
    where R: TryFrom<Buffer, Error=ParseError>
    {
        Ok(Cookie::new(self, self.internal_send_request(bufs, fds)?))
    }

    fn send_request_with_reply_with_fds<R>(&self, _bufs: &[IoSlice], _fds: Vec<RawFdContainer>) -> Result<CookieWithFds<Self, R>, ConnectionError>
    where R: TryFrom<(Buffer, Vec<RawFdContainer>), Error=ParseError>
    {
        unimplemented!()
    }

    fn send_request_without_reply(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<VoidCookie<Self>, ConnectionError> {
        Ok(VoidCookie::new(self, self.internal_send_request(bufs, fds)?))
    }

    fn discard_reply(&self, _sequence: SequenceNumber, _kind: RequestKind, _mode: DiscardMode) {
        // Just ignore this
    }

    fn extension_information(&self, _extension_name: &'static str) -> Option<QueryExtensionReply> {
        unimplemented!()
    }

    fn wait_for_reply_or_error(&self, _sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error> {
        unimplemented!()
    }

    fn wait_for_reply(&self, _sequence: SequenceNumber) -> Result<Option<Buffer>, ConnectionError> {
        unimplemented!()
    }

    fn wait_for_reply_with_fds(&self, _sequence: SequenceNumber) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
        unimplemented!()
    }

    fn check_for_error(&self, _sequence: SequenceNumber) -> Result<Option<GenericError>, ConnectionError> {
        unimplemented!()
    }

    fn maximum_request_bytes(&self) -> usize {
        // Must be at least 4 * 2^16 so that we can test BIG-REQUESTS
        2usize.pow(19)
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
    conn.poly_segment(drawable, gc, &segments)?;

    let mut expected = Vec::new();
    expected.push(x11rb::generated::xproto::POLY_SEGMENT_REQUEST);
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

#[test]
fn test_big_requests() -> Result<(), ConnectionError> {
    let conn = FakeConnection::default();
    let big_buffer = [0; (1 << 18) + 1];
    let drawable: u32 = 42;
    let gc: u32 = 0x1337;
    let x: i16 = 21;
    let y: i16 = 7;
    let padding = 3; // big_buffer's size rounded up to a multiple of 4
    let length: u32 = (16 + big_buffer.len() as u32 + padding) / 4;
    conn.poly_text16(drawable, gc, x, y, &big_buffer)?;

    let mut expected = Vec::new();
    expected.push(x11rb::generated::xproto::POLY_TEXT16_REQUEST);
    expected.push(0); // padding
    // Length of zero: we use big requests
    expected.push(0);
    expected.push(0);
    // Actual length
    expected.extend(&length.to_ne_bytes());

    expected.extend(&drawable.to_ne_bytes());
    expected.extend(&gc.to_ne_bytes());
    expected.extend(&x.to_ne_bytes());
    expected.extend(&y.to_ne_bytes());
    expected.extend(big_buffer.iter());
    expected.extend((0..padding).map(|_| 0));

    conn.check_requests(&[(false, expected)]);
    Ok(())
}

#[test]
fn test_too_large_request() -> Result<(), ConnectionError> {
    let conn = FakeConnection::default();
    let big_buffer = [0; (1 << 19) + 1];
    let drawable: u32 = 42;
    let gc: u32 = 0x1337;
    let x: i16 = 21;
    let y: i16 = 7;
    let res = conn.poly_text16(drawable, gc, x, y, &big_buffer);
    assert_eq!(ConnectionError::MaximumRequestLengthExceeded, res.unwrap_err());
    Ok(())
}

#[test]
fn test_send_event() -> Result<(), ConnectionError> {
    // Prepare the event
    let buffer: [u8; 32] = [11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                            14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                            26, 27, 28, 29, 30];
    let event = KeymapNotifyEvent::try_from(&buffer[..])?;

    // "Send" it
    let conn = FakeConnection::default();
    let propagate = 42;
    let destination = 0x1337;
    let event_mask = 7;
    conn.send_event(propagate, destination, event_mask, event)?;

    let mut expected = Vec::new();
    expected.push(x11rb::generated::xproto::SEND_EVENT_REQUEST);
    expected.push(propagate);
    expected.extend(&((12u16 + 32u16) / 4).to_ne_bytes());
    expected.extend(&destination.to_ne_bytes());
    expected.extend(&event_mask.to_ne_bytes());
    expected.extend(buffer.iter());
    conn.check_requests(&[(false, expected)]);
    Ok(())
}

#[test]
fn test_get_keyboard_mapping() -> Result<(), ConnectionError> {
    let conn = FakeConnection::default();
    let cookie = conn.get_keyboard_mapping(1, 2)?;

    // Prevent call to discard_reply(), we only check request sending
    std::mem::forget(cookie);

    let mut expected = Vec::new();
    let length: u16 = 2;
    expected.push(101); // request major code
    expected.push(0); // padding
    expected.extend(&length.to_ne_bytes()); // length, not in the xml
    expected.push(1); // first keycode
    expected.push(2); // length
    expected.extend(&[0, 0]); // padding

    conn.check_requests(&[(false, expected)]);
    Ok(())
}

#[test]
fn test_client_message_data_parse() {
    let short_array = [0, 0, 0];
    let err = ClientMessageData::try_parse(&short_array);
    assert!(err.is_err());
    assert_eq!(err.unwrap_err(), ParseError::ParseError);
}
