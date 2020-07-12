// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `RandR` X11 extension.

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
use super::render;
use super::xproto;

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

/// Opcode for the BadCrtc error
pub const BAD_CRTC_ERROR: u8 = 1;

/// Opcode for the BadMode error
pub const BAD_MODE_ERROR: u8 = 2;

/// Opcode for the BadProvider error
pub const BAD_PROVIDER_ERROR: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for Rotation {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for Rotation {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
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
    fn serialize(&self) -> [u8; 8] {
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
        let (rates, remaining) = crate::x11_utils::parse_list::<u16>(remaining, n_rates.try_into().or(Err(ParseError::ConversionFailed))?)?;
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
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        let n_rates = u16::try_from(self.rates.len()).expect("`rates` has too many elements");
        n_rates.serialize_into(bytes);
        self.rates.serialize_into(bytes);
    }
}
impl RefreshRates {
    /// Get the value of the `nRates` field.
    ///
    /// The `nRates` field is used as the length field of the `rates` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_rates(&self) -> u16 {
        self.rates.len()
            .try_into().unwrap()
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub major_version: u32,
    pub minor_version: u32,
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
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            major_version_bytes[0],
            major_version_bytes[1],
            major_version_bytes[2],
            major_version_bytes[3],
            minor_version_bytes[0],
            minor_version_bytes[1],
            minor_version_bytes[2],
            minor_version_bytes[3],
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
        let (major_version, remaining) = u32::try_parse(value)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            major_version,
            minor_version,
        })
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major_version,
        minor_version,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for SetConfig {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for SetConfig {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}

/// Opcode for the SetScreenConfig request
pub const SET_SCREEN_CONFIG_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetScreenConfigRequest {
    pub window: xproto::Window,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub size_id: u16,
    pub rotation: u16,
    pub rate: u16,
}
impl SetScreenConfigRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let timestamp_bytes = self.timestamp.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let size_id_bytes = self.size_id.serialize();
        let rotation_bytes = self.rotation.serialize();
        let rate_bytes = self.rate.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_SCREEN_CONFIG_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, SetScreenConfigReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_SCREEN_CONFIG_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (size_id, remaining) = u16::try_parse(remaining)?;
        let (rotation, remaining) = u16::try_parse(remaining)?;
        let (rate, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(SetScreenConfigRequest {
            window,
            timestamp,
            config_timestamp,
            size_id,
            rotation,
            rate,
        })
    }
}
impl Request for SetScreenConfigRequest {
    type Reply = SetScreenConfigReply;
}
pub fn set_screen_config<Conn, A>(conn: &Conn, window: xproto::Window, timestamp: xproto::Timestamp, config_timestamp: xproto::Timestamp, size_id: u16, rotation: A, rate: u16) -> Result<Cookie<'_, Conn, SetScreenConfigReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
{
    let rotation: u16 = rotation.into();
    let request0 = SetScreenConfigRequest {
        window,
        timestamp,
        config_timestamp,
        size_id,
        rotation,
        rate,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetScreenConfigReply {
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub new_timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub root: xproto::Window,
    pub subpixel_order: render::SubPixel,
}
impl TryParse for SetScreenConfigReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (new_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (subpixel_order, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(10..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.try_into()?;
        let subpixel_order = subpixel_order.try_into()?;
        let result = SetScreenConfigReply { status, sequence, length, new_timestamp, config_timestamp, root, subpixel_order };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for NotifyMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for NotifyMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
bitmask_binop!(NotifyMask, u8);

/// Opcode for the SelectInput request
pub const SELECT_INPUT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectInputRequest {
    pub window: xproto::Window,
    pub enable: u16,
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
        let window_bytes = self.window.serialize();
        let enable_bytes = self.enable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SELECT_INPUT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            enable_bytes[0],
            enable_bytes[1],
            0,
            0,
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
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (enable, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(SelectInputRequest {
            window,
            enable,
        })
    }
}
impl Request for SelectInputRequest {
    type Reply = ();
}
pub fn select_input<Conn, A>(conn: &Conn, window: xproto::Window, enable: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
{
    let enable: u16 = enable.into();
    let request0 = SelectInputRequest {
        window,
        enable,
    };
    request0.send(conn)
}

/// Opcode for the GetScreenInfo request
pub const GET_SCREEN_INFO_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenInfoRequest {
    pub window: xproto::Window,
}
impl GetScreenInfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SCREEN_INFO_REQUEST,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetScreenInfoReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SCREEN_INFO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetScreenInfoRequest {
            window,
        })
    }
}
impl Request for GetScreenInfoRequest {
    type Reply = GetScreenInfoReply;
}
pub fn get_screen_info<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetScreenInfoRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetScreenInfoReply {
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
impl TryParse for GetScreenInfoReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (sizes, remaining) = crate::x11_utils::parse_list::<ScreenSize>(remaining, n_sizes.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (rates, remaining) = crate::x11_utils::parse_list::<RefreshRates>(remaining, u32::from(n_info).checked_sub(u32::from(n_sizes)).ok_or(ParseError::InvalidExpression)?.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetScreenInfoReply { rotations, sequence, length, root, timestamp, config_timestamp, size_id, rotation, rate, n_info, sizes, rates };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetScreenInfoReply {
    /// Get the value of the `nSizes` field.
    ///
    /// The `nSizes` field is used as the length field of the `sizes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_sizes(&self) -> u16 {
        self.sizes.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetScreenSizeRange request
pub const GET_SCREEN_SIZE_RANGE_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenSizeRangeRequest {
    pub window: xproto::Window,
}
impl GetScreenSizeRangeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SCREEN_SIZE_RANGE_REQUEST,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetScreenSizeRangeReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SCREEN_SIZE_RANGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetScreenSizeRangeRequest {
            window,
        })
    }
}
impl Request for GetScreenSizeRangeRequest {
    type Reply = GetScreenSizeRangeReply;
}
pub fn get_screen_size_range<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenSizeRangeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetScreenSizeRangeRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenSizeRangeReply {
    pub sequence: u16,
    pub length: u32,
    pub min_width: u16,
    pub min_height: u16,
    pub max_width: u16,
    pub max_height: u16,
}
impl TryParse for GetScreenSizeRangeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (min_width, remaining) = u16::try_parse(remaining)?;
        let (min_height, remaining) = u16::try_parse(remaining)?;
        let (max_width, remaining) = u16::try_parse(remaining)?;
        let (max_height, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetScreenSizeRangeReply { sequence, length, min_width, min_height, max_width, max_height };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetScreenSizeRequest {
    pub window: xproto::Window,
    pub width: u16,
    pub height: u16,
    pub mm_width: u32,
    pub mm_height: u32,
}
impl SetScreenSizeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let mm_width_bytes = self.mm_width.serialize();
        let mm_height_bytes = self.mm_height.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_SCREEN_SIZE_REQUEST,
            0,
            0,
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
        if header.minor_opcode != SET_SCREEN_SIZE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (mm_width, remaining) = u32::try_parse(remaining)?;
        let (mm_height, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetScreenSizeRequest {
            window,
            width,
            height,
            mm_width,
            mm_height,
        })
    }
}
impl Request for SetScreenSizeRequest {
    type Reply = ();
}
pub fn set_screen_size<Conn>(conn: &Conn, window: xproto::Window, width: u16, height: u16, mm_width: u32, mm_height: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetScreenSizeRequest {
        window,
        width,
        height,
        mm_width,
        mm_height,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u32> for ModeFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::InvalidValue))?)
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
    fn serialize(&self) -> [u8; 32] {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenResourcesRequest {
    pub window: xproto::Window,
}
impl GetScreenResourcesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SCREEN_RESOURCES_REQUEST,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetScreenResourcesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SCREEN_RESOURCES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetScreenResourcesRequest {
            window,
        })
    }
}
impl Request for GetScreenResourcesRequest {
    type Reply = GetScreenResourcesReply;
}
pub fn get_screen_resources<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenResourcesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetScreenResourcesRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetScreenResourcesReply {
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub crtcs: Vec<Crtc>,
    pub outputs: Vec<Output>,
    pub modes: Vec<ModeInfo>,
    pub names: Vec<u8>,
}
impl TryParse for GetScreenResourcesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_crtcs, remaining) = u16::try_parse(remaining)?;
        let (num_outputs, remaining) = u16::try_parse(remaining)?;
        let (num_modes, remaining) = u16::try_parse(remaining)?;
        let (names_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (modes, remaining) = crate::x11_utils::parse_list::<ModeInfo>(remaining, num_modes.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (names, remaining) = crate::x11_utils::parse_u8_list(remaining, names_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let names = names.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetScreenResourcesReply { sequence, length, timestamp, config_timestamp, crtcs, outputs, modes, names };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenResourcesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetScreenResourcesReply {
    /// Get the value of the `num_crtcs` field.
    ///
    /// The `num_crtcs` field is used as the length field of the `crtcs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_crtcs(&self) -> u16 {
        self.crtcs.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_outputs` field.
    ///
    /// The `num_outputs` field is used as the length field of the `outputs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_outputs(&self) -> u16 {
        self.outputs.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_modes` field.
    ///
    /// The `num_modes` field is used as the length field of the `modes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_modes(&self) -> u16 {
        self.modes.len()
            .try_into().unwrap()
    }
    /// Get the value of the `names_len` field.
    ///
    /// The `names_len` field is used as the length field of the `names` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn names_len(&self) -> u16 {
        self.names.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for Connection {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for Connection {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}

/// Opcode for the GetOutputInfo request
pub const GET_OUTPUT_INFO_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOutputInfoRequest {
    pub output: Output,
    pub config_timestamp: xproto::Timestamp,
}
impl GetOutputInfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_OUTPUT_INFO_REQUEST,
            0,
            0,
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            config_timestamp_bytes[0],
            config_timestamp_bytes[1],
            config_timestamp_bytes[2],
            config_timestamp_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetOutputInfoReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_OUTPUT_INFO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetOutputInfoRequest {
            output,
            config_timestamp,
        })
    }
}
impl Request for GetOutputInfoRequest {
    type Reply = GetOutputInfoReply;
}
pub fn get_output_info<Conn>(conn: &Conn, output: Output, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Conn, GetOutputInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetOutputInfoRequest {
        output,
        config_timestamp,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetOutputInfoReply {
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
impl TryParse for GetOutputInfoReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (modes, remaining) = crate::x11_utils::parse_list::<Mode>(remaining, num_modes.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (clones, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_clones.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let name = name.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.try_into()?;
        let connection = connection.try_into()?;
        let subpixel_order = subpixel_order.try_into()?;
        let result = GetOutputInfoReply { status, sequence, length, timestamp, crtc, mm_width, mm_height, connection, subpixel_order, num_preferred, crtcs, modes, clones, name };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetOutputInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetOutputInfoReply {
    /// Get the value of the `num_crtcs` field.
    ///
    /// The `num_crtcs` field is used as the length field of the `crtcs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_crtcs(&self) -> u16 {
        self.crtcs.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_modes` field.
    ///
    /// The `num_modes` field is used as the length field of the `modes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_modes(&self) -> u16 {
        self.modes.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_clones` field.
    ///
    /// The `num_clones` field is used as the length field of the `clones` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_clones(&self) -> u16 {
        self.clones.len()
            .try_into().unwrap()
    }
    /// Get the value of the `name_len` field.
    ///
    /// The `name_len` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn name_len(&self) -> u16 {
        self.name.len()
            .try_into().unwrap()
    }
}

/// Opcode for the ListOutputProperties request
pub const LIST_OUTPUT_PROPERTIES_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListOutputPropertiesRequest {
    pub output: Output,
}
impl ListOutputPropertiesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            LIST_OUTPUT_PROPERTIES_REQUEST,
            0,
            0,
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListOutputPropertiesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_OUTPUT_PROPERTIES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let _ = remaining;
        Ok(ListOutputPropertiesRequest {
            output,
        })
    }
}
impl Request for ListOutputPropertiesRequest {
    type Reply = ListOutputPropertiesReply;
}
pub fn list_output_properties<Conn>(conn: &Conn, output: Output) -> Result<Cookie<'_, Conn, ListOutputPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListOutputPropertiesRequest {
        output,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListOutputPropertiesReply {
    pub sequence: u16,
    pub length: u32,
    pub atoms: Vec<xproto::Atom>,
}
impl TryParse for ListOutputPropertiesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_atoms, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (atoms, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_atoms.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListOutputPropertiesReply { sequence, length, atoms };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListOutputPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl ListOutputPropertiesReply {
    /// Get the value of the `num_atoms` field.
    ///
    /// The `num_atoms` field is used as the length field of the `atoms` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_atoms(&self) -> u16 {
        self.atoms.len()
            .try_into().unwrap()
    }
}

/// Opcode for the QueryOutputProperty request
pub const QUERY_OUTPUT_PROPERTY_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryOutputPropertyRequest {
    pub output: Output,
    pub property: xproto::Atom,
}
impl QueryOutputPropertyRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_OUTPUT_PROPERTY_REQUEST,
            0,
            0,
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryOutputPropertyReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_OUTPUT_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryOutputPropertyRequest {
            output,
            property,
        })
    }
}
impl Request for QueryOutputPropertyRequest {
    type Reply = QueryOutputPropertyReply;
}
pub fn query_output_property<Conn>(conn: &Conn, output: Output, property: xproto::Atom) -> Result<Cookie<'_, Conn, QueryOutputPropertyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryOutputPropertyRequest {
        output,
        property,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryOutputPropertyReply {
    pub sequence: u16,
    pub pending: bool,
    pub range: bool,
    pub immutable: bool,
    pub valid_values: Vec<i32>,
}
impl TryParse for QueryOutputPropertyReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pending, remaining) = bool::try_parse(remaining)?;
        let (range, remaining) = bool::try_parse(remaining)?;
        let (immutable, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::InsufficientData)?;
        let (valid_values, remaining) = crate::x11_utils::parse_list::<i32>(remaining, length.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryOutputPropertyReply { sequence, pending, range, immutable, valid_values };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryOutputPropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl QueryOutputPropertyReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `validValues` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.valid_values.len()
            .try_into().unwrap()
    }
}

/// Opcode for the ConfigureOutputProperty request
pub const CONFIGURE_OUTPUT_PROPERTY_REQUEST: u8 = 12;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigureOutputPropertyRequest<'input> {
    pub output: Output,
    pub property: xproto::Atom,
    pub pending: bool,
    pub range: bool,
    pub values: Cow<'input, [i32]>,
}
impl<'input> ConfigureOutputPropertyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let property_bytes = self.property.serialize();
        let pending_bytes = self.pending.serialize();
        let range_bytes = self.range.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CONFIGURE_OUTPUT_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let values_bytes = self.values.serialize();
        let length_so_far = length_so_far + values_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), values_bytes.into(), padding0.into()], vec![]))
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
        if header.minor_opcode != CONFIGURE_OUTPUT_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (pending, remaining) = bool::try_parse(remaining)?;
        let (range, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let mut remaining = remaining;
        // Length is 'everything left in the input'
        let mut values = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = i32::try_parse(remaining)?;
            remaining = new_remaining;
            values.push(v);
        }
        let _ = remaining;
        Ok(ConfigureOutputPropertyRequest {
            output,
            property,
            pending,
            range,
            values: Cow::Owned(values),
        })
    }
    /// Clone all borrowed data in this ConfigureOutputPropertyRequest.
    pub fn into_owned(self) -> ConfigureOutputPropertyRequest<'static> {
        ConfigureOutputPropertyRequest {
            output: self.output,
            property: self.property,
            pending: self.pending,
            range: self.range,
            values: Cow::Owned(self.values.into_owned()),
        }
    }
}
impl<'input> Request for ConfigureOutputPropertyRequest<'input> {
    type Reply = ();
}
pub fn configure_output_property<'c, 'input, Conn>(conn: &'c Conn, output: Output, property: xproto::Atom, pending: bool, range: bool, values: &'input [i32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ConfigureOutputPropertyRequest {
        output,
        property,
        pending,
        range,
        values: Cow::Borrowed(values),
    };
    request0.send(conn)
}

/// Opcode for the ChangeOutputProperty request
pub const CHANGE_OUTPUT_PROPERTY_REQUEST: u8 = 13;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeOutputPropertyRequest<'input> {
    pub output: Output,
    pub property: xproto::Atom,
    pub type_: xproto::Atom,
    pub format: u8,
    pub mode: xproto::PropMode,
    pub num_units: u32,
    pub data: Cow<'input, [u8]>,
}
impl<'input> ChangeOutputPropertyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let property_bytes = self.property.serialize();
        let type_bytes = self.type_.serialize();
        let format_bytes = self.format.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
        let num_units_bytes = self.num_units.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CHANGE_OUTPUT_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(self.data.len(), usize::try_from(self.num_units.checked_mul(u32::from(self.format)).unwrap().checked_div(8u32).unwrap()).unwrap(), "`data` has an incorrect length");
        let length_so_far = length_so_far + self.data.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.data, padding0.into()], vec![]))
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
        if header.minor_opcode != CHANGE_OUTPUT_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let mode = mode.try_into()?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (num_units, remaining) = u32::try_parse(remaining)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, num_units.checked_mul(u32::from(format)).ok_or(ParseError::InvalidExpression)?.checked_div(8u32).ok_or(ParseError::InvalidExpression)?.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(ChangeOutputPropertyRequest {
            output,
            property,
            type_,
            format,
            mode,
            num_units,
            data: Cow::Borrowed(data),
        })
    }
    /// Clone all borrowed data in this ChangeOutputPropertyRequest.
    pub fn into_owned(self) -> ChangeOutputPropertyRequest<'static> {
        ChangeOutputPropertyRequest {
            output: self.output,
            property: self.property,
            type_: self.type_,
            format: self.format,
            mode: self.mode,
            num_units: self.num_units,
            data: Cow::Owned(self.data.into_owned()),
        }
    }
}
impl<'input> Request for ChangeOutputPropertyRequest<'input> {
    type Reply = ();
}
pub fn change_output_property<'c, 'input, Conn>(conn: &'c Conn, output: Output, property: xproto::Atom, type_: xproto::Atom, format: u8, mode: xproto::PropMode, num_units: u32, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeOutputPropertyRequest {
        output,
        property,
        type_,
        format,
        mode,
        num_units,
        data: Cow::Borrowed(data),
    };
    request0.send(conn)
}

/// Opcode for the DeleteOutputProperty request
pub const DELETE_OUTPUT_PROPERTY_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteOutputPropertyRequest {
    pub output: Output,
    pub property: xproto::Atom,
}
impl DeleteOutputPropertyRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DELETE_OUTPUT_PROPERTY_REQUEST,
            0,
            0,
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
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
        if header.minor_opcode != DELETE_OUTPUT_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(DeleteOutputPropertyRequest {
            output,
            property,
        })
    }
}
impl Request for DeleteOutputPropertyRequest {
    type Reply = ();
}
pub fn delete_output_property<Conn>(conn: &Conn, output: Output, property: xproto::Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteOutputPropertyRequest {
        output,
        property,
    };
    request0.send(conn)
}

/// Opcode for the GetOutputProperty request
pub const GET_OUTPUT_PROPERTY_REQUEST: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOutputPropertyRequest {
    pub output: Output,
    pub property: xproto::Atom,
    pub type_: xproto::Atom,
    pub long_offset: u32,
    pub long_length: u32,
    pub delete: bool,
    pub pending: bool,
}
impl GetOutputPropertyRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let property_bytes = self.property.serialize();
        let type_bytes = self.type_.serialize();
        let long_offset_bytes = self.long_offset.serialize();
        let long_length_bytes = self.long_length.serialize();
        let delete_bytes = self.delete.serialize();
        let pending_bytes = self.pending.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_OUTPUT_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetOutputPropertyReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_OUTPUT_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (long_offset, remaining) = u32::try_parse(remaining)?;
        let (long_length, remaining) = u32::try_parse(remaining)?;
        let (delete, remaining) = bool::try_parse(remaining)?;
        let (pending, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetOutputPropertyRequest {
            output,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        })
    }
}
impl Request for GetOutputPropertyRequest {
    type Reply = GetOutputPropertyReply;
}
pub fn get_output_property<Conn, A>(conn: &Conn, output: Output, property: xproto::Atom, type_: A, long_offset: u32, long_length: u32, delete: bool, pending: bool) -> Result<Cookie<'_, Conn, GetOutputPropertyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Atom>,
{
    let type_: xproto::Atom = type_.into();
    let request0 = GetOutputPropertyRequest {
        output,
        property,
        type_,
        long_offset,
        long_length,
        delete,
        pending,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetOutputPropertyReply {
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xproto::Atom,
    pub bytes_after: u32,
    pub num_items: u32,
    pub data: Vec<u8>,
}
impl TryParse for GetOutputPropertyReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, num_items.checked_mul(u32::from(format).checked_div(8u32).ok_or(ParseError::InvalidExpression)?).ok_or(ParseError::InvalidExpression)?.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetOutputPropertyReply { format, sequence, length, type_, bytes_after, num_items, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateModeRequest<'input> {
    pub window: xproto::Window,
    pub mode_info: ModeInfo,
    pub name: Cow<'input, [u8]>,
}
impl<'input> CreateModeRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mode_info_bytes = self.mode_info.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_MODE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.name.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.name, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, CreateModeReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_MODE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (mode_info, remaining) = ModeInfo::try_parse(remaining)?;
        let (name, remaining) = remaining.split_at(remaining.len());
        let _ = remaining;
        Ok(CreateModeRequest {
            window,
            mode_info,
            name: Cow::Borrowed(name),
        })
    }
    /// Clone all borrowed data in this CreateModeRequest.
    pub fn into_owned(self) -> CreateModeRequest<'static> {
        CreateModeRequest {
            window: self.window,
            mode_info: self.mode_info,
            name: Cow::Owned(self.name.into_owned()),
        }
    }
}
impl<'input> Request for CreateModeRequest<'input> {
    type Reply = CreateModeReply;
}
pub fn create_mode<'c, 'input, Conn>(conn: &'c Conn, window: xproto::Window, mode_info: ModeInfo, name: &'input [u8]) -> Result<Cookie<'c, Conn, CreateModeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateModeRequest {
        window,
        mode_info,
        name: Cow::Borrowed(name),
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateModeReply {
    pub sequence: u16,
    pub length: u32,
    pub mode: Mode,
}
impl TryParse for CreateModeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (mode, remaining) = Mode::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = CreateModeReply { sequence, length, mode };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyModeRequest {
    pub mode: Mode,
}
impl DestroyModeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mode_bytes = self.mode.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_MODE_REQUEST,
            0,
            0,
            mode_bytes[0],
            mode_bytes[1],
            mode_bytes[2],
            mode_bytes[3],
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
        if header.minor_opcode != DESTROY_MODE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (mode, remaining) = Mode::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyModeRequest {
            mode,
        })
    }
}
impl Request for DestroyModeRequest {
    type Reply = ();
}
pub fn destroy_mode<Conn>(conn: &Conn, mode: Mode) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyModeRequest {
        mode,
    };
    request0.send(conn)
}

/// Opcode for the AddOutputMode request
pub const ADD_OUTPUT_MODE_REQUEST: u8 = 18;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddOutputModeRequest {
    pub output: Output,
    pub mode: Mode,
}
impl AddOutputModeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let mode_bytes = self.mode.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ADD_OUTPUT_MODE_REQUEST,
            0,
            0,
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            mode_bytes[0],
            mode_bytes[1],
            mode_bytes[2],
            mode_bytes[3],
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
        if header.minor_opcode != ADD_OUTPUT_MODE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let (mode, remaining) = Mode::try_parse(remaining)?;
        let _ = remaining;
        Ok(AddOutputModeRequest {
            output,
            mode,
        })
    }
}
impl Request for AddOutputModeRequest {
    type Reply = ();
}
pub fn add_output_mode<Conn>(conn: &Conn, output: Output, mode: Mode) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AddOutputModeRequest {
        output,
        mode,
    };
    request0.send(conn)
}

/// Opcode for the DeleteOutputMode request
pub const DELETE_OUTPUT_MODE_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteOutputModeRequest {
    pub output: Output,
    pub mode: Mode,
}
impl DeleteOutputModeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let output_bytes = self.output.serialize();
        let mode_bytes = self.mode.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DELETE_OUTPUT_MODE_REQUEST,
            0,
            0,
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
            mode_bytes[0],
            mode_bytes[1],
            mode_bytes[2],
            mode_bytes[3],
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
        if header.minor_opcode != DELETE_OUTPUT_MODE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output, remaining) = Output::try_parse(value)?;
        let (mode, remaining) = Mode::try_parse(remaining)?;
        let _ = remaining;
        Ok(DeleteOutputModeRequest {
            output,
            mode,
        })
    }
}
impl Request for DeleteOutputModeRequest {
    type Reply = ();
}
pub fn delete_output_mode<Conn>(conn: &Conn, output: Output, mode: Mode) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteOutputModeRequest {
        output,
        mode,
    };
    request0.send(conn)
}

/// Opcode for the GetCrtcInfo request
pub const GET_CRTC_INFO_REQUEST: u8 = 20;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrtcInfoRequest {
    pub crtc: Crtc,
    pub config_timestamp: xproto::Timestamp,
}
impl GetCrtcInfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_CRTC_INFO_REQUEST,
            0,
            0,
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
            config_timestamp_bytes[0],
            config_timestamp_bytes[1],
            config_timestamp_bytes[2],
            config_timestamp_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetCrtcInfoReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CRTC_INFO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetCrtcInfoRequest {
            crtc,
            config_timestamp,
        })
    }
}
impl Request for GetCrtcInfoRequest {
    type Reply = GetCrtcInfoReply;
}
pub fn get_crtc_info<Conn>(conn: &Conn, crtc: Crtc, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Conn, GetCrtcInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCrtcInfoRequest {
        crtc,
        config_timestamp,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCrtcInfoReply {
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
impl TryParse for GetCrtcInfoReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (possible, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_possible_outputs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.try_into()?;
        let result = GetCrtcInfoReply { status, sequence, length, timestamp, x, y, width, height, mode, rotation, rotations, outputs, possible };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCrtcInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetCrtcInfoReply {
    /// Get the value of the `num_outputs` field.
    ///
    /// The `num_outputs` field is used as the length field of the `outputs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_outputs(&self) -> u16 {
        self.outputs.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_possible_outputs` field.
    ///
    /// The `num_possible_outputs` field is used as the length field of the `possible` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_possible_outputs(&self) -> u16 {
        self.possible.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetCrtcConfig request
pub const SET_CRTC_CONFIG_REQUEST: u8 = 21;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCrtcConfigRequest<'input> {
    pub crtc: Crtc,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub x: i16,
    pub y: i16,
    pub mode: Mode,
    pub rotation: u16,
    pub outputs: Cow<'input, [Output]>,
}
impl<'input> SetCrtcConfigRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let timestamp_bytes = self.timestamp.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let mode_bytes = self.mode.serialize();
        let rotation_bytes = self.rotation.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_CRTC_CONFIG_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let outputs_bytes = self.outputs.serialize();
        let length_so_far = length_so_far + outputs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), outputs_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, SetCrtcConfigReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_CRTC_CONFIG_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (mode, remaining) = Mode::try_parse(remaining)?;
        let (rotation, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let mut remaining = remaining;
        // Length is 'everything left in the input'
        let mut outputs = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = Output::try_parse(remaining)?;
            remaining = new_remaining;
            outputs.push(v);
        }
        let _ = remaining;
        Ok(SetCrtcConfigRequest {
            crtc,
            timestamp,
            config_timestamp,
            x,
            y,
            mode,
            rotation,
            outputs: Cow::Owned(outputs),
        })
    }
    /// Clone all borrowed data in this SetCrtcConfigRequest.
    pub fn into_owned(self) -> SetCrtcConfigRequest<'static> {
        SetCrtcConfigRequest {
            crtc: self.crtc,
            timestamp: self.timestamp,
            config_timestamp: self.config_timestamp,
            x: self.x,
            y: self.y,
            mode: self.mode,
            rotation: self.rotation,
            outputs: Cow::Owned(self.outputs.into_owned()),
        }
    }
}
impl<'input> Request for SetCrtcConfigRequest<'input> {
    type Reply = SetCrtcConfigReply;
}
pub fn set_crtc_config<'c, 'input, Conn, A>(conn: &'c Conn, crtc: Crtc, timestamp: xproto::Timestamp, config_timestamp: xproto::Timestamp, x: i16, y: i16, mode: Mode, rotation: A, outputs: &'input [Output]) -> Result<Cookie<'c, Conn, SetCrtcConfigReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
{
    let rotation: u16 = rotation.into();
    let request0 = SetCrtcConfigRequest {
        crtc,
        timestamp,
        config_timestamp,
        x,
        y,
        mode,
        rotation,
        outputs: Cow::Borrowed(outputs),
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetCrtcConfigReply {
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
}
impl TryParse for SetCrtcConfigReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.try_into()?;
        let result = SetCrtcConfigReply { status, sequence, length, timestamp };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrtcGammaSizeRequest {
    pub crtc: Crtc,
}
impl GetCrtcGammaSizeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_CRTC_GAMMA_SIZE_REQUEST,
            0,
            0,
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetCrtcGammaSizeReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CRTC_GAMMA_SIZE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
        let _ = remaining;
        Ok(GetCrtcGammaSizeRequest {
            crtc,
        })
    }
}
impl Request for GetCrtcGammaSizeRequest {
    type Reply = GetCrtcGammaSizeReply;
}
pub fn get_crtc_gamma_size<Conn>(conn: &Conn, crtc: Crtc) -> Result<Cookie<'_, Conn, GetCrtcGammaSizeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCrtcGammaSizeRequest {
        crtc,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrtcGammaSizeReply {
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
}
impl TryParse for GetCrtcGammaSizeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetCrtcGammaSizeReply { sequence, length, size };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrtcGammaRequest {
    pub crtc: Crtc,
}
impl GetCrtcGammaRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_CRTC_GAMMA_REQUEST,
            0,
            0,
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetCrtcGammaReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CRTC_GAMMA_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
        let _ = remaining;
        Ok(GetCrtcGammaRequest {
            crtc,
        })
    }
}
impl Request for GetCrtcGammaRequest {
    type Reply = GetCrtcGammaReply;
}
pub fn get_crtc_gamma<Conn>(conn: &Conn, crtc: Crtc) -> Result<Cookie<'_, Conn, GetCrtcGammaReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCrtcGammaRequest {
        crtc,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCrtcGammaReply {
    pub sequence: u16,
    pub length: u32,
    pub red: Vec<u16>,
    pub green: Vec<u16>,
    pub blue: Vec<u16>,
}
impl TryParse for GetCrtcGammaReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (red, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (green, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (blue, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetCrtcGammaReply { sequence, length, red, green, blue };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCrtcGammaReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetCrtcGammaReply {
    /// Get the value of the `size` field.
    ///
    /// The `size` field is used as the length field of the `red` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn size(&self) -> u16 {
        self.red.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetCrtcGamma request
pub const SET_CRTC_GAMMA_REQUEST: u8 = 24;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCrtcGammaRequest<'input> {
    pub crtc: Crtc,
    pub red: Cow<'input, [u16]>,
    pub green: Cow<'input, [u16]>,
    pub blue: Cow<'input, [u16]>,
}
impl<'input> SetCrtcGammaRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let size = u16::try_from(self.red.len()).expect("`red` has too many elements");
        let size_bytes = size.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_CRTC_GAMMA_REQUEST,
            0,
            0,
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
            size_bytes[0],
            size_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let red_bytes = self.red.serialize();
        let length_so_far = length_so_far + red_bytes.len();
        assert_eq!(self.green.len(), usize::try_from(size).unwrap(), "`green` has an incorrect length");
        let green_bytes = self.green.serialize();
        let length_so_far = length_so_far + green_bytes.len();
        assert_eq!(self.blue.len(), usize::try_from(size).unwrap(), "`blue` has an incorrect length");
        let blue_bytes = self.blue.serialize();
        let length_so_far = length_so_far + blue_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), red_bytes.into(), green_bytes.into(), blue_bytes.into(), padding0.into()], vec![]))
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
        if header.minor_opcode != SET_CRTC_GAMMA_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (red, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (green, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (blue, remaining) = crate::x11_utils::parse_list::<u16>(remaining, size.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(SetCrtcGammaRequest {
            crtc,
            red: Cow::Owned(red),
            green: Cow::Owned(green),
            blue: Cow::Owned(blue),
        })
    }
    /// Clone all borrowed data in this SetCrtcGammaRequest.
    pub fn into_owned(self) -> SetCrtcGammaRequest<'static> {
        SetCrtcGammaRequest {
            crtc: self.crtc,
            red: Cow::Owned(self.red.into_owned()),
            green: Cow::Owned(self.green.into_owned()),
            blue: Cow::Owned(self.blue.into_owned()),
        }
    }
}
impl<'input> Request for SetCrtcGammaRequest<'input> {
    type Reply = ();
}
pub fn set_crtc_gamma<'c, 'input, Conn>(conn: &'c Conn, crtc: Crtc, red: &'input [u16], green: &'input [u16], blue: &'input [u16]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetCrtcGammaRequest {
        crtc,
        red: Cow::Borrowed(red),
        green: Cow::Borrowed(green),
        blue: Cow::Borrowed(blue),
    };
    request0.send(conn)
}

/// Opcode for the GetScreenResourcesCurrent request
pub const GET_SCREEN_RESOURCES_CURRENT_REQUEST: u8 = 25;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenResourcesCurrentRequest {
    pub window: xproto::Window,
}
impl GetScreenResourcesCurrentRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SCREEN_RESOURCES_CURRENT_REQUEST,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetScreenResourcesCurrentReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SCREEN_RESOURCES_CURRENT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetScreenResourcesCurrentRequest {
            window,
        })
    }
}
impl Request for GetScreenResourcesCurrentRequest {
    type Reply = GetScreenResourcesCurrentReply;
}
pub fn get_screen_resources_current<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenResourcesCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetScreenResourcesCurrentRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetScreenResourcesCurrentReply {
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub crtcs: Vec<Crtc>,
    pub outputs: Vec<Output>,
    pub modes: Vec<ModeInfo>,
    pub names: Vec<u8>,
}
impl TryParse for GetScreenResourcesCurrentReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_crtcs, remaining) = u16::try_parse(remaining)?;
        let (num_outputs, remaining) = u16::try_parse(remaining)?;
        let (num_modes, remaining) = u16::try_parse(remaining)?;
        let (names_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (modes, remaining) = crate::x11_utils::parse_list::<ModeInfo>(remaining, num_modes.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (names, remaining) = crate::x11_utils::parse_u8_list(remaining, names_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let names = names.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetScreenResourcesCurrentReply { sequence, length, timestamp, config_timestamp, crtcs, outputs, modes, names };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenResourcesCurrentReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetScreenResourcesCurrentReply {
    /// Get the value of the `num_crtcs` field.
    ///
    /// The `num_crtcs` field is used as the length field of the `crtcs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_crtcs(&self) -> u16 {
        self.crtcs.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_outputs` field.
    ///
    /// The `num_outputs` field is used as the length field of the `outputs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_outputs(&self) -> u16 {
        self.outputs.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_modes` field.
    ///
    /// The `num_modes` field is used as the length field of the `modes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_modes(&self) -> u16 {
        self.modes.len()
            .try_into().unwrap()
    }
    /// Get the value of the `names_len` field.
    ///
    /// The `names_len` field is used as the length field of the `names` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn names_len(&self) -> u16 {
        self.names.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for Transform {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for Transform {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
bitmask_binop!(Transform, u8);

/// Opcode for the SetCrtcTransform request
pub const SET_CRTC_TRANSFORM_REQUEST: u8 = 26;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCrtcTransformRequest<'input> {
    pub crtc: Crtc,
    pub transform: render::Transform,
    pub filter_name: Cow<'input, [u8]>,
    pub filter_params: Cow<'input, [render::Fixed]>,
}
impl<'input> SetCrtcTransformRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let transform_bytes = self.transform.serialize();
        let filter_len = u16::try_from(self.filter_name.len()).expect("`filter_name` has too many elements");
        let filter_len_bytes = filter_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_CRTC_TRANSFORM_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.filter_name.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        let filter_params_bytes = self.filter_params.serialize();
        let length_so_far = length_so_far + filter_params_bytes.len();
        let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding1.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.filter_name, padding0.into(), filter_params_bytes.into(), padding1.into()], vec![]))
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
        if header.minor_opcode != SET_CRTC_TRANSFORM_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
        let (transform, remaining) = render::Transform::try_parse(remaining)?;
        let (filter_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (filter_name, remaining) = crate::x11_utils::parse_u8_list(remaining, filter_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let mut remaining = remaining;
        // Length is 'everything left in the input'
        let mut filter_params = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = render::Fixed::try_parse(remaining)?;
            remaining = new_remaining;
            filter_params.push(v);
        }
        let _ = remaining;
        Ok(SetCrtcTransformRequest {
            crtc,
            transform,
            filter_name: Cow::Borrowed(filter_name),
            filter_params: Cow::Owned(filter_params),
        })
    }
    /// Clone all borrowed data in this SetCrtcTransformRequest.
    pub fn into_owned(self) -> SetCrtcTransformRequest<'static> {
        SetCrtcTransformRequest {
            crtc: self.crtc,
            transform: self.transform,
            filter_name: Cow::Owned(self.filter_name.into_owned()),
            filter_params: Cow::Owned(self.filter_params.into_owned()),
        }
    }
}
impl<'input> Request for SetCrtcTransformRequest<'input> {
    type Reply = ();
}
pub fn set_crtc_transform<'c, 'input, Conn>(conn: &'c Conn, crtc: Crtc, transform: render::Transform, filter_name: &'input [u8], filter_params: &'input [render::Fixed]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetCrtcTransformRequest {
        crtc,
        transform,
        filter_name: Cow::Borrowed(filter_name),
        filter_params: Cow::Borrowed(filter_params),
    };
    request0.send(conn)
}

/// Opcode for the GetCrtcTransform request
pub const GET_CRTC_TRANSFORM_REQUEST: u8 = 27;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrtcTransformRequest {
    pub crtc: Crtc,
}
impl GetCrtcTransformRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_CRTC_TRANSFORM_REQUEST,
            0,
            0,
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetCrtcTransformReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CRTC_TRANSFORM_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
        let _ = remaining;
        Ok(GetCrtcTransformRequest {
            crtc,
        })
    }
}
impl Request for GetCrtcTransformRequest {
    type Reply = GetCrtcTransformReply;
}
pub fn get_crtc_transform<Conn>(conn: &Conn, crtc: Crtc) -> Result<Cookie<'_, Conn, GetCrtcTransformReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCrtcTransformRequest {
        crtc,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCrtcTransformReply {
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
impl TryParse for GetCrtcTransformReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pending_transform, remaining) = render::Transform::try_parse(remaining)?;
        let (has_transforms, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (current_transform, remaining) = render::Transform::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (pending_len, remaining) = u16::try_parse(remaining)?;
        let (pending_nparams, remaining) = u16::try_parse(remaining)?;
        let (current_len, remaining) = u16::try_parse(remaining)?;
        let (current_nparams, remaining) = u16::try_parse(remaining)?;
        let (pending_filter_name, remaining) = crate::x11_utils::parse_u8_list(remaining, pending_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let pending_filter_name = pending_filter_name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (pending_params, remaining) = crate::x11_utils::parse_list::<render::Fixed>(remaining, pending_nparams.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (current_filter_name, remaining) = crate::x11_utils::parse_u8_list(remaining, current_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let current_filter_name = current_filter_name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (current_params, remaining) = crate::x11_utils::parse_list::<render::Fixed>(remaining, current_nparams.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetCrtcTransformReply { sequence, length, pending_transform, has_transforms, current_transform, pending_filter_name, pending_params, current_filter_name, current_params };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCrtcTransformReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetCrtcTransformReply {
    /// Get the value of the `pending_len` field.
    ///
    /// The `pending_len` field is used as the length field of the `pending_filter_name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn pending_len(&self) -> u16 {
        self.pending_filter_name.len()
            .try_into().unwrap()
    }
    /// Get the value of the `pending_nparams` field.
    ///
    /// The `pending_nparams` field is used as the length field of the `pending_params` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn pending_nparams(&self) -> u16 {
        self.pending_params.len()
            .try_into().unwrap()
    }
    /// Get the value of the `current_len` field.
    ///
    /// The `current_len` field is used as the length field of the `current_filter_name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn current_len(&self) -> u16 {
        self.current_filter_name.len()
            .try_into().unwrap()
    }
    /// Get the value of the `current_nparams` field.
    ///
    /// The `current_nparams` field is used as the length field of the `current_params` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn current_nparams(&self) -> u16 {
        self.current_params.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetPanning request
pub const GET_PANNING_REQUEST: u8 = 28;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPanningRequest {
    pub crtc: Crtc,
}
impl GetPanningRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_PANNING_REQUEST,
            0,
            0,
            crtc_bytes[0],
            crtc_bytes[1],
            crtc_bytes[2],
            crtc_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetPanningReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PANNING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
        let _ = remaining;
        Ok(GetPanningRequest {
            crtc,
        })
    }
}
impl Request for GetPanningRequest {
    type Reply = GetPanningReply;
}
pub fn get_panning<Conn>(conn: &Conn, crtc: Crtc) -> Result<Cookie<'_, Conn, GetPanningReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPanningRequest {
        crtc,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPanningReply {
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
impl TryParse for GetPanningReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.try_into()?;
        let result = GetPanningReply { status, sequence, length, timestamp, left, top, width, height, track_left, track_top, track_width, track_height, border_left, border_top, border_right, border_bottom };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPanningRequest {
    pub crtc: Crtc,
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
impl SetPanningRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let crtc_bytes = self.crtc.serialize();
        let timestamp_bytes = self.timestamp.serialize();
        let left_bytes = self.left.serialize();
        let top_bytes = self.top.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let track_left_bytes = self.track_left.serialize();
        let track_top_bytes = self.track_top.serialize();
        let track_width_bytes = self.track_width.serialize();
        let track_height_bytes = self.track_height.serialize();
        let border_left_bytes = self.border_left.serialize();
        let border_top_bytes = self.border_top.serialize();
        let border_right_bytes = self.border_right.serialize();
        let border_bottom_bytes = self.border_bottom.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_PANNING_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, SetPanningReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_PANNING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (crtc, remaining) = Crtc::try_parse(value)?;
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
        let _ = remaining;
        Ok(SetPanningRequest {
            crtc,
            timestamp,
            left,
            top,
            width,
            height,
            track_left,
            track_top,
            track_width,
            track_height,
            border_left,
            border_top,
            border_right,
            border_bottom,
        })
    }
}
impl Request for SetPanningRequest {
    type Reply = SetPanningReply;
}
pub fn set_panning<Conn>(conn: &Conn, crtc: Crtc, timestamp: xproto::Timestamp, left: u16, top: u16, width: u16, height: u16, track_left: u16, track_top: u16, track_width: u16, track_height: u16, border_left: i16, border_top: i16, border_right: i16, border_bottom: i16) -> Result<Cookie<'_, Conn, SetPanningReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPanningRequest {
        crtc,
        timestamp,
        left,
        top,
        width,
        height,
        track_left,
        track_top,
        track_width,
        track_height,
        border_left,
        border_top,
        border_right,
        border_bottom,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPanningReply {
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
}
impl TryParse for SetPanningReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.try_into()?;
        let result = SetPanningReply { status, sequence, length, timestamp };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetOutputPrimaryRequest {
    pub window: xproto::Window,
    pub output: Output,
}
impl SetOutputPrimaryRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let output_bytes = self.output.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_OUTPUT_PRIMARY_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            output_bytes[0],
            output_bytes[1],
            output_bytes[2],
            output_bytes[3],
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
        if header.minor_opcode != SET_OUTPUT_PRIMARY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (output, remaining) = Output::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetOutputPrimaryRequest {
            window,
            output,
        })
    }
}
impl Request for SetOutputPrimaryRequest {
    type Reply = ();
}
pub fn set_output_primary<Conn>(conn: &Conn, window: xproto::Window, output: Output) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetOutputPrimaryRequest {
        window,
        output,
    };
    request0.send(conn)
}

/// Opcode for the GetOutputPrimary request
pub const GET_OUTPUT_PRIMARY_REQUEST: u8 = 31;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOutputPrimaryRequest {
    pub window: xproto::Window,
}
impl GetOutputPrimaryRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_OUTPUT_PRIMARY_REQUEST,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetOutputPrimaryReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_OUTPUT_PRIMARY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetOutputPrimaryRequest {
            window,
        })
    }
}
impl Request for GetOutputPrimaryRequest {
    type Reply = GetOutputPrimaryReply;
}
pub fn get_output_primary<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetOutputPrimaryReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetOutputPrimaryRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOutputPrimaryReply {
    pub sequence: u16,
    pub length: u32,
    pub output: Output,
}
impl TryParse for GetOutputPrimaryReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (output, remaining) = Output::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetOutputPrimaryReply { sequence, length, output };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProvidersRequest {
    pub window: xproto::Window,
}
impl GetProvidersRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_PROVIDERS_REQUEST,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetProvidersReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROVIDERS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetProvidersRequest {
            window,
        })
    }
}
impl Request for GetProvidersRequest {
    type Reply = GetProvidersReply;
}
pub fn get_providers<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetProvidersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetProvidersRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProvidersReply {
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub providers: Vec<Provider>,
}
impl TryParse for GetProvidersReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_providers, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::InsufficientData)?;
        let (providers, remaining) = crate::x11_utils::parse_list::<Provider>(remaining, num_providers.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetProvidersReply { sequence, length, timestamp, providers };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetProvidersReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetProvidersReply {
    /// Get the value of the `num_providers` field.
    ///
    /// The `num_providers` field is used as the length field of the `providers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_providers(&self) -> u16 {
        self.providers.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for ProviderCapability {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for ProviderCapability {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
bitmask_binop!(ProviderCapability, u8);

/// Opcode for the GetProviderInfo request
pub const GET_PROVIDER_INFO_REQUEST: u8 = 33;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProviderInfoRequest {
    pub provider: Provider,
    pub config_timestamp: xproto::Timestamp,
}
impl GetProviderInfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_PROVIDER_INFO_REQUEST,
            0,
            0,
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
            config_timestamp_bytes[0],
            config_timestamp_bytes[1],
            config_timestamp_bytes[2],
            config_timestamp_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetProviderInfoReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROVIDER_INFO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetProviderInfoRequest {
            provider,
            config_timestamp,
        })
    }
}
impl Request for GetProviderInfoRequest {
    type Reply = GetProviderInfoReply;
}
pub fn get_provider_info<Conn>(conn: &Conn, provider: Provider, config_timestamp: xproto::Timestamp) -> Result<Cookie<'_, Conn, GetProviderInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetProviderInfoRequest {
        provider,
        config_timestamp,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProviderInfoReply {
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub capabilities: u32,
    pub crtcs: Vec<Crtc>,
    pub outputs: Vec<Output>,
    pub associated_providers: Vec<Provider>,
    pub associated_capability: Vec<u32>,
    pub name: Vec<u8>,
}
impl TryParse for GetProviderInfoReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (associated_providers, remaining) = crate::x11_utils::parse_list::<Provider>(remaining, num_associated_providers.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (associated_capability, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_associated_providers.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let name = name.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetProviderInfoReply { status, sequence, length, timestamp, capabilities, crtcs, outputs, associated_providers, associated_capability, name };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetProviderInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetProviderInfoReply {
    /// Get the value of the `num_crtcs` field.
    ///
    /// The `num_crtcs` field is used as the length field of the `crtcs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_crtcs(&self) -> u16 {
        self.crtcs.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_outputs` field.
    ///
    /// The `num_outputs` field is used as the length field of the `outputs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_outputs(&self) -> u16 {
        self.outputs.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_associated_providers` field.
    ///
    /// The `num_associated_providers` field is used as the length field of the `associated_providers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_associated_providers(&self) -> u16 {
        self.associated_providers.len()
            .try_into().unwrap()
    }
    /// Get the value of the `name_len` field.
    ///
    /// The `name_len` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn name_len(&self) -> u16 {
        self.name.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetProviderOffloadSink request
pub const SET_PROVIDER_OFFLOAD_SINK_REQUEST: u8 = 34;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetProviderOffloadSinkRequest {
    pub provider: Provider,
    pub sink_provider: Provider,
    pub config_timestamp: xproto::Timestamp,
}
impl SetProviderOffloadSinkRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let sink_provider_bytes = self.sink_provider.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_PROVIDER_OFFLOAD_SINK_REQUEST,
            0,
            0,
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
        if header.minor_opcode != SET_PROVIDER_OFFLOAD_SINK_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let (sink_provider, remaining) = Provider::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetProviderOffloadSinkRequest {
            provider,
            sink_provider,
            config_timestamp,
        })
    }
}
impl Request for SetProviderOffloadSinkRequest {
    type Reply = ();
}
pub fn set_provider_offload_sink<Conn>(conn: &Conn, provider: Provider, sink_provider: Provider, config_timestamp: xproto::Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetProviderOffloadSinkRequest {
        provider,
        sink_provider,
        config_timestamp,
    };
    request0.send(conn)
}

/// Opcode for the SetProviderOutputSource request
pub const SET_PROVIDER_OUTPUT_SOURCE_REQUEST: u8 = 35;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetProviderOutputSourceRequest {
    pub provider: Provider,
    pub source_provider: Provider,
    pub config_timestamp: xproto::Timestamp,
}
impl SetProviderOutputSourceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let source_provider_bytes = self.source_provider.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_PROVIDER_OUTPUT_SOURCE_REQUEST,
            0,
            0,
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
        if header.minor_opcode != SET_PROVIDER_OUTPUT_SOURCE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let (source_provider, remaining) = Provider::try_parse(remaining)?;
        let (config_timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetProviderOutputSourceRequest {
            provider,
            source_provider,
            config_timestamp,
        })
    }
}
impl Request for SetProviderOutputSourceRequest {
    type Reply = ();
}
pub fn set_provider_output_source<Conn>(conn: &Conn, provider: Provider, source_provider: Provider, config_timestamp: xproto::Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetProviderOutputSourceRequest {
        provider,
        source_provider,
        config_timestamp,
    };
    request0.send(conn)
}

