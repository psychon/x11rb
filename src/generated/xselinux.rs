// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

#![allow(clippy::unreadable_literal)]
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
use super::xproto::*;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "SELinux";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 0);

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let client_major_bytes = client_major.serialize();
    let client_minor_bytes = client_minor.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        client_major_bytes[0],
        client_minor_bytes[0],
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
    pub server_major: u16,
    pub server_minor: u16,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, server_major, server_minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetDeviceCreateContext request
pub const SET_DEVICE_CREATE_CONTEXT_REQUEST: u8 = 1;
pub fn set_device_create_context<'c, Conn>(conn: &'c Conn, context: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * context.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let context_len: u32 = context.len().try_into()?;
    let context_len_bytes = context_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEVICE_CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_len_bytes[0],
        context_len_bytes[1],
        context_len_bytes[2],
        context_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (context).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(context), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetDeviceCreateContext request
pub const GET_DEVICE_CREATE_CONTEXT_REQUEST: u8 = 2;
pub fn get_device_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetDeviceCreateContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetDeviceCreateContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetDeviceCreateContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetDeviceContext request
pub const SET_DEVICE_CONTEXT_REQUEST: u8 = 3;
pub fn set_device_context<'c, Conn>(conn: &'c Conn, device: u32, context: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 1 * context.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_bytes = device.serialize();
    let context_len: u32 = context.len().try_into()?;
    let context_len_bytes = context_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEVICE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_bytes[0],
        device_bytes[1],
        device_bytes[2],
        device_bytes[3],
        context_len_bytes[0],
        context_len_bytes[1],
        context_len_bytes[2],
        context_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (context).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(context), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetDeviceContext request
pub const GET_DEVICE_CONTEXT_REQUEST: u8 = 4;
pub fn get_device_context<Conn>(conn: &Conn, device: u32) -> Result<Cookie<'_, Conn, GetDeviceContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_bytes = device.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_bytes[0],
        device_bytes[1],
        device_bytes[2],
        device_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetDeviceContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetDeviceContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetWindowCreateContext request
pub const SET_WINDOW_CREATE_CONTEXT_REQUEST: u8 = 5;
pub fn set_window_create_context<'c, Conn>(conn: &'c Conn, context: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * context.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let context_len: u32 = context.len().try_into()?;
    let context_len_bytes = context_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_WINDOW_CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_len_bytes[0],
        context_len_bytes[1],
        context_len_bytes[2],
        context_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (context).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(context), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetWindowCreateContext request
