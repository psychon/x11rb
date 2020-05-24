// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XFixes` X11 extension.

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
use super::render;
use super::shape;
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XFIXES";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (5, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub client_major_version: u32,
    pub client_minor_version: u32,
}
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
        let client_major_version_bytes = self.client_major_version.serialize();
        let client_minor_version_bytes = self.client_minor_version.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QueryVersionRequest::opcode(),
            0,
            0,
            client_major_version_bytes[0],
            client_major_version_bytes[1],
            client_major_version_bytes[2],
            client_major_version_bytes[3],
            client_minor_version_bytes[0],
            client_minor_version_bytes[1],
            client_minor_version_bytes[2],
            client_minor_version_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn query_version<Conn>(conn: &Conn, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major_version: client_major_version,
        client_minor_version: client_minor_version,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
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
impl From<SaveSetMode> for bool {
    fn from(input: SaveSetMode) -> Self {
        match input {
            SaveSetMode::Insert => false,
            SaveSetMode::Delete => true,
        }
    }
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
impl From<SaveSetTarget> for bool {
    fn from(input: SaveSetTarget) -> Self {
        match input {
            SaveSetTarget::Nearest => false,
            SaveSetTarget::Root => true,
        }
    }
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
impl From<SaveSetMapping> for bool {
    fn from(input: SaveSetMapping) -> Self {
        match input {
            SaveSetMapping::Map => false,
            SaveSetMapping::Unmap => true,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeSaveSetRequest {
    pub mode: SaveSetMode,
    pub target: SaveSetTarget,
    pub map: SaveSetMapping,
    pub window: xproto::Window,
}
impl ChangeSaveSetRequest {
    /// Opcode for the ChangeSaveSet request
    pub const fn opcode() -> u8 { 1 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mode_bytes = u8::from(self.mode).serialize();
        let target_bytes = u8::from(self.target).serialize();
        let map_bytes = u8::from(self.map).serialize();
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ChangeSaveSetRequest::opcode(),
            0,
            0,
            mode_bytes[0],
            target_bytes[0],
            map_bytes[0],
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
}
pub fn change_save_set<Conn>(conn: &Conn, mode: SaveSetMode, target: SaveSetTarget, map: SaveSetMapping, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeSaveSetRequest {
        mode: mode,
        target: target,
        map: map,
        window: window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
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
impl TryParse for SelectionNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl From<&SelectionNotifyEvent> for [u8; 32] {
    fn from(input: &SelectionNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let subtype_bytes = u8::from(input.subtype).serialize();
        let sequence_bytes = input.sequence.serialize();
        let window_bytes = input.window.serialize();
        let owner_bytes = input.owner.serialize();
        let selection_bytes = input.selection.serialize();
        let timestamp_bytes = input.timestamp.serialize();
        let selection_timestamp_bytes = input.selection_timestamp.serialize();
        [
            response_type_bytes[0],
            subtype_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            owner_bytes[0],
            owner_bytes[1],
            owner_bytes[2],
            owner_bytes[3],
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            selection_timestamp_bytes[0],
            selection_timestamp_bytes[1],
            selection_timestamp_bytes[2],
            selection_timestamp_bytes[3],
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
impl From<SelectionNotifyEvent> for [u8; 32] {
    fn from(input: SelectionNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectSelectionInputRequest {
    pub window: xproto::Window,
    pub selection: xproto::Atom,
    pub event_mask: u32,
}
impl SelectSelectionInputRequest {
    /// Opcode for the SelectSelectionInput request
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
        let window_bytes = self.window.serialize();
        let selection_bytes = self.selection.serialize();
        let event_mask_bytes = self.event_mask.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SelectSelectionInputRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn select_selection_input<Conn, A>(conn: &Conn, window: xproto::Window, selection: xproto::Atom, event_mask: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectSelectionInputRequest {
        window: window,
        selection: selection,
        event_mask: event_mask,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
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
impl TryParse for CursorNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl From<&CursorNotifyEvent> for [u8; 32] {
    fn from(input: &CursorNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let subtype_bytes = u8::from(input.subtype).serialize();
        let sequence_bytes = input.sequence.serialize();
        let window_bytes = input.window.serialize();
        let cursor_serial_bytes = input.cursor_serial.serialize();
        let timestamp_bytes = input.timestamp.serialize();
        let name_bytes = input.name.serialize();
        [
            response_type_bytes[0],
            subtype_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            cursor_serial_bytes[0],
            cursor_serial_bytes[1],
            cursor_serial_bytes[2],
            cursor_serial_bytes[3],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
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
impl From<CursorNotifyEvent> for [u8; 32] {
    fn from(input: CursorNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectCursorInputRequest {
    pub window: xproto::Window,
    pub event_mask: u32,
}
impl SelectCursorInputRequest {
    /// Opcode for the SelectCursorInput request
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
        let window_bytes = self.window.serialize();
        let event_mask_bytes = self.event_mask.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SelectCursorInputRequest::opcode(),
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
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
}
pub fn select_cursor_input<Conn, A>(conn: &Conn, window: xproto::Window, event_mask: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectCursorInputRequest {
        window: window,
        event_mask: event_mask,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCursorImageRequest;
impl GetCursorImageRequest {
    /// Opcode for the GetCursorImage request
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
        let mut request0 = vec![
            extension_information.major_opcode,
            GetCursorImageRequest::opcode(),
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
pub fn get_cursor_image<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetCursorImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCursorImageRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
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
impl TryParse for GetCursorImageReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
        let (cursor_image, remaining) = crate::x11_utils::parse_list::<u32>(remaining, u32::from(width).checked_mul(u32::from(height)).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
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
impl TryParse for BadRegionError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl From<&BadRegionError> for [u8; 32] {
    fn from(input: &BadRegionError) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let error_code_bytes = input.error_code.serialize();
        let sequence_bytes = input.sequence.serialize();
        [
            response_type_bytes[0],
            error_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
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
            0,
            0,
            0,
            0,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateRegionRequest<'input> {
    pub region: Region,
    pub rectangles: &'input [xproto::Rectangle],
}
impl<'input> CreateRegionRequest<'input> {
    /// Opcode for the CreateRegion request
    pub const fn opcode() -> u8 { 5 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateRegionRequest::opcode(),
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
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
pub fn create_region<'c, 'input, Conn>(conn: &'c Conn, region: Region, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionRequest {
        region: region,
        rectangles: rectangles,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateRegionFromBitmapRequest {
    pub region: Region,
    pub bitmap: xproto::Pixmap,
}
impl CreateRegionFromBitmapRequest {
    /// Opcode for the CreateRegionFromBitmap request
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
        let region_bytes = self.region.serialize();
        let bitmap_bytes = self.bitmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateRegionFromBitmapRequest::opcode(),
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
            bitmap_bytes[0],
            bitmap_bytes[1],
            bitmap_bytes[2],
            bitmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_region_from_bitmap<Conn>(conn: &Conn, region: Region, bitmap: xproto::Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromBitmapRequest {
        region: region,
        bitmap: bitmap,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateRegionFromWindowRequest {
    pub region: Region,
    pub window: xproto::Window,
    pub kind: shape::SK,
}
impl CreateRegionFromWindowRequest {
    /// Opcode for the CreateRegionFromWindow request
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
        let region_bytes = self.region.serialize();
        let window_bytes = self.window.serialize();
        let kind_bytes = shape::Kind::from(self.kind).serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateRegionFromWindowRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_region_from_window<Conn>(conn: &Conn, region: Region, window: xproto::Window, kind: shape::SK) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromWindowRequest {
        region: region,
        window: window,
        kind: kind,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateRegionFromGCRequest {
    pub region: Region,
    pub gc: xproto::Gcontext,
}
impl CreateRegionFromGCRequest {
    /// Opcode for the CreateRegionFromGC request
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
        let region_bytes = self.region.serialize();
        let gc_bytes = self.gc.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateRegionFromGCRequest::opcode(),
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
            gc_bytes[0],
            gc_bytes[1],
            gc_bytes[2],
            gc_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_region_from_gc<Conn>(conn: &Conn, region: Region, gc: xproto::Gcontext) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromGCRequest {
        region: region,
        gc: gc,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateRegionFromPictureRequest {
    pub region: Region,
    pub picture: render::Picture,
}
impl CreateRegionFromPictureRequest {
    /// Opcode for the CreateRegionFromPicture request
    pub const fn opcode() -> u8 { 9 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let picture_bytes = self.picture.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateRegionFromPictureRequest::opcode(),
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
            picture_bytes[0],
            picture_bytes[1],
            picture_bytes[2],
            picture_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_region_from_picture<Conn>(conn: &Conn, region: Region, picture: render::Picture) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromPictureRequest {
        region: region,
        picture: picture,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyRegionRequest {
    pub region: Region,
}
impl DestroyRegionRequest {
    /// Opcode for the DestroyRegion request
    pub const fn opcode() -> u8 { 10 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DestroyRegionRequest::opcode(),
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn destroy_region<Conn>(conn: &Conn, region: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyRegionRequest {
        region: region,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetRegionRequest<'input> {
    pub region: Region,
    pub rectangles: &'input [xproto::Rectangle],
}
impl<'input> SetRegionRequest<'input> {
    /// Opcode for the SetRegion request
    pub const fn opcode() -> u8 { 11 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SetRegionRequest::opcode(),
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
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
pub fn set_region<'c, 'input, Conn>(conn: &'c Conn, region: Region, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetRegionRequest {
        region: region,
        rectangles: rectangles,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyRegionRequest {
    pub source: Region,
    pub destination: Region,
}
impl CopyRegionRequest {
    /// Opcode for the CopyRegion request
    pub const fn opcode() -> u8 { 12 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CopyRegionRequest::opcode(),
            0,
            0,
            source_bytes[0],
            source_bytes[1],
            source_bytes[2],
            source_bytes[3],
            destination_bytes[0],
            destination_bytes[1],
            destination_bytes[2],
            destination_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn copy_region<Conn>(conn: &Conn, source: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CopyRegionRequest {
        source: source,
        destination: destination,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnionRegionRequest {
    pub source1: Region,
    pub source2: Region,
    pub destination: Region,
}
impl UnionRegionRequest {
    /// Opcode for the UnionRegion request
    pub const fn opcode() -> u8 { 13 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let source1_bytes = self.source1.serialize();
        let source2_bytes = self.source2.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            UnionRegionRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn union_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UnionRegionRequest {
        source1: source1,
        source2: source2,
        destination: destination,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntersectRegionRequest {
    pub source1: Region,
    pub source2: Region,
    pub destination: Region,
}
impl IntersectRegionRequest {
    /// Opcode for the IntersectRegion request
    pub const fn opcode() -> u8 { 14 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let source1_bytes = self.source1.serialize();
        let source2_bytes = self.source2.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            IntersectRegionRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn intersect_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IntersectRegionRequest {
        source1: source1,
        source2: source2,
        destination: destination,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubtractRegionRequest {
    pub source1: Region,
    pub source2: Region,
    pub destination: Region,
}
impl SubtractRegionRequest {
    /// Opcode for the SubtractRegion request
    pub const fn opcode() -> u8 { 15 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let source1_bytes = self.source1.serialize();
        let source2_bytes = self.source2.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SubtractRegionRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn subtract_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SubtractRegionRequest {
        source1: source1,
        source2: source2,
        destination: destination,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvertRegionRequest {
    pub source: Region,
    pub bounds: xproto::Rectangle,
    pub destination: Region,
}
impl InvertRegionRequest {
    /// Opcode for the InvertRegion request
    pub const fn opcode() -> u8 { 16 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let bounds_bytes = self.bounds.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            InvertRegionRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn invert_region<Conn>(conn: &Conn, source: Region, bounds: xproto::Rectangle, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InvertRegionRequest {
        source: source,
        bounds: bounds,
        destination: destination,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TranslateRegionRequest {
    pub region: Region,
    pub dx: i16,
    pub dy: i16,
}
impl TranslateRegionRequest {
    /// Opcode for the TranslateRegion request
    pub const fn opcode() -> u8 { 17 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let dx_bytes = self.dx.serialize();
        let dy_bytes = self.dy.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            TranslateRegionRequest::opcode(),
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
            dx_bytes[0],
            dx_bytes[1],
            dy_bytes[0],
            dy_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn translate_region<Conn>(conn: &Conn, region: Region, dx: i16, dy: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TranslateRegionRequest {
        region: region,
        dx: dx,
        dy: dy,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionExtentsRequest {
    pub source: Region,
    pub destination: Region,
}
impl RegionExtentsRequest {
    /// Opcode for the RegionExtents request
    pub const fn opcode() -> u8 { 18 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            RegionExtentsRequest::opcode(),
            0,
            0,
            source_bytes[0],
            source_bytes[1],
            source_bytes[2],
            source_bytes[3],
            destination_bytes[0],
            destination_bytes[1],
            destination_bytes[2],
            destination_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn region_extents<Conn>(conn: &Conn, source: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RegionExtentsRequest {
        source: source,
        destination: destination,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FetchRegionRequest {
    pub region: Region,
}
impl FetchRegionRequest {
    /// Opcode for the FetchRegion request
    pub const fn opcode() -> u8 { 19 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            FetchRegionRequest::opcode(),
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn fetch_region<Conn>(conn: &Conn, region: Region) -> Result<Cookie<'_, Conn, FetchRegionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FetchRegionRequest {
        region: region,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchRegionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub extents: xproto::Rectangle,
    pub rectangles: Vec<xproto::Rectangle>,
}
impl TryParse for FetchRegionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (extents, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (rectangles, remaining) = crate::x11_utils::parse_list::<xproto::Rectangle>(remaining, length.checked_div(2u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
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
impl FetchRegionReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `rectangles` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.rectangles.len()
            .checked_mul(2).unwrap()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetGCClipRegionRequest {
    pub gc: xproto::Gcontext,
    pub region: Region,
    pub x_origin: i16,
    pub y_origin: i16,
}
impl SetGCClipRegionRequest {
    /// Opcode for the SetGCClipRegion request
    pub const fn opcode() -> u8 { 20 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let gc_bytes = self.gc.serialize();
        let region_bytes = self.region.serialize();
        let x_origin_bytes = self.x_origin.serialize();
        let y_origin_bytes = self.y_origin.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SetGCClipRegionRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn set_gc_clip_region<Conn, A>(conn: &Conn, gc: xproto::Gcontext, region: A, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetGCClipRegionRequest {
        gc: gc,
        region: region,
        x_origin: x_origin,
        y_origin: y_origin,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetWindowShapeRegionRequest {
    pub dest: xproto::Window,
    pub dest_kind: shape::SK,
    pub x_offset: i16,
    pub y_offset: i16,
    pub region: Region,
}
impl SetWindowShapeRegionRequest {
    /// Opcode for the SetWindowShapeRegion request
    pub const fn opcode() -> u8 { 21 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let dest_bytes = self.dest.serialize();
        let dest_kind_bytes = shape::Kind::from(self.dest_kind).serialize();
        let x_offset_bytes = self.x_offset.serialize();
        let y_offset_bytes = self.y_offset.serialize();
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SetWindowShapeRegionRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn set_window_shape_region<Conn, A>(conn: &Conn, dest: xproto::Window, dest_kind: shape::SK, x_offset: i16, y_offset: i16, region: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetWindowShapeRegionRequest {
        dest: dest,
        dest_kind: dest_kind,
        x_offset: x_offset,
        y_offset: y_offset,
        region: region,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPictureClipRegionRequest {
    pub picture: render::Picture,
    pub region: Region,
    pub x_origin: i16,
    pub y_origin: i16,
}
impl SetPictureClipRegionRequest {
    /// Opcode for the SetPictureClipRegion request
    pub const fn opcode() -> u8 { 22 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let picture_bytes = self.picture.serialize();
        let region_bytes = self.region.serialize();
        let x_origin_bytes = self.x_origin.serialize();
        let y_origin_bytes = self.y_origin.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SetPictureClipRegionRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn set_picture_clip_region<Conn, A>(conn: &Conn, picture: render::Picture, region: A, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetPictureClipRegionRequest {
        picture: picture,
        region: region,
        x_origin: x_origin,
        y_origin: y_origin,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCursorNameRequest<'input> {
    pub cursor: xproto::Cursor,
    pub name: &'input [u8],
}
impl<'input> SetCursorNameRequest<'input> {
    /// Opcode for the SetCursorName request
    pub const fn opcode() -> u8 { 23 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let cursor_bytes = self.cursor.serialize();
        let nbytes = u16::try_from(self.name.len()).expect("`name` has too many elements");
        let nbytes_bytes = nbytes.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SetCursorNameRequest::opcode(),
            0,
            0,
            cursor_bytes[0],
            cursor_bytes[1],
            cursor_bytes[2],
            cursor_bytes[3],
            nbytes_bytes[0],
            nbytes_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.name[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.name[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_cursor_name<'c, 'input, Conn>(conn: &'c Conn, cursor: xproto::Cursor, name: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetCursorNameRequest {
        cursor: cursor,
        name: name,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCursorNameRequest {
    pub cursor: xproto::Cursor,
}
impl GetCursorNameRequest {
    /// Opcode for the GetCursorName request
    pub const fn opcode() -> u8 { 24 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let cursor_bytes = self.cursor.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetCursorNameRequest::opcode(),
            0,
            0,
            cursor_bytes[0],
            cursor_bytes[1],
            cursor_bytes[2],
            cursor_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_cursor_name<Conn>(conn: &Conn, cursor: xproto::Cursor) -> Result<Cookie<'_, Conn, GetCursorNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCursorNameRequest {
        cursor: cursor,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCursorNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xproto::Atom,
    pub name: Vec<u8>,
}
impl TryParse for GetCursorNameReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (atom, remaining) = xproto::Atom::try_parse(remaining)?;
        let (nbytes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, nbytes.try_into().or(Err(ParseError::ParseError))?)?;
        let name = name.to_vec();
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
impl GetCursorNameReply {
    /// Get the value of the `nbytes` field.
    ///
    /// The `nbytes` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn nbytes(&self) -> u16 {
        self.name.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCursorImageAndNameRequest;
impl GetCursorImageAndNameRequest {
    /// Opcode for the GetCursorImageAndName request
    pub const fn opcode() -> u8 { 25 }
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
            GetCursorImageAndNameRequest::opcode(),
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
pub fn get_cursor_image_and_name<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetCursorImageAndNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCursorImageAndNameRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
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
impl TryParse for GetCursorImageAndNameReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
        let (cursor_image, remaining) = crate::x11_utils::parse_list::<u32>(remaining, u32::from(width).checked_mul(u32::from(height)).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, nbytes.try_into().or(Err(ParseError::ParseError))?)?;
        let name = name.to_vec();
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
impl GetCursorImageAndNameReply {
    /// Get the value of the `nbytes` field.
    ///
    /// The `nbytes` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn nbytes(&self) -> u16 {
        self.name.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeCursorRequest {
    pub source: xproto::Cursor,
    pub destination: xproto::Cursor,
}
impl ChangeCursorRequest {
    /// Opcode for the ChangeCursor request
    pub const fn opcode() -> u8 { 26 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ChangeCursorRequest::opcode(),
            0,
            0,
            source_bytes[0],
            source_bytes[1],
            source_bytes[2],
            source_bytes[3],
            destination_bytes[0],
            destination_bytes[1],
            destination_bytes[2],
            destination_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn change_cursor<Conn>(conn: &Conn, source: xproto::Cursor, destination: xproto::Cursor) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeCursorRequest {
        source: source,
        destination: destination,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeCursorByNameRequest<'input> {
    pub src: xproto::Cursor,
    pub name: &'input [u8],
}
impl<'input> ChangeCursorByNameRequest<'input> {
    /// Opcode for the ChangeCursorByName request
    pub const fn opcode() -> u8 { 27 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let src_bytes = self.src.serialize();
        let nbytes = u16::try_from(self.name.len()).expect("`name` has too many elements");
        let nbytes_bytes = nbytes.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ChangeCursorByNameRequest::opcode(),
            0,
            0,
            src_bytes[0],
            src_bytes[1],
            src_bytes[2],
            src_bytes[3],
            nbytes_bytes[0],
            nbytes_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.name[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.name[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn change_cursor_by_name<'c, 'input, Conn>(conn: &'c Conn, src: xproto::Cursor, name: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeCursorByNameRequest {
        src: src,
        name: name,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpandRegionRequest {
    pub source: Region,
    pub destination: Region,
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}
impl ExpandRegionRequest {
    /// Opcode for the ExpandRegion request
    pub const fn opcode() -> u8 { 28 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let destination_bytes = self.destination.serialize();
        let left_bytes = self.left.serialize();
        let right_bytes = self.right.serialize();
        let top_bytes = self.top.serialize();
        let bottom_bytes = self.bottom.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ExpandRegionRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn expand_region<Conn>(conn: &Conn, source: Region, destination: Region, left: u16, right: u16, top: u16, bottom: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ExpandRegionRequest {
        source: source,
        destination: destination,
        left: left,
        right: right,
        top: top,
        bottom: bottom,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HideCursorRequest {
    pub window: xproto::Window,
}
impl HideCursorRequest {
    /// Opcode for the HideCursor request
    pub const fn opcode() -> u8 { 29 }
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
        let mut request0 = vec![
            extension_information.major_opcode,
            HideCursorRequest::opcode(),
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
}
pub fn hide_cursor<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = HideCursorRequest {
        window: window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShowCursorRequest {
    pub window: xproto::Window,
}
impl ShowCursorRequest {
    /// Opcode for the ShowCursor request
    pub const fn opcode() -> u8 { 30 }
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
        let mut request0 = vec![
            extension_information.major_opcode,
            ShowCursorRequest::opcode(),
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
}
pub fn show_cursor<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ShowCursorRequest {
        window: window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePointerBarrierRequest<'input> {
    pub barrier: Barrier,
    pub window: xproto::Window,
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
    pub directions: u32,
    pub devices: &'input [u16],
}
impl<'input> CreatePointerBarrierRequest<'input> {
    /// Opcode for the CreatePointerBarrier request
    pub const fn opcode() -> u8 { 31 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let barrier_bytes = self.barrier.serialize();
        let window_bytes = self.window.serialize();
        let x1_bytes = self.x1.serialize();
        let y1_bytes = self.y1.serialize();
        let x2_bytes = self.x2.serialize();
        let y2_bytes = self.y2.serialize();
        let directions_bytes = self.directions.serialize();
        let num_devices = u16::try_from(self.devices.len()).expect("`devices` has too many elements");
        let num_devices_bytes = num_devices.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreatePointerBarrierRequest::opcode(),
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let devices_bytes = self.devices.serialize();
        let length_so_far = length_so_far + devices_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), devices_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn create_pointer_barrier<'c, 'input, Conn, A>(conn: &'c Conn, barrier: Barrier, window: xproto::Window, x1: u16, y1: u16, x2: u16, y2: u16, directions: A, devices: &'input [u16]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let directions: u32 = directions.into();
    let request0 = CreatePointerBarrierRequest {
        barrier: barrier,
        window: window,
        x1: x1,
        y1: y1,
        x2: x2,
        y2: y2,
        directions: directions,
        devices: devices,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeletePointerBarrierRequest {
    pub barrier: Barrier,
}
impl DeletePointerBarrierRequest {
    /// Opcode for the DeletePointerBarrier request
    pub const fn opcode() -> u8 { 32 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let barrier_bytes = self.barrier.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DeletePointerBarrierRequest::opcode(),
            0,
            0,
            barrier_bytes[0],
            barrier_bytes[1],
            barrier_bytes[2],
            barrier_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn delete_pointer_barrier<Conn>(conn: &Conn, barrier: Barrier) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeletePointerBarrierRequest {
        barrier: barrier,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xfixes_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }
    fn xfixes_change_save_set(&self, mode: SaveSetMode, target: SaveSetTarget, map: SaveSetMapping, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        change_save_set(self, mode, target, map, window)
    }
    fn xfixes_select_selection_input<A>(&self, window: xproto::Window, selection: xproto::Atom, event_mask: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        select_selection_input(self, window, selection, event_mask)
    }
    fn xfixes_select_cursor_input<A>(&self, window: xproto::Window, event_mask: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        select_cursor_input(self, window, event_mask)
    }
    fn xfixes_get_cursor_image(&self) -> Result<Cookie<'_, Self, GetCursorImageReply>, ConnectionError>
    {
        get_cursor_image(self)
    }
    fn xfixes_create_region<'c, 'input>(&'c self, region: Region, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_region(self, region, rectangles)
    }
    fn xfixes_create_region_from_bitmap(&self, region: Region, bitmap: xproto::Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_bitmap(self, region, bitmap)
    }
    fn xfixes_create_region_from_window(&self, region: Region, window: xproto::Window, kind: shape::SK) -> Result<VoidCookie<'_, Self>, ConnectionError>
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
    fn xfixes_set_region<'c, 'input>(&'c self, region: Region, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn xfixes_set_gc_clip_region<A>(&self, gc: xproto::Gcontext, region: A, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Region>,
    {
        set_gc_clip_region(self, gc, region, x_origin, y_origin)
    }
    fn xfixes_set_window_shape_region<A>(&self, dest: xproto::Window, dest_kind: shape::SK, x_offset: i16, y_offset: i16, region: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Region>,
    {
        set_window_shape_region(self, dest, dest_kind, x_offset, y_offset, region)
    }
    fn xfixes_set_picture_clip_region<A>(&self, picture: render::Picture, region: A, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Region>,
    {
        set_picture_clip_region(self, picture, region, x_origin, y_origin)
    }
    fn xfixes_set_cursor_name<'c, 'input>(&'c self, cursor: xproto::Cursor, name: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn xfixes_change_cursor_by_name<'c, 'input>(&'c self, src: xproto::Cursor, name: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn xfixes_create_pointer_barrier<'c, 'input, A>(&'c self, barrier: Barrier, window: xproto::Window, x1: u16, y1: u16, x2: u16, y2: u16, directions: A, devices: &'input [u16]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        create_pointer_barrier(self, barrier, window, x1, y1, x2, y2, directions, devices)
    }
    fn xfixes_delete_pointer_barrier(&self, barrier: Barrier) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_pointer_barrier(self, barrier)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
