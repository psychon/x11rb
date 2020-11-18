// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `SELinux` X11 extension.

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
use crate::utils::{BitmaskPrettyPrinter, EnumPrettyPrinter, RawFdContainer};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
use super::xproto;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub client_major: u8,
    pub client_minor: u8,
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
        let client_major_bytes = self.client_major.serialize();
        let client_minor_bytes = self.client_minor.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            client_major_bytes[0],
            client_minor_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (client_major, remaining) = u8::try_parse(value)?;
        let (client_minor, remaining) = u8::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            client_major,
            client_minor,
        })
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major,
        client_minor,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply { sequence, length, server_major, server_minor };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetDeviceCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetDeviceCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_DEVICE_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.context, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(SetDeviceCreateContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetDeviceCreateContextRequest.
    pub fn into_owned(self) -> SetDeviceCreateContextRequest<'static> {
        SetDeviceCreateContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetDeviceCreateContextRequest<'input> {
    type Reply = ();
}
pub fn set_device_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    request0.send(conn)
}

/// Opcode for the GetDeviceCreateContext request
pub const GET_DEVICE_CREATE_CONTEXT_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDeviceCreateContextRequest;
impl GetDeviceCreateContextRequest {
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
            GET_DEVICE_CREATE_CONTEXT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetDeviceCreateContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetDeviceCreateContextRequest
        )
    }
}
impl Request for GetDeviceCreateContextRequest {
    type Reply = GetDeviceCreateContextReply;
}
pub fn get_device_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetDeviceCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceCreateContextRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetDeviceCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceCreateContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetDeviceCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetDeviceContext request
pub const SET_DEVICE_CONTEXT_REQUEST: u8 = 3;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetDeviceContextRequest<'input> {
    pub device: u32,
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetDeviceContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_bytes = self.device.serialize();
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_DEVICE_CONTEXT_REQUEST,
            0,
            0,
            device_bytes[0],
            device_bytes[1],
            device_bytes[2],
            device_bytes[3],
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.context, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device, remaining) = u32::try_parse(value)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(SetDeviceContextRequest {
            device,
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetDeviceContextRequest.
    pub fn into_owned(self) -> SetDeviceContextRequest<'static> {
        SetDeviceContextRequest {
            device: self.device,
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetDeviceContextRequest<'input> {
    type Reply = ();
}
pub fn set_device_context<'c, 'input, Conn>(conn: &'c Conn, device: u32, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceContextRequest {
        device,
        context: Cow::Borrowed(context),
    };
    request0.send(conn)
}

/// Opcode for the GetDeviceContext request
pub const GET_DEVICE_CONTEXT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDeviceContextRequest {
    pub device: u32,
}
impl GetDeviceContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_bytes = self.device.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_DEVICE_CONTEXT_REQUEST,
            0,
            0,
            device_bytes[0],
            device_bytes[1],
            device_bytes[2],
            device_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetDeviceContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(GetDeviceContextRequest {
            device,
        })
    }
}
impl Request for GetDeviceContextRequest {
    type Reply = GetDeviceContextReply;
}
pub fn get_device_context<Conn>(conn: &Conn, device: u32) -> Result<Cookie<'_, Conn, GetDeviceContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceContextRequest {
        device,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetDeviceContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetDeviceContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetWindowCreateContext request
pub const SET_WINDOW_CREATE_CONTEXT_REQUEST: u8 = 5;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetWindowCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetWindowCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_WINDOW_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.context, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_WINDOW_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(SetWindowCreateContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetWindowCreateContextRequest.
    pub fn into_owned(self) -> SetWindowCreateContextRequest<'static> {
        SetWindowCreateContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetWindowCreateContextRequest<'input> {
    type Reply = ();
}
pub fn set_window_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetWindowCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    request0.send(conn)
}