pub const GET_WINDOW_CREATE_CONTEXT_REQUEST: u8 = 6;
pub fn get_window_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetWindowCreateContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_WINDOW_CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWindowCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetWindowCreateContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetWindowCreateContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetWindowCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetWindowContext request
pub const GET_WINDOW_CONTEXT_REQUEST: u8 = 7;
pub fn get_window_context<Conn>(conn: &Conn, window: Window) -> Result<Cookie<'_, Conn, GetWindowContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_WINDOW_CONTEXT_REQUEST,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWindowContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetWindowContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetWindowContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetWindowContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub name: Atom,
    pub object_context: Vec<u8>,
    pub data_context: Vec<u8>,
}
impl TryParse for ListItem {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (name, remaining) = Atom::try_parse(remaining)?;
        let (object_context_len, remaining) = u32::try_parse(remaining)?;
        let (data_context_len, remaining) = u32::try_parse(remaining)?;
        let (object_context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, object_context_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (data_context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, data_context_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = ListItem { name, object_context, data_context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListItem {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ListItem {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.name.serialize_into(bytes);
        let object_context_len = self.object_context.len() as u32;
        object_context_len.serialize_into(bytes);
        let data_context_len = self.data_context.len() as u32;
        data_context_len.serialize_into(bytes);
        self.object_context.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.data_context.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}

/// Opcode for the SetPropertyCreateContext request
pub const SET_PROPERTY_CREATE_CONTEXT_REQUEST: u8 = 8;
pub fn set_property_create_context<'c, Conn>(conn: &'c Conn, context: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * context.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let context_len: u32 = context.len().try_into()?;
    let context_len_bytes = context_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_PROPERTY_CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_len_bytes[0],
        context_len_bytes[1],
        context_len_bytes[2],
        context_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (context).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(context), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetPropertyCreateContext request
pub const GET_PROPERTY_CREATE_CONTEXT_REQUEST: u8 = 9;
pub fn get_property_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyCreateContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PROPERTY_CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetPropertyCreateContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetPropertyCreateContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetPropertyUseContext request
pub const SET_PROPERTY_USE_CONTEXT_REQUEST: u8 = 10;
pub fn set_property_use_context<'c, Conn>(conn: &'c Conn, context: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * context.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let context_len: u32 = context.len().try_into()?;
    let context_len_bytes = context_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_PROPERTY_USE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_len_bytes[0],
        context_len_bytes[1],
        context_len_bytes[2],
        context_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (context).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(context), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetPropertyUseContext request
pub const GET_PROPERTY_USE_CONTEXT_REQUEST: u8 = 11;
pub fn get_property_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyUseContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PROPERTY_USE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyUseContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetPropertyUseContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetPropertyUseContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyUseContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPropertyContext request
pub const GET_PROPERTY_CONTEXT_REQUEST: u8 = 12;
pub fn get_property_context<Conn>(conn: &Conn, window: Window, property: Atom) -> Result<Cookie<'_, Conn, GetPropertyContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let property_bytes = property.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PROPERTY_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetPropertyContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetPropertyContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPropertyDataContext request
pub const GET_PROPERTY_DATA_CONTEXT_REQUEST: u8 = 13;
pub fn get_property_data_context<Conn>(conn: &Conn, window: Window, property: Atom) -> Result<Cookie<'_, Conn, GetPropertyDataContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let property_bytes = property.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PROPERTY_DATA_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyDataContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetPropertyDataContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetPropertyDataContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyDataContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListProperties request
pub const LIST_PROPERTIES_REQUEST: u8 = 14;
pub fn list_properties<Conn>(conn: &Conn, window: Window) -> Result<Cookie<'_, Conn, ListPropertiesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_PROPERTIES_REQUEST,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub properties: Vec<ListItem>,
}
impl ListPropertiesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (properties_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (properties, remaining) = crate::x11_utils::parse_list::<ListItem>(remaining, properties_len as usize)?;
        let result = ListPropertiesReply { response_type, sequence, length, properties };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetSelectionCreateContext request
pub const SET_SELECTION_CREATE_CONTEXT_REQUEST: u8 = 15;
pub fn set_selection_create_context<'c, Conn>(conn: &'c Conn, context: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * context.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let context_len: u32 = context.len().try_into()?;
    let context_len_bytes = context_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_SELECTION_CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_len_bytes[0],
        context_len_bytes[1],
        context_len_bytes[2],
        context_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (context).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(context), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetSelectionCreateContext request
pub const GET_SELECTION_CREATE_CONTEXT_REQUEST: u8 = 16;
pub fn get_selection_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionCreateContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SELECTION_CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetSelectionCreateContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetSelectionCreateContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetSelectionUseContext request
pub const SET_SELECTION_USE_CONTEXT_REQUEST: u8 = 17;
pub fn set_selection_use_context<'c, Conn>(conn: &'c Conn, context: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * context.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let context_len: u32 = context.len().try_into()?;
    let context_len_bytes = context_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_SELECTION_USE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_len_bytes[0],
        context_len_bytes[1],
        context_len_bytes[2],
        context_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (context).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(context), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetSelectionUseContext request
pub const GET_SELECTION_USE_CONTEXT_REQUEST: u8 = 18;
pub fn get_selection_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionUseContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SELECTION_USE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionUseContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetSelectionUseContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetSelectionUseContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionUseContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetSelectionContext request
pub const GET_SELECTION_CONTEXT_REQUEST: u8 = 19;
pub fn get_selection_context<Conn>(conn: &Conn, selection: Atom) -> Result<Cookie<'_, Conn, GetSelectionContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let selection_bytes = selection.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SELECTION_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        selection_bytes[0],
        selection_bytes[1],
        selection_bytes[2],
        selection_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetSelectionContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetSelectionContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetSelectionDataContext request
pub const GET_SELECTION_DATA_CONTEXT_REQUEST: u8 = 20;
pub fn get_selection_data_context<Conn>(conn: &Conn, selection: Atom) -> Result<Cookie<'_, Conn, GetSelectionDataContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let selection_bytes = selection.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SELECTION_DATA_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        selection_bytes[0],
        selection_bytes[1],
        selection_bytes[2],
        selection_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionDataContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetSelectionDataContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetSelectionDataContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionDataContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListSelections request
pub const LIST_SELECTIONS_REQUEST: u8 = 21;
pub fn list_selections<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSelectionsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_SELECTIONS_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSelectionsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub selections: Vec<ListItem>,
}
impl ListSelectionsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (selections_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (selections, remaining) = crate::x11_utils::parse_list::<ListItem>(remaining, selections_len as usize)?;
        let result = ListSelectionsReply { response_type, sequence, length, selections };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListSelectionsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetClientContext request
pub const GET_CLIENT_CONTEXT_REQUEST: u8 = 22;
pub fn get_client_context<Conn>(conn: &Conn, resource: u32) -> Result<Cookie<'_, Conn, GetClientContextReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let resource_bytes = resource.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CLIENT_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        resource_bytes[0],
        resource_bytes[1],
        resource_bytes[2],
        resource_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClientContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl GetClientContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_list::<u8>(remaining, context_len as usize)?;
        let result = GetClientContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetClientContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xselinux_query_version(&self, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major, client_minor)
    }

    fn xselinux_set_device_create_context<'c>(&'c self, context: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_create_context(self, context)
    }

    fn xselinux_get_device_create_context(&self) -> Result<Cookie<'_, Self, GetDeviceCreateContextReply>, ConnectionError>
    {
        get_device_create_context(self)
    }

    fn xselinux_set_device_context<'c>(&'c self, device: u32, context: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_context(self, device, context)
    }

    fn xselinux_get_device_context(&self, device: u32) -> Result<Cookie<'_, Self, GetDeviceContextReply>, ConnectionError>
    {
        get_device_context(self, device)
    }

    fn xselinux_set_window_create_context<'c>(&'c self, context: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_window_create_context(self, context)
    }

