// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xevie` X11 extension.

#![allow(clippy::too_many_arguments)]
// The code generator is simpler if it can always use conversions
#![allow(clippy::useless_conversion)]

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
use crate::BufWithFds;
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XEVIE";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 0);

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryVersionRequest {
    pub client_major_version: u16,
    pub client_minor_version: u16,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<[Cow<'static, [u8]>; 1]> {
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
            client_minor_version_bytes[0],
            client_minor_version_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        ([request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (client_major_version, remaining) = u16::try_parse(value)?;
        let (client_minor_version, remaining) = u16::try_parse(remaining)?;
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
impl Serialize for QueryVersionReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let server_major_version_bytes = self.server_major_version.serialize();
        let server_minor_version_bytes = self.server_minor_version.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            server_major_version_bytes[0],
            server_major_version_bytes[1],
            server_minor_version_bytes[0],
            server_minor_version_bytes[1],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.server_major_version.serialize_into(bytes);
        self.server_minor_version.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}
#[cfg(test)]
impl crate::x11_utils::GenerateRandom for QueryVersionReply {
    fn generate(rng: &mut fastrand::Rng) -> Self {
        Self {
            sequence: crate::x11_utils::GenerateRandom::generate(rng),
            length: crate::x11_utils::GenerateRandom::generate(rng),
            server_major_version: crate::x11_utils::GenerateRandom::generate(rng),
            server_minor_version: crate::x11_utils::GenerateRandom::generate(rng),
        }
    }
}
#[cfg(test)]
#[test]
fn test_query_version_reply() {
    crate::x11_utils::test_randomised_type::<QueryVersionReply>();
}

/// Opcode for the Start request
pub const START_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StartRequest {
    pub screen: u32,
}
impl StartRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<[Cow<'static, [u8]>; 1]> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            START_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        ([request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != START_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(StartRequest {
            screen,
        })
    }
}
impl Request for StartRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for StartRequest {
    type Reply = StartReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StartReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for StartReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = StartReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for StartReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
    }
}
#[cfg(test)]
impl crate::x11_utils::GenerateRandom for StartReply {
    fn generate(rng: &mut fastrand::Rng) -> Self {
        Self {
            sequence: crate::x11_utils::GenerateRandom::generate(rng),
            length: crate::x11_utils::GenerateRandom::generate(rng),
        }
    }
}
#[cfg(test)]
#[test]
fn test_start_reply() {
    crate::x11_utils::test_randomised_type::<StartReply>();
}

