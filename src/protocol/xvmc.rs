// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XvMC` X11 extension.

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
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
use super::xv;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XVideo-MotionCompensation";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

pub type Context = u32;

pub type Surface = u32;

pub type Subpicture = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceInfo {
    pub id: Surface,
    pub chroma_format: u16,
    pub pad0: u16,
    pub max_width: u16,
    pub max_height: u16,
    pub subpicture_max_width: u16,
    pub subpicture_max_height: u16,
    pub mc_type: u32,
    pub flags: u32,
}
impl TryParse for SurfaceInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (id, remaining) = Surface::try_parse(remaining)?;
        let (chroma_format, remaining) = u16::try_parse(remaining)?;
        let (pad0, remaining) = u16::try_parse(remaining)?;
        let (max_width, remaining) = u16::try_parse(remaining)?;
        let (max_height, remaining) = u16::try_parse(remaining)?;
        let (subpicture_max_width, remaining) = u16::try_parse(remaining)?;
        let (subpicture_max_height, remaining) = u16::try_parse(remaining)?;
        let (mc_type, remaining) = u32::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let result = SurfaceInfo { id, chroma_format, pad0, max_width, max_height, subpicture_max_width, subpicture_max_height, mc_type, flags };
        Ok((result, remaining))
    }
}
impl Serialize for SurfaceInfo {
    type Bytes = [u8; 24];
    fn serialize(&self) -> [u8; 24] {
        let id_bytes = self.id.serialize();
        let chroma_format_bytes = self.chroma_format.serialize();
        let pad0_bytes = self.pad0.serialize();
        let max_width_bytes = self.max_width.serialize();
        let max_height_bytes = self.max_height.serialize();
        let subpicture_max_width_bytes = self.subpicture_max_width.serialize();
        let subpicture_max_height_bytes = self.subpicture_max_height.serialize();
        let mc_type_bytes = self.mc_type.serialize();
        let flags_bytes = self.flags.serialize();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            chroma_format_bytes[0],
            chroma_format_bytes[1],
            pad0_bytes[0],
            pad0_bytes[1],
            max_width_bytes[0],
            max_width_bytes[1],
            max_height_bytes[0],
            max_height_bytes[1],
            subpicture_max_width_bytes[0],
            subpicture_max_width_bytes[1],
            subpicture_max_height_bytes[0],
            subpicture_max_height_bytes[1],
            mc_type_bytes[0],
            mc_type_bytes[1],
            mc_type_bytes[2],
            mc_type_bytes[3],
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.id.serialize_into(bytes);
        self.chroma_format.serialize_into(bytes);
        self.pad0.serialize_into(bytes);
        self.max_width.serialize_into(bytes);
        self.max_height.serialize_into(bytes);
        self.subpicture_max_width.serialize_into(bytes);
        self.subpicture_max_height.serialize_into(bytes);
        self.mc_type.serialize_into(bytes);
        self.flags.serialize_into(bytes);
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(QueryVersionRequest
        )
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major: u32,
    pub minor: u32,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major, remaining) = u32::try_parse(remaining)?;
        let (minor, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply { sequence, length, major, minor };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the ListSurfaceTypes request
pub const LIST_SURFACE_TYPES_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListSurfaceTypesRequest {
    pub port_id: xv::Port,
}
impl ListSurfaceTypesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_id_bytes = self.port_id.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            LIST_SURFACE_TYPES_REQUEST,
            0,
            0,
            port_id_bytes[0],
            port_id_bytes[1],
            port_id_bytes[2],
            port_id_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListSurfaceTypesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_SURFACE_TYPES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port_id, remaining) = xv::Port::try_parse(value)?;
        let _ = remaining;
        Ok(ListSurfaceTypesRequest {
            port_id,
        })
    }
}
impl Request for ListSurfaceTypesRequest {
    type Reply = ListSurfaceTypesReply;
}
pub fn list_surface_types<Conn>(conn: &Conn, port_id: xv::Port) -> Result<Cookie<'_, Conn, ListSurfaceTypesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSurfaceTypesRequest {
        port_id,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSurfaceTypesReply {
    pub sequence: u16,
    pub length: u32,
    pub surfaces: Vec<SurfaceInfo>,
}
impl TryParse for ListSurfaceTypesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (surfaces, remaining) = crate::x11_utils::parse_list::<SurfaceInfo>(remaining, num.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListSurfaceTypesReply { sequence, length, surfaces };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl ListSurfaceTypesReply {
    /// Get the value of the `num` field.
    ///
    /// The `num` field is used as the length field of the `surfaces` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num(&self) -> u32 {
        self.surfaces.len()
            .try_into().unwrap()
    }
}

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateContextRequest {
    pub context_id: Context,
    pub port_id: xv::Port,
    pub surface_id: Surface,
    pub width: u16,
    pub height: u16,
    pub flags: u32,
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
        let context_id_bytes = self.context_id.serialize();
        let port_id_bytes = self.port_id.serialize();
        let surface_id_bytes = self.surface_id.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let flags_bytes = self.flags.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_id_bytes[0],
            context_id_bytes[1],
            context_id_bytes[2],
            context_id_bytes[3],
            port_id_bytes[0],
            port_id_bytes[1],
            port_id_bytes[2],
            port_id_bytes[3],
            surface_id_bytes[0],
            surface_id_bytes[1],
            surface_id_bytes[2],
            surface_id_bytes[3],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, CreateContextReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_id, remaining) = Context::try_parse(value)?;
        let (port_id, remaining) = xv::Port::try_parse(remaining)?;
        let (surface_id, remaining) = Surface::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateContextRequest {
            context_id,
            port_id,
            surface_id,
            width,
            height,
            flags,
        })
    }
}
impl Request for CreateContextRequest {
    type Reply = CreateContextReply;
}
pub fn create_context<Conn>(conn: &Conn, context_id: Context, port_id: xv::Port, surface_id: Surface, width: u16, height: u16, flags: u32) -> Result<Cookie<'_, Conn, CreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        context_id,
        port_id,
        surface_id,
        width,
        height,
        flags,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateContextReply {
    pub sequence: u16,
    pub width_actual: u16,
    pub height_actual: u16,
    pub flags_return: u32,
    pub priv_data: Vec<u32>,
}
impl TryParse for CreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width_actual, remaining) = u16::try_parse(remaining)?;
        let (height_actual, remaining) = u16::try_parse(remaining)?;
        let (flags_return, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (priv_data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = CreateContextReply { sequence, width_actual, height_actual, flags_return, priv_data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl CreateContextReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `priv_data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.priv_data.len()
            .try_into().unwrap()
    }
}

/// Opcode for the DestroyContext request
pub const DESTROY_CONTEXT_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyContextRequest {
    pub context_id: Context,
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
        let context_id_bytes = self.context_id.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_CONTEXT_REQUEST,
            0,
            0,
            context_id_bytes[0],
            context_id_bytes[1],
            context_id_bytes[2],
            context_id_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_id, remaining) = Context::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyContextRequest {
            context_id,
        })
    }
}
impl Request for DestroyContextRequest {
    type Reply = ();
}
pub fn destroy_context<Conn>(conn: &Conn, context_id: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyContextRequest {
        context_id,
    };
    request0.send(conn)
}

/// Opcode for the CreateSurface request
pub const CREATE_SURFACE_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateSurfaceRequest {
    pub surface_id: Surface,
    pub context_id: Context,
}
impl CreateSurfaceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let surface_id_bytes = self.surface_id.serialize();
        let context_id_bytes = self.context_id.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_SURFACE_REQUEST,
            0,
            0,
            surface_id_bytes[0],
            surface_id_bytes[1],
            surface_id_bytes[2],
            surface_id_bytes[3],
            context_id_bytes[0],
            context_id_bytes[1],
            context_id_bytes[2],
            context_id_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, CreateSurfaceReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_SURFACE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (surface_id, remaining) = Surface::try_parse(value)?;
        let (context_id, remaining) = Context::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateSurfaceRequest {
            surface_id,
            context_id,
        })
    }
}
impl Request for CreateSurfaceRequest {
    type Reply = CreateSurfaceReply;
}
pub fn create_surface<Conn>(conn: &Conn, surface_id: Surface, context_id: Context) -> Result<Cookie<'_, Conn, CreateSurfaceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateSurfaceRequest {
        surface_id,
        context_id,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSurfaceReply {
    pub sequence: u16,
    pub priv_data: Vec<u32>,
}
impl TryParse for CreateSurfaceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        let (priv_data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = CreateSurfaceReply { sequence, priv_data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl CreateSurfaceReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `priv_data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.priv_data.len()
            .try_into().unwrap()
    }
}

/// Opcode for the DestroySurface request
pub const DESTROY_SURFACE_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroySurfaceRequest {
    pub surface_id: Surface,
}
impl DestroySurfaceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let surface_id_bytes = self.surface_id.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_SURFACE_REQUEST,
            0,
            0,
            surface_id_bytes[0],
            surface_id_bytes[1],
            surface_id_bytes[2],
            surface_id_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_SURFACE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (surface_id, remaining) = Surface::try_parse(value)?;
        let _ = remaining;
        Ok(DestroySurfaceRequest {
            surface_id,
        })
    }
}
impl Request for DestroySurfaceRequest {
    type Reply = ();
}
pub fn destroy_surface<Conn>(conn: &Conn, surface_id: Surface) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroySurfaceRequest {
        surface_id,
    };
    request0.send(conn)
}

