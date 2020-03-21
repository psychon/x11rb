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
pub const X11_EXTENSION_NAME: &str = "SHAPE";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

pub type OP = u8;

pub type KIND = u8;

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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
    pub affected_window: WINDOW,
    pub extents_x: i16,
    pub extents_y: i16,
    pub extents_width: u16,
    pub extents_height: u16,
    pub server_time: TIMESTAMP,
    pub shaped: bool,
}
impl NotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (shape_kind, remaining) = KIND::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (affected_window, remaining) = WINDOW::try_parse(remaining)?;
        let (extents_x, remaining) = i16::try_parse(remaining)?;
        let (extents_y, remaining) = i16::try_parse(remaining)?;
        let (extents_width, remaining) = u16::try_parse(remaining)?;
        let (extents_height, remaining) = u16::try_parse(remaining)?;
        let (server_time, remaining) = TIMESTAMP::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for NotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for NotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&NotifyEvent> for [u8; 32] {
    fn from(input: &NotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let shape_kind = Into::<KIND>::into(input.shape_kind).serialize();
        let sequence = input.sequence.serialize();
        let affected_window = input.affected_window.serialize();
        let extents_x = input.extents_x.serialize();
        let extents_y = input.extents_y.serialize();
        let extents_width = input.extents_width.serialize();
        let extents_height = input.extents_height.serialize();
        let server_time = input.server_time.serialize();
        let shaped = input.shaped.serialize();
        [
            response_type[0], shape_kind[0], sequence[0], sequence[1], affected_window[0], affected_window[1], affected_window[2], affected_window[3],
            extents_x[0], extents_x[1], extents_y[0], extents_y[1], extents_width[0], extents_width[1], extents_height[0], extents_height[1],
            server_time[0], server_time[1], server_time[2], server_time[3], shaped[0], 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<NotifyEvent> for [u8; 32] {
    fn from(input: NotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    pub major_version: u16,
    pub minor_version: u16,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

/// Opcode for the Rectangles request
pub const RECTANGLES_REQUEST: u8 = 1;
pub fn rectangles<'c, Conn, A, B, C>(conn: &'c Conn, operation: A, destination_kind: B, ordering: C, destination_window: WINDOW, x_offset: i16, y_offset: i16, rectangles: &[Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<OP>, B: Into<KIND>, C: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16 + 8 * rectangles.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let operation = operation.into();
    let operation_bytes = operation.serialize();
    let destination_kind = destination_kind.into();
    let destination_kind_bytes = destination_kind.serialize();
    let ordering = ordering.into();
    let ordering_bytes = ordering.serialize();
    let destination_window_bytes = destination_window.serialize();
    let x_offset_bytes = x_offset.serialize();
    let y_offset_bytes = y_offset.serialize();
    let rectangles_bytes = rectangles.serialize();
    let request0 = [
        extension_information.major_opcode,
        RECTANGLES_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&rectangles_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&rectangles_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the Mask request
pub const MASK_REQUEST: u8 = 2;
pub fn mask<Conn, A, B>(conn: &Conn, operation: A, destination_kind: B, destination_window: WINDOW, x_offset: i16, y_offset: i16, source_bitmap: PIXMAP) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<OP>, B: Into<KIND>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let operation = operation.into();
    let operation_bytes = operation.serialize();
    let destination_kind = destination_kind.into();
    let destination_kind_bytes = destination_kind.serialize();
    let destination_window_bytes = destination_window.serialize();
    let x_offset_bytes = x_offset.serialize();
    let y_offset_bytes = y_offset.serialize();
    let source_bitmap_bytes = source_bitmap.serialize();
    let request0 = [
        extension_information.major_opcode,
        MASK_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Combine request
pub const COMBINE_REQUEST: u8 = 3;
pub fn combine<Conn, A, B, C>(conn: &Conn, operation: A, destination_kind: B, source_kind: C, destination_window: WINDOW, x_offset: i16, y_offset: i16, source_window: WINDOW) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<OP>, B: Into<KIND>, C: Into<KIND>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let operation = operation.into();
    let operation_bytes = operation.serialize();
    let destination_kind = destination_kind.into();
    let destination_kind_bytes = destination_kind.serialize();
    let source_kind = source_kind.into();
    let source_kind_bytes = source_kind.serialize();
    let destination_window_bytes = destination_window.serialize();
    let x_offset_bytes = x_offset.serialize();
    let y_offset_bytes = y_offset.serialize();
    let source_window_bytes = source_window.serialize();
    let request0 = [
        extension_information.major_opcode,
        COMBINE_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Offset request
pub const OFFSET_REQUEST: u8 = 4;
pub fn offset<Conn, A>(conn: &Conn, destination_kind: A, destination_window: WINDOW, x_offset: i16, y_offset: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<KIND>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let destination_kind = destination_kind.into();
    let destination_kind_bytes = destination_kind.serialize();
    let destination_window_bytes = destination_window.serialize();
    let x_offset_bytes = x_offset.serialize();
    let y_offset_bytes = y_offset.serialize();
    let request0 = [
        extension_information.major_opcode,
        OFFSET_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the QueryExtents request
pub const QUERY_EXTENTS_REQUEST: u8 = 5;
pub fn query_extents<Conn>(conn: &Conn, destination_window: WINDOW) -> Result<Cookie<'_, Conn, QueryExtentsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let destination_window_bytes = destination_window.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_EXTENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        destination_window_bytes[0],
        destination_window_bytes[1],
        destination_window_bytes[2],
        destination_window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl QueryExtentsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

/// Opcode for the SelectInput request
pub const SELECT_INPUT_REQUEST: u8 = 6;
pub fn select_input<Conn>(conn: &Conn, destination_window: WINDOW, enable: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let destination_window_bytes = destination_window.serialize();
    let enable_bytes = (enable as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_INPUT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        destination_window_bytes[0],
        destination_window_bytes[1],
        destination_window_bytes[2],
        destination_window_bytes[3],
        enable_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the InputSelected request
pub const INPUT_SELECTED_REQUEST: u8 = 7;
pub fn input_selected<Conn>(conn: &Conn, destination_window: WINDOW) -> Result<Cookie<'_, Conn, InputSelectedReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let destination_window_bytes = destination_window.serialize();
    let request0 = [
        extension_information.major_opcode,
        INPUT_SELECTED_REQUEST,
        length_bytes[0],
        length_bytes[1],
        destination_window_bytes[0],
        destination_window_bytes[1],
        destination_window_bytes[2],
        destination_window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputSelectedReply {
    pub response_type: u8,
    pub enabled: bool,
    pub sequence: u16,
    pub length: u32,
}
impl InputSelectedReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

/// Opcode for the GetRectangles request
pub const GET_RECTANGLES_REQUEST: u8 = 8;
pub fn get_rectangles<Conn, A>(conn: &Conn, window: WINDOW, source_kind: A) -> Result<Cookie<'_, Conn, GetRectanglesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<KIND>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let source_kind = source_kind.into();
    let source_kind_bytes = source_kind.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_RECTANGLES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        source_kind_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetRectanglesReply {
    pub response_type: u8,
    pub ordering: ClipOrdering,
    pub sequence: u16,
    pub length: u32,
    pub rectangles: Vec<Rectangle>,
}
impl GetRectanglesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (ordering, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (rectangles_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (rectangles, remaining) = crate::x11_utils::parse_list::<Rectangle>(remaining, rectangles_len as usize)?;
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

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn shape_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }

    fn shape_rectangles<'c, A, B, C>(&'c self, operation: A, destination_kind: B, ordering: C, destination_window: WINDOW, x_offset: i16, y_offset: i16, rectangles: &[Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<OP>, B: Into<KIND>, C: Into<u8>
    {
        self::rectangles(self, operation, destination_kind, ordering, destination_window, x_offset, y_offset, rectangles)
    }

    fn shape_mask<A, B>(&self, operation: A, destination_kind: B, destination_window: WINDOW, x_offset: i16, y_offset: i16, source_bitmap: PIXMAP) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<OP>, B: Into<KIND>
    {
        mask(self, operation, destination_kind, destination_window, x_offset, y_offset, source_bitmap)
    }

    fn shape_combine<A, B, C>(&self, operation: A, destination_kind: B, source_kind: C, destination_window: WINDOW, x_offset: i16, y_offset: i16, source_window: WINDOW) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<OP>, B: Into<KIND>, C: Into<KIND>
    {
        combine(self, operation, destination_kind, source_kind, destination_window, x_offset, y_offset, source_window)
    }

    fn shape_offset<A>(&self, destination_kind: A, destination_window: WINDOW, x_offset: i16, y_offset: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<KIND>
    {
        offset(self, destination_kind, destination_window, x_offset, y_offset)
    }

    fn shape_query_extents(&self, destination_window: WINDOW) -> Result<Cookie<'_, Self, QueryExtentsReply>, ConnectionError>
    {
        query_extents(self, destination_window)
    }

    fn shape_select_input(&self, destination_window: WINDOW, enable: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_input(self, destination_window, enable)
    }

    fn shape_input_selected(&self, destination_window: WINDOW) -> Result<Cookie<'_, Self, InputSelectedReply>, ConnectionError>
    {
        input_selected(self, destination_window)
    }

    fn shape_get_rectangles<A>(&self, window: WINDOW, source_kind: A) -> Result<Cookie<'_, Self, GetRectanglesReply>, ConnectionError>
    where A: Into<KIND>
    {
        get_rectangles(self, window, source_kind)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
