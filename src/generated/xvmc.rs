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
#[allow(unused_imports)]
use super::xproto;
#[allow(unused_imports)]
use super::shm;
#[allow(unused_imports)]
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
impl TryFrom<&[u8]> for SurfaceInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SurfaceInfo {
    type Bytes = [u8; 24];
    fn serialize(&self) -> Self::Bytes {
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
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let mut request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        0,
        0,
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u32,
    pub minor: u32,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major, remaining) = u32::try_parse(remaining)?;
        let (minor, remaining) = u32::try_parse(remaining)?;
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

/// Opcode for the ListSurfaceTypes request
pub const LIST_SURFACE_TYPES_REQUEST: u8 = 1;
pub fn list_surface_types<Conn>(conn: &Conn, port_id: xv::Port) -> Result<Cookie<'_, Conn, ListSurfaceTypesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let port_id_bytes = port_id.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        LIST_SURFACE_TYPES_REQUEST,
        0,
        0,
        port_id_bytes[0],
        port_id_bytes[1],
        port_id_bytes[2],
        port_id_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSurfaceTypesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub surfaces: Vec<SurfaceInfo>,
}
impl ListSurfaceTypesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (surfaces, remaining) = crate::x11_utils::parse_list::<SurfaceInfo>(remaining, num as usize)?;
        let result = ListSurfaceTypesReply { response_type, sequence, length, surfaces };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListSurfaceTypesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 2;
pub fn create_context<Conn>(conn: &Conn, context_id: Context, port_id: xv::Port, surface_id: Surface, width: u16, height: u16, flags: u32) -> Result<Cookie<'_, Conn, CreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_id_bytes = context_id.serialize();
    let port_id_bytes = port_id.serialize();
    let surface_id_bytes = surface_id.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let flags_bytes = flags.serialize();
    let mut request0 = [
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width_actual: u16,
    pub height_actual: u16,
    pub flags_return: u32,
    pub priv_data: Vec<u32>,
}
impl CreateContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width_actual, remaining) = u16::try_parse(remaining)?;
        let (height_actual, remaining) = u16::try_parse(remaining)?;
        let (flags_return, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (priv_data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length as usize)?;
        let result = CreateContextReply { response_type, sequence, width_actual, height_actual, flags_return, priv_data };
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
pub const DESTROY_CONTEXT_REQUEST: u8 = 3;
pub fn destroy_context<Conn>(conn: &Conn, context_id: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_id_bytes = context_id.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        DESTROY_CONTEXT_REQUEST,
        0,
        0,
        context_id_bytes[0],
        context_id_bytes[1],
        context_id_bytes[2],
        context_id_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the CreateSurface request
pub const CREATE_SURFACE_REQUEST: u8 = 4;
pub fn create_surface<Conn>(conn: &Conn, surface_id: Surface, context_id: Context) -> Result<Cookie<'_, Conn, CreateSurfaceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let surface_id_bytes = surface_id.serialize();
    let context_id_bytes = context_id.serialize();
    let mut request0 = [
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSurfaceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub priv_data: Vec<u32>,
}
impl CreateSurfaceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (priv_data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length as usize)?;
        let result = CreateSurfaceReply { response_type, sequence, priv_data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CreateSurfaceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DestroySurface request
pub const DESTROY_SURFACE_REQUEST: u8 = 5;
pub fn destroy_surface<Conn>(conn: &Conn, surface_id: Surface) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let surface_id_bytes = surface_id.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        DESTROY_SURFACE_REQUEST,
        0,
        0,
        surface_id_bytes[0],
        surface_id_bytes[1],
        surface_id_bytes[2],
        surface_id_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the CreateSubpicture request
pub const CREATE_SUBPICTURE_REQUEST: u8 = 6;
pub fn create_subpicture<Conn>(conn: &Conn, subpicture_id: Subpicture, context: Context, xvimage_id: u32, width: u16, height: u16) -> Result<Cookie<'_, Conn, CreateSubpictureReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let subpicture_id_bytes = subpicture_id.serialize();
    let context_bytes = context.serialize();
    let xvimage_id_bytes = xvimage_id.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let mut request0 = [
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSubpictureReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width_actual: u16,
    pub height_actual: u16,
    pub num_palette_entries: u16,
    pub entry_bytes: u16,
    pub component_order: [u8; 4],
    pub priv_data: Vec<u32>,
}
impl CreateSubpictureReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width_actual, remaining) = u16::try_parse(remaining)?;
        let (height_actual, remaining) = u16::try_parse(remaining)?;
        let (num_palette_entries, remaining) = u16::try_parse(remaining)?;
        let (entry_bytes, remaining) = u16::try_parse(remaining)?;
        let (component_order_0, remaining) = u8::try_parse(remaining)?;
        let (component_order_1, remaining) = u8::try_parse(remaining)?;
        let (component_order_2, remaining) = u8::try_parse(remaining)?;
        let (component_order_3, remaining) = u8::try_parse(remaining)?;
        let component_order = [
            component_order_0,
            component_order_1,
            component_order_2,
            component_order_3,
        ];
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (priv_data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length as usize)?;
        let result = CreateSubpictureReply { response_type, sequence, width_actual, height_actual, num_palette_entries, entry_bytes, component_order, priv_data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CreateSubpictureReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DestroySubpicture request
pub const DESTROY_SUBPICTURE_REQUEST: u8 = 7;
pub fn destroy_subpicture<Conn>(conn: &Conn, subpicture_id: Subpicture) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let subpicture_id_bytes = subpicture_id.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        DESTROY_SUBPICTURE_REQUEST,
        0,
        0,
        subpicture_id_bytes[0],
        subpicture_id_bytes[1],
        subpicture_id_bytes[2],
        subpicture_id_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the ListSubpictureTypes request
pub const LIST_SUBPICTURE_TYPES_REQUEST: u8 = 8;
pub fn list_subpicture_types<Conn>(conn: &Conn, port_id: xv::Port, surface_id: Surface) -> Result<Cookie<'_, Conn, ListSubpictureTypesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let port_id_bytes = port_id.serialize();
    let surface_id_bytes = surface_id.serialize();
    let mut request0 = [
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSubpictureTypesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub types: Vec<xv::ImageFormatInfo>,
}
impl ListSubpictureTypesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (types, remaining) = crate::x11_utils::parse_list::<xv::ImageFormatInfo>(remaining, num as usize)?;
        let result = ListSubpictureTypesReply { response_type, sequence, length, types };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListSubpictureTypesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