/// Opcode for the GetWindowCreateContext request
pub const GET_WINDOW_CREATE_CONTEXT_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetWindowCreateContextRequest;
impl GetWindowCreateContextRequest {
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
            GET_WINDOW_CREATE_CONTEXT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetWindowCreateContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_WINDOW_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetWindowCreateContextRequest
        )
    }
}
impl Request for GetWindowCreateContextRequest {
    type Reply = GetWindowCreateContextReply;
}
pub fn get_window_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetWindowCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetWindowCreateContextRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWindowCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetWindowCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetWindowCreateContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetWindowCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetWindowCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetWindowContext request
pub const GET_WINDOW_CONTEXT_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetWindowContextRequest {
    pub window: xproto::Window,
}
impl GetWindowContextRequest {
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
            GET_WINDOW_CONTEXT_REQUEST,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetWindowContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_WINDOW_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetWindowContextRequest {
            window,
        })
    }
}
impl Request for GetWindowContextRequest {
    type Reply = GetWindowContextReply;
}
pub fn get_window_context<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetWindowContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetWindowContextRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWindowContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetWindowContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetWindowContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetWindowContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetWindowContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub name: xproto::Atom,
    pub object_context: Vec<u8>,
    pub data_context: Vec<u8>,
}
impl TryParse for ListItem {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (object_context_len, remaining) = u32::try_parse(remaining)?;
        let (data_context_len, remaining) = u32::try_parse(remaining)?;
        let (object_context, remaining) = crate::x11_utils::parse_u8_list(remaining, object_context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let object_context = object_context.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (data_context, remaining) = crate::x11_utils::parse_u8_list(remaining, data_context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let data_context = data_context.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
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
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.name.serialize_into(bytes);
        let object_context_len = u32::try_from(self.object_context.len()).expect("`object_context` has too many elements");
        object_context_len.serialize_into(bytes);
        let data_context_len = u32::try_from(self.data_context.len()).expect("`data_context` has too many elements");
        data_context_len.serialize_into(bytes);
        bytes.extend_from_slice(&self.object_context);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        bytes.extend_from_slice(&self.data_context);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl ListItem {
    /// Get the value of the `object_context_len` field.
    ///
    /// The `object_context_len` field is used as the length field of the `object_context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn object_context_len(&self) -> u32 {
        self.object_context.len()
            .try_into().unwrap()
    }
    /// Get the value of the `data_context_len` field.
    ///
    /// The `data_context_len` field is used as the length field of the `data_context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn data_context_len(&self) -> u32 {
        self.data_context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetPropertyCreateContext request
pub const SET_PROPERTY_CREATE_CONTEXT_REQUEST: u8 = 8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetPropertyCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetPropertyCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_PROPERTY_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.context, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_PROPERTY_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(SetPropertyCreateContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetPropertyCreateContextRequest.
    pub fn into_owned(self) -> SetPropertyCreateContextRequest<'static> {
        SetPropertyCreateContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetPropertyCreateContextRequest<'input> {
    type Reply = ();
}
pub fn set_property_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPropertyCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    request0.send(conn)
}

/// Opcode for the GetPropertyCreateContext request
pub const GET_PROPERTY_CREATE_CONTEXT_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPropertyCreateContextRequest;
impl GetPropertyCreateContextRequest {
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
            GET_PROPERTY_CREATE_CONTEXT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyCreateContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROPERTY_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetPropertyCreateContextRequest
        )
    }
}
impl Request for GetPropertyCreateContextRequest {
    type Reply = GetPropertyCreateContextReply;
}
pub fn get_property_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyCreateContextRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyCreateContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetPropertyCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetPropertyUseContext request
pub const SET_PROPERTY_USE_CONTEXT_REQUEST: u8 = 10;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetPropertyUseContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetPropertyUseContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_PROPERTY_USE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.context, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_PROPERTY_USE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(SetPropertyUseContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetPropertyUseContextRequest.
    pub fn into_owned(self) -> SetPropertyUseContextRequest<'static> {
        SetPropertyUseContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetPropertyUseContextRequest<'input> {
    type Reply = ();
}
pub fn set_property_use_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPropertyUseContextRequest {
        context: Cow::Borrowed(context),
    };
    request0.send(conn)
}

