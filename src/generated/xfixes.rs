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
use super::render;
#[allow(unused_imports)]
use super::shape;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XFIXES";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (5, 0);

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let client_major_version_bytes = client_major_version.serialize();
    let client_minor_version_bytes = client_minor_version.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        client_major_version_bytes[0],
        client_major_version_bytes[1],
        client_major_version_bytes[2],
        client_major_version_bytes[3],
        client_minor_version_bytes[0],
        client_minor_version_bytes[1],
        client_minor_version_bytes[2],
        client_minor_version_bytes[3],
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
    pub major_version: u32,
    pub minor_version: u32,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u32::try_parse(remaining)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SaveSetMode {
    Insert = 0,
    Delete = 1,
}
impl From<SaveSetMode> for u8 {
    fn from(input: SaveSetMode) -> Self {
        match input {
            SaveSetMode::Insert => 0,
            SaveSetMode::Delete => 1,
        }
    }
}
impl From<SaveSetMode> for Option<u8> {
    fn from(input: SaveSetMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<SaveSetMode> for u16 {
    fn from(input: SaveSetMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SaveSetMode> for Option<u16> {
    fn from(input: SaveSetMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<SaveSetMode> for u32 {
    fn from(input: SaveSetMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SaveSetMode> for Option<u32> {
    fn from(input: SaveSetMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SaveSetMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SaveSetMode::Insert),
            1 => Ok(SaveSetMode::Delete),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SaveSetMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SaveSetMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SaveSetTarget {
    Nearest = 0,
    Root = 1,
}
impl From<SaveSetTarget> for u8 {
    fn from(input: SaveSetTarget) -> Self {
        match input {
            SaveSetTarget::Nearest => 0,
            SaveSetTarget::Root => 1,
        }
    }
}
impl From<SaveSetTarget> for Option<u8> {
    fn from(input: SaveSetTarget) -> Self {
        Some(u8::from(input))
    }
}
impl From<SaveSetTarget> for u16 {
    fn from(input: SaveSetTarget) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SaveSetTarget> for Option<u16> {
    fn from(input: SaveSetTarget) -> Self {
        Some(u16::from(input))
    }
}
impl From<SaveSetTarget> for u32 {
    fn from(input: SaveSetTarget) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SaveSetTarget> for Option<u32> {
    fn from(input: SaveSetTarget) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SaveSetTarget {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SaveSetTarget::Nearest),
            1 => Ok(SaveSetTarget::Root),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SaveSetTarget {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SaveSetTarget {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SaveSetMapping {
    Map = 0,
    Unmap = 1,
}
impl From<SaveSetMapping> for u8 {
    fn from(input: SaveSetMapping) -> Self {
        match input {
            SaveSetMapping::Map => 0,
            SaveSetMapping::Unmap => 1,
        }
    }
}
impl From<SaveSetMapping> for Option<u8> {
    fn from(input: SaveSetMapping) -> Self {
        Some(u8::from(input))
    }
}
impl From<SaveSetMapping> for u16 {
    fn from(input: SaveSetMapping) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SaveSetMapping> for Option<u16> {
    fn from(input: SaveSetMapping) -> Self {
        Some(u16::from(input))
    }
}
impl From<SaveSetMapping> for u32 {
    fn from(input: SaveSetMapping) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SaveSetMapping> for Option<u32> {
    fn from(input: SaveSetMapping) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SaveSetMapping {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SaveSetMapping::Map),
            1 => Ok(SaveSetMapping::Unmap),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SaveSetMapping {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SaveSetMapping {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ChangeSaveSet request
pub const CHANGE_SAVE_SET_REQUEST: u8 = 1;
pub fn change_save_set<Conn, A, B, C>(conn: &Conn, mode: A, target: B, map: C, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>, C: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let target = target.into();
    let target_bytes = target.serialize();
    let map = map.into();
    let map_bytes = map.serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_SAVE_SET_REQUEST,
        length_bytes[0],
        length_bytes[1],
        mode_bytes[0],
        target_bytes[0],
        map_bytes[0],
        0,
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SelectionEvent {
    SetSelectionOwner = 0,
    SelectionWindowDestroy = 1,
    SelectionClientClose = 2,
}
impl From<SelectionEvent> for u8 {
    fn from(input: SelectionEvent) -> Self {
        match input {
            SelectionEvent::SetSelectionOwner => 0,
            SelectionEvent::SelectionWindowDestroy => 1,
            SelectionEvent::SelectionClientClose => 2,
        }
    }
}
impl From<SelectionEvent> for Option<u8> {
    fn from(input: SelectionEvent) -> Self {
        Some(u8::from(input))
    }
}
impl From<SelectionEvent> for u16 {
    fn from(input: SelectionEvent) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SelectionEvent> for Option<u16> {
    fn from(input: SelectionEvent) -> Self {
        Some(u16::from(input))
    }
}
impl From<SelectionEvent> for u32 {
    fn from(input: SelectionEvent) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SelectionEvent> for Option<u32> {
    fn from(input: SelectionEvent) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SelectionEvent {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SelectionEvent::SetSelectionOwner),
            1 => Ok(SelectionEvent::SelectionWindowDestroy),
            2 => Ok(SelectionEvent::SelectionClientClose),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SelectionEvent {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SelectionEvent {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SelectionEventMask {
    SetSelectionOwner = 1 << 0,
    SelectionWindowDestroy = 1 << 1,
    SelectionClientClose = 1 << 2,
}
impl From<SelectionEventMask> for u8 {
    fn from(input: SelectionEventMask) -> Self {
        match input {
            SelectionEventMask::SetSelectionOwner => 1 << 0,
            SelectionEventMask::SelectionWindowDestroy => 1 << 1,
            SelectionEventMask::SelectionClientClose => 1 << 2,
        }
    }
}
impl From<SelectionEventMask> for Option<u8> {
    fn from(input: SelectionEventMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<SelectionEventMask> for u16 {
    fn from(input: SelectionEventMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SelectionEventMask> for Option<u16> {
    fn from(input: SelectionEventMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<SelectionEventMask> for u32 {
    fn from(input: SelectionEventMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SelectionEventMask> for Option<u32> {
    fn from(input: SelectionEventMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SelectionEventMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SelectionEventMask::SetSelectionOwner),
            2 => Ok(SelectionEventMask::SelectionWindowDestroy),
            4 => Ok(SelectionEventMask::SelectionClientClose),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SelectionEventMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SelectionEventMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(SelectionEventMask, u8);

/// Opcode for the SelectionNotify event
pub const SELECTION_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionNotifyEvent {
    pub response_type: u8,
    pub subtype: SelectionEvent,
    pub sequence: u16,
    pub window: xproto::Window,
    pub owner: xproto::Window,
    pub selection: xproto::Atom,
    pub timestamp: xproto::Timestamp,
    pub selection_timestamp: xproto::Timestamp,
}
impl SelectionNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (subtype, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (owner, remaining) = xproto::Window::try_parse(remaining)?;
        let (selection, remaining) = xproto::Atom::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (selection_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let subtype = subtype.try_into()?;
        let result = SelectionNotifyEvent { response_type, subtype, sequence, window, owner, selection, timestamp, selection_timestamp };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SelectionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for SelectionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for SelectionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&SelectionNotifyEvent> for [u8; 32] {
    fn from(input: &SelectionNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let subtype = Into::<u8>::into(input.subtype).serialize();
        let sequence = input.sequence.serialize();
        let window = input.window.serialize();
        let owner = input.owner.serialize();
        let selection = input.selection.serialize();
        let timestamp = input.timestamp.serialize();
        let selection_timestamp = input.selection_timestamp.serialize();
        [
            response_type[0], subtype[0], sequence[0], sequence[1], window[0], window[1], window[2], window[3],
            owner[0], owner[1], owner[2], owner[3], selection[0], selection[1], selection[2], selection[3],
            timestamp[0], timestamp[1], timestamp[2], timestamp[3], selection_timestamp[0], selection_timestamp[1], selection_timestamp[2], selection_timestamp[3],
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<SelectionNotifyEvent> for [u8; 32] {
    fn from(input: SelectionNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the SelectSelectionInput request
pub const SELECT_SELECTION_INPUT_REQUEST: u8 = 2;
pub fn select_selection_input<Conn>(conn: &Conn, window: xproto::Window, selection: xproto::Atom, event_mask: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let selection_bytes = selection.serialize();
    let event_mask_bytes = event_mask.serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_SELECTION_INPUT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        selection_bytes[0],
        selection_bytes[1],
        selection_bytes[2],
        selection_bytes[3],
        event_mask_bytes[0],
        event_mask_bytes[1],
        event_mask_bytes[2],
        event_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CursorNotify {
    DisplayCursor = 0,
}
impl From<CursorNotify> for u8 {
    fn from(input: CursorNotify) -> Self {
        match input {
            CursorNotify::DisplayCursor => 0,
        }
    }
}
impl From<CursorNotify> for Option<u8> {
    fn from(input: CursorNotify) -> Self {
        Some(u8::from(input))
    }
}
impl From<CursorNotify> for u16 {
    fn from(input: CursorNotify) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CursorNotify> for Option<u16> {
    fn from(input: CursorNotify) -> Self {
        Some(u16::from(input))
    }
}
impl From<CursorNotify> for u32 {
    fn from(input: CursorNotify) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CursorNotify> for Option<u32> {
    fn from(input: CursorNotify) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CursorNotify {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CursorNotify::DisplayCursor),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CursorNotify {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CursorNotify {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CursorNotifyMask {
    DisplayCursor = 1 << 0,
}
impl From<CursorNotifyMask> for u8 {
    fn from(input: CursorNotifyMask) -> Self {
        match input {
            CursorNotifyMask::DisplayCursor => 1 << 0,
        }
    }
}
impl From<CursorNotifyMask> for Option<u8> {
    fn from(input: CursorNotifyMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<CursorNotifyMask> for u16 {
    fn from(input: CursorNotifyMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CursorNotifyMask> for Option<u16> {
    fn from(input: CursorNotifyMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<CursorNotifyMask> for u32 {
    fn from(input: CursorNotifyMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CursorNotifyMask> for Option<u32> {
    fn from(input: CursorNotifyMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CursorNotifyMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CursorNotifyMask::DisplayCursor),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CursorNotifyMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CursorNotifyMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(CursorNotifyMask, u8);

/// Opcode for the CursorNotify event
pub const CURSOR_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorNotifyEvent {
    pub response_type: u8,
    pub subtype: CursorNotify,
    pub sequence: u16,
    pub window: xproto::Window,
    pub cursor_serial: u32,
    pub timestamp: xproto::Timestamp,
    pub name: xproto::Atom,
}
impl CursorNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (subtype, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (cursor_serial, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let subtype = subtype.try_into()?;
        let result = CursorNotifyEvent { response_type, subtype, sequence, window, cursor_serial, timestamp, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CursorNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for CursorNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for CursorNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&CursorNotifyEvent> for [u8; 32] {
    fn from(input: &CursorNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let subtype = Into::<u8>::into(input.subtype).serialize();
        let sequence = input.sequence.serialize();
        let window = input.window.serialize();
        let cursor_serial = input.cursor_serial.serialize();
        let timestamp = input.timestamp.serialize();
        let name = input.name.serialize();
        [
            response_type[0], subtype[0], sequence[0], sequence[1], window[0], window[1], window[2], window[3],
            cursor_serial[0], cursor_serial[1], cursor_serial[2], cursor_serial[3], timestamp[0], timestamp[1], timestamp[2], timestamp[3],
            name[0], name[1], name[2], name[3], 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<CursorNotifyEvent> for [u8; 32] {
    fn from(input: CursorNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the SelectCursorInput request
pub const SELECT_CURSOR_INPUT_REQUEST: u8 = 3;
pub fn select_cursor_input<Conn>(conn: &Conn, window: xproto::Window, event_mask: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let event_mask_bytes = event_mask.serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_CURSOR_INPUT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        event_mask_bytes[0],
        event_mask_bytes[1],
        event_mask_bytes[2],
        event_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetCursorImage request
pub const GET_CURSOR_IMAGE_REQUEST: u8 = 4;
pub fn get_cursor_image<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetCursorImageReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CURSOR_IMAGE_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCursorImageReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: u32,
    pub cursor_image: Vec<u32>,
}
impl GetCursorImageReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (xhot, remaining) = u16::try_parse(remaining)?;
        let (yhot, remaining) = u16::try_parse(remaining)?;
        let (cursor_serial, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (cursor_image, remaining) = crate::x11_utils::parse_list::<u32>(remaining, (width as usize) * (height as usize))?;
        let result = GetCursorImageReply { response_type, sequence, length, x, y, width, height, xhot, yhot, cursor_serial, cursor_image };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCursorImageReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

pub type Region = u32;

/// Opcode for the BadRegion error
pub const BAD_REGION_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadRegionError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadRegionError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadRegionError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadRegionError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadRegionError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadRegionError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadRegionError> for [u8; 32] {
    fn from(input: &BadRegionError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadRegionError> for [u8; 32] {
    fn from(input: BadRegionError) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegionEnum {
    None = 0,
}
impl From<RegionEnum> for u8 {
    fn from(input: RegionEnum) -> Self {
        match input {
            RegionEnum::None => 0,
        }
    }
}
impl From<RegionEnum> for Option<u8> {
    fn from(input: RegionEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<RegionEnum> for u16 {
    fn from(input: RegionEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<RegionEnum> for Option<u16> {
    fn from(input: RegionEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<RegionEnum> for u32 {
    fn from(input: RegionEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<RegionEnum> for Option<u32> {
    fn from(input: RegionEnum) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for RegionEnum {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RegionEnum::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for RegionEnum {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for RegionEnum {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the CreateRegion request
pub const CREATE_REGION_REQUEST: u8 = 5;
pub fn create_region<'c, Conn>(conn: &'c Conn, region: Region, rectangles: &[xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 8 * rectangles.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let rectangles_bytes = rectangles.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&rectangles_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&rectangles_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the CreateRegionFromBitmap request
pub const CREATE_REGION_FROM_BITMAP_REQUEST: u8 = 6;
pub fn create_region_from_bitmap<Conn>(conn: &Conn, region: Region, bitmap: xproto::Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let bitmap_bytes = bitmap.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_REGION_FROM_BITMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
        bitmap_bytes[0],
        bitmap_bytes[1],
        bitmap_bytes[2],
        bitmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CreateRegionFromWindow request
pub const CREATE_REGION_FROM_WINDOW_REQUEST: u8 = 7;
pub fn create_region_from_window<Conn, A>(conn: &Conn, region: Region, window: xproto::Window, kind: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<shape::Kind>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let window_bytes = window.serialize();
    let kind = kind.into();
    let kind_bytes = kind.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_REGION_FROM_WINDOW_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        kind_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CreateRegionFromGC request
pub const CREATE_REGION_FROM_GC_REQUEST: u8 = 8;
pub fn create_region_from_gc<Conn>(conn: &Conn, region: Region, gc: xproto::Gcontext) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let gc_bytes = gc.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_REGION_FROM_GC_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CreateRegionFromPicture request
pub const CREATE_REGION_FROM_PICTURE_REQUEST: u8 = 9;
pub fn create_region_from_picture<Conn>(conn: &Conn, region: Region, picture: render::Picture) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let picture_bytes = picture.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_REGION_FROM_PICTURE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the DestroyRegion request
pub const DESTROY_REGION_REQUEST: u8 = 10;
pub fn destroy_region<Conn>(conn: &Conn, region: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetRegion request
pub const SET_REGION_REQUEST: u8 = 11;
pub fn set_region<'c, Conn>(conn: &'c Conn, region: Region, rectangles: &[xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 8 * rectangles.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let rectangles_bytes = rectangles.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&rectangles_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&rectangles_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the CopyRegion request
pub const COPY_REGION_REQUEST: u8 = 12;
pub fn copy_region<Conn>(conn: &Conn, source: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let source_bytes = source.serialize();
    let destination_bytes = destination.serialize();
    let request0 = [
        extension_information.major_opcode,
        COPY_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        source_bytes[0],
        source_bytes[1],
        source_bytes[2],
        source_bytes[3],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the UnionRegion request
pub const UNION_REGION_REQUEST: u8 = 13;
pub fn union_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let source1_bytes = source1.serialize();
    let source2_bytes = source2.serialize();
    let destination_bytes = destination.serialize();
    let request0 = [
        extension_information.major_opcode,
        UNION_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        source1_bytes[0],
        source1_bytes[1],
        source1_bytes[2],
        source1_bytes[3],
        source2_bytes[0],
        source2_bytes[1],
        source2_bytes[2],
        source2_bytes[3],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the IntersectRegion request
pub const INTERSECT_REGION_REQUEST: u8 = 14;
pub fn intersect_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let source1_bytes = source1.serialize();
    let source2_bytes = source2.serialize();
    let destination_bytes = destination.serialize();
    let request0 = [
        extension_information.major_opcode,
        INTERSECT_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        source1_bytes[0],
        source1_bytes[1],
        source1_bytes[2],
        source1_bytes[3],
        source2_bytes[0],
        source2_bytes[1],
        source2_bytes[2],
        source2_bytes[3],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SubtractRegion request
pub const SUBTRACT_REGION_REQUEST: u8 = 15;
pub fn subtract_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let source1_bytes = source1.serialize();
    let source2_bytes = source2.serialize();
    let destination_bytes = destination.serialize();
    let request0 = [
        extension_information.major_opcode,
        SUBTRACT_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        source1_bytes[0],
        source1_bytes[1],
        source1_bytes[2],
        source1_bytes[3],
        source2_bytes[0],
        source2_bytes[1],
        source2_bytes[2],
        source2_bytes[3],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the InvertRegion request
pub const INVERT_REGION_REQUEST: u8 = 16;
pub fn invert_region<Conn>(conn: &Conn, source: Region, bounds: xproto::Rectangle, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let source_bytes = source.serialize();
    let bounds_bytes = bounds.serialize();
    let destination_bytes = destination.serialize();
    let request0 = [
        extension_information.major_opcode,
        INVERT_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        source_bytes[0],
        source_bytes[1],
        source_bytes[2],
        source_bytes[3],
        bounds_bytes[0],
        bounds_bytes[1],
        bounds_bytes[2],
        bounds_bytes[3],
        bounds_bytes[4],
        bounds_bytes[5],
        bounds_bytes[6],
        bounds_bytes[7],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the TranslateRegion request
pub const TRANSLATE_REGION_REQUEST: u8 = 17;
pub fn translate_region<Conn>(conn: &Conn, region: Region, dx: i16, dy: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let dx_bytes = dx.serialize();
    let dy_bytes = dy.serialize();
    let request0 = [
        extension_information.major_opcode,
        TRANSLATE_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
        dx_bytes[0],
        dx_bytes[1],
        dy_bytes[0],
        dy_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the RegionExtents request
pub const REGION_EXTENTS_REQUEST: u8 = 18;
pub fn region_extents<Conn>(conn: &Conn, source: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let source_bytes = source.serialize();
    let destination_bytes = destination.serialize();
    let request0 = [
        extension_information.major_opcode,
        REGION_EXTENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        source_bytes[0],
        source_bytes[1],
        source_bytes[2],
        source_bytes[3],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the FetchRegion request
pub const FETCH_REGION_REQUEST: u8 = 19;
pub fn fetch_region<Conn>(conn: &Conn, region: Region) -> Result<Cookie<'_, Conn, FetchRegionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let region_bytes = region.serialize();
    let request0 = [
        extension_information.major_opcode,
        FETCH_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchRegionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub extents: xproto::Rectangle,
    pub rectangles: Vec<xproto::Rectangle>,
}
impl FetchRegionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (extents, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (rectangles, remaining) = crate::x11_utils::parse_list::<xproto::Rectangle>(remaining, (length as usize) / (2))?;
        let result = FetchRegionReply { response_type, sequence, extents, rectangles };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FetchRegionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetGCClipRegion request
pub const SET_GC_CLIP_REGION_REQUEST: u8 = 20;
pub fn set_gc_clip_region<Conn>(conn: &Conn, gc: xproto::Gcontext, region: Region, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let gc_bytes = gc.serialize();
    let region_bytes = region.serialize();
    let x_origin_bytes = x_origin.serialize();
    let y_origin_bytes = y_origin.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_GC_CLIP_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
        x_origin_bytes[0],
        x_origin_bytes[1],
        y_origin_bytes[0],
        y_origin_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetWindowShapeRegion request
pub const SET_WINDOW_SHAPE_REGION_REQUEST: u8 = 21;
pub fn set_window_shape_region<Conn, A>(conn: &Conn, dest: xproto::Window, dest_kind: A, x_offset: i16, y_offset: i16, region: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<shape::Kind>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let dest_bytes = dest.serialize();
    let dest_kind = dest_kind.into();
    let dest_kind_bytes = dest_kind.serialize();
    let x_offset_bytes = x_offset.serialize();
    let y_offset_bytes = y_offset.serialize();
    let region_bytes = region.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_WINDOW_SHAPE_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        dest_bytes[0],
        dest_bytes[1],
        dest_bytes[2],
        dest_bytes[3],
        dest_kind_bytes[0],
        0,
        0,
        0,
        x_offset_bytes[0],
        x_offset_bytes[1],
        y_offset_bytes[0],
        y_offset_bytes[1],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetPictureClipRegion request
pub const SET_PICTURE_CLIP_REGION_REQUEST: u8 = 22;
pub fn set_picture_clip_region<Conn>(conn: &Conn, picture: render::Picture, region: Region, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let picture_bytes = picture.serialize();
    let region_bytes = region.serialize();
    let x_origin_bytes = x_origin.serialize();
    let y_origin_bytes = y_origin.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_PICTURE_CLIP_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
        x_origin_bytes[0],
        x_origin_bytes[1],
        y_origin_bytes[0],
        y_origin_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetCursorName request
pub const SET_CURSOR_NAME_REQUEST: u8 = 23;
pub fn set_cursor_name<'c, Conn>(conn: &'c Conn, cursor: xproto::Cursor, name: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 1 * name.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let cursor_bytes = cursor.serialize();
    let nbytes: u16 = name.len().try_into()?;
    let nbytes_bytes = nbytes.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_CURSOR_NAME_REQUEST,
        length_bytes[0],
        length_bytes[1],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
        nbytes_bytes[0],
        nbytes_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetCursorName request
pub const GET_CURSOR_NAME_REQUEST: u8 = 24;
pub fn get_cursor_name<Conn>(conn: &Conn, cursor: xproto::Cursor) -> Result<Cookie<'_, Conn, GetCursorNameReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let cursor_bytes = cursor.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CURSOR_NAME_REQUEST,
        length_bytes[0],
        length_bytes[1],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCursorNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xproto::Atom,
    pub name: Vec<u8>,
}
impl GetCursorNameReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (atom, remaining) = xproto::Atom::try_parse(remaining)?;
        let (nbytes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, nbytes as usize)?;
        let result = GetCursorNameReply { response_type, sequence, length, atom, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCursorNameReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetCursorImageAndName request
pub const GET_CURSOR_IMAGE_AND_NAME_REQUEST: u8 = 25;
pub fn get_cursor_image_and_name<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetCursorImageAndNameReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CURSOR_IMAGE_AND_NAME_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCursorImageAndNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: u32,
    pub cursor_atom: xproto::Atom,
    pub cursor_image: Vec<u32>,
    pub name: Vec<u8>,
}
impl GetCursorImageAndNameReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (xhot, remaining) = u16::try_parse(remaining)?;
        let (yhot, remaining) = u16::try_parse(remaining)?;
        let (cursor_serial, remaining) = u32::try_parse(remaining)?;
        let (cursor_atom, remaining) = xproto::Atom::try_parse(remaining)?;
        let (nbytes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (cursor_image, remaining) = crate::x11_utils::parse_list::<u32>(remaining, (width as usize) * (height as usize))?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, nbytes as usize)?;
        let result = GetCursorImageAndNameReply { response_type, sequence, length, x, y, width, height, xhot, yhot, cursor_serial, cursor_atom, cursor_image, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCursorImageAndNameReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ChangeCursor request
pub const CHANGE_CURSOR_REQUEST: u8 = 26;
pub fn change_cursor<Conn>(conn: &Conn, source: xproto::Cursor, destination: xproto::Cursor) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let source_bytes = source.serialize();
    let destination_bytes = destination.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_CURSOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
        source_bytes[0],
        source_bytes[1],
        source_bytes[2],
        source_bytes[3],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ChangeCursorByName request
pub const CHANGE_CURSOR_BY_NAME_REQUEST: u8 = 27;
pub fn change_cursor_by_name<'c, Conn>(conn: &'c Conn, src: xproto::Cursor, name: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 1 * name.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let src_bytes = src.serialize();
    let nbytes: u16 = name.len().try_into()?;
    let nbytes_bytes = nbytes.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_CURSOR_BY_NAME_REQUEST,
        length_bytes[0],
        length_bytes[1],
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        nbytes_bytes[0],
        nbytes_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ExpandRegion request
pub const EXPAND_REGION_REQUEST: u8 = 28;
pub fn expand_region<Conn>(conn: &Conn, source: Region, destination: Region, left: u16, right: u16, top: u16, bottom: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let source_bytes = source.serialize();
    let destination_bytes = destination.serialize();
    let left_bytes = left.serialize();
    let right_bytes = right.serialize();
    let top_bytes = top.serialize();
    let bottom_bytes = bottom.serialize();
    let request0 = [
        extension_information.major_opcode,
        EXPAND_REGION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        source_bytes[0],
        source_bytes[1],
        source_bytes[2],
        source_bytes[3],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
        left_bytes[0],
        left_bytes[1],
        right_bytes[0],
        right_bytes[1],
        top_bytes[0],
        top_bytes[1],
        bottom_bytes[0],
        bottom_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the HideCursor request
pub const HIDE_CURSOR_REQUEST: u8 = 29;
pub fn hide_cursor<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        HIDE_CURSOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ShowCursor request
pub const SHOW_CURSOR_REQUEST: u8 = 30;
pub fn show_cursor<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        SHOW_CURSOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

pub type Barrier = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BarrierDirections {
    PositiveX = 1 << 0,
    PositiveY = 1 << 1,
    NegativeX = 1 << 2,
    NegativeY = 1 << 3,
}
impl From<BarrierDirections> for u8 {
    fn from(input: BarrierDirections) -> Self {
        match input {
            BarrierDirections::PositiveX => 1 << 0,
            BarrierDirections::PositiveY => 1 << 1,
            BarrierDirections::NegativeX => 1 << 2,
            BarrierDirections::NegativeY => 1 << 3,
        }
    }
}
impl From<BarrierDirections> for Option<u8> {
    fn from(input: BarrierDirections) -> Self {
        Some(u8::from(input))
    }
}
impl From<BarrierDirections> for u16 {
    fn from(input: BarrierDirections) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BarrierDirections> for Option<u16> {
    fn from(input: BarrierDirections) -> Self {
        Some(u16::from(input))
    }
}
impl From<BarrierDirections> for u32 {
    fn from(input: BarrierDirections) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BarrierDirections> for Option<u32> {
    fn from(input: BarrierDirections) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BarrierDirections {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BarrierDirections::PositiveX),
            2 => Ok(BarrierDirections::PositiveY),
            4 => Ok(BarrierDirections::NegativeX),
            8 => Ok(BarrierDirections::NegativeY),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for BarrierDirections {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for BarrierDirections {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(BarrierDirections, u8);

/// Opcode for the CreatePointerBarrier request
pub const CREATE_POINTER_BARRIER_REQUEST: u8 = 31;
pub fn create_pointer_barrier<'c, Conn>(conn: &'c Conn, barrier: Barrier, window: xproto::Window, x1: u16, y1: u16, x2: u16, y2: u16, directions: u32, devices: &[u16]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28 + 2 * devices.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let barrier_bytes = barrier.serialize();
    let window_bytes = window.serialize();
    let x1_bytes = x1.serialize();
    let y1_bytes = y1.serialize();
    let x2_bytes = x2.serialize();
    let y2_bytes = y2.serialize();
    let directions_bytes = directions.serialize();
    let num_devices: u16 = devices.len().try_into()?;
    let num_devices_bytes = num_devices.serialize();
    let devices_bytes = devices.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_POINTER_BARRIER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        barrier_bytes[0],
        barrier_bytes[1],
        barrier_bytes[2],
        barrier_bytes[3],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        x1_bytes[0],
        x1_bytes[1],
        y1_bytes[0],
        y1_bytes[1],
        x2_bytes[0],
        x2_bytes[1],
        y2_bytes[0],
        y2_bytes[1],
        directions_bytes[0],
        directions_bytes[1],
        directions_bytes[2],
        directions_bytes[3],
        0,
        0,
        num_devices_bytes[0],
        num_devices_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&devices_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&devices_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeletePointerBarrier request
pub const DELETE_POINTER_BARRIER_REQUEST: u8 = 32;
pub fn delete_pointer_barrier<Conn>(conn: &Conn, barrier: Barrier) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let barrier_bytes = barrier.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_POINTER_BARRIER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        barrier_bytes[0],
        barrier_bytes[1],
        barrier_bytes[2],
        barrier_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xfixes_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }

    fn xfixes_change_save_set<A, B, C>(&self, mode: A, target: B, map: C, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>, B: Into<u8>, C: Into<u8>
    {
        change_save_set(self, mode, target, map, window)
    }

    fn xfixes_select_selection_input(&self, window: xproto::Window, selection: xproto::Atom, event_mask: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_selection_input(self, window, selection, event_mask)
    }

    fn xfixes_select_cursor_input(&self, window: xproto::Window, event_mask: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_cursor_input(self, window, event_mask)
    }

    fn xfixes_get_cursor_image(&self) -> Result<Cookie<'_, Self, GetCursorImageReply>, ConnectionError>
    {
        get_cursor_image(self)
    }

    fn xfixes_create_region<'c>(&'c self, region: Region, rectangles: &[xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_region(self, region, rectangles)
    }

    fn xfixes_create_region_from_bitmap(&self, region: Region, bitmap: xproto::Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_bitmap(self, region, bitmap)
    }

    fn xfixes_create_region_from_window<A>(&self, region: Region, window: xproto::Window, kind: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<shape::Kind>
    {
        create_region_from_window(self, region, window, kind)
    }

    fn xfixes_create_region_from_gc(&self, region: Region, gc: xproto::Gcontext) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_gc(self, region, gc)
    }

    fn xfixes_create_region_from_picture(&self, region: Region, picture: render::Picture) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_picture(self, region, picture)
    }

    fn xfixes_destroy_region(&self, region: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_region(self, region)
    }

    fn xfixes_set_region<'c>(&'c self, region: Region, rectangles: &[xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_region(self, region, rectangles)
    }

    fn xfixes_copy_region(&self, source: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        copy_region(self, source, destination)
    }

    fn xfixes_union_region(&self, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        union_region(self, source1, source2, destination)
    }

    fn xfixes_intersect_region(&self, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        intersect_region(self, source1, source2, destination)
    }

    fn xfixes_subtract_region(&self, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        subtract_region(self, source1, source2, destination)
    }

    fn xfixes_invert_region(&self, source: Region, bounds: xproto::Rectangle, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        invert_region(self, source, bounds, destination)
    }

    fn xfixes_translate_region(&self, region: Region, dx: i16, dy: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        translate_region(self, region, dx, dy)
    }

    fn xfixes_region_extents(&self, source: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        region_extents(self, source, destination)
    }

    fn xfixes_fetch_region(&self, region: Region) -> Result<Cookie<'_, Self, FetchRegionReply>, ConnectionError>
    {
        fetch_region(self, region)
    }

    fn xfixes_set_gc_clip_region(&self, gc: xproto::Gcontext, region: Region, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_gc_clip_region(self, gc, region, x_origin, y_origin)
    }

    fn xfixes_set_window_shape_region<A>(&self, dest: xproto::Window, dest_kind: A, x_offset: i16, y_offset: i16, region: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<shape::Kind>
    {
        set_window_shape_region(self, dest, dest_kind, x_offset, y_offset, region)
    }

    fn xfixes_set_picture_clip_region(&self, picture: render::Picture, region: Region, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_picture_clip_region(self, picture, region, x_origin, y_origin)
    }

    fn xfixes_set_cursor_name<'c>(&'c self, cursor: xproto::Cursor, name: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_cursor_name(self, cursor, name)
    }

    fn xfixes_get_cursor_name(&self, cursor: xproto::Cursor) -> Result<Cookie<'_, Self, GetCursorNameReply>, ConnectionError>
    {
        get_cursor_name(self, cursor)
    }

    fn xfixes_get_cursor_image_and_name(&self) -> Result<Cookie<'_, Self, GetCursorImageAndNameReply>, ConnectionError>
    {
        get_cursor_image_and_name(self)
    }

    fn xfixes_change_cursor(&self, source: xproto::Cursor, destination: xproto::Cursor) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        change_cursor(self, source, destination)
    }

    fn xfixes_change_cursor_by_name<'c>(&'c self, src: xproto::Cursor, name: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_cursor_by_name(self, src, name)
    }

    fn xfixes_expand_region(&self, source: Region, destination: Region, left: u16, right: u16, top: u16, bottom: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        expand_region(self, source, destination, left, right, top, bottom)
    }

    fn xfixes_hide_cursor(&self, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        hide_cursor(self, window)
    }

    fn xfixes_show_cursor(&self, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        show_cursor(self, window)
    }

    fn xfixes_create_pointer_barrier<'c>(&'c self, barrier: Barrier, window: xproto::Window, x1: u16, y1: u16, x2: u16, y2: u16, directions: u32, devices: &[u16]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_pointer_barrier(self, barrier, window, x1, y1, x2, y2, directions, devices)
    }

    fn xfixes_delete_pointer_barrier(&self, barrier: Barrier) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_pointer_barrier(self, barrier)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
