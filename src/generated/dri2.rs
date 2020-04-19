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

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "DRI2";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Attachment {
    BufferFrontLeft = 0,
    BufferBackLeft = 1,
    BufferFrontRight = 2,
    BufferBackRight = 3,
    BufferDepth = 4,
    BufferStencil = 5,
    BufferAccum = 6,
    BufferFakeFrontLeft = 7,
    BufferFakeFrontRight = 8,
    BufferDepthStencil = 9,
    BufferHiz = 10,
}
impl From<Attachment> for u8 {
    fn from(input: Attachment) -> Self {
        match input {
            Attachment::BufferFrontLeft => 0,
            Attachment::BufferBackLeft => 1,
            Attachment::BufferFrontRight => 2,
            Attachment::BufferBackRight => 3,
            Attachment::BufferDepth => 4,
            Attachment::BufferStencil => 5,
            Attachment::BufferAccum => 6,
            Attachment::BufferFakeFrontLeft => 7,
            Attachment::BufferFakeFrontRight => 8,
            Attachment::BufferDepthStencil => 9,
            Attachment::BufferHiz => 10,
        }
    }
}
impl From<Attachment> for Option<u8> {
    fn from(input: Attachment) -> Self {
        Some(u8::from(input))
    }
}
impl From<Attachment> for u16 {
    fn from(input: Attachment) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Attachment> for Option<u16> {
    fn from(input: Attachment) -> Self {
        Some(u16::from(input))
    }
}
impl From<Attachment> for u32 {
    fn from(input: Attachment) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Attachment> for Option<u32> {
    fn from(input: Attachment) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Attachment {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Attachment::BufferFrontLeft),
            1 => Ok(Attachment::BufferBackLeft),
            2 => Ok(Attachment::BufferFrontRight),
            3 => Ok(Attachment::BufferBackRight),
            4 => Ok(Attachment::BufferDepth),
            5 => Ok(Attachment::BufferStencil),
            6 => Ok(Attachment::BufferAccum),
            7 => Ok(Attachment::BufferFakeFrontLeft),
            8 => Ok(Attachment::BufferFakeFrontRight),
            9 => Ok(Attachment::BufferDepthStencil),
            10 => Ok(Attachment::BufferHiz),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Attachment {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Attachment {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum DriverType {
    DRI = 0,
    VDPAU = 1,
}
impl From<DriverType> for u8 {
    fn from(input: DriverType) -> Self {
        match input {
            DriverType::DRI => 0,
            DriverType::VDPAU => 1,
        }
    }
}
impl From<DriverType> for Option<u8> {
    fn from(input: DriverType) -> Self {
        Some(u8::from(input))
    }
}
impl From<DriverType> for u16 {
    fn from(input: DriverType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DriverType> for Option<u16> {
    fn from(input: DriverType) -> Self {
        Some(u16::from(input))
    }
}
impl From<DriverType> for u32 {
    fn from(input: DriverType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DriverType> for Option<u32> {
    fn from(input: DriverType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DriverType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DriverType::DRI),
            1 => Ok(DriverType::VDPAU),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for DriverType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DriverType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventType {
    ExchangeComplete = 1,
    BlitComplete = 2,
    FlipComplete = 3,
}
impl From<EventType> for u8 {
    fn from(input: EventType) -> Self {
        match input {
            EventType::ExchangeComplete => 1,
            EventType::BlitComplete => 2,
            EventType::FlipComplete => 3,
        }
    }
}
impl From<EventType> for Option<u8> {
    fn from(input: EventType) -> Self {
        Some(u8::from(input))
    }
}
impl From<EventType> for u16 {
    fn from(input: EventType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EventType> for Option<u16> {
    fn from(input: EventType) -> Self {
        Some(u16::from(input))
    }
}
impl From<EventType> for u32 {
    fn from(input: EventType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EventType> for Option<u32> {
    fn from(input: EventType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for EventType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EventType::ExchangeComplete),
            2 => Ok(EventType::BlitComplete),
            3 => Ok(EventType::FlipComplete),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for EventType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for EventType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DRI2Buffer {
    pub attachment: Attachment,
    pub name: u32,
    pub pitch: u32,
    pub cpp: u32,
    pub flags: u32,
}
impl TryParse for DRI2Buffer {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (attachment, remaining) = u32::try_parse(remaining)?;
        let (name, remaining) = u32::try_parse(remaining)?;
        let (pitch, remaining) = u32::try_parse(remaining)?;
        let (cpp, remaining) = u32::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let attachment = attachment.try_into()?;
        let result = DRI2Buffer { attachment, name, pitch, cpp, flags };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DRI2Buffer {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DRI2Buffer {
    type Bytes = [u8; 20];
    fn serialize(&self) -> Self::Bytes {
        let attachment_bytes = u32::from(self.attachment).serialize();
        let name_bytes = self.name.serialize();
        let pitch_bytes = self.pitch.serialize();
        let cpp_bytes = self.cpp.serialize();
        let flags_bytes = self.flags.serialize();
        [
            attachment_bytes[0],
            attachment_bytes[1],
            attachment_bytes[2],
            attachment_bytes[3],
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
            pitch_bytes[0],
            pitch_bytes[1],
            pitch_bytes[2],
            pitch_bytes[3],
            cpp_bytes[0],
            cpp_bytes[1],
            cpp_bytes[2],
            cpp_bytes[3],
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(20);
        u32::from(self.attachment).serialize_into(bytes);
        self.name.serialize_into(bytes);
        self.pitch.serialize_into(bytes);
        self.cpp.serialize_into(bytes);
        self.flags.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttachFormat {
    pub attachment: Attachment,
    pub format: u32,
}
impl TryParse for AttachFormat {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (attachment, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u32::try_parse(remaining)?;
        let attachment = attachment.try_into()?;
        let result = AttachFormat { attachment, format };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AttachFormat {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for AttachFormat {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let attachment_bytes = u32::from(self.attachment).serialize();
        let format_bytes = self.format.serialize();
        [
            attachment_bytes[0],
            attachment_bytes[1],
            attachment_bytes[2],
            attachment_bytes[3],
            format_bytes[0],
            format_bytes[1],
            format_bytes[2],
            format_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u32::from(self.attachment).serialize_into(bytes);
        self.format.serialize_into(bytes);
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let major_version_bytes = major_version.serialize();
    let minor_version_bytes = minor_version.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        0,
        0,
        major_version_bytes[0],
        major_version_bytes[1],
        major_version_bytes[2],
        major_version_bytes[3],
        minor_version_bytes[0],
        minor_version_bytes[1],
        minor_version_bytes[2],
        minor_version_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
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
    pub major_version: u32,
    pub minor_version: u32,
}
impl TryParse for QueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u32::try_parse(remaining)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, major_version, minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the Connect request
pub const CONNECT_REQUEST: u8 = 1;
pub fn connect<Conn, A>(conn: &Conn, window: xproto::Window, driver_type: A) -> Result<Cookie<'_, Conn, ConnectReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let driver_type = driver_type.into();
    let driver_type_bytes = driver_type.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CONNECT_REQUEST,
        0,
        0,
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        driver_type_bytes[0],
        driver_type_bytes[1],
        driver_type_bytes[2],
        driver_type_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub driver_name: Vec<u8>,
    pub alignment_pad: Vec<u8>,
    pub device_name: Vec<u8>,
}
impl TryParse for ConnectReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (driver_name_length, remaining) = u32::try_parse(remaining)?;
        let (device_name_length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (driver_name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, driver_name_length as usize)?;
        let (alignment_pad, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (((driver_name_length as usize) + (3)) & (!(3))) - (driver_name_length as usize))?;
        let (device_name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, device_name_length as usize)?;
        let result = ConnectReply { response_type, sequence, length, driver_name, alignment_pad, device_name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ConnectReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the Authenticate request
pub const AUTHENTICATE_REQUEST: u8 = 2;
pub fn authenticate<Conn>(conn: &Conn, window: xproto::Window, magic: u32) -> Result<Cookie<'_, Conn, AuthenticateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let magic_bytes = magic.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        AUTHENTICATE_REQUEST,
        0,
        0,
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        magic_bytes[0],
        magic_bytes[1],
        magic_bytes[2],
        magic_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticateReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: u32,
}
impl TryParse for AuthenticateReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (authenticated, remaining) = u32::try_parse(remaining)?;
        let result = AuthenticateReply { response_type, sequence, length, authenticated };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AuthenticateReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreateDrawable request
pub const CREATE_DRAWABLE_REQUEST: u8 = 3;
pub fn create_drawable<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_DRAWABLE_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the DestroyDrawable request
pub const DESTROY_DRAWABLE_REQUEST: u8 = 4;
pub fn destroy_drawable<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        DESTROY_DRAWABLE_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the GetBuffers request
pub const GET_BUFFERS_REQUEST: u8 = 5;
pub fn get_buffers<'c, Conn>(conn: &'c Conn, drawable: xproto::Drawable, count: u32, attachments: &[u32]) -> Result<Cookie<'c, Conn, GetBuffersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let count_bytes = count.serialize();
    let attachments_bytes = attachments.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_BUFFERS_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        count_bytes[0],
        count_bytes[1],
        count_bytes[2],
        count_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    let length_so_far = length_so_far + attachments_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&attachments_bytes), IoSlice::new(&padding0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBuffersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub buffers: Vec<DRI2Buffer>,
}
impl TryParse for GetBuffersReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u32::try_parse(remaining)?;
        let (height, remaining) = u32::try_parse(remaining)?;
        let (count, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (buffers, remaining) = crate::x11_utils::parse_list::<DRI2Buffer>(remaining, count as usize)?;
        let result = GetBuffersReply { response_type, sequence, length, width, height, buffers };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetBuffersReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CopyRegion request
pub const COPY_REGION_REQUEST: u8 = 6;
pub fn copy_region<Conn>(conn: &Conn, drawable: xproto::Drawable, region: u32, dest: u32, src: u32) -> Result<Cookie<'_, Conn, CopyRegionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let region_bytes = region.serialize();
    let dest_bytes = dest.serialize();
    let src_bytes = src.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        COPY_REGION_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
        dest_bytes[0],
        dest_bytes[1],
        dest_bytes[2],
        dest_bytes[3],
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyRegionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for CopyRegionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let result = CopyRegionReply { response_type, sequence, length };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CopyRegionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetBuffersWithFormat request
pub const GET_BUFFERS_WITH_FORMAT_REQUEST: u8 = 7;
pub fn get_buffers_with_format<'c, Conn>(conn: &'c Conn, drawable: xproto::Drawable, count: u32, attachments: &[AttachFormat]) -> Result<Cookie<'c, Conn, GetBuffersWithFormatReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let count_bytes = count.serialize();
    let attachments_bytes = attachments.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_BUFFERS_WITH_FORMAT_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        count_bytes[0],
        count_bytes[1],
        count_bytes[2],
        count_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    let length_so_far = length_so_far + attachments_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&attachments_bytes), IoSlice::new(&padding0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBuffersWithFormatReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub buffers: Vec<DRI2Buffer>,
}
impl TryParse for GetBuffersWithFormatReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u32::try_parse(remaining)?;
        let (height, remaining) = u32::try_parse(remaining)?;
        let (count, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (buffers, remaining) = crate::x11_utils::parse_list::<DRI2Buffer>(remaining, count as usize)?;
        let result = GetBuffersWithFormatReply { response_type, sequence, length, width, height, buffers };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetBuffersWithFormatReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SwapBuffers request
pub const SWAP_BUFFERS_REQUEST: u8 = 8;
pub fn swap_buffers<Conn>(conn: &Conn, drawable: xproto::Drawable, target_msc_hi: u32, target_msc_lo: u32, divisor_hi: u32, divisor_lo: u32, remainder_hi: u32, remainder_lo: u32) -> Result<Cookie<'_, Conn, SwapBuffersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let target_msc_hi_bytes = target_msc_hi.serialize();
    let target_msc_lo_bytes = target_msc_lo.serialize();
    let divisor_hi_bytes = divisor_hi.serialize();
    let divisor_lo_bytes = divisor_lo.serialize();
    let remainder_hi_bytes = remainder_hi.serialize();
    let remainder_lo_bytes = remainder_lo.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SWAP_BUFFERS_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        target_msc_hi_bytes[0],
        target_msc_hi_bytes[1],
        target_msc_hi_bytes[2],
        target_msc_hi_bytes[3],
        target_msc_lo_bytes[0],
        target_msc_lo_bytes[1],
        target_msc_lo_bytes[2],
        target_msc_lo_bytes[3],
        divisor_hi_bytes[0],
        divisor_hi_bytes[1],
        divisor_hi_bytes[2],
        divisor_hi_bytes[3],
        divisor_lo_bytes[0],
        divisor_lo_bytes[1],
        divisor_lo_bytes[2],
        divisor_lo_bytes[3],
        remainder_hi_bytes[0],
        remainder_hi_bytes[1],
        remainder_hi_bytes[2],
        remainder_hi_bytes[3],
        remainder_lo_bytes[0],
        remainder_lo_bytes[1],
        remainder_lo_bytes[2],
        remainder_lo_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwapBuffersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub swap_hi: u32,
    pub swap_lo: u32,
}
impl TryParse for SwapBuffersReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (swap_hi, remaining) = u32::try_parse(remaining)?;
        let (swap_lo, remaining) = u32::try_parse(remaining)?;
        let result = SwapBuffersReply { response_type, sequence, length, swap_hi, swap_lo };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SwapBuffersReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMSC request
pub const GET_MSC_REQUEST: u8 = 9;
pub fn get_msc<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<Cookie<'_, Conn, GetMSCReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_MSC_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMSCReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}
impl TryParse for GetMSCReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ust_hi, remaining) = u32::try_parse(remaining)?;
        let (ust_lo, remaining) = u32::try_parse(remaining)?;
        let (msc_hi, remaining) = u32::try_parse(remaining)?;
        let (msc_lo, remaining) = u32::try_parse(remaining)?;
        let (sbc_hi, remaining) = u32::try_parse(remaining)?;
        let (sbc_lo, remaining) = u32::try_parse(remaining)?;
        let result = GetMSCReply { response_type, sequence, length, ust_hi, ust_lo, msc_hi, msc_lo, sbc_hi, sbc_lo };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMSCReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the WaitMSC request
pub const WAIT_MSC_REQUEST: u8 = 10;
pub fn wait_msc<Conn>(conn: &Conn, drawable: xproto::Drawable, target_msc_hi: u32, target_msc_lo: u32, divisor_hi: u32, divisor_lo: u32, remainder_hi: u32, remainder_lo: u32) -> Result<Cookie<'_, Conn, WaitMSCReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let target_msc_hi_bytes = target_msc_hi.serialize();
    let target_msc_lo_bytes = target_msc_lo.serialize();
    let divisor_hi_bytes = divisor_hi.serialize();
    let divisor_lo_bytes = divisor_lo.serialize();
    let remainder_hi_bytes = remainder_hi.serialize();
    let remainder_lo_bytes = remainder_lo.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        WAIT_MSC_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        target_msc_hi_bytes[0],
        target_msc_hi_bytes[1],
        target_msc_hi_bytes[2],
        target_msc_hi_bytes[3],
        target_msc_lo_bytes[0],
        target_msc_lo_bytes[1],
        target_msc_lo_bytes[2],
        target_msc_lo_bytes[3],
        divisor_hi_bytes[0],
        divisor_hi_bytes[1],
        divisor_hi_bytes[2],
        divisor_hi_bytes[3],
        divisor_lo_bytes[0],
        divisor_lo_bytes[1],
        divisor_lo_bytes[2],
        divisor_lo_bytes[3],
        remainder_hi_bytes[0],
        remainder_hi_bytes[1],
        remainder_hi_bytes[2],
        remainder_hi_bytes[3],
        remainder_lo_bytes[0],
        remainder_lo_bytes[1],
        remainder_lo_bytes[2],
        remainder_lo_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitMSCReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}
impl TryParse for WaitMSCReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ust_hi, remaining) = u32::try_parse(remaining)?;
        let (ust_lo, remaining) = u32::try_parse(remaining)?;
        let (msc_hi, remaining) = u32::try_parse(remaining)?;
        let (msc_lo, remaining) = u32::try_parse(remaining)?;
        let (sbc_hi, remaining) = u32::try_parse(remaining)?;
        let (sbc_lo, remaining) = u32::try_parse(remaining)?;
        let result = WaitMSCReply { response_type, sequence, length, ust_hi, ust_lo, msc_hi, msc_lo, sbc_hi, sbc_lo };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for WaitMSCReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the WaitSBC request
pub const WAIT_SBC_REQUEST: u8 = 11;
pub fn wait_sbc<Conn>(conn: &Conn, drawable: xproto::Drawable, target_sbc_hi: u32, target_sbc_lo: u32) -> Result<Cookie<'_, Conn, WaitSBCReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let target_sbc_hi_bytes = target_sbc_hi.serialize();
    let target_sbc_lo_bytes = target_sbc_lo.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        WAIT_SBC_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        target_sbc_hi_bytes[0],
        target_sbc_hi_bytes[1],
        target_sbc_hi_bytes[2],
        target_sbc_hi_bytes[3],
        target_sbc_lo_bytes[0],
        target_sbc_lo_bytes[1],
        target_sbc_lo_bytes[2],
        target_sbc_lo_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitSBCReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}
impl TryParse for WaitSBCReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ust_hi, remaining) = u32::try_parse(remaining)?;
        let (ust_lo, remaining) = u32::try_parse(remaining)?;
        let (msc_hi, remaining) = u32::try_parse(remaining)?;
        let (msc_lo, remaining) = u32::try_parse(remaining)?;
        let (sbc_hi, remaining) = u32::try_parse(remaining)?;
        let (sbc_lo, remaining) = u32::try_parse(remaining)?;
        let result = WaitSBCReply { response_type, sequence, length, ust_hi, ust_lo, msc_hi, msc_lo, sbc_hi, sbc_lo };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for WaitSBCReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SwapInterval request
pub const SWAP_INTERVAL_REQUEST: u8 = 12;
pub fn swap_interval<Conn>(conn: &Conn, drawable: xproto::Drawable, interval: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let interval_bytes = interval.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SWAP_INTERVAL_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        interval_bytes[0],
        interval_bytes[1],
        interval_bytes[2],
        interval_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the GetParam request
pub const GET_PARAM_REQUEST: u8 = 13;
pub fn get_param<Conn>(conn: &Conn, drawable: xproto::Drawable, param: u32) -> Result<Cookie<'_, Conn, GetParamReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let param_bytes = param.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_PARAM_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        param_bytes[0],
        param_bytes[1],
        param_bytes[2],
        param_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetParamReply {
    pub response_type: u8,
    pub is_param_recognized: bool,
    pub sequence: u16,
    pub length: u32,
    pub value_hi: u32,
    pub value_lo: u32,
}
impl TryParse for GetParamReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (is_param_recognized, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (value_hi, remaining) = u32::try_parse(remaining)?;
        let (value_lo, remaining) = u32::try_parse(remaining)?;
        let result = GetParamReply { response_type, is_param_recognized, sequence, length, value_hi, value_lo };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetParamReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the BufferSwapComplete event
pub const BUFFER_SWAP_COMPLETE_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferSwapCompleteEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event_type: EventType,
    pub drawable: xproto::Drawable,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc: u32,
}
impl BufferSwapCompleteEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (ust_hi, remaining) = u32::try_parse(remaining)?;
        let (ust_lo, remaining) = u32::try_parse(remaining)?;
        let (msc_hi, remaining) = u32::try_parse(remaining)?;
        let (msc_lo, remaining) = u32::try_parse(remaining)?;
        let (sbc, remaining) = u32::try_parse(remaining)?;
        let event_type = event_type.try_into()?;
        let result = BufferSwapCompleteEvent { response_type, sequence, event_type, drawable, ust_hi, ust_lo, msc_hi, msc_lo, sbc };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BufferSwapCompleteEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for BufferSwapCompleteEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for BufferSwapCompleteEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&BufferSwapCompleteEvent> for [u8; 32] {
    fn from(input: &BufferSwapCompleteEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let event_type_bytes = u16::from(input.event_type).serialize();
        let drawable_bytes = input.drawable.serialize();
        let ust_hi_bytes = input.ust_hi.serialize();
        let ust_lo_bytes = input.ust_lo.serialize();
        let msc_hi_bytes = input.msc_hi.serialize();
        let msc_lo_bytes = input.msc_lo.serialize();
        let sbc_bytes = input.sbc.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            event_type_bytes[0],
            event_type_bytes[1],
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            ust_hi_bytes[0],
            ust_hi_bytes[1],
            ust_hi_bytes[2],
            ust_hi_bytes[3],
            ust_lo_bytes[0],
            ust_lo_bytes[1],
            ust_lo_bytes[2],
            ust_lo_bytes[3],
            msc_hi_bytes[0],
            msc_hi_bytes[1],
            msc_hi_bytes[2],
            msc_hi_bytes[3],
            msc_lo_bytes[0],
            msc_lo_bytes[1],
            msc_lo_bytes[2],
            msc_lo_bytes[3],
            sbc_bytes[0],
            sbc_bytes[1],
            sbc_bytes[2],
            sbc_bytes[3],
        ]
    }
}
impl From<BufferSwapCompleteEvent> for [u8; 32] {
    fn from(input: BufferSwapCompleteEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the InvalidateBuffers event
pub const INVALIDATE_BUFFERS_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidateBuffersEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub drawable: xproto::Drawable,
}
impl InvalidateBuffersEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let result = InvalidateBuffersEvent { response_type, sequence, drawable };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InvalidateBuffersEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for InvalidateBuffersEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for InvalidateBuffersEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&InvalidateBuffersEvent> for [u8; 32] {
    fn from(input: &InvalidateBuffersEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let drawable_bytes = input.drawable.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            // trailing padding
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    }
}
impl From<InvalidateBuffersEvent> for [u8; 32] {
    fn from(input: InvalidateBuffersEvent) -> Self {
        Self::from(&input)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn dri2_query_version(&self, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }

    fn dri2_connect<A>(&self, window: xproto::Window, driver_type: A) -> Result<Cookie<'_, Self, ConnectReply>, ConnectionError>
    where
        A: Into<u32>,
    {
        connect(self, window, driver_type)
    }

    fn dri2_authenticate(&self, window: xproto::Window, magic: u32) -> Result<Cookie<'_, Self, AuthenticateReply>, ConnectionError>
    {
        authenticate(self, window, magic)
    }

    fn dri2_create_drawable(&self, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_drawable(self, drawable)
    }

    fn dri2_destroy_drawable(&self, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_drawable(self, drawable)
    }

    fn dri2_get_buffers<'c>(&'c self, drawable: xproto::Drawable, count: u32, attachments: &[u32]) -> Result<Cookie<'c, Self, GetBuffersReply>, ConnectionError>
    {
        get_buffers(self, drawable, count, attachments)
    }

    fn dri2_copy_region(&self, drawable: xproto::Drawable, region: u32, dest: u32, src: u32) -> Result<Cookie<'_, Self, CopyRegionReply>, ConnectionError>
    {
        copy_region(self, drawable, region, dest, src)
    }

    fn dri2_get_buffers_with_format<'c>(&'c self, drawable: xproto::Drawable, count: u32, attachments: &[AttachFormat]) -> Result<Cookie<'c, Self, GetBuffersWithFormatReply>, ConnectionError>
    {
        get_buffers_with_format(self, drawable, count, attachments)
    }

    fn dri2_swap_buffers(&self, drawable: xproto::Drawable, target_msc_hi: u32, target_msc_lo: u32, divisor_hi: u32, divisor_lo: u32, remainder_hi: u32, remainder_lo: u32) -> Result<Cookie<'_, Self, SwapBuffersReply>, ConnectionError>
    {
        swap_buffers(self, drawable, target_msc_hi, target_msc_lo, divisor_hi, divisor_lo, remainder_hi, remainder_lo)
    }

    fn dri2_get_msc(&self, drawable: xproto::Drawable) -> Result<Cookie<'_, Self, GetMSCReply>, ConnectionError>
    {
        get_msc(self, drawable)
    }

    fn dri2_wait_msc(&self, drawable: xproto::Drawable, target_msc_hi: u32, target_msc_lo: u32, divisor_hi: u32, divisor_lo: u32, remainder_hi: u32, remainder_lo: u32) -> Result<Cookie<'_, Self, WaitMSCReply>, ConnectionError>
    {
        wait_msc(self, drawable, target_msc_hi, target_msc_lo, divisor_hi, divisor_lo, remainder_hi, remainder_lo)
    }

    fn dri2_wait_sbc(&self, drawable: xproto::Drawable, target_sbc_hi: u32, target_sbc_lo: u32) -> Result<Cookie<'_, Self, WaitSBCReply>, ConnectionError>
    {
        wait_sbc(self, drawable, target_sbc_hi, target_sbc_lo)
    }

    fn dri2_swap_interval(&self, drawable: xproto::Drawable, interval: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        swap_interval(self, drawable, interval)
    }

    fn dri2_get_param(&self, drawable: xproto::Drawable, param: u32) -> Result<Cookie<'_, Self, GetParamReply>, ConnectionError>
    {
        get_param(self, drawable, param)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
