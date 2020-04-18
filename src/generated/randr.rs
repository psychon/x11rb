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

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "RANDR";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 6);

pub type Mode = u32;

pub type Crtc = u32;

pub type Output = u32;

pub type Provider = u32;

pub type Lease = u32;

/// Opcode for the BadOutput error
pub const BAD_OUTPUT_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadOutputError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadOutputError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadOutputError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadOutputError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadOutputError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadOutputError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadOutputError> for [u8; 32] {
    fn from(input: &BadOutputError) -> Self {
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
impl From<BadOutputError> for [u8; 32] {
    fn from(input: BadOutputError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadCrtc error
pub const BAD_CRTC_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadCrtcError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadCrtcError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadCrtcError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadCrtcError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadCrtcError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadCrtcError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadCrtcError> for [u8; 32] {
    fn from(input: &BadCrtcError) -> Self {
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
impl From<BadCrtcError> for [u8; 32] {
    fn from(input: BadCrtcError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadMode error
pub const BAD_MODE_ERROR: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadModeError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadModeError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadModeError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadModeError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadModeError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadModeError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadModeError> for [u8; 32] {
    fn from(input: &BadModeError) -> Self {
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
impl From<BadModeError> for [u8; 32] {
    fn from(input: BadModeError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadProvider error
pub const BAD_PROVIDER_ERROR: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadProviderError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadProviderError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadProviderError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadProviderError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadProviderError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadProviderError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadProviderError> for [u8; 32] {
    fn from(input: &BadProviderError) -> Self {
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
impl From<BadProviderError> for [u8; 32] {
    fn from(input: BadProviderError) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Rotation {
    Rotate0 = 1 << 0,
    Rotate90 = 1 << 1,
    Rotate180 = 1 << 2,
    Rotate270 = 1 << 3,
    ReflectX = 1 << 4,
    ReflectY = 1 << 5,
}
impl From<Rotation> for u8 {
    fn from(input: Rotation) -> Self {
        match input {
            Rotation::Rotate0 => 1 << 0,
            Rotation::Rotate90 => 1 << 1,
            Rotation::Rotate180 => 1 << 2,
            Rotation::Rotate270 => 1 << 3,
            Rotation::ReflectX => 1 << 4,
            Rotation::ReflectY => 1 << 5,
        }
    }
}
impl From<Rotation> for Option<u8> {
    fn from(input: Rotation) -> Self {
        Some(u8::from(input))
    }
}
impl From<Rotation> for u16 {
    fn from(input: Rotation) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Rotation> for Option<u16> {
    fn from(input: Rotation) -> Self {
        Some(u16::from(input))
    }
}
impl From<Rotation> for u32 {
    fn from(input: Rotation) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Rotation> for Option<u32> {
    fn from(input: Rotation) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Rotation {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Rotation::Rotate0),
            2 => Ok(Rotation::Rotate90),
            4 => Ok(Rotation::Rotate180),
            8 => Ok(Rotation::Rotate270),
            16 => Ok(Rotation::ReflectX),
            32 => Ok(Rotation::ReflectY),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Rotation {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Rotation {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(Rotation, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenSize {
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}
impl TryParse for ScreenSize {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (mwidth, remaining) = u16::try_parse(remaining)?;
        let (mheight, remaining) = u16::try_parse(remaining)?;
        let result = ScreenSize { width, height, mwidth, mheight };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ScreenSize {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ScreenSize {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let mwidth_bytes = self.mwidth.serialize();
        let mheight_bytes = self.mheight.serialize();
        [
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            mwidth_bytes[0],
            mwidth_bytes[1],
            mheight_bytes[0],
            mheight_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.mwidth.serialize_into(bytes);
        self.mheight.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshRates {
    pub rates: Vec<u16>,
}
impl TryParse for RefreshRates {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (n_rates, remaining) = u16::try_parse(remaining)?;
        let (rates, remaining) = crate::x11_utils::parse_list::<u16>(remaining, n_rates as usize)?;
        let result = RefreshRates { rates };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RefreshRates {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for RefreshRates {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        let n_rates = self.rates.len() as u16;
        n_rates.serialize_into(bytes);
        self.rates.serialize_into(bytes);
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let major_version_bytes = major_version.serialize();
    let minor_version_bytes = minor_version.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        major_version_bytes[0],
        major_version_bytes[1],
        major_version_bytes[2],
        major_version_bytes[3],
        minor_version_bytes[0],
        minor_version_bytes[1],
        minor_version_bytes[2],
        minor_version_bytes[3],
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
pub enum SetConfig {
    Success = 0,
    InvalidConfigTime = 1,
    InvalidTime = 2,
    Failed = 3,
}
impl From<SetConfig> for u8 {
    fn from(input: SetConfig) -> Self {
        match input {
            SetConfig::Success => 0,
            SetConfig::InvalidConfigTime => 1,
            SetConfig::InvalidTime => 2,
            SetConfig::Failed => 3,
        }
    }
}
impl From<SetConfig> for Option<u8> {
    fn from(input: SetConfig) -> Self {
        Some(u8::from(input))
    }
}
impl From<SetConfig> for u16 {
    fn from(input: SetConfig) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetConfig> for Option<u16> {
    fn from(input: SetConfig) -> Self {
        Some(u16::from(input))
    }
}
impl From<SetConfig> for u32 {
    fn from(input: SetConfig) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetConfig> for Option<u32> {
    fn from(input: SetConfig) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SetConfig {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SetConfig::Success),
            1 => Ok(SetConfig::InvalidConfigTime),
            2 => Ok(SetConfig::InvalidTime),
            3 => Ok(SetConfig::Failed),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for SetConfig {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SetConfig {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SetScreenConfig request
pub const SET_SCREEN_CONFIG_REQUEST: u8 = 2;
pub fn set_screen_config<Conn>(conn: &Conn, window: xproto::Window, timestamp: xproto::Timestamp, config_timestamp: xproto::Timestamp, size_id: u16, rotation: u16, rate: u16) -> Result<Cookie<'_, Conn, SetScreenConfigReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let timestamp_bytes = timestamp.serialize();
    let config_timestamp_bytes = config_timestamp.serialize();
    let size_id_bytes = size_id.serialize();
    let rotation_bytes = rotation.serialize();
    let rate_bytes = rate.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_SCREEN_CONFIG_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        timestamp_bytes[0],
        timestamp_bytes[1],
        timestamp_bytes[2],
        timestamp_bytes[3],
        config_timestamp_bytes[0],
        config_timestamp_bytes[1],
        config_timestamp_bytes[2],
        config_timestamp_bytes[3],
        size_id_bytes[0],
        size_id_bytes[1],
        rotation_bytes[0],
        rotation_bytes[1],
        rate_bytes[0],
        rate_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetScreenConfigReply {
    pub response_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub new_timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub root: xproto::Window,
    pub subpixel_order: render::SubPixel,
}
impl SetScreenConfigReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (new_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (subpixel_order, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(10..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let subpixel_order = subpixel_order.try_into()?;
        let result = SetScreenConfigReply { response_type, status, sequence, length, new_timestamp, config_timestamp, root, subpixel_order };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetScreenConfigReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NotifyMask {
    ScreenChange = 1 << 0,
    CrtcChange = 1 << 1,
    OutputChange = 1 << 2,
    OutputProperty = 1 << 3,
    ProviderChange = 1 << 4,
    ProviderProperty = 1 << 5,
    ResourceChange = 1 << 6,
    Lease = 1 << 7,
}
impl From<NotifyMask> for u8 {
    fn from(input: NotifyMask) -> Self {
        match input {
            NotifyMask::ScreenChange => 1 << 0,
            NotifyMask::CrtcChange => 1 << 1,
            NotifyMask::OutputChange => 1 << 2,
            NotifyMask::OutputProperty => 1 << 3,
            NotifyMask::ProviderChange => 1 << 4,
            NotifyMask::ProviderProperty => 1 << 5,
            NotifyMask::ResourceChange => 1 << 6,
            NotifyMask::Lease => 1 << 7,
        }
    }
}
impl From<NotifyMask> for Option<u8> {
    fn from(input: NotifyMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<NotifyMask> for u16 {
    fn from(input: NotifyMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyMask> for Option<u16> {
    fn from(input: NotifyMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<NotifyMask> for u32 {
    fn from(input: NotifyMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyMask> for Option<u32> {
    fn from(input: NotifyMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for NotifyMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NotifyMask::ScreenChange),
            2 => Ok(NotifyMask::CrtcChange),
            4 => Ok(NotifyMask::OutputChange),
            8 => Ok(NotifyMask::OutputProperty),
            16 => Ok(NotifyMask::ProviderChange),
            32 => Ok(NotifyMask::ProviderProperty),
            64 => Ok(NotifyMask::ResourceChange),
            128 => Ok(NotifyMask::Lease),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for NotifyMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for NotifyMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(NotifyMask, u8);

/// Opcode for the SelectInput request
pub const SELECT_INPUT_REQUEST: u8 = 4;
pub fn select_input<Conn>(conn: &Conn, window: xproto::Window, enable: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let enable_bytes = enable.serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_INPUT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        enable_bytes[0],
        enable_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetScreenInfo request
pub const GET_SCREEN_INFO_REQUEST: u8 = 5;
pub fn get_screen_info<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenInfoReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SCREEN_INFO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetScreenInfoReply {
    pub response_type: u8,
    pub rotations: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xproto::Window,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub size_id: u16,
    pub rotation: u16,
    pub rate: u16,
    pub n_info: u16,
    pub sizes: Vec<ScreenSize>,
    pub rates: Vec<RefreshRates>,
}
impl GetScreenInfoReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (rotations, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (n_sizes, remaining) = u16::try_parse(remaining)?;
        let (size_id, remaining) = u16::try_parse(remaining)?;
        let (rotation, remaining) = u16::try_parse(remaining)?;
        let (rate, remaining) = u16::try_parse(remaining)?;
        let (n_info, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (sizes, remaining) = crate::x11_utils::parse_list::<ScreenSize>(remaining, n_sizes as usize)?;
        let (rates, remaining) = crate::x11_utils::parse_list::<RefreshRates>(remaining, (n_info as usize) - (n_sizes as usize))?;
        let result = GetScreenInfoReply { response_type, rotations, sequence, length, root, timestamp, config_timestamp, size_id, rotation, rate, n_info, sizes, rates };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetScreenSizeRange request
pub const GET_SCREEN_SIZE_RANGE_REQUEST: u8 = 6;
pub fn get_screen_size_range<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenSizeRangeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SCREEN_SIZE_RANGE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenSizeRangeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_width: u16,
    pub min_height: u16,
    pub max_width: u16,
    pub max_height: u16,
}
impl GetScreenSizeRangeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (min_width, remaining) = u16::try_parse(remaining)?;
        let (min_height, remaining) = u16::try_parse(remaining)?;
        let (max_width, remaining) = u16::try_parse(remaining)?;
        let (max_height, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let result = GetScreenSizeRangeReply { response_type, sequence, length, min_width, min_height, max_width, max_height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenSizeRangeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetScreenSize request
pub const SET_SCREEN_SIZE_REQUEST: u8 = 7;
pub fn set_screen_size<Conn>(conn: &Conn, window: xproto::Window, width: u16, height: u16, mm_width: u32, mm_height: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let mm_width_bytes = mm_width.serialize();
    let mm_height_bytes = mm_height.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_SCREEN_SIZE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        mm_width_bytes[0],
        mm_width_bytes[1],
        mm_width_bytes[2],
        mm_width_bytes[3],
        mm_height_bytes[0],
        mm_height_bytes[1],
        mm_height_bytes[2],
        mm_height_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ModeFlag {
    HsyncPositive = 1 << 0,
    HsyncNegative = 1 << 1,
    VsyncPositive = 1 << 2,
    VsyncNegative = 1 << 3,
    Interlace = 1 << 4,
    DoubleScan = 1 << 5,
    Csync = 1 << 6,
    CsyncPositive = 1 << 7,
    CsyncNegative = 1 << 8,
    HskewPresent = 1 << 9,
    Bcast = 1 << 10,
    PixelMultiplex = 1 << 11,
    DoubleClock = 1 << 12,
    HalveClock = 1 << 13,
}
impl From<ModeFlag> for u16 {
    fn from(input: ModeFlag) -> Self {
        match input {
            ModeFlag::HsyncPositive => 1 << 0,
            ModeFlag::HsyncNegative => 1 << 1,
            ModeFlag::VsyncPositive => 1 << 2,
            ModeFlag::VsyncNegative => 1 << 3,
            ModeFlag::Interlace => 1 << 4,
            ModeFlag::DoubleScan => 1 << 5,
            ModeFlag::Csync => 1 << 6,
            ModeFlag::CsyncPositive => 1 << 7,
            ModeFlag::CsyncNegative => 1 << 8,
            ModeFlag::HskewPresent => 1 << 9,
            ModeFlag::Bcast => 1 << 10,
            ModeFlag::PixelMultiplex => 1 << 11,
            ModeFlag::DoubleClock => 1 << 12,
            ModeFlag::HalveClock => 1 << 13,
        }
    }
}
impl From<ModeFlag> for Option<u16> {
    fn from(input: ModeFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<ModeFlag> for u32 {
    fn from(input: ModeFlag) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<ModeFlag> for Option<u32> {
    fn from(input: ModeFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for ModeFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ModeFlag::HsyncPositive),
            2 => Ok(ModeFlag::HsyncNegative),
            4 => Ok(ModeFlag::VsyncPositive),
            8 => Ok(ModeFlag::VsyncNegative),
            16 => Ok(ModeFlag::Interlace),
            32 => Ok(ModeFlag::DoubleScan),
            64 => Ok(ModeFlag::Csync),
            128 => Ok(ModeFlag::CsyncPositive),
            256 => Ok(ModeFlag::CsyncNegative),
            512 => Ok(ModeFlag::HskewPresent),
            1024 => Ok(ModeFlag::Bcast),
            2048 => Ok(ModeFlag::PixelMultiplex),
            4096 => Ok(ModeFlag::DoubleClock),
            8192 => Ok(ModeFlag::HalveClock),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u32> for ModeFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ModeFlag, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeInfo {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub dot_clock: u32,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub name_len: u16,
    pub mode_flags: u32,
}
impl TryParse for ModeInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (id, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (dot_clock, remaining) = u32::try_parse(remaining)?;
        let (hsync_start, remaining) = u16::try_parse(remaining)?;
        let (hsync_end, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u16::try_parse(remaining)?;
        let (vsync_start, remaining) = u16::try_parse(remaining)?;
        let (vsync_end, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (mode_flags, remaining) = u32::try_parse(remaining)?;
        let result = ModeInfo { id, width, height, dot_clock, hsync_start, hsync_end, htotal, hskew, vsync_start, vsync_end, vtotal, name_len, mode_flags };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ModeInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ModeInfo {
    type Bytes = [u8; 32];
    fn serialize(&self) -> Self::Bytes {
        let id_bytes = self.id.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let dot_clock_bytes = self.dot_clock.serialize();
        let hsync_start_bytes = self.hsync_start.serialize();
        let hsync_end_bytes = self.hsync_end.serialize();
        let htotal_bytes = self.htotal.serialize();
        let hskew_bytes = self.hskew.serialize();
        let vsync_start_bytes = self.vsync_start.serialize();
        let vsync_end_bytes = self.vsync_end.serialize();
        let vtotal_bytes = self.vtotal.serialize();
        let name_len_bytes = self.name_len.serialize();
        let mode_flags_bytes = self.mode_flags.serialize();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            dot_clock_bytes[0],
            dot_clock_bytes[1],
            dot_clock_bytes[2],
            dot_clock_bytes[3],
            hsync_start_bytes[0],
            hsync_start_bytes[1],
            hsync_end_bytes[0],
            hsync_end_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            vsync_start_bytes[0],
            vsync_start_bytes[1],
            vsync_end_bytes[0],
            vsync_end_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            name_len_bytes[0],
            name_len_bytes[1],
            mode_flags_bytes[0],
            mode_flags_bytes[1],
            mode_flags_bytes[2],
            mode_flags_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.id.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.dot_clock.serialize_into(bytes);
        self.hsync_start.serialize_into(bytes);
        self.hsync_end.serialize_into(bytes);
        self.htotal.serialize_into(bytes);
        self.hskew.serialize_into(bytes);
        self.vsync_start.serialize_into(bytes);
        self.vsync_end.serialize_into(bytes);
        self.vtotal.serialize_into(bytes);
        self.name_len.serialize_into(bytes);
        self.mode_flags.serialize_into(bytes);
    }
}

/// Opcode for the GetScreenResources request
pub const GET_SCREEN_RESOURCES_REQUEST: u8 = 8;
pub fn get_screen_resources<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenResourcesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SCREEN_RESOURCES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetScreenResourcesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub crtcs: Vec<Crtc>,
    pub outputs: Vec<Output>,
    pub modes: Vec<ModeInfo>,
    pub names: Vec<u8>,
}
impl GetScreenResourcesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_crtcs, remaining) = u16::try_parse(remaining)?;
        let (num_outputs, remaining) = u16::try_parse(remaining)?;
        let (num_modes, remaining) = u16::try_parse(remaining)?;
        let (names_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs as usize)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs as usize)?;
        let (modes, remaining) = crate::x11_utils::parse_list::<ModeInfo>(remaining, num_modes as usize)?;
        let (names, remaining) = crate::x11_utils::parse_list::<u8>(remaining, names_len as usize)?;
        let result = GetScreenResourcesReply { response_type, sequence, length, timestamp, config_timestamp, crtcs, outputs, modes, names };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenResourcesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Connection {
    Connected = 0,
    Disconnected = 1,
    Unknown = 2,
}
impl From<Connection> for u8 {
    fn from(input: Connection) -> Self {
        match input {
            Connection::Connected => 0,
            Connection::Disconnected => 1,
            Connection::Unknown => 2,
        }
    }
}
impl From<Connection> for Option<u8> {
    fn from(input: Connection) -> Self {
        Some(u8::from(input))
    }
}
impl From<Connection> for u16 {
    fn from(input: Connection) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Connection> for Option<u16> {
    fn from(input: Connection) -> Self {
        Some(u16::from(input))
    }
}
impl From<Connection> for u32 {
    fn from(input: Connection) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Connection> for Option<u32> {
    fn from(input: Connection) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Connection {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Connection::Connected),
            1 => Ok(Connection::Disconnected),
            2 => Ok(Connection::Unknown),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Connection {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Connection {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the GetOutputInfo request
pub const GET_OUTPUT_INFO_REQUEST: u8 = 9;
pub fn get_output_info<Conn>(conn: &Conn, output: Output, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Conn, GetOutputInfoReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let config_timestamp_bytes = config_timestamp.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_OUTPUT_INFO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
        config_timestamp_bytes[0],
        config_timestamp_bytes[1],
        config_timestamp_bytes[2],
        config_timestamp_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetOutputInfoReply {
    pub response_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub crtc: Crtc,
    pub mm_width: u32,
    pub mm_height: u32,
    pub connection: Connection,
    pub subpixel_order: render::SubPixel,
    pub num_preferred: u16,
    pub crtcs: Vec<Crtc>,
    pub modes: Vec<Mode>,
    pub clones: Vec<Output>,
    pub name: Vec<u8>,
}
impl GetOutputInfoReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (crtc, remaining) = Crtc::try_parse(remaining)?;
        let (mm_width, remaining) = u32::try_parse(remaining)?;
        let (mm_height, remaining) = u32::try_parse(remaining)?;
        let (connection, remaining) = u8::try_parse(remaining)?;
        let (subpixel_order, remaining) = u8::try_parse(remaining)?;
        let (num_crtcs, remaining) = u16::try_parse(remaining)?;
        let (num_modes, remaining) = u16::try_parse(remaining)?;
        let (num_preferred, remaining) = u16::try_parse(remaining)?;
        let (num_clones, remaining) = u16::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs as usize)?;
        let (modes, remaining) = crate::x11_utils::parse_list::<Mode>(remaining, num_modes as usize)?;
        let (clones, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_clones as usize)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
        let status = status.try_into()?;
        let connection = connection.try_into()?;
        let subpixel_order = subpixel_order.try_into()?;
        let result = GetOutputInfoReply { response_type, status, sequence, length, timestamp, crtc, mm_width, mm_height, connection, subpixel_order, num_preferred, crtcs, modes, clones, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetOutputInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListOutputProperties request
pub const LIST_OUTPUT_PROPERTIES_REQUEST: u8 = 10;
pub fn list_output_properties<Conn>(conn: &Conn, output: Output) -> Result<Cookie<'_, Conn, ListOutputPropertiesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_OUTPUT_PROPERTIES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListOutputPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Vec<xproto::Atom>,
}
impl ListOutputPropertiesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_atoms, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (atoms, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_atoms as usize)?;
        let result = ListOutputPropertiesReply { response_type, sequence, length, atoms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListOutputPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryOutputProperty request
pub const QUERY_OUTPUT_PROPERTY_REQUEST: u8 = 11;
pub fn query_output_property<Conn>(conn: &Conn, output: Output, property: xproto::Atom) -> Result<Cookie<'_, Conn, QueryOutputPropertyReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let property_bytes = property.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_OUTPUT_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryOutputPropertyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub pending: bool,
    pub range: bool,
    pub immutable: bool,
    pub valid_values: Vec<i32>,
}
impl QueryOutputPropertyReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pending, remaining) = bool::try_parse(remaining)?;
        let (range, remaining) = bool::try_parse(remaining)?;
        let (immutable, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let (valid_values, remaining) = crate::x11_utils::parse_list::<i32>(remaining, length as usize)?;
        let result = QueryOutputPropertyReply { response_type, sequence, pending, range, immutable, valid_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryOutputPropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ConfigureOutputProperty request
pub const CONFIGURE_OUTPUT_PROPERTY_REQUEST: u8 = 12;
pub fn configure_output_property<'c, Conn>(conn: &'c Conn, output: Output, property: xproto::Atom, pending: bool, range: bool, values: &[i32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16 + 4 * values.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let property_bytes = property.serialize();
    let pending_bytes = (pending as u8).serialize();
    let range_bytes = (range as u8).serialize();
    let values_bytes = values.serialize();
    let request0 = [
        extension_information.major_opcode,
        CONFIGURE_OUTPUT_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        pending_bytes[0],
        range_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&values_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&values_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ChangeOutputProperty request
pub const CHANGE_OUTPUT_PROPERTY_REQUEST: u8 = 13;
pub fn change_output_property<'c, Conn, A>(conn: &'c Conn, output: Output, property: xproto::Atom, type_: xproto::Atom, format: u8, mode: A, num_units: u32, data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24 + 1 * data.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let format_bytes = format.serialize();
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let num_units_bytes = num_units.serialize();
    assert_eq!(data.len(), ((num_units as usize) * (format as usize)) / (8), "Argument data has an incorrect length");
    let request0 = [
        extension_information.major_opcode,
        CHANGE_OUTPUT_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        format_bytes[0],
        mode_bytes[0],
        0,
        0,
        num_units_bytes[0],
        num_units_bytes[1],
        num_units_bytes[2],
        num_units_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeleteOutputProperty request
pub const DELETE_OUTPUT_PROPERTY_REQUEST: u8 = 14;
pub fn delete_output_property<Conn>(conn: &Conn, output: Output, property: xproto::Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let property_bytes = property.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_OUTPUT_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetOutputProperty request
pub const GET_OUTPUT_PROPERTY_REQUEST: u8 = 15;
pub fn get_output_property<Conn>(conn: &Conn, output: Output, property: xproto::Atom, type_: xproto::Atom, long_offset: u32, long_length: u32, delete: bool, pending: bool) -> Result<Cookie<'_, Conn, GetOutputPropertyReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let long_offset_bytes = long_offset.serialize();
    let long_length_bytes = long_length.serialize();
    let delete_bytes = (delete as u8).serialize();
    let pending_bytes = (pending as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_OUTPUT_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        long_offset_bytes[0],
        long_offset_bytes[1],
        long_offset_bytes[2],
        long_offset_bytes[3],
        long_length_bytes[0],
        long_length_bytes[1],
        long_length_bytes[2],
        long_length_bytes[3],
        delete_bytes[0],
        pending_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetOutputPropertyReply {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xproto::Atom,
    pub bytes_after: u32,
    pub num_items: u32,
    pub data: Vec<u8>,
}
impl GetOutputPropertyReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (num_items as usize) * ((format as usize) / (8)))?;
        let result = GetOutputPropertyReply { response_type, format, sequence, length, type_, bytes_after, num_items, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetOutputPropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreateMode request
pub const CREATE_MODE_REQUEST: u8 = 16;
pub fn create_mode<'c, Conn>(conn: &'c Conn, window: xproto::Window, mode_info: ModeInfo, name: &[u8]) -> Result<Cookie<'c, Conn, CreateModeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (40 + 1 * name.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let mode_info_bytes = mode_info.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_MODE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        mode_info_bytes[0],
        mode_info_bytes[1],
        mode_info_bytes[2],
        mode_info_bytes[3],
        mode_info_bytes[4],
        mode_info_bytes[5],
        mode_info_bytes[6],
        mode_info_bytes[7],
        mode_info_bytes[8],
        mode_info_bytes[9],
        mode_info_bytes[10],
        mode_info_bytes[11],
        mode_info_bytes[12],
        mode_info_bytes[13],
        mode_info_bytes[14],
        mode_info_bytes[15],
        mode_info_bytes[16],
        mode_info_bytes[17],
        mode_info_bytes[18],
        mode_info_bytes[19],
        mode_info_bytes[20],
        mode_info_bytes[21],
        mode_info_bytes[22],
        mode_info_bytes[23],
        mode_info_bytes[24],
        mode_info_bytes[25],
        mode_info_bytes[26],
        mode_info_bytes[27],
        mode_info_bytes[28],
        mode_info_bytes[29],
        mode_info_bytes[30],
        mode_info_bytes[31],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateModeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub mode: Mode,
}
impl CreateModeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (mode, remaining) = Mode::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = CreateModeReply { response_type, sequence, length, mode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CreateModeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DestroyMode request
pub const DESTROY_MODE_REQUEST: u8 = 17;
pub fn destroy_mode<Conn>(conn: &Conn, mode: Mode) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let mode_bytes = mode.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_MODE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        mode_bytes[0],
        mode_bytes[1],
        mode_bytes[2],
        mode_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the AddOutputMode request
pub const ADD_OUTPUT_MODE_REQUEST: u8 = 18;
pub fn add_output_mode<Conn>(conn: &Conn, output: Output, mode: Mode) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let mode_bytes = mode.serialize();
    let request0 = [
        extension_information.major_opcode,
        ADD_OUTPUT_MODE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
        mode_bytes[0],
        mode_bytes[1],
        mode_bytes[2],
        mode_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the DeleteOutputMode request
pub const DELETE_OUTPUT_MODE_REQUEST: u8 = 19;
pub fn delete_output_mode<Conn>(conn: &Conn, output: Output, mode: Mode) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let output_bytes = output.serialize();
    let mode_bytes = mode.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_OUTPUT_MODE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
        mode_bytes[0],
        mode_bytes[1],
        mode_bytes[2],
        mode_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetCrtcInfo request
pub const GET_CRTC_INFO_REQUEST: u8 = 20;
pub fn get_crtc_info<Conn>(conn: &Conn, crtc: Crtc, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Conn, GetCrtcInfoReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let config_timestamp_bytes = config_timestamp.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CRTC_INFO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
        config_timestamp_bytes[0],
        config_timestamp_bytes[1],
        config_timestamp_bytes[2],
        config_timestamp_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCrtcInfoReply {
    pub response_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub mode: Mode,
    pub rotation: u16,
    pub rotations: u16,
    pub outputs: Vec<Output>,
    pub possible: Vec<Output>,
}
impl GetCrtcInfoReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (mode, remaining) = Mode::try_parse(remaining)?;
        let (rotation, remaining) = u16::try_parse(remaining)?;
        let (rotations, remaining) = u16::try_parse(remaining)?;
        let (num_outputs, remaining) = u16::try_parse(remaining)?;
        let (num_possible_outputs, remaining) = u16::try_parse(remaining)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs as usize)?;
        let (possible, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_possible_outputs as usize)?;
        let status = status.try_into()?;
        let result = GetCrtcInfoReply { response_type, status, sequence, length, timestamp, x, y, width, height, mode, rotation, rotations, outputs, possible };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCrtcInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetCrtcConfig request
pub const SET_CRTC_CONFIG_REQUEST: u8 = 21;
pub fn set_crtc_config<'c, Conn>(conn: &'c Conn, crtc: Crtc, timestamp: xproto::Timestamp, config_timestamp: xproto::Timestamp, x: i16, y: i16, mode: Mode, rotation: u16, outputs: &[Output]) -> Result<Cookie<'c, Conn, SetCrtcConfigReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28 + 4 * outputs.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let timestamp_bytes = timestamp.serialize();
    let config_timestamp_bytes = config_timestamp.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let mode_bytes = mode.serialize();
    let rotation_bytes = rotation.serialize();
    let outputs_bytes = outputs.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_CRTC_CONFIG_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
        timestamp_bytes[0],
        timestamp_bytes[1],
        timestamp_bytes[2],
        timestamp_bytes[3],
        config_timestamp_bytes[0],
        config_timestamp_bytes[1],
        config_timestamp_bytes[2],
        config_timestamp_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
        mode_bytes[0],
        mode_bytes[1],
        mode_bytes[2],
        mode_bytes[3],
        rotation_bytes[0],
        rotation_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&outputs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&outputs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetCrtcConfigReply {
    pub response_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
}
impl SetCrtcConfigReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = SetCrtcConfigReply { response_type, status, sequence, length, timestamp };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetCrtcConfigReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetCrtcGammaSize request
pub const GET_CRTC_GAMMA_SIZE_REQUEST: u8 = 22;
pub fn get_crtc_gamma_size<Conn>(conn: &Conn, crtc: Crtc) -> Result<Cookie<'_, Conn, GetCrtcGammaSizeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CRTC_GAMMA_SIZE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrtcGammaSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
}
impl GetCrtcGammaSizeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let result = GetCrtcGammaSizeReply { response_type, sequence, length, size };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCrtcGammaSizeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetCrtcGamma request
pub const GET_CRTC_GAMMA_REQUEST: u8 = 23;
pub fn get_crtc_gamma<Conn>(conn: &Conn, crtc: Crtc) -> Result<Cookie<'_, Conn, GetCrtcGammaReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CRTC_GAMMA_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCrtcGammaReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub red: Vec<u16>,
    pub green: Vec<u16>,
    pub blue: Vec<u16>,
}
impl GetCrtcGammaReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (red, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size as usize)?;
        let (green, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size as usize)?;
        let (blue, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size as usize)?;
        let result = GetCrtcGammaReply { response_type, sequence, length, size, red, green, blue };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCrtcGammaReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetCrtcGamma request
pub const SET_CRTC_GAMMA_REQUEST: u8 = 24;
pub fn set_crtc_gamma<'c, Conn>(conn: &'c Conn, crtc: Crtc, size: u16, red: &[u16], green: &[u16], blue: &[u16]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 2 * red.len() + 2 * green.len() + 2 * blue.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let size_bytes = size.serialize();
    assert_eq!(red.len(), size as usize, "Argument red has an incorrect length");
    let red_bytes = red.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_CRTC_GAMMA_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
        size_bytes[0],
        size_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&red_bytes).len();
    assert_eq!(green.len(), size as usize, "Argument green has an incorrect length");
    let green_bytes = green.serialize();
    let length_so_far = length_so_far + (&green_bytes).len();
    assert_eq!(blue.len(), size as usize, "Argument blue has an incorrect length");
    let blue_bytes = blue.serialize();
    let length_so_far = length_so_far + (&blue_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&red_bytes), IoSlice::new(&green_bytes), IoSlice::new(&blue_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetScreenResourcesCurrent request
pub const GET_SCREEN_RESOURCES_CURRENT_REQUEST: u8 = 25;
pub fn get_screen_resources_current<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenResourcesCurrentReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SCREEN_RESOURCES_CURRENT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetScreenResourcesCurrentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub crtcs: Vec<Crtc>,
    pub outputs: Vec<Output>,
    pub modes: Vec<ModeInfo>,
    pub names: Vec<u8>,
}
impl GetScreenResourcesCurrentReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_crtcs, remaining) = u16::try_parse(remaining)?;
        let (num_outputs, remaining) = u16::try_parse(remaining)?;
        let (num_modes, remaining) = u16::try_parse(remaining)?;
        let (names_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs as usize)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs as usize)?;
        let (modes, remaining) = crate::x11_utils::parse_list::<ModeInfo>(remaining, num_modes as usize)?;
        let (names, remaining) = crate::x11_utils::parse_list::<u8>(remaining, names_len as usize)?;
        let result = GetScreenResourcesCurrentReply { response_type, sequence, length, timestamp, config_timestamp, crtcs, outputs, modes, names };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenResourcesCurrentReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Transform {
    Unit = 1 << 0,
    ScaleUp = 1 << 1,
    ScaleDown = 1 << 2,
    Projective = 1 << 3,
}
impl From<Transform> for u8 {
    fn from(input: Transform) -> Self {
        match input {
            Transform::Unit => 1 << 0,
            Transform::ScaleUp => 1 << 1,
            Transform::ScaleDown => 1 << 2,
            Transform::Projective => 1 << 3,
        }
    }
}
impl From<Transform> for Option<u8> {
    fn from(input: Transform) -> Self {
        Some(u8::from(input))
    }
}
impl From<Transform> for u16 {
    fn from(input: Transform) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Transform> for Option<u16> {
    fn from(input: Transform) -> Self {
        Some(u16::from(input))
    }
}
impl From<Transform> for u32 {
    fn from(input: Transform) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Transform> for Option<u32> {
    fn from(input: Transform) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Transform {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Transform::Unit),
            2 => Ok(Transform::ScaleUp),
            4 => Ok(Transform::ScaleDown),
            8 => Ok(Transform::Projective),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Transform {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Transform {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(Transform, u8);

/// Opcode for the SetCrtcTransform request
pub const SET_CRTC_TRANSFORM_REQUEST: u8 = 26;
pub fn set_crtc_transform<'c, Conn>(conn: &'c Conn, crtc: Crtc, transform: render::Transform, filter_name: &[u8], filter_params: &[render::Fixed]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (48 + 1 * filter_name.len() + 4 * filter_params.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let transform_bytes = transform.serialize();
    let filter_len: u16 = filter_name.len().try_into()?;
    let filter_len_bytes = filter_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_CRTC_TRANSFORM_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
        transform_bytes[0],
        transform_bytes[1],
        transform_bytes[2],
        transform_bytes[3],
        transform_bytes[4],
        transform_bytes[5],
        transform_bytes[6],
        transform_bytes[7],
        transform_bytes[8],
        transform_bytes[9],
        transform_bytes[10],
        transform_bytes[11],
        transform_bytes[12],
        transform_bytes[13],
        transform_bytes[14],
        transform_bytes[15],
        transform_bytes[16],
        transform_bytes[17],
        transform_bytes[18],
        transform_bytes[19],
        transform_bytes[20],
        transform_bytes[21],
        transform_bytes[22],
        transform_bytes[23],
        transform_bytes[24],
        transform_bytes[25],
        transform_bytes[26],
        transform_bytes[27],
        transform_bytes[28],
        transform_bytes[29],
        transform_bytes[30],
        transform_bytes[31],
        transform_bytes[32],
        transform_bytes[33],
        transform_bytes[34],
        transform_bytes[35],
        filter_len_bytes[0],
        filter_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (filter_name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    let filter_params_bytes = filter_params.serialize();
    let length_so_far = length_so_far + (&filter_params_bytes).len();
    let padding2 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding2).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(filter_name), IoSlice::new(&padding1), IoSlice::new(&filter_params_bytes), IoSlice::new(&padding2)], Vec::new())?)
}

/// Opcode for the GetCrtcTransform request
pub const GET_CRTC_TRANSFORM_REQUEST: u8 = 27;
pub fn get_crtc_transform<Conn>(conn: &Conn, crtc: Crtc) -> Result<Cookie<'_, Conn, GetCrtcTransformReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CRTC_TRANSFORM_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCrtcTransformReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending_transform: render::Transform,
    pub has_transforms: bool,
    pub current_transform: render::Transform,
    pub pending_filter_name: Vec<u8>,
    pub pending_params: Vec<render::Fixed>,
    pub current_filter_name: Vec<u8>,
    pub current_params: Vec<render::Fixed>,
}
impl GetCrtcTransformReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pending_transform, remaining) = render::Transform::try_parse(remaining)?;
        let (has_transforms, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (current_transform, remaining) = render::Transform::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (pending_len, remaining) = u16::try_parse(remaining)?;
        let (pending_nparams, remaining) = u16::try_parse(remaining)?;
        let (current_len, remaining) = u16::try_parse(remaining)?;
        let (current_nparams, remaining) = u16::try_parse(remaining)?;
        let (pending_filter_name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, pending_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (pending_params, remaining) = crate::x11_utils::parse_list::<render::Fixed>(remaining, pending_nparams as usize)?;
        let (current_filter_name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, current_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (current_params, remaining) = crate::x11_utils::parse_list::<render::Fixed>(remaining, current_nparams as usize)?;
        let result = GetCrtcTransformReply { response_type, sequence, length, pending_transform, has_transforms, current_transform, pending_filter_name, pending_params, current_filter_name, current_params };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCrtcTransformReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPanning request
pub const GET_PANNING_REQUEST: u8 = 28;
pub fn get_panning<Conn>(conn: &Conn, crtc: Crtc) -> Result<Cookie<'_, Conn, GetPanningReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PANNING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPanningReply {
    pub response_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub track_left: u16,
    pub track_top: u16,
    pub track_width: u16,
    pub track_height: u16,
    pub border_left: i16,
    pub border_top: i16,
    pub border_right: i16,
    pub border_bottom: i16,
}
impl GetPanningReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (left, remaining) = u16::try_parse(remaining)?;
        let (top, remaining) = u16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (track_left, remaining) = u16::try_parse(remaining)?;
        let (track_top, remaining) = u16::try_parse(remaining)?;
        let (track_width, remaining) = u16::try_parse(remaining)?;
        let (track_height, remaining) = u16::try_parse(remaining)?;
        let (border_left, remaining) = i16::try_parse(remaining)?;
        let (border_top, remaining) = i16::try_parse(remaining)?;
        let (border_right, remaining) = i16::try_parse(remaining)?;
        let (border_bottom, remaining) = i16::try_parse(remaining)?;
        let status = status.try_into()?;
        let result = GetPanningReply { response_type, status, sequence, length, timestamp, left, top, width, height, track_left, track_top, track_width, track_height, border_left, border_top, border_right, border_bottom };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPanningReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetPanning request
pub const SET_PANNING_REQUEST: u8 = 29;
pub fn set_panning<Conn>(conn: &Conn, crtc: Crtc, timestamp: xproto::Timestamp, left: u16, top: u16, width: u16, height: u16, track_left: u16, track_top: u16, track_width: u16, track_height: u16, border_left: i16, border_top: i16, border_right: i16, border_bottom: i16) -> Result<Cookie<'_, Conn, SetPanningReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (36) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let crtc_bytes = crtc.serialize();
    let timestamp_bytes = timestamp.serialize();
    let left_bytes = left.serialize();
    let top_bytes = top.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let track_left_bytes = track_left.serialize();
    let track_top_bytes = track_top.serialize();
    let track_width_bytes = track_width.serialize();
    let track_height_bytes = track_height.serialize();
    let border_left_bytes = border_left.serialize();
    let border_top_bytes = border_top.serialize();
    let border_right_bytes = border_right.serialize();
    let border_bottom_bytes = border_bottom.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_PANNING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        crtc_bytes[0],
        crtc_bytes[1],
        crtc_bytes[2],
        crtc_bytes[3],
        timestamp_bytes[0],
        timestamp_bytes[1],
        timestamp_bytes[2],
        timestamp_bytes[3],
        left_bytes[0],
        left_bytes[1],
        top_bytes[0],
        top_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        track_left_bytes[0],
        track_left_bytes[1],
        track_top_bytes[0],
        track_top_bytes[1],
        track_width_bytes[0],
        track_width_bytes[1],
        track_height_bytes[0],
        track_height_bytes[1],
        border_left_bytes[0],
        border_left_bytes[1],
        border_top_bytes[0],
        border_top_bytes[1],
        border_right_bytes[0],
        border_right_bytes[1],
        border_bottom_bytes[0],
        border_bottom_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPanningReply {
    pub response_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
}
impl SetPanningReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let status = status.try_into()?;
        let result = SetPanningReply { response_type, status, sequence, length, timestamp };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetPanningReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetOutputPrimary request
pub const SET_OUTPUT_PRIMARY_REQUEST: u8 = 30;
pub fn set_output_primary<Conn>(conn: &Conn, window: xproto::Window, output: Output) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let output_bytes = output.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_OUTPUT_PRIMARY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetOutputPrimary request
pub const GET_OUTPUT_PRIMARY_REQUEST: u8 = 31;
pub fn get_output_primary<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetOutputPrimaryReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_OUTPUT_PRIMARY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOutputPrimaryReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub output: Output,
}
impl GetOutputPrimaryReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (output, remaining) = Output::try_parse(remaining)?;
        let result = GetOutputPrimaryReply { response_type, sequence, length, output };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetOutputPrimaryReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetProviders request
pub const GET_PROVIDERS_REQUEST: u8 = 32;
pub fn get_providers<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetProvidersReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PROVIDERS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProvidersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub providers: Vec<Provider>,
}
impl GetProvidersReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_providers, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let (providers, remaining) = crate::x11_utils::parse_list::<Provider>(remaining, num_providers as usize)?;
        let result = GetProvidersReply { response_type, sequence, length, timestamp, providers };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetProvidersReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProviderCapability {
    SourceOutput = 1 << 0,
    SinkOutput = 1 << 1,
    SourceOffload = 1 << 2,
    SinkOffload = 1 << 3,
}
impl From<ProviderCapability> for u8 {
    fn from(input: ProviderCapability) -> Self {
        match input {
            ProviderCapability::SourceOutput => 1 << 0,
            ProviderCapability::SinkOutput => 1 << 1,
            ProviderCapability::SourceOffload => 1 << 2,
            ProviderCapability::SinkOffload => 1 << 3,
        }
    }
}
impl From<ProviderCapability> for Option<u8> {
    fn from(input: ProviderCapability) -> Self {
        Some(u8::from(input))
    }
}
impl From<ProviderCapability> for u16 {
    fn from(input: ProviderCapability) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ProviderCapability> for Option<u16> {
    fn from(input: ProviderCapability) -> Self {
        Some(u16::from(input))
    }
}
impl From<ProviderCapability> for u32 {
    fn from(input: ProviderCapability) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ProviderCapability> for Option<u32> {
    fn from(input: ProviderCapability) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ProviderCapability {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ProviderCapability::SourceOutput),
            2 => Ok(ProviderCapability::SinkOutput),
            4 => Ok(ProviderCapability::SourceOffload),
            8 => Ok(ProviderCapability::SinkOffload),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ProviderCapability {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ProviderCapability {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ProviderCapability, u8);

/// Opcode for the GetProviderInfo request
pub const GET_PROVIDER_INFO_REQUEST: u8 = 33;
pub fn get_provider_info<Conn>(conn: &Conn, provider: Provider, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Conn, GetProviderInfoReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let config_timestamp_bytes = config_timestamp.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PROVIDER_INFO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
        config_timestamp_bytes[0],
        config_timestamp_bytes[1],
        config_timestamp_bytes[2],
        config_timestamp_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProviderInfoReply {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub capabilities: u32,
    pub num_associated_providers: u16,
    pub crtcs: Vec<Crtc>,
    pub outputs: Vec<Output>,
    pub associated_providers: Vec<Provider>,
    pub associated_capability: Vec<u32>,
    pub name: Vec<u8>,
}
impl GetProviderInfoReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (capabilities, remaining) = u32::try_parse(remaining)?;
        let (num_crtcs, remaining) = u16::try_parse(remaining)?;
        let (num_outputs, remaining) = u16::try_parse(remaining)?;
        let (num_associated_providers, remaining) = u16::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs as usize)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs as usize)?;
        let (associated_providers, remaining) = crate::x11_utils::parse_list::<Provider>(remaining, num_associated_providers as usize)?;
        let (associated_capability, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_associated_providers as usize)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
        let result = GetProviderInfoReply { response_type, status, sequence, length, timestamp, capabilities, num_associated_providers, crtcs, outputs, associated_providers, associated_capability, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetProviderInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetProviderOffloadSink request
pub const SET_PROVIDER_OFFLOAD_SINK_REQUEST: u8 = 34;
pub fn set_provider_offload_sink<Conn>(conn: &Conn, provider: Provider, sink_provider: Provider, config_timestamp: xproto::Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let sink_provider_bytes = sink_provider.serialize();
    let config_timestamp_bytes = config_timestamp.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_PROVIDER_OFFLOAD_SINK_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
        sink_provider_bytes[0],
        sink_provider_bytes[1],
        sink_provider_bytes[2],
        sink_provider_bytes[3],
        config_timestamp_bytes[0],
        config_timestamp_bytes[1],
        config_timestamp_bytes[2],
        config_timestamp_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetProviderOutputSource request
pub const SET_PROVIDER_OUTPUT_SOURCE_REQUEST: u8 = 35;
pub fn set_provider_output_source<Conn>(conn: &Conn, provider: Provider, source_provider: Provider, config_timestamp: xproto::Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let source_provider_bytes = source_provider.serialize();
    let config_timestamp_bytes = config_timestamp.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_PROVIDER_OUTPUT_SOURCE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
        source_provider_bytes[0],
        source_provider_bytes[1],
        source_provider_bytes[2],
        source_provider_bytes[3],
        config_timestamp_bytes[0],
        config_timestamp_bytes[1],
        config_timestamp_bytes[2],
        config_timestamp_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ListProviderProperties request
pub const LIST_PROVIDER_PROPERTIES_REQUEST: u8 = 36;
pub fn list_provider_properties<Conn>(conn: &Conn, provider: Provider) -> Result<Cookie<'_, Conn, ListProviderPropertiesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_PROVIDER_PROPERTIES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListProviderPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Vec<xproto::Atom>,
}
impl ListProviderPropertiesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_atoms, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (atoms, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_atoms as usize)?;
        let result = ListProviderPropertiesReply { response_type, sequence, length, atoms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListProviderPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryProviderProperty request
pub const QUERY_PROVIDER_PROPERTY_REQUEST: u8 = 37;
pub fn query_provider_property<Conn>(conn: &Conn, provider: Provider, property: xproto::Atom) -> Result<Cookie<'_, Conn, QueryProviderPropertyReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let property_bytes = property.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_PROVIDER_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryProviderPropertyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub pending: bool,
    pub range: bool,
    pub immutable: bool,
    pub valid_values: Vec<i32>,
}
impl QueryProviderPropertyReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pending, remaining) = bool::try_parse(remaining)?;
        let (range, remaining) = bool::try_parse(remaining)?;
        let (immutable, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let (valid_values, remaining) = crate::x11_utils::parse_list::<i32>(remaining, length as usize)?;
        let result = QueryProviderPropertyReply { response_type, sequence, pending, range, immutable, valid_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryProviderPropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ConfigureProviderProperty request
pub const CONFIGURE_PROVIDER_PROPERTY_REQUEST: u8 = 38;
pub fn configure_provider_property<'c, Conn>(conn: &'c Conn, provider: Provider, property: xproto::Atom, pending: bool, range: bool, values: &[i32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16 + 4 * values.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let property_bytes = property.serialize();
    let pending_bytes = (pending as u8).serialize();
    let range_bytes = (range as u8).serialize();
    let values_bytes = values.serialize();
    let request0 = [
        extension_information.major_opcode,
        CONFIGURE_PROVIDER_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        pending_bytes[0],
        range_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&values_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&values_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ChangeProviderProperty request
pub const CHANGE_PROVIDER_PROPERTY_REQUEST: u8 = 39;
pub fn change_provider_property<'c, Conn>(conn: &'c Conn, provider: Provider, property: xproto::Atom, type_: xproto::Atom, format: u8, mode: u8, num_items: u32, data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24 + 1 * data.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let format_bytes = format.serialize();
    let mode_bytes = mode.serialize();
    let num_items_bytes = num_items.serialize();
    assert_eq!(data.len(), (num_items as usize) * ((format as usize) / (8)), "Argument data has an incorrect length");
    let request0 = [
        extension_information.major_opcode,
        CHANGE_PROVIDER_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        format_bytes[0],
        mode_bytes[0],
        0,
        0,
        num_items_bytes[0],
        num_items_bytes[1],
        num_items_bytes[2],
        num_items_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeleteProviderProperty request
pub const DELETE_PROVIDER_PROPERTY_REQUEST: u8 = 40;
pub fn delete_provider_property<Conn>(conn: &Conn, provider: Provider, property: xproto::Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let property_bytes = property.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_PROVIDER_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetProviderProperty request
pub const GET_PROVIDER_PROPERTY_REQUEST: u8 = 41;
pub fn get_provider_property<Conn>(conn: &Conn, provider: Provider, property: xproto::Atom, type_: xproto::Atom, long_offset: u32, long_length: u32, delete: bool, pending: bool) -> Result<Cookie<'_, Conn, GetProviderPropertyReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let provider_bytes = provider.serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let long_offset_bytes = long_offset.serialize();
    let long_length_bytes = long_length.serialize();
    let delete_bytes = (delete as u8).serialize();
    let pending_bytes = (pending as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PROVIDER_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        long_offset_bytes[0],
        long_offset_bytes[1],
        long_offset_bytes[2],
        long_offset_bytes[3],
        long_length_bytes[0],
        long_length_bytes[1],
        long_length_bytes[2],
        long_length_bytes[3],
        delete_bytes[0],
        pending_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProviderPropertyReply {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xproto::Atom,
    pub bytes_after: u32,
    pub num_items: u32,
    pub data: Vec<u8>,
}
impl GetProviderPropertyReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (num_items as usize) * ((format as usize) / (8)))?;
        let result = GetProviderPropertyReply { response_type, format, sequence, length, type_, bytes_after, num_items, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetProviderPropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ScreenChangeNotify event
pub const SCREEN_CHANGE_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenChangeNotifyEvent {
    pub response_type: u8,
    pub rotation: u8,
    pub sequence: u16,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub root: xproto::Window,
    pub request_window: xproto::Window,
    pub size_id: u16,
    pub subpixel_order: render::SubPixel,
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}
impl ScreenChangeNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (rotation, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (request_window, remaining) = xproto::Window::try_parse(remaining)?;
        let (size_id, remaining) = u16::try_parse(remaining)?;
        let (subpixel_order, remaining) = u16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (mwidth, remaining) = u16::try_parse(remaining)?;
        let (mheight, remaining) = u16::try_parse(remaining)?;
        let subpixel_order = subpixel_order.try_into()?;
        let result = ScreenChangeNotifyEvent { response_type, rotation, sequence, timestamp, config_timestamp, root, request_window, size_id, subpixel_order, width, height, mwidth, mheight };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ScreenChangeNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ScreenChangeNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ScreenChangeNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ScreenChangeNotifyEvent> for [u8; 32] {
    fn from(input: &ScreenChangeNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let rotation = input.rotation.serialize();
        let sequence = input.sequence.serialize();
        let timestamp = input.timestamp.serialize();
        let config_timestamp = input.config_timestamp.serialize();
        let root = input.root.serialize();
        let request_window = input.request_window.serialize();
        let size_id = input.size_id.serialize();
        let subpixel_order = Into::<u16>::into(input.subpixel_order).serialize();
        let width = input.width.serialize();
        let height = input.height.serialize();
        let mwidth = input.mwidth.serialize();
        let mheight = input.mheight.serialize();
        [
            response_type[0], rotation[0], sequence[0], sequence[1], timestamp[0], timestamp[1], timestamp[2], timestamp[3],
            config_timestamp[0], config_timestamp[1], config_timestamp[2], config_timestamp[3], root[0], root[1], root[2], root[3],
            request_window[0], request_window[1], request_window[2], request_window[3], size_id[0], size_id[1], subpixel_order[0], subpixel_order[1],
            width[0], width[1], height[0], height[1], mwidth[0], mwidth[1], mheight[0], mheight[1]
        ]
    }
}
impl From<ScreenChangeNotifyEvent> for [u8; 32] {
    fn from(input: ScreenChangeNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Notify {
    CrtcChange = 0,
    OutputChange = 1,
    OutputProperty = 2,
    ProviderChange = 3,
    ProviderProperty = 4,
    ResourceChange = 5,
    Lease = 6,
}
impl From<Notify> for u8 {
    fn from(input: Notify) -> Self {
        match input {
            Notify::CrtcChange => 0,
            Notify::OutputChange => 1,
            Notify::OutputProperty => 2,
            Notify::ProviderChange => 3,
            Notify::ProviderProperty => 4,
            Notify::ResourceChange => 5,
            Notify::Lease => 6,
        }
    }
}
impl From<Notify> for Option<u8> {
    fn from(input: Notify) -> Self {
        Some(u8::from(input))
    }
}
impl From<Notify> for u16 {
    fn from(input: Notify) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Notify> for Option<u16> {
    fn from(input: Notify) -> Self {
        Some(u16::from(input))
    }
}
impl From<Notify> for u32 {
    fn from(input: Notify) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Notify> for Option<u32> {
    fn from(input: Notify) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Notify {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Notify::CrtcChange),
            1 => Ok(Notify::OutputChange),
            2 => Ok(Notify::OutputProperty),
            3 => Ok(Notify::ProviderChange),
            4 => Ok(Notify::ProviderProperty),
            5 => Ok(Notify::ResourceChange),
            6 => Ok(Notify::Lease),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Notify {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Notify {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrtcChange {
    pub timestamp: xproto::Timestamp,
    pub window: xproto::Window,
    pub crtc: Crtc,
    pub mode: Mode,
    pub rotation: u16,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
impl TryParse for CrtcChange {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (crtc, remaining) = Crtc::try_parse(remaining)?;
        let (mode, remaining) = Mode::try_parse(remaining)?;
        let (rotation, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let result = CrtcChange { timestamp, window, crtc, mode, rotation, x, y, width, height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CrtcChange {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for CrtcChange {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let timestamp_bytes = self.timestamp.serialize();
        let window_bytes = self.window.serialize();
        let crtc_bytes = self.crtc.serialize();
        let mode_bytes = self.mode.serialize();
        let rotation_bytes = self.rotation.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
            mode_bytes[0],
            mode_bytes[1],
            mode_bytes[2],
            mode_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            0,
            0,
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(28);
        self.timestamp.serialize_into(bytes);
        self.window.serialize_into(bytes);
        self.crtc.serialize_into(bytes);
        self.mode.serialize_into(bytes);
        self.rotation.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputChange {
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub window: xproto::Window,
    pub output: Output,
    pub crtc: Crtc,
    pub mode: Mode,
    pub rotation: u16,
    pub connection: Connection,
    pub subpixel_order: render::SubPixel,
}
impl TryParse for OutputChange {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (output, remaining) = Output::try_parse(remaining)?;
        let (crtc, remaining) = Crtc::try_parse(remaining)?;
        let (mode, remaining) = Mode::try_parse(remaining)?;
        let (rotation, remaining) = u16::try_parse(remaining)?;
        let (connection, remaining) = u8::try_parse(remaining)?;
        let (subpixel_order, remaining) = u8::try_parse(remaining)?;
        let connection = connection.try_into()?;
        let subpixel_order = subpixel_order.try_into()?;
        let result = OutputChange { timestamp, config_timestamp, window, output, crtc, mode, rotation, connection, subpixel_order };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for OutputChange {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for OutputChange {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let timestamp_bytes = self.timestamp.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let window_bytes = self.window.serialize();
        let output_bytes = self.output.serialize();
        let crtc_bytes = self.crtc.serialize();
        let mode_bytes = self.mode.serialize();
        let rotation_bytes = self.rotation.serialize();
        let connection_bytes = Into::<u8>::into(self.connection).serialize();
        let subpixel_order_bytes = Into::<u8>::into(self.subpixel_order).serialize();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            config_timestamp_bytes[0],
            config_timestamp_bytes[1],
            config_timestamp_bytes[2],
            config_timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
            mode_bytes[0],
            mode_bytes[1],
            mode_bytes[2],
            mode_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            connection_bytes[0],
            subpixel_order_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(28);
        self.timestamp.serialize_into(bytes);
        self.config_timestamp.serialize_into(bytes);
        self.window.serialize_into(bytes);
        self.output.serialize_into(bytes);
        self.crtc.serialize_into(bytes);
        self.mode.serialize_into(bytes);
        self.rotation.serialize_into(bytes);
        Into::<u8>::into(self.connection).serialize_into(bytes);
        Into::<u8>::into(self.subpixel_order).serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputProperty {
    pub window: xproto::Window,
    pub output: Output,
    pub atom: xproto::Atom,
    pub timestamp: xproto::Timestamp,
    pub status: xproto::Property,
}
impl TryParse for OutputProperty {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (output, remaining) = Output::try_parse(remaining)?;
        let (atom, remaining) = xproto::Atom::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = OutputProperty { window, output, atom, timestamp, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for OutputProperty {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for OutputProperty {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let window_bytes = self.window.serialize();
        let output_bytes = self.output.serialize();
        let atom_bytes = self.atom.serialize();
        let timestamp_bytes = self.timestamp.serialize();
        let status_bytes = Into::<u8>::into(self.status).serialize();
        [
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            atom_bytes[0],
            atom_bytes[1],
            atom_bytes[2],
            atom_bytes[3],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            status_bytes[0],
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
        bytes.reserve(28);
        self.window.serialize_into(bytes);
        self.output.serialize_into(bytes);
        self.atom.serialize_into(bytes);
        self.timestamp.serialize_into(bytes);
        Into::<u8>::into(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 11]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderChange {
    pub timestamp: xproto::Timestamp,
    pub window: xproto::Window,
    pub provider: Provider,
}
impl TryParse for ProviderChange {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (provider, remaining) = Provider::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let result = ProviderChange { timestamp, window, provider };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ProviderChange {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ProviderChange {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let timestamp_bytes = self.timestamp.serialize();
        let window_bytes = self.window.serialize();
        let provider_bytes = self.provider.serialize();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
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
        bytes.reserve(28);
        self.timestamp.serialize_into(bytes);
        self.window.serialize_into(bytes);
        self.provider.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderProperty {
    pub window: xproto::Window,
    pub provider: Provider,
    pub atom: xproto::Atom,
    pub timestamp: xproto::Timestamp,
    pub state: u8,
}
impl TryParse for ProviderProperty {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (provider, remaining) = Provider::try_parse(remaining)?;
        let (atom, remaining) = xproto::Atom::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::ParseError)?;
        let result = ProviderProperty { window, provider, atom, timestamp, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ProviderProperty {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ProviderProperty {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let window_bytes = self.window.serialize();
        let provider_bytes = self.provider.serialize();
        let atom_bytes = self.atom.serialize();
        let timestamp_bytes = self.timestamp.serialize();
        let state_bytes = self.state.serialize();
        [
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
            atom_bytes[0],
            atom_bytes[1],
            atom_bytes[2],
            atom_bytes[3],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            state_bytes[0],
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
        bytes.reserve(28);
        self.window.serialize_into(bytes);
        self.provider.serialize_into(bytes);
        self.atom.serialize_into(bytes);
        self.timestamp.serialize_into(bytes);
        self.state.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 11]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceChange {
    pub timestamp: xproto::Timestamp,
    pub window: xproto::Window,
}
impl TryParse for ResourceChange {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = ResourceChange { timestamp, window };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ResourceChange {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ResourceChange {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let timestamp_bytes = self.timestamp.serialize();
        let window_bytes = self.window.serialize();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
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
        bytes.reserve(28);
        self.timestamp.serialize_into(bytes);
        self.window.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorInfo {
    pub name: xproto::Atom,
    pub primary: bool,
    pub automatic: bool,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub width_in_millimeters: u32,
    pub height_in_millimeters: u32,
    pub outputs: Vec<Output>,
}
impl TryParse for MonitorInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (primary, remaining) = bool::try_parse(remaining)?;
        let (automatic, remaining) = bool::try_parse(remaining)?;
        let (n_output, remaining) = u16::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (width_in_millimeters, remaining) = u32::try_parse(remaining)?;
        let (height_in_millimeters, remaining) = u32::try_parse(remaining)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, n_output as usize)?;
        let result = MonitorInfo { name, primary, automatic, x, y, width, height, width_in_millimeters, height_in_millimeters, outputs };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MonitorInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for MonitorInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.name.serialize_into(bytes);
        self.primary.serialize_into(bytes);
        self.automatic.serialize_into(bytes);
        let n_output = self.outputs.len() as u16;
        n_output.serialize_into(bytes);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.width_in_millimeters.serialize_into(bytes);
        self.height_in_millimeters.serialize_into(bytes);
        self.outputs.serialize_into(bytes);
    }
}

/// Opcode for the GetMonitors request
pub const GET_MONITORS_REQUEST: u8 = 42;
pub fn get_monitors<Conn>(conn: &Conn, window: xproto::Window, get_active: bool) -> Result<Cookie<'_, Conn, GetMonitorsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let get_active_bytes = (get_active as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MONITORS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        get_active_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMonitorsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub n_outputs: u32,
    pub monitors: Vec<MonitorInfo>,
}
impl GetMonitorsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (n_monitors, remaining) = u32::try_parse(remaining)?;
        let (n_outputs, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (monitors, remaining) = crate::x11_utils::parse_list::<MonitorInfo>(remaining, n_monitors as usize)?;
        let result = GetMonitorsReply { response_type, sequence, length, timestamp, n_outputs, monitors };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMonitorsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetMonitor request
pub const SET_MONITOR_REQUEST: u8 = 43;
pub fn set_monitor<Conn>(conn: &Conn, window: xproto::Window, monitorinfo: MonitorInfo) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let monitorinfo_bytes = monitorinfo.serialize();
    let length: usize = (8 + monitorinfo_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_MONITOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&monitorinfo_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&monitorinfo_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeleteMonitor request
pub const DELETE_MONITOR_REQUEST: u8 = 44;
pub fn delete_monitor<Conn>(conn: &Conn, window: xproto::Window, name: xproto::Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let name_bytes = name.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_MONITOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        name_bytes[0],
        name_bytes[1],
        name_bytes[2],
        name_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CreateLease request
pub const CREATE_LEASE_REQUEST: u8 = 45;
pub fn create_lease<'c, Conn>(conn: &'c Conn, window: xproto::Window, lid: Lease, crtcs: &[Crtc], outputs: &[Output]) -> Result<CookieWithFds<'c, Conn, CreateLeaseReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16 + 4 * crtcs.len() + 4 * outputs.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let lid_bytes = lid.serialize();
    let num_crtcs: u16 = crtcs.len().try_into()?;
    let num_crtcs_bytes = num_crtcs.serialize();
    let num_outputs: u16 = outputs.len().try_into()?;
    let num_outputs_bytes = num_outputs.serialize();
    let crtcs_bytes = crtcs.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_LEASE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        lid_bytes[0],
        lid_bytes[1],
        lid_bytes[2],
        lid_bytes[3],
        num_crtcs_bytes[0],
        num_crtcs_bytes[1],
        num_outputs_bytes[0],
        num_outputs_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&crtcs_bytes).len();
    let outputs_bytes = outputs.serialize();
    let length_so_far = length_so_far + (&outputs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply_with_fds(&[IoSlice::new(&request0), IoSlice::new(&crtcs_bytes), IoSlice::new(&outputs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, PartialEq, Eq)]
pub struct CreateLeaseReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub master_fd: RawFdContainer,
}
impl CreateLeaseReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::ParseError) }
        let master_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let result = CreateLeaseReply { response_type, nfd, sequence, length, master_fd };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for CreateLeaseReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

/// Opcode for the FreeLease request
pub const FREE_LEASE_REQUEST: u8 = 46;
pub fn free_lease<Conn>(conn: &Conn, lid: Lease, terminate: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let lid_bytes = lid.serialize();
    let terminate_bytes = terminate.serialize();
    let request0 = [
        extension_information.major_opcode,
        FREE_LEASE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        lid_bytes[0],
        lid_bytes[1],
        lid_bytes[2],
        lid_bytes[3],
        terminate_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeaseNotify {
    pub timestamp: xproto::Timestamp,
    pub window: xproto::Window,
    pub lease: Lease,
    pub created: u8,
}
impl TryParse for LeaseNotify {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (lease, remaining) = Lease::try_parse(remaining)?;
        let (created, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::ParseError)?;
        let result = LeaseNotify { timestamp, window, lease, created };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for LeaseNotify {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for LeaseNotify {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let timestamp_bytes = self.timestamp.serialize();
        let window_bytes = self.window.serialize();
        let lease_bytes = self.lease.serialize();
        let created_bytes = self.created.serialize();
        [
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            lease_bytes[0],
            lease_bytes[1],
            lease_bytes[2],
            lease_bytes[3],
            created_bytes[0],
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
        bytes.reserve(28);
        self.timestamp.serialize_into(bytes);
        self.window.serialize_into(bytes);
        self.lease.serialize_into(bytes);
        self.created.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 15]);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NotifyData([u8; 28]);
impl NotifyData {
    pub fn as_xproto_cc(&self) -> CrtcChange {
        fn do_the_parse(remaining: &[u8]) -> Result<CrtcChange, ParseError> {
            let (cc, remaining) = CrtcChange::try_parse(remaining)?;
            let _ = remaining;
            Ok(cc)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_oc(&self) -> OutputChange {
        fn do_the_parse(remaining: &[u8]) -> Result<OutputChange, ParseError> {
            let (oc, remaining) = OutputChange::try_parse(remaining)?;
            let _ = remaining;
            Ok(oc)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_op(&self) -> OutputProperty {
        fn do_the_parse(remaining: &[u8]) -> Result<OutputProperty, ParseError> {
            let (op, remaining) = OutputProperty::try_parse(remaining)?;
            let _ = remaining;
            Ok(op)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_pc(&self) -> ProviderChange {
        fn do_the_parse(remaining: &[u8]) -> Result<ProviderChange, ParseError> {
            let (pc, remaining) = ProviderChange::try_parse(remaining)?;
            let _ = remaining;
            Ok(pc)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_pp(&self) -> ProviderProperty {
        fn do_the_parse(remaining: &[u8]) -> Result<ProviderProperty, ParseError> {
            let (pp, remaining) = ProviderProperty::try_parse(remaining)?;
            let _ = remaining;
            Ok(pp)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_rc(&self) -> ResourceChange {
        fn do_the_parse(remaining: &[u8]) -> Result<ResourceChange, ParseError> {
            let (rc, remaining) = ResourceChange::try_parse(remaining)?;
            let _ = remaining;
            Ok(rc)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_lc(&self) -> LeaseNotify {
        fn do_the_parse(remaining: &[u8]) -> Result<LeaseNotify, ParseError> {
            let (lc, remaining) = LeaseNotify::try_parse(remaining)?;
            let _ = remaining;
            Ok(lc)
        }
        do_the_parse(&self.0).unwrap()
    }
}
impl Serialize for NotifyData {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        self.0
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}
impl TryParse for NotifyData {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner: [u8; 28] = value.get(..28)
            .ok_or(ParseError::ParseError)?
            .try_into()
            .unwrap();
        let result = NotifyData(inner);
        Ok((result, &value[28..]))
    }
}
impl From<CrtcChange> for NotifyData {
    fn from(value: CrtcChange) -> Self {
        Self(value.serialize())
    }
}
impl From<OutputChange> for NotifyData {
    fn from(value: OutputChange) -> Self {
        Self(value.serialize())
    }
}
impl From<OutputProperty> for NotifyData {
    fn from(value: OutputProperty) -> Self {
        Self(value.serialize())
    }
}
impl From<ProviderChange> for NotifyData {
    fn from(value: ProviderChange) -> Self {
        Self(value.serialize())
    }
}
impl From<ProviderProperty> for NotifyData {
    fn from(value: ProviderProperty) -> Self {
        Self(value.serialize())
    }
}
impl From<ResourceChange> for NotifyData {
    fn from(value: ResourceChange) -> Self {
        Self(value.serialize())
    }
}
impl From<LeaseNotify> for NotifyData {
    fn from(value: LeaseNotify) -> Self {
        Self(value.serialize())
    }
}

/// Opcode for the Notify event
pub const NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy)]
pub struct NotifyEvent {
    pub response_type: u8,
    pub sub_code: Notify,
    pub sequence: u16,
    pub u: NotifyData,
}
impl NotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (sub_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (u, remaining) = NotifyData::try_parse(remaining)?;
        let sub_code = sub_code.try_into()?;
        let result = NotifyEvent { response_type, sub_code, sequence, u };
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
        let response_type = input.response_type.serialize();
        let sub_code = Into::<u8>::into(input.sub_code).serialize();
        let sequence = input.sequence.serialize();
        let u = input.u.serialize();
        [
            response_type[0], sub_code[0], sequence[0], sequence[1], u[0], u[1], u[2], u[3],
            u[4], u[5], u[6], u[7], u[8], u[9], u[10], u[11],
            u[12], u[13], u[14], u[15], u[16], u[17], u[18], u[19],
            u[20], u[21], u[22], u[23], u[24], u[25], u[26], u[27]
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
    fn randr_query_version(&self, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }

    fn randr_set_screen_config(&self, window: xproto::Window, timestamp: xproto::Timestamp, config_timestamp: xproto::Timestamp, size_id: u16, rotation: u16, rate: u16) -> Result<Cookie<'_, Self, SetScreenConfigReply>, ConnectionError>
    {
        set_screen_config(self, window, timestamp, config_timestamp, size_id, rotation, rate)
    }

    fn randr_select_input(&self, window: xproto::Window, enable: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_input(self, window, enable)
    }

    fn randr_get_screen_info(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetScreenInfoReply>, ConnectionError>
    {
        get_screen_info(self, window)
    }

    fn randr_get_screen_size_range(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetScreenSizeRangeReply>, ConnectionError>
    {
        get_screen_size_range(self, window)
    }

    fn randr_set_screen_size(&self, window: xproto::Window, width: u16, height: u16, mm_width: u32, mm_height: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_screen_size(self, window, width, height, mm_width, mm_height)
    }

    fn randr_get_screen_resources(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetScreenResourcesReply>, ConnectionError>
    {
        get_screen_resources(self, window)
    }

    fn randr_get_output_info(&self, output: Output, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Self, GetOutputInfoReply>, ConnectionError>
    {
        get_output_info(self, output, config_timestamp)
    }

    fn randr_list_output_properties(&self, output: Output) -> Result<Cookie<'_, Self, ListOutputPropertiesReply>, ConnectionError>
    {
        list_output_properties(self, output)
    }

    fn randr_query_output_property(&self, output: Output, property: xproto::Atom) -> Result<Cookie<'_, Self, QueryOutputPropertyReply>, ConnectionError>
    {
        query_output_property(self, output, property)
    }

    fn randr_configure_output_property<'c>(&'c self, output: Output, property: xproto::Atom, pending: bool, range: bool, values: &[i32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        configure_output_property(self, output, property, pending, range, values)
    }

    fn randr_change_output_property<'c, A>(&'c self, output: Output, property: xproto::Atom, type_: xproto::Atom, format: u8, mode: A, num_units: u32, data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        change_output_property(self, output, property, type_, format, mode, num_units, data)
    }

    fn randr_delete_output_property(&self, output: Output, property: xproto::Atom) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_output_property(self, output, property)
    }

    fn randr_get_output_property(&self, output: Output, property: xproto::Atom, type_: xproto::Atom, long_offset: u32, long_length: u32, delete: bool, pending: bool) -> Result<Cookie<'_, Self, GetOutputPropertyReply>, ConnectionError>
    {
        get_output_property(self, output, property, type_, long_offset, long_length, delete, pending)
    }

    fn randr_create_mode<'c>(&'c self, window: xproto::Window, mode_info: ModeInfo, name: &[u8]) -> Result<Cookie<'c, Self, CreateModeReply>, ConnectionError>
    {
        create_mode(self, window, mode_info, name)
    }

    fn randr_destroy_mode(&self, mode: Mode) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_mode(self, mode)
    }

    fn randr_add_output_mode(&self, output: Output, mode: Mode) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        add_output_mode(self, output, mode)
    }

    fn randr_delete_output_mode(&self, output: Output, mode: Mode) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_output_mode(self, output, mode)
    }

    fn randr_get_crtc_info(&self, crtc: Crtc, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Self, GetCrtcInfoReply>, ConnectionError>
    {
        get_crtc_info(self, crtc, config_timestamp)
    }

    fn randr_set_crtc_config<'c>(&'c self, crtc: Crtc, timestamp: xproto::Timestamp, config_timestamp: xproto::Timestamp, x: i16, y: i16, mode: Mode, rotation: u16, outputs: &[Output]) -> Result<Cookie<'c, Self, SetCrtcConfigReply>, ConnectionError>
    {
        set_crtc_config(self, crtc, timestamp, config_timestamp, x, y, mode, rotation, outputs)
    }

    fn randr_get_crtc_gamma_size(&self, crtc: Crtc) -> Result<Cookie<'_, Self, GetCrtcGammaSizeReply>, ConnectionError>
    {
        get_crtc_gamma_size(self, crtc)
    }

    fn randr_get_crtc_gamma(&self, crtc: Crtc) -> Result<Cookie<'_, Self, GetCrtcGammaReply>, ConnectionError>
    {
        get_crtc_gamma(self, crtc)
    }

    fn randr_set_crtc_gamma<'c>(&'c self, crtc: Crtc, size: u16, red: &[u16], green: &[u16], blue: &[u16]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_crtc_gamma(self, crtc, size, red, green, blue)
    }

    fn randr_get_screen_resources_current(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetScreenResourcesCurrentReply>, ConnectionError>
    {
        get_screen_resources_current(self, window)
    }

    fn randr_set_crtc_transform<'c>(&'c self, crtc: Crtc, transform: render::Transform, filter_name: &[u8], filter_params: &[render::Fixed]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_crtc_transform(self, crtc, transform, filter_name, filter_params)
    }

    fn randr_get_crtc_transform(&self, crtc: Crtc) -> Result<Cookie<'_, Self, GetCrtcTransformReply>, ConnectionError>
    {
        get_crtc_transform(self, crtc)
    }

    fn randr_get_panning(&self, crtc: Crtc) -> Result<Cookie<'_, Self, GetPanningReply>, ConnectionError>
    {
        get_panning(self, crtc)
    }

    fn randr_set_panning(&self, crtc: Crtc, timestamp: xproto::Timestamp, left: u16, top: u16, width: u16, height: u16, track_left: u16, track_top: u16, track_width: u16, track_height: u16, border_left: i16, border_top: i16, border_right: i16, border_bottom: i16) -> Result<Cookie<'_, Self, SetPanningReply>, ConnectionError>
    {
        set_panning(self, crtc, timestamp, left, top, width, height, track_left, track_top, track_width, track_height, border_left, border_top, border_right, border_bottom)
    }

    fn randr_set_output_primary(&self, window: xproto::Window, output: Output) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_output_primary(self, window, output)
    }

    fn randr_get_output_primary(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetOutputPrimaryReply>, ConnectionError>
    {
        get_output_primary(self, window)
    }

    fn randr_get_providers(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetProvidersReply>, ConnectionError>
    {
        get_providers(self, window)
    }

    fn randr_get_provider_info(&self, provider: Provider, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Self, GetProviderInfoReply>, ConnectionError>
    {
        get_provider_info(self, provider, config_timestamp)
    }

    fn randr_set_provider_offload_sink(&self, provider: Provider, sink_provider: Provider, config_timestamp: xproto::Timestamp) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_provider_offload_sink(self, provider, sink_provider, config_timestamp)
    }

    fn randr_set_provider_output_source(&self, provider: Provider, source_provider: Provider, config_timestamp: xproto::Timestamp) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_provider_output_source(self, provider, source_provider, config_timestamp)
    }

    fn randr_list_provider_properties(&self, provider: Provider) -> Result<Cookie<'_, Self, ListProviderPropertiesReply>, ConnectionError>
    {
        list_provider_properties(self, provider)
    }

    fn randr_query_provider_property(&self, provider: Provider, property: xproto::Atom) -> Result<Cookie<'_, Self, QueryProviderPropertyReply>, ConnectionError>
    {
        query_provider_property(self, provider, property)
    }

    fn randr_configure_provider_property<'c>(&'c self, provider: Provider, property: xproto::Atom, pending: bool, range: bool, values: &[i32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        configure_provider_property(self, provider, property, pending, range, values)
    }

    fn randr_change_provider_property<'c>(&'c self, provider: Provider, property: xproto::Atom, type_: xproto::Atom, format: u8, mode: u8, num_items: u32, data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_provider_property(self, provider, property, type_, format, mode, num_items, data)
    }

    fn randr_delete_provider_property(&self, provider: Provider, property: xproto::Atom) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_provider_property(self, provider, property)
    }

    fn randr_get_provider_property(&self, provider: Provider, property: xproto::Atom, type_: xproto::Atom, long_offset: u32, long_length: u32, delete: bool, pending: bool) -> Result<Cookie<'_, Self, GetProviderPropertyReply>, ConnectionError>
    {
        get_provider_property(self, provider, property, type_, long_offset, long_length, delete, pending)
    }

    fn randr_get_monitors(&self, window: xproto::Window, get_active: bool) -> Result<Cookie<'_, Self, GetMonitorsReply>, ConnectionError>
    {
        get_monitors(self, window, get_active)
    }

    fn randr_set_monitor(&self, window: xproto::Window, monitorinfo: MonitorInfo) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_monitor(self, window, monitorinfo)
    }

    fn randr_delete_monitor(&self, window: xproto::Window, name: xproto::Atom) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_monitor(self, window, name)
    }

    fn randr_create_lease<'c>(&'c self, window: xproto::Window, lid: Lease, crtcs: &[Crtc], outputs: &[Output]) -> Result<CookieWithFds<'c, Self, CreateLeaseReply>, ConnectionError>
    {
        create_lease(self, window, lid, crtcs, outputs)
    }

    fn randr_free_lease(&self, lid: Lease, terminate: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_lease(self, lid, terminate)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
