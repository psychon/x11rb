// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XFixes` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use alloc::borrow::Cow;
#[allow(unused_imports)]
use core::convert::TryInto;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
use crate::errors::ParseError;
#[allow(unused_imports)]
use crate::x11_utils::TryIntoUSize;
use crate::{BufWithFds, PiecewiseBuf};
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
#[allow(unused_imports)]
use super::render;
#[allow(unused_imports)]
use super::shape;
#[allow(unused_imports)]
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

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryVersionRequest {
    pub client_major_version: u32,
    pub client_minor_version: u32,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let client_major_version_bytes = self.client_major_version.serialize();
        let client_minor_version_bytes = self.client_minor_version.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_VERSION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (client_major_version, remaining) = u32::try_parse(value)?;
        let (client_minor_version, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            client_major_version,
            client_minor_version,
        })
    }
}
impl Request for QueryVersionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryVersionRequest {
    type Reply = QueryVersionReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u32::try_parse(remaining)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply { sequence, length, major_version, minor_version };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryVersionReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            major_version_bytes[0],
            major_version_bytes[1],
            major_version_bytes[2],
            major_version_bytes[3],
            minor_version_bytes[0],
            minor_version_bytes[1],
            minor_version_bytes[2],
            minor_version_bytes[3],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.major_version.serialize_into(bytes);
        self.minor_version.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
    }
}
#[cfg(test)]
mod query_version_reply {
    #![allow(dead_code, unused_imports)]
    use super::QueryVersionReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryVersionReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let major_version: u32 = GenRandom::generate(rng);
            let minor_version: u32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                major_version,
                minor_version,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(228006558612858880);
        let value = QueryVersionReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SaveSetMode(u8);
impl SaveSetMode {
    pub const INSERT: Self = Self(0);
    pub const DELETE: Self = Self(1);
}
impl From<SaveSetMode> for u8 {
    #[inline]
    fn from(input: SaveSetMode) -> Self {
        input.0
    }
}
impl From<SaveSetMode> for Option<u8> {
    #[inline]
    fn from(input: SaveSetMode) -> Self {
        Some(input.0)
    }
}
impl From<SaveSetMode> for u16 {
    #[inline]
    fn from(input: SaveSetMode) -> Self {
        u16::from(input.0)
    }
}
impl From<SaveSetMode> for Option<u16> {
    #[inline]
    fn from(input: SaveSetMode) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SaveSetMode> for u32 {
    #[inline]
    fn from(input: SaveSetMode) -> Self {
        u32::from(input.0)
    }
}
impl From<SaveSetMode> for Option<u32> {
    #[inline]
    fn from(input: SaveSetMode) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SaveSetMode {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for SaveSetMode  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::INSERT.0.into(), "INSERT", "Insert"),
            (Self::DELETE.0.into(), "DELETE", "Delete"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for SaveSetMode {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::INSERT.0,
            Self::DELETE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SaveSetTarget(u8);
impl SaveSetTarget {
    pub const NEAREST: Self = Self(0);
    pub const ROOT: Self = Self(1);
}
impl From<SaveSetTarget> for u8 {
    #[inline]
    fn from(input: SaveSetTarget) -> Self {
        input.0
    }
}
impl From<SaveSetTarget> for Option<u8> {
    #[inline]
    fn from(input: SaveSetTarget) -> Self {
        Some(input.0)
    }
}
impl From<SaveSetTarget> for u16 {
    #[inline]
    fn from(input: SaveSetTarget) -> Self {
        u16::from(input.0)
    }
}
impl From<SaveSetTarget> for Option<u16> {
    #[inline]
    fn from(input: SaveSetTarget) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SaveSetTarget> for u32 {
    #[inline]
    fn from(input: SaveSetTarget) -> Self {
        u32::from(input.0)
    }
}
impl From<SaveSetTarget> for Option<u32> {
    #[inline]
    fn from(input: SaveSetTarget) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SaveSetTarget {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for SaveSetTarget  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::NEAREST.0.into(), "NEAREST", "Nearest"),
            (Self::ROOT.0.into(), "ROOT", "Root"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for SaveSetTarget {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::NEAREST.0,
            Self::ROOT.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SaveSetMapping(u8);
impl SaveSetMapping {
    pub const MAP: Self = Self(0);
    pub const UNMAP: Self = Self(1);
}
impl From<SaveSetMapping> for u8 {
    #[inline]
    fn from(input: SaveSetMapping) -> Self {
        input.0
    }
}
impl From<SaveSetMapping> for Option<u8> {
    #[inline]
    fn from(input: SaveSetMapping) -> Self {
        Some(input.0)
    }
}
impl From<SaveSetMapping> for u16 {
    #[inline]
    fn from(input: SaveSetMapping) -> Self {
        u16::from(input.0)
    }
}
impl From<SaveSetMapping> for Option<u16> {
    #[inline]
    fn from(input: SaveSetMapping) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SaveSetMapping> for u32 {
    #[inline]
    fn from(input: SaveSetMapping) -> Self {
        u32::from(input.0)
    }
}
impl From<SaveSetMapping> for Option<u32> {
    #[inline]
    fn from(input: SaveSetMapping) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SaveSetMapping {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for SaveSetMapping  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::MAP.0.into(), "MAP", "Map"),
            (Self::UNMAP.0.into(), "UNMAP", "Unmap"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for SaveSetMapping {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::MAP.0,
            Self::UNMAP.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the ChangeSaveSet request
pub const CHANGE_SAVE_SET_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeSaveSetRequest {
    pub mode: SaveSetMode,
    pub target: SaveSetTarget,
    pub map: SaveSetMapping,
    pub window: xproto::Window,
}
impl ChangeSaveSetRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mode_bytes = u8::from(self.mode).serialize();
        let target_bytes = u8::from(self.target).serialize();
        let map_bytes = u8::from(self.map).serialize();
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_SAVE_SET_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_SAVE_SET_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (mode, remaining) = u8::try_parse(value)?;
        let mode = mode.into();
        let (target, remaining) = u8::try_parse(remaining)?;
        let target = target.into();
        let (map, remaining) = u8::try_parse(remaining)?;
        let map = map.into();
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let _ = remaining;
        Ok(ChangeSaveSetRequest {
            mode,
            target,
            map,
            window,
        })
    }
}
impl Request for ChangeSaveSetRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for ChangeSaveSetRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectionEvent(u8);
impl SelectionEvent {
    pub const SET_SELECTION_OWNER: Self = Self(0);
    pub const SELECTION_WINDOW_DESTROY: Self = Self(1);
    pub const SELECTION_CLIENT_CLOSE: Self = Self(2);
}
impl From<SelectionEvent> for u8 {
    #[inline]
    fn from(input: SelectionEvent) -> Self {
        input.0
    }
}
impl From<SelectionEvent> for Option<u8> {
    #[inline]
    fn from(input: SelectionEvent) -> Self {
        Some(input.0)
    }
}
impl From<SelectionEvent> for u16 {
    #[inline]
    fn from(input: SelectionEvent) -> Self {
        u16::from(input.0)
    }
}
impl From<SelectionEvent> for Option<u16> {
    #[inline]
    fn from(input: SelectionEvent) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SelectionEvent> for u32 {
    #[inline]
    fn from(input: SelectionEvent) -> Self {
        u32::from(input.0)
    }
}
impl From<SelectionEvent> for Option<u32> {
    #[inline]
    fn from(input: SelectionEvent) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SelectionEvent {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for SelectionEvent  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::SET_SELECTION_OWNER.0.into(), "SET_SELECTION_OWNER", "SetSelectionOwner"),
            (Self::SELECTION_WINDOW_DESTROY.0.into(), "SELECTION_WINDOW_DESTROY", "SelectionWindowDestroy"),
            (Self::SELECTION_CLIENT_CLOSE.0.into(), "SELECTION_CLIENT_CLOSE", "SelectionClientClose"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for SelectionEvent {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::SET_SELECTION_OWNER.0,
            Self::SELECTION_WINDOW_DESTROY.0,
            Self::SELECTION_CLIENT_CLOSE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectionEventMask(u8);
impl SelectionEventMask {
    pub const SET_SELECTION_OWNER: Self = Self(1 << 0);
    pub const SELECTION_WINDOW_DESTROY: Self = Self(1 << 1);
    pub const SELECTION_CLIENT_CLOSE: Self = Self(1 << 2);
}
impl From<SelectionEventMask> for u8 {
    #[inline]
    fn from(input: SelectionEventMask) -> Self {
        input.0
    }
}
impl From<SelectionEventMask> for Option<u8> {
    #[inline]
    fn from(input: SelectionEventMask) -> Self {
        Some(input.0)
    }
}
impl From<SelectionEventMask> for u16 {
    #[inline]
    fn from(input: SelectionEventMask) -> Self {
        u16::from(input.0)
    }
}
impl From<SelectionEventMask> for Option<u16> {
    #[inline]
    fn from(input: SelectionEventMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SelectionEventMask> for u32 {
    #[inline]
    fn from(input: SelectionEventMask) -> Self {
        u32::from(input.0)
    }
}
impl From<SelectionEventMask> for Option<u32> {
    #[inline]
    fn from(input: SelectionEventMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SelectionEventMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for SelectionEventMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::SET_SELECTION_OWNER.0.into(), "SET_SELECTION_OWNER", "SetSelectionOwner"),
            (Self::SELECTION_WINDOW_DESTROY.0.into(), "SELECTION_WINDOW_DESTROY", "SelectionWindowDestroy"),
            (Self::SELECTION_CLIENT_CLOSE.0.into(), "SELECTION_CLIENT_CLOSE", "SelectionClientClose"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(SelectionEventMask, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for SelectionEventMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::SET_SELECTION_OWNER.0,
            Self::SELECTION_WINDOW_DESTROY.0,
            Self::SELECTION_CLIENT_CLOSE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the SelectionNotify event
pub const SELECTION_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (subtype, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (owner, remaining) = xproto::Window::try_parse(remaining)?;
        let (selection, remaining) = xproto::Atom::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (selection_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let subtype = subtype.into();
        let result = SelectionNotifyEvent { response_type, subtype, sequence, window, owner, selection, timestamp, selection_timestamp };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for SelectionNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let subtype_bytes = u8::from(self.subtype).serialize();
        let sequence_bytes = self.sequence.serialize();
        let window_bytes = self.window.serialize();
        let owner_bytes = self.owner.serialize();
        let selection_bytes = self.selection.serialize();
        let timestamp_bytes = self.timestamp.serialize();
        let selection_timestamp_bytes = self.selection_timestamp.serialize();
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        u8::from(self.subtype).serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.window.serialize_into(bytes);
        self.owner.serialize_into(bytes);
        self.selection.serialize_into(bytes);
        self.timestamp.serialize_into(bytes);
        self.selection_timestamp.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
    }
}
#[cfg(test)]
mod selection_notify_event {
    #![allow(dead_code, unused_imports)]
    use super::SelectionNotifyEvent;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for SelectionNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            let response_type: u8 = GenRandom::generate(rng);
            let subtype = GenRandom::generate(rng);
            let sequence: u16 = GenRandom::generate(rng);
            let window = GenRandom::generate(rng);
            let owner = GenRandom::generate(rng);
            let selection = GenRandom::generate(rng);
            let timestamp = GenRandom::generate(rng);
            let selection_timestamp = GenRandom::generate(rng);
            Self {
                response_type,
                subtype,
                sequence,
                window,
                owner,
                selection,
                timestamp,
                selection_timestamp,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10942616926495842304);
        let value = SelectionNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
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

/// Opcode for the SelectSelectionInput request
pub const SELECT_SELECTION_INPUT_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectSelectionInputRequest {
    pub window: xproto::Window,
    pub selection: xproto::Atom,
    pub event_mask: u32,
}
impl SelectSelectionInputRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let selection_bytes = self.selection.serialize();
        let event_mask_bytes = self.event_mask.serialize();
        let mut request0 = vec![
            major_opcode,
            SELECT_SELECTION_INPUT_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SELECT_SELECTION_INPUT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (selection, remaining) = xproto::Atom::try_parse(remaining)?;
        let (event_mask, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SelectSelectionInputRequest {
            window,
            selection,
            event_mask,
        })
    }
}
impl Request for SelectSelectionInputRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SelectSelectionInputRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CursorNotify(u8);
impl CursorNotify {
    pub const DISPLAY_CURSOR: Self = Self(0);
}
impl From<CursorNotify> for u8 {
    #[inline]
    fn from(input: CursorNotify) -> Self {
        input.0
    }
}
impl From<CursorNotify> for Option<u8> {
    #[inline]
    fn from(input: CursorNotify) -> Self {
        Some(input.0)
    }
}
impl From<CursorNotify> for u16 {
    #[inline]
    fn from(input: CursorNotify) -> Self {
        u16::from(input.0)
    }
}
impl From<CursorNotify> for Option<u16> {
    #[inline]
    fn from(input: CursorNotify) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<CursorNotify> for u32 {
    #[inline]
    fn from(input: CursorNotify) -> Self {
        u32::from(input.0)
    }
}
impl From<CursorNotify> for Option<u32> {
    #[inline]
    fn from(input: CursorNotify) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for CursorNotify {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for CursorNotify  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::DISPLAY_CURSOR.0.into(), "DISPLAY_CURSOR", "DisplayCursor"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for CursorNotify {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::DISPLAY_CURSOR.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CursorNotifyMask(u8);
impl CursorNotifyMask {
    pub const DISPLAY_CURSOR: Self = Self(1 << 0);
}
impl From<CursorNotifyMask> for u8 {
    #[inline]
    fn from(input: CursorNotifyMask) -> Self {
        input.0
    }
}
impl From<CursorNotifyMask> for Option<u8> {
    #[inline]
    fn from(input: CursorNotifyMask) -> Self {
        Some(input.0)
    }
}
impl From<CursorNotifyMask> for u16 {
    #[inline]
    fn from(input: CursorNotifyMask) -> Self {
        u16::from(input.0)
    }
}
impl From<CursorNotifyMask> for Option<u16> {
    #[inline]
    fn from(input: CursorNotifyMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<CursorNotifyMask> for u32 {
    #[inline]
    fn from(input: CursorNotifyMask) -> Self {
        u32::from(input.0)
    }
}
impl From<CursorNotifyMask> for Option<u32> {
    #[inline]
    fn from(input: CursorNotifyMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for CursorNotifyMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for CursorNotifyMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::DISPLAY_CURSOR.0.into(), "DISPLAY_CURSOR", "DisplayCursor"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(CursorNotifyMask, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for CursorNotifyMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::DISPLAY_CURSOR.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the CursorNotify event
pub const CURSOR_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (subtype, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (cursor_serial, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let subtype = subtype.into();
        let result = CursorNotifyEvent { response_type, subtype, sequence, window, cursor_serial, timestamp, name };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for CursorNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let subtype_bytes = u8::from(self.subtype).serialize();
        let sequence_bytes = self.sequence.serialize();
        let window_bytes = self.window.serialize();
        let cursor_serial_bytes = self.cursor_serial.serialize();
        let timestamp_bytes = self.timestamp.serialize();
        let name_bytes = self.name.serialize();
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        u8::from(self.subtype).serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.window.serialize_into(bytes);
        self.cursor_serial.serialize_into(bytes);
        self.timestamp.serialize_into(bytes);
        self.name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
    }
}
#[cfg(test)]
mod cursor_notify_event {
    #![allow(dead_code, unused_imports)]
    use super::CursorNotifyEvent;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for CursorNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            let response_type: u8 = GenRandom::generate(rng);
            let subtype = GenRandom::generate(rng);
            let sequence: u16 = GenRandom::generate(rng);
            let window = GenRandom::generate(rng);
            let cursor_serial: u32 = GenRandom::generate(rng);
            let timestamp = GenRandom::generate(rng);
            let name = GenRandom::generate(rng);
            Self {
                response_type,
                subtype,
                sequence,
                window,
                cursor_serial,
                timestamp,
                name,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14842739939850843136);
        let value = CursorNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
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

/// Opcode for the SelectCursorInput request
pub const SELECT_CURSOR_INPUT_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectCursorInputRequest {
    pub window: xproto::Window,
    pub event_mask: u32,
}
impl SelectCursorInputRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let event_mask_bytes = self.event_mask.serialize();
        let mut request0 = vec![
            major_opcode,
            SELECT_CURSOR_INPUT_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SELECT_CURSOR_INPUT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (event_mask, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SelectCursorInputRequest {
            window,
            event_mask,
        })
    }
}
impl Request for SelectCursorInputRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SelectCursorInputRequest {
}

/// Opcode for the GetCursorImage request
pub const GET_CURSOR_IMAGE_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCursorImageRequest;
impl GetCursorImageRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_CURSOR_IMAGE_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CURSOR_IMAGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetCursorImageRequest
        )
    }
}
impl Request for GetCursorImageRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetCursorImageRequest {
    type Reply = GetCursorImageReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCursorImageReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (xhot, remaining) = u16::try_parse(remaining)?;
        let (yhot, remaining) = u16::try_parse(remaining)?;
        let (cursor_serial, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (cursor_image, remaining) = crate::x11_utils::parse_list::<u32>(remaining, u32::from(width).checked_mul(u32::from(height)).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetCursorImageReply { sequence, length, x, y, width, height, xhot, yhot, cursor_serial, cursor_image };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetCursorImageReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.xhot.serialize_into(bytes);
        self.yhot.serialize_into(bytes);
        self.cursor_serial.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        assert_eq!(self.cursor_image.len(), usize::try_from(u32::from(self.width).checked_mul(u32::from(self.height)).unwrap()).unwrap(), "`cursor_image` has an incorrect length");
        self.cursor_image.serialize_into(bytes);
    }
}
#[cfg(test)]
mod get_cursor_image_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetCursorImageReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetCursorImageReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let x: i16 = GenRandom::generate(rng);
            let y: i16 = GenRandom::generate(rng);
            let width: u16 = GenRandom::generate(rng);
            let height: u16 = GenRandom::generate(rng);
            let xhot: u16 = GenRandom::generate(rng);
            let yhot: u16 = GenRandom::generate(rng);
            let cursor_serial: u32 = GenRandom::generate(rng);
            let cursor_image: Vec<u32> = gen_random_list(rng, usize::try_from(u32::from(width).checked_mul(u32::from(height)).unwrap()).unwrap());
            Self {
                sequence,
                length,
                x,
                y,
                width,
                height,
                xhot,
                yhot,
                cursor_serial,
                cursor_image,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3397547910108862464);
        let value = GetCursorImageReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

pub type Region = u32;

/// Opcode for the BadRegion error
pub const BAD_REGION_ERROR: u8 = 0;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegionEnum(u8);
impl RegionEnum {
    pub const NONE: Self = Self(0);
}
impl From<RegionEnum> for u8 {
    #[inline]
    fn from(input: RegionEnum) -> Self {
        input.0
    }
}
impl From<RegionEnum> for Option<u8> {
    #[inline]
    fn from(input: RegionEnum) -> Self {
        Some(input.0)
    }
}
impl From<RegionEnum> for u16 {
    #[inline]
    fn from(input: RegionEnum) -> Self {
        u16::from(input.0)
    }
}
impl From<RegionEnum> for Option<u16> {
    #[inline]
    fn from(input: RegionEnum) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<RegionEnum> for u32 {
    #[inline]
    fn from(input: RegionEnum) -> Self {
        u32::from(input.0)
    }
}
impl From<RegionEnum> for Option<u32> {
    #[inline]
    fn from(input: RegionEnum) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for RegionEnum {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for RegionEnum  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::NONE.0.into(), "NONE", "None"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for RegionEnum {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::NONE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the CreateRegion request
pub const CREATE_REGION_REQUEST: u8 = 5;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRegionRequest<'input> {
    pub region: Region,
    pub rectangles: Cow<'input, [xproto::Rectangle]>,
}
impl<'input> CreateRegionRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_REGION_REQUEST,
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
        (vec![request0.into(), rectangles_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let mut remaining = remaining;
        // Length is 'everything left in the input'
        let mut rectangles = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = xproto::Rectangle::try_parse(remaining)?;
            remaining = new_remaining;
            rectangles.push(v);
        }
        let _ = remaining;
        Ok(CreateRegionRequest {
            region,
            rectangles: Cow::Owned(rectangles),
        })
    }
    /// Clone all borrowed data in this CreateRegionRequest.
    pub fn into_owned(self) -> CreateRegionRequest<'static> {
        CreateRegionRequest {
            region: self.region,
            rectangles: Cow::Owned(self.rectangles.into_owned()),
        }
    }
}
impl<'input> Request for CreateRegionRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for CreateRegionRequest<'input> {
}

/// Opcode for the CreateRegionFromBitmap request
pub const CREATE_REGION_FROM_BITMAP_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRegionFromBitmapRequest {
    pub region: Region,
    pub bitmap: xproto::Pixmap,
}
impl CreateRegionFromBitmapRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let bitmap_bytes = self.bitmap.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_REGION_FROM_BITMAP_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_REGION_FROM_BITMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let (bitmap, remaining) = xproto::Pixmap::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateRegionFromBitmapRequest {
            region,
            bitmap,
        })
    }
}
impl Request for CreateRegionFromBitmapRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CreateRegionFromBitmapRequest {
}

/// Opcode for the CreateRegionFromWindow request
pub const CREATE_REGION_FROM_WINDOW_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRegionFromWindowRequest {
    pub region: Region,
    pub window: xproto::Window,
    pub kind: shape::SK,
}
impl CreateRegionFromWindowRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let window_bytes = self.window.serialize();
        let kind_bytes = shape::Kind::from(self.kind).serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_REGION_FROM_WINDOW_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_REGION_FROM_WINDOW_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (kind, remaining) = shape::Kind::try_parse(remaining)?;
        let kind = kind.into();
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(CreateRegionFromWindowRequest {
            region,
            window,
            kind,
        })
    }
}
impl Request for CreateRegionFromWindowRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CreateRegionFromWindowRequest {
}

/// Opcode for the CreateRegionFromGC request
pub const CREATE_REGION_FROM_GC_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRegionFromGCRequest {
    pub region: Region,
    pub gc: xproto::Gcontext,
}
impl CreateRegionFromGCRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let gc_bytes = self.gc.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_REGION_FROM_GC_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_REGION_FROM_GC_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let (gc, remaining) = xproto::Gcontext::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateRegionFromGCRequest {
            region,
            gc,
        })
    }
}
impl Request for CreateRegionFromGCRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CreateRegionFromGCRequest {
}

/// Opcode for the CreateRegionFromPicture request
pub const CREATE_REGION_FROM_PICTURE_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRegionFromPictureRequest {
    pub region: Region,
    pub picture: render::Picture,
}
impl CreateRegionFromPictureRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let picture_bytes = self.picture.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_REGION_FROM_PICTURE_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_REGION_FROM_PICTURE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let (picture, remaining) = render::Picture::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateRegionFromPictureRequest {
            region,
            picture,
        })
    }
}
impl Request for CreateRegionFromPictureRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CreateRegionFromPictureRequest {
}

/// Opcode for the DestroyRegion request
pub const DESTROY_REGION_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DestroyRegionRequest {
    pub region: Region,
}
impl DestroyRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            major_opcode,
            DESTROY_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyRegionRequest {
            region,
        })
    }
}
impl Request for DestroyRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DestroyRegionRequest {
}

/// Opcode for the SetRegion request
pub const SET_REGION_REQUEST: u8 = 11;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRegionRequest<'input> {
    pub region: Region,
    pub rectangles: Cow<'input, [xproto::Rectangle]>,
}
impl<'input> SetRegionRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_REGION_REQUEST,
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
        (vec![request0.into(), rectangles_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let mut remaining = remaining;
        // Length is 'everything left in the input'
        let mut rectangles = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = xproto::Rectangle::try_parse(remaining)?;
            remaining = new_remaining;
            rectangles.push(v);
        }
        let _ = remaining;
        Ok(SetRegionRequest {
            region,
            rectangles: Cow::Owned(rectangles),
        })
    }
    /// Clone all borrowed data in this SetRegionRequest.
    pub fn into_owned(self) -> SetRegionRequest<'static> {
        SetRegionRequest {
            region: self.region,
            rectangles: Cow::Owned(self.rectangles.into_owned()),
        }
    }
}
impl<'input> Request for SetRegionRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetRegionRequest<'input> {
}

/// Opcode for the CopyRegion request
pub const COPY_REGION_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CopyRegionRequest {
    pub source: Region,
    pub destination: Region,
}
impl CopyRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            major_opcode,
            COPY_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != COPY_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (source, remaining) = Region::try_parse(value)?;
        let (destination, remaining) = Region::try_parse(remaining)?;
        let _ = remaining;
        Ok(CopyRegionRequest {
            source,
            destination,
        })
    }
}
impl Request for CopyRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CopyRegionRequest {
}

/// Opcode for the UnionRegion request
pub const UNION_REGION_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionRegionRequest {
    pub source1: Region,
    pub source2: Region,
    pub destination: Region,
}
impl UnionRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let source1_bytes = self.source1.serialize();
        let source2_bytes = self.source2.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            major_opcode,
            UNION_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != UNION_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (source1, remaining) = Region::try_parse(value)?;
        let (source2, remaining) = Region::try_parse(remaining)?;
        let (destination, remaining) = Region::try_parse(remaining)?;
        let _ = remaining;
        Ok(UnionRegionRequest {
            source1,
            source2,
            destination,
        })
    }
}
impl Request for UnionRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for UnionRegionRequest {
}

/// Opcode for the IntersectRegion request
pub const INTERSECT_REGION_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntersectRegionRequest {
    pub source1: Region,
    pub source2: Region,
    pub destination: Region,
}
impl IntersectRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let source1_bytes = self.source1.serialize();
        let source2_bytes = self.source2.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            major_opcode,
            INTERSECT_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != INTERSECT_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (source1, remaining) = Region::try_parse(value)?;
        let (source2, remaining) = Region::try_parse(remaining)?;
        let (destination, remaining) = Region::try_parse(remaining)?;
        let _ = remaining;
        Ok(IntersectRegionRequest {
            source1,
            source2,
            destination,
        })
    }
}
impl Request for IntersectRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for IntersectRegionRequest {
}

/// Opcode for the SubtractRegion request
pub const SUBTRACT_REGION_REQUEST: u8 = 15;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubtractRegionRequest {
    pub source1: Region,
    pub source2: Region,
    pub destination: Region,
}
impl SubtractRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let source1_bytes = self.source1.serialize();
        let source2_bytes = self.source2.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            major_opcode,
            SUBTRACT_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SUBTRACT_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (source1, remaining) = Region::try_parse(value)?;
        let (source2, remaining) = Region::try_parse(remaining)?;
        let (destination, remaining) = Region::try_parse(remaining)?;
        let _ = remaining;
        Ok(SubtractRegionRequest {
            source1,
            source2,
            destination,
        })
    }
}
impl Request for SubtractRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SubtractRegionRequest {
}

/// Opcode for the InvertRegion request
pub const INVERT_REGION_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InvertRegionRequest {
    pub source: Region,
    pub bounds: xproto::Rectangle,
    pub destination: Region,
}
impl InvertRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let bounds_bytes = self.bounds.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            major_opcode,
            INVERT_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != INVERT_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (source, remaining) = Region::try_parse(value)?;
        let (bounds, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let (destination, remaining) = Region::try_parse(remaining)?;
        let _ = remaining;
        Ok(InvertRegionRequest {
            source,
            bounds,
            destination,
        })
    }
}
impl Request for InvertRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for InvertRegionRequest {
}

/// Opcode for the TranslateRegion request
pub const TRANSLATE_REGION_REQUEST: u8 = 17;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranslateRegionRequest {
    pub region: Region,
    pub dx: i16,
    pub dy: i16,
}
impl TranslateRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let dx_bytes = self.dx.serialize();
        let dy_bytes = self.dy.serialize();
        let mut request0 = vec![
            major_opcode,
            TRANSLATE_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != TRANSLATE_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let (dx, remaining) = i16::try_parse(remaining)?;
        let (dy, remaining) = i16::try_parse(remaining)?;
        let _ = remaining;
        Ok(TranslateRegionRequest {
            region,
            dx,
            dy,
        })
    }
}
impl Request for TranslateRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for TranslateRegionRequest {
}

/// Opcode for the RegionExtents request
pub const REGION_EXTENTS_REQUEST: u8 = 18;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegionExtentsRequest {
    pub source: Region,
    pub destination: Region,
}
impl RegionExtentsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            major_opcode,
            REGION_EXTENTS_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != REGION_EXTENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (source, remaining) = Region::try_parse(value)?;
        let (destination, remaining) = Region::try_parse(remaining)?;
        let _ = remaining;
        Ok(RegionExtentsRequest {
            source,
            destination,
        })
    }
}
impl Request for RegionExtentsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for RegionExtentsRequest {
}

