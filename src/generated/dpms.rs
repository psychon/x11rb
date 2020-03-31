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
use crate::x11_utils::{TryParse, Serialize};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ParseError, ConnectionError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;

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
pub fn get_version<Conn>(conn: &Conn, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Conn, GetVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let client_major_version_bytes = client_major_version.serialize();
    let client_minor_version_bytes = client_minor_version.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        client_major_version_bytes[0],
        client_major_version_bytes[1],
        client_minor_version_bytes[0],
        client_minor_version_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}
impl GetVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major_version, remaining) = u16::try_parse(remaining)?;
        let (server_minor_version, remaining) = u16::try_parse(remaining)?;
        let result = GetVersionReply { response_type, sequence, length, server_major_version, server_minor_version };
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
pub fn capable<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, CapableReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        CAPABLE_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub capable: bool,
}
impl CapableReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (capable, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let result = CapableReply { response_type, sequence, length, capable };
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
pub fn get_timeouts<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetTimeoutsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TIMEOUTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeoutsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub standby_timeout: u16,
    pub suspend_timeout: u16,
    pub off_timeout: u16,
}
impl GetTimeoutsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (standby_timeout, remaining) = u16::try_parse(remaining)?;
        let (suspend_timeout, remaining) = u16::try_parse(remaining)?;
        let (off_timeout, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let result = GetTimeoutsReply { response_type, sequence, length, standby_timeout, suspend_timeout, off_timeout };
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
pub fn set_timeouts<Conn>(conn: &Conn, standby_timeout: u16, suspend_timeout: u16, off_timeout: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let standby_timeout_bytes = standby_timeout.serialize();
    let suspend_timeout_bytes = suspend_timeout.serialize();
    let off_timeout_bytes = off_timeout.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_TIMEOUTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        standby_timeout_bytes[0],
        standby_timeout_bytes[1],
        suspend_timeout_bytes[0],
        suspend_timeout_bytes[1],
        off_timeout_bytes[0],
        off_timeout_bytes[1],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Enable request
pub const ENABLE_REQUEST: u8 = 4;
pub fn enable<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        ENABLE_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Disable request
pub const DISABLE_REQUEST: u8 = 5;
pub fn disable<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        DISABLE_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
            _ => Err(ParseError::ParseError)
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
pub fn force_level<Conn, A>(conn: &Conn, power_level: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u16>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let power_level = power_level.into();
    let power_level_bytes = power_level.serialize();
    let request0 = [
        extension_information.major_opcode,
        FORCE_LEVEL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        power_level_bytes[0],
        power_level_bytes[1],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Info request
pub const INFO_REQUEST: u8 = 7;
pub fn info<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, InfoReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        INFO_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub power_level: DPMSMode,
    pub state: bool,
}
impl InfoReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (power_level, remaining) = u16::try_parse(remaining)?;
        let (state, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let power_level = power_level.try_into()?;
        let result = InfoReply { response_type, sequence, length, power_level, state };
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

    fn dpms_force_level<A>(&self, power_level: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u16>
    {
        force_level(self, power_level)
    }

    fn dpms_info(&self) -> Result<Cookie<'_, Self, InfoReply>, ConnectionError>
    {
        info(self)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
