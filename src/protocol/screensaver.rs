// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `ScreenSaver` X11 extension.

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
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "MIT-SCREEN-SAVER";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
pub enum Kind {
    Blanked = 0,
    Internal = 1,
    External = 2,
}
impl From<Kind> for u8 {
    fn from(input: Kind) -> Self {
        match input {
            Kind::Blanked => 0,
            Kind::Internal => 1,
            Kind::External => 2,
        }
    }
}
impl From<Kind> for Option<u8> {
    fn from(input: Kind) -> Self {
        Some(u8::from(input))
    }
}
impl From<Kind> for u16 {
    fn from(input: Kind) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Kind> for Option<u16> {
    fn from(input: Kind) -> Self {
        Some(u16::from(input))
    }
}
impl From<Kind> for u32 {
    fn from(input: Kind) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Kind> for Option<u32> {
    fn from(input: Kind) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Kind {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Kind::Blanked),
            1 => Ok(Kind::Internal),
            2 => Ok(Kind::External),
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for Kind {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for Kind {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
pub enum Event {
    NotifyMask = 1 << 0,
    CycleMask = 1 << 1,
}
impl From<Event> for u8 {
    fn from(input: Event) -> Self {
        match input {
            Event::NotifyMask => 1 << 0,
            Event::CycleMask => 1 << 1,
        }
    }
}
impl From<Event> for Option<u8> {
    fn from(input: Event) -> Self {
        Some(u8::from(input))
    }
}
impl From<Event> for u16 {
    fn from(input: Event) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Event> for Option<u16> {
    fn from(input: Event) -> Self {
        Some(u16::from(input))
    }
}
impl From<Event> for u32 {
    fn from(input: Event) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Event> for Option<u32> {
    fn from(input: Event) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Event {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Event::NotifyMask),
            2 => Ok(Event::CycleMask),
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for Event {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for Event {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
bitmask_binop!(Event, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
pub enum State {
    Off = 0,
    On = 1,
    Cycle = 2,
    Disabled = 3,
}
impl From<State> for u8 {
    fn from(input: State) -> Self {
        match input {
            State::Off => 0,
            State::On => 1,
            State::Cycle => 2,
            State::Disabled => 3,
        }
    }
}
impl From<State> for Option<u8> {
    fn from(input: State) -> Self {
        Some(u8::from(input))
    }
}
impl From<State> for u16 {
    fn from(input: State) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<State> for Option<u16> {
    fn from(input: State) -> Self {
        Some(u16::from(input))
    }
}
impl From<State> for u32 {
    fn from(input: State) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<State> for Option<u32> {
    fn from(input: State) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for State {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(State::Off),
            1 => Ok(State::On),
            2 => Ok(State::Cycle),
            3 => Ok(State::Disabled),
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for State {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for State {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub client_major_version: u8,
    pub client_minor_version: u8,
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
        let client_major_version_bytes = self.client_major_version.serialize();
        let client_minor_version_bytes = self.client_minor_version.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            client_major_version_bytes[0],
            client_minor_version_bytes[0],
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
        let (client_major_version, remaining) = u8::try_parse(value)?;
        let (client_minor_version, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            client_major_version,
            client_minor_version,
        })
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn, client_major_version: u8, client_minor_version: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major_version, remaining) = u16::try_parse(remaining)?;
        let (server_minor_version, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply { sequence, length, server_major_version, server_minor_version };
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

/// Opcode for the QueryInfo request
pub const QUERY_INFO_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryInfoRequest {
    pub drawable: xproto::Drawable,
}
impl QueryInfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_INFO_REQUEST,
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryInfoReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_INFO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let _ = remaining;
        Ok(QueryInfoRequest {
            drawable,
        })
    }
}
impl Request for QueryInfoRequest {
    type Reply = QueryInfoReply;
}
pub fn query_info<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<Cookie<'_, Conn, QueryInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryInfoRequest {
        drawable,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryInfoReply {
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub saver_window: xproto::Window,
    pub ms_until_server: u32,
    pub ms_since_user_input: u32,
    pub event_mask: u32,
    pub kind: Kind,
}
impl TryParse for QueryInfoReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (saver_window, remaining) = xproto::Window::try_parse(remaining)?;
        let (ms_until_server, remaining) = u32::try_parse(remaining)?;
        let (ms_since_user_input, remaining) = u32::try_parse(remaining)?;
        let (event_mask, remaining) = u32::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(7..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let kind = kind.try_into()?;
        let result = QueryInfoReply { state, sequence, length, saver_window, ms_until_server, ms_since_user_input, event_mask, kind };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SelectInput request
pub const SELECT_INPUT_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectInputRequest {
    pub drawable: xproto::Drawable,
    pub event_mask: u32,
}
impl SelectInputRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let event_mask_bytes = self.event_mask.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SELECT_INPUT_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            event_mask_bytes[0],
            event_mask_bytes[1],
            event_mask_bytes[2],
            event_mask_bytes[3],
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
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SELECT_INPUT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (event_mask, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SelectInputRequest {
            drawable,
            event_mask,
        })
    }
}
impl Request for SelectInputRequest {
    type Reply = ();
}
pub fn select_input<Conn, A>(conn: &Conn, drawable: xproto::Drawable, event_mask: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectInputRequest {
        drawable,
        event_mask,
    };
    request0.send(conn)
}

/// Auxiliary and optional information for the `set_attributes` function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SetAttributesAux {
    pub background_pixmap: Option<xproto::Pixmap>,
    pub background_pixel: Option<u32>,
    pub border_pixmap: Option<xproto::Pixmap>,
    pub border_pixel: Option<u32>,
    pub bit_gravity: Option<xproto::Gravity>,
    pub win_gravity: Option<xproto::Gravity>,
    pub backing_store: Option<xproto::BackingStore>,
    pub backing_planes: Option<u32>,
    pub backing_pixel: Option<u32>,
    pub override_redirect: Option<xproto::Bool32>,
    pub save_under: Option<xproto::Bool32>,
    pub event_mask: Option<u32>,
    pub do_not_propogate_mask: Option<u32>,
    pub colormap: Option<xproto::Colormap>,
    pub cursor: Option<xproto::Cursor>,
}
impl SetAttributesAux {
    fn try_parse(value: &[u8], value_mask: u32) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = value_mask;
        let mut outer_remaining = value;
        let background_pixmap = if switch_expr & u32::from(xproto::CW::BackPixmap) != 0 {
            let remaining = outer_remaining;
            let (background_pixmap, remaining) = xproto::Pixmap::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(background_pixmap)
        } else {
            None
        };
        let background_pixel = if switch_expr & u32::from(xproto::CW::BackPixel) != 0 {
            let remaining = outer_remaining;
            let (background_pixel, remaining) = u32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(background_pixel)
        } else {
            None
        };
        let border_pixmap = if switch_expr & u32::from(xproto::CW::BorderPixmap) != 0 {
            let remaining = outer_remaining;
            let (border_pixmap, remaining) = xproto::Pixmap::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(border_pixmap)
        } else {
            None
        };
        let border_pixel = if switch_expr & u32::from(xproto::CW::BorderPixel) != 0 {
            let remaining = outer_remaining;
            let (border_pixel, remaining) = u32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(border_pixel)
        } else {
            None
        };
        let bit_gravity = if switch_expr & u32::from(xproto::CW::BitGravity) != 0 {
            let remaining = outer_remaining;
            let (bit_gravity, remaining) = u32::try_parse(remaining)?;
            let bit_gravity = xproto::Gravity::try_from(bit_gravity, xproto::Gravity::BitForget)?;
            outer_remaining = remaining;
            Some(bit_gravity)
        } else {
            None
        };
        let win_gravity = if switch_expr & u32::from(xproto::CW::WinGravity) != 0 {
            let remaining = outer_remaining;
            let (win_gravity, remaining) = u32::try_parse(remaining)?;
            let win_gravity = xproto::Gravity::try_from(win_gravity, xproto::Gravity::WinUnmap)?;
            outer_remaining = remaining;
            Some(win_gravity)
        } else {
            None
        };
        let backing_store = if switch_expr & u32::from(xproto::CW::BackingStore) != 0 {
            let remaining = outer_remaining;
            let (backing_store, remaining) = u32::try_parse(remaining)?;
            let backing_store = backing_store.try_into()?;
            outer_remaining = remaining;
            Some(backing_store)
        } else {
            None
        };
        let backing_planes = if switch_expr & u32::from(xproto::CW::BackingPlanes) != 0 {
            let remaining = outer_remaining;
            let (backing_planes, remaining) = u32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(backing_planes)
        } else {
            None
        };
        let backing_pixel = if switch_expr & u32::from(xproto::CW::BackingPixel) != 0 {
            let remaining = outer_remaining;
            let (backing_pixel, remaining) = u32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(backing_pixel)
        } else {
            None
        };
        let override_redirect = if switch_expr & u32::from(xproto::CW::OverrideRedirect) != 0 {
            let remaining = outer_remaining;
            let (override_redirect, remaining) = xproto::Bool32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(override_redirect)
        } else {
            None
        };
        let save_under = if switch_expr & u32::from(xproto::CW::SaveUnder) != 0 {
            let remaining = outer_remaining;
            let (save_under, remaining) = xproto::Bool32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(save_under)
        } else {
            None
        };
        let event_mask = if switch_expr & u32::from(xproto::CW::EventMask) != 0 {
            let remaining = outer_remaining;
            let (event_mask, remaining) = u32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(event_mask)
        } else {
            None
        };
        let do_not_propogate_mask = if switch_expr & u32::from(xproto::CW::DontPropagate) != 0 {
            let remaining = outer_remaining;
            let (do_not_propogate_mask, remaining) = u32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(do_not_propogate_mask)
        } else {
            None
        };
        let colormap = if switch_expr & u32::from(xproto::CW::Colormap) != 0 {
            let remaining = outer_remaining;
            let (colormap, remaining) = xproto::Colormap::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(colormap)
        } else {
            None
        };
        let cursor = if switch_expr & u32::from(xproto::CW::Cursor) != 0 {
            let remaining = outer_remaining;
            let (cursor, remaining) = xproto::Cursor::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(cursor)
        } else {
            None
        };
        let result = SetAttributesAux { background_pixmap, background_pixel, border_pixmap, border_pixel, bit_gravity, win_gravity, backing_store, backing_planes, backing_pixel, override_redirect, save_under, event_mask, do_not_propogate_mask, colormap, cursor };
        Ok((result, outer_remaining))
    }
}
impl SetAttributesAux {
    #[allow(dead_code)]
    fn serialize(&self, value_mask: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, value_mask);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, value_mask: u32) {
        assert_eq!(self.switch_expr(), value_mask, "switch `value_list` has an inconsistent discriminant");
        if let Some(background_pixmap) = self.background_pixmap {
            background_pixmap.serialize_into(bytes);
        }
        if let Some(background_pixel) = self.background_pixel {
            background_pixel.serialize_into(bytes);
        }
        if let Some(border_pixmap) = self.border_pixmap {
            border_pixmap.serialize_into(bytes);
        }
        if let Some(border_pixel) = self.border_pixel {
            border_pixel.serialize_into(bytes);
        }
        if let Some(bit_gravity) = self.bit_gravity {
            u32::from(bit_gravity).serialize_into(bytes);
        }
        if let Some(win_gravity) = self.win_gravity {
            u32::from(win_gravity).serialize_into(bytes);
        }
        if let Some(backing_store) = self.backing_store {
            u32::from(backing_store).serialize_into(bytes);
        }
        if let Some(backing_planes) = self.backing_planes {
            backing_planes.serialize_into(bytes);
        }
        if let Some(backing_pixel) = self.backing_pixel {
            backing_pixel.serialize_into(bytes);
        }
        if let Some(override_redirect) = self.override_redirect {
            override_redirect.serialize_into(bytes);
        }
        if let Some(save_under) = self.save_under {
            save_under.serialize_into(bytes);
        }
        if let Some(event_mask) = self.event_mask {
            event_mask.serialize_into(bytes);
        }
        if let Some(do_not_propogate_mask) = self.do_not_propogate_mask {
            do_not_propogate_mask.serialize_into(bytes);
        }
        if let Some(colormap) = self.colormap {
            colormap.serialize_into(bytes);
        }
        if let Some(cursor) = self.cursor {
            cursor.serialize_into(bytes);
        }
    }
}
impl SetAttributesAux {
    fn switch_expr(&self) -> u32 {
        let mut expr_value = 0;
        if self.background_pixmap.is_some() {
            expr_value |= u32::from(xproto::CW::BackPixmap);
        }
        if self.background_pixel.is_some() {
            expr_value |= u32::from(xproto::CW::BackPixel);
        }
        if self.border_pixmap.is_some() {
            expr_value |= u32::from(xproto::CW::BorderPixmap);
        }
        if self.border_pixel.is_some() {
            expr_value |= u32::from(xproto::CW::BorderPixel);
        }
        if self.bit_gravity.is_some() {
            expr_value |= u32::from(xproto::CW::BitGravity);
        }
        if self.win_gravity.is_some() {
            expr_value |= u32::from(xproto::CW::WinGravity);
        }
        if self.backing_store.is_some() {
            expr_value |= u32::from(xproto::CW::BackingStore);
        }
        if self.backing_planes.is_some() {
            expr_value |= u32::from(xproto::CW::BackingPlanes);
        }
        if self.backing_pixel.is_some() {
            expr_value |= u32::from(xproto::CW::BackingPixel);
        }
        if self.override_redirect.is_some() {
            expr_value |= u32::from(xproto::CW::OverrideRedirect);
        }
        if self.save_under.is_some() {
            expr_value |= u32::from(xproto::CW::SaveUnder);
        }
        if self.event_mask.is_some() {
            expr_value |= u32::from(xproto::CW::EventMask);
        }
        if self.do_not_propogate_mask.is_some() {
            expr_value |= u32::from(xproto::CW::DontPropagate);
        }
        if self.colormap.is_some() {
            expr_value |= u32::from(xproto::CW::Colormap);
        }
        if self.cursor.is_some() {
            expr_value |= u32::from(xproto::CW::Cursor);
        }
        expr_value
    }
}
impl SetAttributesAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the `background_pixmap` field of this structure.
    pub fn background_pixmap<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Pixmap>> {
        self.background_pixmap = value.into();
        self
    }
    /// Set the `background_pixel` field of this structure.
    pub fn background_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.background_pixel = value.into();
        self
    }
    /// Set the `border_pixmap` field of this structure.
    pub fn border_pixmap<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Pixmap>> {
        self.border_pixmap = value.into();
        self
    }
    /// Set the `border_pixel` field of this structure.
    pub fn border_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.border_pixel = value.into();
        self
    }
    /// Set the `bit_gravity` field of this structure.
    pub fn bit_gravity<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Gravity>> {
        self.bit_gravity = value.into();
        self
    }
    /// Set the `win_gravity` field of this structure.
    pub fn win_gravity<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Gravity>> {
        self.win_gravity = value.into();
        self
    }
    /// Set the `backing_store` field of this structure.
    pub fn backing_store<I>(mut self, value: I) -> Self where I: Into<Option<xproto::BackingStore>> {
        self.backing_store = value.into();
        self
    }
    /// Set the `backing_planes` field of this structure.
    pub fn backing_planes<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_planes = value.into();
        self
    }
    /// Set the `backing_pixel` field of this structure.
    pub fn backing_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_pixel = value.into();
        self
    }
    /// Set the `override_redirect` field of this structure.
    pub fn override_redirect<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Bool32>> {
        self.override_redirect = value.into();
        self
    }
    /// Set the `save_under` field of this structure.
    pub fn save_under<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Bool32>> {
        self.save_under = value.into();
        self
    }
    /// Set the `event_mask` field of this structure.
    pub fn event_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.event_mask = value.into();
        self
    }
    /// Set the `do_not_propogate_mask` field of this structure.
    pub fn do_not_propogate_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.do_not_propogate_mask = value.into();
        self
    }
    /// Set the `colormap` field of this structure.
    pub fn colormap<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Colormap>> {
        self.colormap = value.into();
        self
    }
    /// Set the `cursor` field of this structure.
    pub fn cursor<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Cursor>> {
        self.cursor = value.into();
        self
    }
}

/// Opcode for the SetAttributes request
pub const SET_ATTRIBUTES_REQUEST: u8 = 3;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAttributesRequest<'input> {
    pub drawable: xproto::Drawable,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub class: xproto::WindowClass,
    pub depth: u8,
    pub visual: xproto::Visualid,
    pub value_list: Cow<'input, SetAttributesAux>,
}
impl<'input> SetAttributesRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let border_width_bytes = self.border_width.serialize();
        let class_bytes = u8::from(self.class).serialize();
        let depth_bytes = self.depth.serialize();
        let visual_bytes = self.visual.serialize();
        let value_mask = self.value_list.switch_expr();
        let value_mask_bytes = value_mask.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_ATTRIBUTES_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            border_width_bytes[0],
            border_width_bytes[1],
            class_bytes[0],
            depth_bytes[0],
            visual_bytes[0],
            visual_bytes[1],
            visual_bytes[2],
            visual_bytes[3],
            value_mask_bytes[0],
            value_mask_bytes[1],
            value_mask_bytes[2],
            value_mask_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let value_list_bytes = self.value_list.serialize(value_mask);
        let length_so_far = length_so_far + value_list_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), value_list_bytes.into(), padding0.into()], vec![]))
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
        if header.minor_opcode != SET_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (border_width, remaining) = u16::try_parse(remaining)?;
        let (class, remaining) = u8::try_parse(remaining)?;
        let class = class.try_into()?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (visual, remaining) = xproto::Visualid::try_parse(remaining)?;
        let (value_mask, remaining) = u32::try_parse(remaining)?;
        let (value_list, remaining) = SetAttributesAux::try_parse(remaining, value_mask)?;
        let _ = remaining;
        Ok(SetAttributesRequest {
            drawable,
            x,
            y,
            width,
            height,
            border_width,
            class,
            depth,
            visual,
            value_list: Cow::Owned(value_list),
        })
    }
    /// Clone all borrowed data in this SetAttributesRequest.
    pub fn into_owned(self) -> SetAttributesRequest<'static> {
        SetAttributesRequest {
            drawable: self.drawable,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            border_width: self.border_width,
            class: self.class,
            depth: self.depth,
            visual: self.visual,
            value_list: Cow::Owned(self.value_list.into_owned()),
        }
    }
}
impl<'input> Request for SetAttributesRequest<'input> {
    type Reply = ();
}
pub fn set_attributes<'c, 'input, Conn>(conn: &'c Conn, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, border_width: u16, class: xproto::WindowClass, depth: u8, visual: xproto::Visualid, value_list: &'input SetAttributesAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetAttributesRequest {
        drawable,
        x,
        y,
        width,
        height,
        border_width,
        class,
        depth,
        visual,
        value_list: Cow::Borrowed(value_list),
    };
    request0.send(conn)
}

