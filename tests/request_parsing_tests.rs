use std::borrow::Cow;

use x11rb::errors::ParseError;
use x11rb::x11_utils::RequestHeader;

macro_rules! add_ne {
    ($data:expr, $add:expr) => {
        $data.extend(&$add.to_ne_bytes())
    };
}

#[test]
fn test_bad_request_header_opcode() {
    use x11rb::protocol::xproto::QueryExtensionRequest;
    let header = RequestHeader {
        major_opcode: 1,
        minor_opcode: 2,
        remaining_length: 0,
    };
    let body = &[];
    assert_eq!(
        QueryExtensionRequest::try_parse_request(header, body),
        Err(ParseError::ParseError)
    );
}

#[test]
fn test_bad_request_header_length() {
    use x11rb::protocol::xproto::CreateWindowRequest;
    let header = RequestHeader {
        major_opcode: 1,
        minor_opcode: 0,
        remaining_length: 42,
    };
    let body = &[];
    assert_eq!(
        CreateWindowRequest::try_parse_request(header, body),
        Err(ParseError::ParseError)
    );
}

#[test]
fn test_create_window1() {
    use x11rb::protocol::xproto::{CreateWindowAux, CreateWindowRequest, Gravity, WindowClass};
    let header = RequestHeader {
        major_opcode: 1,
        minor_opcode: 0x18,
        remaining_length: 10,
    };
    let mut body = vec![];
    add_ne!(body, 0x05c00001u32);
    add_ne!(body, 0x00000513u32);
    add_ne!(body, 0x047bu16);
    add_ne!(body, 0x0134u16);
    add_ne!(body, 0x03f5u16);
    add_ne!(body, 0x033bu16);
    add_ne!(body, 0x0000u16);
    add_ne!(body, 0x0001u16);
    add_ne!(body, 0x00000047u32);
    add_ne!(body, 0x0000001au32);
    add_ne!(body, 0xfff2f1f0u32);
    add_ne!(body, 0x00000000u32);
    body.push(0x01);
    body.extend(&[0x00, 0x00, 0x00]); // Final padding.
    let r = CreateWindowRequest::try_parse_request(header, &body).unwrap();
    assert_eq!(
        r,
        CreateWindowRequest {
            depth: 0x18,
            wid: 0x05c00001,
            parent: 0x00000513,
            x: 0x047b,
            y: 0x0134,
            width: 0x03f5,
            height: 0x033b,
            border_width: 0,
            class: WindowClass::InputOutput,
            visual: 0x47,
            value_list: Cow::Owned(CreateWindowAux {
                background_pixel: Some(0xfff2f1f0),
                border_pixel: Some(0),
                bit_gravity: Some(Gravity::NorthWest),
                ..Default::default()
            }),
        }
    );
}

#[test]
fn test_create_window2() {
    use x11rb::protocol::xproto::{CreateWindowAux, CreateWindowRequest, Gravity, WindowClass};
    let header = RequestHeader {
        major_opcode: 1,
        minor_opcode: 0x18,
        remaining_length: 11,
    };
    let mut body = vec![];
    add_ne!(body, 0x05400003u32);
    add_ne!(body, 0x00000513u32);
    add_ne!(body, 0x00000000u32);
    add_ne!(body, 0x0001u16);
    add_ne!(body, 0x0001u16);
    add_ne!(body, 0x0000u16);
    add_ne!(body, 0x0001u16);
    add_ne!(body, 0x0000035au32);
    add_ne!(body, 0x0000201au32);
    add_ne!(body, 0x00000000u32);
    add_ne!(body, 0x00000000u32);
    add_ne!(body, 0x00000001u32);
    add_ne!(body, 0x05400002u32);
    let r = CreateWindowRequest::try_parse_request(header, &body).unwrap();
    assert_eq!(
        r,
        CreateWindowRequest {
            depth: 0x18,
            wid: 0x05400003,
            parent: 0x00000513,
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            border_width: 0,
            class: WindowClass::InputOutput,
            visual: 0x35a,
            value_list: Cow::Owned(CreateWindowAux {
                background_pixel: Some(0),
                border_pixel: Some(0),
                bit_gravity: Some(Gravity::NorthWest),
                colormap: Some(0x05400002),
                ..Default::default()
            }),
        }
    );
}