/// Opcode for the CreateSubpicture request
pub const CREATE_SUBPICTURE_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateSubpictureRequest {
    pub subpicture_id: Subpicture,
    pub context: Context,
    pub xvimage_id: u32,
    pub width: u16,
    pub height: u16,
}
impl CreateSubpictureRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let subpicture_id_bytes = self.subpicture_id.serialize();
        let context_bytes = self.context.serialize();
        let xvimage_id_bytes = self.xvimage_id.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_SUBPICTURE_REQUEST,
            0,
            0,
            subpicture_id_bytes[0],
            subpicture_id_bytes[1],
            subpicture_id_bytes[2],
            subpicture_id_bytes[3],
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
            xvimage_id_bytes[0],
            xvimage_id_bytes[1],
            xvimage_id_bytes[2],
            xvimage_id_bytes[3],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, CreateSubpictureReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_SUBPICTURE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (subpicture_id, remaining) = Subpicture::try_parse(value)?;
        let (context, remaining) = Context::try_parse(remaining)?;
        let (xvimage_id, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateSubpictureRequest {
            subpicture_id,
            context,
            xvimage_id,
            width,
            height,
        })
    }
}
impl Request for CreateSubpictureRequest {
    type Reply = CreateSubpictureReply;
}
pub fn create_subpicture<Conn>(conn: &Conn, subpicture_id: Subpicture, context: Context, xvimage_id: u32, width: u16, height: u16) -> Result<Cookie<'_, Conn, CreateSubpictureReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateSubpictureRequest {
        subpicture_id,
        context,
        xvimage_id,
        width,
        height,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSubpictureReply {
    pub sequence: u16,
    pub width_actual: u16,
    pub height_actual: u16,
    pub num_palette_entries: u16,
    pub entry_bytes: u16,
    pub component_order: [u8; 4],
    pub priv_data: Vec<u32>,
}
impl TryParse for CreateSubpictureReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width_actual, remaining) = u16::try_parse(remaining)?;
        let (height_actual, remaining) = u16::try_parse(remaining)?;
        let (num_palette_entries, remaining) = u16::try_parse(remaining)?;
        let (entry_bytes, remaining) = u16::try_parse(remaining)?;
        let (component_order, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let component_order = <[u8; 4]>::try_from(component_order).unwrap();
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (priv_data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = CreateSubpictureReply { sequence, width_actual, height_actual, num_palette_entries, entry_bytes, component_order, priv_data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl CreateSubpictureReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `priv_data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.priv_data.len()
            .try_into().unwrap()
    }
}

/// Opcode for the DestroySubpicture request
pub const DESTROY_SUBPICTURE_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroySubpictureRequest {
    pub subpicture_id: Subpicture,
}
impl DestroySubpictureRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let subpicture_id_bytes = self.subpicture_id.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_SUBPICTURE_REQUEST,
            0,
            0,
            subpicture_id_bytes[0],
            subpicture_id_bytes[1],
            subpicture_id_bytes[2],
            subpicture_id_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_SUBPICTURE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (subpicture_id, remaining) = Subpicture::try_parse(value)?;
        let _ = remaining;
        Ok(DestroySubpictureRequest {
            subpicture_id,
        })
    }
}
impl Request for DestroySubpictureRequest {
    type Reply = ();
}
pub fn destroy_subpicture<Conn>(conn: &Conn, subpicture_id: Subpicture) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroySubpictureRequest {
        subpicture_id,
    };
    request0.send(conn)
}

/// Opcode for the ListSubpictureTypes request
pub const LIST_SUBPICTURE_TYPES_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListSubpictureTypesRequest {
    pub port_id: xv::Port,
    pub surface_id: Surface,
}
impl ListSubpictureTypesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_id_bytes = self.port_id.serialize();
        let surface_id_bytes = self.surface_id.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            LIST_SUBPICTURE_TYPES_REQUEST,
            0,
            0,
            port_id_bytes[0],
            port_id_bytes[1],
            port_id_bytes[2],
            port_id_bytes[3],
            surface_id_bytes[0],
            surface_id_bytes[1],
            surface_id_bytes[2],
            surface_id_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListSubpictureTypesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_SUBPICTURE_TYPES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port_id, remaining) = xv::Port::try_parse(value)?;
        let (surface_id, remaining) = Surface::try_parse(remaining)?;
        let _ = remaining;
        Ok(ListSubpictureTypesRequest {
            port_id,
            surface_id,
        })
    }
}
impl Request for ListSubpictureTypesRequest {
    type Reply = ListSubpictureTypesReply;
}
pub fn list_subpicture_types<Conn>(conn: &Conn, port_id: xv::Port, surface_id: Surface) -> Result<Cookie<'_, Conn, ListSubpictureTypesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSubpictureTypesRequest {
        port_id,
        surface_id,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSubpictureTypesReply {
    pub sequence: u16,
    pub length: u32,
    pub types: Vec<xv::ImageFormatInfo>,
}
impl TryParse for ListSubpictureTypesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (types, remaining) = crate::x11_utils::parse_list::<xv::ImageFormatInfo>(remaining, num.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListSubpictureTypesReply { sequence, length, types };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl ListSubpictureTypesReply {
    /// Get the value of the `num` field.
    ///
    /// The `num` field is used as the length field of the `types` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num(&self) -> u32 {
        self.types.len()
            .try_into().unwrap()
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xvmc_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }
    fn xvmc_list_surface_types(&self, port_id: xv::Port) -> Result<Cookie<'_, Self, ListSurfaceTypesReply>, ConnectionError>
    {
        list_surface_types(self, port_id)
    }
    fn xvmc_create_context(&self, context_id: Context, port_id: xv::Port, surface_id: Surface, width: u16, height: u16, flags: u32) -> Result<Cookie<'_, Self, CreateContextReply>, ConnectionError>
    {
        create_context(self, context_id, port_id, surface_id, width, height, flags)
    }
    fn xvmc_destroy_context(&self, context_id: Context) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_context(self, context_id)
    }
    fn xvmc_create_surface(&self, surface_id: Surface, context_id: Context) -> Result<Cookie<'_, Self, CreateSurfaceReply>, ConnectionError>
    {
        create_surface(self, surface_id, context_id)
    }
    fn xvmc_destroy_surface(&self, surface_id: Surface) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_surface(self, surface_id)
    }
    fn xvmc_create_subpicture(&self, subpicture_id: Subpicture, context: Context, xvimage_id: u32, width: u16, height: u16) -> Result<Cookie<'_, Self, CreateSubpictureReply>, ConnectionError>
    {
        create_subpicture(self, subpicture_id, context, xvimage_id, width, height)
    }
    fn xvmc_destroy_subpicture(&self, subpicture_id: Subpicture) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_subpicture(self, subpicture_id)
    }
    fn xvmc_list_subpicture_types(&self, port_id: xv::Port, surface_id: Surface) -> Result<Cookie<'_, Self, ListSubpictureTypesReply>, ConnectionError>
    {
        list_subpicture_types(self, port_id, surface_id)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
