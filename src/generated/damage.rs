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
#[allow(unused_imports)]
use super::render;
#[allow(unused_imports)]
use super::shape;
#[allow(unused_imports)]
use super::xfixes;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "DAMAGE";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 1);

pub type DAMAGE = u32;

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
            _ => Err(ParseError::ParseError)
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
impl BadDamageError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadDamageError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadDamageError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadDamageError> for [u8; 32] {
    fn from(input: &BadDamageError) -> Self {
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
impl From<BadDamageError> for [u8; 32] {
    fn from(input: BadDamageError) -> Self {
        Self::from(&input)
    }
}

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

/// Opcode for the Create request
pub const CREATE_REQUEST: u8 = 1;
pub fn create<Conn, A>(conn: &Conn, damage: DAMAGE, drawable: DRAWABLE, level: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let damage_bytes = damage.serialize();
    let drawable_bytes = drawable.serialize();
    let level = level.into();
    let level_bytes = level.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Destroy request
pub const DESTROY_REQUEST: u8 = 2;
pub fn destroy<Conn>(conn: &Conn, damage: DAMAGE) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let damage_bytes = damage.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        damage_bytes[0],
        damage_bytes[1],
        damage_bytes[2],
        damage_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Subtract request
pub const SUBTRACT_REQUEST: u8 = 3;
pub fn subtract<Conn>(conn: &Conn, damage: DAMAGE, repair: xfixes::REGION, parts: xfixes::REGION) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let damage_bytes = damage.serialize();
    let repair_bytes = repair.serialize();
    let parts_bytes = parts.serialize();
    let request0 = [
        extension_information.major_opcode,
        SUBTRACT_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Add request
pub const ADD_REQUEST: u8 = 4;
pub fn add<Conn>(conn: &Conn, drawable: DRAWABLE, region: xfixes::REGION) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let region_bytes = region.serialize();
    let request0 = [
        extension_information.major_opcode,
        ADD_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        region_bytes[0],
        region_bytes[1],
        region_bytes[2],
        region_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Notify event
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotifyEvent {
    pub response_type: u8,
    pub level: u8,
    pub sequence: u16,
    pub drawable: DRAWABLE,
    pub damage: DAMAGE,
    pub timestamp: TIMESTAMP,
    pub area: Rectangle,
    pub geometry: Rectangle,
}
impl NotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (level, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = DRAWABLE::try_parse(remaining)?;
        let (damage, remaining) = DAMAGE::try_parse(remaining)?;
        let (timestamp, remaining) = TIMESTAMP::try_parse(remaining)?;
        let (area, remaining) = Rectangle::try_parse(remaining)?;
        let (geometry, remaining) = Rectangle::try_parse(remaining)?;
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
        let level = input.level.serialize();
        let sequence = input.sequence.serialize();
        let drawable = input.drawable.serialize();
        let damage = input.damage.serialize();
        let timestamp = input.timestamp.serialize();
        let area = input.area.serialize();
        let geometry = input.geometry.serialize();
        [
            response_type[0], level[0], sequence[0], sequence[1], drawable[0], drawable[1], drawable[2], drawable[3],
            damage[0], damage[1], damage[2], damage[3], timestamp[0], timestamp[1], timestamp[2], timestamp[3],
            area[0], area[1], area[2], area[3], area[4], area[5], area[6], area[7],
            geometry[0], geometry[1], geometry[2], geometry[3], geometry[4], geometry[5], geometry[6], geometry[7]
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

    fn damage_create<A>(&self, damage: DAMAGE, drawable: DRAWABLE, level: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        create(self, damage, drawable, level)
    }

    fn damage_destroy(&self, damage: DAMAGE) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy(self, damage)
    }

    fn damage_subtract(&self, damage: DAMAGE, repair: xfixes::REGION, parts: xfixes::REGION) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        subtract(self, damage, repair, parts)
    }

    fn damage_add(&self, drawable: DRAWABLE, region: xfixes::REGION) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        add(self, drawable, region)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