/// Opcode for the ListProviderProperties request
pub const LIST_PROVIDER_PROPERTIES_REQUEST: u8 = 36;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListProviderPropertiesRequest {
    pub provider: Provider,
}
impl ListProviderPropertiesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            LIST_PROVIDER_PROPERTIES_REQUEST,
            0,
            0,
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListProviderPropertiesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_PROVIDER_PROPERTIES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let _ = remaining;
        Ok(ListProviderPropertiesRequest {
            provider,
        })
    }
}
impl Request for ListProviderPropertiesRequest {
    type Reply = ListProviderPropertiesReply;
}
pub fn list_provider_properties<Conn>(conn: &Conn, provider: Provider) -> Result<Cookie<'_, Conn, ListProviderPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListProviderPropertiesRequest {
        provider,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListProviderPropertiesReply {
    pub sequence: u16,
    pub length: u32,
    pub atoms: Vec<xproto::Atom>,
}
impl TryParse for ListProviderPropertiesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_atoms, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (atoms, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_atoms.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListProviderPropertiesReply { sequence, length, atoms };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListProviderPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl ListProviderPropertiesReply {
    /// Get the value of the `num_atoms` field.
    ///
    /// The `num_atoms` field is used as the length field of the `atoms` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_atoms(&self) -> u16 {
        self.atoms.len()
            .try_into().unwrap()
    }
}

