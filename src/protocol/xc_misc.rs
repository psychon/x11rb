// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XCMisc` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd, TryIntoUSize};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::connection::Connection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
#[allow(unused_imports)]
use crate::errors::ReplyOrIdError;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XC-MISC";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

/// Opcode for the GetVersion request
pub const GET_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVersionRequest {
    pub client_major_version: u16,
    pub client_minor_version: u16,
}
impl GetVersionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let client_major_version_bytes = self.client_major_version.serialize();
        let client_minor_version_bytes = self.client_minor_version.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_VERSION_REQUEST,
            0,
            0,
            client_major_version_bytes[0],
            client_major_version_bytes[1],
            client_minor_version_bytes[0],
            client_minor_version_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetVersionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (client_major_version, remaining) = u16::try_parse(value)?;
        let (client_minor_version, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetVersionRequest {
            client_major_version,
            client_minor_version,
        })
    }
}
impl Request for GetVersionRequest {
    type Reply = GetVersionReply;
}
pub fn get_version<Conn>(conn: &Conn, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Conn, GetVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetVersionRequest {
        client_major_version,
        client_minor_version,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl TryParse for GetVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major_version, remaining) = u16::try_parse(remaining)?;
        let (server_minor_version, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetVersionReply { sequence, length, server_major_version, server_minor_version };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the GetXIDRange request
pub const GET_XID_RANGE_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetXIDRangeRequest;
impl GetXIDRangeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_XID_RANGE_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetXIDRangeReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_XID_RANGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetXIDRangeRequest
        )
    }
}
impl Request for GetXIDRangeRequest {
    type Reply = GetXIDRangeReply;
}
pub fn get_xid_range<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetXIDRangeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetXIDRangeRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetXIDRangeReply {
    pub sequence: u16,
    pub length: u32,
    pub start_id: u32,
    pub count: u32,
}
impl TryParse for GetXIDRangeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (start_id, remaining) = u32::try_parse(remaining)?;
        let (count, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetXIDRangeReply { sequence, length, start_id, count };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the GetXIDList request
pub const GET_XID_LIST_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetXIDListRequest {
    pub count: u32,
}
impl GetXIDListRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let count_bytes = self.count.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_XID_LIST_REQUEST,
            0,
            0,
            count_bytes[0],
            count_bytes[1],
            count_bytes[2],
            count_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetXIDListReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_XID_LIST_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (count, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(GetXIDListRequest {
            count,
        })
    }
}
impl Request for GetXIDListRequest {
    type Reply = GetXIDListReply;
}
pub fn get_xid_list<Conn>(conn: &Conn, count: u32) -> Result<Cookie<'_, Conn, GetXIDListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetXIDListRequest {
        count,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetXIDListReply {
    pub sequence: u16,
    pub length: u32,
    pub ids: Vec<u32>,
}
impl TryParse for GetXIDListReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ids_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (ids, remaining) = crate::x11_utils::parse_list::<u32>(remaining, ids_len.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetXIDListReply { sequence, length, ids };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetXIDListReply {
    /// Get the value of the `ids_len` field.
    ///
    /// The `ids_len` field is used as the length field of the `ids` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn ids_len(&self) -> u32 {
        self.ids.len()
            .try_into().unwrap()
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xc_misc_get_version(&self, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Self, GetVersionReply>, ConnectionError>
    {
        get_version(self, client_major_version, client_minor_version)
    }
    fn xc_misc_get_xid_range(&self) -> Result<Cookie<'_, Self, GetXIDRangeReply>, ConnectionError>
    {
        get_xid_range(self)
    }
    fn xc_misc_get_xid_list(&self, count: u32) -> Result<Cookie<'_, Self, GetXIDListReply>, ConnectionError>
    {
        get_xid_list(self, count)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