/// Opcode for the End request
pub const END_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndRequest {
    pub cmap: u32,
}
impl EndRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<[Cow<'static, [u8]>; 1]> {
        let length_so_far = 0;
        let cmap_bytes = self.cmap.serialize();
        let mut request0 = vec![
            major_opcode,
            END_REQUEST,
            0,
            0,
            cmap_bytes[0],
            cmap_bytes[1],
            cmap_bytes[2],
            cmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        ([request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != END_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (cmap, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(EndRequest {
            cmap,
        })
    }
}
impl Request for EndRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for EndRequest {
    type Reply = EndReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for EndReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = EndReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for EndReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
    }
}
#[cfg(test)]
impl crate::x11_utils::GenerateRandom for EndReply {
    fn generate(rng: &mut fastrand::Rng) -> Self {
        Self {
            sequence: crate::x11_utils::GenerateRandom::generate(rng),
            length: crate::x11_utils::GenerateRandom::generate(rng),
        }
    }
}
#[cfg(test)]
#[test]
fn test_end_reply() {
    crate::x11_utils::test_randomised_type::<EndReply>();
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Datatype(bool);
impl Datatype {
    pub const UNMODIFIED: Self = Self(false);
    pub const MODIFIED: Self = Self(true);
}
impl From<Datatype> for bool {
    #[inline]
    fn from(input: Datatype) -> Self {
        input.0
    }
}
impl From<Datatype> for Option<bool> {
    #[inline]
    fn from(input: Datatype) -> Self {
        Some(input.0)
    }
}
impl From<Datatype> for u8 {
    #[inline]
    fn from(input: Datatype) -> Self {
        u8::from(input.0)
    }
}
impl From<Datatype> for Option<u8> {
    #[inline]
    fn from(input: Datatype) -> Self {
        Some(u8::from(input.0))
    }
}
impl From<Datatype> for u16 {
    #[inline]
    fn from(input: Datatype) -> Self {
        u16::from(input.0)
    }
}
impl From<Datatype> for Option<u16> {
    #[inline]
    fn from(input: Datatype) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Datatype> for u32 {
    #[inline]
    fn from(input: Datatype) -> Self {
        u32::from(input.0)
    }
}
impl From<Datatype> for Option<u32> {
    #[inline]
    fn from(input: Datatype) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<bool> for Datatype {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for Datatype  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::UNMODIFIED.0.into(), "UNMODIFIED", "Unmodified"),
            (Self::MODIFIED.0.into(), "MODIFIED", "Modified"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenerateRandom for Datatype {
    fn generate(rng: &mut fastrand::Rng) -> Self {
        let possible_values = [
            Self::UNMODIFIED,
            Self::MODIFIED,
        ];
        let index = rng.usize(..possible_values.len());
        possible_values[index]
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Event {
}
impl TryParse for Event {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = remaining.get(32..).ok_or(ParseError::InsufficientData)?;
        let result = Event {  };
        Ok((result, remaining))
    }
}
impl Serialize for Event {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        [
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
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        bytes.extend_from_slice(&[0; 32]);
    }
}
#[cfg(test)]
impl crate::x11_utils::GenerateRandom for Event {
    fn generate(rng: &mut fastrand::Rng) -> Self {
        Self {
        }
    }
}
#[cfg(test)]
#[test]
fn test_event() {
    crate::x11_utils::test_randomised_type::<Event>();
}

/// Opcode for the Send request
pub const SEND_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SendRequest {
    pub event: Event,
    pub data_type: u32,
}
impl SendRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<[Cow<'static, [u8]>; 1]> {
        let length_so_far = 0;
        let event_bytes = self.event.serialize();
        let data_type_bytes = self.data_type.serialize();
        let mut request0 = vec![
            major_opcode,
            SEND_REQUEST,
            0,
            0,
            event_bytes[0],
            event_bytes[1],
            event_bytes[2],
            event_bytes[3],
            event_bytes[4],
            event_bytes[5],
            event_bytes[6],
            event_bytes[7],
            event_bytes[8],
            event_bytes[9],
            event_bytes[10],
            event_bytes[11],
            event_bytes[12],
            event_bytes[13],
            event_bytes[14],
            event_bytes[15],
            event_bytes[16],
            event_bytes[17],
            event_bytes[18],
            event_bytes[19],
            event_bytes[20],
            event_bytes[21],
            event_bytes[22],
            event_bytes[23],
            event_bytes[24],
            event_bytes[25],
            event_bytes[26],
            event_bytes[27],
            event_bytes[28],
            event_bytes[29],
            event_bytes[30],
            event_bytes[31],
            data_type_bytes[0],
            data_type_bytes[1],
            data_type_bytes[2],
            data_type_bytes[3],
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
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        ([request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SEND_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (event, remaining) = Event::try_parse(value)?;
        let (data_type, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(64..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(SendRequest {
            event,
            data_type,
        })
    }
}
impl Request for SendRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for SendRequest {
    type Reply = SendReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SendReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for SendReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = SendReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for SendReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
    }
}
#[cfg(test)]
impl crate::x11_utils::GenerateRandom for SendReply {
    fn generate(rng: &mut fastrand::Rng) -> Self {
        Self {
            sequence: crate::x11_utils::GenerateRandom::generate(rng),
            length: crate::x11_utils::GenerateRandom::generate(rng),
        }
    }
}
#[cfg(test)]
#[test]
fn test_send_reply() {
    crate::x11_utils::test_randomised_type::<SendReply>();
}

/// Opcode for the SelectInput request
pub const SELECT_INPUT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectInputRequest {
    pub event_mask: u32,
}
impl SelectInputRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<[Cow<'static, [u8]>; 1]> {
        let length_so_far = 0;
        let event_mask_bytes = self.event_mask.serialize();
        let mut request0 = vec![
            major_opcode,
            SELECT_INPUT_REQUEST,
            0,
            0,
            event_mask_bytes[0],
            event_mask_bytes[1],
            event_mask_bytes[2],
            event_mask_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        ([request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SELECT_INPUT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (event_mask, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(SelectInputRequest {
            event_mask,
        })
    }
}
impl Request for SelectInputRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for SelectInputRequest {
    type Reply = SelectInputReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectInputReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for SelectInputReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = SelectInputReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for SelectInputReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
    }
}
#[cfg(test)]
impl crate::x11_utils::GenerateRandom for SelectInputReply {
    fn generate(rng: &mut fastrand::Rng) -> Self {
        Self {
            sequence: crate::x11_utils::GenerateRandom::generate(rng),
            length: crate::x11_utils::GenerateRandom::generate(rng),
        }
    }
}
#[cfg(test)]
#[test]
fn test_select_input_reply() {
    crate::x11_utils::test_randomised_type::<SelectInputReply>();
}