/// Opcode for the UnsetAttributes request
pub const UNSET_ATTRIBUTES_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsetAttributesRequest {
    pub drawable: xproto::Drawable,
}
impl UnsetAttributesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            UNSET_ATTRIBUTES_REQUEST,
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
        Ok((vec![request0.into()], vec![]))
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
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != UNSET_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let _ = remaining;
        Ok(UnsetAttributesRequest {
            drawable,
        })
    }
}
impl Request for UnsetAttributesRequest {
    type Reply = ();
}
pub fn unset_attributes<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UnsetAttributesRequest {
        drawable,
    };
    request0.send(conn)
}

/// Opcode for the Suspend request
pub const SUSPEND_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SuspendRequest {
    pub suspend: u32,
}
impl SuspendRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let suspend_bytes = self.suspend.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SUSPEND_REQUEST,
            0,
            0,
            suspend_bytes[0],
            suspend_bytes[1],
            suspend_bytes[2],
            suspend_bytes[3],
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
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SUSPEND_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (suspend, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(SuspendRequest {
            suspend,
        })
    }
}
impl Request for SuspendRequest {
    type Reply = ();
}
pub fn suspend<Conn>(conn: &Conn, suspend: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SuspendRequest {
        suspend,
    };
    request0.send(conn)
}

