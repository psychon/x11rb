// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `DPMS` X11 extension.

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
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "DPMS";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (0, 0);

/// Opcode for the GetVersion request
pub const GET_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVersionRequest {
    pub client_major_version: u16,
    pub client_minor_version: u16,
}
impl GetVersionRequest {
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
            GET_VERSION_REQUEST,
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
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_VERSION_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (client_major_version, remaining) = u16::try_parse(value)?;
        let (client_minor_version, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetVersionRequest {
            client_major_version,
            client_minor_version,
        })
    }
}
impl Request for GetVersionRequest {
    type Reply = GetVersionReply;
}
pub fn get_version<Conn>(conn: &Conn, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Conn, GetVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetVersionRequest {
        client_major_version,
        client_minor_version,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl TryParse for GetVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major_version, remaining) = u16::try_parse(remaining)?;
        let (server_minor_version, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = GetVersionReply { sequence, length, server_major_version, server_minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the Capable request
pub const CAPABLE_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapableRequest;
impl CapableRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            CAPABLE_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CAPABLE_REQUEST {
            return Err(ParseError::ParseError);
        }
        let _ = value;
        Ok(CapableRequest
        )
    }
}
impl Request for CapableRequest {
    type Reply = CapableReply;
}
pub fn capable<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, CapableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CapableRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapableReply {
    pub sequence: u16,
    pub length: u32,
    pub capable: bool,
}
impl TryParse for CapableReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (capable, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = CapableReply { sequence, length, capable };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CapableReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTimeouts request
pub const GET_TIMEOUTS_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeoutsRequest;
impl GetTimeoutsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_TIMEOUTS_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TIMEOUTS_REQUEST {
            return Err(ParseError::ParseError);
        }
        let _ = value;
        Ok(GetTimeoutsRequest
        )
    }
}
impl Request for GetTimeoutsRequest {
    type Reply = GetTimeoutsReply;
}
pub fn get_timeouts<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetTimeoutsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTimeoutsRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeoutsReply {
    pub sequence: u16,
    pub length: u32,
    pub standby_timeout: u16,
    pub suspend_timeout: u16,
    pub off_timeout: u16,
}
impl TryParse for GetTimeoutsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (standby_timeout, remaining) = u16::try_parse(remaining)?;
        let (suspend_timeout, remaining) = u16::try_parse(remaining)?;
        let (off_timeout, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = GetTimeoutsReply { sequence, length, standby_timeout, suspend_timeout, off_timeout };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTimeoutsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetTimeouts request
pub const SET_TIMEOUTS_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetTimeoutsRequest {
    pub standby_timeout: u16,
    pub suspend_timeout: u16,
    pub off_timeout: u16,
}
impl SetTimeoutsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let standby_timeout_bytes = self.standby_timeout.serialize();
        let suspend_timeout_bytes = self.suspend_timeout.serialize();
        let off_timeout_bytes = self.off_timeout.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_TIMEOUTS_REQUEST,
            0,
            0,
            standby_timeout_bytes[0],
            standby_timeout_bytes[1],
            suspend_timeout_bytes[0],
            suspend_timeout_bytes[1],
            off_timeout_bytes[0],
            off_timeout_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_TIMEOUTS_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (standby_timeout, remaining) = u16::try_parse(value)?;
        let (suspend_timeout, remaining) = u16::try_parse(remaining)?;
        let (off_timeout, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetTimeoutsRequest {
            standby_timeout,
            suspend_timeout,
            off_timeout,
        })
    }
}
impl Request for SetTimeoutsRequest {
    type Reply = ();
}
pub fn set_timeouts<Conn>(conn: &Conn, standby_timeout: u16, suspend_timeout: u16, off_timeout: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetTimeoutsRequest {
        standby_timeout,
        suspend_timeout,
        off_timeout,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the Enable request
pub const ENABLE_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableRequest;
impl EnableRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            ENABLE_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != ENABLE_REQUEST {
            return Err(ParseError::ParseError);
        }
        let _ = value;
        Ok(EnableRequest
        )
    }
}
impl Request for EnableRequest {
    type Reply = ();
}
pub fn enable<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = EnableRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the Disable request
pub const DISABLE_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableRequest;
impl DisableRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            DISABLE_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DISABLE_REQUEST {
            return Err(ParseError::ParseError);
        }
        let _ = value;
        Ok(DisableRequest
        )
    }
}
impl Request for DisableRequest {
    type Reply = ();
}
pub fn disable<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DisableRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DPMSMode {
    On = 0,
    Standby = 1,
    Suspend = 2,
    Off = 3,
}
impl From<DPMSMode> for u8 {
    fn from(input: DPMSMode) -> Self {
        match input {
            DPMSMode::On => 0,
            DPMSMode::Standby => 1,
            DPMSMode::Suspend => 2,
            DPMSMode::Off => 3,
        }
    }
}
impl From<DPMSMode> for Option<u8> {
    fn from(input: DPMSMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<DPMSMode> for u16 {
    fn from(input: DPMSMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DPMSMode> for Option<u16> {
    fn from(input: DPMSMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<DPMSMode> for u32 {
    fn from(input: DPMSMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DPMSMode> for Option<u32> {
    fn from(input: DPMSMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DPMSMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DPMSMode::On),
            1 => Ok(DPMSMode::Standby),
            2 => Ok(DPMSMode::Suspend),
            3 => Ok(DPMSMode::Off),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for DPMSMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DPMSMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ForceLevel request
pub const FORCE_LEVEL_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForceLevelRequest {
    pub power_level: DPMSMode,
}
impl ForceLevelRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let power_level_bytes = u16::from(self.power_level).serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            FORCE_LEVEL_REQUEST,
            0,
            0,
            power_level_bytes[0],
            power_level_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != FORCE_LEVEL_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (power_level, remaining) = u16::try_parse(value)?;
        let power_level = power_level.try_into()?;
        let _ = remaining;
        Ok(ForceLevelRequest {
            power_level,
        })
    }
}
impl Request for ForceLevelRequest {
    type Reply = ();
}
pub fn force_level<Conn>(conn: &Conn, power_level: DPMSMode) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ForceLevelRequest {
        power_level,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the Info request
pub const INFO_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfoRequest;
impl InfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            INFO_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != INFO_REQUEST {
            return Err(ParseError::ParseError);
        }
        let _ = value;
        Ok(InfoRequest
        )
    }
}
impl Request for InfoRequest {
    type Reply = InfoReply;
}
pub fn info<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, InfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InfoRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfoReply {
    pub sequence: u16,
    pub length: u32,
    pub power_level: DPMSMode,
    pub state: bool,
}
impl TryParse for InfoReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (power_level, remaining) = u16::try_parse(remaining)?;
        let (state, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let power_level = power_level.try_into()?;
        let result = InfoReply { sequence, length, power_level, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn dpms_get_version(&self, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Self, GetVersionReply>, ConnectionError>
    {
        get_version(self, client_major_version, client_minor_version)
    }
    fn dpms_capable(&self) -> Result<Cookie<'_, Self, CapableReply>, ConnectionError>
    {
        capable(self)
    }
    fn dpms_get_timeouts(&self) -> Result<Cookie<'_, Self, GetTimeoutsReply>, ConnectionError>
    {
        get_timeouts(self)
    }
    fn dpms_set_timeouts(&self, standby_timeout: u16, suspend_timeout: u16, off_timeout: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_timeouts(self, standby_timeout, suspend_timeout, off_timeout)
    }
    fn dpms_enable(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        enable(self)
    }
    fn dpms_disable(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        disable(self)
    }
    fn dpms_force_level(&self, power_level: DPMSMode) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        force_level(self, power_level)
    }
    fn dpms_info(&self) -> Result<Cookie<'_, Self, InfoReply>, ConnectionError>
    {
        info(self)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