#[test]
fn test_change_window_attributes() {
    use x11rb::protocol::xproto::{
        ChangeWindowAttributesAux, ChangeWindowAttributesRequest, EventMask,
    };
    let header = RequestHeader {
        major_opcode: 2,
        minor_opcode: 0,
        remaining_length: 3,
    };
    let mut body = vec![];
    add_ne!(body, 0x00000513u32);
    add_ne!(body, 0x00000800u32);
    add_ne!(body, 0x00400000u32);
    let r = ChangeWindowAttributesRequest::try_parse_request(header, &body).unwrap();
    assert_eq!(
        r,
        ChangeWindowAttributesRequest {
            window: 0x0513,
            value_list: Cow::Owned(ChangeWindowAttributesAux {
                event_mask: Some(EventMask::PropertyChange.into()),
                ..Default::default()
            }),
        }
    );
}

#[test]
fn test_get_window_attributes() {
    use x11rb::protocol::xproto::GetWindowAttributesRequest;
    let header = RequestHeader {
        major_opcode: 3,
        minor_opcode: 0,
        remaining_length: 1,
    };
    let mut body = vec![];
    add_ne!(body, 0x00e00002u32);
    let r = GetWindowAttributesRequest::try_parse_request(header, &body).unwrap();
    assert_eq!(r, GetWindowAttributesRequest { window: 0x00e00002 });
}

#[test]
fn test_get_input_focus() {
    use x11rb::protocol::xproto::GetInputFocusRequest;
    let header = RequestHeader {
        major_opcode: 43,
        minor_opcode: 0,
        remaining_length: 0,
    };
    let body = &[];
    let r = GetInputFocusRequest::try_parse_request(header, body).unwrap();
    assert_eq!(r, GetInputFocusRequest,);
}

#[test]
fn test_query_text_extents() {
    use x11rb::protocol::xproto::{Char2b, QueryTextExtentsRequest};
    let header = RequestHeader {
        major_opcode: 48,
        minor_opcode: 0,
        remaining_length: 2,
    };
    let mut body = vec![];
    add_ne!(body, 0x12345678u32);
    add_ne!(body, 0x9abcu16);
    add_ne!(body, 0xdef0u16);
    let r = QueryTextExtentsRequest::try_parse_request(header, &body).unwrap();
    assert_eq!(
        r,
        QueryTextExtentsRequest {
            font: 0x12345678,
            string: Cow::Owned(vec![
                Char2b {
                    byte1: 0xbc,
                    byte2: 0x9a,
                },
                Char2b {
                    byte1: 0xf0,
                    byte2: 0xde,
                },
            ]),
        }
    );
}

#[test]
fn test_query_text_extents_odd_length() {
    use x11rb::protocol::xproto::{Char2b, QueryTextExtentsRequest};
    let header = RequestHeader {
        major_opcode: 48,
        minor_opcode: 1,
        remaining_length: 2,
    };
    let mut body = vec![];
    add_ne!(body, 0x12345678u32);
    add_ne!(body, 0x9abcu16);
    add_ne!(body, 0xdef0u16);
    let r = QueryTextExtentsRequest::try_parse_request(header, &body).unwrap();
    assert_eq!(
        r,
        QueryTextExtentsRequest {
            font: 0x12345678,
            string: Cow::Owned(vec![Char2b {
                byte1: 0xbc,
                byte2: 0x9a,
            },]),
        }
    );
}

#[cfg(feature = "randr")]
#[test]
fn test_randr_get_output_property() {
    use x11rb::protocol::randr::GetOutputPropertyRequest;
    let header = RequestHeader {
        major_opcode: 140,
        minor_opcode: 15,
        remaining_length: 6,
    };
    let mut body = vec![];
    add_ne!(body, 0x0000008au32);
    add_ne!(body, 0x00000045u32);
    add_ne!(body, 0x00000000u32);
    add_ne!(body, 0x00000000u32);
    add_ne!(body, 0x00000080u32);
    add_ne!(body, 0x00000000u32);
    let r = GetOutputPropertyRequest::try_parse_request(header, &body).unwrap();
    assert_eq!(
        r,
        GetOutputPropertyRequest {
            output: 0x8a,
            property: 0x45,
            type_: 0,
            long_offset: 0,
            long_length: 128,
            delete: false,
            pending: false,
        },
    );
}
