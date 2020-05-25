// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XF86Dri` X11 extension.

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
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XFree86-DRI";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (4, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrmClipRect {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub x3: i16,
}
impl TryParse for DrmClipRect {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (x1, remaining) = i16::try_parse(remaining)?;
        let (y1, remaining) = i16::try_parse(remaining)?;
        let (x2, remaining) = i16::try_parse(remaining)?;
        let (x3, remaining) = i16::try_parse(remaining)?;
        let result = DrmClipRect { x1, y1, x2, x3 };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DrmClipRect {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DrmClipRect {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let x1_bytes = self.x1.serialize();
        let y1_bytes = self.y1.serialize();
        let x2_bytes = self.x2.serialize();
        let x3_bytes = self.x3.serialize();
        [
            x1_bytes[0],
            x1_bytes[1],
            y1_bytes[0],
            y1_bytes[1],
            x2_bytes[0],
            x2_bytes[1],
            x3_bytes[0],
            x3_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.x1.serialize_into(bytes);
        self.y1.serialize_into(bytes);
        self.x2.serialize_into(bytes);
        self.x3.serialize_into(bytes);
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest;
impl QueryVersionRequest {
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
            QUERY_VERSION_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub dri_major_version: u16,
    pub dri_minor_version: u16,
    pub dri_minor_patch: u32,
}
impl TryParse for QueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (dri_major_version, remaining) = u16::try_parse(remaining)?;
        let (dri_minor_version, remaining) = u16::try_parse(remaining)?;
        let (dri_minor_patch, remaining) = u32::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, dri_major_version, dri_minor_version, dri_minor_patch };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryDirectRenderingCapable request
pub const QUERY_DIRECT_RENDERING_CAPABLE_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryDirectRenderingCapableRequest {
    pub screen: u32,
}
impl QueryDirectRenderingCapableRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_DIRECT_RENDERING_CAPABLE_REQUEST,
            0,
            0,
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
}
pub fn query_direct_rendering_capable<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, QueryDirectRenderingCapableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryDirectRenderingCapableRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryDirectRenderingCapableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_capable: bool,
}
impl TryParse for QueryDirectRenderingCapableReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (is_capable, remaining) = bool::try_parse(remaining)?;
        let result = QueryDirectRenderingCapableReply { response_type, sequence, length, is_capable };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryDirectRenderingCapableReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the OpenConnection request
pub const OPEN_CONNECTION_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenConnectionRequest {
    pub screen: u32,
}
impl OpenConnectionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            OPEN_CONNECTION_REQUEST,
            0,
            0,
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
}
pub fn open_connection<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, OpenConnectionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = OpenConnectionRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenConnectionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub sarea_handle_low: u32,
    pub sarea_handle_high: u32,
    pub bus_id: Vec<u8>,
}
impl TryParse for OpenConnectionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (sarea_handle_low, remaining) = u32::try_parse(remaining)?;
        let (sarea_handle_high, remaining) = u32::try_parse(remaining)?;
        let (bus_id_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (bus_id, remaining) = crate::x11_utils::parse_u8_list(remaining, bus_id_len.try_into().or(Err(ParseError::ParseError))?)?;
        let bus_id = bus_id.to_vec();
        let result = OpenConnectionReply { response_type, sequence, length, sarea_handle_low, sarea_handle_high, bus_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for OpenConnectionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl OpenConnectionReply {
    /// Get the value of the `bus_id_len` field.
    ///
    /// The `bus_id_len` field is used as the length field of the `bus_id` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn bus_id_len(&self) -> u32 {
        self.bus_id.len()
            .try_into().unwrap()
    }
}

/// Opcode for the CloseConnection request
pub const CLOSE_CONNECTION_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloseConnectionRequest {
    pub screen: u32,
}
impl CloseConnectionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CLOSE_CONNECTION_REQUEST,
            0,
            0,
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
}
pub fn close_connection<Conn>(conn: &Conn, screen: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CloseConnectionRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the GetClientDriverName request
pub const GET_CLIENT_DRIVER_NAME_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetClientDriverNameRequest {
    pub screen: u32,
}
impl GetClientDriverNameRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_CLIENT_DRIVER_NAME_REQUEST,
            0,
            0,
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
}
pub fn get_client_driver_name<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetClientDriverNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClientDriverNameRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClientDriverNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub client_driver_major_version: u32,
    pub client_driver_minor_version: u32,
    pub client_driver_patch_version: u32,
    pub client_driver_name: Vec<u8>,
}
impl TryParse for GetClientDriverNameReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (client_driver_major_version, remaining) = u32::try_parse(remaining)?;
        let (client_driver_minor_version, remaining) = u32::try_parse(remaining)?;
        let (client_driver_patch_version, remaining) = u32::try_parse(remaining)?;
        let (client_driver_name_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (client_driver_name, remaining) = crate::x11_utils::parse_u8_list(remaining, client_driver_name_len.try_into().or(Err(ParseError::ParseError))?)?;
        let client_driver_name = client_driver_name.to_vec();
        let result = GetClientDriverNameReply { response_type, sequence, length, client_driver_major_version, client_driver_minor_version, client_driver_patch_version, client_driver_name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetClientDriverNameReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetClientDriverNameReply {
    /// Get the value of the `client_driver_name_len` field.
    ///
    /// The `client_driver_name_len` field is used as the length field of the `client_driver_name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn client_driver_name_len(&self) -> u32 {
        self.client_driver_name.len()
            .try_into().unwrap()
    }
}

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateContextRequest {
    pub screen: u32,
    pub visual: u32,
    pub context: u32,
}
impl CreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let visual_bytes = self.visual.serialize();
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_CONTEXT_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            visual_bytes[0],
            visual_bytes[1],
            visual_bytes[2],
            visual_bytes[3],
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_context<Conn>(conn: &Conn, screen: u32, visual: u32, context: u32) -> Result<Cookie<'_, Conn, CreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        screen,
        visual,
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_context: u32,
}
impl TryParse for CreateContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (hw_context, remaining) = u32::try_parse(remaining)?;
        let result = CreateContextReply { response_type, sequence, length, hw_context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DestroyContext request
pub const DESTROY_CONTEXT_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyContextRequest {
    pub screen: u32,
    pub context: u32,
}
impl DestroyContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_CONTEXT_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn destroy_context<Conn>(conn: &Conn, screen: u32, context: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyContextRequest {
        screen,
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the CreateDrawable request
pub const CREATE_DRAWABLE_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateDrawableRequest {
    pub screen: u32,
    pub drawable: u32,
}
impl CreateDrawableRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_DRAWABLE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_drawable<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<Cookie<'_, Conn, CreateDrawableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateDrawableRequest {
        screen,
        drawable,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateDrawableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_drawable_handle: u32,
}
impl TryParse for CreateDrawableReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (hw_drawable_handle, remaining) = u32::try_parse(remaining)?;
        let result = CreateDrawableReply { response_type, sequence, length, hw_drawable_handle };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CreateDrawableReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DestroyDrawable request
pub const DESTROY_DRAWABLE_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyDrawableRequest {
    pub screen: u32,
    pub drawable: u32,
}
impl DestroyDrawableRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_DRAWABLE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn destroy_drawable<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyDrawableRequest {
        screen,
        drawable,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the GetDrawableInfo request
pub const GET_DRAWABLE_INFO_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDrawableInfoRequest {
    pub screen: u32,
    pub drawable: u32,
}
impl GetDrawableInfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_DRAWABLE_INFO_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_drawable_info<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<Cookie<'_, Conn, GetDrawableInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDrawableInfoRequest {
        screen,
        drawable,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDrawableInfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub drawable_table_index: u32,
    pub drawable_table_stamp: u32,
    pub drawable_origin_x: i16,
    pub drawable_origin_y: i16,
    pub drawable_size_w: i16,
    pub drawable_size_h: i16,
    pub back_x: i16,
    pub back_y: i16,
    pub clip_rects: Vec<DrmClipRect>,
    pub back_clip_rects: Vec<DrmClipRect>,
}
impl TryParse for GetDrawableInfoReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (drawable_table_index, remaining) = u32::try_parse(remaining)?;
        let (drawable_table_stamp, remaining) = u32::try_parse(remaining)?;
        let (drawable_origin_x, remaining) = i16::try_parse(remaining)?;
        let (drawable_origin_y, remaining) = i16::try_parse(remaining)?;
        let (drawable_size_w, remaining) = i16::try_parse(remaining)?;
        let (drawable_size_h, remaining) = i16::try_parse(remaining)?;
        let (num_clip_rects, remaining) = u32::try_parse(remaining)?;
        let (back_x, remaining) = i16::try_parse(remaining)?;
        let (back_y, remaining) = i16::try_parse(remaining)?;
        let (num_back_clip_rects, remaining) = u32::try_parse(remaining)?;
        let (clip_rects, remaining) = crate::x11_utils::parse_list::<DrmClipRect>(remaining, num_clip_rects.try_into().or(Err(ParseError::ParseError))?)?;
        let (back_clip_rects, remaining) = crate::x11_utils::parse_list::<DrmClipRect>(remaining, num_back_clip_rects.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetDrawableInfoReply { response_type, sequence, length, drawable_table_index, drawable_table_stamp, drawable_origin_x, drawable_origin_y, drawable_size_w, drawable_size_h, back_x, back_y, clip_rects, back_clip_rects };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDrawableInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetDrawableInfoReply {
    /// Get the value of the `num_clip_rects` field.
    ///
    /// The `num_clip_rects` field is used as the length field of the `clip_rects` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_clip_rects(&self) -> u32 {
        self.clip_rects.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_back_clip_rects` field.
    ///
    /// The `num_back_clip_rects` field is used as the length field of the `back_clip_rects` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_back_clip_rects(&self) -> u32 {
        self.back_clip_rects.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetDeviceInfo request
pub const GET_DEVICE_INFO_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDeviceInfoRequest {
    pub screen: u32,
}
impl GetDeviceInfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_DEVICE_INFO_REQUEST,
            0,
            0,
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
}
pub fn get_device_info<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetDeviceInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceInfoRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceInfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub framebuffer_handle_low: u32,
    pub framebuffer_handle_high: u32,
    pub framebuffer_origin_offset: u32,
    pub framebuffer_size: u32,
    pub framebuffer_stride: u32,
    pub device_private: Vec<u32>,
}
impl TryParse for GetDeviceInfoReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (framebuffer_handle_low, remaining) = u32::try_parse(remaining)?;
        let (framebuffer_handle_high, remaining) = u32::try_parse(remaining)?;
        let (framebuffer_origin_offset, remaining) = u32::try_parse(remaining)?;
        let (framebuffer_size, remaining) = u32::try_parse(remaining)?;
        let (framebuffer_stride, remaining) = u32::try_parse(remaining)?;
        let (device_private_size, remaining) = u32::try_parse(remaining)?;
        let (device_private, remaining) = crate::x11_utils::parse_list::<u32>(remaining, device_private_size.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetDeviceInfoReply { response_type, sequence, length, framebuffer_handle_low, framebuffer_handle_high, framebuffer_origin_offset, framebuffer_size, framebuffer_stride, device_private };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetDeviceInfoReply {
    /// Get the value of the `device_private_size` field.
    ///
    /// The `device_private_size` field is used as the length field of the `device_private` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn device_private_size(&self) -> u32 {
        self.device_private.len()
            .try_into().unwrap()
    }
}

/// Opcode for the AuthConnection request
pub const AUTH_CONNECTION_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthConnectionRequest {
    pub screen: u32,
    pub magic: u32,
}
impl AuthConnectionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let magic_bytes = self.magic.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            AUTH_CONNECTION_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            magic_bytes[0],
            magic_bytes[1],
            magic_bytes[2],
            magic_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn auth_connection<Conn>(conn: &Conn, screen: u32, magic: u32) -> Result<Cookie<'_, Conn, AuthConnectionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AuthConnectionRequest {
        screen,
        magic,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthConnectionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: u32,
}
impl TryParse for AuthConnectionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (authenticated, remaining) = u32::try_parse(remaining)?;
        let result = AuthConnectionReply { response_type, sequence, length, authenticated };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AuthConnectionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xf86dri_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }
    fn xf86dri_query_direct_rendering_capable(&self, screen: u32) -> Result<Cookie<'_, Self, QueryDirectRenderingCapableReply>, ConnectionError>
    {
        query_direct_rendering_capable(self, screen)
    }
    fn xf86dri_open_connection(&self, screen: u32) -> Result<Cookie<'_, Self, OpenConnectionReply>, ConnectionError>
    {
        open_connection(self, screen)
    }
    fn xf86dri_close_connection(&self, screen: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        close_connection(self, screen)
    }
    fn xf86dri_get_client_driver_name(&self, screen: u32) -> Result<Cookie<'_, Self, GetClientDriverNameReply>, ConnectionError>
    {
        get_client_driver_name(self, screen)
    }
    fn xf86dri_create_context(&self, screen: u32, visual: u32, context: u32) -> Result<Cookie<'_, Self, CreateContextReply>, ConnectionError>
    {
        create_context(self, screen, visual, context)
    }
    fn xf86dri_destroy_context(&self, screen: u32, context: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_context(self, screen, context)
    }
    fn xf86dri_create_drawable(&self, screen: u32, drawable: u32) -> Result<Cookie<'_, Self, CreateDrawableReply>, ConnectionError>
    {
        create_drawable(self, screen, drawable)
    }
    fn xf86dri_destroy_drawable(&self, screen: u32, drawable: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_drawable(self, screen, drawable)
    }
    fn xf86dri_get_drawable_info(&self, screen: u32, drawable: u32) -> Result<Cookie<'_, Self, GetDrawableInfoReply>, ConnectionError>
    {
        get_drawable_info(self, screen, drawable)
    }
    fn xf86dri_get_device_info(&self, screen: u32) -> Result<Cookie<'_, Self, GetDeviceInfoReply>, ConnectionError>
    {
        get_device_info(self, screen)
    }
    fn xf86dri_auth_connection(&self, screen: u32, magic: u32) -> Result<Cookie<'_, Self, AuthConnectionReply>, ConnectionError>
    {
        auth_connection(self, screen, magic)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