/// Opcode for the FetchRegion request
pub const FETCH_REGION_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FetchRegionRequest {
    pub region: Region,
}
impl FetchRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            major_opcode,
            FETCH_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != FETCH_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = Region::try_parse(value)?;
        let _ = remaining;
        Ok(FetchRegionRequest {
            region,
        })
    }
}
impl Request for FetchRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for FetchRegionRequest {
    type Reply = FetchRegionReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FetchRegionReply {
    pub sequence: u16,
    pub extents: xproto::Rectangle,
    pub rectangles: Vec<xproto::Rectangle>,
}
impl TryParse for FetchRegionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (extents, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (rectangles, remaining) = crate::x11_utils::parse_list::<xproto::Rectangle>(remaining, length.checked_div(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = FetchRegionReply { sequence, extents, rectangles };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for FetchRegionReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        let length = u32::try_from(self.rectangles.len()).ok().and_then(|len| len.checked_mul(2)).expect("`rectangles` has too many elements");
        length.serialize_into(bytes);
        self.extents.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
        self.rectangles.serialize_into(bytes);
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
#[cfg(test)]
mod fetch_region_reply {
    #![allow(dead_code, unused_imports)]
    use super::FetchRegionReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for FetchRegionReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let extents = GenRandom::generate(rng);
            let rectangles = gen_random_list(rng, usize::try_from(length.checked_div(2u32).unwrap()).unwrap());
            Self {
                sequence,
                extents,
                rectangles,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12239822329583730688);
        let value = FetchRegionReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetGCClipRegion request
pub const SET_GC_CLIP_REGION_REQUEST: u8 = 20;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetGCClipRegionRequest {
    pub gc: xproto::Gcontext,
    pub region: Region,
    pub x_origin: i16,
    pub y_origin: i16,
}
impl SetGCClipRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let gc_bytes = self.gc.serialize();
        let region_bytes = self.region.serialize();
        let x_origin_bytes = self.x_origin.serialize();
        let y_origin_bytes = self.y_origin.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_GC_CLIP_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_GC_CLIP_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (gc, remaining) = xproto::Gcontext::try_parse(value)?;
        let (region, remaining) = Region::try_parse(remaining)?;
        let (x_origin, remaining) = i16::try_parse(remaining)?;
        let (y_origin, remaining) = i16::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetGCClipRegionRequest {
            gc,
            region,
            x_origin,
            y_origin,
        })
    }
}
impl Request for SetGCClipRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SetGCClipRegionRequest {
}

/// Opcode for the SetWindowShapeRegion request
pub const SET_WINDOW_SHAPE_REGION_REQUEST: u8 = 21;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetWindowShapeRegionRequest {
    pub dest: xproto::Window,
    pub dest_kind: shape::SK,
    pub x_offset: i16,
    pub y_offset: i16,
    pub region: Region,
}
impl SetWindowShapeRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let dest_bytes = self.dest.serialize();
        let dest_kind_bytes = shape::Kind::from(self.dest_kind).serialize();
        let x_offset_bytes = self.x_offset.serialize();
        let y_offset_bytes = self.y_offset.serialize();
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_WINDOW_SHAPE_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_WINDOW_SHAPE_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (dest, remaining) = xproto::Window::try_parse(value)?;
        let (dest_kind, remaining) = shape::Kind::try_parse(remaining)?;
        let dest_kind = dest_kind.into();
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (x_offset, remaining) = i16::try_parse(remaining)?;
        let (y_offset, remaining) = i16::try_parse(remaining)?;
        let (region, remaining) = Region::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetWindowShapeRegionRequest {
            dest,
            dest_kind,
            x_offset,
            y_offset,
            region,
        })
    }
}
impl Request for SetWindowShapeRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SetWindowShapeRegionRequest {
}