/// Opcode for the QueryProviderProperty request
pub const QUERY_PROVIDER_PROPERTY_REQUEST: u8 = 37;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryProviderPropertyRequest {
    pub provider: Provider,
    pub property: xproto::Atom,
}
impl QueryProviderPropertyRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_PROVIDER_PROPERTY_REQUEST,
            0,
            0,
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryProviderPropertyReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_PROVIDER_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryProviderPropertyRequest {
            provider,
            property,
        })
    }
}
impl Request for QueryProviderPropertyRequest {
    type Reply = QueryProviderPropertyReply;
}
pub fn query_provider_property<Conn>(conn: &Conn, provider: Provider, property: xproto::Atom) -> Result<Cookie<'_, Conn, QueryProviderPropertyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryProviderPropertyRequest {
        provider,
        property,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryProviderPropertyReply {
    pub sequence: u16,
    pub pending: bool,
    pub range: bool,
    pub immutable: bool,
    pub valid_values: Vec<i32>,
}
impl TryParse for QueryProviderPropertyReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pending, remaining) = bool::try_parse(remaining)?;
        let (range, remaining) = bool::try_parse(remaining)?;
        let (immutable, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::InsufficientData)?;
        let (valid_values, remaining) = crate::x11_utils::parse_list::<i32>(remaining, length.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryProviderPropertyReply { sequence, pending, range, immutable, valid_values };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryProviderPropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl QueryProviderPropertyReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `valid_values` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.valid_values.len()
            .try_into().unwrap()
    }
}

