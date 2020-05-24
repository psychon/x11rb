// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Shape` X11 extension.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]

use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "SHAPE";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

pub type Op = u8;

pub type Kind = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SO {
    Set = 0,
    Union = 1,
    Intersect = 2,
    Subtract = 3,
    Invert = 4,
}
impl From<SO> for u8 {
    fn from(input: SO) -> Self {
        match input {
            SO::Set => 0,
            SO::Union => 1,
            SO::Intersect => 2,
            SO::Subtract => 3,
            SO::Invert => 4,
        }
    }
}
impl From<SO> for Option<u8> {
    fn from(input: SO) -> Self {
        Some(u8::from(input))
    }
}
impl From<SO> for u16 {
    fn from(input: SO) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SO> for Option<u16> {
    fn from(input: SO) -> Self {
        Some(u16::from(input))
    }
}
impl From<SO> for u32 {
    fn from(input: SO) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SO> for Option<u32> {
    fn from(input: SO) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SO {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SO::Set),
            1 => Ok(SO::Union),
            2 => Ok(SO::Intersect),
            3 => Ok(SO::Subtract),
            4 => Ok(SO::Invert),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SO {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SO {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SK {
    Bounding = 0,
    Clip = 1,
    Input = 2,
}
impl From<SK> for u8 {
    fn from(input: SK) -> Self {
        match input {
            SK::Bounding => 0,
            SK::Clip => 1,
            SK::Input => 2,
        }
    }
}
impl From<SK> for Option<u8> {
    fn from(input: SK) -> Self {
        Some(u8::from(input))
    }
}
impl From<SK> for u16 {
    fn from(input: SK) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SK> for Option<u16> {
    fn from(input: SK) -> Self {
        Some(u16::from(input))
    }
}
impl From<SK> for u32 {
    fn from(input: SK) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SK> for Option<u32> {
    fn from(input: SK) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SK {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SK::Bounding),
            1 => Ok(SK::Clip),
            2 => Ok(SK::Input),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SK {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SK {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the Notify event
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotifyEvent {
    pub response_type: u8,
    pub shape_kind: SK,
    pub sequence: u16,
    pub affected_window: xproto::Window,
    pub extents_x: i16,
    pub extents_y: i16,
    pub extents_width: u16,
    pub extents_height: u16,
    pub server_time: xproto::Timestamp,
    pub shaped: bool,
}
impl TryParse for NotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (shape_kind, remaining) = Kind::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (affected_window, remaining) = xproto::Window::try_parse(remaining)?;
        let (extents_x, remaining) = i16::try_parse(remaining)?;
        let (extents_y, remaining) = i16::try_parse(remaining)?;
        let (extents_width, remaining) = u16::try_parse(remaining)?;
        let (extents_height, remaining) = u16::try_parse(remaining)?;
        let (server_time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (shaped, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::ParseError)?;
        let shape_kind = shape_kind.try_into()?;
        let result = NotifyEvent { response_type, shape_kind, sequence, affected_window, extents_x, extents_y, extents_width, extents_height, server_time, shaped };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for NotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&NotifyEvent> for [u8; 32] {
    fn from(input: &NotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let shape_kind_bytes = Kind::from(input.shape_kind).serialize();
        let sequence_bytes = input.sequence.serialize();
        let affected_window_bytes = input.affected_window.serialize();
        let extents_x_bytes = input.extents_x.serialize();
        let extents_y_bytes = input.extents_y.serialize();
        let extents_width_bytes = input.extents_width.serialize();
        let extents_height_bytes = input.extents_height.serialize();
        let server_time_bytes = input.server_time.serialize();
        let shaped_bytes = input.shaped.serialize();
        [
            response_type_bytes[0],
            shape_kind_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            affected_window_bytes[0],
            affected_window_bytes[1],
            affected_window_bytes[2],
            affected_window_bytes[3],
            extents_x_bytes[0],
            extents_x_bytes[1],
            extents_y_bytes[0],
            extents_y_bytes[1],
            extents_width_bytes[0],
            extents_width_bytes[1],
            extents_height_bytes[0],
            extents_height_bytes[1],
            server_time_bytes[0],
            server_time_bytes[1],
            server_time_bytes[2],
            server_time_bytes[3],
            shaped_bytes[0],
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
impl From<NotifyEvent> for [u8; 32] {
    fn from(input: NotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest;
impl QueryVersionRequest {
    /// Opcode for the QueryVersion request
    pub const fn opcode() -> u8 { 0 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
    pub major_version: u16,
    pub minor_version: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RectanglesRequest<'input> {
    pub operation: SO,
    pub destination_kind: SK,
    pub ordering: xproto::ClipOrdering,
    pub destination_window: xproto::Window,
    pub x_offset: i16,
    pub y_offset: i16,
    pub rectangles: &'input [xproto::Rectangle],
}
impl<'input> RectanglesRequest<'input> {
    /// Opcode for the Rectangles request
    pub const fn opcode() -> u8 { 1 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let operation_bytes = Op::from(self.operation).serialize();
        let destination_kind_bytes = Kind::from(self.destination_kind).serialize();
        let ordering_bytes = u8::from(self.ordering).serialize();
        let destination_window_bytes = self.destination_window.serialize();
        let x_offset_bytes = self.x_offset.serialize();
        let y_offset_bytes = self.y_offset.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            operation_bytes[0],
            destination_kind_bytes[0],
            ordering_bytes[0],
            0,
            destination_window_bytes[0],
            destination_window_bytes[1],
            destination_window_bytes[2],
            destination_window_bytes[3],
            x_offset_bytes[0],
            x_offset_bytes[1],
            y_offset_bytes[0],
            y_offset_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        let rectangles_bytes = self.rectangles.serialize();
        let length_so_far = length_so_far + rectangles_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), rectangles_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn rectangles<'c, 'input, Conn>(conn: &'c Conn, operation: SO, destination_kind: SK, ordering: xproto::ClipOrdering, destination_window: xproto::Window, x_offset: i16, y_offset: i16, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RectanglesRequest {
        operation,
        destination_kind,
        ordering,
        destination_window,
        x_offset,
        y_offset,
        rectangles,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaskRequest {
    pub operation: SO,
    pub destination_kind: SK,
    pub destination_window: xproto::Window,
    pub x_offset: i16,
    pub y_offset: i16,
    pub source_bitmap: xproto::Pixmap,
}
impl MaskRequest {
    /// Opcode for the Mask request
    pub const fn opcode() -> u8 { 2 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let operation_bytes = Op::from(self.operation).serialize();
        let destination_kind_bytes = Kind::from(self.destination_kind).serialize();
        let destination_window_bytes = self.destination_window.serialize();
        let x_offset_bytes = self.x_offset.serialize();
        let y_offset_bytes = self.y_offset.serialize();
        let source_bitmap_bytes = self.source_bitmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            operation_bytes[0],
            destination_kind_bytes[0],
            0,
            0,
            destination_window_bytes[0],
            destination_window_bytes[1],
            destination_window_bytes[2],
            destination_window_bytes[3],
            x_offset_bytes[0],
            x_offset_bytes[1],
            y_offset_bytes[0],
            y_offset_bytes[1],
            source_bitmap_bytes[0],
            source_bitmap_bytes[1],
            source_bitmap_bytes[2],
            source_bitmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn mask<Conn, A>(conn: &Conn, operation: SO, destination_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16, source_bitmap: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Pixmap>,
{
    let source_bitmap: xproto::Pixmap = source_bitmap.into();
    let request0 = MaskRequest {
        operation,
        destination_kind,
        destination_window,
        x_offset,
        y_offset,
        source_bitmap,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombineRequest {
    pub operation: SO,
    pub destination_kind: SK,
    pub source_kind: SK,
    pub destination_window: xproto::Window,
    pub x_offset: i16,
    pub y_offset: i16,
    pub source_window: xproto::Window,
}
impl CombineRequest {
    /// Opcode for the Combine request
    pub const fn opcode() -> u8 { 3 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let operation_bytes = Op::from(self.operation).serialize();
        let destination_kind_bytes = Kind::from(self.destination_kind).serialize();
        let source_kind_bytes = Kind::from(self.source_kind).serialize();
        let destination_window_bytes = self.destination_window.serialize();
        let x_offset_bytes = self.x_offset.serialize();
        let y_offset_bytes = self.y_offset.serialize();
        let source_window_bytes = self.source_window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            operation_bytes[0],
            destination_kind_bytes[0],
            source_kind_bytes[0],
            0,
            destination_window_bytes[0],
            destination_window_bytes[1],
            destination_window_bytes[2],
            destination_window_bytes[3],
            x_offset_bytes[0],
            x_offset_bytes[1],
            y_offset_bytes[0],
            y_offset_bytes[1],
            source_window_bytes[0],
            source_window_bytes[1],
            source_window_bytes[2],
            source_window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn combine<Conn>(conn: &Conn, operation: SO, destination_kind: SK, source_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16, source_window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CombineRequest {
        operation,
        destination_kind,
        source_kind,
        destination_window,
        x_offset,
        y_offset,
        source_window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetRequest {
    pub destination_kind: SK,
    pub destination_window: xproto::Window,
    pub x_offset: i16,
    pub y_offset: i16,
}
impl OffsetRequest {
    /// Opcode for the Offset request
    pub const fn opcode() -> u8 { 4 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let destination_kind_bytes = Kind::from(self.destination_kind).serialize();
        let destination_window_bytes = self.destination_window.serialize();
        let x_offset_bytes = self.x_offset.serialize();
        let y_offset_bytes = self.y_offset.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            destination_kind_bytes[0],
            0,
            0,
            0,
            destination_window_bytes[0],
            destination_window_bytes[1],
            destination_window_bytes[2],
            destination_window_bytes[3],
            x_offset_bytes[0],
            x_offset_bytes[1],
            y_offset_bytes[0],
            y_offset_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn offset<Conn>(conn: &Conn, destination_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = OffsetRequest {
        destination_kind,
        destination_window,
        x_offset,
        y_offset,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtentsRequest {
    pub destination_window: xproto::Window,
}
impl QueryExtentsRequest {
    /// Opcode for the QueryExtents request
    pub const fn opcode() -> u8 { 5 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let destination_window_bytes = self.destination_window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            destination_window_bytes[0],
            destination_window_bytes[1],
            destination_window_bytes[2],
            destination_window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn query_extents<Conn>(conn: &Conn, destination_window: xproto::Window) -> Result<Cookie<'_, Conn, QueryExtentsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryExtentsRequest {
        destination_window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtentsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub bounding_shaped: bool,
    pub clip_shaped: bool,
    pub bounding_shape_extents_x: i16,
    pub bounding_shape_extents_y: i16,
    pub bounding_shape_extents_width: u16,
    pub bounding_shape_extents_height: u16,
    pub clip_shape_extents_x: i16,
    pub clip_shape_extents_y: i16,
    pub clip_shape_extents_width: u16,
    pub clip_shape_extents_height: u16,
}
impl TryParse for QueryExtentsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (bounding_shaped, remaining) = bool::try_parse(remaining)?;
        let (clip_shaped, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (bounding_shape_extents_x, remaining) = i16::try_parse(remaining)?;
        let (bounding_shape_extents_y, remaining) = i16::try_parse(remaining)?;
        let (bounding_shape_extents_width, remaining) = u16::try_parse(remaining)?;
        let (bounding_shape_extents_height, remaining) = u16::try_parse(remaining)?;
        let (clip_shape_extents_x, remaining) = i16::try_parse(remaining)?;
        let (clip_shape_extents_y, remaining) = i16::try_parse(remaining)?;
        let (clip_shape_extents_width, remaining) = u16::try_parse(remaining)?;
        let (clip_shape_extents_height, remaining) = u16::try_parse(remaining)?;
        let result = QueryExtentsReply { response_type, sequence, length, bounding_shaped, clip_shaped, bounding_shape_extents_x, bounding_shape_extents_y, bounding_shape_extents_width, bounding_shape_extents_height, clip_shape_extents_x, clip_shape_extents_y, clip_shape_extents_width, clip_shape_extents_height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryExtentsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectInputRequest {
    pub destination_window: xproto::Window,
    pub enable: bool,
}
impl SelectInputRequest {
    /// Opcode for the SelectInput request
    pub const fn opcode() -> u8 { 6 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let destination_window_bytes = self.destination_window.serialize();
        let enable_bytes = self.enable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            destination_window_bytes[0],
            destination_window_bytes[1],
            destination_window_bytes[2],
            destination_window_bytes[3],
            enable_bytes[0],
            0,
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
pub fn select_input<Conn>(conn: &Conn, destination_window: xproto::Window, enable: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectInputRequest {
        destination_window,
        enable,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputSelectedRequest {
    pub destination_window: xproto::Window,
}
impl InputSelectedRequest {
    /// Opcode for the InputSelected request
    pub const fn opcode() -> u8 { 7 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let destination_window_bytes = self.destination_window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            destination_window_bytes[0],
            destination_window_bytes[1],
            destination_window_bytes[2],
            destination_window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn input_selected<Conn>(conn: &Conn, destination_window: xproto::Window) -> Result<Cookie<'_, Conn, InputSelectedReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InputSelectedRequest {
        destination_window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputSelectedReply {
    pub response_type: u8,
    pub enabled: bool,
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for InputSelectedReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (enabled, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let result = InputSelectedReply { response_type, enabled, sequence, length };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputSelectedReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetRectanglesRequest {
    pub window: xproto::Window,
    pub source_kind: SK,
}
impl GetRectanglesRequest {
    /// Opcode for the GetRectangles request
    pub const fn opcode() -> u8 { 8 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let source_kind_bytes = Kind::from(self.source_kind).serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            source_kind_bytes[0],
            0,
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
pub fn get_rectangles<Conn>(conn: &Conn, window: xproto::Window, source_kind: SK) -> Result<Cookie<'_, Conn, GetRectanglesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetRectanglesRequest {
        window,
        source_kind,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetRectanglesReply {
    pub response_type: u8,
    pub ordering: xproto::ClipOrdering,
    pub sequence: u16,
    pub length: u32,
    pub rectangles: Vec<xproto::Rectangle>,
}
impl TryParse for GetRectanglesReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (ordering, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (rectangles_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (rectangles, remaining) = crate::x11_utils::parse_list::<xproto::Rectangle>(remaining, rectangles_len.try_into().or(Err(ParseError::ParseError))?)?;
        let ordering = ordering.try_into()?;
        let result = GetRectanglesReply { response_type, ordering, sequence, length, rectangles };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetRectanglesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetRectanglesReply {
    /// Get the value of the `rectangles_len` field.
    ///
    /// The `rectangles_len` field is used as the length field of the `rectangles` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn rectangles_len(&self) -> u32 {
        self.rectangles.len()
            .try_into().unwrap()
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn shape_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }
    fn shape_rectangles<'c, 'input>(&'c self, operation: SO, destination_kind: SK, ordering: xproto::ClipOrdering, destination_window: xproto::Window, x_offset: i16, y_offset: i16, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        self::rectangles(self, operation, destination_kind, ordering, destination_window, x_offset, y_offset, rectangles)
    }
    fn shape_mask<A>(&self, operation: SO, destination_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16, source_bitmap: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Pixmap>,
    {
        mask(self, operation, destination_kind, destination_window, x_offset, y_offset, source_bitmap)
    }
    fn shape_combine(&self, operation: SO, destination_kind: SK, source_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16, source_window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        combine(self, operation, destination_kind, source_kind, destination_window, x_offset, y_offset, source_window)
    }
    fn shape_offset(&self, destination_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        offset(self, destination_kind, destination_window, x_offset, y_offset)
    }
    fn shape_query_extents(&self, destination_window: xproto::Window) -> Result<Cookie<'_, Self, QueryExtentsReply>, ConnectionError>
    {
        query_extents(self, destination_window)
    }
    fn shape_select_input(&self, destination_window: xproto::Window, enable: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_input(self, destination_window, enable)
    }
    fn shape_input_selected(&self, destination_window: xproto::Window) -> Result<Cookie<'_, Self, InputSelectedReply>, ConnectionError>
    {
        input_selected(self, destination_window)
    }
    fn shape_get_rectangles(&self, window: xproto::Window, source_kind: SK) -> Result<Cookie<'_, Self, GetRectanglesReply>, ConnectionError>
    {
        get_rectangles(self, window, source_kind)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