/// Opcode for the SetPictureClipRegion request
pub const SET_PICTURE_CLIP_REGION_REQUEST: u8 = 22;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPictureClipRegionRequest {
    pub picture: render::Picture,
    pub region: Region,
    pub x_origin: i16,
    pub y_origin: i16,
}
impl SetPictureClipRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let picture_bytes = self.picture.serialize();
        let region_bytes = self.region.serialize();
        let x_origin_bytes = self.x_origin.serialize();
        let y_origin_bytes = self.y_origin.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_PICTURE_CLIP_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_PICTURE_CLIP_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (picture, remaining) = render::Picture::try_parse(value)?;
        let (region, remaining) = Region::try_parse(remaining)?;
        let (x_origin, remaining) = i16::try_parse(remaining)?;
        let (y_origin, remaining) = i16::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetPictureClipRegionRequest {
            picture,
            region,
            x_origin,
            y_origin,
        })
    }
}
impl Request for SetPictureClipRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SetPictureClipRegionRequest {
}

/// Opcode for the SetCursorName request
pub const SET_CURSOR_NAME_REQUEST: u8 = 23;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetCursorNameRequest<'input> {
    pub cursor: xproto::Cursor,
    pub name: Cow<'input, [u8]>,
}
impl<'input> SetCursorNameRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let cursor_bytes = self.cursor.serialize();
        let nbytes = u16::try_from(self.name.len()).expect("`name` has too many elements");
        let nbytes_bytes = nbytes.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_CURSOR_NAME_REQUEST,
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
        let length_so_far = length_so_far + self.name.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.name, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_CURSOR_NAME_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (cursor, remaining) = xproto::Cursor::try_parse(value)?;
        let (nbytes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, nbytes.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetCursorNameRequest {
            cursor,
            name: Cow::Borrowed(name),
        })
    }
    /// Clone all borrowed data in this SetCursorNameRequest.
    pub fn into_owned(self) -> SetCursorNameRequest<'static> {
        SetCursorNameRequest {
            cursor: self.cursor,
            name: Cow::Owned(self.name.into_owned()),
        }
    }
}
impl<'input> Request for SetCursorNameRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetCursorNameRequest<'input> {
}

