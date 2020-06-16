// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xinerama` X11 extension.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]

#[allow(unused_imports)]
use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
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
    fn serialize(&self) -> [u8; 8] {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub major: u8,
    pub minor: u8,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let major_bytes = self.major.serialize();
        let minor_bytes = self.minor.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            major_bytes[0],
            minor_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (major, remaining) = u8::try_parse(value)?;
        let (minor, remaining) = u8::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            major,
            minor,
        })
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn, major: u8, minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major,
        minor,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major, remaining) = u16::try_parse(remaining)?;
        let (minor, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = QueryVersionReply { sequence, length, major, minor };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::ParseError)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStateRequest {
    pub window: xproto::Window,
}
impl GetStateRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_STATE_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_STATE_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetStateRequest {
            window,
        })
    }
}
impl Request for GetStateRequest {
    type Reply = GetStateReply;
}
pub fn get_state<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetStateRequest {
        window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStateReply {
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: xproto::Window,
}
impl TryParse for GetStateReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = GetStateReply { state, sequence, length, window };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::ParseError)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenCountRequest {
    pub window: xproto::Window,
}
impl GetScreenCountRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SCREEN_COUNT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SCREEN_COUNT_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetScreenCountRequest {
            window,
        })
    }
}
impl Request for GetScreenCountRequest {
    type Reply = GetScreenCountReply;
}
pub fn get_screen_count<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenCountReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetScreenCountRequest {
        window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenCountReply {
    pub screen_count: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: xproto::Window,
}
impl TryParse for GetScreenCountReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (screen_count, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = GetScreenCountReply { screen_count, sequence, length, window };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::ParseError)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenSizeRequest {
    pub window: xproto::Window,
    pub screen: u32,
}
impl GetScreenSizeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SCREEN_SIZE_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SCREEN_SIZE_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (screen, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetScreenSizeRequest {
            window,
            screen,
        })
    }
}
impl Request for GetScreenSizeRequest {
    type Reply = GetScreenSizeReply;
}
pub fn get_screen_size<Conn>(conn: &Conn, window: xproto::Window, screen: u32) -> Result<Cookie<'_, Conn, GetScreenSizeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetScreenSizeRequest {
        window,
        screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenSizeReply {
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub window: xproto::Window,
    pub screen: u32,
}
impl TryParse for GetScreenSizeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u32::try_parse(remaining)?;
        let (height, remaining) = u32::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (screen, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = GetScreenSizeReply { sequence, length, width, height, window, screen };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::ParseError)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsActiveRequest;
impl IsActiveRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            IS_ACTIVE_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != IS_ACTIVE_REQUEST {
            return Err(ParseError::ParseError);
        }
        let _ = value;
        Ok(IsActiveRequest
        )
    }
}
impl Request for IsActiveRequest {
    type Reply = IsActiveReply;
}
pub fn is_active<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, IsActiveReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsActiveRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsActiveReply {
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}
impl TryParse for IsActiveReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = IsActiveReply { sequence, length, state };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::ParseError)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryScreensRequest;
impl QueryScreensRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_SCREENS_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_SCREENS_REQUEST {
            return Err(ParseError::ParseError);
        }
        let _ = value;
        Ok(QueryScreensRequest
        )
    }
}
impl Request for QueryScreensRequest {
    type Reply = QueryScreensReply;
}
pub fn query_screens<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryScreensReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryScreensRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryScreensReply {
    pub sequence: u16,
    pub length: u32,
    pub screen_info: Vec<ScreenInfo>,
}
impl TryParse for QueryScreensReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (number, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (screen_info, remaining) = crate::x11_utils::parse_list::<ScreenInfo>(remaining, number.try_into().or(Err(ParseError::ParseError))?)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = QueryScreensReply { sequence, length, screen_info };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::ParseError)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryScreensReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl QueryScreensReply {
    /// Get the value of the `number` field.
    ///
    /// The `number` field is used as the length field of the `screen_info` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn number(&self) -> u32 {
        self.screen_info.len()
            .try_into().unwrap()
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
