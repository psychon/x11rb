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
use crate::x11_utils::GenericError;
#[allow(unused_imports)]
use super::xproto;
#[allow(unused_imports)]
use super::render;
#[allow(unused_imports)]
use super::randr;
#[allow(unused_imports)]
use super::shape;
#[allow(unused_imports)]
use super::xfixes;
#[allow(unused_imports)]
use super::sync;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "Present";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventEnum {
    ConfigureNotify = 0,
    CompleteNotify = 1,
    IdleNotify = 2,
    RedirectNotify = 3,
}
impl From<EventEnum> for u8 {
    fn from(input: EventEnum) -> Self {
        match input {
            EventEnum::ConfigureNotify => 0,
            EventEnum::CompleteNotify => 1,
            EventEnum::IdleNotify => 2,
            EventEnum::RedirectNotify => 3,
        }
    }
}
impl From<EventEnum> for std::option::Option<u8> {
    fn from(input: EventEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<EventEnum> for u16 {
    fn from(input: EventEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EventEnum> for std::option::Option<u16> {
    fn from(input: EventEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<EventEnum> for u32 {
    fn from(input: EventEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EventEnum> for std::option::Option<u32> {
    fn from(input: EventEnum) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for EventEnum {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventEnum::ConfigureNotify),
            1 => Ok(EventEnum::CompleteNotify),
            2 => Ok(EventEnum::IdleNotify),
            3 => Ok(EventEnum::RedirectNotify),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for EventEnum {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for EventEnum {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventMask {
    NoEvent = 0,
    ConfigureNotify = 1 << 0,
    CompleteNotify = 1 << 1,
    IdleNotify = 1 << 2,
    RedirectNotify = 1 << 3,
}
impl From<EventMask> for u8 {
    fn from(input: EventMask) -> Self {
        match input {
            EventMask::NoEvent => 0,
            EventMask::ConfigureNotify => 1 << 0,
            EventMask::CompleteNotify => 1 << 1,
            EventMask::IdleNotify => 1 << 2,
            EventMask::RedirectNotify => 1 << 3,
        }
    }
}
impl From<EventMask> for std::option::Option<u8> {
    fn from(input: EventMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<EventMask> for u16 {
    fn from(input: EventMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EventMask> for std::option::Option<u16> {
    fn from(input: EventMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<EventMask> for u32 {
    fn from(input: EventMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EventMask> for std::option::Option<u32> {
    fn from(input: EventMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for EventMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventMask::NoEvent),
            1 => Ok(EventMask::ConfigureNotify),
            2 => Ok(EventMask::CompleteNotify),
            4 => Ok(EventMask::IdleNotify),
            8 => Ok(EventMask::RedirectNotify),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for EventMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for EventMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(EventMask, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Option {
    None = 0,
    Async = 1 << 0,
    Copy = 1 << 1,
    UST = 1 << 2,
    Suboptimal = 1 << 3,
}
impl From<Option> for u8 {
    fn from(input: Option) -> Self {
        match input {
            Option::None => 0,
            Option::Async => 1 << 0,
            Option::Copy => 1 << 1,
            Option::UST => 1 << 2,
            Option::Suboptimal => 1 << 3,
        }
    }
}
impl From<Option> for std::option::Option<u8> {
    fn from(input: Option) -> Self {
        Some(u8::from(input))
    }
}
impl From<Option> for u16 {
    fn from(input: Option) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Option> for std::option::Option<u16> {
    fn from(input: Option) -> Self {
        Some(u16::from(input))
    }
}
impl From<Option> for u32 {
    fn from(input: Option) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Option> for std::option::Option<u32> {
    fn from(input: Option) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Option {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Option::None),
            1 => Ok(Option::Async),
            2 => Ok(Option::Copy),
            4 => Ok(Option::UST),
            8 => Ok(Option::Suboptimal),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Option {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Option {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(Option, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Capability {
    None = 0,
    Async = 1 << 0,
    Fence = 1 << 1,
    UST = 1 << 2,
}
impl From<Capability> for u8 {
    fn from(input: Capability) -> Self {
        match input {
            Capability::None => 0,
            Capability::Async => 1 << 0,
            Capability::Fence => 1 << 1,
            Capability::UST => 1 << 2,
        }
    }
}
impl From<Capability> for std::option::Option<u8> {
    fn from(input: Capability) -> Self {
        Some(u8::from(input))
    }
}
impl From<Capability> for u16 {
    fn from(input: Capability) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Capability> for std::option::Option<u16> {
    fn from(input: Capability) -> Self {
        Some(u16::from(input))
    }
}
impl From<Capability> for u32 {
    fn from(input: Capability) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Capability> for std::option::Option<u32> {
    fn from(input: Capability) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Capability {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Capability::None),
            1 => Ok(Capability::Async),
            2 => Ok(Capability::Fence),
            4 => Ok(Capability::UST),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Capability {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Capability {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(Capability, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompleteKind {
    Pixmap = 0,
    NotifyMSC = 1,
}
impl From<CompleteKind> for u8 {
    fn from(input: CompleteKind) -> Self {
        match input {
            CompleteKind::Pixmap => 0,
            CompleteKind::NotifyMSC => 1,
        }
    }
}
impl From<CompleteKind> for std::option::Option<u8> {
    fn from(input: CompleteKind) -> Self {
        Some(u8::from(input))
    }
}
impl From<CompleteKind> for u16 {
    fn from(input: CompleteKind) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CompleteKind> for std::option::Option<u16> {
    fn from(input: CompleteKind) -> Self {
        Some(u16::from(input))
    }
}
impl From<CompleteKind> for u32 {
    fn from(input: CompleteKind) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CompleteKind> for std::option::Option<u32> {
    fn from(input: CompleteKind) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CompleteKind {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CompleteKind::Pixmap),
            1 => Ok(CompleteKind::NotifyMSC),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for CompleteKind {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CompleteKind {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompleteMode {
    Copy = 0,
    Flip = 1,
    Skip = 2,
    SuboptimalCopy = 3,
}
impl From<CompleteMode> for u8 {
    fn from(input: CompleteMode) -> Self {
        match input {
            CompleteMode::Copy => 0,
            CompleteMode::Flip => 1,
            CompleteMode::Skip => 2,
            CompleteMode::SuboptimalCopy => 3,
        }
    }
}
impl From<CompleteMode> for std::option::Option<u8> {
    fn from(input: CompleteMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<CompleteMode> for u16 {
    fn from(input: CompleteMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CompleteMode> for std::option::Option<u16> {
    fn from(input: CompleteMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<CompleteMode> for u32 {
    fn from(input: CompleteMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CompleteMode> for std::option::Option<u32> {
    fn from(input: CompleteMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CompleteMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CompleteMode::Copy),
            1 => Ok(CompleteMode::Flip),
            2 => Ok(CompleteMode::Skip),
            3 => Ok(CompleteMode::SuboptimalCopy),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for CompleteMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CompleteMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Notify {
    pub window: xproto::Window,
    pub serial: u32,
}
impl TryParse for Notify {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (serial, remaining) = u32::try_parse(remaining)?;
        let result = Notify { window, serial };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Notify {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Notify {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let window_bytes = self.window.serialize();
        let serial_bytes = self.serial.serialize();
        [
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            serial_bytes[0],
            serial_bytes[1],
            serial_bytes[2],
            serial_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.window.serialize_into(bytes);
        self.serial.serialize_into(bytes);
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

/// Opcode for the Pixmap request
pub const PIXMAP_REQUEST: u8 = 1;
pub fn pixmap<'c, Conn>(conn: &'c Conn, window: xproto::Window, pixmap: xproto::Pixmap, serial: u32, valid: xfixes::Region, update: xfixes::Region, x_off: i16, y_off: i16, target_crtc: randr::Crtc, wait_fence: sync::Fence, idle_fence: sync::Fence, options: u32, target_msc: u64, divisor: u64, remainder: u64, notifies: &[Notify]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (72 + 8 * notifies.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let pixmap_bytes = pixmap.serialize();
    let serial_bytes = serial.serialize();
    let valid_bytes = valid.serialize();
    let update_bytes = update.serialize();
    let x_off_bytes = x_off.serialize();
    let y_off_bytes = y_off.serialize();
    let target_crtc_bytes = target_crtc.serialize();
    let wait_fence_bytes = wait_fence.serialize();
    let idle_fence_bytes = idle_fence.serialize();
    let options_bytes = options.serialize();
    let target_msc_bytes = target_msc.serialize();
    let divisor_bytes = divisor.serialize();
    let remainder_bytes = remainder.serialize();
    let notifies_bytes = notifies.serialize();
    let request0 = [
        extension_information.major_opcode,
        PIXMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        pixmap_bytes[0],
        pixmap_bytes[1],
        pixmap_bytes[2],
        pixmap_bytes[3],
        serial_bytes[0],
        serial_bytes[1],
        serial_bytes[2],
        serial_bytes[3],
        valid_bytes[0],
        valid_bytes[1],
        valid_bytes[2],
        valid_bytes[3],
        update_bytes[0],
        update_bytes[1],
        update_bytes[2],
        update_bytes[3],
        x_off_bytes[0],
        x_off_bytes[1],
        y_off_bytes[0],
        y_off_bytes[1],
        target_crtc_bytes[0],
        target_crtc_bytes[1],
        target_crtc_bytes[2],
        target_crtc_bytes[3],
        wait_fence_bytes[0],
        wait_fence_bytes[1],
        wait_fence_bytes[2],
        wait_fence_bytes[3],
        idle_fence_bytes[0],
        idle_fence_bytes[1],
        idle_fence_bytes[2],
        idle_fence_bytes[3],
        options_bytes[0],
        options_bytes[1],
        options_bytes[2],
        options_bytes[3],
        0,
        0,
        0,
        0,
        target_msc_bytes[0],
        target_msc_bytes[1],
        target_msc_bytes[2],
        target_msc_bytes[3],
        target_msc_bytes[4],
        target_msc_bytes[5],
        target_msc_bytes[6],
        target_msc_bytes[7],
        divisor_bytes[0],
        divisor_bytes[1],
        divisor_bytes[2],
        divisor_bytes[3],
        divisor_bytes[4],
        divisor_bytes[5],
        divisor_bytes[6],
        divisor_bytes[7],
        remainder_bytes[0],
        remainder_bytes[1],
        remainder_bytes[2],
        remainder_bytes[3],
        remainder_bytes[4],
        remainder_bytes[5],
        remainder_bytes[6],
        remainder_bytes[7],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&notifies_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&notifies_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the NotifyMSC request
pub const NOTIFY_MSC_REQUEST: u8 = 2;
pub fn notify_msc<Conn>(conn: &Conn, window: xproto::Window, serial: u32, target_msc: u64, divisor: u64, remainder: u64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (40) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let serial_bytes = serial.serialize();
    let target_msc_bytes = target_msc.serialize();
    let divisor_bytes = divisor.serialize();
    let remainder_bytes = remainder.serialize();
    let request0 = [
        extension_information.major_opcode,
        NOTIFY_MSC_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        serial_bytes[0],
        serial_bytes[1],
        serial_bytes[2],
        serial_bytes[3],
        0,
        0,
        0,
        0,
        target_msc_bytes[0],
        target_msc_bytes[1],
        target_msc_bytes[2],
        target_msc_bytes[3],
        target_msc_bytes[4],
        target_msc_bytes[5],
        target_msc_bytes[6],
        target_msc_bytes[7],
        divisor_bytes[0],
        divisor_bytes[1],
        divisor_bytes[2],
        divisor_bytes[3],
        divisor_bytes[4],
        divisor_bytes[5],
        divisor_bytes[6],
        divisor_bytes[7],
        remainder_bytes[0],
        remainder_bytes[1],
        remainder_bytes[2],
        remainder_bytes[3],
        remainder_bytes[4],
        remainder_bytes[5],
        remainder_bytes[6],
        remainder_bytes[7],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

pub type Event = u32;

/// Opcode for the SelectInput request
pub const SELECT_INPUT_REQUEST: u8 = 3;
pub fn select_input<Conn>(conn: &Conn, eid: Event, window: xproto::Window, event_mask: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let eid_bytes = eid.serialize();
    let window_bytes = window.serialize();
    let event_mask_bytes = event_mask.serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_INPUT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        eid_bytes[0],
        eid_bytes[1],
        eid_bytes[2],
        eid_bytes[3],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        event_mask_bytes[0],
        event_mask_bytes[1],
        event_mask_bytes[2],
        event_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the QueryCapabilities request
pub const QUERY_CAPABILITIES_REQUEST: u8 = 4;
pub fn query_capabilities<Conn>(conn: &Conn, target: u32) -> Result<Cookie<'_, Conn, QueryCapabilitiesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let target_bytes = target.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_CAPABILITIES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryCapabilitiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub capabilities: u32,
}
impl QueryCapabilitiesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (capabilities, remaining) = u32::try_parse(remaining)?;
        let result = QueryCapabilitiesReply { response_type, sequence, length, capabilities };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryCapabilitiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the Generic event
pub const GENERIC_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub evtype: u16,
    pub event: Event,
}
impl GenericEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (evtype, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (event, remaining) = Event::try_parse(remaining)?;
        let result = GenericEvent { response_type, extension, sequence, length, evtype, event };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GenericEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<crate::x11_utils::GenericEvent<B>> for GenericEvent {
    type Error = ParseError;
    fn try_from(value: crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&crate::x11_utils::GenericEvent<B>> for GenericEvent {
    type Error = ParseError;
    fn try_from(value: &crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&GenericEvent> for [u8; 32] {
    fn from(input: &GenericEvent) -> Self {
        let response_type = input.response_type.serialize();
        let extension = input.extension.serialize();
        let sequence = input.sequence.serialize();
        let length = input.length.serialize();
        let evtype = input.evtype.serialize();
        let event = input.event.serialize();
        [
            response_type[0], extension[0], sequence[0], sequence[1], length[0], length[1], length[2], length[3],
            evtype[0], evtype[1], 0, 0, event[0], event[1], event[2], event[3],
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<GenericEvent> for [u8; 32] {
    fn from(input: GenericEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ConfigureNotify event
pub const CONFIGURE_NOTIFY_EVENT: u16 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigureNotifyEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub event: Event,
    pub window: xproto::Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub off_x: i16,
    pub off_y: i16,
    pub pixmap_width: u16,
    pub pixmap_height: u16,
    pub pixmap_flags: u32,
}
impl ConfigureNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (event, remaining) = Event::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (off_x, remaining) = i16::try_parse(remaining)?;
        let (off_y, remaining) = i16::try_parse(remaining)?;
        let (pixmap_width, remaining) = u16::try_parse(remaining)?;
        let (pixmap_height, remaining) = u16::try_parse(remaining)?;
        let (pixmap_flags, remaining) = u32::try_parse(remaining)?;
        let result = ConfigureNotifyEvent { response_type, extension, sequence, length, event_type, event, window, x, y, width, height, off_x, off_y, pixmap_width, pixmap_height, pixmap_flags };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ConfigureNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<crate::x11_utils::GenericEvent<B>> for ConfigureNotifyEvent {
    type Error = ParseError;
    fn try_from(value: crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&crate::x11_utils::GenericEvent<B>> for ConfigureNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the CompleteNotify event
pub const COMPLETE_NOTIFY_EVENT: u16 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompleteNotifyEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub kind: CompleteKind,
    pub mode: CompleteMode,
    pub event: Event,
    pub window: xproto::Window,
    pub serial: u32,
    pub ust: u64,
    pub msc: u64,
}
impl CompleteNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (event, remaining) = Event::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (serial, remaining) = u32::try_parse(remaining)?;
        let (ust, remaining) = u64::try_parse(remaining)?;
        let (msc, remaining) = u64::try_parse(remaining)?;
        let kind = kind.try_into()?;
        let mode = mode.try_into()?;
        let result = CompleteNotifyEvent { response_type, extension, sequence, length, event_type, kind, mode, event, window, serial, ust, msc };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CompleteNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<crate::x11_utils::GenericEvent<B>> for CompleteNotifyEvent {
    type Error = ParseError;
    fn try_from(value: crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&crate::x11_utils::GenericEvent<B>> for CompleteNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the IdleNotify event
pub const IDLE_NOTIFY_EVENT: u16 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdleNotifyEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub event: Event,
    pub window: xproto::Window,
    pub serial: u32,
    pub pixmap: xproto::Pixmap,
    pub idle_fence: sync::Fence,
}
impl IdleNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (event, remaining) = Event::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (serial, remaining) = u32::try_parse(remaining)?;
        let (pixmap, remaining) = xproto::Pixmap::try_parse(remaining)?;
        let (idle_fence, remaining) = sync::Fence::try_parse(remaining)?;
        let result = IdleNotifyEvent { response_type, extension, sequence, length, event_type, event, window, serial, pixmap, idle_fence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IdleNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<crate::x11_utils::GenericEvent<B>> for IdleNotifyEvent {
    type Error = ParseError;
    fn try_from(value: crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&crate::x11_utils::GenericEvent<B>> for IdleNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RedirectNotify event
pub const REDIRECT_NOTIFY_EVENT: u16 = 3;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedirectNotifyEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub update_window: bool,
    pub event: Event,
    pub event_window: xproto::Window,
    pub window: xproto::Window,
    pub pixmap: xproto::Pixmap,
    pub serial: u32,
    pub valid_region: xfixes::Region,
    pub update_region: xfixes::Region,
    pub valid_rect: xproto::Rectangle,
    pub update_rect: xproto::Rectangle,
    pub x_off: i16,
    pub y_off: i16,
    pub target_crtc: randr::Crtc,
    pub wait_fence: sync::Fence,
    pub idle_fence: sync::Fence,
    pub options: u32,
    pub target_msc: u64,
    pub divisor: u64,
    pub remainder: u64,
    pub notifies: Vec<Notify>,
}
impl RedirectNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (update_window, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (event, remaining) = Event::try_parse(remaining)?;
        let (event_window, remaining) = xproto::Window::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (pixmap, remaining) = xproto::Pixmap::try_parse(remaining)?;
        let (serial, remaining) = u32::try_parse(remaining)?;
        let (valid_region, remaining) = xfixes::Region::try_parse(remaining)?;
        let (update_region, remaining) = xfixes::Region::try_parse(remaining)?;
        let (valid_rect, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let (update_rect, remaining) = xproto::Rectangle::try_parse(remaining)?;
        let (x_off, remaining) = i16::try_parse(remaining)?;
        let (y_off, remaining) = i16::try_parse(remaining)?;
        let (target_crtc, remaining) = randr::Crtc::try_parse(remaining)?;
        let (wait_fence, remaining) = sync::Fence::try_parse(remaining)?;
        let (idle_fence, remaining) = sync::Fence::try_parse(remaining)?;
        let (options, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (target_msc, remaining) = u64::try_parse(remaining)?;
        let (divisor, remaining) = u64::try_parse(remaining)?;
        let (remainder, remaining) = u64::try_parse(remaining)?;
        let mut remaining = remaining;
        // Length is 'everything left in the input'
        let mut notifies = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = Notify::try_parse(remaining)?;
            notifies.push(v);
            remaining = new_remaining;
        }
        let result = RedirectNotifyEvent { response_type, extension, sequence, length, event_type, update_window, event, event_window, window, pixmap, serial, valid_region, update_region, valid_rect, update_rect, x_off, y_off, target_crtc, wait_fence, idle_fence, options, target_msc, divisor, remainder, notifies };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RedirectNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<crate::x11_utils::GenericEvent<B>> for RedirectNotifyEvent {
    type Error = ParseError;
    fn try_from(value: crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&crate::x11_utils::GenericEvent<B>> for RedirectNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &crate::x11_utils::GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn present_query_version(&self, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }

    fn present_pixmap<'c>(&'c self, window: xproto::Window, pixmap: xproto::Pixmap, serial: u32, valid: xfixes::Region, update: xfixes::Region, x_off: i16, y_off: i16, target_crtc: randr::Crtc, wait_fence: sync::Fence, idle_fence: sync::Fence, options: u32, target_msc: u64, divisor: u64, remainder: u64, notifies: &[Notify]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        self::pixmap(self, window, pixmap, serial, valid, update, x_off, y_off, target_crtc, wait_fence, idle_fence, options, target_msc, divisor, remainder, notifies)
    }

    fn present_notify_msc(&self, window: xproto::Window, serial: u32, target_msc: u64, divisor: u64, remainder: u64) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        notify_msc(self, window, serial, target_msc, divisor, remainder)
    }

    fn present_select_input(&self, eid: Event, window: xproto::Window, event_mask: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_input(self, eid, window, event_mask)
    }

    fn present_query_capabilities(&self, target: u32) -> Result<Cookie<'_, Self, QueryCapabilitiesReply>, ConnectionError>
    {
        query_capabilities(self, target)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