/// Opcode for the GetCursorName request
pub const GET_CURSOR_NAME_REQUEST: u8 = 24;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCursorNameRequest {
    pub cursor: xproto::Cursor,
}
impl GetCursorNameRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let cursor_bytes = self.cursor.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_CURSOR_NAME_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CURSOR_NAME_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (cursor, remaining) = xproto::Cursor::try_parse(value)?;
        let _ = remaining;
        Ok(GetCursorNameRequest {
            cursor,
        })
    }
}
impl Request for GetCursorNameRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetCursorNameRequest {
    type Reply = GetCursorNameReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCursorNameReply {
    pub sequence: u16,
    pub length: u32,
    pub atom: xproto::Atom,
    pub name: Vec<u8>,
}
impl TryParse for GetCursorNameReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (atom, remaining) = xproto::Atom::try_parse(remaining)?;
        let (nbytes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::InsufficientData)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, nbytes.try_to_usize()?)?;
        let name = name.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetCursorNameReply { sequence, length, atom, name };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetCursorNameReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.atom.serialize_into(bytes);
        let nbytes = u16::try_from(self.name.len()).expect("`name` has too many elements");
        nbytes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 18]);
        bytes.extend_from_slice(&self.name);
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
#[cfg(test)]
mod get_cursor_name_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetCursorNameReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetCursorNameReply {
        fn generate(rng: &Rng) -> Self {
            let nbytes = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let atom = GenRandom::generate(rng);
            let name: Vec<u8> = gen_random_list(rng, usize::try_from(nbytes).unwrap());
            Self {
                sequence,
                length,
                atom,
                name,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(148099343580147712);
        let value = GetCursorNameReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetCursorImageAndName request
pub const GET_CURSOR_IMAGE_AND_NAME_REQUEST: u8 = 25;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCursorImageAndNameRequest;
impl GetCursorImageAndNameRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_CURSOR_IMAGE_AND_NAME_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CURSOR_IMAGE_AND_NAME_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetCursorImageAndNameRequest
        )
    }
}
impl Request for GetCursorImageAndNameRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetCursorImageAndNameRequest {
    type Reply = GetCursorImageAndNameReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCursorImageAndNameReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (cursor_image, remaining) = crate::x11_utils::parse_list::<u32>(remaining, u32::from(width).checked_mul(u32::from(height)).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, nbytes.try_to_usize()?)?;
        let name = name.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetCursorImageAndNameReply { sequence, length, x, y, width, height, xhot, yhot, cursor_serial, cursor_atom, cursor_image, name };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetCursorImageAndNameReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.xhot.serialize_into(bytes);
        self.yhot.serialize_into(bytes);
        self.cursor_serial.serialize_into(bytes);
        self.cursor_atom.serialize_into(bytes);
        let nbytes = u16::try_from(self.name.len()).expect("`name` has too many elements");
        nbytes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        assert_eq!(self.cursor_image.len(), usize::try_from(u32::from(self.width).checked_mul(u32::from(self.height)).unwrap()).unwrap(), "`cursor_image` has an incorrect length");
        self.cursor_image.serialize_into(bytes);
        bytes.extend_from_slice(&self.name);
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
#[cfg(test)]
mod get_cursor_image_and_name_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetCursorImageAndNameReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetCursorImageAndNameReply {
        fn generate(rng: &Rng) -> Self {
            let nbytes = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let x: i16 = GenRandom::generate(rng);
            let y: i16 = GenRandom::generate(rng);
            let width: u16 = GenRandom::generate(rng);
            let height: u16 = GenRandom::generate(rng);
            let xhot: u16 = GenRandom::generate(rng);
            let yhot: u16 = GenRandom::generate(rng);
            let cursor_serial: u32 = GenRandom::generate(rng);
            let cursor_atom = GenRandom::generate(rng);
            let cursor_image: Vec<u32> = gen_random_list(rng, usize::try_from(u32::from(width).checked_mul(u32::from(height)).unwrap()).unwrap());
            let name: Vec<u8> = gen_random_list(rng, usize::try_from(nbytes).unwrap());
            Self {
                sequence,
                length,
                x,
                y,
                width,
                height,
                xhot,
                yhot,
                cursor_serial,
                cursor_atom,
                cursor_image,
                name,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1092039385405292544);
        let value = GetCursorImageAndNameReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ChangeCursor request
pub const CHANGE_CURSOR_REQUEST: u8 = 26;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeCursorRequest {
    pub source: xproto::Cursor,
    pub destination: xproto::Cursor,
}
impl ChangeCursorRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let destination_bytes = self.destination.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_CURSOR_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_CURSOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (source, remaining) = xproto::Cursor::try_parse(value)?;
        let (destination, remaining) = xproto::Cursor::try_parse(remaining)?;
        let _ = remaining;
        Ok(ChangeCursorRequest {
            source,
            destination,
        })
    }
}
impl Request for ChangeCursorRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for ChangeCursorRequest {
}

/// Opcode for the ChangeCursorByName request
pub const CHANGE_CURSOR_BY_NAME_REQUEST: u8 = 27;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeCursorByNameRequest<'input> {
    pub src: xproto::Cursor,
    pub name: Cow<'input, [u8]>,
}
impl<'input> ChangeCursorByNameRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let src_bytes = self.src.serialize();
        let nbytes = u16::try_from(self.name.len()).expect("`name` has too many elements");
        let nbytes_bytes = nbytes.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_CURSOR_BY_NAME_REQUEST,
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
        let length_so_far = length_so_far + self.name.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.name, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_CURSOR_BY_NAME_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (src, remaining) = xproto::Cursor::try_parse(value)?;
        let (nbytes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, nbytes.try_to_usize()?)?;
        let _ = remaining;
        Ok(ChangeCursorByNameRequest {
            src,
            name: Cow::Borrowed(name),
        })
    }
    /// Clone all borrowed data in this ChangeCursorByNameRequest.
    pub fn into_owned(self) -> ChangeCursorByNameRequest<'static> {
        ChangeCursorByNameRequest {
            src: self.src,
            name: Cow::Owned(self.name.into_owned()),
        }
    }
}
impl<'input> Request for ChangeCursorByNameRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for ChangeCursorByNameRequest<'input> {
}