/// Opcode for the ConfigureProviderProperty request
pub const CONFIGURE_PROVIDER_PROPERTY_REQUEST: u8 = 38;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigureProviderPropertyRequest<'input> {
    pub provider: Provider,
    pub property: xproto::Atom,
    pub pending: bool,
    pub range: bool,
    pub values: Cow<'input, [i32]>,
}
impl<'input> ConfigureProviderPropertyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let property_bytes = self.property.serialize();
        let pending_bytes = self.pending.serialize();
        let range_bytes = self.range.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CONFIGURE_PROVIDER_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let values_bytes = self.values.serialize();
        let length_so_far = length_so_far + values_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), values_bytes.into(), padding0.into()], vec![]))
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
        if header.minor_opcode != CONFIGURE_PROVIDER_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (pending, remaining) = bool::try_parse(remaining)?;
        let (range, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let mut remaining = remaining;
        // Length is 'everything left in the input'
        let mut values = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = i32::try_parse(remaining)?;
            remaining = new_remaining;
            values.push(v);
        }
        let _ = remaining;
        Ok(ConfigureProviderPropertyRequest {
            provider,
            property,
            pending,
            range,
            values: Cow::Owned(values),
        })
    }
    /// Clone all borrowed data in this ConfigureProviderPropertyRequest.
    pub fn into_owned(self) -> ConfigureProviderPropertyRequest<'static> {
        ConfigureProviderPropertyRequest {
            provider: self.provider,
            property: self.property,
            pending: self.pending,
            range: self.range,
            values: Cow::Owned(self.values.into_owned()),
        }
    }
}
impl<'input> Request for ConfigureProviderPropertyRequest<'input> {
    type Reply = ();
}
pub fn configure_provider_property<'c, 'input, Conn>(conn: &'c Conn, provider: Provider, property: xproto::Atom, pending: bool, range: bool, values: &'input [i32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ConfigureProviderPropertyRequest {
        provider,
        property,
        pending,
        range,
        values: Cow::Borrowed(values),
    };
    request0.send(conn)
}

/// Opcode for the ChangeProviderProperty request
pub const CHANGE_PROVIDER_PROPERTY_REQUEST: u8 = 39;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeProviderPropertyRequest<'input> {
    pub provider: Provider,
    pub property: xproto::Atom,
    pub type_: xproto::Atom,
    pub format: u8,
    pub mode: u8,
    pub num_items: u32,
    pub data: Cow<'input, [u8]>,
}
impl<'input> ChangeProviderPropertyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let property_bytes = self.property.serialize();
        let type_bytes = self.type_.serialize();
        let format_bytes = self.format.serialize();
        let mode_bytes = self.mode.serialize();
        let num_items_bytes = self.num_items.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CHANGE_PROVIDER_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(self.data.len(), usize::try_from(self.num_items.checked_mul(u32::from(self.format).checked_div(8u32).unwrap()).unwrap()).unwrap(), "`data` has an incorrect length");
        let length_so_far = length_so_far + self.data.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.data, padding0.into()], vec![]))
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
        if header.minor_opcode != CHANGE_PROVIDER_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, num_items.checked_mul(u32::from(format).checked_div(8u32).ok_or(ParseError::InvalidExpression)?).ok_or(ParseError::InvalidExpression)?.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(ChangeProviderPropertyRequest {
            provider,
            property,
            type_,
            format,
            mode,
            num_items,
            data: Cow::Borrowed(data),
        })
    }
    /// Clone all borrowed data in this ChangeProviderPropertyRequest.
    pub fn into_owned(self) -> ChangeProviderPropertyRequest<'static> {
        ChangeProviderPropertyRequest {
            provider: self.provider,
            property: self.property,
            type_: self.type_,
            format: self.format,
            mode: self.mode,
            num_items: self.num_items,
            data: Cow::Owned(self.data.into_owned()),
        }
    }
}
impl<'input> Request for ChangeProviderPropertyRequest<'input> {
    type Reply = ();
}
pub fn change_provider_property<'c, 'input, Conn>(conn: &'c Conn, provider: Provider, property: xproto::Atom, type_: xproto::Atom, format: u8, mode: u8, num_items: u32, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeProviderPropertyRequest {
        provider,
        property,
        type_,
        format,
        mode,
        num_items,
        data: Cow::Borrowed(data),
    };
    request0.send(conn)
}