/// Opcode for the Notify event
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotifyEvent {
    pub response_type: u8,
    pub state: State,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub root: xproto::Window,
    pub window: xproto::Window,
    pub kind: Kind,
    pub forced: bool,
}
impl TryParse for NotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let (forced, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(14..).ok_or(ParseError::InsufficientData)?;
        let state = state.try_into()?;
        let kind = kind.try_into()?;
        let result = NotifyEvent { response_type, state, sequence, time, root, window, kind, forced };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
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
        let state_bytes = u8::from(input.state).serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let root_bytes = input.root.serialize();
        let window_bytes = input.window.serialize();
        let kind_bytes = u8::from(input.kind).serialize();
        let forced_bytes = input.forced.serialize();
        [
            response_type_bytes[0],
            state_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            root_bytes[0],
            root_bytes[1],
            root_bytes[2],
            root_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            kind_bytes[0],
            forced_bytes[0],
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
impl From<NotifyEvent> for [u8; 32] {
    fn from(input: NotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn screensaver_query_version(&self, client_major_version: u8, client_minor_version: u8) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }
    fn screensaver_query_info(&self, drawable: xproto::Drawable) -> Result<Cookie<'_, Self, QueryInfoReply>, ConnectionError>
    {
        query_info(self, drawable)
    }
    fn screensaver_select_input<A>(&self, drawable: xproto::Drawable, event_mask: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        select_input(self, drawable, event_mask)
    }
    fn screensaver_set_attributes<'c, 'input>(&'c self, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, border_width: u16, class: xproto::WindowClass, depth: u8, visual: xproto::Visualid, value_list: &'input SetAttributesAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_attributes(self, drawable, x, y, width, height, border_width, class, depth, visual, value_list)
    }
    fn screensaver_unset_attributes(&self, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        unset_attributes(self, drawable)
    }
    fn screensaver_suspend(&self, suspend: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        self::suspend(self, suspend)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
