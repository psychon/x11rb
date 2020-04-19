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
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Kind {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Kind {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Event {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Event {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(Event, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for State {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for State {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, client_major_version: u8, client_minor_version: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major_version, remaining) = u16::try_parse(remaining)?;
        let (server_minor_version, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = QueryVersionReply { response_type, sequence, length, server_major_version, server_minor_version };
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
pub fn query_info<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<Cookie<'_, Conn, QueryInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let mut request0 = [
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryInfoReply {
    pub response_type: u8,
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
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (saver_window, remaining) = xproto::Window::try_parse(remaining)?;
        let (ms_until_server, remaining) = u32::try_parse(remaining)?;
        let (ms_since_user_input, remaining) = u32::try_parse(remaining)?;
        let (event_mask, remaining) = u32::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(7..).ok_or(ParseError::ParseError)?;
        let kind = kind.try_into()?;
        let result = QueryInfoReply { response_type, state, sequence, length, saver_window, ms_until_server, ms_since_user_input, event_mask, kind };
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
pub fn select_input<Conn>(conn: &Conn, drawable: xproto::Drawable, event_mask: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let event_mask_bytes = event_mask.serialize();
    let mut request0 = [
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the SetAttributes request
pub const SET_ATTRIBUTES_REQUEST: u8 = 3;
/// Auxiliary and optional information for the set_attributes function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SetAttributesAux {
    pub background_pixmap: Option<xproto::Pixmap>,
    pub background_pixel: Option<u32>,
    pub border_pixmap: Option<xproto::Pixmap>,
    pub border_pixel: Option<u32>,
    pub bit_gravity: Option<u32>,
    pub win_gravity: Option<u32>,
    pub backing_store: Option<u32>,
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
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.background_pixmap.is_some() {
            mask |= u32::from(xproto::CW::BackPixmap);
        }
        if self.background_pixel.is_some() {
            mask |= u32::from(xproto::CW::BackPixel);
        }
        if self.border_pixmap.is_some() {
            mask |= u32::from(xproto::CW::BorderPixmap);
        }
        if self.border_pixel.is_some() {
            mask |= u32::from(xproto::CW::BorderPixel);
        }
        if self.bit_gravity.is_some() {
            mask |= u32::from(xproto::CW::BitGravity);
        }
        if self.win_gravity.is_some() {
            mask |= u32::from(xproto::CW::WinGravity);
        }
        if self.backing_store.is_some() {
            mask |= u32::from(xproto::CW::BackingStore);
        }
        if self.backing_planes.is_some() {
            mask |= u32::from(xproto::CW::BackingPlanes);
        }
        if self.backing_pixel.is_some() {
            mask |= u32::from(xproto::CW::BackingPixel);
        }
        if self.override_redirect.is_some() {
            mask |= u32::from(xproto::CW::OverrideRedirect);
        }
        if self.save_under.is_some() {
            mask |= u32::from(xproto::CW::SaveUnder);
        }
        if self.event_mask.is_some() {
            mask |= u32::from(xproto::CW::EventMask);
        }
        if self.do_not_propogate_mask.is_some() {
            mask |= u32::from(xproto::CW::DontPropagate);
        }
        if self.colormap.is_some() {
            mask |= u32::from(xproto::CW::Colormap);
        }
        if self.cursor.is_some() {
            mask |= u32::from(xproto::CW::Cursor);
        }
        mask
    }
    /// Set the background_pixmap field of this structure.
    pub fn background_pixmap<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Pixmap>> {
        self.background_pixmap = value.into();
        self
    }
    /// Set the background_pixel field of this structure.
    pub fn background_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.background_pixel = value.into();
        self
    }
    /// Set the border_pixmap field of this structure.
    pub fn border_pixmap<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Pixmap>> {
        self.border_pixmap = value.into();
        self
    }
    /// Set the border_pixel field of this structure.
    pub fn border_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.border_pixel = value.into();
        self
    }
    /// Set the bit_gravity field of this structure.
    pub fn bit_gravity<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.bit_gravity = value.into();
        self
    }
    /// Set the win_gravity field of this structure.
    pub fn win_gravity<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.win_gravity = value.into();
        self
    }
    /// Set the backing_store field of this structure.
    pub fn backing_store<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_store = value.into();
        self
    }
    /// Set the backing_planes field of this structure.
    pub fn backing_planes<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_planes = value.into();
        self
    }
    /// Set the backing_pixel field of this structure.
    pub fn backing_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_pixel = value.into();
        self
    }
    /// Set the override_redirect field of this structure.
    pub fn override_redirect<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Bool32>> {
        self.override_redirect = value.into();
        self
    }
    /// Set the save_under field of this structure.
    pub fn save_under<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Bool32>> {
        self.save_under = value.into();
        self
    }
    /// Set the event_mask field of this structure.
    pub fn event_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.event_mask = value.into();
        self
    }
    /// Set the do_not_propogate_mask field of this structure.
    pub fn do_not_propogate_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.do_not_propogate_mask = value.into();
        self
    }
    /// Set the colormap field of this structure.
    pub fn colormap<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Colormap>> {
        self.colormap = value.into();
        self
    }
    /// Set the cursor field of this structure.
    pub fn cursor<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Cursor>> {
        self.cursor = value.into();
        self
    }
}
impl Serialize for SetAttributesAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.background_pixmap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.background_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.border_pixmap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.border_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bit_gravity {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.win_gravity {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_store {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_planes {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.override_redirect {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.save_under {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.event_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.do_not_propogate_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.colormap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.cursor {
            value.serialize_into(bytes);
        }
    }
}
pub fn set_attributes<'c, Conn, A>(conn: &'c Conn, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, border_width: u16, class: A, depth: u8, visual: xproto::Visualid, value_list: &SetAttributesAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let border_width_bytes = border_width.serialize();
    let class = class.into();
    let class_bytes = class.serialize();
    let depth_bytes = depth.serialize();
    let visual_bytes = visual.serialize();
    let value_mask_bytes = value_mask.serialize();
    let mut request0 = [
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
    let length_so_far = length_so_far + value_list_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the UnsetAttributes request
pub const UNSET_ATTRIBUTES_REQUEST: u8 = 4;
pub fn unset_attributes<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let mut request0 = [
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the Suspend request
pub const SUSPEND_REQUEST: u8 = 5;
pub fn suspend<Conn>(conn: &Conn, suspend: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let suspend_bytes = suspend.serialize();
    let mut request0 = [
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
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
impl NotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let (forced, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(14..).ok_or(ParseError::ParseError)?;
        let state = state.try_into()?;
        let kind = kind.try_into()?;
        let result = NotifyEvent { response_type, state, sequence, time, root, window, kind, forced };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for NotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for NotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for NotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
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
            response_type_bytes[0], state_bytes[0], sequence_bytes[0], sequence_bytes[1], time_bytes[0], time_bytes[1], time_bytes[2], time_bytes[3],
            root_bytes[0], root_bytes[1], root_bytes[2], root_bytes[3], window_bytes[0], window_bytes[1], window_bytes[2], window_bytes[3],
            kind_bytes[0], forced_bytes[0], 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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

    fn screensaver_select_input(&self, drawable: xproto::Drawable, event_mask: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_input(self, drawable, event_mask)
    }

    fn screensaver_set_attributes<'c, A>(&'c self, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, border_width: u16, class: A, depth: u8, visual: xproto::Visualid, value_list: &SetAttributesAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
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
