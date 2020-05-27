use std::borrow::Cow;

use x11rb::errors::ParseError;
use x11rb::x11_utils::RequestHeader;

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
    let body = &[
        0x1, 0x0, 0xc0, 0x5, 0x13, 0x5, 0x0, 0x0, 0x7b, 0x4, 0x34, 0x1, 0xf5, 0x3, 0x3b, 0x3, 0x0,
        0x0, 0x1, 0x0, 0x47, 0x0, 0x0, 0x0, 0x1a, 0x0, 0x0, 0x0, 0xf0, 0xf1, 0xf2, 0xff, 0x0, 0x0,
        0x0, 0x0, 0x1, 0x0, 0x0, 0x0,
    ];
    let r = CreateWindowRequest::try_parse_request(header, body).unwrap();
    assert_eq!(
        r,
        CreateWindowRequest {
            depth: 0x18,
            wid: 0x05c00001,
            parent: 0x00000513,
            x: 1147,
            y: 308,
            width: 1013,
            height: 827,
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
    let body = &[
        0x3, 0x0, 0x40, 0x5, 0x13, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x1, 0x0, 0x0, 0x0,
        0x1, 0x0, 0x5a, 0x3, 0x0, 0x0, 0x1a, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0x0, 0x0, 0x0, 0x2, 0x0, 0x40, 0x5,
    ];
    let r = CreateWindowRequest::try_parse_request(header, body).unwrap();
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
    let body = &[0x13, 0x5, 0x0, 0x0, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x40, 0x0];
    let r = ChangeWindowAttributesRequest::try_parse_request(header, body).unwrap();
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
    let body = &[0x2, 0x0, 0xe0, 0x0];
    let r = GetWindowAttributesRequest::try_parse_request(header, body).unwrap();
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

#[cfg(feature = "randr")]
#[test]
fn test_randr_get_output_property() {
    use x11rb::protocol::randr::GetOutputPropertyRequest;
    let header = RequestHeader {
        major_opcode: 140,
        minor_opcode: 15,
        remaining_length: 6,
    };
    let body = &[
        0x8a, 0x0, 0x0, 0x0, 0x45, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x80,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    let r = GetOutputPropertyRequest::try_parse_request(header, body).unwrap();
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