/// Opcode for the GetPropertyUseContext request
pub const GET_PROPERTY_USE_CONTEXT_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPropertyUseContextRequest;
impl GetPropertyUseContextRequest {
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
            GET_PROPERTY_USE_CONTEXT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyUseContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROPERTY_USE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetPropertyUseContextRequest
        )
    }
}
impl Request for GetPropertyUseContextRequest {
    type Reply = GetPropertyUseContextReply;
}
pub fn get_property_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyUseContextRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyUseContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyUseContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyUseContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyUseContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetPropertyUseContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetPropertyContext request
pub const GET_PROPERTY_CONTEXT_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPropertyContextRequest {
    pub window: xproto::Window,
    pub property: xproto::Atom,
}
impl GetPropertyContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_PROPERTY_CONTEXT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROPERTY_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPropertyContextRequest {
            window,
            property,
        })
    }
}
impl Request for GetPropertyContextRequest {
    type Reply = GetPropertyContextReply;
}
pub fn get_property_context<Conn>(conn: &Conn, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Conn, GetPropertyContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyContextRequest {
        window,
        property,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetPropertyContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetPropertyDataContext request
pub const GET_PROPERTY_DATA_CONTEXT_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPropertyDataContextRequest {
    pub window: xproto::Window,
    pub property: xproto::Atom,
}
impl GetPropertyDataContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_PROPERTY_DATA_CONTEXT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyDataContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROPERTY_DATA_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPropertyDataContextRequest {
            window,
            property,
        })
    }
}
impl Request for GetPropertyDataContextRequest {
    type Reply = GetPropertyDataContextReply;
}
pub fn get_property_data_context<Conn>(conn: &Conn, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Conn, GetPropertyDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyDataContextRequest {
        window,
        property,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyDataContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyDataContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyDataContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyDataContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetPropertyDataContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the ListProperties request
pub const LIST_PROPERTIES_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListPropertiesRequest {
    pub window: xproto::Window,
}
impl ListPropertiesRequest {
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
            LIST_PROPERTIES_REQUEST,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListPropertiesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_PROPERTIES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(ListPropertiesRequest {
            window,
        })
    }
}
impl Request for ListPropertiesRequest {
    type Reply = ListPropertiesReply;
}
pub fn list_properties<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, ListPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListPropertiesRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPropertiesReply {
    pub sequence: u16,
    pub length: u32,
    pub properties: Vec<ListItem>,
}
impl TryParse for ListPropertiesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (properties_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (properties, remaining) = crate::x11_utils::parse_list::<ListItem>(remaining, properties_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListPropertiesReply { sequence, length, properties };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl ListPropertiesReply {
    /// Get the value of the `properties_len` field.
    ///
    /// The `properties_len` field is used as the length field of the `properties` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn properties_len(&self) -> u32 {
        self.properties.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetSelectionCreateContext request
pub const SET_SELECTION_CREATE_CONTEXT_REQUEST: u8 = 15;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSelectionCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetSelectionCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_SELECTION_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.context, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_SELECTION_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(SetSelectionCreateContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetSelectionCreateContextRequest.
    pub fn into_owned(self) -> SetSelectionCreateContextRequest<'static> {
        SetSelectionCreateContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetSelectionCreateContextRequest<'input> {
    type Reply = ();
}
pub fn set_selection_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetSelectionCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    request0.send(conn)
}

/// Opcode for the GetSelectionCreateContext request
pub const GET_SELECTION_CREATE_CONTEXT_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionCreateContextRequest;
impl GetSelectionCreateContextRequest {
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
            GET_SELECTION_CREATE_CONTEXT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionCreateContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SELECTION_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetSelectionCreateContextRequest
        )
    }
}
impl Request for GetSelectionCreateContextRequest {
    type Reply = GetSelectionCreateContextReply;
}
pub fn get_selection_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionCreateContextRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionCreateContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSelectionCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetSelectionUseContext request
pub const SET_SELECTION_USE_CONTEXT_REQUEST: u8 = 17;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSelectionUseContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetSelectionUseContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_SELECTION_USE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.context, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_SELECTION_USE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(SetSelectionUseContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetSelectionUseContextRequest.
    pub fn into_owned(self) -> SetSelectionUseContextRequest<'static> {
        SetSelectionUseContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetSelectionUseContextRequest<'input> {
    type Reply = ();
}
pub fn set_selection_use_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetSelectionUseContextRequest {
        context: Cow::Borrowed(context),
    };
    request0.send(conn)
}

