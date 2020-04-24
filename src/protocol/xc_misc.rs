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
#[allow(unused_imports)]
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XC-MISC";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

/// Opcode for the GetVersion request
pub const GET_VERSION_REQUEST: u8 = 0;
pub fn get_version<Conn>(conn: &Conn, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Conn, GetVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let client_major_version_bytes = client_major_version.serialize();
    let client_minor_version_bytes = client_minor_version.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl TryParse for GetVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major_version, remaining) = u16::try_parse(remaining)?;
        let (server_minor_version, remaining) = u16::try_parse(remaining)?;
        let result = GetVersionReply { response_type, sequence, length, server_major_version, server_minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetXIDRange request
pub const GET_XID_RANGE_REQUEST: u8 = 1;
pub fn get_xid_range<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetXIDRangeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let mut request0 = [
        extension_information.major_opcode,
        GET_XID_RANGE_REQUEST,
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetXIDRangeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub start_id: u32,
    pub count: u32,
}
impl TryParse for GetXIDRangeReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (start_id, remaining) = u32::try_parse(remaining)?;
        let (count, remaining) = u32::try_parse(remaining)?;
        let result = GetXIDRangeReply { response_type, sequence, length, start_id, count };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetXIDRangeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetXIDList request
pub const GET_XID_LIST_REQUEST: u8 = 2;
pub fn get_xid_list<Conn>(conn: &Conn, count: u32) -> Result<Cookie<'_, Conn, GetXIDListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let count_bytes = count.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetXIDListReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ids: Vec<u32>,
}
impl TryParse for GetXIDListReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ids_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (ids, remaining) = crate::x11_utils::parse_list::<u32>(remaining, ids_len.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetXIDListReply { response_type, sequence, length, ids };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetXIDListReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