/// Opcode for the ExpandRegion request
pub const EXPAND_REGION_REQUEST: u8 = 28;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpandRegionRequest {
    pub source: Region,
    pub destination: Region,
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}
impl ExpandRegionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let source_bytes = self.source.serialize();
        let destination_bytes = self.destination.serialize();
        let left_bytes = self.left.serialize();
        let right_bytes = self.right.serialize();
        let top_bytes = self.top.serialize();
        let bottom_bytes = self.bottom.serialize();
        let mut request0 = vec![
            major_opcode,
            EXPAND_REGION_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != EXPAND_REGION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (source, remaining) = Region::try_parse(value)?;
        let (destination, remaining) = Region::try_parse(remaining)?;
        let (left, remaining) = u16::try_parse(remaining)?;
        let (right, remaining) = u16::try_parse(remaining)?;
        let (top, remaining) = u16::try_parse(remaining)?;
        let (bottom, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(ExpandRegionRequest {
            source,
            destination,
            left,
            right,
            top,
            bottom,
        })
    }
}
impl Request for ExpandRegionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for ExpandRegionRequest {
}

/// Opcode for the HideCursor request
pub const HIDE_CURSOR_REQUEST: u8 = 29;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HideCursorRequest {
    pub window: xproto::Window,
}
impl HideCursorRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            HIDE_CURSOR_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != HIDE_CURSOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(HideCursorRequest {
            window,
        })
    }
}
impl Request for HideCursorRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for HideCursorRequest {
}

