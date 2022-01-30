// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XF86VidMode` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd, TryIntoUSize};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::connection::Connection as X11Connection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
#[allow(unused_imports)]
use crate::errors::ReplyOrIdError;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XFree86-VidModeExtension";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (2, 2);

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub type Syncrange = u32;

pub type Dotclock = u32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ModeFlag(u16);
impl ModeFlag {
    pub const POSITIVE_H_SYNC: Self = Self(1 << 0);
    pub const NEGATIVE_H_SYNC: Self = Self(1 << 1);
    pub const POSITIVE_V_SYNC: Self = Self(1 << 2);
    pub const NEGATIVE_V_SYNC: Self = Self(1 << 3);
    pub const INTERLACE: Self = Self(1 << 4);
    pub const COMPOSITE_SYNC: Self = Self(1 << 5);
    pub const POSITIVE_C_SYNC: Self = Self(1 << 6);
    pub const NEGATIVE_C_SYNC: Self = Self(1 << 7);
    pub const H_SKEW: Self = Self(1 << 8);
    pub const BROADCAST: Self = Self(1 << 9);
    pub const PIXMUX: Self = Self(1 << 10);
    pub const DOUBLE_CLOCK: Self = Self(1 << 11);
    pub const HALF_CLOCK: Self = Self(1 << 12);
}
impl From<ModeFlag> for u16 {
    #[inline]
    fn from(input: ModeFlag) -> Self {
        input.0
    }
}
impl From<ModeFlag> for Option<u16> {
    #[inline]
    fn from(input: ModeFlag) -> Self {
        Some(input.0)
    }
}
impl From<ModeFlag> for u32 {
    #[inline]
    fn from(input: ModeFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<ModeFlag> for Option<u32> {
    #[inline]
    fn from(input: ModeFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ModeFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for ModeFlag {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ModeFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::POSITIVE_H_SYNC.0.into(), "POSITIVE_H_SYNC", "PositiveHSync"),
            (Self::NEGATIVE_H_SYNC.0.into(), "NEGATIVE_H_SYNC", "NegativeHSync"),
            (Self::POSITIVE_V_SYNC.0.into(), "POSITIVE_V_SYNC", "PositiveVSync"),
            (Self::NEGATIVE_V_SYNC.0.into(), "NEGATIVE_V_SYNC", "NegativeVSync"),
            (Self::INTERLACE.0.into(), "INTERLACE", "Interlace"),
            (Self::COMPOSITE_SYNC.0.into(), "COMPOSITE_SYNC", "CompositeSync"),
            (Self::POSITIVE_C_SYNC.0.into(), "POSITIVE_C_SYNC", "PositiveCSync"),
            (Self::NEGATIVE_C_SYNC.0.into(), "NEGATIVE_C_SYNC", "NegativeCSync"),
            (Self::H_SKEW.0.into(), "H_SKEW", "HSkew"),
            (Self::BROADCAST.0.into(), "BROADCAST", "Broadcast"),
            (Self::PIXMUX.0.into(), "PIXMUX", "Pixmux"),
            (Self::DOUBLE_CLOCK.0.into(), "DOUBLE_CLOCK", "DoubleClock"),
            (Self::HALF_CLOCK.0.into(), "HALF_CLOCK", "HalfClock"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(ModeFlag, u16);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClockFlag(u8);
impl ClockFlag {
    pub const PROGRAMABLE: Self = Self(1 << 0);
}
impl From<ClockFlag> for u8 {
    #[inline]
    fn from(input: ClockFlag) -> Self {
        input.0
    }
}
impl From<ClockFlag> for Option<u8> {
    #[inline]
    fn from(input: ClockFlag) -> Self {
        Some(input.0)
    }
}
impl From<ClockFlag> for u16 {
    #[inline]
    fn from(input: ClockFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<ClockFlag> for Option<u16> {
    #[inline]
    fn from(input: ClockFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ClockFlag> for u32 {
    #[inline]
    fn from(input: ClockFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<ClockFlag> for Option<u32> {
    #[inline]
    fn from(input: ClockFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ClockFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ClockFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::PROGRAMABLE.0.into(), "PROGRAMABLE", "Programable"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(ClockFlag, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Permission(u8);
impl Permission {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
}
impl From<Permission> for u8 {
    #[inline]
    fn from(input: Permission) -> Self {
        input.0
    }
}
impl From<Permission> for Option<u8> {
    #[inline]
    fn from(input: Permission) -> Self {
        Some(input.0)
    }
}
impl From<Permission> for u16 {
    #[inline]
    fn from(input: Permission) -> Self {
        u16::from(input.0)
    }
}
impl From<Permission> for Option<u16> {
    #[inline]
    fn from(input: Permission) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Permission> for u32 {
    #[inline]
    fn from(input: Permission) -> Self {
        u32::from(input.0)
    }
}
impl From<Permission> for Option<u32> {
    #[inline]
    fn from(input: Permission) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Permission {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Permission  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::READ.0.into(), "READ", "Read"),
            (Self::WRITE.0.into(), "WRITE", "Write"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(Permission, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeInfo {
    pub dotclock: Dotclock,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u32,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub privsize: u32,
}
impl TryParse for ModeInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (dotclock, remaining) = Dotclock::try_parse(remaining)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u32::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let result = ModeInfo { dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize };
        Ok((result, remaining))
    }
}
impl Serialize for ModeInfo {
    type Bytes = [u8; 48];
    fn serialize(&self) -> [u8; 48] {
        let dotclock_bytes = self.dotclock.serialize();
        let hdisplay_bytes = self.hdisplay.serialize();
        let hsyncstart_bytes = self.hsyncstart.serialize();
        let hsyncend_bytes = self.hsyncend.serialize();
        let htotal_bytes = self.htotal.serialize();
        let hskew_bytes = self.hskew.serialize();
        let vdisplay_bytes = self.vdisplay.serialize();
        let vsyncstart_bytes = self.vsyncstart.serialize();
        let vsyncend_bytes = self.vsyncend.serialize();
        let vtotal_bytes = self.vtotal.serialize();
        let flags_bytes = self.flags.serialize();
        let privsize_bytes = self.privsize.serialize();
        [
            dotclock_bytes[0],
            dotclock_bytes[1],
            dotclock_bytes[2],
            dotclock_bytes[3],
            hdisplay_bytes[0],
            hdisplay_bytes[1],
            hsyncstart_bytes[0],
            hsyncstart_bytes[1],
            hsyncend_bytes[0],
            hsyncend_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            hskew_bytes[2],
            hskew_bytes[3],
            vdisplay_bytes[0],
            vdisplay_bytes[1],
            vsyncstart_bytes[0],
            vsyncstart_bytes[1],
            vsyncend_bytes[0],
            vsyncend_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            0,
            0,
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            privsize_bytes[0],
            privsize_bytes[1],
            privsize_bytes[2],
            privsize_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(48);
        self.dotclock.serialize_into(bytes);
        self.hdisplay.serialize_into(bytes);
        self.hsyncstart.serialize_into(bytes);
        self.hsyncend.serialize_into(bytes);
        self.htotal.serialize_into(bytes);
        self.hskew.serialize_into(bytes);
        self.vdisplay.serialize_into(bytes);
        self.vsyncstart.serialize_into(bytes);
        self.vsyncend.serialize_into(bytes);
        self.vtotal.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.privsize.serialize_into(bytes);
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest;
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(QueryVersionRequest
        )
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryVersionRequest {}
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
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

/// Opcode for the GetModeLine request
pub const GET_MODE_LINE_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetModeLineRequest {
    pub screen: u16,
}
impl GetModeLineRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MODE_LINE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetModeLineReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MODE_LINE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetModeLineRequest {
            screen,
        })
    }
}
impl Request for GetModeLineRequest {
    type Reply = GetModeLineReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetModeLineRequest {}
pub fn get_mode_line<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetModeLineReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetModeLineRequest {
        screen,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetModeLineReply {
    pub sequence: u16,
    pub length: u32,
    pub dotclock: Dotclock,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub private: Vec<u8>,
}
impl TryParse for GetModeLineReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (dotclock, remaining) = Dotclock::try_parse(remaining)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u16::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let (private, remaining) = crate::x11_utils::parse_u8_list(remaining, privsize.try_to_usize()?)?;
        let private = private.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetModeLineReply { sequence, length, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetModeLineReply {
    /// Get the value of the `privsize` field.
    ///
    /// The `privsize` field is used as the length field of the `private` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn privsize(&self) -> u32 {
        self.private.len()
            .try_into().unwrap()
    }
}

/// Opcode for the ModModeLine request
pub const MOD_MODE_LINE_REQUEST: u8 = 2;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModModeLineRequest<'input> {
    pub screen: u32,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub private: Cow<'input, [u8]>,
}
impl<'input> ModModeLineRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let hdisplay_bytes = self.hdisplay.serialize();
        let hsyncstart_bytes = self.hsyncstart.serialize();
        let hsyncend_bytes = self.hsyncend.serialize();
        let htotal_bytes = self.htotal.serialize();
        let hskew_bytes = self.hskew.serialize();
        let vdisplay_bytes = self.vdisplay.serialize();
        let vsyncstart_bytes = self.vsyncstart.serialize();
        let vsyncend_bytes = self.vsyncend.serialize();
        let vtotal_bytes = self.vtotal.serialize();
        let flags_bytes = self.flags.serialize();
        let privsize = u32::try_from(self.private.len()).expect("`private` has too many elements");
        let privsize_bytes = privsize.serialize();
        let mut request0 = vec![
            major_opcode,
            MOD_MODE_LINE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            hdisplay_bytes[0],
            hdisplay_bytes[1],
            hsyncstart_bytes[0],
            hsyncstart_bytes[1],
            hsyncend_bytes[0],
            hsyncend_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            vdisplay_bytes[0],
            vdisplay_bytes[1],
            vsyncstart_bytes[0],
            vsyncstart_bytes[1],
            vsyncend_bytes[0],
            vsyncend_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            privsize_bytes[0],
            privsize_bytes[1],
            privsize_bytes[2],
            privsize_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.private.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.private, padding0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != MOD_MODE_LINE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u16::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let (private, remaining) = crate::x11_utils::parse_u8_list(remaining, privsize.try_to_usize()?)?;
        let _ = remaining;
        Ok(ModModeLineRequest {
            screen,
            hdisplay,
            hsyncstart,
            hsyncend,
            htotal,
            hskew,
            vdisplay,
            vsyncstart,
            vsyncend,
            vtotal,
            flags,
            private: Cow::Borrowed(private),
        })
    }
    /// Clone all borrowed data in this ModModeLineRequest.
    pub fn into_owned(self) -> ModModeLineRequest<'static> {
        ModModeLineRequest {
            screen: self.screen,
            hdisplay: self.hdisplay,
            hsyncstart: self.hsyncstart,
            hsyncend: self.hsyncend,
            htotal: self.htotal,
            hskew: self.hskew,
            vdisplay: self.vdisplay,
            vsyncstart: self.vsyncstart,
            vsyncend: self.vsyncend,
            vtotal: self.vtotal,
            flags: self.flags,
            private: Cow::Owned(self.private.into_owned()),
        }
    }
}
impl<'input> Request for ModModeLineRequest<'input> {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for ModModeLineRequest<'input> {}
pub fn mod_mode_line<'c, 'input, Conn, A>(conn: &'c Conn, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = ModModeLineRequest {
        screen,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    request0.send(conn)
}

/// Opcode for the SwitchMode request
pub const SWITCH_MODE_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchModeRequest {
    pub screen: u16,
    pub zoom: u16,
}
impl SwitchModeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let zoom_bytes = self.zoom.serialize();
        let mut request0 = vec![
            major_opcode,
            SWITCH_MODE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            zoom_bytes[0],
            zoom_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SWITCH_MODE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let (zoom, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(SwitchModeRequest {
            screen,
            zoom,
        })
    }
}
impl Request for SwitchModeRequest {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SwitchModeRequest {}
pub fn switch_mode<Conn>(conn: &Conn, screen: u16, zoom: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SwitchModeRequest {
        screen,
        zoom,
    };
    request0.send(conn)
}

/// Opcode for the GetMonitor request
pub const GET_MONITOR_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMonitorRequest {
    pub screen: u16,
}
impl GetMonitorRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MONITOR_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetMonitorReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MONITOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetMonitorRequest {
            screen,
        })
    }
}
impl Request for GetMonitorRequest {
    type Reply = GetMonitorReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMonitorRequest {}
pub fn get_monitor<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetMonitorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMonitorRequest {
        screen,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMonitorReply {
    pub sequence: u16,
    pub length: u32,
    pub hsync: Vec<Syncrange>,
    pub vsync: Vec<Syncrange>,
    pub vendor: Vec<u8>,
    pub alignment_pad: Vec<u8>,
    pub model: Vec<u8>,
}
impl TryParse for GetMonitorReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (vendor_length, remaining) = u8::try_parse(remaining)?;
        let (model_length, remaining) = u8::try_parse(remaining)?;
        let (num_hsync, remaining) = u8::try_parse(remaining)?;
        let (num_vsync, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (hsync, remaining) = crate::x11_utils::parse_list::<Syncrange>(remaining, num_hsync.try_to_usize()?)?;
        let (vsync, remaining) = crate::x11_utils::parse_list::<Syncrange>(remaining, num_vsync.try_to_usize()?)?;
        let (vendor, remaining) = crate::x11_utils::parse_u8_list(remaining, vendor_length.try_to_usize()?)?;
        let vendor = vendor.to_vec();
        let (alignment_pad, remaining) = crate::x11_utils::parse_u8_list(remaining, (u32::from(vendor_length).checked_add(3u32).ok_or(ParseError::InvalidExpression)? & (!3u32)).checked_sub(u32::from(vendor_length)).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let alignment_pad = alignment_pad.to_vec();
        let (model, remaining) = crate::x11_utils::parse_u8_list(remaining, model_length.try_to_usize()?)?;
        let model = model.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMonitorReply { sequence, length, hsync, vsync, vendor, alignment_pad, model };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetMonitorReply {
    /// Get the value of the `vendor_length` field.
    ///
    /// The `vendor_length` field is used as the length field of the `vendor` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn vendor_length(&self) -> u8 {
        self.vendor.len()
            .try_into().unwrap()
    }
    /// Get the value of the `model_length` field.
    ///
    /// The `model_length` field is used as the length field of the `model` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn model_length(&self) -> u8 {
        self.model.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_hsync` field.
    ///
    /// The `num_hsync` field is used as the length field of the `hsync` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_hsync(&self) -> u8 {
        self.hsync.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_vsync` field.
    ///
    /// The `num_vsync` field is used as the length field of the `vsync` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_vsync(&self) -> u8 {
        self.vsync.len()
            .try_into().unwrap()
    }
}

/// Opcode for the LockModeSwitch request
pub const LOCK_MODE_SWITCH_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockModeSwitchRequest {
    pub screen: u16,
    pub lock: u16,
}
impl LockModeSwitchRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let lock_bytes = self.lock.serialize();
        let mut request0 = vec![
            major_opcode,
            LOCK_MODE_SWITCH_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            lock_bytes[0],
            lock_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LOCK_MODE_SWITCH_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let (lock, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(LockModeSwitchRequest {
            screen,
            lock,
        })
    }
}
impl Request for LockModeSwitchRequest {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for LockModeSwitchRequest {}
pub fn lock_mode_switch<Conn>(conn: &Conn, screen: u16, lock: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = LockModeSwitchRequest {
        screen,
        lock,
    };
    request0.send(conn)
}

/// Opcode for the GetAllModeLines request
pub const GET_ALL_MODE_LINES_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAllModeLinesRequest {
    pub screen: u16,
}
impl GetAllModeLinesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_ALL_MODE_LINES_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetAllModeLinesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_ALL_MODE_LINES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetAllModeLinesRequest {
            screen,
        })
    }
}
impl Request for GetAllModeLinesRequest {
    type Reply = GetAllModeLinesReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetAllModeLinesRequest {}
pub fn get_all_mode_lines<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetAllModeLinesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetAllModeLinesRequest {
        screen,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAllModeLinesReply {
    pub sequence: u16,
    pub length: u32,
    pub modeinfo: Vec<ModeInfo>,
}
impl TryParse for GetAllModeLinesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (modecount, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (modeinfo, remaining) = crate::x11_utils::parse_list::<ModeInfo>(remaining, modecount.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetAllModeLinesReply { sequence, length, modeinfo };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetAllModeLinesReply {
    /// Get the value of the `modecount` field.
    ///
    /// The `modecount` field is used as the length field of the `modeinfo` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn modecount(&self) -> u32 {
        self.modeinfo.len()
            .try_into().unwrap()
    }
}

/// Opcode for the AddModeLine request
pub const ADD_MODE_LINE_REQUEST: u8 = 7;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddModeLineRequest<'input> {
    pub screen: u32,
    pub dotclock: Dotclock,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub after_dotclock: Dotclock,
    pub after_hdisplay: u16,
    pub after_hsyncstart: u16,
    pub after_hsyncend: u16,
    pub after_htotal: u16,
    pub after_hskew: u16,
    pub after_vdisplay: u16,
    pub after_vsyncstart: u16,
    pub after_vsyncend: u16,
    pub after_vtotal: u16,
    pub after_flags: u32,
    pub private: Cow<'input, [u8]>,
}
impl<'input> AddModeLineRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let dotclock_bytes = self.dotclock.serialize();
        let hdisplay_bytes = self.hdisplay.serialize();
        let hsyncstart_bytes = self.hsyncstart.serialize();
        let hsyncend_bytes = self.hsyncend.serialize();
        let htotal_bytes = self.htotal.serialize();
        let hskew_bytes = self.hskew.serialize();
        let vdisplay_bytes = self.vdisplay.serialize();
        let vsyncstart_bytes = self.vsyncstart.serialize();
        let vsyncend_bytes = self.vsyncend.serialize();
        let vtotal_bytes = self.vtotal.serialize();
        let flags_bytes = self.flags.serialize();
        let privsize = u32::try_from(self.private.len()).expect("`private` has too many elements");
        let privsize_bytes = privsize.serialize();
        let after_dotclock_bytes = self.after_dotclock.serialize();
        let after_hdisplay_bytes = self.after_hdisplay.serialize();
        let after_hsyncstart_bytes = self.after_hsyncstart.serialize();
        let after_hsyncend_bytes = self.after_hsyncend.serialize();
        let after_htotal_bytes = self.after_htotal.serialize();
        let after_hskew_bytes = self.after_hskew.serialize();
        let after_vdisplay_bytes = self.after_vdisplay.serialize();
        let after_vsyncstart_bytes = self.after_vsyncstart.serialize();
        let after_vsyncend_bytes = self.after_vsyncend.serialize();
        let after_vtotal_bytes = self.after_vtotal.serialize();
        let after_flags_bytes = self.after_flags.serialize();
        let mut request0 = vec![
            major_opcode,
            ADD_MODE_LINE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            dotclock_bytes[0],
            dotclock_bytes[1],
            dotclock_bytes[2],
            dotclock_bytes[3],
            hdisplay_bytes[0],
            hdisplay_bytes[1],
            hsyncstart_bytes[0],
            hsyncstart_bytes[1],
            hsyncend_bytes[0],
            hsyncend_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            vdisplay_bytes[0],
            vdisplay_bytes[1],
            vsyncstart_bytes[0],
            vsyncstart_bytes[1],
            vsyncend_bytes[0],
            vsyncend_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            privsize_bytes[0],
            privsize_bytes[1],
            privsize_bytes[2],
            privsize_bytes[3],
            after_dotclock_bytes[0],
            after_dotclock_bytes[1],
            after_dotclock_bytes[2],
            after_dotclock_bytes[3],
            after_hdisplay_bytes[0],
            after_hdisplay_bytes[1],
            after_hsyncstart_bytes[0],
            after_hsyncstart_bytes[1],
            after_hsyncend_bytes[0],
            after_hsyncend_bytes[1],
            after_htotal_bytes[0],
            after_htotal_bytes[1],
            after_hskew_bytes[0],
            after_hskew_bytes[1],
            after_vdisplay_bytes[0],
            after_vdisplay_bytes[1],
            after_vsyncstart_bytes[0],
            after_vsyncstart_bytes[1],
            after_vsyncend_bytes[0],
            after_vsyncend_bytes[1],
            after_vtotal_bytes[0],
            after_vtotal_bytes[1],
            0,
            0,
            after_flags_bytes[0],
            after_flags_bytes[1],
            after_flags_bytes[2],
            after_flags_bytes[3],
            0,
            0,
            0,
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
        let length_so_far = length_so_far + self.private.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.private, padding0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != ADD_MODE_LINE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (dotclock, remaining) = Dotclock::try_parse(remaining)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u16::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let (after_dotclock, remaining) = Dotclock::try_parse(remaining)?;
        let (after_hdisplay, remaining) = u16::try_parse(remaining)?;
        let (after_hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (after_hsyncend, remaining) = u16::try_parse(remaining)?;
        let (after_htotal, remaining) = u16::try_parse(remaining)?;
        let (after_hskew, remaining) = u16::try_parse(remaining)?;
        let (after_vdisplay, remaining) = u16::try_parse(remaining)?;
        let (after_vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (after_vsyncend, remaining) = u16::try_parse(remaining)?;
        let (after_vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (after_flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (private, remaining) = crate::x11_utils::parse_u8_list(remaining, privsize.try_to_usize()?)?;
        let _ = remaining;
        Ok(AddModeLineRequest {
            screen,
            dotclock,
            hdisplay,
            hsyncstart,
            hsyncend,
            htotal,
            hskew,
            vdisplay,
            vsyncstart,
            vsyncend,
            vtotal,
            flags,
            after_dotclock,
            after_hdisplay,
            after_hsyncstart,
            after_hsyncend,
            after_htotal,
            after_hskew,
            after_vdisplay,
            after_vsyncstart,
            after_vsyncend,
            after_vtotal,
            after_flags,
            private: Cow::Borrowed(private),
        })
    }
    /// Clone all borrowed data in this AddModeLineRequest.
    pub fn into_owned(self) -> AddModeLineRequest<'static> {
        AddModeLineRequest {
            screen: self.screen,
            dotclock: self.dotclock,
            hdisplay: self.hdisplay,
            hsyncstart: self.hsyncstart,
            hsyncend: self.hsyncend,
            htotal: self.htotal,
            hskew: self.hskew,
            vdisplay: self.vdisplay,
            vsyncstart: self.vsyncstart,
            vsyncend: self.vsyncend,
            vtotal: self.vtotal,
            flags: self.flags,
            after_dotclock: self.after_dotclock,
            after_hdisplay: self.after_hdisplay,
            after_hsyncstart: self.after_hsyncstart,
            after_hsyncend: self.after_hsyncend,
            after_htotal: self.after_htotal,
            after_hskew: self.after_hskew,
            after_vdisplay: self.after_vdisplay,
            after_vsyncstart: self.after_vsyncstart,
            after_vsyncend: self.after_vsyncend,
            after_vtotal: self.after_vtotal,
            after_flags: self.after_flags,
            private: Cow::Owned(self.private.into_owned()),
        }
    }
}
impl<'input> Request for AddModeLineRequest<'input> {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for AddModeLineRequest<'input> {}
pub fn add_mode_line<'c, 'input, Conn, A, B>(conn: &'c Conn, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, after_dotclock: Dotclock, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: B, private: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
    B: Into<u32>,
{
    let flags: u32 = flags.into();
    let after_flags: u32 = after_flags.into();
    let request0 = AddModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        after_dotclock,
        after_hdisplay,
        after_hsyncstart,
        after_hsyncend,
        after_htotal,
        after_hskew,
        after_vdisplay,
        after_vsyncstart,
        after_vsyncend,
        after_vtotal,
        after_flags,
        private: Cow::Borrowed(private),
    };
    request0.send(conn)
}

/// Opcode for the DeleteModeLine request
pub const DELETE_MODE_LINE_REQUEST: u8 = 8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteModeLineRequest<'input> {
    pub screen: u32,
    pub dotclock: Dotclock,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub private: Cow<'input, [u8]>,
}
impl<'input> DeleteModeLineRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let dotclock_bytes = self.dotclock.serialize();
        let hdisplay_bytes = self.hdisplay.serialize();
        let hsyncstart_bytes = self.hsyncstart.serialize();
        let hsyncend_bytes = self.hsyncend.serialize();
        let htotal_bytes = self.htotal.serialize();
        let hskew_bytes = self.hskew.serialize();
        let vdisplay_bytes = self.vdisplay.serialize();
        let vsyncstart_bytes = self.vsyncstart.serialize();
        let vsyncend_bytes = self.vsyncend.serialize();
        let vtotal_bytes = self.vtotal.serialize();
        let flags_bytes = self.flags.serialize();
        let privsize = u32::try_from(self.private.len()).expect("`private` has too many elements");
        let privsize_bytes = privsize.serialize();
        let mut request0 = vec![
            major_opcode,
            DELETE_MODE_LINE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            dotclock_bytes[0],
            dotclock_bytes[1],
            dotclock_bytes[2],
            dotclock_bytes[3],
            hdisplay_bytes[0],
            hdisplay_bytes[1],
            hsyncstart_bytes[0],
            hsyncstart_bytes[1],
            hsyncend_bytes[0],
            hsyncend_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            vdisplay_bytes[0],
            vdisplay_bytes[1],
            vsyncstart_bytes[0],
            vsyncstart_bytes[1],
            vsyncend_bytes[0],
            vsyncend_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            privsize_bytes[0],
            privsize_bytes[1],
            privsize_bytes[2],
            privsize_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.private.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.private, padding0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DELETE_MODE_LINE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (dotclock, remaining) = Dotclock::try_parse(remaining)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u16::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let (private, remaining) = crate::x11_utils::parse_u8_list(remaining, privsize.try_to_usize()?)?;
        let _ = remaining;
        Ok(DeleteModeLineRequest {
            screen,
            dotclock,
            hdisplay,
            hsyncstart,
            hsyncend,
            htotal,
            hskew,
            vdisplay,
            vsyncstart,
            vsyncend,
            vtotal,
            flags,
            private: Cow::Borrowed(private),
        })
    }
    /// Clone all borrowed data in this DeleteModeLineRequest.
    pub fn into_owned(self) -> DeleteModeLineRequest<'static> {
        DeleteModeLineRequest {
            screen: self.screen,
            dotclock: self.dotclock,
            hdisplay: self.hdisplay,
            hsyncstart: self.hsyncstart,
            hsyncend: self.hsyncend,
            htotal: self.htotal,
            hskew: self.hskew,
            vdisplay: self.vdisplay,
            vsyncstart: self.vsyncstart,
            vsyncend: self.vsyncend,
            vtotal: self.vtotal,
            flags: self.flags,
            private: Cow::Owned(self.private.into_owned()),
        }
    }
}
impl<'input> Request for DeleteModeLineRequest<'input> {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for DeleteModeLineRequest<'input> {}
pub fn delete_mode_line<'c, 'input, Conn, A>(conn: &'c Conn, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = DeleteModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    request0.send(conn)
}

/// Opcode for the ValidateModeLine request
pub const VALIDATE_MODE_LINE_REQUEST: u8 = 9;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateModeLineRequest<'input> {
    pub screen: u32,
    pub dotclock: Dotclock,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub private: Cow<'input, [u8]>,
}
impl<'input> ValidateModeLineRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let dotclock_bytes = self.dotclock.serialize();
        let hdisplay_bytes = self.hdisplay.serialize();
        let hsyncstart_bytes = self.hsyncstart.serialize();
        let hsyncend_bytes = self.hsyncend.serialize();
        let htotal_bytes = self.htotal.serialize();
        let hskew_bytes = self.hskew.serialize();
        let vdisplay_bytes = self.vdisplay.serialize();
        let vsyncstart_bytes = self.vsyncstart.serialize();
        let vsyncend_bytes = self.vsyncend.serialize();
        let vtotal_bytes = self.vtotal.serialize();
        let flags_bytes = self.flags.serialize();
        let privsize = u32::try_from(self.private.len()).expect("`private` has too many elements");
        let privsize_bytes = privsize.serialize();
        let mut request0 = vec![
            major_opcode,
            VALIDATE_MODE_LINE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            dotclock_bytes[0],
            dotclock_bytes[1],
            dotclock_bytes[2],
            dotclock_bytes[3],
            hdisplay_bytes[0],
            hdisplay_bytes[1],
            hsyncstart_bytes[0],
            hsyncstart_bytes[1],
            hsyncend_bytes[0],
            hsyncend_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            vdisplay_bytes[0],
            vdisplay_bytes[1],
            vsyncstart_bytes[0],
            vsyncstart_bytes[1],
            vsyncend_bytes[0],
            vsyncend_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            privsize_bytes[0],
            privsize_bytes[1],
            privsize_bytes[2],
            privsize_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.private.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.private, padding0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ValidateModeLineReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != VALIDATE_MODE_LINE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (dotclock, remaining) = Dotclock::try_parse(remaining)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u16::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let (private, remaining) = crate::x11_utils::parse_u8_list(remaining, privsize.try_to_usize()?)?;
        let _ = remaining;
        Ok(ValidateModeLineRequest {
            screen,
            dotclock,
            hdisplay,
            hsyncstart,
            hsyncend,
            htotal,
            hskew,
            vdisplay,
            vsyncstart,
            vsyncend,
            vtotal,
            flags,
            private: Cow::Borrowed(private),
        })
    }
    /// Clone all borrowed data in this ValidateModeLineRequest.
    pub fn into_owned(self) -> ValidateModeLineRequest<'static> {
        ValidateModeLineRequest {
            screen: self.screen,
            dotclock: self.dotclock,
            hdisplay: self.hdisplay,
            hsyncstart: self.hsyncstart,
            hsyncend: self.hsyncend,
            htotal: self.htotal,
            hskew: self.hskew,
            vdisplay: self.vdisplay,
            vsyncstart: self.vsyncstart,
            vsyncend: self.vsyncend,
            vtotal: self.vtotal,
            flags: self.flags,
            private: Cow::Owned(self.private.into_owned()),
        }
    }
}
impl<'input> Request for ValidateModeLineRequest<'input> {
    type Reply = ValidateModeLineReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for ValidateModeLineRequest<'input> {}
pub fn validate_mode_line<'c, 'input, Conn, A>(conn: &'c Conn, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<Cookie<'c, Conn, ValidateModeLineReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = ValidateModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidateModeLineReply {
    pub sequence: u16,
    pub length: u32,
    pub status: u32,
}
impl TryParse for ValidateModeLineReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ValidateModeLineReply { sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SwitchToMode request
pub const SWITCH_TO_MODE_REQUEST: u8 = 10;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchToModeRequest<'input> {
    pub screen: u32,
    pub dotclock: Dotclock,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub private: Cow<'input, [u8]>,
}
impl<'input> SwitchToModeRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let dotclock_bytes = self.dotclock.serialize();
        let hdisplay_bytes = self.hdisplay.serialize();
        let hsyncstart_bytes = self.hsyncstart.serialize();
        let hsyncend_bytes = self.hsyncend.serialize();
        let htotal_bytes = self.htotal.serialize();
        let hskew_bytes = self.hskew.serialize();
        let vdisplay_bytes = self.vdisplay.serialize();
        let vsyncstart_bytes = self.vsyncstart.serialize();
        let vsyncend_bytes = self.vsyncend.serialize();
        let vtotal_bytes = self.vtotal.serialize();
        let flags_bytes = self.flags.serialize();
        let privsize = u32::try_from(self.private.len()).expect("`private` has too many elements");
        let privsize_bytes = privsize.serialize();
        let mut request0 = vec![
            major_opcode,
            SWITCH_TO_MODE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            dotclock_bytes[0],
            dotclock_bytes[1],
            dotclock_bytes[2],
            dotclock_bytes[3],
            hdisplay_bytes[0],
            hdisplay_bytes[1],
            hsyncstart_bytes[0],
            hsyncstart_bytes[1],
            hsyncend_bytes[0],
            hsyncend_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            vdisplay_bytes[0],
            vdisplay_bytes[1],
            vsyncstart_bytes[0],
            vsyncstart_bytes[1],
            vsyncend_bytes[0],
            vsyncend_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            privsize_bytes[0],
            privsize_bytes[1],
            privsize_bytes[2],
            privsize_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.private.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.private, padding0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SWITCH_TO_MODE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (dotclock, remaining) = Dotclock::try_parse(remaining)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u16::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let (private, remaining) = crate::x11_utils::parse_u8_list(remaining, privsize.try_to_usize()?)?;
        let _ = remaining;
        Ok(SwitchToModeRequest {
            screen,
            dotclock,
            hdisplay,
            hsyncstart,
            hsyncend,
            htotal,
            hskew,
            vdisplay,
            vsyncstart,
            vsyncend,
            vtotal,
            flags,
            private: Cow::Borrowed(private),
        })
    }
    /// Clone all borrowed data in this SwitchToModeRequest.
    pub fn into_owned(self) -> SwitchToModeRequest<'static> {
        SwitchToModeRequest {
            screen: self.screen,
            dotclock: self.dotclock,
            hdisplay: self.hdisplay,
            hsyncstart: self.hsyncstart,
            hsyncend: self.hsyncend,
            htotal: self.htotal,
            hskew: self.hskew,
            vdisplay: self.vdisplay,
            vsyncstart: self.vsyncstart,
            vsyncend: self.vsyncend,
            vtotal: self.vtotal,
            flags: self.flags,
            private: Cow::Owned(self.private.into_owned()),
        }
    }
}
impl<'input> Request for SwitchToModeRequest<'input> {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SwitchToModeRequest<'input> {}
pub fn switch_to_mode<'c, 'input, Conn, A>(conn: &'c Conn, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = SwitchToModeRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    request0.send(conn)
}

/// Opcode for the GetViewPort request
pub const GET_VIEW_PORT_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetViewPortRequest {
    pub screen: u16,
}
impl GetViewPortRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_VIEW_PORT_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetViewPortReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_VIEW_PORT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetViewPortRequest {
            screen,
        })
    }
}
impl Request for GetViewPortRequest {
    type Reply = GetViewPortReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetViewPortRequest {}
pub fn get_view_port<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetViewPortReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetViewPortRequest {
        screen,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetViewPortReply {
    pub sequence: u16,
    pub length: u32,
    pub x: u32,
    pub y: u32,
}
impl TryParse for GetViewPortReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (x, remaining) = u32::try_parse(remaining)?;
        let (y, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetViewPortReply { sequence, length, x, y };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetViewPort request
pub const SET_VIEW_PORT_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetViewPortRequest {
    pub screen: u16,
    pub x: u32,
    pub y: u32,
}
impl SetViewPortRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_VIEW_PORT_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
            x_bytes[0],
            x_bytes[1],
            x_bytes[2],
            x_bytes[3],
            y_bytes[0],
            y_bytes[1],
            y_bytes[2],
            y_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_VIEW_PORT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (x, remaining) = u32::try_parse(remaining)?;
        let (y, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetViewPortRequest {
            screen,
            x,
            y,
        })
    }
}
impl Request for SetViewPortRequest {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SetViewPortRequest {}
pub fn set_view_port<Conn>(conn: &Conn, screen: u16, x: u32, y: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetViewPortRequest {
        screen,
        x,
        y,
    };
    request0.send(conn)
}

/// Opcode for the GetDotClocks request
pub const GET_DOT_CLOCKS_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDotClocksRequest {
    pub screen: u16,
}
impl GetDotClocksRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DOT_CLOCKS_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetDotClocksReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DOT_CLOCKS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetDotClocksRequest {
            screen,
        })
    }
}
impl Request for GetDotClocksRequest {
    type Reply = GetDotClocksReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDotClocksRequest {}
pub fn get_dot_clocks<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetDotClocksReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDotClocksRequest {
        screen,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDotClocksReply {
    pub sequence: u16,
    pub length: u32,
    pub flags: u32,
    pub clocks: u32,
    pub maxclocks: u32,
    pub clock: Vec<u32>,
}
impl TryParse for GetDotClocksReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (clocks, remaining) = u32::try_parse(remaining)?;
        let (maxclocks, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (clock, remaining) = crate::x11_utils::parse_list::<u32>(remaining, 1u32.checked_sub(flags & 1u32).ok_or(ParseError::InvalidExpression)?.checked_mul(clocks).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDotClocksReply { sequence, length, flags, clocks, maxclocks, clock };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetClientVersion request
pub const SET_CLIENT_VERSION_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetClientVersionRequest {
    pub major: u16,
    pub minor: u16,
}
impl SetClientVersionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let major_bytes = self.major.serialize();
        let minor_bytes = self.minor.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_CLIENT_VERSION_REQUEST,
            0,
            0,
            major_bytes[0],
            major_bytes[1],
            minor_bytes[0],
            minor_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_CLIENT_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (major, remaining) = u16::try_parse(value)?;
        let (minor, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetClientVersionRequest {
            major,
            minor,
        })
    }
}
impl Request for SetClientVersionRequest {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SetClientVersionRequest {}
pub fn set_client_version<Conn>(conn: &Conn, major: u16, minor: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetClientVersionRequest {
        major,
        minor,
    };
    request0.send(conn)
}

/// Opcode for the SetGamma request
pub const SET_GAMMA_REQUEST: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetGammaRequest {
    pub screen: u16,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
impl SetGammaRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let red_bytes = self.red.serialize();
        let green_bytes = self.green.serialize();
        let blue_bytes = self.blue.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_GAMMA_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
            red_bytes[0],
            red_bytes[1],
            red_bytes[2],
            red_bytes[3],
            green_bytes[0],
            green_bytes[1],
            green_bytes[2],
            green_bytes[3],
            blue_bytes[0],
            blue_bytes[1],
            blue_bytes[2],
            blue_bytes[3],
            0,
            0,
            0,
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
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_GAMMA_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (red, remaining) = u32::try_parse(remaining)?;
        let (green, remaining) = u32::try_parse(remaining)?;
        let (blue, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(SetGammaRequest {
            screen,
            red,
            green,
            blue,
        })
    }
}
impl Request for SetGammaRequest {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SetGammaRequest {}
pub fn set_gamma<Conn>(conn: &Conn, screen: u16, red: u32, green: u32, blue: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetGammaRequest {
        screen,
        red,
        green,
        blue,
    };
    request0.send(conn)
}

/// Opcode for the GetGamma request
pub const GET_GAMMA_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetGammaRequest {
    pub screen: u16,
}
impl GetGammaRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_GAMMA_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetGammaReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_GAMMA_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(26..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetGammaRequest {
            screen,
        })
    }
}
impl Request for GetGammaRequest {
    type Reply = GetGammaReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetGammaRequest {}
pub fn get_gamma<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetGammaReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetGammaRequest {
        screen,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetGammaReply {
    pub sequence: u16,
    pub length: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
impl TryParse for GetGammaReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (red, remaining) = u32::try_parse(remaining)?;
        let (green, remaining) = u32::try_parse(remaining)?;
        let (blue, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetGammaReply { sequence, length, red, green, blue };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the GetGammaRamp request
pub const GET_GAMMA_RAMP_REQUEST: u8 = 17;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetGammaRampRequest {
    pub screen: u16,
    pub size: u16,
}
impl GetGammaRampRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let size_bytes = self.size.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_GAMMA_RAMP_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            size_bytes[0],
            size_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetGammaRampReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_GAMMA_RAMP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetGammaRampRequest {
            screen,
            size,
        })
    }
}
impl Request for GetGammaRampRequest {
    type Reply = GetGammaRampReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetGammaRampRequest {}
pub fn get_gamma_ramp<Conn>(conn: &Conn, screen: u16, size: u16) -> Result<Cookie<'_, Conn, GetGammaRampReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetGammaRampRequest {
        screen,
        size,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetGammaRampReply {
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub red: Vec<u16>,
    pub green: Vec<u16>,
    pub blue: Vec<u16>,
}
impl TryParse for GetGammaRampReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (red, remaining) = crate::x11_utils::parse_list::<u16>(remaining, (u32::from(size).checked_add(1u32).ok_or(ParseError::InvalidExpression)? & (!1u32)).try_to_usize()?)?;
        let (green, remaining) = crate::x11_utils::parse_list::<u16>(remaining, (u32::from(size).checked_add(1u32).ok_or(ParseError::InvalidExpression)? & (!1u32)).try_to_usize()?)?;
        let (blue, remaining) = crate::x11_utils::parse_list::<u16>(remaining, (u32::from(size).checked_add(1u32).ok_or(ParseError::InvalidExpression)? & (!1u32)).try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetGammaRampReply { sequence, length, size, red, green, blue };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetGammaRamp request
pub const SET_GAMMA_RAMP_REQUEST: u8 = 18;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetGammaRampRequest<'input> {
    pub screen: u16,
    pub size: u16,
    pub red: Cow<'input, [u16]>,
    pub green: Cow<'input, [u16]>,
    pub blue: Cow<'input, [u16]>,
}
impl<'input> SetGammaRampRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let size_bytes = self.size.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_GAMMA_RAMP_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            size_bytes[0],
            size_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(self.red.len(), usize::try_from(u32::from(self.size).checked_add(1u32).unwrap() & (!1u32)).unwrap(), "`red` has an incorrect length");
        let red_bytes = self.red.serialize();
        let length_so_far = length_so_far + red_bytes.len();
        assert_eq!(self.green.len(), usize::try_from(u32::from(self.size).checked_add(1u32).unwrap() & (!1u32)).unwrap(), "`green` has an incorrect length");
        let green_bytes = self.green.serialize();
        let length_so_far = length_so_far + green_bytes.len();
        assert_eq!(self.blue.len(), usize::try_from(u32::from(self.size).checked_add(1u32).unwrap() & (!1u32)).unwrap(), "`blue` has an incorrect length");
        let blue_bytes = self.blue.serialize();
        let length_so_far = length_so_far + blue_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), red_bytes.into(), green_bytes.into(), blue_bytes.into(), padding0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_GAMMA_RAMP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let (red, remaining) = crate::x11_utils::parse_list::<u16>(remaining, (u32::from(size).checked_add(1u32).ok_or(ParseError::InvalidExpression)? & (!1u32)).try_to_usize()?)?;
        let (green, remaining) = crate::x11_utils::parse_list::<u16>(remaining, (u32::from(size).checked_add(1u32).ok_or(ParseError::InvalidExpression)? & (!1u32)).try_to_usize()?)?;
        let (blue, remaining) = crate::x11_utils::parse_list::<u16>(remaining, (u32::from(size).checked_add(1u32).ok_or(ParseError::InvalidExpression)? & (!1u32)).try_to_usize()?)?;
        let _ = remaining;
        Ok(SetGammaRampRequest {
            screen,
            size,
            red: Cow::Owned(red),
            green: Cow::Owned(green),
            blue: Cow::Owned(blue),
        })
    }
    /// Clone all borrowed data in this SetGammaRampRequest.
    pub fn into_owned(self) -> SetGammaRampRequest<'static> {
        SetGammaRampRequest {
            screen: self.screen,
            size: self.size,
            red: Cow::Owned(self.red.into_owned()),
            green: Cow::Owned(self.green.into_owned()),
            blue: Cow::Owned(self.blue.into_owned()),
        }
    }
}
impl<'input> Request for SetGammaRampRequest<'input> {
    type Reply = ();

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetGammaRampRequest<'input> {}
pub fn set_gamma_ramp<'c, 'input, Conn>(conn: &'c Conn, screen: u16, size: u16, red: &'input [u16], green: &'input [u16], blue: &'input [u16]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetGammaRampRequest {
        screen,
        size,
        red: Cow::Borrowed(red),
        green: Cow::Borrowed(green),
        blue: Cow::Borrowed(blue),
    };
    request0.send(conn)
}

/// Opcode for the GetGammaRampSize request
pub const GET_GAMMA_RAMP_SIZE_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetGammaRampSizeRequest {
    pub screen: u16,
}
impl GetGammaRampSizeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_GAMMA_RAMP_SIZE_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetGammaRampSizeReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_GAMMA_RAMP_SIZE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetGammaRampSizeRequest {
            screen,
        })
    }
}
impl Request for GetGammaRampSizeRequest {
    type Reply = GetGammaRampSizeReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetGammaRampSizeRequest {}
pub fn get_gamma_ramp_size<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetGammaRampSizeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetGammaRampSizeRequest {
        screen,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetGammaRampSizeReply {
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
}
impl TryParse for GetGammaRampSizeReply {
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
        let result = GetGammaRampSizeReply { sequence, length, size };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the GetPermissions request
pub const GET_PERMISSIONS_REQUEST: u8 = 20;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPermissionsRequest {
    pub screen: u16,
}
impl GetPermissionsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize_impl(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_PERMISSIONS_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetPermissionsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize_impl(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PERMISSIONS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetPermissionsRequest {
            screen,
        })
    }
}
impl Request for GetPermissionsRequest {
    type Reply = GetPermissionsReply;

    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize_impl(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPermissionsRequest {}
pub fn get_permissions<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetPermissionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPermissionsRequest {
        screen,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPermissionsReply {
    pub sequence: u16,
    pub length: u32,
    pub permissions: u32,
}
impl TryParse for GetPermissionsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (permissions, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPermissionsReply { sequence, length, permissions };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the BadClock error
pub const BAD_CLOCK_ERROR: u8 = 0;

/// Opcode for the BadHTimings error
pub const BAD_H_TIMINGS_ERROR: u8 = 1;

/// Opcode for the BadVTimings error
pub const BAD_V_TIMINGS_ERROR: u8 = 2;

/// Opcode for the ModeUnsuitable error
pub const MODE_UNSUITABLE_ERROR: u8 = 3;

/// Opcode for the ExtensionDisabled error
pub const EXTENSION_DISABLED_ERROR: u8 = 4;

/// Opcode for the ClientNotLocal error
pub const CLIENT_NOT_LOCAL_ERROR: u8 = 5;

/// Opcode for the ZoomLocked error
pub const ZOOM_LOCKED_ERROR: u8 = 6;

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xf86vidmode_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }
    fn xf86vidmode_get_mode_line(&self, screen: u16) -> Result<Cookie<'_, Self, GetModeLineReply>, ConnectionError>
    {
        get_mode_line(self, screen)
    }
    fn xf86vidmode_mod_mode_line<'c, 'input, A>(&'c self, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        mod_mode_line(self, screen, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }
    fn xf86vidmode_switch_mode(&self, screen: u16, zoom: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        switch_mode(self, screen, zoom)
    }
    fn xf86vidmode_get_monitor(&self, screen: u16) -> Result<Cookie<'_, Self, GetMonitorReply>, ConnectionError>
    {
        get_monitor(self, screen)
    }
    fn xf86vidmode_lock_mode_switch(&self, screen: u16, lock: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        lock_mode_switch(self, screen, lock)
    }
    fn xf86vidmode_get_all_mode_lines(&self, screen: u16) -> Result<Cookie<'_, Self, GetAllModeLinesReply>, ConnectionError>
    {
        get_all_mode_lines(self, screen)
    }
    fn xf86vidmode_add_mode_line<'c, 'input, A, B>(&'c self, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, after_dotclock: Dotclock, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: B, private: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
        B: Into<u32>,
    {
        add_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, after_dotclock, after_hdisplay, after_hsyncstart, after_hsyncend, after_htotal, after_hskew, after_vdisplay, after_vsyncstart, after_vsyncend, after_vtotal, after_flags, private)
    }
    fn xf86vidmode_delete_mode_line<'c, 'input, A>(&'c self, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        delete_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }
    fn xf86vidmode_validate_mode_line<'c, 'input, A>(&'c self, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<Cookie<'c, Self, ValidateModeLineReply>, ConnectionError>
    where
        A: Into<u32>,
    {
        validate_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }
    fn xf86vidmode_switch_to_mode<'c, 'input, A>(&'c self, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        switch_to_mode(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }
    fn xf86vidmode_get_view_port(&self, screen: u16) -> Result<Cookie<'_, Self, GetViewPortReply>, ConnectionError>
    {
        get_view_port(self, screen)
    }
    fn xf86vidmode_set_view_port(&self, screen: u16, x: u32, y: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_view_port(self, screen, x, y)
    }
    fn xf86vidmode_get_dot_clocks(&self, screen: u16) -> Result<Cookie<'_, Self, GetDotClocksReply>, ConnectionError>
    {
        get_dot_clocks(self, screen)
    }
    fn xf86vidmode_set_client_version(&self, major: u16, minor: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_client_version(self, major, minor)
    }
    fn xf86vidmode_set_gamma(&self, screen: u16, red: u32, green: u32, blue: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_gamma(self, screen, red, green, blue)
    }
    fn xf86vidmode_get_gamma(&self, screen: u16) -> Result<Cookie<'_, Self, GetGammaReply>, ConnectionError>
    {
        get_gamma(self, screen)
    }
    fn xf86vidmode_get_gamma_ramp(&self, screen: u16, size: u16) -> Result<Cookie<'_, Self, GetGammaRampReply>, ConnectionError>
    {
        get_gamma_ramp(self, screen, size)
    }
    fn xf86vidmode_set_gamma_ramp<'c, 'input>(&'c self, screen: u16, size: u16, red: &'input [u16], green: &'input [u16], blue: &'input [u16]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_gamma_ramp(self, screen, size, red, green, blue)
    }
    fn xf86vidmode_get_gamma_ramp_size(&self, screen: u16) -> Result<Cookie<'_, Self, GetGammaRampSizeReply>, ConnectionError>
    {
        get_gamma_ramp_size(self, screen)
    }
    fn xf86vidmode_get_permissions(&self, screen: u16) -> Result<Cookie<'_, Self, GetPermissionsReply>, ConnectionError>
    {
        get_permissions(self, screen)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
