// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::Event as _;
use crate::x11_utils::{TryParse, Serialize};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ParseError, ConnectionError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;
#[allow(unused_imports)]
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XINERAMA";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenInfo {
    pub x_org: i16,
    pub y_org: i16,
    pub width: u16,
    pub height: u16,
}
impl TryParse for ScreenInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (x_org, remaining) = i16::try_parse(remaining)?;
        let (y_org, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let result = ScreenInfo { x_org, y_org, width, height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ScreenInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ScreenInfo {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let x_org_bytes = self.x_org.serialize();
        let y_org_bytes = self.y_org.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        [
            x_org_bytes[0],
            x_org_bytes[1],
            y_org_bytes[0],
            y_org_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.x_org.serialize_into(bytes);
        self.y_org.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, major: u8, minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let major_bytes = major.serialize();
    let minor_bytes = minor.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        major_bytes[0],
        minor_bytes[0],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major, remaining) = u16::try_parse(remaining)?;
        let (minor, remaining) = u16::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, major, minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetState request
pub const GET_STATE_REQUEST: u8 = 1;
pub fn get_state<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetStateReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_STATE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStateReply {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: xproto::Window,
}
impl GetStateReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let result = GetStateReply { response_type, state, sequence, length, window };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetStateReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetScreenCount request
pub const GET_SCREEN_COUNT_REQUEST: u8 = 2;
pub fn get_screen_count<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenCountReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SCREEN_COUNT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenCountReply {
    pub response_type: u8,
    pub screen_count: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: xproto::Window,
}
impl GetScreenCountReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (screen_count, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let result = GetScreenCountReply { response_type, screen_count, sequence, length, window };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenCountReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetScreenSize request
pub const GET_SCREEN_SIZE_REQUEST: u8 = 3;
pub fn get_screen_size<Conn>(conn: &Conn, window: xproto::Window, screen: u32) -> Result<Cookie<'_, Conn, GetScreenSizeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SCREEN_SIZE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub window: xproto::Window,
    pub screen: u32,
}
impl GetScreenSizeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u32::try_parse(remaining)?;
        let (height, remaining) = u32::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (screen, remaining) = u32::try_parse(remaining)?;
        let result = GetScreenSizeReply { response_type, sequence, length, width, height, window, screen };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenSizeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the IsActive request
pub const IS_ACTIVE_REQUEST: u8 = 4;
pub fn is_active<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, IsActiveReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        IS_ACTIVE_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsActiveReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}
impl IsActiveReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        let result = IsActiveReply { response_type, sequence, length, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IsActiveReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryScreens request
pub const QUERY_SCREENS_REQUEST: u8 = 5;
pub fn query_screens<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryScreensReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_SCREENS_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryScreensReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub screen_info: Vec<ScreenInfo>,
}
impl QueryScreensReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (number, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (screen_info, remaining) = crate::x11_utils::parse_list::<ScreenInfo>(remaining, number as usize)?;
        let result = QueryScreensReply { response_type, sequence, length, screen_info };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryScreensReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xinerama_query_version(&self, major: u8, minor: u8) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major, minor)
    }

    fn xinerama_get_state(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetStateReply>, ConnectionError>
    {
        get_state(self, window)
    }

    fn xinerama_get_screen_count(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetScreenCountReply>, ConnectionError>
    {
        get_screen_count(self, window)
    }

    fn xinerama_get_screen_size(&self, window: xproto::Window, screen: u32) -> Result<Cookie<'_, Self, GetScreenSizeReply>, ConnectionError>
    {
        get_screen_size(self, window, screen)
    }

    fn xinerama_is_active(&self) -> Result<Cookie<'_, Self, IsActiveReply>, ConnectionError>
    {
        is_active(self)
    }

    fn xinerama_query_screens(&self) -> Result<Cookie<'_, Self, QueryScreensReply>, ConnectionError>
    {
        query_screens(self)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