/// Opcode for the ShowCursor request
pub const SHOW_CURSOR_REQUEST: u8 = 30;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShowCursorRequest {
    pub window: xproto::Window,
}
impl ShowCursorRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            SHOW_CURSOR_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SHOW_CURSOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(ShowCursorRequest {
            window,
        })
    }
}
impl Request for ShowCursorRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for ShowCursorRequest {
}

pub type Barrier = u32;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarrierDirections(u8);
impl BarrierDirections {
    pub const POSITIVE_X: Self = Self(1 << 0);
    pub const POSITIVE_Y: Self = Self(1 << 1);
    pub const NEGATIVE_X: Self = Self(1 << 2);
    pub const NEGATIVE_Y: Self = Self(1 << 3);
}
impl From<BarrierDirections> for u8 {
    #[inline]
    fn from(input: BarrierDirections) -> Self {
        input.0
    }
}
impl From<BarrierDirections> for Option<u8> {
    #[inline]
    fn from(input: BarrierDirections) -> Self {
        Some(input.0)
    }
}
impl From<BarrierDirections> for u16 {
    #[inline]
    fn from(input: BarrierDirections) -> Self {
        u16::from(input.0)
    }
}
impl From<BarrierDirections> for Option<u16> {
    #[inline]
    fn from(input: BarrierDirections) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<BarrierDirections> for u32 {
    #[inline]
    fn from(input: BarrierDirections) -> Self {
        u32::from(input.0)
    }
}
impl From<BarrierDirections> for Option<u32> {
    #[inline]
    fn from(input: BarrierDirections) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for BarrierDirections {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for BarrierDirections  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::POSITIVE_X.0.into(), "POSITIVE_X", "PositiveX"),
            (Self::POSITIVE_Y.0.into(), "POSITIVE_Y", "PositiveY"),
            (Self::NEGATIVE_X.0.into(), "NEGATIVE_X", "NegativeX"),
            (Self::NEGATIVE_Y.0.into(), "NEGATIVE_Y", "NegativeY"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(BarrierDirections, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for BarrierDirections {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::POSITIVE_X.0,
            Self::POSITIVE_Y.0,
            Self::NEGATIVE_X.0,
            Self::NEGATIVE_Y.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the CreatePointerBarrier request
pub const CREATE_POINTER_BARRIER_REQUEST: u8 = 31;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePointerBarrierRequest<'input> {
    pub barrier: Barrier,
    pub window: xproto::Window,
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
    pub directions: u32,
    pub devices: Cow<'input, [u16]>,
}
impl<'input> CreatePointerBarrierRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
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
            major_opcode,
            CREATE_POINTER_BARRIER_REQUEST,
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
        (vec![request0.into(), devices_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_POINTER_BARRIER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (barrier, remaining) = Barrier::try_parse(value)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (x1, remaining) = u16::try_parse(remaining)?;
        let (y1, remaining) = u16::try_parse(remaining)?;
        let (x2, remaining) = u16::try_parse(remaining)?;
        let (y2, remaining) = u16::try_parse(remaining)?;
        let (directions, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (num_devices, remaining) = u16::try_parse(remaining)?;
        let (devices, remaining) = crate::x11_utils::parse_list::<u16>(remaining, num_devices.try_to_usize()?)?;
        let _ = remaining;
        Ok(CreatePointerBarrierRequest {
            barrier,
            window,
            x1,
            y1,
            x2,
            y2,
            directions,
            devices: Cow::Owned(devices),
        })
    }
    /// Clone all borrowed data in this CreatePointerBarrierRequest.
    pub fn into_owned(self) -> CreatePointerBarrierRequest<'static> {
        CreatePointerBarrierRequest {
            barrier: self.barrier,
            window: self.window,
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            directions: self.directions,
            devices: Cow::Owned(self.devices.into_owned()),
        }
    }
}
impl<'input> Request for CreatePointerBarrierRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for CreatePointerBarrierRequest<'input> {
}

/// Opcode for the DeletePointerBarrier request
pub const DELETE_POINTER_BARRIER_REQUEST: u8 = 32;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeletePointerBarrierRequest {
    pub barrier: Barrier,
}
impl DeletePointerBarrierRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let barrier_bytes = self.barrier.serialize();
        let mut request0 = vec![
            major_opcode,
            DELETE_POINTER_BARRIER_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DELETE_POINTER_BARRIER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (barrier, remaining) = Barrier::try_parse(value)?;
        let _ = remaining;
        Ok(DeletePointerBarrierRequest {
            barrier,
        })
    }
}
impl Request for DeletePointerBarrierRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DeletePointerBarrierRequest {
}

