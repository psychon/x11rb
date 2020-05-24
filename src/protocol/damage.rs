// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Damage` X11 extension.

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
use super::xfixes;
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "DAMAGE";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

pub type Damage = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReportLevel {
    RawRectangles = 0,
    DeltaRectangles = 1,
    BoundingBox = 2,
    NonEmpty = 3,
}
impl From<ReportLevel> for u8 {
    fn from(input: ReportLevel) -> Self {
        match input {
            ReportLevel::RawRectangles => 0,
            ReportLevel::DeltaRectangles => 1,
            ReportLevel::BoundingBox => 2,
            ReportLevel::NonEmpty => 3,
        }
    }
}
impl From<ReportLevel> for Option<u8> {
    fn from(input: ReportLevel) -> Self {
        Some(u8::from(input))
    }
}
impl From<ReportLevel> for u16 {
    fn from(input: ReportLevel) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ReportLevel> for Option<u16> {
    fn from(input: ReportLevel) -> Self {
        Some(u16::from(input))
    }
}
impl From<ReportLevel> for u32 {
    fn from(input: ReportLevel) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ReportLevel> for Option<u32> {
    fn from(input: ReportLevel) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ReportLevel {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReportLevel::RawRectangles),
            1 => Ok(ReportLevel::DeltaRectangles),
            2 => Ok(ReportLevel::BoundingBox),
            3 => Ok(ReportLevel::NonEmpty),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ReportLevel {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ReportLevel {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the BadDamage error
pub const BAD_DAMAGE_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadDamageError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl TryParse for BadDamageError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadDamageError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadDamageError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&BadDamageError> for [u8; 32] {
    fn from(input: &BadDamageError) -> Self {
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
impl From<BadDamageError> for [u8; 32] {
    fn from(input: BadDamageError) -> Self {
        Self::from(&input)
    }
}

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
            Self::opcode(),
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
        client_major_version,
        client_minor_version,
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
pub struct CreateRequest {
    pub damage: Damage,
    pub drawable: xproto::Drawable,
    pub level: ReportLevel,
}
impl CreateRequest {
    /// Opcode for the Create request
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
        let damage_bytes = self.damage.serialize();
        let drawable_bytes = self.drawable.serialize();
        let level_bytes = u8::from(self.level).serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            damage_bytes[0],
            damage_bytes[1],
            damage_bytes[2],
            damage_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            level_bytes[0],
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
pub fn create<Conn>(conn: &Conn, damage: Damage, drawable: xproto::Drawable, level: ReportLevel) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRequest {
        damage,
        drawable,
        level,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyRequest {
    pub damage: Damage,
}
impl DestroyRequest {
    /// Opcode for the Destroy request
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
        let damage_bytes = self.damage.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            damage_bytes[0],
            damage_bytes[1],
            damage_bytes[2],
            damage_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn destroy<Conn>(conn: &Conn, damage: Damage) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyRequest {
        damage,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubtractRequest {
    pub damage: Damage,
    pub repair: xfixes::Region,
    pub parts: xfixes::Region,
}
impl SubtractRequest {
    /// Opcode for the Subtract request
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
        let damage_bytes = self.damage.serialize();
        let repair_bytes = self.repair.serialize();
        let parts_bytes = self.parts.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            damage_bytes[0],
            damage_bytes[1],
            damage_bytes[2],
            damage_bytes[3],
            repair_bytes[0],
            repair_bytes[1],
            repair_bytes[2],
            repair_bytes[3],
            parts_bytes[0],
            parts_bytes[1],
            parts_bytes[2],
            parts_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn subtract<Conn, A, B>(conn: &Conn, damage: Damage, repair: A, parts: B) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xfixes::Region>,
    B: Into<xfixes::Region>,
{
    let repair: xfixes::Region = repair.into();
    let parts: xfixes::Region = parts.into();
    let request0 = SubtractRequest {
        damage,
        repair,
        parts,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddRequest {
    pub drawable: xproto::Drawable,
    pub region: xfixes::Region,
}
impl AddRequest {
    /// Opcode for the Add request
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
        let drawable_bytes = self.drawable.serialize();
        let region_bytes = self.region.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn add<Conn>(conn: &Conn, drawable: xproto::Drawable, region: xfixes::Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AddRequest {
        drawable,
        region,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the Notify event
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotifyEvent {
    pub response_type: u8,
    pub level: ReportLevel,
    pub sequence: u16,
    pub drawable: xproto::Drawable,
    pub damage: Damage,
    pub timestamp: xproto::Timestamp,
    pub area: xproto::Rectangle,
    pub geometry: xproto::Rectangle,
}
impl TryParse for NotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (level, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (damage, remaining) = Damage::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (area, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let (geometry, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let level = level.try_into()?;
        let result = NotifyEvent { response_type, level, sequence, drawable, damage, timestamp, area, geometry };
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
        let level_bytes = u8::from(input.level).serialize();
        let sequence_bytes = input.sequence.serialize();
        let drawable_bytes = input.drawable.serialize();
        let damage_bytes = input.damage.serialize();
        let timestamp_bytes = input.timestamp.serialize();
        let area_bytes = input.area.serialize();
        let geometry_bytes = input.geometry.serialize();
        [
            response_type_bytes[0],
            level_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            damage_bytes[0],
            damage_bytes[1],
            damage_bytes[2],
            damage_bytes[3],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            area_bytes[0],
            area_bytes[1],
            area_bytes[2],
            area_bytes[3],
            area_bytes[4],
            area_bytes[5],
            area_bytes[6],
            area_bytes[7],
            geometry_bytes[0],
            geometry_bytes[1],
            geometry_bytes[2],
            geometry_bytes[3],
            geometry_bytes[4],
            geometry_bytes[5],
            geometry_bytes[6],
            geometry_bytes[7],
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
    fn damage_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }
    fn damage_create(&self, damage: Damage, drawable: xproto::Drawable, level: ReportLevel) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create(self, damage, drawable, level)
    }
    fn damage_destroy(&self, damage: Damage) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy(self, damage)
    }
    fn damage_subtract<A, B>(&self, damage: Damage, repair: A, parts: B) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xfixes::Region>,
        B: Into<xfixes::Region>,
    {
        subtract(self, damage, repair, parts)
    }
    fn damage_add(&self, drawable: xproto::Drawable, region: xfixes::Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        add(self, drawable, region)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