/// Opcode for the GetSelectionUseContext request
pub const GET_SELECTION_USE_CONTEXT_REQUEST: u8 = 18;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionUseContextRequest;
impl GetSelectionUseContextRequest {
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
            GET_SELECTION_USE_CONTEXT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionUseContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SELECTION_USE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetSelectionUseContextRequest
        )
    }
}
impl Request for GetSelectionUseContextRequest {
    type Reply = GetSelectionUseContextReply;
}
pub fn get_selection_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionUseContextRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionUseContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionUseContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionUseContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionUseContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSelectionUseContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetSelectionContext request
pub const GET_SELECTION_CONTEXT_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionContextRequest {
    pub selection: xproto::Atom,
}
impl GetSelectionContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let selection_bytes = self.selection.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SELECTION_CONTEXT_REQUEST,
            0,
            0,
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SELECTION_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (selection, remaining) = xproto::Atom::try_parse(value)?;
        let _ = remaining;
        Ok(GetSelectionContextRequest {
            selection,
        })
    }
}
impl Request for GetSelectionContextRequest {
    type Reply = GetSelectionContextReply;
}
pub fn get_selection_context<Conn>(conn: &Conn, selection: xproto::Atom) -> Result<Cookie<'_, Conn, GetSelectionContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionContextRequest {
        selection,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSelectionContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetSelectionDataContext request
pub const GET_SELECTION_DATA_CONTEXT_REQUEST: u8 = 20;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionDataContextRequest {
    pub selection: xproto::Atom,
}
impl GetSelectionDataContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let selection_bytes = self.selection.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SELECTION_DATA_CONTEXT_REQUEST,
            0,
            0,
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionDataContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SELECTION_DATA_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (selection, remaining) = xproto::Atom::try_parse(value)?;
        let _ = remaining;
        Ok(GetSelectionDataContextRequest {
            selection,
        })
    }
}
impl Request for GetSelectionDataContextRequest {
    type Reply = GetSelectionDataContextReply;
}
pub fn get_selection_data_context<Conn>(conn: &Conn, selection: xproto::Atom) -> Result<Cookie<'_, Conn, GetSelectionDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionDataContextRequest {
        selection,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionDataContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionDataContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionDataContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionDataContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSelectionDataContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Opcode for the ListSelections request