    fn xselinux_get_window_create_context(&self) -> Result<Cookie<'_, Self, GetWindowCreateContextReply>, ConnectionError>
    {
        get_window_create_context(self)
    }

    fn xselinux_get_window_context(&self, window: Window) -> Result<Cookie<'_, Self, GetWindowContextReply>, ConnectionError>
    {
        get_window_context(self, window)
    }

    fn xselinux_set_property_create_context<'c>(&'c self, context: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_property_create_context(self, context)
    }

    fn xselinux_get_property_create_context(&self) -> Result<Cookie<'_, Self, GetPropertyCreateContextReply>, ConnectionError>
    {
        get_property_create_context(self)
    }

    fn xselinux_set_property_use_context<'c>(&'c self, context: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_property_use_context(self, context)
    }

    fn xselinux_get_property_use_context(&self) -> Result<Cookie<'_, Self, GetPropertyUseContextReply>, ConnectionError>
    {
        get_property_use_context(self)
    }

    fn xselinux_get_property_context(&self, window: Window, property: Atom) -> Result<Cookie<'_, Self, GetPropertyContextReply>, ConnectionError>
    {
        get_property_context(self, window, property)
    }

    fn xselinux_get_property_data_context(&self, window: Window, property: Atom) -> Result<Cookie<'_, Self, GetPropertyDataContextReply>, ConnectionError>
    {
        get_property_data_context(self, window, property)
    }

    fn xselinux_list_properties(&self, window: Window) -> Result<Cookie<'_, Self, ListPropertiesReply>, ConnectionError>
    {
        list_properties(self, window)
    }

    fn xselinux_set_selection_create_context<'c>(&'c self, context: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_selection_create_context(self, context)
    }

    fn xselinux_get_selection_create_context(&self) -> Result<Cookie<'_, Self, GetSelectionCreateContextReply>, ConnectionError>
    {
        get_selection_create_context(self)
    }

    fn xselinux_set_selection_use_context<'c>(&'c self, context: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_selection_use_context(self, context)
    }

    fn xselinux_get_selection_use_context(&self) -> Result<Cookie<'_, Self, GetSelectionUseContextReply>, ConnectionError>
    {
        get_selection_use_context(self)
    }

    fn xselinux_get_selection_context(&self, selection: Atom) -> Result<Cookie<'_, Self, GetSelectionContextReply>, ConnectionError>
    {
        get_selection_context(self, selection)
    }

    fn xselinux_get_selection_data_context(&self, selection: Atom) -> Result<Cookie<'_, Self, GetSelectionDataContextReply>, ConnectionError>
    {
        get_selection_data_context(self, selection)
    }

    fn xselinux_list_selections(&self) -> Result<Cookie<'_, Self, ListSelectionsReply>, ConnectionError>
    {
        list_selections(self)
    }

    fn xselinux_get_client_context(&self, resource: u32) -> Result<Cookie<'_, Self, GetClientContextReply>, ConnectionError>
    {
        get_client_context(self, resource)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