/// Opcode for the DeleteProviderProperty request
pub const DELETE_PROVIDER_PROPERTY_REQUEST: u8 = 40;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteProviderPropertyRequest {
    pub provider: Provider,
    pub property: xproto::Atom,
}
impl DeleteProviderPropertyRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DELETE_PROVIDER_PROPERTY_REQUEST,
            0,
            0,
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
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
        if header.minor_opcode != DELETE_PROVIDER_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(DeleteProviderPropertyRequest {
            provider,
            property,
        })
    }
}
impl Request for DeleteProviderPropertyRequest {
    type Reply = ();
}
pub fn delete_provider_property<Conn>(conn: &Conn, provider: Provider, property: xproto::Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteProviderPropertyRequest {
        provider,
        property,
    };
    request0.send(conn)
}

/// Opcode for the GetProviderProperty request
pub const GET_PROVIDER_PROPERTY_REQUEST: u8 = 41;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProviderPropertyRequest {
    pub provider: Provider,
    pub property: xproto::Atom,
    pub type_: xproto::Atom,
    pub long_offset: u32,
    pub long_length: u32,
    pub delete: bool,
    pub pending: bool,
}
impl GetProviderPropertyRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let provider_bytes = self.provider.serialize();
        let property_bytes = self.property.serialize();
        let type_bytes = self.type_.serialize();
        let long_offset_bytes = self.long_offset.serialize();
        let long_length_bytes = self.long_length.serialize();
        let delete_bytes = self.delete.serialize();
        let pending_bytes = self.pending.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_PROVIDER_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetProviderPropertyReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROVIDER_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (provider, remaining) = Provider::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (long_offset, remaining) = u32::try_parse(remaining)?;
        let (long_length, remaining) = u32::try_parse(remaining)?;
        let (delete, remaining) = bool::try_parse(remaining)?;
        let (pending, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetProviderPropertyRequest {
            provider,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        })
    }
}
impl Request for GetProviderPropertyRequest {
    type Reply = GetProviderPropertyReply;
}
pub fn get_provider_property<Conn>(conn: &Conn, provider: Provider, property: xproto::Atom, type_: xproto::Atom, long_offset: u32, long_length: u32, delete: bool, pending: bool) -> Result<Cookie<'_, Conn, GetProviderPropertyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetProviderPropertyRequest {
        provider,
        property,
        type_,
        long_offset,
        long_length,
        delete,
        pending,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProviderPropertyReply {
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xproto::Atom,
    pub bytes_after: u32,
    pub num_items: u32,
    pub data: Vec<u8>,
}
impl TryParse for GetProviderPropertyReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, num_items.checked_mul(u32::from(format).checked_div(8u32).ok_or(ParseError::InvalidExpression)?).ok_or(ParseError::InvalidExpression)?.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetProviderPropertyReply { format, sequence, length, type_, bytes_after, num_items, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
impl TryParse for ScreenChangeNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ScreenChangeNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&ScreenChangeNotifyEvent> for [u8; 32] {
    fn from(input: &ScreenChangeNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let rotation_bytes = input.rotation.serialize();
        let sequence_bytes = input.sequence.serialize();
        let timestamp_bytes = input.timestamp.serialize();
        let config_timestamp_bytes = input.config_timestamp.serialize();
        let root_bytes = input.root.serialize();
        let request_window_bytes = input.request_window.serialize();
        let size_id_bytes = input.size_id.serialize();
        let subpixel_order_bytes = u16::from(input.subpixel_order).serialize();
        let width_bytes = input.width.serialize();
        let height_bytes = input.height.serialize();
        let mwidth_bytes = input.mwidth.serialize();
        let mheight_bytes = input.mheight.serialize();
        [
            response_type_bytes[0],
            rotation_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            config_timestamp_bytes[0],
            config_timestamp_bytes[1],
            config_timestamp_bytes[2],
            config_timestamp_bytes[3],
            root_bytes[0],
            root_bytes[1],
            root_bytes[2],
            root_bytes[3],
            request_window_bytes[0],
            request_window_bytes[1],
            request_window_bytes[2],
            request_window_bytes[3],
            size_id_bytes[0],
            size_id_bytes[1],
            subpixel_order_bytes[0],
            subpixel_order_bytes[1],
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
}
impl From<ScreenChangeNotifyEvent> for [u8; 32] {
    fn from(input: ScreenChangeNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for Notify {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for Notify {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
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
    fn serialize(&self) -> [u8; 28] {
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
    fn serialize(&self) -> [u8; 28] {
        let timestamp_bytes = self.timestamp.serialize();
        let config_timestamp_bytes = self.config_timestamp.serialize();
        let window_bytes = self.window.serialize();
        let output_bytes = self.output.serialize();
        let crtc_bytes = self.crtc.serialize();
        let mode_bytes = self.mode.serialize();
        let rotation_bytes = self.rotation.serialize();
        let connection_bytes = u8::from(self.connection).serialize();
        let subpixel_order_bytes = u8::from(self.subpixel_order).serialize();
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
        u8::from(self.connection).serialize_into(bytes);
        u8::from(self.subpixel_order).serialize_into(bytes);
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
        let remaining = remaining.get(11..).ok_or(ParseError::InsufficientData)?;
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
    fn serialize(&self) -> [u8; 28] {
        let window_bytes = self.window.serialize();
        let output_bytes = self.output.serialize();
        let atom_bytes = self.atom.serialize();
        let timestamp_bytes = self.timestamp.serialize();
        let status_bytes = u8::from(self.status).serialize();
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
        u8::from(self.status).serialize_into(bytes);
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
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
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
    fn serialize(&self) -> [u8; 28] {
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
        let remaining = remaining.get(11..).ok_or(ParseError::InsufficientData)?;
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
    fn serialize(&self) -> [u8; 28] {
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
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
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
    fn serialize(&self) -> [u8; 28] {
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
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, n_output.try_into().or(Err(ParseError::ConversionFailed))?)?;
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
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.name.serialize_into(bytes);
        self.primary.serialize_into(bytes);
        self.automatic.serialize_into(bytes);
        let n_output = u16::try_from(self.outputs.len()).expect("`outputs` has too many elements");
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
impl MonitorInfo {
    /// Get the value of the `nOutput` field.
    ///
    /// The `nOutput` field is used as the length field of the `outputs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_output(&self) -> u16 {
        self.outputs.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetMonitors request
pub const GET_MONITORS_REQUEST: u8 = 42;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMonitorsRequest {
    pub window: xproto::Window,
    pub get_active: bool,
}
impl GetMonitorsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let get_active_bytes = self.get_active.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_MONITORS_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            get_active_bytes[0],
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetMonitorsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MONITORS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (get_active, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMonitorsRequest {
            window,
            get_active,
        })
    }
}
impl Request for GetMonitorsRequest {
    type Reply = GetMonitorsReply;
}
pub fn get_monitors<Conn>(conn: &Conn, window: xproto::Window, get_active: bool) -> Result<Cookie<'_, Conn, GetMonitorsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMonitorsRequest {
        window,
        get_active,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMonitorsReply {
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xproto::Timestamp,
    pub n_outputs: u32,
    pub monitors: Vec<MonitorInfo>,
}
impl TryParse for GetMonitorsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (n_monitors, remaining) = u32::try_parse(remaining)?;
        let (n_outputs, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (monitors, remaining) = crate::x11_utils::parse_list::<MonitorInfo>(remaining, n_monitors.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMonitorsReply { sequence, length, timestamp, n_outputs, monitors };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMonitorsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetMonitorsReply {
    /// Get the value of the `nMonitors` field.
    ///
    /// The `nMonitors` field is used as the length field of the `monitors` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_monitors(&self) -> u32 {
        self.monitors.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetMonitor request
pub const SET_MONITOR_REQUEST: u8 = 43;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetMonitorRequest {
    pub window: xproto::Window,
    pub monitorinfo: MonitorInfo,
}
impl SetMonitorRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_MONITOR_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let monitorinfo_bytes = self.monitorinfo.serialize();
        let length_so_far = length_so_far + monitorinfo_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), monitorinfo_bytes.into(), padding0.into()], vec![]))
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
        if header.minor_opcode != SET_MONITOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (monitorinfo, remaining) = MonitorInfo::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetMonitorRequest {
            window,
            monitorinfo,
        })
    }
}
impl Request for SetMonitorRequest {
    type Reply = ();
}
pub fn set_monitor<Conn>(conn: &Conn, window: xproto::Window, monitorinfo: MonitorInfo) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetMonitorRequest {
        window,
        monitorinfo,
    };
    request0.send(conn)
}

/// Opcode for the DeleteMonitor request
pub const DELETE_MONITOR_REQUEST: u8 = 44;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteMonitorRequest {
    pub window: xproto::Window,
    pub name: xproto::Atom,
}
impl DeleteMonitorRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let name_bytes = self.name.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DELETE_MONITOR_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
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
        if header.minor_opcode != DELETE_MONITOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(DeleteMonitorRequest {
            window,
            name,
        })
    }
}
impl Request for DeleteMonitorRequest {
    type Reply = ();
}
pub fn delete_monitor<Conn>(conn: &Conn, window: xproto::Window, name: xproto::Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteMonitorRequest {
        window,
        name,
    };
    request0.send(conn)
}

/// Opcode for the CreateLease request
pub const CREATE_LEASE_REQUEST: u8 = 45;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateLeaseRequest<'input> {
    pub window: xproto::Window,
    pub lid: Lease,
    pub crtcs: Cow<'input, [Crtc]>,
    pub outputs: Cow<'input, [Output]>,
}
impl<'input> CreateLeaseRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let lid_bytes = self.lid.serialize();
        let num_crtcs = u16::try_from(self.crtcs.len()).expect("`crtcs` has too many elements");
        let num_crtcs_bytes = num_crtcs.serialize();
        let num_outputs = u16::try_from(self.outputs.len()).expect("`outputs` has too many elements");
        let num_outputs_bytes = num_outputs.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_LEASE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let crtcs_bytes = self.crtcs.serialize();
        let length_so_far = length_so_far + crtcs_bytes.len();
        let outputs_bytes = self.outputs.serialize();
        let length_so_far = length_so_far + outputs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), crtcs_bytes.into(), outputs_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<CookieWithFds<'_, Conn, CreateLeaseReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply_with_fds(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_LEASE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (lid, remaining) = Lease::try_parse(remaining)?;
        let (num_crtcs, remaining) = u16::try_parse(remaining)?;
        let (num_outputs, remaining) = u16::try_parse(remaining)?;
        let (crtcs, remaining) = crate::x11_utils::parse_list::<Crtc>(remaining, num_crtcs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (outputs, remaining) = crate::x11_utils::parse_list::<Output>(remaining, num_outputs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(CreateLeaseRequest {
            window,
            lid,
            crtcs: Cow::Owned(crtcs),
            outputs: Cow::Owned(outputs),
        })
    }
    /// Clone all borrowed data in this CreateLeaseRequest.
    pub fn into_owned(self) -> CreateLeaseRequest<'static> {
        CreateLeaseRequest {
            window: self.window,
            lid: self.lid,
            crtcs: Cow::Owned(self.crtcs.into_owned()),
            outputs: Cow::Owned(self.outputs.into_owned()),
        }
    }
}
impl<'input> Request for CreateLeaseRequest<'input> {
    type Reply = CreateLeaseReply;
}
pub fn create_lease<'c, 'input, Conn>(conn: &'c Conn, window: xproto::Window, lid: Lease, crtcs: &'input [Crtc], outputs: &'input [Output]) -> Result<CookieWithFds<'c, Conn, CreateLeaseReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateLeaseRequest {
        window,
        lid,
        crtcs: Cow::Borrowed(crtcs),
        outputs: Cow::Borrowed(outputs),
    };
    request0.send(conn)
}

#[derive(Debug, PartialEq, Eq)]
pub struct CreateLeaseReply {
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub master_fd: RawFdContainer,
}
impl TryParseFd for CreateLeaseReply {
    fn try_parse_fd<'a>(initial_value: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::MissingFileDescriptors) }
        let master_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = CreateLeaseReply { nfd, sequence, length, master_fd };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeLeaseRequest {
    pub lid: Lease,
    pub terminate: u8,
}
impl FreeLeaseRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let lid_bytes = self.lid.serialize();
        let terminate_bytes = self.terminate.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            FREE_LEASE_REQUEST,
            0,
            0,
            lid_bytes[0],
            lid_bytes[1],
            lid_bytes[2],
            lid_bytes[3],
            terminate_bytes[0],
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
        if header.minor_opcode != FREE_LEASE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (lid, remaining) = Lease::try_parse(value)?;
        let (terminate, remaining) = u8::try_parse(remaining)?;
        let _ = remaining;
        Ok(FreeLeaseRequest {
            lid,
            terminate,
        })
    }
}
impl Request for FreeLeaseRequest {
    type Reply = ();
}
pub fn free_lease<Conn>(conn: &Conn, lid: Lease, terminate: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FreeLeaseRequest {
        lid,
        terminate,
    };
    request0.send(conn)
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
        let remaining = remaining.get(15..).ok_or(ParseError::InsufficientData)?;
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
    fn serialize(&self) -> [u8; 28] {
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
    pub fn as_cc(&self) -> CrtcChange {
        fn do_the_parse(remaining: &[u8]) -> Result<CrtcChange, ParseError> {
            let (cc, remaining) = CrtcChange::try_parse(remaining)?;
            let _ = remaining;
            Ok(cc)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_oc(&self) -> OutputChange {
        fn do_the_parse(remaining: &[u8]) -> Result<OutputChange, ParseError> {
            let (oc, remaining) = OutputChange::try_parse(remaining)?;
            let _ = remaining;
            Ok(oc)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_op(&self) -> OutputProperty {
        fn do_the_parse(remaining: &[u8]) -> Result<OutputProperty, ParseError> {
            let (op, remaining) = OutputProperty::try_parse(remaining)?;
            let _ = remaining;
            Ok(op)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_pc(&self) -> ProviderChange {
        fn do_the_parse(remaining: &[u8]) -> Result<ProviderChange, ParseError> {
            let (pc, remaining) = ProviderChange::try_parse(remaining)?;
            let _ = remaining;
            Ok(pc)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_pp(&self) -> ProviderProperty {
        fn do_the_parse(remaining: &[u8]) -> Result<ProviderProperty, ParseError> {
            let (pp, remaining) = ProviderProperty::try_parse(remaining)?;
            let _ = remaining;
            Ok(pp)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_rc(&self) -> ResourceChange {
        fn do_the_parse(remaining: &[u8]) -> Result<ResourceChange, ParseError> {
            let (rc, remaining) = ResourceChange::try_parse(remaining)?;
            let _ = remaining;
            Ok(rc)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_lc(&self) -> LeaseNotify {
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
    fn serialize(&self) -> [u8; 28] {
        self.0
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}
impl TryParse for NotifyData {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner: [u8; 28] = value.get(..28)
            .ok_or(ParseError::InsufficientData)?
            .try_into()
            .unwrap();
        let result = NotifyData(inner);
        Ok((result, &value[28..]))
    }
}
impl From<CrtcChange> for NotifyData {
    fn from(cc: CrtcChange) -> Self {
        let cc_bytes = cc.serialize();
        Self(cc_bytes)
    }
}
impl From<OutputChange> for NotifyData {
    fn from(oc: OutputChange) -> Self {
        let oc_bytes = oc.serialize();
        Self(oc_bytes)
    }
}
impl From<OutputProperty> for NotifyData {
    fn from(op: OutputProperty) -> Self {
        let op_bytes = op.serialize();
        Self(op_bytes)
    }
}
impl From<ProviderChange> for NotifyData {
    fn from(pc: ProviderChange) -> Self {
        let pc_bytes = pc.serialize();
        Self(pc_bytes)
    }
}
impl From<ProviderProperty> for NotifyData {
    fn from(pp: ProviderProperty) -> Self {
        let pp_bytes = pp.serialize();
        Self(pp_bytes)
    }
}
impl From<ResourceChange> for NotifyData {
    fn from(rc: ResourceChange) -> Self {
        let rc_bytes = rc.serialize();
        Self(rc_bytes)
    }
}
impl From<LeaseNotify> for NotifyData {
    fn from(lc: LeaseNotify) -> Self {
        let lc_bytes = lc.serialize();
        Self(lc_bytes)
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
impl TryParse for NotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (sub_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (u, remaining) = NotifyData::try_parse(remaining)?;
        let sub_code = sub_code.try_into()?;
        let result = NotifyEvent { response_type, sub_code, sequence, u };
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
        let sub_code_bytes = u8::from(input.sub_code).serialize();
        let sequence_bytes = input.sequence.serialize();
        let u_bytes = input.u.serialize();
        [
            response_type_bytes[0],
            sub_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            u_bytes[0],
            u_bytes[1],
            u_bytes[2],
            u_bytes[3],
            u_bytes[4],
            u_bytes[5],
            u_bytes[6],
            u_bytes[7],
            u_bytes[8],
            u_bytes[9],
            u_bytes[10],
            u_bytes[11],
            u_bytes[12],
            u_bytes[13],
            u_bytes[14],
            u_bytes[15],
            u_bytes[16],
            u_bytes[17],
            u_bytes[18],
            u_bytes[19],
            u_bytes[20],
            u_bytes[21],
            u_bytes[22],
            u_bytes[23],
            u_bytes[24],
            u_bytes[25],
            u_bytes[26],
            u_bytes[27],
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
    fn randr_set_screen_config<A>(&self, window: xproto::Window, timestamp: xproto::Timestamp, config_timestamp: xproto::Timestamp, size_id: u16, rotation: A, rate: u16) -> Result<Cookie<'_, Self, SetScreenConfigReply>, ConnectionError>
    where
        A: Into<u16>,
    {
        set_screen_config(self, window, timestamp, config_timestamp, size_id, rotation, rate)
    }
    fn randr_select_input<A>(&self, window: xproto::Window, enable: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u16>,
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
    fn randr_configure_output_property<'c, 'input>(&'c self, output: Output, property: xproto::Atom, pending: bool, range: bool, values: &'input [i32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        configure_output_property(self, output, property, pending, range, values)
    }
    fn randr_change_output_property<'c, 'input>(&'c self, output: Output, property: xproto::Atom, type_: xproto::Atom, format: u8, mode: xproto::PropMode, num_units: u32, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_output_property(self, output, property, type_, format, mode, num_units, data)
    }
    fn randr_delete_output_property(&self, output: Output, property: xproto::Atom) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_output_property(self, output, property)
    }
    fn randr_get_output_property<A>(&self, output: Output, property: xproto::Atom, type_: A, long_offset: u32, long_length: u32, delete: bool, pending: bool) -> Result<Cookie<'_, Self, GetOutputPropertyReply>, ConnectionError>
    where
        A: Into<xproto::Atom>,
    {
        get_output_property(self, output, property, type_, long_offset, long_length, delete, pending)
    }
    fn randr_create_mode<'c, 'input>(&'c self, window: xproto::Window, mode_info: ModeInfo, name: &'input [u8]) -> Result<Cookie<'c, Self, CreateModeReply>, ConnectionError>
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
    fn randr_set_crtc_config<'c, 'input, A>(&'c self, crtc: Crtc, timestamp: xproto::Timestamp, config_timestamp: xproto::Timestamp, x: i16, y: i16, mode: Mode, rotation: A, outputs: &'input [Output]) -> Result<Cookie<'c, Self, SetCrtcConfigReply>, ConnectionError>
    where
        A: Into<u16>,
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
    fn randr_set_crtc_gamma<'c, 'input>(&'c self, crtc: Crtc, red: &'input [u16], green: &'input [u16], blue: &'input [u16]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_crtc_gamma(self, crtc, red, green, blue)
    }
    fn randr_get_screen_resources_current(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetScreenResourcesCurrentReply>, ConnectionError>
    {
        get_screen_resources_current(self, window)
    }
    fn randr_set_crtc_transform<'c, 'input>(&'c self, crtc: Crtc, transform: render::Transform, filter_name: &'input [u8], filter_params: &'input [render::Fixed]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn randr_configure_provider_property<'c, 'input>(&'c self, provider: Provider, property: xproto::Atom, pending: bool, range: bool, values: &'input [i32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        configure_provider_property(self, provider, property, pending, range, values)
    }
    fn randr_change_provider_property<'c, 'input>(&'c self, provider: Provider, property: xproto::Atom, type_: xproto::Atom, format: u8, mode: u8, num_items: u32, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn randr_create_lease<'c, 'input>(&'c self, window: xproto::Window, lid: Lease, crtcs: &'input [Crtc], outputs: &'input [Output]) -> Result<CookieWithFds<'c, Self, CreateLeaseReply>, ConnectionError>
    {
        create_lease(self, window, lid, crtcs, outputs)
    }
    fn randr_free_lease(&self, lid: Lease, terminate: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_lease(self, lid, terminate)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