pub const LIST_SELECTIONS_REQUEST: u8 = 21;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListSelectionsRequest;
impl ListSelectionsRequest {
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
            LIST_SELECTIONS_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListSelectionsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_SELECTIONS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(ListSelectionsRequest
        )
    }
}
impl Request for ListSelectionsRequest {
    type Reply = ListSelectionsReply;
}
pub fn list_selections<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSelectionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSelectionsRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSelectionsReply {
    pub sequence: u16,
    pub length: u32,
    pub selections: Vec<ListItem>,
}
impl TryParse for ListSelectionsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (selections_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (selections, remaining) = crate::x11_utils::parse_list::<ListItem>(remaining, selections_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListSelectionsReply { sequence, length, selections };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListSelectionsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl ListSelectionsReply {
    /// Get the value of the `selections_len` field.
    ///
    /// The `selections_len` field is used as the length field of the `selections` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn selections_len(&self) -> u32 {
        self.selections.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetClientContext request
pub const GET_CLIENT_CONTEXT_REQUEST: u8 = 22;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetClientContextRequest {
    pub resource: u32,
}
impl GetClientContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let resource_bytes = self.resource.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_CLIENT_CONTEXT_REQUEST,
            0,
            0,
            resource_bytes[0],
            resource_bytes[1],
            resource_bytes[2],
            resource_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetClientContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CLIENT_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (resource, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(GetClientContextRequest {
            resource,
        })
    }
}
impl Request for GetClientContextRequest {
    type Reply = GetClientContextReply;
}
pub fn get_client_context<Conn>(conn: &Conn, resource: u32) -> Result<Cookie<'_, Conn, GetClientContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClientContextRequest {
        resource,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClientContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetClientContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetClientContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetClientContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetClientContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xselinux_query_version(&self, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major, client_minor)
    }
    fn xselinux_set_device_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_create_context(self, context)
    }
    fn xselinux_get_device_create_context(&self) -> Result<Cookie<'_, Self, GetDeviceCreateContextReply>, ConnectionError>
    {
        get_device_create_context(self)
    }
    fn xselinux_set_device_context<'c, 'input>(&'c self, device: u32, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_context(self, device, context)
    }
    fn xselinux_get_device_context(&self, device: u32) -> Result<Cookie<'_, Self, GetDeviceContextReply>, ConnectionError>
    {
        get_device_context(self, device)
    }
    fn xselinux_set_window_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_window_create_context(self, context)
    }
    fn xselinux_get_window_create_context(&self) -> Result<Cookie<'_, Self, GetWindowCreateContextReply>, ConnectionError>
    {
        get_window_create_context(self)
    }
    fn xselinux_get_window_context(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetWindowContextReply>, ConnectionError>
    {
        get_window_context(self, window)
    }
    fn xselinux_set_property_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_property_create_context(self, context)
    }
    fn xselinux_get_property_create_context(&self) -> Result<Cookie<'_, Self, GetPropertyCreateContextReply>, ConnectionError>
    {
        get_property_create_context(self)
    }
    fn xselinux_set_property_use_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_property_use_context(self, context)
    }
    fn xselinux_get_property_use_context(&self) -> Result<Cookie<'_, Self, GetPropertyUseContextReply>, ConnectionError>
    {
        get_property_use_context(self)
    }
    fn xselinux_get_property_context(&self, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Self, GetPropertyContextReply>, ConnectionError>
    {
        get_property_context(self, window, property)
    }
    fn xselinux_get_property_data_context(&self, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Self, GetPropertyDataContextReply>, ConnectionError>
    {
        get_property_data_context(self, window, property)
    }
    fn xselinux_list_properties(&self, window: xproto::Window) -> Result<Cookie<'_, Self, ListPropertiesReply>, ConnectionError>
    {
        list_properties(self, window)
    }
    fn xselinux_set_selection_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_selection_create_context(self, context)
    }
    fn xselinux_get_selection_create_context(&self) -> Result<Cookie<'_, Self, GetSelectionCreateContextReply>, ConnectionError>
    {
        get_selection_create_context(self)
    }
    fn xselinux_set_selection_use_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_selection_use_context(self, context)
    }
    fn xselinux_get_selection_use_context(&self) -> Result<Cookie<'_, Self, GetSelectionUseContextReply>, ConnectionError>
    {
        get_selection_use_context(self)
    }
    fn xselinux_get_selection_context(&self, selection: xproto::Atom) -> Result<Cookie<'_, Self, GetSelectionContextReply>, ConnectionError>
    {
        get_selection_context(self, selection)
    }
    fn xselinux_get_selection_data_context(&self, selection: xproto::Atom) -> Result<Cookie<'_, Self, GetSelectionDataContextReply>, ConnectionError>
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
