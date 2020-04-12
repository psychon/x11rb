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
#[allow(unused_imports)]
use super::xproto;
#[allow(unused_imports)]
use super::render;
#[allow(unused_imports)]
use super::shape;
#[allow(unused_imports)]
use super::xfixes;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XInputExtension";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (2, 3);

pub type EventClass = u32;

pub type KeyCode = u8;

pub type DeviceId = u16;

pub type Fp1616 = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fp3232 {
    pub integral: i32,
    pub frac: u32,
}
impl TryParse for Fp3232 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (integral, remaining) = i32::try_parse(remaining)?;
        let (frac, remaining) = u32::try_parse(remaining)?;
        let result = Fp3232 { integral, frac };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Fp3232 {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Fp3232 {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let integral_bytes = self.integral.serialize();
        let frac_bytes = self.frac.serialize();
        [
            integral_bytes[0],
            integral_bytes[1],
            integral_bytes[2],
            integral_bytes[3],
            frac_bytes[0],
            frac_bytes[1],
            frac_bytes[2],
            frac_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.integral.serialize_into(bytes);
        self.frac.serialize_into(bytes);
    }
}

/// Opcode for the GetExtensionVersion request
pub const GET_EXTENSION_VERSION_REQUEST: u8 = 1;
pub fn get_extension_version<'c, Conn>(conn: &'c Conn, name: &[u8]) -> Result<Cookie<'c, Conn, GetExtensionVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * name.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let name_len: u16 = name.len().try_into()?;
    let name_len_bytes = name_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_EXTENSION_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        name_len_bytes[0],
        name_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetExtensionVersionReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
    pub present: bool,
}
impl GetExtensionVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        let (present, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(19..).ok_or(ParseError::ParseError)?;
        let result = GetExtensionVersionReply { response_type, xi_reply_type, sequence, length, server_major, server_minor, present };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetExtensionVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceUse {
    IsXPointer = 0,
    IsXKeyboard = 1,
    IsXExtensionDevice = 2,
    IsXExtensionKeyboard = 3,
    IsXExtensionPointer = 4,
}
impl From<DeviceUse> for u8 {
    fn from(input: DeviceUse) -> Self {
        match input {
            DeviceUse::IsXPointer => 0,
            DeviceUse::IsXKeyboard => 1,
            DeviceUse::IsXExtensionDevice => 2,
            DeviceUse::IsXExtensionKeyboard => 3,
            DeviceUse::IsXExtensionPointer => 4,
        }
    }
}
impl From<DeviceUse> for Option<u8> {
    fn from(input: DeviceUse) -> Self {
        Some(u8::from(input))
    }
}
impl From<DeviceUse> for u16 {
    fn from(input: DeviceUse) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceUse> for Option<u16> {
    fn from(input: DeviceUse) -> Self {
        Some(u16::from(input))
    }
}
impl From<DeviceUse> for u32 {
    fn from(input: DeviceUse) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceUse> for Option<u32> {
    fn from(input: DeviceUse) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DeviceUse {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeviceUse::IsXPointer),
            1 => Ok(DeviceUse::IsXKeyboard),
            2 => Ok(DeviceUse::IsXExtensionDevice),
            3 => Ok(DeviceUse::IsXExtensionKeyboard),
            4 => Ok(DeviceUse::IsXExtensionPointer),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for DeviceUse {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DeviceUse {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InputClass {
    Key = 0,
    Button = 1,
    Valuator = 2,
    Feedback = 3,
    Proximity = 4,
    Focus = 5,
    Other = 6,
}
impl From<InputClass> for u8 {
    fn from(input: InputClass) -> Self {
        match input {
            InputClass::Key => 0,
            InputClass::Button => 1,
            InputClass::Valuator => 2,
            InputClass::Feedback => 3,
            InputClass::Proximity => 4,
            InputClass::Focus => 5,
            InputClass::Other => 6,
        }
    }
}
impl From<InputClass> for Option<u8> {
    fn from(input: InputClass) -> Self {
        Some(u8::from(input))
    }
}
impl From<InputClass> for u16 {
    fn from(input: InputClass) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<InputClass> for Option<u16> {
    fn from(input: InputClass) -> Self {
        Some(u16::from(input))
    }
}
impl From<InputClass> for u32 {
    fn from(input: InputClass) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<InputClass> for Option<u32> {
    fn from(input: InputClass) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for InputClass {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(InputClass::Key),
            1 => Ok(InputClass::Button),
            2 => Ok(InputClass::Valuator),
            3 => Ok(InputClass::Feedback),
            4 => Ok(InputClass::Proximity),
            5 => Ok(InputClass::Focus),
            6 => Ok(InputClass::Other),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for InputClass {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for InputClass {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ValuatorMode {
    Relative = 0,
    Absolute = 1,
}
impl From<ValuatorMode> for u8 {
    fn from(input: ValuatorMode) -> Self {
        match input {
            ValuatorMode::Relative => 0,
            ValuatorMode::Absolute => 1,
        }
    }
}
impl From<ValuatorMode> for Option<u8> {
    fn from(input: ValuatorMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<ValuatorMode> for u16 {
    fn from(input: ValuatorMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ValuatorMode> for Option<u16> {
    fn from(input: ValuatorMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<ValuatorMode> for u32 {
    fn from(input: ValuatorMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ValuatorMode> for Option<u32> {
    fn from(input: ValuatorMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ValuatorMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ValuatorMode::Relative),
            1 => Ok(ValuatorMode::Absolute),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ValuatorMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ValuatorMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceInfo {
    pub device_type: xproto::Atom,
    pub device_id: u8,
    pub num_class_info: u8,
    pub device_use: DeviceUse,
}
impl TryParse for DeviceInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (device_type, remaining) = xproto::Atom::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (num_class_info, remaining) = u8::try_parse(remaining)?;
        let (device_use, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let device_use = device_use.try_into()?;
        let result = DeviceInfo { device_type, device_id, num_class_info, device_use };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceInfo {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let device_type_bytes = self.device_type.serialize();
        let device_id_bytes = self.device_id.serialize();
        let num_class_info_bytes = self.num_class_info.serialize();
        let device_use_bytes = Into::<u8>::into(self.device_use).serialize();
        [
            device_type_bytes[0],
            device_type_bytes[1],
            device_type_bytes[2],
            device_type_bytes[3],
            device_id_bytes[0],
            num_class_info_bytes[0],
            device_use_bytes[0],
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.device_type.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        self.num_class_info.serialize_into(bytes);
        Into::<u8>::into(self.device_use).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyInfo {
    pub class_id: InputClass,
    pub len: u8,
    pub min_keycode: KeyCode,
    pub max_keycode: KeyCode,
    pub num_keys: u16,
}
impl TryParse for KeyInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (min_keycode, remaining) = KeyCode::try_parse(remaining)?;
        let (max_keycode, remaining) = KeyCode::try_parse(remaining)?;
        let (num_keys, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let class_id = class_id.try_into()?;
        let result = KeyInfo { class_id, len, min_keycode, max_keycode, num_keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeyInfo {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let len_bytes = self.len.serialize();
        let min_keycode_bytes = self.min_keycode.serialize();
        let max_keycode_bytes = self.max_keycode.serialize();
        let num_keys_bytes = self.num_keys.serialize();
        [
            class_id_bytes[0],
            len_bytes[0],
            min_keycode_bytes[0],
            max_keycode_bytes[0],
            num_keys_bytes[0],
            num_keys_bytes[1],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.min_keycode.serialize_into(bytes);
        self.max_keycode.serialize_into(bytes);
        self.num_keys.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonInfo {
    pub class_id: InputClass,
    pub len: u8,
    pub num_buttons: u16,
}
impl TryParse for ButtonInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (num_buttons, remaining) = u16::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = ButtonInfo { class_id, len, num_buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ButtonInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ButtonInfo {
    type Bytes = [u8; 4];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let len_bytes = self.len.serialize();
        let num_buttons_bytes = self.num_buttons.serialize();
        [
            class_id_bytes[0],
            len_bytes[0],
            num_buttons_bytes[0],
            num_buttons_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.num_buttons.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AxisInfo {
    pub resolution: u32,
    pub minimum: i32,
    pub maximum: i32,
}
impl TryParse for AxisInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (resolution, remaining) = u32::try_parse(remaining)?;
        let (minimum, remaining) = i32::try_parse(remaining)?;
        let (maximum, remaining) = i32::try_parse(remaining)?;
        let result = AxisInfo { resolution, minimum, maximum };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AxisInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for AxisInfo {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let resolution_bytes = self.resolution.serialize();
        let minimum_bytes = self.minimum.serialize();
        let maximum_bytes = self.maximum.serialize();
        [
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            minimum_bytes[0],
            minimum_bytes[1],
            minimum_bytes[2],
            minimum_bytes[3],
            maximum_bytes[0],
            maximum_bytes[1],
            maximum_bytes[2],
            maximum_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.resolution.serialize_into(bytes);
        self.minimum.serialize_into(bytes);
        self.maximum.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValuatorInfo {
    pub class_id: InputClass,
    pub len: u8,
    pub mode: ValuatorMode,
    pub motion_size: u32,
    pub axes: Vec<AxisInfo>,
}
impl TryParse for ValuatorInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (axes_len, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (motion_size, remaining) = u32::try_parse(remaining)?;
        let (axes, remaining) = crate::x11_utils::parse_list::<AxisInfo>(remaining, axes_len as usize)?;
        let class_id = class_id.try_into()?;
        let mode = mode.try_into()?;
        let result = ValuatorInfo { class_id, len, mode, motion_size, axes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ValuatorInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ValuatorInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        let axes_len = self.axes.len() as u8;
        axes_len.serialize_into(bytes);
        Into::<u8>::into(self.mode).serialize_into(bytes);
        self.motion_size.serialize_into(bytes);
        self.axes.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputInfoInfoKey {
    pub min_keycode: KeyCode,
    pub max_keycode: KeyCode,
    pub num_keys: u16,
}
impl TryParse for InputInfoInfoKey {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (min_keycode, remaining) = KeyCode::try_parse(remaining)?;
        let (max_keycode, remaining) = KeyCode::try_parse(remaining)?;
        let (num_keys, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let result = InputInfoInfoKey { min_keycode, max_keycode, num_keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputInfoInfoKey {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for InputInfoInfoKey {
    type Bytes = [u8; 6];
    fn serialize(&self) -> Self::Bytes {
        let min_keycode_bytes = self.min_keycode.serialize();
        let max_keycode_bytes = self.max_keycode.serialize();
        let num_keys_bytes = self.num_keys.serialize();
        [
            min_keycode_bytes[0],
            max_keycode_bytes[0],
            num_keys_bytes[0],
            num_keys_bytes[1],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(6);
        self.min_keycode.serialize_into(bytes);
        self.max_keycode.serialize_into(bytes);
        self.num_keys.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputInfoInfoValuator {
    pub mode: ValuatorMode,
    pub motion_size: u32,
    pub axes: Vec<AxisInfo>,
}
impl TryParse for InputInfoInfoValuator {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (axes_len, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (motion_size, remaining) = u32::try_parse(remaining)?;
        let (axes, remaining) = crate::x11_utils::parse_list::<AxisInfo>(remaining, axes_len as usize)?;
        let mode = mode.try_into()?;
        let result = InputInfoInfoValuator { mode, motion_size, axes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputInfoInfoValuator {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for InputInfoInfoValuator {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(6);
        let axes_len = self.axes.len() as u8;
        axes_len.serialize_into(bytes);
        Into::<u8>::into(self.mode).serialize_into(bytes);
        self.motion_size.serialize_into(bytes);
        self.axes.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputInfoInfo {
    Key(InputInfoInfoKey),
    NumButtons(u16),
    Valuator(InputInfoInfoValuator),
}
impl InputInfoInfo {
    fn try_parse(value: &[u8], class_id: u8) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if class_id == Into::<u8>::into(InputClass::Key) {
            let (key, new_remaining) = InputInfoInfoKey::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputInfoInfo::Key(key));
        }
        if class_id == Into::<u8>::into(InputClass::Button) {
            let remaining = outer_remaining;
            let (num_buttons, remaining) = u16::try_parse(remaining)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputInfoInfo::NumButtons(num_buttons));
        }
        if class_id == Into::<u8>::into(InputClass::Valuator) {
            let (valuator, new_remaining) = InputInfoInfoValuator::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputInfoInfo::Valuator(valuator));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_key(&self) -> Option<&InputInfoInfoKey> {
        match self {
            InputInfoInfo::Key(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_num_buttons(&self) -> Option<&u16> {
        match self {
            InputInfoInfo::NumButtons(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_valuator(&self) -> Option<&InputInfoInfoValuator> {
        match self {
            InputInfoInfo::Valuator(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for InputInfoInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            InputInfoInfo::Key(value) => value.serialize().to_vec(),
            InputInfoInfo::NumButtons(value) => value.serialize().to_vec(),
            InputInfoInfo::Valuator(value) => value.serialize(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            InputInfoInfo::Key(value) => value.serialize_into(bytes),
            InputInfoInfo::NumButtons(value) => value.serialize_into(bytes),
            InputInfoInfo::Valuator(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputInfo {
    pub class_id: InputClass,
    pub len: u8,
    pub info: InputInfoInfo,
}
impl TryParse for InputInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (info, remaining) = InputInfoInfo::try_parse(remaining, class_id)?;
        let class_id = class_id.try_into()?;
        let result = InputInfo { class_id, len, info };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for InputInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.info.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceName {
    pub string: Vec<u8>,
}
impl TryParse for DeviceName {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (len, remaining) = u8::try_parse(remaining)?;
        let (string, remaining) = crate::x11_utils::parse_list::<u8>(remaining, len as usize)?;
        let result = DeviceName { string };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceName {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceName {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(1);
        let len = self.string.len() as u8;
        len.serialize_into(bytes);
        self.string.serialize_into(bytes);
    }
}

/// Opcode for the ListInputDevices request
pub const LIST_INPUT_DEVICES_REQUEST: u8 = 2;
pub fn list_input_devices<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListInputDevicesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_INPUT_DEVICES_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListInputDevicesReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub devices_len: u8,
    pub devices: Vec<DeviceInfo>,
    pub infos: Vec<InputInfo>,
    pub names: Vec<xproto::Str>,
}
impl ListInputDevicesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (devices_len, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let (devices, remaining) = crate::x11_utils::parse_list::<DeviceInfo>(remaining, devices_len as usize)?;
        let (infos, remaining) = crate::x11_utils::parse_list::<InputInfo>(remaining, devices.iter().map(|x| TryInto::<usize>::try_into(x.num_class_info).unwrap()).sum())?;
        let (names, remaining) = crate::x11_utils::parse_list::<xproto::Str>(remaining, devices_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = ListInputDevicesReply { response_type, xi_reply_type, sequence, length, devices_len, devices, infos, names };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListInputDevicesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

pub type EventTypeBase = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputClassInfo {
    pub class_id: InputClass,
    pub event_type_base: EventTypeBase,
}
impl TryParse for InputClassInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (event_type_base, remaining) = EventTypeBase::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = InputClassInfo { class_id, event_type_base };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputClassInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for InputClassInfo {
    type Bytes = [u8; 2];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let event_type_base_bytes = self.event_type_base.serialize();
        [
            class_id_bytes[0],
            event_type_base_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.event_type_base.serialize_into(bytes);
    }
}

/// Opcode for the OpenDevice request
pub const OPEN_DEVICE_REQUEST: u8 = 3;
pub fn open_device<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, OpenDeviceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        OPEN_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenDeviceReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub class_info: Vec<InputClassInfo>,
}
impl OpenDeviceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_classes, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let (class_info, remaining) = crate::x11_utils::parse_list::<InputClassInfo>(remaining, num_classes as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = OpenDeviceReply { response_type, xi_reply_type, sequence, length, class_info };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for OpenDeviceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CloseDevice request
pub const CLOSE_DEVICE_REQUEST: u8 = 4;
pub fn close_device<Conn>(conn: &Conn, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        CLOSE_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetDeviceMode request
pub const SET_DEVICE_MODE_REQUEST: u8 = 5;
pub fn set_device_mode<Conn, A>(conn: &Conn, device_id: u8, mode: A) -> Result<Cookie<'_, Conn, SetDeviceModeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEVICE_MODE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        mode_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetDeviceModeReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl SetDeviceModeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = SetDeviceModeReply { response_type, xi_reply_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetDeviceModeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SelectExtensionEvent request
pub const SELECT_EXTENSION_EVENT_REQUEST: u8 = 6;
pub fn select_extension_event<'c, Conn>(conn: &'c Conn, window: xproto::Window, classes: &[EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 4 * classes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let num_classes: u16 = classes.len().try_into()?;
    let num_classes_bytes = num_classes.serialize();
    let classes_bytes = classes.serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_EXTENSION_EVENT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        num_classes_bytes[0],
        num_classes_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&classes_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&classes_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetSelectedExtensionEvents request
pub const GET_SELECTED_EXTENSION_EVENTS_REQUEST: u8 = 7;
pub fn get_selected_extension_events<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetSelectedExtensionEventsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SELECTED_EXTENSION_EVENTS_REQUEST,
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
pub struct GetSelectedExtensionEventsReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub this_classes: Vec<EventClass>,
    pub all_classes: Vec<EventClass>,
}
impl GetSelectedExtensionEventsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_this_classes, remaining) = u16::try_parse(remaining)?;
        let (num_all_classes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (this_classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_this_classes as usize)?;
        let (all_classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_all_classes as usize)?;
        let result = GetSelectedExtensionEventsReply { response_type, xi_reply_type, sequence, length, this_classes, all_classes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectedExtensionEventsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PropagateMode {
    AddToList = 0,
    DeleteFromList = 1,
}
impl From<PropagateMode> for u8 {
    fn from(input: PropagateMode) -> Self {
        match input {
            PropagateMode::AddToList => 0,
            PropagateMode::DeleteFromList => 1,
        }
    }
}
impl From<PropagateMode> for Option<u8> {
    fn from(input: PropagateMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<PropagateMode> for u16 {
    fn from(input: PropagateMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PropagateMode> for Option<u16> {
    fn from(input: PropagateMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<PropagateMode> for u32 {
    fn from(input: PropagateMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PropagateMode> for Option<u32> {
    fn from(input: PropagateMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PropagateMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PropagateMode::AddToList),
            1 => Ok(PropagateMode::DeleteFromList),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for PropagateMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PropagateMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ChangeDeviceDontPropagateList request
pub const CHANGE_DEVICE_DONT_PROPAGATE_LIST_REQUEST: u8 = 8;
pub fn change_device_dont_propagate_list<'c, Conn, A>(conn: &'c Conn, window: xproto::Window, mode: A, classes: &[EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 4 * classes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let num_classes: u16 = classes.len().try_into()?;
    let num_classes_bytes = num_classes.serialize();
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let classes_bytes = classes.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_DEVICE_DONT_PROPAGATE_LIST_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        num_classes_bytes[0],
        num_classes_bytes[1],
        mode_bytes[0],
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&classes_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&classes_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetDeviceDontPropagateList request
pub const GET_DEVICE_DONT_PROPAGATE_LIST_REQUEST: u8 = 9;
pub fn get_device_dont_propagate_list<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetDeviceDontPropagateListReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_DONT_PROPAGATE_LIST_REQUEST,
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
pub struct GetDeviceDontPropagateListReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub classes: Vec<EventClass>,
}
impl GetDeviceDontPropagateListReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_classes as usize)?;
        let result = GetDeviceDontPropagateListReply { response_type, xi_reply_type, sequence, length, classes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceDontPropagateListReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceTimeCoord {
    pub time: xproto::Timestamp,
    pub axisvalues: Vec<i32>,
}
impl DeviceTimeCoord {
    pub fn try_parse(remaining: &[u8], num_axes: u8) -> Result<(Self, &[u8]), ParseError> {
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<i32>(remaining, num_axes as usize)?;
        let result = DeviceTimeCoord { time, axisvalues };
        Ok((result, remaining))
    }
}
// Skipping TryFrom implementations because of unresolved members
impl Serialize for DeviceTimeCoord {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(5);
        self.time.serialize_into(bytes);
        let num_axes = self.axisvalues.len() as u8;
        num_axes.serialize_into(bytes);
        self.axisvalues.serialize_into(bytes);
    }
}

/// Opcode for the GetDeviceMotionEvents request
pub const GET_DEVICE_MOTION_EVENTS_REQUEST: u8 = 10;
pub fn get_device_motion_events<Conn>(conn: &Conn, start: xproto::Timestamp, stop: xproto::Timestamp, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceMotionEventsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let start_bytes = start.serialize();
    let stop_bytes = stop.serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_MOTION_EVENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        start_bytes[0],
        start_bytes[1],
        start_bytes[2],
        start_bytes[3],
        stop_bytes[0],
        stop_bytes[1],
        stop_bytes[2],
        stop_bytes[3],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceMotionEventsReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_axes: u8,
    pub device_mode: ValuatorMode,
    pub events: Vec<DeviceTimeCoord>,
}
impl GetDeviceMotionEventsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_events, remaining) = u32::try_parse(remaining)?;
        let (num_axes, remaining) = u8::try_parse(remaining)?;
        let (device_mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let mut remaining = remaining;
        let list_length = num_events as usize;
        let mut events = Vec::with_capacity(list_length);
        for _ in 0..list_length {
            let (v, new_remaining) = DeviceTimeCoord::try_parse(remaining, num_axes)?;
            events.push(v);
            remaining = new_remaining;
        }
        let device_mode = device_mode.try_into()?;
        let result = GetDeviceMotionEventsReply { response_type, xi_reply_type, sequence, length, num_axes, device_mode, events };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceMotionEventsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ChangeKeyboardDevice request
pub const CHANGE_KEYBOARD_DEVICE_REQUEST: u8 = 11;
pub fn change_keyboard_device<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, ChangeKeyboardDeviceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_KEYBOARD_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeKeyboardDeviceReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl ChangeKeyboardDeviceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = ChangeKeyboardDeviceReply { response_type, xi_reply_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ChangeKeyboardDeviceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ChangePointerDevice request
pub const CHANGE_POINTER_DEVICE_REQUEST: u8 = 12;
pub fn change_pointer_device<Conn>(conn: &Conn, x_axis: u8, y_axis: u8, device_id: u8) -> Result<Cookie<'_, Conn, ChangePointerDeviceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let x_axis_bytes = x_axis.serialize();
    let y_axis_bytes = y_axis.serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_POINTER_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        x_axis_bytes[0],
        y_axis_bytes[0],
        device_id_bytes[0],
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangePointerDeviceReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl ChangePointerDeviceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = ChangePointerDeviceReply { response_type, xi_reply_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ChangePointerDeviceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GrabDevice request
pub const GRAB_DEVICE_REQUEST: u8 = 13;
pub fn grab_device<'c, Conn, A, B>(conn: &'c Conn, grab_window: xproto::Window, time: xproto::Timestamp, this_device_mode: A, other_device_mode: B, owner_events: bool, device_id: u8, classes: &[EventClass]) -> Result<Cookie<'c, Conn, GrabDeviceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20 + 4 * classes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let time_bytes = time.serialize();
    let num_classes: u16 = classes.len().try_into()?;
    let num_classes_bytes = num_classes.serialize();
    let this_device_mode = this_device_mode.into();
    let this_device_mode_bytes = this_device_mode.serialize();
    let other_device_mode = other_device_mode.into();
    let other_device_mode_bytes = other_device_mode.serialize();
    let owner_events_bytes = (owner_events as u8).serialize();
    let device_id_bytes = device_id.serialize();
    let classes_bytes = classes.serialize();
    let request0 = [
        extension_information.major_opcode,
        GRAB_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        num_classes_bytes[0],
        num_classes_bytes[1],
        this_device_mode_bytes[0],
        other_device_mode_bytes[0],
        owner_events_bytes[0],
        device_id_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&classes_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&classes_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabDeviceReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl GrabDeviceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = GrabDeviceReply { response_type, xi_reply_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GrabDeviceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the UngrabDevice request
pub const UNGRAB_DEVICE_REQUEST: u8 = 14;
pub fn ungrab_device<Conn>(conn: &Conn, time: xproto::Timestamp, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let time_bytes = time.serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        UNGRAB_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ModifierDevice {
    UseXKeyboard = 255,
}
impl From<ModifierDevice> for u8 {
    fn from(input: ModifierDevice) -> Self {
        match input {
            ModifierDevice::UseXKeyboard => 255,
        }
    }
}
impl From<ModifierDevice> for Option<u8> {
    fn from(input: ModifierDevice) -> Self {
        Some(u8::from(input))
    }
}
impl From<ModifierDevice> for u16 {
    fn from(input: ModifierDevice) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ModifierDevice> for Option<u16> {
    fn from(input: ModifierDevice) -> Self {
        Some(u16::from(input))
    }
}
impl From<ModifierDevice> for u32 {
    fn from(input: ModifierDevice) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ModifierDevice> for Option<u32> {
    fn from(input: ModifierDevice) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ModifierDevice {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            255 => Ok(ModifierDevice::UseXKeyboard),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ModifierDevice {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ModifierDevice {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the GrabDeviceKey request
pub const GRAB_DEVICE_KEY_REQUEST: u8 = 15;
pub fn grab_device_key<'c, Conn, A, B>(conn: &'c Conn, grab_window: xproto::Window, modifiers: u16, modifier_device: u8, grabbed_device: u8, key: u8, this_device_mode: A, other_device_mode: B, owner_events: bool, classes: &[EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20 + 4 * classes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let num_classes: u16 = classes.len().try_into()?;
    let num_classes_bytes = num_classes.serialize();
    let modifiers_bytes = modifiers.serialize();
    let modifier_device_bytes = modifier_device.serialize();
    let grabbed_device_bytes = grabbed_device.serialize();
    let key_bytes = key.serialize();
    let this_device_mode = this_device_mode.into();
    let this_device_mode_bytes = this_device_mode.serialize();
    let other_device_mode = other_device_mode.into();
    let other_device_mode_bytes = other_device_mode.serialize();
    let owner_events_bytes = (owner_events as u8).serialize();
    let classes_bytes = classes.serialize();
    let request0 = [
        extension_information.major_opcode,
        GRAB_DEVICE_KEY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        num_classes_bytes[0],
        num_classes_bytes[1],
        modifiers_bytes[0],
        modifiers_bytes[1],
        modifier_device_bytes[0],
        grabbed_device_bytes[0],
        key_bytes[0],
        this_device_mode_bytes[0],
        other_device_mode_bytes[0],
        owner_events_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&classes_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&classes_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the UngrabDeviceKey request
pub const UNGRAB_DEVICE_KEY_REQUEST: u8 = 16;
pub fn ungrab_device_key<Conn>(conn: &Conn, grab_window: xproto::Window, modifiers: u16, modifier_device: u8, key: u8, grabbed_device: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let modifiers_bytes = modifiers.serialize();
    let modifier_device_bytes = modifier_device.serialize();
    let key_bytes = key.serialize();
    let grabbed_device_bytes = grabbed_device.serialize();
    let request0 = [
        extension_information.major_opcode,
        UNGRAB_DEVICE_KEY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        modifiers_bytes[0],
        modifiers_bytes[1],
        modifier_device_bytes[0],
        key_bytes[0],
        grabbed_device_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GrabDeviceButton request
pub const GRAB_DEVICE_BUTTON_REQUEST: u8 = 17;
pub fn grab_device_button<'c, Conn, A, B>(conn: &'c Conn, grab_window: xproto::Window, grabbed_device: u8, modifier_device: u8, modifiers: u16, this_device_mode: A, other_device_mode: B, button: u8, owner_events: bool, classes: &[EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20 + 4 * classes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let grabbed_device_bytes = grabbed_device.serialize();
    let modifier_device_bytes = modifier_device.serialize();
    let num_classes: u16 = classes.len().try_into()?;
    let num_classes_bytes = num_classes.serialize();
    let modifiers_bytes = modifiers.serialize();
    let this_device_mode = this_device_mode.into();
    let this_device_mode_bytes = this_device_mode.serialize();
    let other_device_mode = other_device_mode.into();
    let other_device_mode_bytes = other_device_mode.serialize();
    let button_bytes = button.serialize();
    let owner_events_bytes = (owner_events as u8).serialize();
    let classes_bytes = classes.serialize();
    let request0 = [
        extension_information.major_opcode,
        GRAB_DEVICE_BUTTON_REQUEST,
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        grabbed_device_bytes[0],
        modifier_device_bytes[0],
        num_classes_bytes[0],
        num_classes_bytes[1],
        modifiers_bytes[0],
        modifiers_bytes[1],
        this_device_mode_bytes[0],
        other_device_mode_bytes[0],
        button_bytes[0],
        owner_events_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&classes_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&classes_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the UngrabDeviceButton request
pub const UNGRAB_DEVICE_BUTTON_REQUEST: u8 = 18;
pub fn ungrab_device_button<Conn>(conn: &Conn, grab_window: xproto::Window, modifiers: u16, modifier_device: u8, button: u8, grabbed_device: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let modifiers_bytes = modifiers.serialize();
    let modifier_device_bytes = modifier_device.serialize();
    let button_bytes = button.serialize();
    let grabbed_device_bytes = grabbed_device.serialize();
    let request0 = [
        extension_information.major_opcode,
        UNGRAB_DEVICE_BUTTON_REQUEST,
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        modifiers_bytes[0],
        modifiers_bytes[1],
        modifier_device_bytes[0],
        button_bytes[0],
        grabbed_device_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceInputMode {
    AsyncThisDevice = 0,
    SyncThisDevice = 1,
    ReplayThisDevice = 2,
    AsyncOtherDevices = 3,
    AsyncAll = 4,
    SyncAll = 5,
}
impl From<DeviceInputMode> for u8 {
    fn from(input: DeviceInputMode) -> Self {
        match input {
            DeviceInputMode::AsyncThisDevice => 0,
            DeviceInputMode::SyncThisDevice => 1,
            DeviceInputMode::ReplayThisDevice => 2,
            DeviceInputMode::AsyncOtherDevices => 3,
            DeviceInputMode::AsyncAll => 4,
            DeviceInputMode::SyncAll => 5,
        }
    }
}
impl From<DeviceInputMode> for Option<u8> {
    fn from(input: DeviceInputMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<DeviceInputMode> for u16 {
    fn from(input: DeviceInputMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceInputMode> for Option<u16> {
    fn from(input: DeviceInputMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<DeviceInputMode> for u32 {
    fn from(input: DeviceInputMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceInputMode> for Option<u32> {
    fn from(input: DeviceInputMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DeviceInputMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeviceInputMode::AsyncThisDevice),
            1 => Ok(DeviceInputMode::SyncThisDevice),
            2 => Ok(DeviceInputMode::ReplayThisDevice),
            3 => Ok(DeviceInputMode::AsyncOtherDevices),
            4 => Ok(DeviceInputMode::AsyncAll),
            5 => Ok(DeviceInputMode::SyncAll),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for DeviceInputMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DeviceInputMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the AllowDeviceEvents request
pub const ALLOW_DEVICE_EVENTS_REQUEST: u8 = 19;
pub fn allow_device_events<Conn, A>(conn: &Conn, time: xproto::Timestamp, mode: A, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let time_bytes = time.serialize();
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        ALLOW_DEVICE_EVENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        mode_bytes[0],
        device_id_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetDeviceFocus request
pub const GET_DEVICE_FOCUS_REQUEST: u8 = 20;
pub fn get_device_focus<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceFocusReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_FOCUS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDeviceFocusReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xproto::Window,
    pub time: xproto::Timestamp,
    pub revert_to: xproto::InputFocus,
}
impl GetDeviceFocusReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (focus, remaining) = xproto::Window::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (revert_to, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::ParseError)?;
        let revert_to = revert_to.try_into()?;
        let result = GetDeviceFocusReply { response_type, xi_reply_type, sequence, length, focus, time, revert_to };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceFocusReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetDeviceFocus request
pub const SET_DEVICE_FOCUS_REQUEST: u8 = 21;
pub fn set_device_focus<Conn, A>(conn: &Conn, focus: xproto::Window, time: xproto::Timestamp, revert_to: A, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let focus_bytes = focus.serialize();
    let time_bytes = time.serialize();
    let revert_to = revert_to.into();
    let revert_to_bytes = revert_to.serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEVICE_FOCUS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        focus_bytes[0],
        focus_bytes[1],
        focus_bytes[2],
        focus_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        revert_to_bytes[0],
        device_id_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FeedbackClass {
    Keyboard = 0,
    Pointer = 1,
    String = 2,
    Integer = 3,
    Led = 4,
    Bell = 5,
}
impl From<FeedbackClass> for u8 {
    fn from(input: FeedbackClass) -> Self {
        match input {
            FeedbackClass::Keyboard => 0,
            FeedbackClass::Pointer => 1,
            FeedbackClass::String => 2,
            FeedbackClass::Integer => 3,
            FeedbackClass::Led => 4,
            FeedbackClass::Bell => 5,
        }
    }
}
impl From<FeedbackClass> for Option<u8> {
    fn from(input: FeedbackClass) -> Self {
        Some(u8::from(input))
    }
}
impl From<FeedbackClass> for u16 {
    fn from(input: FeedbackClass) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FeedbackClass> for Option<u16> {
    fn from(input: FeedbackClass) -> Self {
        Some(u16::from(input))
    }
}
impl From<FeedbackClass> for u32 {
    fn from(input: FeedbackClass) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FeedbackClass> for Option<u32> {
    fn from(input: FeedbackClass) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for FeedbackClass {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FeedbackClass::Keyboard),
            1 => Ok(FeedbackClass::Pointer),
            2 => Ok(FeedbackClass::String),
            3 => Ok(FeedbackClass::Integer),
            4 => Ok(FeedbackClass::Led),
            5 => Ok(FeedbackClass::Bell),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for FeedbackClass {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for FeedbackClass {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KbdFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub pitch: u16,
    pub duration: u16,
    pub led_mask: u32,
    pub led_values: u32,
    pub global_auto_repeat: bool,
    pub click: u8,
    pub percent: u8,
    pub auto_repeats: [u8; 32],
}
impl TryParse for KbdFeedbackState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (pitch, remaining) = u16::try_parse(remaining)?;
        let (duration, remaining) = u16::try_parse(remaining)?;
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (led_values, remaining) = u32::try_parse(remaining)?;
        let (global_auto_repeat, remaining) = bool::try_parse(remaining)?;
        let (click, remaining) = u8::try_parse(remaining)?;
        let (percent, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (auto_repeats_0, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_1, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_2, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_3, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_4, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_5, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_6, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_7, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_8, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_9, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_10, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_11, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_12, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_13, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_14, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_15, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_16, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_17, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_18, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_19, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_20, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_21, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_22, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_23, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_24, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_25, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_26, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_27, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_28, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_29, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_30, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_31, remaining) = u8::try_parse(remaining)?;
        let auto_repeats = [
            auto_repeats_0,
            auto_repeats_1,
            auto_repeats_2,
            auto_repeats_3,
            auto_repeats_4,
            auto_repeats_5,
            auto_repeats_6,
            auto_repeats_7,
            auto_repeats_8,
            auto_repeats_9,
            auto_repeats_10,
            auto_repeats_11,
            auto_repeats_12,
            auto_repeats_13,
            auto_repeats_14,
            auto_repeats_15,
            auto_repeats_16,
            auto_repeats_17,
            auto_repeats_18,
            auto_repeats_19,
            auto_repeats_20,
            auto_repeats_21,
            auto_repeats_22,
            auto_repeats_23,
            auto_repeats_24,
            auto_repeats_25,
            auto_repeats_26,
            auto_repeats_27,
            auto_repeats_28,
            auto_repeats_29,
            auto_repeats_30,
            auto_repeats_31,
        ];
        let class_id = class_id.try_into()?;
        let result = KbdFeedbackState { class_id, feedback_id, len, pitch, duration, led_mask, led_values, global_auto_repeat, click, percent, auto_repeats };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KbdFeedbackState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KbdFeedbackState {
    type Bytes = [u8; 52];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let pitch_bytes = self.pitch.serialize();
        let duration_bytes = self.duration.serialize();
        let led_mask_bytes = self.led_mask.serialize();
        let led_values_bytes = self.led_values.serialize();
        let global_auto_repeat_bytes = self.global_auto_repeat.serialize();
        let click_bytes = self.click.serialize();
        let percent_bytes = self.percent.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
            global_auto_repeat_bytes[0],
            click_bytes[0],
            percent_bytes[0],
            0,
            self.auto_repeats[0],
            self.auto_repeats[1],
            self.auto_repeats[2],
            self.auto_repeats[3],
            self.auto_repeats[4],
            self.auto_repeats[5],
            self.auto_repeats[6],
            self.auto_repeats[7],
            self.auto_repeats[8],
            self.auto_repeats[9],
            self.auto_repeats[10],
            self.auto_repeats[11],
            self.auto_repeats[12],
            self.auto_repeats[13],
            self.auto_repeats[14],
            self.auto_repeats[15],
            self.auto_repeats[16],
            self.auto_repeats[17],
            self.auto_repeats[18],
            self.auto_repeats[19],
            self.auto_repeats[20],
            self.auto_repeats[21],
            self.auto_repeats[22],
            self.auto_repeats[23],
            self.auto_repeats[24],
            self.auto_repeats[25],
            self.auto_repeats[26],
            self.auto_repeats[27],
            self.auto_repeats[28],
            self.auto_repeats[29],
            self.auto_repeats[30],
            self.auto_repeats[31],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(52);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.pitch.serialize_into(bytes);
        self.duration.serialize_into(bytes);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
        self.global_auto_repeat.serialize_into(bytes);
        self.click.serialize_into(bytes);
        self.percent.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.auto_repeats.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtrFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}
impl TryParse for PtrFeedbackState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (accel_num, remaining) = u16::try_parse(remaining)?;
        let (accel_denom, remaining) = u16::try_parse(remaining)?;
        let (threshold, remaining) = u16::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = PtrFeedbackState { class_id, feedback_id, len, accel_num, accel_denom, threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PtrFeedbackState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for PtrFeedbackState {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let accel_num_bytes = self.accel_num.serialize();
        let accel_denom_bytes = self.accel_denom.serialize();
        let threshold_bytes = self.threshold.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            0,
            0,
            accel_num_bytes[0],
            accel_num_bytes[1],
            accel_denom_bytes[0],
            accel_denom_bytes[1],
            threshold_bytes[0],
            threshold_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.accel_num.serialize_into(bytes);
        self.accel_denom.serialize_into(bytes);
        self.threshold.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub resolution: u32,
    pub min_value: i32,
    pub max_value: i32,
}
impl TryParse for IntegerFeedbackState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (resolution, remaining) = u32::try_parse(remaining)?;
        let (min_value, remaining) = i32::try_parse(remaining)?;
        let (max_value, remaining) = i32::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = IntegerFeedbackState { class_id, feedback_id, len, resolution, min_value, max_value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IntegerFeedbackState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for IntegerFeedbackState {
    type Bytes = [u8; 16];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let resolution_bytes = self.resolution.serialize();
        let min_value_bytes = self.min_value.serialize();
        let max_value_bytes = self.max_value.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            min_value_bytes[0],
            min_value_bytes[1],
            min_value_bytes[2],
            min_value_bytes[3],
            max_value_bytes[0],
            max_value_bytes[1],
            max_value_bytes[2],
            max_value_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.resolution.serialize_into(bytes);
        self.min_value.serialize_into(bytes);
        self.max_value.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub max_symbols: u16,
    pub keysyms: Vec<xproto::Keysym>,
}
impl TryParse for StringFeedbackState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (max_symbols, remaining) = u16::try_parse(remaining)?;
        let (num_keysyms, remaining) = u16::try_parse(remaining)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, num_keysyms as usize)?;
        let class_id = class_id.try_into()?;
        let result = StringFeedbackState { class_id, feedback_id, len, max_symbols, keysyms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for StringFeedbackState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for StringFeedbackState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.max_symbols.serialize_into(bytes);
        let num_keysyms = self.keysyms.len() as u16;
        num_keysyms.serialize_into(bytes);
        self.keysyms.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BellFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
}
impl TryParse for BellFeedbackState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (percent, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (pitch, remaining) = u16::try_parse(remaining)?;
        let (duration, remaining) = u16::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = BellFeedbackState { class_id, feedback_id, len, percent, pitch, duration };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BellFeedbackState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for BellFeedbackState {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let percent_bytes = self.percent.serialize();
        let pitch_bytes = self.pitch.serialize();
        let duration_bytes = self.duration.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            percent_bytes[0],
            0,
            0,
            0,
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.percent.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.pitch.serialize_into(bytes);
        self.duration.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub led_mask: u32,
    pub led_values: u32,
}
impl TryParse for LedFeedbackState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (led_values, remaining) = u32::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = LedFeedbackState { class_id, feedback_id, len, led_mask, led_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for LedFeedbackState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for LedFeedbackState {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let led_mask_bytes = self.led_mask.serialize();
        let led_values_bytes = self.led_values.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackStateDataKeyboard {
    pub pitch: u16,
    pub duration: u16,
    pub led_mask: u32,
    pub led_values: u32,
    pub global_auto_repeat: bool,
    pub click: u8,
    pub percent: u8,
    pub auto_repeats: [u8; 32],
}
impl TryParse for FeedbackStateDataKeyboard {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (pitch, remaining) = u16::try_parse(remaining)?;
        let (duration, remaining) = u16::try_parse(remaining)?;
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (led_values, remaining) = u32::try_parse(remaining)?;
        let (global_auto_repeat, remaining) = bool::try_parse(remaining)?;
        let (click, remaining) = u8::try_parse(remaining)?;
        let (percent, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (auto_repeats_0, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_1, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_2, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_3, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_4, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_5, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_6, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_7, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_8, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_9, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_10, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_11, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_12, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_13, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_14, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_15, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_16, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_17, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_18, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_19, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_20, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_21, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_22, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_23, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_24, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_25, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_26, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_27, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_28, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_29, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_30, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_31, remaining) = u8::try_parse(remaining)?;
        let auto_repeats = [
            auto_repeats_0,
            auto_repeats_1,
            auto_repeats_2,
            auto_repeats_3,
            auto_repeats_4,
            auto_repeats_5,
            auto_repeats_6,
            auto_repeats_7,
            auto_repeats_8,
            auto_repeats_9,
            auto_repeats_10,
            auto_repeats_11,
            auto_repeats_12,
            auto_repeats_13,
            auto_repeats_14,
            auto_repeats_15,
            auto_repeats_16,
            auto_repeats_17,
            auto_repeats_18,
            auto_repeats_19,
            auto_repeats_20,
            auto_repeats_21,
            auto_repeats_22,
            auto_repeats_23,
            auto_repeats_24,
            auto_repeats_25,
            auto_repeats_26,
            auto_repeats_27,
            auto_repeats_28,
            auto_repeats_29,
            auto_repeats_30,
            auto_repeats_31,
        ];
        let result = FeedbackStateDataKeyboard { pitch, duration, led_mask, led_values, global_auto_repeat, click, percent, auto_repeats };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackStateDataKeyboard {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackStateDataKeyboard {
    type Bytes = [u8; 48];
    fn serialize(&self) -> Self::Bytes {
        let pitch_bytes = self.pitch.serialize();
        let duration_bytes = self.duration.serialize();
        let led_mask_bytes = self.led_mask.serialize();
        let led_values_bytes = self.led_values.serialize();
        let global_auto_repeat_bytes = self.global_auto_repeat.serialize();
        let click_bytes = self.click.serialize();
        let percent_bytes = self.percent.serialize();
        [
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
            global_auto_repeat_bytes[0],
            click_bytes[0],
            percent_bytes[0],
            0,
            self.auto_repeats[0],
            self.auto_repeats[1],
            self.auto_repeats[2],
            self.auto_repeats[3],
            self.auto_repeats[4],
            self.auto_repeats[5],
            self.auto_repeats[6],
            self.auto_repeats[7],
            self.auto_repeats[8],
            self.auto_repeats[9],
            self.auto_repeats[10],
            self.auto_repeats[11],
            self.auto_repeats[12],
            self.auto_repeats[13],
            self.auto_repeats[14],
            self.auto_repeats[15],
            self.auto_repeats[16],
            self.auto_repeats[17],
            self.auto_repeats[18],
            self.auto_repeats[19],
            self.auto_repeats[20],
            self.auto_repeats[21],
            self.auto_repeats[22],
            self.auto_repeats[23],
            self.auto_repeats[24],
            self.auto_repeats[25],
            self.auto_repeats[26],
            self.auto_repeats[27],
            self.auto_repeats[28],
            self.auto_repeats[29],
            self.auto_repeats[30],
            self.auto_repeats[31],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(48);
        self.pitch.serialize_into(bytes);
        self.duration.serialize_into(bytes);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
        self.global_auto_repeat.serialize_into(bytes);
        self.click.serialize_into(bytes);
        self.percent.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.auto_repeats.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackStateDataPointer {
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}
impl TryParse for FeedbackStateDataPointer {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (accel_num, remaining) = u16::try_parse(remaining)?;
        let (accel_denom, remaining) = u16::try_parse(remaining)?;
        let (threshold, remaining) = u16::try_parse(remaining)?;
        let result = FeedbackStateDataPointer { accel_num, accel_denom, threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackStateDataPointer {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackStateDataPointer {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let accel_num_bytes = self.accel_num.serialize();
        let accel_denom_bytes = self.accel_denom.serialize();
        let threshold_bytes = self.threshold.serialize();
        [
            0,
            0,
            accel_num_bytes[0],
            accel_num_bytes[1],
            accel_denom_bytes[0],
            accel_denom_bytes[1],
            threshold_bytes[0],
            threshold_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        bytes.extend_from_slice(&[0; 2]);
        self.accel_num.serialize_into(bytes);
        self.accel_denom.serialize_into(bytes);
        self.threshold.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackStateDataString {
    pub max_symbols: u16,
    pub keysyms: Vec<xproto::Keysym>,
}
impl TryParse for FeedbackStateDataString {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (max_symbols, remaining) = u16::try_parse(remaining)?;
        let (num_keysyms, remaining) = u16::try_parse(remaining)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, num_keysyms as usize)?;
        let result = FeedbackStateDataString { max_symbols, keysyms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackStateDataString {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackStateDataString {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.max_symbols.serialize_into(bytes);
        let num_keysyms = self.keysyms.len() as u16;
        num_keysyms.serialize_into(bytes);
        self.keysyms.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackStateDataInteger {
    pub resolution: u32,
    pub min_value: i32,
    pub max_value: i32,
}
impl TryParse for FeedbackStateDataInteger {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (resolution, remaining) = u32::try_parse(remaining)?;
        let (min_value, remaining) = i32::try_parse(remaining)?;
        let (max_value, remaining) = i32::try_parse(remaining)?;
        let result = FeedbackStateDataInteger { resolution, min_value, max_value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackStateDataInteger {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackStateDataInteger {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let resolution_bytes = self.resolution.serialize();
        let min_value_bytes = self.min_value.serialize();
        let max_value_bytes = self.max_value.serialize();
        [
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            min_value_bytes[0],
            min_value_bytes[1],
            min_value_bytes[2],
            min_value_bytes[3],
            max_value_bytes[0],
            max_value_bytes[1],
            max_value_bytes[2],
            max_value_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.resolution.serialize_into(bytes);
        self.min_value.serialize_into(bytes);
        self.max_value.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackStateDataLed {
    pub led_mask: u32,
    pub led_values: u32,
}
impl TryParse for FeedbackStateDataLed {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (led_values, remaining) = u32::try_parse(remaining)?;
        let result = FeedbackStateDataLed { led_mask, led_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackStateDataLed {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackStateDataLed {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let led_mask_bytes = self.led_mask.serialize();
        let led_values_bytes = self.led_values.serialize();
        [
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackStateDataBell {
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
}
impl TryParse for FeedbackStateDataBell {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (percent, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (pitch, remaining) = u16::try_parse(remaining)?;
        let (duration, remaining) = u16::try_parse(remaining)?;
        let result = FeedbackStateDataBell { percent, pitch, duration };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackStateDataBell {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackStateDataBell {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let percent_bytes = self.percent.serialize();
        let pitch_bytes = self.pitch.serialize();
        let duration_bytes = self.duration.serialize();
        [
            percent_bytes[0],
            0,
            0,
            0,
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.percent.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.pitch.serialize_into(bytes);
        self.duration.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedbackStateData {
    Keyboard(FeedbackStateDataKeyboard),
    Pointer(FeedbackStateDataPointer),
    String(FeedbackStateDataString),
    Integer(FeedbackStateDataInteger),
    Led(FeedbackStateDataLed),
    Bell(FeedbackStateDataBell),
}
impl FeedbackStateData {
    fn try_parse(value: &[u8], class_id: u8) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if class_id == Into::<u8>::into(FeedbackClass::Keyboard) {
            let (keyboard, new_remaining) = FeedbackStateDataKeyboard::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Keyboard(keyboard));
        }
        if class_id == Into::<u8>::into(FeedbackClass::Pointer) {
            let (pointer, new_remaining) = FeedbackStateDataPointer::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Pointer(pointer));
        }
        if class_id == Into::<u8>::into(FeedbackClass::String) {
            let (string, new_remaining) = FeedbackStateDataString::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::String(string));
        }
        if class_id == Into::<u8>::into(FeedbackClass::Integer) {
            let (integer, new_remaining) = FeedbackStateDataInteger::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Integer(integer));
        }
        if class_id == Into::<u8>::into(FeedbackClass::Led) {
            let (led, new_remaining) = FeedbackStateDataLed::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Led(led));
        }
        if class_id == Into::<u8>::into(FeedbackClass::Bell) {
            let (bell, new_remaining) = FeedbackStateDataBell::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Bell(bell));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_keyboard(&self) -> Option<&FeedbackStateDataKeyboard> {
        match self {
            FeedbackStateData::Keyboard(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_pointer(&self) -> Option<&FeedbackStateDataPointer> {
        match self {
            FeedbackStateData::Pointer(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_string(&self) -> Option<&FeedbackStateDataString> {
        match self {
            FeedbackStateData::String(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_integer(&self) -> Option<&FeedbackStateDataInteger> {
        match self {
            FeedbackStateData::Integer(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_led(&self) -> Option<&FeedbackStateDataLed> {
        match self {
            FeedbackStateData::Led(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_bell(&self) -> Option<&FeedbackStateDataBell> {
        match self {
            FeedbackStateData::Bell(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for FeedbackStateData {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            FeedbackStateData::Keyboard(value) => value.serialize().to_vec(),
            FeedbackStateData::Pointer(value) => value.serialize().to_vec(),
            FeedbackStateData::String(value) => value.serialize(),
            FeedbackStateData::Integer(value) => value.serialize().to_vec(),
            FeedbackStateData::Led(value) => value.serialize().to_vec(),
            FeedbackStateData::Bell(value) => value.serialize().to_vec(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            FeedbackStateData::Keyboard(value) => value.serialize_into(bytes),
            FeedbackStateData::Pointer(value) => value.serialize_into(bytes),
            FeedbackStateData::String(value) => value.serialize_into(bytes),
            FeedbackStateData::Integer(value) => value.serialize_into(bytes),
            FeedbackStateData::Led(value) => value.serialize_into(bytes),
            FeedbackStateData::Bell(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub data: FeedbackStateData,
}
impl TryParse for FeedbackState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = FeedbackStateData::try_parse(remaining, class_id)?;
        let class_id = class_id.try_into()?;
        let result = FeedbackState { class_id, feedback_id, len, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

/// Opcode for the GetFeedbackControl request
pub const GET_FEEDBACK_CONTROL_REQUEST: u8 = 22;
pub fn get_feedback_control<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, GetFeedbackControlReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_FEEDBACK_CONTROL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFeedbackControlReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub feedbacks: Vec<FeedbackState>,
}
impl GetFeedbackControlReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_feedbacks, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (feedbacks, remaining) = crate::x11_utils::parse_list::<FeedbackState>(remaining, num_feedbacks as usize)?;
        let result = GetFeedbackControlReply { response_type, xi_reply_type, sequence, length, feedbacks };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetFeedbackControlReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KbdFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub key: KeyCode,
    pub auto_repeat_mode: u8,
    pub key_click_percent: i8,
    pub bell_percent: i8,
    pub bell_pitch: i16,
    pub bell_duration: i16,
    pub led_mask: u32,
    pub led_values: u32,
}
impl TryParse for KbdFeedbackCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (key, remaining) = KeyCode::try_parse(remaining)?;
        let (auto_repeat_mode, remaining) = u8::try_parse(remaining)?;
        let (key_click_percent, remaining) = i8::try_parse(remaining)?;
        let (bell_percent, remaining) = i8::try_parse(remaining)?;
        let (bell_pitch, remaining) = i16::try_parse(remaining)?;
        let (bell_duration, remaining) = i16::try_parse(remaining)?;
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (led_values, remaining) = u32::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = KbdFeedbackCtl { class_id, feedback_id, len, key, auto_repeat_mode, key_click_percent, bell_percent, bell_pitch, bell_duration, led_mask, led_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KbdFeedbackCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KbdFeedbackCtl {
    type Bytes = [u8; 20];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let key_bytes = self.key.serialize();
        let auto_repeat_mode_bytes = self.auto_repeat_mode.serialize();
        let key_click_percent_bytes = self.key_click_percent.serialize();
        let bell_percent_bytes = self.bell_percent.serialize();
        let bell_pitch_bytes = self.bell_pitch.serialize();
        let bell_duration_bytes = self.bell_duration.serialize();
        let led_mask_bytes = self.led_mask.serialize();
        let led_values_bytes = self.led_values.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            key_bytes[0],
            auto_repeat_mode_bytes[0],
            key_click_percent_bytes[0],
            bell_percent_bytes[0],
            bell_pitch_bytes[0],
            bell_pitch_bytes[1],
            bell_duration_bytes[0],
            bell_duration_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(20);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.key.serialize_into(bytes);
        self.auto_repeat_mode.serialize_into(bytes);
        self.key_click_percent.serialize_into(bytes);
        self.bell_percent.serialize_into(bytes);
        self.bell_pitch.serialize_into(bytes);
        self.bell_duration.serialize_into(bytes);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtrFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}
impl TryParse for PtrFeedbackCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (num, remaining) = i16::try_parse(remaining)?;
        let (denom, remaining) = i16::try_parse(remaining)?;
        let (threshold, remaining) = i16::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = PtrFeedbackCtl { class_id, feedback_id, len, num, denom, threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PtrFeedbackCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for PtrFeedbackCtl {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let num_bytes = self.num.serialize();
        let denom_bytes = self.denom.serialize();
        let threshold_bytes = self.threshold.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            0,
            0,
            num_bytes[0],
            num_bytes[1],
            denom_bytes[0],
            denom_bytes[1],
            threshold_bytes[0],
            threshold_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.num.serialize_into(bytes);
        self.denom.serialize_into(bytes);
        self.threshold.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub int_to_display: i32,
}
impl TryParse for IntegerFeedbackCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (int_to_display, remaining) = i32::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = IntegerFeedbackCtl { class_id, feedback_id, len, int_to_display };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IntegerFeedbackCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for IntegerFeedbackCtl {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let int_to_display_bytes = self.int_to_display.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            int_to_display_bytes[0],
            int_to_display_bytes[1],
            int_to_display_bytes[2],
            int_to_display_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.int_to_display.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub keysyms: Vec<xproto::Keysym>,
}
impl TryParse for StringFeedbackCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (num_keysyms, remaining) = u16::try_parse(remaining)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, num_keysyms as usize)?;
        let class_id = class_id.try_into()?;
        let result = StringFeedbackCtl { class_id, feedback_id, len, keysyms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for StringFeedbackCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for StringFeedbackCtl {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        let num_keysyms = self.keysyms.len() as u16;
        num_keysyms.serialize_into(bytes);
        self.keysyms.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BellFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub percent: i8,
    pub pitch: i16,
    pub duration: i16,
}
impl TryParse for BellFeedbackCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (percent, remaining) = i8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (pitch, remaining) = i16::try_parse(remaining)?;
        let (duration, remaining) = i16::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = BellFeedbackCtl { class_id, feedback_id, len, percent, pitch, duration };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BellFeedbackCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for BellFeedbackCtl {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let percent_bytes = self.percent.serialize();
        let pitch_bytes = self.pitch.serialize();
        let duration_bytes = self.duration.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            percent_bytes[0],
            0,
            0,
            0,
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.percent.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.pitch.serialize_into(bytes);
        self.duration.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub led_mask: u32,
    pub led_values: u32,
}
impl TryParse for LedFeedbackCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (led_values, remaining) = u32::try_parse(remaining)?;
        let class_id = class_id.try_into()?;
        let result = LedFeedbackCtl { class_id, feedback_id, len, led_mask, led_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for LedFeedbackCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for LedFeedbackCtl {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let len_bytes = self.len.serialize();
        let led_mask_bytes = self.led_mask.serialize();
        let led_values_bytes = self.led_values.serialize();
        [
            class_id_bytes[0],
            feedback_id_bytes[0],
            len_bytes[0],
            len_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackCtlDataKeyboard {
    pub key: KeyCode,
    pub auto_repeat_mode: u8,
    pub key_click_percent: i8,
    pub bell_percent: i8,
    pub bell_pitch: i16,
    pub bell_duration: i16,
    pub led_mask: u32,
    pub led_values: u32,
}
impl TryParse for FeedbackCtlDataKeyboard {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (key, remaining) = KeyCode::try_parse(remaining)?;
        let (auto_repeat_mode, remaining) = u8::try_parse(remaining)?;
        let (key_click_percent, remaining) = i8::try_parse(remaining)?;
        let (bell_percent, remaining) = i8::try_parse(remaining)?;
        let (bell_pitch, remaining) = i16::try_parse(remaining)?;
        let (bell_duration, remaining) = i16::try_parse(remaining)?;
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (led_values, remaining) = u32::try_parse(remaining)?;
        let result = FeedbackCtlDataKeyboard { key, auto_repeat_mode, key_click_percent, bell_percent, bell_pitch, bell_duration, led_mask, led_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackCtlDataKeyboard {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackCtlDataKeyboard {
    type Bytes = [u8; 16];
    fn serialize(&self) -> Self::Bytes {
        let key_bytes = self.key.serialize();
        let auto_repeat_mode_bytes = self.auto_repeat_mode.serialize();
        let key_click_percent_bytes = self.key_click_percent.serialize();
        let bell_percent_bytes = self.bell_percent.serialize();
        let bell_pitch_bytes = self.bell_pitch.serialize();
        let bell_duration_bytes = self.bell_duration.serialize();
        let led_mask_bytes = self.led_mask.serialize();
        let led_values_bytes = self.led_values.serialize();
        [
            key_bytes[0],
            auto_repeat_mode_bytes[0],
            key_click_percent_bytes[0],
            bell_percent_bytes[0],
            bell_pitch_bytes[0],
            bell_pitch_bytes[1],
            bell_duration_bytes[0],
            bell_duration_bytes[1],
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.key.serialize_into(bytes);
        self.auto_repeat_mode.serialize_into(bytes);
        self.key_click_percent.serialize_into(bytes);
        self.bell_percent.serialize_into(bytes);
        self.bell_pitch.serialize_into(bytes);
        self.bell_duration.serialize_into(bytes);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackCtlDataPointer {
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}
impl TryParse for FeedbackCtlDataPointer {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (num, remaining) = i16::try_parse(remaining)?;
        let (denom, remaining) = i16::try_parse(remaining)?;
        let (threshold, remaining) = i16::try_parse(remaining)?;
        let result = FeedbackCtlDataPointer { num, denom, threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackCtlDataPointer {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackCtlDataPointer {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let num_bytes = self.num.serialize();
        let denom_bytes = self.denom.serialize();
        let threshold_bytes = self.threshold.serialize();
        [
            0,
            0,
            num_bytes[0],
            num_bytes[1],
            denom_bytes[0],
            denom_bytes[1],
            threshold_bytes[0],
            threshold_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        bytes.extend_from_slice(&[0; 2]);
        self.num.serialize_into(bytes);
        self.denom.serialize_into(bytes);
        self.threshold.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackCtlDataString {
    pub keysyms: Vec<xproto::Keysym>,
}
impl TryParse for FeedbackCtlDataString {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (num_keysyms, remaining) = u16::try_parse(remaining)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, num_keysyms as usize)?;
        let result = FeedbackCtlDataString { keysyms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackCtlDataString {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackCtlDataString {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        bytes.extend_from_slice(&[0; 2]);
        let num_keysyms = self.keysyms.len() as u16;
        num_keysyms.serialize_into(bytes);
        self.keysyms.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackCtlDataLed {
    pub led_mask: u32,
    pub led_values: u32,
}
impl TryParse for FeedbackCtlDataLed {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (led_values, remaining) = u32::try_parse(remaining)?;
        let result = FeedbackCtlDataLed { led_mask, led_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackCtlDataLed {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackCtlDataLed {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let led_mask_bytes = self.led_mask.serialize();
        let led_values_bytes = self.led_values.serialize();
        [
            led_mask_bytes[0],
            led_mask_bytes[1],
            led_mask_bytes[2],
            led_mask_bytes[3],
            led_values_bytes[0],
            led_values_bytes[1],
            led_values_bytes[2],
            led_values_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackCtlDataBell {
    pub percent: i8,
    pub pitch: i16,
    pub duration: i16,
}
impl TryParse for FeedbackCtlDataBell {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (percent, remaining) = i8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (pitch, remaining) = i16::try_parse(remaining)?;
        let (duration, remaining) = i16::try_parse(remaining)?;
        let result = FeedbackCtlDataBell { percent, pitch, duration };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackCtlDataBell {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackCtlDataBell {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let percent_bytes = self.percent.serialize();
        let pitch_bytes = self.pitch.serialize();
        let duration_bytes = self.duration.serialize();
        [
            percent_bytes[0],
            0,
            0,
            0,
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.percent.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.pitch.serialize_into(bytes);
        self.duration.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedbackCtlData {
    Keyboard(FeedbackCtlDataKeyboard),
    Pointer(FeedbackCtlDataPointer),
    String(FeedbackCtlDataString),
    IntToDisplay(i32),
    Led(FeedbackCtlDataLed),
    Bell(FeedbackCtlDataBell),
}
impl FeedbackCtlData {
    fn try_parse(value: &[u8], class_id: u8) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if class_id == Into::<u8>::into(FeedbackClass::Keyboard) {
            let (keyboard, new_remaining) = FeedbackCtlDataKeyboard::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Keyboard(keyboard));
        }
        if class_id == Into::<u8>::into(FeedbackClass::Pointer) {
            let (pointer, new_remaining) = FeedbackCtlDataPointer::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Pointer(pointer));
        }
        if class_id == Into::<u8>::into(FeedbackClass::String) {
            let (string, new_remaining) = FeedbackCtlDataString::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::String(string));
        }
        if class_id == Into::<u8>::into(FeedbackClass::Integer) {
            let remaining = outer_remaining;
            let (int_to_display, remaining) = i32::try_parse(remaining)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::IntToDisplay(int_to_display));
        }
        if class_id == Into::<u8>::into(FeedbackClass::Led) {
            let (led, new_remaining) = FeedbackCtlDataLed::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Led(led));
        }
        if class_id == Into::<u8>::into(FeedbackClass::Bell) {
            let (bell, new_remaining) = FeedbackCtlDataBell::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Bell(bell));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_keyboard(&self) -> Option<&FeedbackCtlDataKeyboard> {
        match self {
            FeedbackCtlData::Keyboard(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_pointer(&self) -> Option<&FeedbackCtlDataPointer> {
        match self {
            FeedbackCtlData::Pointer(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_string(&self) -> Option<&FeedbackCtlDataString> {
        match self {
            FeedbackCtlData::String(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_int_to_display(&self) -> Option<&i32> {
        match self {
            FeedbackCtlData::IntToDisplay(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_led(&self) -> Option<&FeedbackCtlDataLed> {
        match self {
            FeedbackCtlData::Led(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_bell(&self) -> Option<&FeedbackCtlDataBell> {
        match self {
            FeedbackCtlData::Bell(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for FeedbackCtlData {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            FeedbackCtlData::Keyboard(value) => value.serialize().to_vec(),
            FeedbackCtlData::Pointer(value) => value.serialize().to_vec(),
            FeedbackCtlData::String(value) => value.serialize(),
            FeedbackCtlData::IntToDisplay(value) => value.serialize().to_vec(),
            FeedbackCtlData::Led(value) => value.serialize().to_vec(),
            FeedbackCtlData::Bell(value) => value.serialize().to_vec(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            FeedbackCtlData::Keyboard(value) => value.serialize_into(bytes),
            FeedbackCtlData::Pointer(value) => value.serialize_into(bytes),
            FeedbackCtlData::String(value) => value.serialize_into(bytes),
            FeedbackCtlData::IntToDisplay(value) => value.serialize_into(bytes),
            FeedbackCtlData::Led(value) => value.serialize_into(bytes),
            FeedbackCtlData::Bell(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: u8,
    pub len: u16,
    pub data: FeedbackCtlData,
}
impl TryParse for FeedbackCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = FeedbackCtlData::try_parse(remaining, class_id)?;
        let class_id = class_id.try_into()?;
        let result = FeedbackCtl { class_id, feedback_id, len, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FeedbackCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for FeedbackCtl {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeFeedbackControlMask {
    KeyClickPercent,
    Percent,
    Pitch,
    Duration,
    Led,
    LedMode,
    Key,
    AutoRepeatMode,
    String,
    Integer,
    AccelNum,
    AccelDenom,
    Threshold,
}
impl From<ChangeFeedbackControlMask> for u8 {
    fn from(input: ChangeFeedbackControlMask) -> Self {
        match input {
            ChangeFeedbackControlMask::KeyClickPercent => 1 << 0,
            ChangeFeedbackControlMask::Percent => 1 << 1,
            ChangeFeedbackControlMask::Pitch => 1 << 2,
            ChangeFeedbackControlMask::Duration => 1 << 3,
            ChangeFeedbackControlMask::Led => 1 << 4,
            ChangeFeedbackControlMask::LedMode => 1 << 5,
            ChangeFeedbackControlMask::Key => 1 << 6,
            ChangeFeedbackControlMask::AutoRepeatMode => 1 << 7,
            ChangeFeedbackControlMask::String => 1 << 0,
            ChangeFeedbackControlMask::Integer => 1 << 0,
            ChangeFeedbackControlMask::AccelNum => 1 << 0,
            ChangeFeedbackControlMask::AccelDenom => 1 << 1,
            ChangeFeedbackControlMask::Threshold => 1 << 2,
        }
    }
}
impl From<ChangeFeedbackControlMask> for Option<u8> {
    fn from(input: ChangeFeedbackControlMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<ChangeFeedbackControlMask> for u16 {
    fn from(input: ChangeFeedbackControlMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ChangeFeedbackControlMask> for Option<u16> {
    fn from(input: ChangeFeedbackControlMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<ChangeFeedbackControlMask> for u32 {
    fn from(input: ChangeFeedbackControlMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ChangeFeedbackControlMask> for Option<u32> {
    fn from(input: ChangeFeedbackControlMask) -> Self {
        Some(u32::from(input))
    }
}
bitmask_binop!(ChangeFeedbackControlMask, u8);

/// Opcode for the ChangeFeedbackControl request
pub const CHANGE_FEEDBACK_CONTROL_REQUEST: u8 = 23;
pub fn change_feedback_control<Conn>(conn: &Conn, mask: u32, device_id: u8, feedback_id: u8, feedback: FeedbackCtl) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let feedback_bytes = feedback.serialize();
    let length: usize = (12 + feedback_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let mask_bytes = mask.serialize();
    let device_id_bytes = device_id.serialize();
    let feedback_id_bytes = feedback_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_FEEDBACK_CONTROL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        mask_bytes[0],
        mask_bytes[1],
        mask_bytes[2],
        mask_bytes[3],
        device_id_bytes[0],
        feedback_id_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&feedback_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&feedback_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetDeviceKeyMapping request
pub const GET_DEVICE_KEY_MAPPING_REQUEST: u8 = 24;
pub fn get_device_key_mapping<Conn>(conn: &Conn, device_id: u8, first_keycode: KeyCode, count: u8) -> Result<Cookie<'_, Conn, GetDeviceKeyMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let first_keycode_bytes = first_keycode.serialize();
    let count_bytes = count.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_KEY_MAPPING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        first_keycode_bytes[0],
        count_bytes[0],
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceKeyMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub keysyms_per_keycode: u8,
    pub keysyms: Vec<xproto::Keysym>,
}
impl GetDeviceKeyMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (keysyms_per_keycode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, length as usize)?;
        let result = GetDeviceKeyMappingReply { response_type, xi_reply_type, sequence, keysyms_per_keycode, keysyms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceKeyMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ChangeDeviceKeyMapping request
pub const CHANGE_DEVICE_KEY_MAPPING_REQUEST: u8 = 25;
pub fn change_device_key_mapping<'c, Conn>(conn: &'c Conn, device_id: u8, first_keycode: KeyCode, keysyms_per_keycode: u8, keycode_count: u8, keysyms: &[xproto::Keysym]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 4 * keysyms.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let first_keycode_bytes = first_keycode.serialize();
    let keysyms_per_keycode_bytes = keysyms_per_keycode.serialize();
    let keycode_count_bytes = keycode_count.serialize();
    assert_eq!(keysyms.len(), (keycode_count as usize) * (keysyms_per_keycode as usize), "Argument keysyms has an incorrect length");
    let keysyms_bytes = keysyms.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_DEVICE_KEY_MAPPING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        first_keycode_bytes[0],
        keysyms_per_keycode_bytes[0],
        keycode_count_bytes[0],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&keysyms_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&keysyms_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetDeviceModifierMapping request
pub const GET_DEVICE_MODIFIER_MAPPING_REQUEST: u8 = 26;
pub fn get_device_modifier_mapping<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceModifierMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_MODIFIER_MAPPING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceModifierMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keymaps: Vec<u8>,
}
impl GetDeviceModifierMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (keycodes_per_modifier, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let (keymaps, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (keycodes_per_modifier as usize) * (8))?;
        let result = GetDeviceModifierMappingReply { response_type, xi_reply_type, sequence, length, keymaps };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceModifierMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetDeviceModifierMapping request
pub const SET_DEVICE_MODIFIER_MAPPING_REQUEST: u8 = 27;
pub fn set_device_modifier_mapping<'c, Conn>(conn: &'c Conn, device_id: u8, keymaps: &[u8]) -> Result<Cookie<'c, Conn, SetDeviceModifierMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * keymaps.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    assert_eq!(0, keymaps.len() % 8, "Argument keycodes_per_modifier has an incorrect length, must be a multiple of 8");
    let keycodes_per_modifier: u8 = TryInto::try_into(keymaps.len() / 8).unwrap();
    let keycodes_per_modifier_bytes = keycodes_per_modifier.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEVICE_MODIFIER_MAPPING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        keycodes_per_modifier_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (keymaps).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(keymaps), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetDeviceModifierMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::MappingStatus,
}
impl SetDeviceModifierMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = SetDeviceModifierMappingReply { response_type, xi_reply_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetDeviceModifierMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetDeviceButtonMapping request
pub const GET_DEVICE_BUTTON_MAPPING_REQUEST: u8 = 28;
pub fn get_device_button_mapping<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceButtonMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_BUTTON_MAPPING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceButtonMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub map: Vec<u8>,
}
impl GetDeviceButtonMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (map_size, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let (map, remaining) = crate::x11_utils::parse_list::<u8>(remaining, map_size as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = GetDeviceButtonMappingReply { response_type, xi_reply_type, sequence, length, map };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceButtonMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetDeviceButtonMapping request
pub const SET_DEVICE_BUTTON_MAPPING_REQUEST: u8 = 29;
pub fn set_device_button_mapping<'c, Conn>(conn: &'c Conn, device_id: u8, map: &[u8]) -> Result<Cookie<'c, Conn, SetDeviceButtonMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * map.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let map_size: u8 = map.len().try_into()?;
    let map_size_bytes = map_size.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEVICE_BUTTON_MAPPING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        map_size_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (map).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(map), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetDeviceButtonMappingReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::MappingStatus,
}
impl SetDeviceButtonMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = SetDeviceButtonMappingReply { response_type, xi_reply_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetDeviceButtonMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyState {
    pub class_id: InputClass,
    pub len: u8,
    pub num_keys: u8,
    pub keys: [u8; 32],
}
impl TryParse for KeyState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (num_keys, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (keys_0, remaining) = u8::try_parse(remaining)?;
        let (keys_1, remaining) = u8::try_parse(remaining)?;
        let (keys_2, remaining) = u8::try_parse(remaining)?;
        let (keys_3, remaining) = u8::try_parse(remaining)?;
        let (keys_4, remaining) = u8::try_parse(remaining)?;
        let (keys_5, remaining) = u8::try_parse(remaining)?;
        let (keys_6, remaining) = u8::try_parse(remaining)?;
        let (keys_7, remaining) = u8::try_parse(remaining)?;
        let (keys_8, remaining) = u8::try_parse(remaining)?;
        let (keys_9, remaining) = u8::try_parse(remaining)?;
        let (keys_10, remaining) = u8::try_parse(remaining)?;
        let (keys_11, remaining) = u8::try_parse(remaining)?;
        let (keys_12, remaining) = u8::try_parse(remaining)?;
        let (keys_13, remaining) = u8::try_parse(remaining)?;
        let (keys_14, remaining) = u8::try_parse(remaining)?;
        let (keys_15, remaining) = u8::try_parse(remaining)?;
        let (keys_16, remaining) = u8::try_parse(remaining)?;
        let (keys_17, remaining) = u8::try_parse(remaining)?;
        let (keys_18, remaining) = u8::try_parse(remaining)?;
        let (keys_19, remaining) = u8::try_parse(remaining)?;
        let (keys_20, remaining) = u8::try_parse(remaining)?;
        let (keys_21, remaining) = u8::try_parse(remaining)?;
        let (keys_22, remaining) = u8::try_parse(remaining)?;
        let (keys_23, remaining) = u8::try_parse(remaining)?;
        let (keys_24, remaining) = u8::try_parse(remaining)?;
        let (keys_25, remaining) = u8::try_parse(remaining)?;
        let (keys_26, remaining) = u8::try_parse(remaining)?;
        let (keys_27, remaining) = u8::try_parse(remaining)?;
        let (keys_28, remaining) = u8::try_parse(remaining)?;
        let (keys_29, remaining) = u8::try_parse(remaining)?;
        let (keys_30, remaining) = u8::try_parse(remaining)?;
        let (keys_31, remaining) = u8::try_parse(remaining)?;
        let keys = [
            keys_0,
            keys_1,
            keys_2,
            keys_3,
            keys_4,
            keys_5,
            keys_6,
            keys_7,
            keys_8,
            keys_9,
            keys_10,
            keys_11,
            keys_12,
            keys_13,
            keys_14,
            keys_15,
            keys_16,
            keys_17,
            keys_18,
            keys_19,
            keys_20,
            keys_21,
            keys_22,
            keys_23,
            keys_24,
            keys_25,
            keys_26,
            keys_27,
            keys_28,
            keys_29,
            keys_30,
            keys_31,
        ];
        let class_id = class_id.try_into()?;
        let result = KeyState { class_id, len, num_keys, keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeyState {
    type Bytes = [u8; 36];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let len_bytes = self.len.serialize();
        let num_keys_bytes = self.num_keys.serialize();
        [
            class_id_bytes[0],
            len_bytes[0],
            num_keys_bytes[0],
            0,
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            self.keys[4],
            self.keys[5],
            self.keys[6],
            self.keys[7],
            self.keys[8],
            self.keys[9],
            self.keys[10],
            self.keys[11],
            self.keys[12],
            self.keys[13],
            self.keys[14],
            self.keys[15],
            self.keys[16],
            self.keys[17],
            self.keys[18],
            self.keys[19],
            self.keys[20],
            self.keys[21],
            self.keys[22],
            self.keys[23],
            self.keys[24],
            self.keys[25],
            self.keys[26],
            self.keys[27],
            self.keys[28],
            self.keys[29],
            self.keys[30],
            self.keys[31],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(36);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.num_keys.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.keys.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonState {
    pub class_id: InputClass,
    pub len: u8,
    pub num_buttons: u8,
    pub buttons: [u8; 32],
}
impl TryParse for ButtonState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (num_buttons, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (buttons_0, remaining) = u8::try_parse(remaining)?;
        let (buttons_1, remaining) = u8::try_parse(remaining)?;
        let (buttons_2, remaining) = u8::try_parse(remaining)?;
        let (buttons_3, remaining) = u8::try_parse(remaining)?;
        let (buttons_4, remaining) = u8::try_parse(remaining)?;
        let (buttons_5, remaining) = u8::try_parse(remaining)?;
        let (buttons_6, remaining) = u8::try_parse(remaining)?;
        let (buttons_7, remaining) = u8::try_parse(remaining)?;
        let (buttons_8, remaining) = u8::try_parse(remaining)?;
        let (buttons_9, remaining) = u8::try_parse(remaining)?;
        let (buttons_10, remaining) = u8::try_parse(remaining)?;
        let (buttons_11, remaining) = u8::try_parse(remaining)?;
        let (buttons_12, remaining) = u8::try_parse(remaining)?;
        let (buttons_13, remaining) = u8::try_parse(remaining)?;
        let (buttons_14, remaining) = u8::try_parse(remaining)?;
        let (buttons_15, remaining) = u8::try_parse(remaining)?;
        let (buttons_16, remaining) = u8::try_parse(remaining)?;
        let (buttons_17, remaining) = u8::try_parse(remaining)?;
        let (buttons_18, remaining) = u8::try_parse(remaining)?;
        let (buttons_19, remaining) = u8::try_parse(remaining)?;
        let (buttons_20, remaining) = u8::try_parse(remaining)?;
        let (buttons_21, remaining) = u8::try_parse(remaining)?;
        let (buttons_22, remaining) = u8::try_parse(remaining)?;
        let (buttons_23, remaining) = u8::try_parse(remaining)?;
        let (buttons_24, remaining) = u8::try_parse(remaining)?;
        let (buttons_25, remaining) = u8::try_parse(remaining)?;
        let (buttons_26, remaining) = u8::try_parse(remaining)?;
        let (buttons_27, remaining) = u8::try_parse(remaining)?;
        let (buttons_28, remaining) = u8::try_parse(remaining)?;
        let (buttons_29, remaining) = u8::try_parse(remaining)?;
        let (buttons_30, remaining) = u8::try_parse(remaining)?;
        let (buttons_31, remaining) = u8::try_parse(remaining)?;
        let buttons = [
            buttons_0,
            buttons_1,
            buttons_2,
            buttons_3,
            buttons_4,
            buttons_5,
            buttons_6,
            buttons_7,
            buttons_8,
            buttons_9,
            buttons_10,
            buttons_11,
            buttons_12,
            buttons_13,
            buttons_14,
            buttons_15,
            buttons_16,
            buttons_17,
            buttons_18,
            buttons_19,
            buttons_20,
            buttons_21,
            buttons_22,
            buttons_23,
            buttons_24,
            buttons_25,
            buttons_26,
            buttons_27,
            buttons_28,
            buttons_29,
            buttons_30,
            buttons_31,
        ];
        let class_id = class_id.try_into()?;
        let result = ButtonState { class_id, len, num_buttons, buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ButtonState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ButtonState {
    type Bytes = [u8; 36];
    fn serialize(&self) -> Self::Bytes {
        let class_id_bytes = Into::<u8>::into(self.class_id).serialize();
        let len_bytes = self.len.serialize();
        let num_buttons_bytes = self.num_buttons.serialize();
        [
            class_id_bytes[0],
            len_bytes[0],
            num_buttons_bytes[0],
            0,
            self.buttons[0],
            self.buttons[1],
            self.buttons[2],
            self.buttons[3],
            self.buttons[4],
            self.buttons[5],
            self.buttons[6],
            self.buttons[7],
            self.buttons[8],
            self.buttons[9],
            self.buttons[10],
            self.buttons[11],
            self.buttons[12],
            self.buttons[13],
            self.buttons[14],
            self.buttons[15],
            self.buttons[16],
            self.buttons[17],
            self.buttons[18],
            self.buttons[19],
            self.buttons[20],
            self.buttons[21],
            self.buttons[22],
            self.buttons[23],
            self.buttons[24],
            self.buttons[25],
            self.buttons[26],
            self.buttons[27],
            self.buttons[28],
            self.buttons[29],
            self.buttons[30],
            self.buttons[31],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(36);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.num_buttons.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.buttons.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ValuatorStateModeMask {
    DeviceModeAbsolute = 1 << 0,
    OutOfProximity = 1 << 1,
}
impl From<ValuatorStateModeMask> for u8 {
    fn from(input: ValuatorStateModeMask) -> Self {
        match input {
            ValuatorStateModeMask::DeviceModeAbsolute => 1 << 0,
            ValuatorStateModeMask::OutOfProximity => 1 << 1,
        }
    }
}
impl From<ValuatorStateModeMask> for Option<u8> {
    fn from(input: ValuatorStateModeMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<ValuatorStateModeMask> for u16 {
    fn from(input: ValuatorStateModeMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ValuatorStateModeMask> for Option<u16> {
    fn from(input: ValuatorStateModeMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<ValuatorStateModeMask> for u32 {
    fn from(input: ValuatorStateModeMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ValuatorStateModeMask> for Option<u32> {
    fn from(input: ValuatorStateModeMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ValuatorStateModeMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ValuatorStateModeMask::DeviceModeAbsolute),
            2 => Ok(ValuatorStateModeMask::OutOfProximity),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ValuatorStateModeMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ValuatorStateModeMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ValuatorStateModeMask, u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValuatorState {
    pub class_id: InputClass,
    pub len: u8,
    pub mode: u8,
    pub valuators: Vec<i32>,
}
impl TryParse for ValuatorState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (valuators, remaining) = crate::x11_utils::parse_list::<i32>(remaining, num_valuators as usize)?;
        let class_id = class_id.try_into()?;
        let result = ValuatorState { class_id, len, mode, valuators };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ValuatorState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ValuatorState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        let num_valuators = self.valuators.len() as u8;
        num_valuators.serialize_into(bytes);
        self.mode.serialize_into(bytes);
        self.valuators.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputStateDataKey {
    pub num_keys: u8,
    pub keys: [u8; 32],
}
impl TryParse for InputStateDataKey {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_keys, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (keys_0, remaining) = u8::try_parse(remaining)?;
        let (keys_1, remaining) = u8::try_parse(remaining)?;
        let (keys_2, remaining) = u8::try_parse(remaining)?;
        let (keys_3, remaining) = u8::try_parse(remaining)?;
        let (keys_4, remaining) = u8::try_parse(remaining)?;
        let (keys_5, remaining) = u8::try_parse(remaining)?;
        let (keys_6, remaining) = u8::try_parse(remaining)?;
        let (keys_7, remaining) = u8::try_parse(remaining)?;
        let (keys_8, remaining) = u8::try_parse(remaining)?;
        let (keys_9, remaining) = u8::try_parse(remaining)?;
        let (keys_10, remaining) = u8::try_parse(remaining)?;
        let (keys_11, remaining) = u8::try_parse(remaining)?;
        let (keys_12, remaining) = u8::try_parse(remaining)?;
        let (keys_13, remaining) = u8::try_parse(remaining)?;
        let (keys_14, remaining) = u8::try_parse(remaining)?;
        let (keys_15, remaining) = u8::try_parse(remaining)?;
        let (keys_16, remaining) = u8::try_parse(remaining)?;
        let (keys_17, remaining) = u8::try_parse(remaining)?;
        let (keys_18, remaining) = u8::try_parse(remaining)?;
        let (keys_19, remaining) = u8::try_parse(remaining)?;
        let (keys_20, remaining) = u8::try_parse(remaining)?;
        let (keys_21, remaining) = u8::try_parse(remaining)?;
        let (keys_22, remaining) = u8::try_parse(remaining)?;
        let (keys_23, remaining) = u8::try_parse(remaining)?;
        let (keys_24, remaining) = u8::try_parse(remaining)?;
        let (keys_25, remaining) = u8::try_parse(remaining)?;
        let (keys_26, remaining) = u8::try_parse(remaining)?;
        let (keys_27, remaining) = u8::try_parse(remaining)?;
        let (keys_28, remaining) = u8::try_parse(remaining)?;
        let (keys_29, remaining) = u8::try_parse(remaining)?;
        let (keys_30, remaining) = u8::try_parse(remaining)?;
        let (keys_31, remaining) = u8::try_parse(remaining)?;
        let keys = [
            keys_0,
            keys_1,
            keys_2,
            keys_3,
            keys_4,
            keys_5,
            keys_6,
            keys_7,
            keys_8,
            keys_9,
            keys_10,
            keys_11,
            keys_12,
            keys_13,
            keys_14,
            keys_15,
            keys_16,
            keys_17,
            keys_18,
            keys_19,
            keys_20,
            keys_21,
            keys_22,
            keys_23,
            keys_24,
            keys_25,
            keys_26,
            keys_27,
            keys_28,
            keys_29,
            keys_30,
            keys_31,
        ];
        let result = InputStateDataKey { num_keys, keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputStateDataKey {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for InputStateDataKey {
    type Bytes = [u8; 34];
    fn serialize(&self) -> Self::Bytes {
        let num_keys_bytes = self.num_keys.serialize();
        [
            num_keys_bytes[0],
            0,
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            self.keys[4],
            self.keys[5],
            self.keys[6],
            self.keys[7],
            self.keys[8],
            self.keys[9],
            self.keys[10],
            self.keys[11],
            self.keys[12],
            self.keys[13],
            self.keys[14],
            self.keys[15],
            self.keys[16],
            self.keys[17],
            self.keys[18],
            self.keys[19],
            self.keys[20],
            self.keys[21],
            self.keys[22],
            self.keys[23],
            self.keys[24],
            self.keys[25],
            self.keys[26],
            self.keys[27],
            self.keys[28],
            self.keys[29],
            self.keys[30],
            self.keys[31],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(34);
        self.num_keys.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.keys.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputStateDataButton {
    pub num_buttons: u8,
    pub buttons: [u8; 32],
}
impl TryParse for InputStateDataButton {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_buttons, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (buttons_0, remaining) = u8::try_parse(remaining)?;
        let (buttons_1, remaining) = u8::try_parse(remaining)?;
        let (buttons_2, remaining) = u8::try_parse(remaining)?;
        let (buttons_3, remaining) = u8::try_parse(remaining)?;
        let (buttons_4, remaining) = u8::try_parse(remaining)?;
        let (buttons_5, remaining) = u8::try_parse(remaining)?;
        let (buttons_6, remaining) = u8::try_parse(remaining)?;
        let (buttons_7, remaining) = u8::try_parse(remaining)?;
        let (buttons_8, remaining) = u8::try_parse(remaining)?;
        let (buttons_9, remaining) = u8::try_parse(remaining)?;
        let (buttons_10, remaining) = u8::try_parse(remaining)?;
        let (buttons_11, remaining) = u8::try_parse(remaining)?;
        let (buttons_12, remaining) = u8::try_parse(remaining)?;
        let (buttons_13, remaining) = u8::try_parse(remaining)?;
        let (buttons_14, remaining) = u8::try_parse(remaining)?;
        let (buttons_15, remaining) = u8::try_parse(remaining)?;
        let (buttons_16, remaining) = u8::try_parse(remaining)?;
        let (buttons_17, remaining) = u8::try_parse(remaining)?;
        let (buttons_18, remaining) = u8::try_parse(remaining)?;
        let (buttons_19, remaining) = u8::try_parse(remaining)?;
        let (buttons_20, remaining) = u8::try_parse(remaining)?;
        let (buttons_21, remaining) = u8::try_parse(remaining)?;
        let (buttons_22, remaining) = u8::try_parse(remaining)?;
        let (buttons_23, remaining) = u8::try_parse(remaining)?;
        let (buttons_24, remaining) = u8::try_parse(remaining)?;
        let (buttons_25, remaining) = u8::try_parse(remaining)?;
        let (buttons_26, remaining) = u8::try_parse(remaining)?;
        let (buttons_27, remaining) = u8::try_parse(remaining)?;
        let (buttons_28, remaining) = u8::try_parse(remaining)?;
        let (buttons_29, remaining) = u8::try_parse(remaining)?;
        let (buttons_30, remaining) = u8::try_parse(remaining)?;
        let (buttons_31, remaining) = u8::try_parse(remaining)?;
        let buttons = [
            buttons_0,
            buttons_1,
            buttons_2,
            buttons_3,
            buttons_4,
            buttons_5,
            buttons_6,
            buttons_7,
            buttons_8,
            buttons_9,
            buttons_10,
            buttons_11,
            buttons_12,
            buttons_13,
            buttons_14,
            buttons_15,
            buttons_16,
            buttons_17,
            buttons_18,
            buttons_19,
            buttons_20,
            buttons_21,
            buttons_22,
            buttons_23,
            buttons_24,
            buttons_25,
            buttons_26,
            buttons_27,
            buttons_28,
            buttons_29,
            buttons_30,
            buttons_31,
        ];
        let result = InputStateDataButton { num_buttons, buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputStateDataButton {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for InputStateDataButton {
    type Bytes = [u8; 34];
    fn serialize(&self) -> Self::Bytes {
        let num_buttons_bytes = self.num_buttons.serialize();
        [
            num_buttons_bytes[0],
            0,
            self.buttons[0],
            self.buttons[1],
            self.buttons[2],
            self.buttons[3],
            self.buttons[4],
            self.buttons[5],
            self.buttons[6],
            self.buttons[7],
            self.buttons[8],
            self.buttons[9],
            self.buttons[10],
            self.buttons[11],
            self.buttons[12],
            self.buttons[13],
            self.buttons[14],
            self.buttons[15],
            self.buttons[16],
            self.buttons[17],
            self.buttons[18],
            self.buttons[19],
            self.buttons[20],
            self.buttons[21],
            self.buttons[22],
            self.buttons[23],
            self.buttons[24],
            self.buttons[25],
            self.buttons[26],
            self.buttons[27],
            self.buttons[28],
            self.buttons[29],
            self.buttons[30],
            self.buttons[31],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(34);
        self.num_buttons.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.buttons.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputStateDataValuator {
    pub mode: u8,
    pub valuators: Vec<i32>,
}
impl TryParse for InputStateDataValuator {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (valuators, remaining) = crate::x11_utils::parse_list::<i32>(remaining, num_valuators as usize)?;
        let result = InputStateDataValuator { mode, valuators };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputStateDataValuator {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for InputStateDataValuator {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        let num_valuators = self.valuators.len() as u8;
        num_valuators.serialize_into(bytes);
        self.mode.serialize_into(bytes);
        self.valuators.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputStateData {
    Key(InputStateDataKey),
    Button(InputStateDataButton),
    Valuator(InputStateDataValuator),
}
impl InputStateData {
    fn try_parse(value: &[u8], class_id: u8) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if class_id == Into::<u8>::into(InputClass::Key) {
            let (key, new_remaining) = InputStateDataKey::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputStateData::Key(key));
        }
        if class_id == Into::<u8>::into(InputClass::Button) {
            let (button, new_remaining) = InputStateDataButton::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputStateData::Button(button));
        }
        if class_id == Into::<u8>::into(InputClass::Valuator) {
            let (valuator, new_remaining) = InputStateDataValuator::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputStateData::Valuator(valuator));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_key(&self) -> Option<&InputStateDataKey> {
        match self {
            InputStateData::Key(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_button(&self) -> Option<&InputStateDataButton> {
        match self {
            InputStateData::Button(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_valuator(&self) -> Option<&InputStateDataValuator> {
        match self {
            InputStateData::Valuator(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for InputStateData {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            InputStateData::Key(value) => value.serialize().to_vec(),
            InputStateData::Button(value) => value.serialize().to_vec(),
            InputStateData::Valuator(value) => value.serialize(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            InputStateData::Key(value) => value.serialize_into(bytes),
            InputStateData::Button(value) => value.serialize_into(bytes),
            InputStateData::Valuator(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputState {
    pub class_id: InputClass,
    pub len: u8,
    pub data: InputStateData,
}
impl TryParse for InputState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (data, remaining) = InputStateData::try_parse(remaining, class_id)?;
        let class_id = class_id.try_into()?;
        let result = InputState { class_id, len, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InputState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for InputState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        Into::<u8>::into(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

/// Opcode for the QueryDeviceState request
pub const QUERY_DEVICE_STATE_REQUEST: u8 = 30;
pub fn query_device_state<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, QueryDeviceStateReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_DEVICE_STATE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryDeviceStateReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub classes: Vec<InputState>,
}
impl QueryDeviceStateReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_classes, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<InputState>(remaining, num_classes as usize)?;
        let result = QueryDeviceStateReply { response_type, xi_reply_type, sequence, length, classes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryDeviceStateReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DeviceBell request
pub const DEVICE_BELL_REQUEST: u8 = 32;
pub fn device_bell<Conn>(conn: &Conn, device_id: u8, feedback_id: u8, feedback_class: u8, percent: i8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let feedback_id_bytes = feedback_id.serialize();
    let feedback_class_bytes = feedback_class.serialize();
    let percent_bytes = percent.serialize();
    let request0 = [
        extension_information.major_opcode,
        DEVICE_BELL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        feedback_id_bytes[0],
        feedback_class_bytes[0],
        percent_bytes[0],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetDeviceValuators request
pub const SET_DEVICE_VALUATORS_REQUEST: u8 = 33;
pub fn set_device_valuators<'c, Conn>(conn: &'c Conn, device_id: u8, first_valuator: u8, valuators: &[i32]) -> Result<Cookie<'c, Conn, SetDeviceValuatorsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 4 * valuators.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let first_valuator_bytes = first_valuator.serialize();
    let num_valuators: u8 = valuators.len().try_into()?;
    let num_valuators_bytes = num_valuators.serialize();
    let valuators_bytes = valuators.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEVICE_VALUATORS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        first_valuator_bytes[0],
        num_valuators_bytes[0],
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&valuators_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&valuators_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetDeviceValuatorsReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl SetDeviceValuatorsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = SetDeviceValuatorsReply { response_type, xi_reply_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetDeviceValuatorsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceControl {
    Resolution = 1,
    Abscalib = 2,
    Core = 3,
    Enable = 4,
    Absarea = 5,
}
impl From<DeviceControl> for u8 {
    fn from(input: DeviceControl) -> Self {
        match input {
            DeviceControl::Resolution => 1,
            DeviceControl::Abscalib => 2,
            DeviceControl::Core => 3,
            DeviceControl::Enable => 4,
            DeviceControl::Absarea => 5,
        }
    }
}
impl From<DeviceControl> for Option<u8> {
    fn from(input: DeviceControl) -> Self {
        Some(u8::from(input))
    }
}
impl From<DeviceControl> for u16 {
    fn from(input: DeviceControl) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceControl> for Option<u16> {
    fn from(input: DeviceControl) -> Self {
        Some(u16::from(input))
    }
}
impl From<DeviceControl> for u32 {
    fn from(input: DeviceControl) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceControl> for Option<u32> {
    fn from(input: DeviceControl) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DeviceControl {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DeviceControl::Resolution),
            2 => Ok(DeviceControl::Abscalib),
            3 => Ok(DeviceControl::Core),
            4 => Ok(DeviceControl::Enable),
            5 => Ok(DeviceControl::Absarea),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for DeviceControl {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DeviceControl {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceResolutionState {
    pub control_id: DeviceControl,
    pub len: u16,
    pub num_valuators: u32,
    pub resolution_values: Vec<u32>,
    pub resolution_min: Vec<u32>,
    pub resolution_max: Vec<u32>,
}
impl TryParse for DeviceResolutionState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (num_valuators, remaining) = u32::try_parse(remaining)?;
        let (resolution_values, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators as usize)?;
        let (resolution_min, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators as usize)?;
        let (resolution_max, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators as usize)?;
        let control_id = control_id.try_into()?;
        let result = DeviceResolutionState { control_id, len, num_valuators, resolution_values, resolution_min, resolution_max };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceResolutionState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceResolutionState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.num_valuators.serialize_into(bytes);
        self.resolution_values.serialize_into(bytes);
        self.resolution_min.serialize_into(bytes);
        self.resolution_max.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceAbsCalibState {
    pub control_id: DeviceControl,
    pub len: u16,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}
impl TryParse for DeviceAbsCalibState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (min_x, remaining) = i32::try_parse(remaining)?;
        let (max_x, remaining) = i32::try_parse(remaining)?;
        let (min_y, remaining) = i32::try_parse(remaining)?;
        let (max_y, remaining) = i32::try_parse(remaining)?;
        let (flip_x, remaining) = u32::try_parse(remaining)?;
        let (flip_y, remaining) = u32::try_parse(remaining)?;
        let (rotation, remaining) = u32::try_parse(remaining)?;
        let (button_threshold, remaining) = u32::try_parse(remaining)?;
        let control_id = control_id.try_into()?;
        let result = DeviceAbsCalibState { control_id, len, min_x, max_x, min_y, max_y, flip_x, flip_y, rotation, button_threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceAbsCalibState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceAbsCalibState {
    type Bytes = [u8; 36];
    fn serialize(&self) -> Self::Bytes {
        let control_id_bytes = Into::<u16>::into(self.control_id).serialize();
        let len_bytes = self.len.serialize();
        let min_x_bytes = self.min_x.serialize();
        let max_x_bytes = self.max_x.serialize();
        let min_y_bytes = self.min_y.serialize();
        let max_y_bytes = self.max_y.serialize();
        let flip_x_bytes = self.flip_x.serialize();
        let flip_y_bytes = self.flip_y.serialize();
        let rotation_bytes = self.rotation.serialize();
        let button_threshold_bytes = self.button_threshold.serialize();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            min_x_bytes[0],
            min_x_bytes[1],
            min_x_bytes[2],
            min_x_bytes[3],
            max_x_bytes[0],
            max_x_bytes[1],
            max_x_bytes[2],
            max_x_bytes[3],
            min_y_bytes[0],
            min_y_bytes[1],
            min_y_bytes[2],
            min_y_bytes[3],
            max_y_bytes[0],
            max_y_bytes[1],
            max_y_bytes[2],
            max_y_bytes[3],
            flip_x_bytes[0],
            flip_x_bytes[1],
            flip_x_bytes[2],
            flip_x_bytes[3],
            flip_y_bytes[0],
            flip_y_bytes[1],
            flip_y_bytes[2],
            flip_y_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            rotation_bytes[2],
            rotation_bytes[3],
            button_threshold_bytes[0],
            button_threshold_bytes[1],
            button_threshold_bytes[2],
            button_threshold_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(36);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.min_x.serialize_into(bytes);
        self.max_x.serialize_into(bytes);
        self.min_y.serialize_into(bytes);
        self.max_y.serialize_into(bytes);
        self.flip_x.serialize_into(bytes);
        self.flip_y.serialize_into(bytes);
        self.rotation.serialize_into(bytes);
        self.button_threshold.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceAbsAreaState {
    pub control_id: DeviceControl,
    pub len: u16,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub screen: u32,
    pub following: u32,
}
impl TryParse for DeviceAbsAreaState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (offset_x, remaining) = u32::try_parse(remaining)?;
        let (offset_y, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u32::try_parse(remaining)?;
        let (height, remaining) = u32::try_parse(remaining)?;
        let (screen, remaining) = u32::try_parse(remaining)?;
        let (following, remaining) = u32::try_parse(remaining)?;
        let control_id = control_id.try_into()?;
        let result = DeviceAbsAreaState { control_id, len, offset_x, offset_y, width, height, screen, following };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceAbsAreaState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceAbsAreaState {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let control_id_bytes = Into::<u16>::into(self.control_id).serialize();
        let len_bytes = self.len.serialize();
        let offset_x_bytes = self.offset_x.serialize();
        let offset_y_bytes = self.offset_y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let screen_bytes = self.screen.serialize();
        let following_bytes = self.following.serialize();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            offset_x_bytes[0],
            offset_x_bytes[1],
            offset_x_bytes[2],
            offset_x_bytes[3],
            offset_y_bytes[0],
            offset_y_bytes[1],
            offset_y_bytes[2],
            offset_y_bytes[3],
            width_bytes[0],
            width_bytes[1],
            width_bytes[2],
            width_bytes[3],
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            following_bytes[0],
            following_bytes[1],
            following_bytes[2],
            following_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(28);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.offset_x.serialize_into(bytes);
        self.offset_y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.screen.serialize_into(bytes);
        self.following.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceCoreState {
    pub control_id: DeviceControl,
    pub len: u16,
    pub status: u8,
    pub iscore: u8,
}
impl TryParse for DeviceCoreState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (iscore, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let control_id = control_id.try_into()?;
        let result = DeviceCoreState { control_id, len, status, iscore };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceCoreState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceCoreState {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let control_id_bytes = Into::<u16>::into(self.control_id).serialize();
        let len_bytes = self.len.serialize();
        let status_bytes = self.status.serialize();
        let iscore_bytes = self.iscore.serialize();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            status_bytes[0],
            iscore_bytes[0],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.status.serialize_into(bytes);
        self.iscore.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceEnableState {
    pub control_id: DeviceControl,
    pub len: u16,
    pub enable: u8,
}
impl TryParse for DeviceEnableState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (enable, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let control_id = control_id.try_into()?;
        let result = DeviceEnableState { control_id, len, enable };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceEnableState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceEnableState {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let control_id_bytes = Into::<u16>::into(self.control_id).serialize();
        let len_bytes = self.len.serialize();
        let enable_bytes = self.enable.serialize();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            enable_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.enable.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceStateDataResolution {
    pub num_valuators: u32,
    pub resolution_values: Vec<u32>,
    pub resolution_min: Vec<u32>,
    pub resolution_max: Vec<u32>,
}
impl TryParse for DeviceStateDataResolution {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_valuators, remaining) = u32::try_parse(remaining)?;
        let (resolution_values, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators as usize)?;
        let (resolution_min, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators as usize)?;
        let (resolution_max, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators as usize)?;
        let result = DeviceStateDataResolution { num_valuators, resolution_values, resolution_min, resolution_max };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceStateDataResolution {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceStateDataResolution {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.num_valuators.serialize_into(bytes);
        self.resolution_values.serialize_into(bytes);
        self.resolution_min.serialize_into(bytes);
        self.resolution_max.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceStateDataAbsCalib {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}
impl TryParse for DeviceStateDataAbsCalib {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (min_x, remaining) = i32::try_parse(remaining)?;
        let (max_x, remaining) = i32::try_parse(remaining)?;
        let (min_y, remaining) = i32::try_parse(remaining)?;
        let (max_y, remaining) = i32::try_parse(remaining)?;
        let (flip_x, remaining) = u32::try_parse(remaining)?;
        let (flip_y, remaining) = u32::try_parse(remaining)?;
        let (rotation, remaining) = u32::try_parse(remaining)?;
        let (button_threshold, remaining) = u32::try_parse(remaining)?;
        let result = DeviceStateDataAbsCalib { min_x, max_x, min_y, max_y, flip_x, flip_y, rotation, button_threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceStateDataAbsCalib {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceStateDataAbsCalib {
    type Bytes = [u8; 32];
    fn serialize(&self) -> Self::Bytes {
        let min_x_bytes = self.min_x.serialize();
        let max_x_bytes = self.max_x.serialize();
        let min_y_bytes = self.min_y.serialize();
        let max_y_bytes = self.max_y.serialize();
        let flip_x_bytes = self.flip_x.serialize();
        let flip_y_bytes = self.flip_y.serialize();
        let rotation_bytes = self.rotation.serialize();
        let button_threshold_bytes = self.button_threshold.serialize();
        [
            min_x_bytes[0],
            min_x_bytes[1],
            min_x_bytes[2],
            min_x_bytes[3],
            max_x_bytes[0],
            max_x_bytes[1],
            max_x_bytes[2],
            max_x_bytes[3],
            min_y_bytes[0],
            min_y_bytes[1],
            min_y_bytes[2],
            min_y_bytes[3],
            max_y_bytes[0],
            max_y_bytes[1],
            max_y_bytes[2],
            max_y_bytes[3],
            flip_x_bytes[0],
            flip_x_bytes[1],
            flip_x_bytes[2],
            flip_x_bytes[3],
            flip_y_bytes[0],
            flip_y_bytes[1],
            flip_y_bytes[2],
            flip_y_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            rotation_bytes[2],
            rotation_bytes[3],
            button_threshold_bytes[0],
            button_threshold_bytes[1],
            button_threshold_bytes[2],
            button_threshold_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.min_x.serialize_into(bytes);
        self.max_x.serialize_into(bytes);
        self.min_y.serialize_into(bytes);
        self.max_y.serialize_into(bytes);
        self.flip_x.serialize_into(bytes);
        self.flip_y.serialize_into(bytes);
        self.rotation.serialize_into(bytes);
        self.button_threshold.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceStateDataCore {
    pub status: u8,
    pub iscore: u8,
}
impl TryParse for DeviceStateDataCore {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (status, remaining) = u8::try_parse(remaining)?;
        let (iscore, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let result = DeviceStateDataCore { status, iscore };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceStateDataCore {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceStateDataCore {
    type Bytes = [u8; 4];
    fn serialize(&self) -> Self::Bytes {
        let status_bytes = self.status.serialize();
        let iscore_bytes = self.iscore.serialize();
        [
            status_bytes[0],
            iscore_bytes[0],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.status.serialize_into(bytes);
        self.iscore.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceStateDataAbsArea {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub screen: u32,
    pub following: u32,
}
impl TryParse for DeviceStateDataAbsArea {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (offset_x, remaining) = u32::try_parse(remaining)?;
        let (offset_y, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u32::try_parse(remaining)?;
        let (height, remaining) = u32::try_parse(remaining)?;
        let (screen, remaining) = u32::try_parse(remaining)?;
        let (following, remaining) = u32::try_parse(remaining)?;
        let result = DeviceStateDataAbsArea { offset_x, offset_y, width, height, screen, following };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceStateDataAbsArea {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceStateDataAbsArea {
    type Bytes = [u8; 24];
    fn serialize(&self) -> Self::Bytes {
        let offset_x_bytes = self.offset_x.serialize();
        let offset_y_bytes = self.offset_y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let screen_bytes = self.screen.serialize();
        let following_bytes = self.following.serialize();
        [
            offset_x_bytes[0],
            offset_x_bytes[1],
            offset_x_bytes[2],
            offset_x_bytes[3],
            offset_y_bytes[0],
            offset_y_bytes[1],
            offset_y_bytes[2],
            offset_y_bytes[3],
            width_bytes[0],
            width_bytes[1],
            width_bytes[2],
            width_bytes[3],
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            following_bytes[0],
            following_bytes[1],
            following_bytes[2],
            following_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.offset_x.serialize_into(bytes);
        self.offset_y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.screen.serialize_into(bytes);
        self.following.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceStateData {
    Resolution(DeviceStateDataResolution),
    AbsCalib(DeviceStateDataAbsCalib),
    Core(DeviceStateDataCore),
    Enable(u8),
    AbsArea(DeviceStateDataAbsArea),
}
impl DeviceStateData {
    fn try_parse(value: &[u8], control_id: u16) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if control_id == Into::<u16>::into(DeviceControl::Resolution) {
            let (resolution, new_remaining) = DeviceStateDataResolution::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::Resolution(resolution));
        }
        if control_id == Into::<u16>::into(DeviceControl::Abscalib) {
            let (abs_calib, new_remaining) = DeviceStateDataAbsCalib::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::AbsCalib(abs_calib));
        }
        if control_id == Into::<u16>::into(DeviceControl::Core) {
            let (core, new_remaining) = DeviceStateDataCore::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::Core(core));
        }
        if control_id == Into::<u16>::into(DeviceControl::Enable) {
            let remaining = outer_remaining;
            let (enable, remaining) = u8::try_parse(remaining)?;
            let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::Enable(enable));
        }
        if control_id == Into::<u16>::into(DeviceControl::Absarea) {
            let (abs_area, new_remaining) = DeviceStateDataAbsArea::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::AbsArea(abs_area));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_resolution(&self) -> Option<&DeviceStateDataResolution> {
        match self {
            DeviceStateData::Resolution(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_abs_calib(&self) -> Option<&DeviceStateDataAbsCalib> {
        match self {
            DeviceStateData::AbsCalib(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_core(&self) -> Option<&DeviceStateDataCore> {
        match self {
            DeviceStateData::Core(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_enable(&self) -> Option<&u8> {
        match self {
            DeviceStateData::Enable(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_abs_area(&self) -> Option<&DeviceStateDataAbsArea> {
        match self {
            DeviceStateData::AbsArea(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for DeviceStateData {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            DeviceStateData::Resolution(value) => value.serialize(),
            DeviceStateData::AbsCalib(value) => value.serialize().to_vec(),
            DeviceStateData::Core(value) => value.serialize().to_vec(),
            DeviceStateData::Enable(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3]);
                result
            }
            DeviceStateData::AbsArea(value) => value.serialize().to_vec(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            DeviceStateData::Resolution(value) => value.serialize_into(bytes),
            DeviceStateData::AbsCalib(value) => value.serialize_into(bytes),
            DeviceStateData::Core(value) => value.serialize_into(bytes),
            DeviceStateData::Enable(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3]);
            }
            DeviceStateData::AbsArea(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceState {
    pub control_id: DeviceControl,
    pub len: u16,
    pub data: DeviceStateData,
}
impl TryParse for DeviceState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = DeviceStateData::try_parse(remaining, control_id)?;
        let control_id = control_id.try_into()?;
        let result = DeviceState { control_id, len, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceState {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

/// Opcode for the GetDeviceControl request
pub const GET_DEVICE_CONTROL_REQUEST: u8 = 34;
pub fn get_device_control<Conn, A>(conn: &Conn, control_id: A, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceControlReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u16>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let control_id = control_id.into();
    let control_id_bytes = control_id.serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_CONTROL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        control_id_bytes[0],
        control_id_bytes[1],
        device_id_bytes[0],
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceControlReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub control: DeviceState,
}
impl GetDeviceControlReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let (control, remaining) = DeviceState::try_parse(remaining)?;
        let result = GetDeviceControlReply { response_type, xi_reply_type, sequence, length, status, control };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceControlReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceResolutionCtl {
    pub control_id: DeviceControl,
    pub len: u16,
    pub first_valuator: u8,
    pub resolution_values: Vec<u32>,
}
impl TryParse for DeviceResolutionCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (first_valuator, remaining) = u8::try_parse(remaining)?;
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (resolution_values, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators as usize)?;
        let control_id = control_id.try_into()?;
        let result = DeviceResolutionCtl { control_id, len, first_valuator, resolution_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceResolutionCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceResolutionCtl {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.first_valuator.serialize_into(bytes);
        let num_valuators = self.resolution_values.len() as u8;
        num_valuators.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.resolution_values.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceAbsCalibCtl {
    pub control_id: DeviceControl,
    pub len: u16,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}
impl TryParse for DeviceAbsCalibCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (min_x, remaining) = i32::try_parse(remaining)?;
        let (max_x, remaining) = i32::try_parse(remaining)?;
        let (min_y, remaining) = i32::try_parse(remaining)?;
        let (max_y, remaining) = i32::try_parse(remaining)?;
        let (flip_x, remaining) = u32::try_parse(remaining)?;
        let (flip_y, remaining) = u32::try_parse(remaining)?;
        let (rotation, remaining) = u32::try_parse(remaining)?;
        let (button_threshold, remaining) = u32::try_parse(remaining)?;
        let control_id = control_id.try_into()?;
        let result = DeviceAbsCalibCtl { control_id, len, min_x, max_x, min_y, max_y, flip_x, flip_y, rotation, button_threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceAbsCalibCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceAbsCalibCtl {
    type Bytes = [u8; 36];
    fn serialize(&self) -> Self::Bytes {
        let control_id_bytes = Into::<u16>::into(self.control_id).serialize();
        let len_bytes = self.len.serialize();
        let min_x_bytes = self.min_x.serialize();
        let max_x_bytes = self.max_x.serialize();
        let min_y_bytes = self.min_y.serialize();
        let max_y_bytes = self.max_y.serialize();
        let flip_x_bytes = self.flip_x.serialize();
        let flip_y_bytes = self.flip_y.serialize();
        let rotation_bytes = self.rotation.serialize();
        let button_threshold_bytes = self.button_threshold.serialize();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            min_x_bytes[0],
            min_x_bytes[1],
            min_x_bytes[2],
            min_x_bytes[3],
            max_x_bytes[0],
            max_x_bytes[1],
            max_x_bytes[2],
            max_x_bytes[3],
            min_y_bytes[0],
            min_y_bytes[1],
            min_y_bytes[2],
            min_y_bytes[3],
            max_y_bytes[0],
            max_y_bytes[1],
            max_y_bytes[2],
            max_y_bytes[3],
            flip_x_bytes[0],
            flip_x_bytes[1],
            flip_x_bytes[2],
            flip_x_bytes[3],
            flip_y_bytes[0],
            flip_y_bytes[1],
            flip_y_bytes[2],
            flip_y_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            rotation_bytes[2],
            rotation_bytes[3],
            button_threshold_bytes[0],
            button_threshold_bytes[1],
            button_threshold_bytes[2],
            button_threshold_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(36);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.min_x.serialize_into(bytes);
        self.max_x.serialize_into(bytes);
        self.min_y.serialize_into(bytes);
        self.max_y.serialize_into(bytes);
        self.flip_x.serialize_into(bytes);
        self.flip_y.serialize_into(bytes);
        self.rotation.serialize_into(bytes);
        self.button_threshold.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceAbsAreaCtrl {
    pub control_id: DeviceControl,
    pub len: u16,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: i32,
    pub height: i32,
    pub screen: i32,
    pub following: u32,
}
impl TryParse for DeviceAbsAreaCtrl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (offset_x, remaining) = u32::try_parse(remaining)?;
        let (offset_y, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let (screen, remaining) = i32::try_parse(remaining)?;
        let (following, remaining) = u32::try_parse(remaining)?;
        let control_id = control_id.try_into()?;
        let result = DeviceAbsAreaCtrl { control_id, len, offset_x, offset_y, width, height, screen, following };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceAbsAreaCtrl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceAbsAreaCtrl {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let control_id_bytes = Into::<u16>::into(self.control_id).serialize();
        let len_bytes = self.len.serialize();
        let offset_x_bytes = self.offset_x.serialize();
        let offset_y_bytes = self.offset_y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let screen_bytes = self.screen.serialize();
        let following_bytes = self.following.serialize();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            offset_x_bytes[0],
            offset_x_bytes[1],
            offset_x_bytes[2],
            offset_x_bytes[3],
            offset_y_bytes[0],
            offset_y_bytes[1],
            offset_y_bytes[2],
            offset_y_bytes[3],
            width_bytes[0],
            width_bytes[1],
            width_bytes[2],
            width_bytes[3],
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            following_bytes[0],
            following_bytes[1],
            following_bytes[2],
            following_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(28);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.offset_x.serialize_into(bytes);
        self.offset_y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.screen.serialize_into(bytes);
        self.following.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceCoreCtrl {
    pub control_id: DeviceControl,
    pub len: u16,
    pub status: u8,
}
impl TryParse for DeviceCoreCtrl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let control_id = control_id.try_into()?;
        let result = DeviceCoreCtrl { control_id, len, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceCoreCtrl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceCoreCtrl {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let control_id_bytes = Into::<u16>::into(self.control_id).serialize();
        let len_bytes = self.len.serialize();
        let status_bytes = self.status.serialize();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            status_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.status.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceEnableCtrl {
    pub control_id: DeviceControl,
    pub len: u16,
    pub enable: u8,
}
impl TryParse for DeviceEnableCtrl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (enable, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let control_id = control_id.try_into()?;
        let result = DeviceEnableCtrl { control_id, len, enable };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceEnableCtrl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceEnableCtrl {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let control_id_bytes = Into::<u16>::into(self.control_id).serialize();
        let len_bytes = self.len.serialize();
        let enable_bytes = self.enable.serialize();
        [
            control_id_bytes[0],
            control_id_bytes[1],
            len_bytes[0],
            len_bytes[1],
            enable_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.enable.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCtlDataResolution {
    pub first_valuator: u8,
    pub resolution_values: Vec<u32>,
}
impl TryParse for DeviceCtlDataResolution {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (first_valuator, remaining) = u8::try_parse(remaining)?;
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (resolution_values, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators as usize)?;
        let result = DeviceCtlDataResolution { first_valuator, resolution_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceCtlDataResolution {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceCtlDataResolution {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.first_valuator.serialize_into(bytes);
        let num_valuators = self.resolution_values.len() as u8;
        num_valuators.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.resolution_values.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceCtlDataAbsCalib {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}
impl TryParse for DeviceCtlDataAbsCalib {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (min_x, remaining) = i32::try_parse(remaining)?;
        let (max_x, remaining) = i32::try_parse(remaining)?;
        let (min_y, remaining) = i32::try_parse(remaining)?;
        let (max_y, remaining) = i32::try_parse(remaining)?;
        let (flip_x, remaining) = u32::try_parse(remaining)?;
        let (flip_y, remaining) = u32::try_parse(remaining)?;
        let (rotation, remaining) = u32::try_parse(remaining)?;
        let (button_threshold, remaining) = u32::try_parse(remaining)?;
        let result = DeviceCtlDataAbsCalib { min_x, max_x, min_y, max_y, flip_x, flip_y, rotation, button_threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceCtlDataAbsCalib {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceCtlDataAbsCalib {
    type Bytes = [u8; 32];
    fn serialize(&self) -> Self::Bytes {
        let min_x_bytes = self.min_x.serialize();
        let max_x_bytes = self.max_x.serialize();
        let min_y_bytes = self.min_y.serialize();
        let max_y_bytes = self.max_y.serialize();
        let flip_x_bytes = self.flip_x.serialize();
        let flip_y_bytes = self.flip_y.serialize();
        let rotation_bytes = self.rotation.serialize();
        let button_threshold_bytes = self.button_threshold.serialize();
        [
            min_x_bytes[0],
            min_x_bytes[1],
            min_x_bytes[2],
            min_x_bytes[3],
            max_x_bytes[0],
            max_x_bytes[1],
            max_x_bytes[2],
            max_x_bytes[3],
            min_y_bytes[0],
            min_y_bytes[1],
            min_y_bytes[2],
            min_y_bytes[3],
            max_y_bytes[0],
            max_y_bytes[1],
            max_y_bytes[2],
            max_y_bytes[3],
            flip_x_bytes[0],
            flip_x_bytes[1],
            flip_x_bytes[2],
            flip_x_bytes[3],
            flip_y_bytes[0],
            flip_y_bytes[1],
            flip_y_bytes[2],
            flip_y_bytes[3],
            rotation_bytes[0],
            rotation_bytes[1],
            rotation_bytes[2],
            rotation_bytes[3],
            button_threshold_bytes[0],
            button_threshold_bytes[1],
            button_threshold_bytes[2],
            button_threshold_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.min_x.serialize_into(bytes);
        self.max_x.serialize_into(bytes);
        self.min_y.serialize_into(bytes);
        self.max_y.serialize_into(bytes);
        self.flip_x.serialize_into(bytes);
        self.flip_y.serialize_into(bytes);
        self.rotation.serialize_into(bytes);
        self.button_threshold.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceCtlDataAbsArea {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: i32,
    pub height: i32,
    pub screen: i32,
    pub following: u32,
}
impl TryParse for DeviceCtlDataAbsArea {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (offset_x, remaining) = u32::try_parse(remaining)?;
        let (offset_y, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let (screen, remaining) = i32::try_parse(remaining)?;
        let (following, remaining) = u32::try_parse(remaining)?;
        let result = DeviceCtlDataAbsArea { offset_x, offset_y, width, height, screen, following };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceCtlDataAbsArea {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceCtlDataAbsArea {
    type Bytes = [u8; 24];
    fn serialize(&self) -> Self::Bytes {
        let offset_x_bytes = self.offset_x.serialize();
        let offset_y_bytes = self.offset_y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let screen_bytes = self.screen.serialize();
        let following_bytes = self.following.serialize();
        [
            offset_x_bytes[0],
            offset_x_bytes[1],
            offset_x_bytes[2],
            offset_x_bytes[3],
            offset_y_bytes[0],
            offset_y_bytes[1],
            offset_y_bytes[2],
            offset_y_bytes[3],
            width_bytes[0],
            width_bytes[1],
            width_bytes[2],
            width_bytes[3],
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            following_bytes[0],
            following_bytes[1],
            following_bytes[2],
            following_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.offset_x.serialize_into(bytes);
        self.offset_y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.screen.serialize_into(bytes);
        self.following.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceCtlData {
    Resolution(DeviceCtlDataResolution),
    AbsCalib(DeviceCtlDataAbsCalib),
    Status(u8),
    Enable(u8),
    AbsArea(DeviceCtlDataAbsArea),
}
impl DeviceCtlData {
    fn try_parse(value: &[u8], control_id: u16) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if control_id == Into::<u16>::into(DeviceControl::Resolution) {
            let (resolution, new_remaining) = DeviceCtlDataResolution::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::Resolution(resolution));
        }
        if control_id == Into::<u16>::into(DeviceControl::Abscalib) {
            let (abs_calib, new_remaining) = DeviceCtlDataAbsCalib::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::AbsCalib(abs_calib));
        }
        if control_id == Into::<u16>::into(DeviceControl::Core) {
            let remaining = outer_remaining;
            let (status, remaining) = u8::try_parse(remaining)?;
            let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::Status(status));
        }
        if control_id == Into::<u16>::into(DeviceControl::Enable) {
            let remaining = outer_remaining;
            let (enable, remaining) = u8::try_parse(remaining)?;
            let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::Enable(enable));
        }
        if control_id == Into::<u16>::into(DeviceControl::Absarea) {
            let (abs_area, new_remaining) = DeviceCtlDataAbsArea::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::AbsArea(abs_area));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_resolution(&self) -> Option<&DeviceCtlDataResolution> {
        match self {
            DeviceCtlData::Resolution(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_abs_calib(&self) -> Option<&DeviceCtlDataAbsCalib> {
        match self {
            DeviceCtlData::AbsCalib(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_status(&self) -> Option<&u8> {
        match self {
            DeviceCtlData::Status(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_enable(&self) -> Option<&u8> {
        match self {
            DeviceCtlData::Enable(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_abs_area(&self) -> Option<&DeviceCtlDataAbsArea> {
        match self {
            DeviceCtlData::AbsArea(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for DeviceCtlData {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            DeviceCtlData::Resolution(value) => value.serialize(),
            DeviceCtlData::AbsCalib(value) => value.serialize().to_vec(),
            DeviceCtlData::Status(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3]);
                result
            }
            DeviceCtlData::Enable(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3]);
                result
            }
            DeviceCtlData::AbsArea(value) => value.serialize().to_vec(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            DeviceCtlData::Resolution(value) => value.serialize_into(bytes),
            DeviceCtlData::AbsCalib(value) => value.serialize_into(bytes),
            DeviceCtlData::Status(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3]);
            }
            DeviceCtlData::Enable(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3]);
            }
            DeviceCtlData::AbsArea(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCtl {
    pub control_id: DeviceControl,
    pub len: u16,
    pub data: DeviceCtlData,
}
impl TryParse for DeviceCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = DeviceCtlData::try_parse(remaining, control_id)?;
        let control_id = control_id.try_into()?;
        let result = DeviceCtl { control_id, len, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceCtl {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceCtl {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        Into::<u16>::into(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

/// Opcode for the ChangeDeviceControl request
pub const CHANGE_DEVICE_CONTROL_REQUEST: u8 = 35;
pub fn change_device_control<Conn, A>(conn: &Conn, control_id: A, device_id: u8, control: DeviceCtl) -> Result<Cookie<'_, Conn, ChangeDeviceControlReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u16>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let control_bytes = control.serialize();
    let length: usize = (8 + control_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let control_id = control_id.into();
    let control_id_bytes = control_id.serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_DEVICE_CONTROL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        control_id_bytes[0],
        control_id_bytes[1],
        device_id_bytes[0],
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&control_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&control_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeDeviceControlReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
}
impl ChangeDeviceControlReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let result = ChangeDeviceControlReply { response_type, xi_reply_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ChangeDeviceControlReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListDeviceProperties request
pub const LIST_DEVICE_PROPERTIES_REQUEST: u8 = 36;
pub fn list_device_properties<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, ListDevicePropertiesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_DEVICE_PROPERTIES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListDevicePropertiesReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Vec<xproto::Atom>,
}
impl ListDevicePropertiesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_atoms, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (atoms, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_atoms as usize)?;
        let result = ListDevicePropertiesReply { response_type, xi_reply_type, sequence, length, atoms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListDevicePropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PropertyFormat {
    M8Bits = 8,
    M16Bits = 16,
    M32Bits = 32,
}
impl From<PropertyFormat> for u8 {
    fn from(input: PropertyFormat) -> Self {
        match input {
            PropertyFormat::M8Bits => 8,
            PropertyFormat::M16Bits => 16,
            PropertyFormat::M32Bits => 32,
        }
    }
}
impl From<PropertyFormat> for Option<u8> {
    fn from(input: PropertyFormat) -> Self {
        Some(u8::from(input))
    }
}
impl From<PropertyFormat> for u16 {
    fn from(input: PropertyFormat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PropertyFormat> for Option<u16> {
    fn from(input: PropertyFormat) -> Self {
        Some(u16::from(input))
    }
}
impl From<PropertyFormat> for u32 {
    fn from(input: PropertyFormat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PropertyFormat> for Option<u32> {
    fn from(input: PropertyFormat) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PropertyFormat {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            8 => Ok(PropertyFormat::M8Bits),
            16 => Ok(PropertyFormat::M16Bits),
            32 => Ok(PropertyFormat::M32Bits),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for PropertyFormat {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PropertyFormat {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ChangeDeviceProperty request
pub const CHANGE_DEVICE_PROPERTY_REQUEST: u8 = 37;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeDevicePropertyAux {
    Data8(Vec<u8>),
    Data16(Vec<u16>),
    Data32(Vec<u32>),
}
impl Serialize for ChangeDevicePropertyAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            ChangeDevicePropertyAux::Data8(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3][..(4 - (result.len() % 4)) % 4]);
                result
            }
            ChangeDevicePropertyAux::Data16(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3][..(4 - (result.len() % 4)) % 4]);
                result
            }
            ChangeDevicePropertyAux::Data32(value) => value.serialize(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            ChangeDevicePropertyAux::Data8(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            ChangeDevicePropertyAux::Data16(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            ChangeDevicePropertyAux::Data32(value) => value.serialize_into(bytes),
        }
    }
}
impl ChangeDevicePropertyAux {
    fn format(&self) -> u8 {
        match self {
            ChangeDevicePropertyAux::Data8(_) => PropertyFormat::M8Bits.into(),
            ChangeDevicePropertyAux::Data16(_) => PropertyFormat::M16Bits.into(),
            ChangeDevicePropertyAux::Data32(_) => PropertyFormat::M32Bits.into(),
        }
    }
}
pub fn change_device_property<'c, Conn, A>(conn: &'c Conn, property: xproto::Atom, type_: xproto::Atom, device_id: u8, mode: A, num_items: u32, items: &ChangeDevicePropertyAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let format = items.format();
    let items_bytes = items.serialize();
    let length: usize = (20 + items_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let device_id_bytes = device_id.serialize();
    let format_bytes = format.serialize();
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let num_items_bytes = num_items.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_DEVICE_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        device_id_bytes[0],
        format_bytes[0],
        mode_bytes[0],
        0,
        num_items_bytes[0],
        num_items_bytes[1],
        num_items_bytes[2],
        num_items_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&items_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&items_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeleteDeviceProperty request
pub const DELETE_DEVICE_PROPERTY_REQUEST: u8 = 38;
pub fn delete_device_property<Conn>(conn: &Conn, property: xproto::Atom, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let property_bytes = property.serialize();
    let device_id_bytes = device_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_DEVICE_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        device_id_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetDeviceProperty request
pub const GET_DEVICE_PROPERTY_REQUEST: u8 = 39;
pub fn get_device_property<Conn>(conn: &Conn, property: xproto::Atom, type_: xproto::Atom, offset: u32, len: u32, device_id: u8, delete: bool) -> Result<Cookie<'_, Conn, GetDevicePropertyReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let offset_bytes = offset.serialize();
    let len_bytes = len.serialize();
    let device_id_bytes = device_id.serialize();
    let delete_bytes = (delete as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        offset_bytes[0],
        offset_bytes[1],
        offset_bytes[2],
        offset_bytes[3],
        len_bytes[0],
        len_bytes[1],
        len_bytes[2],
        len_bytes[3],
        device_id_bytes[0],
        delete_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetDevicePropertyItems {
    Data8(Vec<u8>),
    Data16(Vec<u16>),
    Data32(Vec<u32>),
}
impl GetDevicePropertyItems {
    fn try_parse(value: &[u8], format: u8, num_items: u32) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if format == Into::<u8>::into(PropertyFormat::M8Bits) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data8, remaining) = crate::x11_utils::parse_list::<u8>(remaining, num_items as usize)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(GetDevicePropertyItems::Data8(data8));
        }
        if format == Into::<u8>::into(PropertyFormat::M16Bits) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data16, remaining) = crate::x11_utils::parse_list::<u16>(remaining, num_items as usize)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(GetDevicePropertyItems::Data16(data16));
        }
        if format == Into::<u8>::into(PropertyFormat::M32Bits) {
            let remaining = outer_remaining;
            let (data32, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_items as usize)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(GetDevicePropertyItems::Data32(data32));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_data8(&self) -> Option<&Vec<u8>> {
        match self {
            GetDevicePropertyItems::Data8(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_data16(&self) -> Option<&Vec<u16>> {
        match self {
            GetDevicePropertyItems::Data16(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_data32(&self) -> Option<&Vec<u32>> {
        match self {
            GetDevicePropertyItems::Data32(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for GetDevicePropertyItems {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            GetDevicePropertyItems::Data8(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3][..(4 - (result.len() % 4)) % 4]);
                result
            }
            GetDevicePropertyItems::Data16(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3][..(4 - (result.len() % 4)) % 4]);
                result
            }
            GetDevicePropertyItems::Data32(value) => value.serialize(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            GetDevicePropertyItems::Data8(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            GetDevicePropertyItems::Data16(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            GetDevicePropertyItems::Data32(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDevicePropertyReply {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xproto::Atom,
    pub bytes_after: u32,
    pub num_items: u32,
    pub format: PropertyFormat,
    pub device_id: u8,
    pub items: GetDevicePropertyItems,
}
impl GetDevicePropertyReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(10..).ok_or(ParseError::ParseError)?;
        let (items, remaining) = GetDevicePropertyItems::try_parse(remaining, format, num_items)?;
        let format = format.try_into()?;
        let result = GetDevicePropertyReply { response_type, xi_reply_type, sequence, length, type_, bytes_after, num_items, format, device_id, items };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDevicePropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Device {
    All = 0,
    AllMaster = 1,
}
impl From<Device> for u8 {
    fn from(input: Device) -> Self {
        match input {
            Device::All => 0,
            Device::AllMaster => 1,
        }
    }
}
impl From<Device> for Option<u8> {
    fn from(input: Device) -> Self {
        Some(u8::from(input))
    }
}
impl From<Device> for u16 {
    fn from(input: Device) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Device> for Option<u16> {
    fn from(input: Device) -> Self {
        Some(u16::from(input))
    }
}
impl From<Device> for u32 {
    fn from(input: Device) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Device> for Option<u32> {
    fn from(input: Device) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Device {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Device::All),
            1 => Ok(Device::AllMaster),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Device {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Device {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupInfo {
    pub base: u8,
    pub latched: u8,
    pub locked: u8,
    pub effective: u8,
}
impl TryParse for GroupInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (base, remaining) = u8::try_parse(remaining)?;
        let (latched, remaining) = u8::try_parse(remaining)?;
        let (locked, remaining) = u8::try_parse(remaining)?;
        let (effective, remaining) = u8::try_parse(remaining)?;
        let result = GroupInfo { base, latched, locked, effective };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GroupInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for GroupInfo {
    type Bytes = [u8; 4];
    fn serialize(&self) -> Self::Bytes {
        let base_bytes = self.base.serialize();
        let latched_bytes = self.latched.serialize();
        let locked_bytes = self.locked.serialize();
        let effective_bytes = self.effective.serialize();
        [
            base_bytes[0],
            latched_bytes[0],
            locked_bytes[0],
            effective_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.base.serialize_into(bytes);
        self.latched.serialize_into(bytes);
        self.locked.serialize_into(bytes);
        self.effective.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModifierInfo {
    pub base: u32,
    pub latched: u32,
    pub locked: u32,
    pub effective: u32,
}
impl TryParse for ModifierInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (base, remaining) = u32::try_parse(remaining)?;
        let (latched, remaining) = u32::try_parse(remaining)?;
        let (locked, remaining) = u32::try_parse(remaining)?;
        let (effective, remaining) = u32::try_parse(remaining)?;
        let result = ModifierInfo { base, latched, locked, effective };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ModifierInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ModifierInfo {
    type Bytes = [u8; 16];
    fn serialize(&self) -> Self::Bytes {
        let base_bytes = self.base.serialize();
        let latched_bytes = self.latched.serialize();
        let locked_bytes = self.locked.serialize();
        let effective_bytes = self.effective.serialize();
        [
            base_bytes[0],
            base_bytes[1],
            base_bytes[2],
            base_bytes[3],
            latched_bytes[0],
            latched_bytes[1],
            latched_bytes[2],
            latched_bytes[3],
            locked_bytes[0],
            locked_bytes[1],
            locked_bytes[2],
            locked_bytes[3],
            effective_bytes[0],
            effective_bytes[1],
            effective_bytes[2],
            effective_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.base.serialize_into(bytes);
        self.latched.serialize_into(bytes);
        self.locked.serialize_into(bytes);
        self.effective.serialize_into(bytes);
    }
}

/// Opcode for the XIQueryPointer request
pub const XI_QUERY_POINTER_REQUEST: u8 = 40;
pub fn xi_query_pointer<Conn>(conn: &Conn, window: xproto::Window, deviceid: DeviceId) -> Result<Cookie<'_, Conn, XIQueryPointerReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_QUERY_POINTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XIQueryPointerReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub win_x: Fp1616,
    pub win_y: Fp1616,
    pub same_screen: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
}
impl XIQueryPointerReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (win_x, remaining) = Fp1616::try_parse(remaining)?;
        let (win_y, remaining) = Fp1616::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (buttons, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let result = XIQueryPointerReply { response_type, sequence, length, root, child, root_x, root_y, win_x, win_y, same_screen, mods, group, buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIQueryPointerReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the XIWarpPointer request
pub const XI_WARP_POINTER_REQUEST: u8 = 41;
pub fn xi_warp_pointer<Conn>(conn: &Conn, src_win: xproto::Window, dst_win: xproto::Window, src_x: Fp1616, src_y: Fp1616, src_width: u16, src_height: u16, dst_x: Fp1616, dst_y: Fp1616, deviceid: DeviceId) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (36) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let src_win_bytes = src_win.serialize();
    let dst_win_bytes = dst_win.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let src_width_bytes = src_width.serialize();
    let src_height_bytes = src_height.serialize();
    let dst_x_bytes = dst_x.serialize();
    let dst_y_bytes = dst_y.serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_WARP_POINTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        src_win_bytes[0],
        src_win_bytes[1],
        src_win_bytes[2],
        src_win_bytes[3],
        dst_win_bytes[0],
        dst_win_bytes[1],
        dst_win_bytes[2],
        dst_win_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_x_bytes[2],
        src_x_bytes[3],
        src_y_bytes[0],
        src_y_bytes[1],
        src_y_bytes[2],
        src_y_bytes[3],
        src_width_bytes[0],
        src_width_bytes[1],
        src_height_bytes[0],
        src_height_bytes[1],
        dst_x_bytes[0],
        dst_x_bytes[1],
        dst_x_bytes[2],
        dst_x_bytes[3],
        dst_y_bytes[0],
        dst_y_bytes[1],
        dst_y_bytes[2],
        dst_y_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the XIChangeCursor request
pub const XI_CHANGE_CURSOR_REQUEST: u8 = 42;
pub fn xi_change_cursor<Conn>(conn: &Conn, window: xproto::Window, cursor: xproto::Cursor, deviceid: DeviceId) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let cursor_bytes = cursor.serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_CHANGE_CURSOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HierarchyChangeType {
    AddMaster = 1,
    RemoveMaster = 2,
    AttachSlave = 3,
    DetachSlave = 4,
}
impl From<HierarchyChangeType> for u8 {
    fn from(input: HierarchyChangeType) -> Self {
        match input {
            HierarchyChangeType::AddMaster => 1,
            HierarchyChangeType::RemoveMaster => 2,
            HierarchyChangeType::AttachSlave => 3,
            HierarchyChangeType::DetachSlave => 4,
        }
    }
}
impl From<HierarchyChangeType> for Option<u8> {
    fn from(input: HierarchyChangeType) -> Self {
        Some(u8::from(input))
    }
}
impl From<HierarchyChangeType> for u16 {
    fn from(input: HierarchyChangeType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<HierarchyChangeType> for Option<u16> {
    fn from(input: HierarchyChangeType) -> Self {
        Some(u16::from(input))
    }
}
impl From<HierarchyChangeType> for u32 {
    fn from(input: HierarchyChangeType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<HierarchyChangeType> for Option<u32> {
    fn from(input: HierarchyChangeType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for HierarchyChangeType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HierarchyChangeType::AddMaster),
            2 => Ok(HierarchyChangeType::RemoveMaster),
            3 => Ok(HierarchyChangeType::AttachSlave),
            4 => Ok(HierarchyChangeType::DetachSlave),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for HierarchyChangeType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for HierarchyChangeType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChangeMode {
    Attach = 1,
    Float = 2,
}
impl From<ChangeMode> for u8 {
    fn from(input: ChangeMode) -> Self {
        match input {
            ChangeMode::Attach => 1,
            ChangeMode::Float => 2,
        }
    }
}
impl From<ChangeMode> for Option<u8> {
    fn from(input: ChangeMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<ChangeMode> for u16 {
    fn from(input: ChangeMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ChangeMode> for Option<u16> {
    fn from(input: ChangeMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<ChangeMode> for u32 {
    fn from(input: ChangeMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ChangeMode> for Option<u32> {
    fn from(input: ChangeMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ChangeMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ChangeMode::Attach),
            2 => Ok(ChangeMode::Float),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ChangeMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ChangeMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMaster {
    pub type_: HierarchyChangeType,
    pub len: u16,
    pub send_core: bool,
    pub enable: bool,
    pub name: Vec<u8>,
}
impl TryParse for AddMaster {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (send_core, remaining) = bool::try_parse(remaining)?;
        let (enable, remaining) = bool::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = AddMaster { type_, len, send_core, enable, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AddMaster {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for AddMaster {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        let name_len = self.name.len() as u16;
        name_len.serialize_into(bytes);
        self.send_core.serialize_into(bytes);
        self.enable.serialize_into(bytes);
        self.name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoveMaster {
    pub type_: HierarchyChangeType,
    pub len: u16,
    pub deviceid: DeviceId,
    pub return_mode: ChangeMode,
    pub return_pointer: DeviceId,
    pub return_keyboard: DeviceId,
}
impl TryParse for RemoveMaster {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (return_mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (return_pointer, remaining) = DeviceId::try_parse(remaining)?;
        let (return_keyboard, remaining) = DeviceId::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let return_mode = return_mode.try_into()?;
        let result = RemoveMaster { type_, len, deviceid, return_mode, return_pointer, return_keyboard };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RemoveMaster {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for RemoveMaster {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u16>::into(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let return_mode_bytes = Into::<u8>::into(self.return_mode).serialize();
        let return_pointer_bytes = self.return_pointer.serialize();
        let return_keyboard_bytes = self.return_keyboard.serialize();
        [
            type_bytes[0],
            type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            return_mode_bytes[0],
            0,
            return_pointer_bytes[0],
            return_pointer_bytes[1],
            return_keyboard_bytes[0],
            return_keyboard_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        Into::<u8>::into(self.return_mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.return_pointer.serialize_into(bytes);
        self.return_keyboard.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttachSlave {
    pub type_: HierarchyChangeType,
    pub len: u16,
    pub deviceid: DeviceId,
    pub master: DeviceId,
}
impl TryParse for AttachSlave {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (master, remaining) = DeviceId::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let result = AttachSlave { type_, len, deviceid, master };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AttachSlave {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for AttachSlave {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u16>::into(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let master_bytes = self.master.serialize();
        [
            type_bytes[0],
            type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            master_bytes[0],
            master_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.master.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DetachSlave {
    pub type_: HierarchyChangeType,
    pub len: u16,
    pub deviceid: DeviceId,
}
impl TryParse for DetachSlave {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = DetachSlave { type_, len, deviceid };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DetachSlave {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DetachSlave {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u16>::into(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        [
            type_bytes[0],
            type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyChangeDataAddMaster {
    pub send_core: bool,
    pub enable: bool,
    pub name: Vec<u8>,
}
impl TryParse for HierarchyChangeDataAddMaster {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (send_core, remaining) = bool::try_parse(remaining)?;
        let (enable, remaining) = bool::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = HierarchyChangeDataAddMaster { send_core, enable, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for HierarchyChangeDataAddMaster {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for HierarchyChangeDataAddMaster {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let name_len = self.name.len() as u16;
        name_len.serialize_into(bytes);
        self.send_core.serialize_into(bytes);
        self.enable.serialize_into(bytes);
        self.name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HierarchyChangeDataRemoveMaster {
    pub deviceid: DeviceId,
    pub return_mode: ChangeMode,
    pub return_pointer: DeviceId,
    pub return_keyboard: DeviceId,
}
impl TryParse for HierarchyChangeDataRemoveMaster {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (return_mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (return_pointer, remaining) = DeviceId::try_parse(remaining)?;
        let (return_keyboard, remaining) = DeviceId::try_parse(remaining)?;
        let return_mode = return_mode.try_into()?;
        let result = HierarchyChangeDataRemoveMaster { deviceid, return_mode, return_pointer, return_keyboard };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for HierarchyChangeDataRemoveMaster {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for HierarchyChangeDataRemoveMaster {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let deviceid_bytes = self.deviceid.serialize();
        let return_mode_bytes = Into::<u8>::into(self.return_mode).serialize();
        let return_pointer_bytes = self.return_pointer.serialize();
        let return_keyboard_bytes = self.return_keyboard.serialize();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            return_mode_bytes[0],
            0,
            return_pointer_bytes[0],
            return_pointer_bytes[1],
            return_keyboard_bytes[0],
            return_keyboard_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.deviceid.serialize_into(bytes);
        Into::<u8>::into(self.return_mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.return_pointer.serialize_into(bytes);
        self.return_keyboard.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HierarchyChangeDataAttachSlave {
    pub deviceid: DeviceId,
    pub master: DeviceId,
}
impl TryParse for HierarchyChangeDataAttachSlave {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (master, remaining) = DeviceId::try_parse(remaining)?;
        let result = HierarchyChangeDataAttachSlave { deviceid, master };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for HierarchyChangeDataAttachSlave {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for HierarchyChangeDataAttachSlave {
    type Bytes = [u8; 4];
    fn serialize(&self) -> Self::Bytes {
        let deviceid_bytes = self.deviceid.serialize();
        let master_bytes = self.master.serialize();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            master_bytes[0],
            master_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.deviceid.serialize_into(bytes);
        self.master.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HierarchyChangeData {
    AddMaster(HierarchyChangeDataAddMaster),
    RemoveMaster(HierarchyChangeDataRemoveMaster),
    AttachSlave(HierarchyChangeDataAttachSlave),
    Deviceid(DeviceId),
}
impl HierarchyChangeData {
    fn try_parse(value: &[u8], type_: u16) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if type_ == Into::<u16>::into(HierarchyChangeType::AddMaster) {
            let (add_master, new_remaining) = HierarchyChangeDataAddMaster::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(HierarchyChangeData::AddMaster(add_master));
        }
        if type_ == Into::<u16>::into(HierarchyChangeType::RemoveMaster) {
            let (remove_master, new_remaining) = HierarchyChangeDataRemoveMaster::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(HierarchyChangeData::RemoveMaster(remove_master));
        }
        if type_ == Into::<u16>::into(HierarchyChangeType::AttachSlave) {
            let (attach_slave, new_remaining) = HierarchyChangeDataAttachSlave::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(HierarchyChangeData::AttachSlave(attach_slave));
        }
        if type_ == Into::<u16>::into(HierarchyChangeType::DetachSlave) {
            let remaining = outer_remaining;
            let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
            let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(HierarchyChangeData::Deviceid(deviceid));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_add_master(&self) -> Option<&HierarchyChangeDataAddMaster> {
        match self {
            HierarchyChangeData::AddMaster(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_remove_master(&self) -> Option<&HierarchyChangeDataRemoveMaster> {
        match self {
            HierarchyChangeData::RemoveMaster(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_attach_slave(&self) -> Option<&HierarchyChangeDataAttachSlave> {
        match self {
            HierarchyChangeData::AttachSlave(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_deviceid(&self) -> Option<&DeviceId> {
        match self {
            HierarchyChangeData::Deviceid(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for HierarchyChangeData {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            HierarchyChangeData::AddMaster(value) => value.serialize(),
            HierarchyChangeData::RemoveMaster(value) => value.serialize().to_vec(),
            HierarchyChangeData::AttachSlave(value) => value.serialize().to_vec(),
            HierarchyChangeData::Deviceid(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 2]);
                result
            }
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            HierarchyChangeData::AddMaster(value) => value.serialize_into(bytes),
            HierarchyChangeData::RemoveMaster(value) => value.serialize_into(bytes),
            HierarchyChangeData::AttachSlave(value) => value.serialize_into(bytes),
            HierarchyChangeData::Deviceid(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 2]);
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyChange {
    pub type_: HierarchyChangeType,
    pub len: u16,
    pub data: HierarchyChangeData,
}
impl TryParse for HierarchyChange {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = HierarchyChangeData::try_parse(remaining, type_)?;
        let type_ = type_.try_into()?;
        let result = HierarchyChange { type_, len, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for HierarchyChange {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for HierarchyChange {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

/// Opcode for the XIChangeHierarchy request
pub const XI_CHANGE_HIERARCHY_REQUEST: u8 = 43;
pub fn xi_change_hierarchy<'c, Conn>(conn: &'c Conn, changes: &[HierarchyChange]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let changes_bytes = changes.serialize();
    let length: usize = (8 + changes_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let num_changes: u8 = changes.len().try_into()?;
    let num_changes_bytes = num_changes.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_CHANGE_HIERARCHY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        num_changes_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&changes_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&changes_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the XISetClientPointer request
pub const XI_SET_CLIENT_POINTER_REQUEST: u8 = 44;
pub fn xi_set_client_pointer<Conn>(conn: &Conn, window: xproto::Window, deviceid: DeviceId) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_SET_CLIENT_POINTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the XIGetClientPointer request
pub const XI_GET_CLIENT_POINTER_REQUEST: u8 = 45;
pub fn xi_get_client_pointer<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, XIGetClientPointerReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_GET_CLIENT_POINTER_REQUEST,
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
pub struct XIGetClientPointerReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub set: bool,
    pub deviceid: DeviceId,
}
impl XIGetClientPointerReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (set, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = XIGetClientPointerReply { response_type, sequence, length, set, deviceid };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIGetClientPointerReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum XIEventMask {
    DeviceChanged = 1 << 1,
    KeyPress = 1 << 2,
    KeyRelease = 1 << 3,
    ButtonPress = 1 << 4,
    ButtonRelease = 1 << 5,
    Motion = 1 << 6,
    Enter = 1 << 7,
    Leave = 1 << 8,
    FocusIn = 1 << 9,
    FocusOut = 1 << 10,
    Hierarchy = 1 << 11,
    Property = 1 << 12,
    RawKeyPress = 1 << 13,
    RawKeyRelease = 1 << 14,
    RawButtonPress = 1 << 15,
    RawButtonRelease = 1 << 16,
    RawMotion = 1 << 17,
    TouchBegin = 1 << 18,
    TouchUpdate = 1 << 19,
    TouchEnd = 1 << 20,
    TouchOwnership = 1 << 21,
    RawTouchBegin = 1 << 22,
    RawTouchUpdate = 1 << 23,
    RawTouchEnd = 1 << 24,
    BarrierHit = 1 << 25,
    BarrierLeave = 1 << 26,
}
impl From<XIEventMask> for u32 {
    fn from(input: XIEventMask) -> Self {
        match input {
            XIEventMask::DeviceChanged => 1 << 1,
            XIEventMask::KeyPress => 1 << 2,
            XIEventMask::KeyRelease => 1 << 3,
            XIEventMask::ButtonPress => 1 << 4,
            XIEventMask::ButtonRelease => 1 << 5,
            XIEventMask::Motion => 1 << 6,
            XIEventMask::Enter => 1 << 7,
            XIEventMask::Leave => 1 << 8,
            XIEventMask::FocusIn => 1 << 9,
            XIEventMask::FocusOut => 1 << 10,
            XIEventMask::Hierarchy => 1 << 11,
            XIEventMask::Property => 1 << 12,
            XIEventMask::RawKeyPress => 1 << 13,
            XIEventMask::RawKeyRelease => 1 << 14,
            XIEventMask::RawButtonPress => 1 << 15,
            XIEventMask::RawButtonRelease => 1 << 16,
            XIEventMask::RawMotion => 1 << 17,
            XIEventMask::TouchBegin => 1 << 18,
            XIEventMask::TouchUpdate => 1 << 19,
            XIEventMask::TouchEnd => 1 << 20,
            XIEventMask::TouchOwnership => 1 << 21,
            XIEventMask::RawTouchBegin => 1 << 22,
            XIEventMask::RawTouchUpdate => 1 << 23,
            XIEventMask::RawTouchEnd => 1 << 24,
            XIEventMask::BarrierHit => 1 << 25,
            XIEventMask::BarrierLeave => 1 << 26,
        }
    }
}
impl From<XIEventMask> for Option<u32> {
    fn from(input: XIEventMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for XIEventMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(XIEventMask::DeviceChanged),
            4 => Ok(XIEventMask::KeyPress),
            8 => Ok(XIEventMask::KeyRelease),
            16 => Ok(XIEventMask::ButtonPress),
            32 => Ok(XIEventMask::ButtonRelease),
            64 => Ok(XIEventMask::Motion),
            128 => Ok(XIEventMask::Enter),
            256 => Ok(XIEventMask::Leave),
            512 => Ok(XIEventMask::FocusIn),
            1024 => Ok(XIEventMask::FocusOut),
            2048 => Ok(XIEventMask::Hierarchy),
            4096 => Ok(XIEventMask::Property),
            8192 => Ok(XIEventMask::RawKeyPress),
            16384 => Ok(XIEventMask::RawKeyRelease),
            32768 => Ok(XIEventMask::RawButtonPress),
            65536 => Ok(XIEventMask::RawButtonRelease),
            131_072 => Ok(XIEventMask::RawMotion),
            262_144 => Ok(XIEventMask::TouchBegin),
            524_288 => Ok(XIEventMask::TouchUpdate),
            1_048_576 => Ok(XIEventMask::TouchEnd),
            2_097_152 => Ok(XIEventMask::TouchOwnership),
            4_194_304 => Ok(XIEventMask::RawTouchBegin),
            8_388_608 => Ok(XIEventMask::RawTouchUpdate),
            16_777_216 => Ok(XIEventMask::RawTouchEnd),
            33_554_432 => Ok(XIEventMask::BarrierHit),
            67_108_864 => Ok(XIEventMask::BarrierLeave),
            _ => Err(ParseError::ParseError)
        }
    }
}
bitmask_binop!(XIEventMask, u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventMask {
    pub deviceid: DeviceId,
    pub mask: Vec<u32>,
}
impl TryParse for EventMask {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (mask_len, remaining) = u16::try_parse(remaining)?;
        let (mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, mask_len as usize)?;
        let result = EventMask { deviceid, mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for EventMask {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for EventMask {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.deviceid.serialize_into(bytes);
        let mask_len = self.mask.len() as u16;
        mask_len.serialize_into(bytes);
        self.mask.serialize_into(bytes);
    }
}

/// Opcode for the XISelectEvents request
pub const XI_SELECT_EVENTS_REQUEST: u8 = 46;
pub fn xi_select_events<'c, Conn>(conn: &'c Conn, window: xproto::Window, masks: &[EventMask]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let masks_bytes = masks.serialize();
    let length: usize = (12 + masks_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let num_mask: u16 = masks.len().try_into()?;
    let num_mask_bytes = num_mask.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_SELECT_EVENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        num_mask_bytes[0],
        num_mask_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&masks_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&masks_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the XIQueryVersion request
pub const XI_QUERY_VERSION_REQUEST: u8 = 47;
pub fn xi_query_version<Conn>(conn: &Conn, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Conn, XIQueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let major_version_bytes = major_version.serialize();
    let minor_version_bytes = minor_version.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        major_version_bytes[0],
        major_version_bytes[1],
        minor_version_bytes[0],
        minor_version_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XIQueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl XIQueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = XIQueryVersionReply { response_type, sequence, length, major_version, minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIQueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceClassType {
    Key = 0,
    Button = 1,
    Valuator = 2,
    Scroll = 3,
    Touch = 8,
}
impl From<DeviceClassType> for u8 {
    fn from(input: DeviceClassType) -> Self {
        match input {
            DeviceClassType::Key => 0,
            DeviceClassType::Button => 1,
            DeviceClassType::Valuator => 2,
            DeviceClassType::Scroll => 3,
            DeviceClassType::Touch => 8,
        }
    }
}
impl From<DeviceClassType> for Option<u8> {
    fn from(input: DeviceClassType) -> Self {
        Some(u8::from(input))
    }
}
impl From<DeviceClassType> for u16 {
    fn from(input: DeviceClassType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceClassType> for Option<u16> {
    fn from(input: DeviceClassType) -> Self {
        Some(u16::from(input))
    }
}
impl From<DeviceClassType> for u32 {
    fn from(input: DeviceClassType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceClassType> for Option<u32> {
    fn from(input: DeviceClassType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DeviceClassType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeviceClassType::Key),
            1 => Ok(DeviceClassType::Button),
            2 => Ok(DeviceClassType::Valuator),
            3 => Ok(DeviceClassType::Scroll),
            8 => Ok(DeviceClassType::Touch),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for DeviceClassType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DeviceClassType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceType {
    MasterPointer = 1,
    MasterKeyboard = 2,
    SlavePointer = 3,
    SlaveKeyboard = 4,
    FloatingSlave = 5,
}
impl From<DeviceType> for u8 {
    fn from(input: DeviceType) -> Self {
        match input {
            DeviceType::MasterPointer => 1,
            DeviceType::MasterKeyboard => 2,
            DeviceType::SlavePointer => 3,
            DeviceType::SlaveKeyboard => 4,
            DeviceType::FloatingSlave => 5,
        }
    }
}
impl From<DeviceType> for Option<u8> {
    fn from(input: DeviceType) -> Self {
        Some(u8::from(input))
    }
}
impl From<DeviceType> for u16 {
    fn from(input: DeviceType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceType> for Option<u16> {
    fn from(input: DeviceType) -> Self {
        Some(u16::from(input))
    }
}
impl From<DeviceType> for u32 {
    fn from(input: DeviceType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceType> for Option<u32> {
    fn from(input: DeviceType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DeviceType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DeviceType::MasterPointer),
            2 => Ok(DeviceType::MasterKeyboard),
            3 => Ok(DeviceType::SlavePointer),
            4 => Ok(DeviceType::SlaveKeyboard),
            5 => Ok(DeviceType::FloatingSlave),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for DeviceType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DeviceType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollFlags {
    NoEmulation = 1 << 0,
    Preferred = 1 << 1,
}
impl From<ScrollFlags> for u8 {
    fn from(input: ScrollFlags) -> Self {
        match input {
            ScrollFlags::NoEmulation => 1 << 0,
            ScrollFlags::Preferred => 1 << 1,
        }
    }
}
impl From<ScrollFlags> for Option<u8> {
    fn from(input: ScrollFlags) -> Self {
        Some(u8::from(input))
    }
}
impl From<ScrollFlags> for u16 {
    fn from(input: ScrollFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ScrollFlags> for Option<u16> {
    fn from(input: ScrollFlags) -> Self {
        Some(u16::from(input))
    }
}
impl From<ScrollFlags> for u32 {
    fn from(input: ScrollFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ScrollFlags> for Option<u32> {
    fn from(input: ScrollFlags) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ScrollFlags {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ScrollFlags::NoEmulation),
            2 => Ok(ScrollFlags::Preferred),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ScrollFlags {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ScrollFlags {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ScrollFlags, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollType {
    Vertical = 1,
    Horizontal = 2,
}
impl From<ScrollType> for u8 {
    fn from(input: ScrollType) -> Self {
        match input {
            ScrollType::Vertical => 1,
            ScrollType::Horizontal => 2,
        }
    }
}
impl From<ScrollType> for Option<u8> {
    fn from(input: ScrollType) -> Self {
        Some(u8::from(input))
    }
}
impl From<ScrollType> for u16 {
    fn from(input: ScrollType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ScrollType> for Option<u16> {
    fn from(input: ScrollType) -> Self {
        Some(u16::from(input))
    }
}
impl From<ScrollType> for u32 {
    fn from(input: ScrollType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ScrollType> for Option<u32> {
    fn from(input: ScrollType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ScrollType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ScrollType::Vertical),
            2 => Ok(ScrollType::Horizontal),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ScrollType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ScrollType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TouchMode {
    Direct = 1,
    Dependent = 2,
}
impl From<TouchMode> for u8 {
    fn from(input: TouchMode) -> Self {
        match input {
            TouchMode::Direct => 1,
            TouchMode::Dependent => 2,
        }
    }
}
impl From<TouchMode> for Option<u8> {
    fn from(input: TouchMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<TouchMode> for u16 {
    fn from(input: TouchMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<TouchMode> for Option<u16> {
    fn from(input: TouchMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<TouchMode> for u32 {
    fn from(input: TouchMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<TouchMode> for Option<u32> {
    fn from(input: TouchMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for TouchMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TouchMode::Direct),
            2 => Ok(TouchMode::Dependent),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for TouchMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for TouchMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonClass {
    pub type_: DeviceClassType,
    pub len: u16,
    pub sourceid: DeviceId,
    pub num_buttons: u16,
    pub state: Vec<u32>,
    pub labels: Vec<xproto::Atom>,
}
impl TryParse for ButtonClass {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (num_buttons, remaining) = u16::try_parse(remaining)?;
        let (state, remaining) = crate::x11_utils::parse_list::<u32>(remaining, ((num_buttons as usize) + (31)) / (32))?;
        let (labels, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_buttons as usize)?;
        let type_ = type_.try_into()?;
        let result = ButtonClass { type_, len, sourceid, num_buttons, state, labels };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ButtonClass {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ButtonClass {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        self.num_buttons.serialize_into(bytes);
        self.state.serialize_into(bytes);
        self.labels.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyClass {
    pub type_: DeviceClassType,
    pub len: u16,
    pub sourceid: DeviceId,
    pub keys: Vec<u32>,
}
impl TryParse for KeyClass {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (num_keys, remaining) = u16::try_parse(remaining)?;
        let (keys, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_keys as usize)?;
        let type_ = type_.try_into()?;
        let result = KeyClass { type_, len, sourceid, keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyClass {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeyClass {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        let num_keys = self.keys.len() as u16;
        num_keys.serialize_into(bytes);
        self.keys.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollClass {
    pub type_: DeviceClassType,
    pub len: u16,
    pub sourceid: DeviceId,
    pub number: u16,
    pub scroll_type: ScrollType,
    pub flags: u32,
    pub increment: Fp3232,
}
impl TryParse for ScrollClass {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (number, remaining) = u16::try_parse(remaining)?;
        let (scroll_type, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (increment, remaining) = Fp3232::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let scroll_type = scroll_type.try_into()?;
        let result = ScrollClass { type_, len, sourceid, number, scroll_type, flags, increment };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ScrollClass {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ScrollClass {
    type Bytes = [u8; 24];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u16>::into(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let sourceid_bytes = self.sourceid.serialize();
        let number_bytes = self.number.serialize();
        let scroll_type_bytes = Into::<u16>::into(self.scroll_type).serialize();
        let flags_bytes = self.flags.serialize();
        let increment_bytes = self.increment.serialize();
        [
            type_bytes[0],
            type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            sourceid_bytes[0],
            sourceid_bytes[1],
            number_bytes[0],
            number_bytes[1],
            scroll_type_bytes[0],
            scroll_type_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            increment_bytes[0],
            increment_bytes[1],
            increment_bytes[2],
            increment_bytes[3],
            increment_bytes[4],
            increment_bytes[5],
            increment_bytes[6],
            increment_bytes[7],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        self.number.serialize_into(bytes);
        Into::<u16>::into(self.scroll_type).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
        self.increment.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TouchClass {
    pub type_: DeviceClassType,
    pub len: u16,
    pub sourceid: DeviceId,
    pub mode: TouchMode,
    pub num_touches: u8,
}
impl TryParse for TouchClass {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (num_touches, remaining) = u8::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let mode = mode.try_into()?;
        let result = TouchClass { type_, len, sourceid, mode, num_touches };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for TouchClass {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for TouchClass {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u16>::into(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let sourceid_bytes = self.sourceid.serialize();
        let mode_bytes = Into::<u8>::into(self.mode).serialize();
        let num_touches_bytes = self.num_touches.serialize();
        [
            type_bytes[0],
            type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            sourceid_bytes[0],
            sourceid_bytes[1],
            mode_bytes[0],
            num_touches_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        Into::<u8>::into(self.mode).serialize_into(bytes);
        self.num_touches.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValuatorClass {
    pub type_: DeviceClassType,
    pub len: u16,
    pub sourceid: DeviceId,
    pub number: u16,
    pub label: xproto::Atom,
    pub min: Fp3232,
    pub max: Fp3232,
    pub value: Fp3232,
    pub resolution: u32,
    pub mode: ValuatorMode,
}
impl TryParse for ValuatorClass {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (number, remaining) = u16::try_parse(remaining)?;
        let (label, remaining) = xproto::Atom::try_parse(remaining)?;
        let (min, remaining) = Fp3232::try_parse(remaining)?;
        let (max, remaining) = Fp3232::try_parse(remaining)?;
        let (value, remaining) = Fp3232::try_parse(remaining)?;
        let (resolution, remaining) = u32::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let mode = mode.try_into()?;
        let result = ValuatorClass { type_, len, sourceid, number, label, min, max, value, resolution, mode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ValuatorClass {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ValuatorClass {
    type Bytes = [u8; 44];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u16>::into(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let sourceid_bytes = self.sourceid.serialize();
        let number_bytes = self.number.serialize();
        let label_bytes = self.label.serialize();
        let min_bytes = self.min.serialize();
        let max_bytes = self.max.serialize();
        let value_bytes = self.value.serialize();
        let resolution_bytes = self.resolution.serialize();
        let mode_bytes = Into::<u8>::into(self.mode).serialize();
        [
            type_bytes[0],
            type_bytes[1],
            len_bytes[0],
            len_bytes[1],
            sourceid_bytes[0],
            sourceid_bytes[1],
            number_bytes[0],
            number_bytes[1],
            label_bytes[0],
            label_bytes[1],
            label_bytes[2],
            label_bytes[3],
            min_bytes[0],
            min_bytes[1],
            min_bytes[2],
            min_bytes[3],
            min_bytes[4],
            min_bytes[5],
            min_bytes[6],
            min_bytes[7],
            max_bytes[0],
            max_bytes[1],
            max_bytes[2],
            max_bytes[3],
            max_bytes[4],
            max_bytes[5],
            max_bytes[6],
            max_bytes[7],
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
            value_bytes[4],
            value_bytes[5],
            value_bytes[6],
            value_bytes[7],
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            mode_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(44);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        self.number.serialize_into(bytes);
        self.label.serialize_into(bytes);
        self.min.serialize_into(bytes);
        self.max.serialize_into(bytes);
        self.value.serialize_into(bytes);
        self.resolution.serialize_into(bytes);
        Into::<u8>::into(self.mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceClassDataKey {
    pub keys: Vec<u32>,
}
impl TryParse for DeviceClassDataKey {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_keys, remaining) = u16::try_parse(remaining)?;
        let (keys, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_keys as usize)?;
        let result = DeviceClassDataKey { keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceClassDataKey {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceClassDataKey {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        let num_keys = self.keys.len() as u16;
        num_keys.serialize_into(bytes);
        self.keys.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceClassDataButton {
    pub num_buttons: u16,
    pub state: Vec<u32>,
    pub labels: Vec<xproto::Atom>,
}
impl TryParse for DeviceClassDataButton {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_buttons, remaining) = u16::try_parse(remaining)?;
        let (state, remaining) = crate::x11_utils::parse_list::<u32>(remaining, ((num_buttons as usize) + (31)) / (32))?;
        let (labels, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_buttons as usize)?;
        let result = DeviceClassDataButton { num_buttons, state, labels };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceClassDataButton {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceClassDataButton {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.num_buttons.serialize_into(bytes);
        self.state.serialize_into(bytes);
        self.labels.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceClassDataValuator {
    pub number: u16,
    pub label: xproto::Atom,
    pub min: Fp3232,
    pub max: Fp3232,
    pub value: Fp3232,
    pub resolution: u32,
    pub mode: ValuatorMode,
}
impl TryParse for DeviceClassDataValuator {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (number, remaining) = u16::try_parse(remaining)?;
        let (label, remaining) = xproto::Atom::try_parse(remaining)?;
        let (min, remaining) = Fp3232::try_parse(remaining)?;
        let (max, remaining) = Fp3232::try_parse(remaining)?;
        let (value, remaining) = Fp3232::try_parse(remaining)?;
        let (resolution, remaining) = u32::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let mode = mode.try_into()?;
        let result = DeviceClassDataValuator { number, label, min, max, value, resolution, mode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceClassDataValuator {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceClassDataValuator {
    type Bytes = [u8; 38];
    fn serialize(&self) -> Self::Bytes {
        let number_bytes = self.number.serialize();
        let label_bytes = self.label.serialize();
        let min_bytes = self.min.serialize();
        let max_bytes = self.max.serialize();
        let value_bytes = self.value.serialize();
        let resolution_bytes = self.resolution.serialize();
        let mode_bytes = Into::<u8>::into(self.mode).serialize();
        [
            number_bytes[0],
            number_bytes[1],
            label_bytes[0],
            label_bytes[1],
            label_bytes[2],
            label_bytes[3],
            min_bytes[0],
            min_bytes[1],
            min_bytes[2],
            min_bytes[3],
            min_bytes[4],
            min_bytes[5],
            min_bytes[6],
            min_bytes[7],
            max_bytes[0],
            max_bytes[1],
            max_bytes[2],
            max_bytes[3],
            max_bytes[4],
            max_bytes[5],
            max_bytes[6],
            max_bytes[7],
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
            value_bytes[4],
            value_bytes[5],
            value_bytes[6],
            value_bytes[7],
            resolution_bytes[0],
            resolution_bytes[1],
            resolution_bytes[2],
            resolution_bytes[3],
            mode_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(38);
        self.number.serialize_into(bytes);
        self.label.serialize_into(bytes);
        self.min.serialize_into(bytes);
        self.max.serialize_into(bytes);
        self.value.serialize_into(bytes);
        self.resolution.serialize_into(bytes);
        Into::<u8>::into(self.mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceClassDataScroll {
    pub number: u16,
    pub scroll_type: ScrollType,
    pub flags: u32,
    pub increment: Fp3232,
}
impl TryParse for DeviceClassDataScroll {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (number, remaining) = u16::try_parse(remaining)?;
        let (scroll_type, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (increment, remaining) = Fp3232::try_parse(remaining)?;
        let scroll_type = scroll_type.try_into()?;
        let result = DeviceClassDataScroll { number, scroll_type, flags, increment };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceClassDataScroll {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceClassDataScroll {
    type Bytes = [u8; 18];
    fn serialize(&self) -> Self::Bytes {
        let number_bytes = self.number.serialize();
        let scroll_type_bytes = Into::<u16>::into(self.scroll_type).serialize();
        let flags_bytes = self.flags.serialize();
        let increment_bytes = self.increment.serialize();
        [
            number_bytes[0],
            number_bytes[1],
            scroll_type_bytes[0],
            scroll_type_bytes[1],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            increment_bytes[0],
            increment_bytes[1],
            increment_bytes[2],
            increment_bytes[3],
            increment_bytes[4],
            increment_bytes[5],
            increment_bytes[6],
            increment_bytes[7],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(18);
        self.number.serialize_into(bytes);
        Into::<u16>::into(self.scroll_type).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
        self.increment.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceClassDataTouch {
    pub mode: TouchMode,
    pub num_touches: u8,
}
impl TryParse for DeviceClassDataTouch {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (num_touches, remaining) = u8::try_parse(remaining)?;
        let mode = mode.try_into()?;
        let result = DeviceClassDataTouch { mode, num_touches };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceClassDataTouch {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceClassDataTouch {
    type Bytes = [u8; 2];
    fn serialize(&self) -> Self::Bytes {
        let mode_bytes = Into::<u8>::into(self.mode).serialize();
        let num_touches_bytes = self.num_touches.serialize();
        [
            mode_bytes[0],
            num_touches_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        Into::<u8>::into(self.mode).serialize_into(bytes);
        self.num_touches.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceClassData {
    Key(DeviceClassDataKey),
    Button(DeviceClassDataButton),
    Valuator(DeviceClassDataValuator),
    Scroll(DeviceClassDataScroll),
    Touch(DeviceClassDataTouch),
}
impl DeviceClassData {
    fn try_parse(value: &[u8], type_: u16) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if type_ == Into::<u16>::into(DeviceClassType::Key) {
            let (key, new_remaining) = DeviceClassDataKey::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Key(key));
        }
        if type_ == Into::<u16>::into(DeviceClassType::Button) {
            let (button, new_remaining) = DeviceClassDataButton::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Button(button));
        }
        if type_ == Into::<u16>::into(DeviceClassType::Valuator) {
            let (valuator, new_remaining) = DeviceClassDataValuator::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Valuator(valuator));
        }
        if type_ == Into::<u16>::into(DeviceClassType::Scroll) {
            let (scroll, new_remaining) = DeviceClassDataScroll::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Scroll(scroll));
        }
        if type_ == Into::<u16>::into(DeviceClassType::Touch) {
            let (touch, new_remaining) = DeviceClassDataTouch::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Touch(touch));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_key(&self) -> Option<&DeviceClassDataKey> {
        match self {
            DeviceClassData::Key(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_button(&self) -> Option<&DeviceClassDataButton> {
        match self {
            DeviceClassData::Button(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_valuator(&self) -> Option<&DeviceClassDataValuator> {
        match self {
            DeviceClassData::Valuator(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_scroll(&self) -> Option<&DeviceClassDataScroll> {
        match self {
            DeviceClassData::Scroll(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_touch(&self) -> Option<&DeviceClassDataTouch> {
        match self {
            DeviceClassData::Touch(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for DeviceClassData {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            DeviceClassData::Key(value) => value.serialize(),
            DeviceClassData::Button(value) => value.serialize(),
            DeviceClassData::Valuator(value) => value.serialize().to_vec(),
            DeviceClassData::Scroll(value) => value.serialize().to_vec(),
            DeviceClassData::Touch(value) => value.serialize().to_vec(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            DeviceClassData::Key(value) => value.serialize_into(bytes),
            DeviceClassData::Button(value) => value.serialize_into(bytes),
            DeviceClassData::Valuator(value) => value.serialize_into(bytes),
            DeviceClassData::Scroll(value) => value.serialize_into(bytes),
            DeviceClassData::Touch(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceClass {
    pub type_: DeviceClassType,
    pub len: u16,
    pub sourceid: DeviceId,
    pub data: DeviceClassData,
}
impl TryParse for DeviceClass {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (data, remaining) = DeviceClassData::try_parse(remaining, type_)?;
        let type_ = type_.try_into()?;
        let result = DeviceClass { type_, len, sourceid, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceClass {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceClass {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(6);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XIDeviceInfo {
    pub deviceid: DeviceId,
    pub type_: DeviceType,
    pub attachment: DeviceId,
    pub enabled: bool,
    pub name: Vec<u8>,
    pub classes: Vec<DeviceClass>,
}
impl TryParse for XIDeviceInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (attachment, remaining) = DeviceId::try_parse(remaining)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (enabled, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<DeviceClass>(remaining, num_classes as usize)?;
        let type_ = type_.try_into()?;
        let result = XIDeviceInfo { deviceid, type_, attachment, enabled, name, classes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIDeviceInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for XIDeviceInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.deviceid.serialize_into(bytes);
        Into::<u16>::into(self.type_).serialize_into(bytes);
        self.attachment.serialize_into(bytes);
        let num_classes = self.classes.len() as u16;
        num_classes.serialize_into(bytes);
        let name_len = self.name.len() as u16;
        name_len.serialize_into(bytes);
        self.enabled.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.classes.serialize_into(bytes);
    }
}

/// Opcode for the XIQueryDevice request
pub const XI_QUERY_DEVICE_REQUEST: u8 = 48;
pub fn xi_query_device<Conn>(conn: &Conn, deviceid: DeviceId) -> Result<Cookie<'_, Conn, XIQueryDeviceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_QUERY_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XIQueryDeviceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub infos: Vec<XIDeviceInfo>,
}
impl XIQueryDeviceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_infos, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (infos, remaining) = crate::x11_utils::parse_list::<XIDeviceInfo>(remaining, num_infos as usize)?;
        let result = XIQueryDeviceReply { response_type, sequence, length, infos };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIQueryDeviceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the XISetFocus request
pub const XI_SET_FOCUS_REQUEST: u8 = 49;
pub fn xi_set_focus<Conn>(conn: &Conn, window: xproto::Window, time: xproto::Timestamp, deviceid: DeviceId) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let time_bytes = time.serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_SET_FOCUS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the XIGetFocus request
pub const XI_GET_FOCUS_REQUEST: u8 = 50;
pub fn xi_get_focus<Conn>(conn: &Conn, deviceid: DeviceId) -> Result<Cookie<'_, Conn, XIGetFocusReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_GET_FOCUS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XIGetFocusReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xproto::Window,
}
impl XIGetFocusReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (focus, remaining) = xproto::Window::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = XIGetFocusReply { response_type, sequence, length, focus };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIGetFocusReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GrabOwner {
    NoOwner = 0,
    Owner = 1,
}
impl From<GrabOwner> for u8 {
    fn from(input: GrabOwner) -> Self {
        match input {
            GrabOwner::NoOwner => 0,
            GrabOwner::Owner => 1,
        }
    }
}
impl From<GrabOwner> for Option<u8> {
    fn from(input: GrabOwner) -> Self {
        Some(u8::from(input))
    }
}
impl From<GrabOwner> for u16 {
    fn from(input: GrabOwner) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabOwner> for Option<u16> {
    fn from(input: GrabOwner) -> Self {
        Some(u16::from(input))
    }
}
impl From<GrabOwner> for u32 {
    fn from(input: GrabOwner) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabOwner> for Option<u32> {
    fn from(input: GrabOwner) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GrabOwner {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GrabOwner::NoOwner),
            1 => Ok(GrabOwner::Owner),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for GrabOwner {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GrabOwner {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the XIGrabDevice request
pub const XI_GRAB_DEVICE_REQUEST: u8 = 51;
pub fn xi_grab_device<'c, Conn, A, B, C>(conn: &'c Conn, window: xproto::Window, time: xproto::Timestamp, cursor: xproto::Cursor, deviceid: DeviceId, mode: A, paired_device_mode: B, owner_events: C, mask: &[u32]) -> Result<Cookie<'c, Conn, XIGrabDeviceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>, C: Into<bool>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24 + 4 * mask.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let time_bytes = time.serialize();
    let cursor_bytes = cursor.serialize();
    let deviceid_bytes = deviceid.serialize();
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let paired_device_mode = paired_device_mode.into();
    let paired_device_mode_bytes = paired_device_mode.serialize();
    let owner_events = owner_events.into();
    let owner_events_bytes = (owner_events as u8).serialize();
    let mask_len: u16 = mask.len().try_into()?;
    let mask_len_bytes = mask_len.serialize();
    let mask_bytes = mask.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_GRAB_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        mode_bytes[0],
        paired_device_mode_bytes[0],
        owner_events_bytes[0],
        0,
        mask_len_bytes[0],
        mask_len_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&mask_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&mask_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XIGrabDeviceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl XIGrabDeviceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = XIGrabDeviceReply { response_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIGrabDeviceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the XIUngrabDevice request
pub const XI_UNGRAB_DEVICE_REQUEST: u8 = 52;
pub fn xi_ungrab_device<Conn>(conn: &Conn, time: xproto::Timestamp, deviceid: DeviceId) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let time_bytes = time.serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_UNGRAB_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventMode {
    AsyncDevice = 0,
    SyncDevice = 1,
    ReplayDevice = 2,
    AsyncPairedDevice = 3,
    AsyncPair = 4,
    SyncPair = 5,
    AcceptTouch = 6,
    RejectTouch = 7,
}
impl From<EventMode> for u8 {
    fn from(input: EventMode) -> Self {
        match input {
            EventMode::AsyncDevice => 0,
            EventMode::SyncDevice => 1,
            EventMode::ReplayDevice => 2,
            EventMode::AsyncPairedDevice => 3,
            EventMode::AsyncPair => 4,
            EventMode::SyncPair => 5,
            EventMode::AcceptTouch => 6,
            EventMode::RejectTouch => 7,
        }
    }
}
impl From<EventMode> for Option<u8> {
    fn from(input: EventMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<EventMode> for u16 {
    fn from(input: EventMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EventMode> for Option<u16> {
    fn from(input: EventMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<EventMode> for u32 {
    fn from(input: EventMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EventMode> for Option<u32> {
    fn from(input: EventMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for EventMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventMode::AsyncDevice),
            1 => Ok(EventMode::SyncDevice),
            2 => Ok(EventMode::ReplayDevice),
            3 => Ok(EventMode::AsyncPairedDevice),
            4 => Ok(EventMode::AsyncPair),
            5 => Ok(EventMode::SyncPair),
            6 => Ok(EventMode::AcceptTouch),
            7 => Ok(EventMode::RejectTouch),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for EventMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for EventMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the XIAllowEvents request
pub const XI_ALLOW_EVENTS_REQUEST: u8 = 53;
pub fn xi_allow_events<Conn, A>(conn: &Conn, time: xproto::Timestamp, deviceid: DeviceId, event_mode: A, touchid: u32, grab_window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let time_bytes = time.serialize();
    let deviceid_bytes = deviceid.serialize();
    let event_mode = event_mode.into();
    let event_mode_bytes = event_mode.serialize();
    let touchid_bytes = touchid.serialize();
    let grab_window_bytes = grab_window.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_ALLOW_EVENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        event_mode_bytes[0],
        0,
        touchid_bytes[0],
        touchid_bytes[1],
        touchid_bytes[2],
        touchid_bytes[3],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GrabMode22 {
    Sync = 0,
    Async = 1,
    Touch = 2,
}
impl From<GrabMode22> for u8 {
    fn from(input: GrabMode22) -> Self {
        match input {
            GrabMode22::Sync => 0,
            GrabMode22::Async => 1,
            GrabMode22::Touch => 2,
        }
    }
}
impl From<GrabMode22> for Option<u8> {
    fn from(input: GrabMode22) -> Self {
        Some(u8::from(input))
    }
}
impl From<GrabMode22> for u16 {
    fn from(input: GrabMode22) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabMode22> for Option<u16> {
    fn from(input: GrabMode22) -> Self {
        Some(u16::from(input))
    }
}
impl From<GrabMode22> for u32 {
    fn from(input: GrabMode22) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabMode22> for Option<u32> {
    fn from(input: GrabMode22) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GrabMode22 {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GrabMode22::Sync),
            1 => Ok(GrabMode22::Async),
            2 => Ok(GrabMode22::Touch),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for GrabMode22 {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GrabMode22 {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GrabType {
    Button = 0,
    Keycode = 1,
    Enter = 2,
    FocusIn = 3,
    TouchBegin = 4,
}
impl From<GrabType> for u8 {
    fn from(input: GrabType) -> Self {
        match input {
            GrabType::Button => 0,
            GrabType::Keycode => 1,
            GrabType::Enter => 2,
            GrabType::FocusIn => 3,
            GrabType::TouchBegin => 4,
        }
    }
}
impl From<GrabType> for Option<u8> {
    fn from(input: GrabType) -> Self {
        Some(u8::from(input))
    }
}
impl From<GrabType> for u16 {
    fn from(input: GrabType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabType> for Option<u16> {
    fn from(input: GrabType) -> Self {
        Some(u16::from(input))
    }
}
impl From<GrabType> for u32 {
    fn from(input: GrabType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabType> for Option<u32> {
    fn from(input: GrabType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GrabType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GrabType::Button),
            1 => Ok(GrabType::Keycode),
            2 => Ok(GrabType::Enter),
            3 => Ok(GrabType::FocusIn),
            4 => Ok(GrabType::TouchBegin),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for GrabType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GrabType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ModifierMask {
    Any = 1 << 31,
}
impl From<ModifierMask> for u32 {
    fn from(input: ModifierMask) -> Self {
        match input {
            ModifierMask::Any => 1 << 31,
        }
    }
}
impl From<ModifierMask> for Option<u32> {
    fn from(input: ModifierMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for ModifierMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2_147_483_648 => Ok(ModifierMask::Any),
            _ => Err(ParseError::ParseError)
        }
    }
}
bitmask_binop!(ModifierMask, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabModifierInfo {
    pub modifiers: u32,
    pub status: xproto::GrabStatus,
}
impl TryParse for GrabModifierInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (modifiers, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let status = status.try_into()?;
        let result = GrabModifierInfo { modifiers, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GrabModifierInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for GrabModifierInfo {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let modifiers_bytes = self.modifiers.serialize();
        let status_bytes = Into::<u8>::into(self.status).serialize();
        [
            modifiers_bytes[0],
            modifiers_bytes[1],
            modifiers_bytes[2],
            modifiers_bytes[3],
            status_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.modifiers.serialize_into(bytes);
        Into::<u8>::into(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}

/// Opcode for the XIPassiveGrabDevice request
pub const XI_PASSIVE_GRAB_DEVICE_REQUEST: u8 = 54;
pub fn xi_passive_grab_device<'c, Conn, A, B, C, D>(conn: &'c Conn, time: xproto::Timestamp, grab_window: xproto::Window, cursor: xproto::Cursor, detail: u32, deviceid: DeviceId, grab_type: A, grab_mode: B, paired_device_mode: C, owner_events: D, mask: &[u32], modifiers: &[u32]) -> Result<Cookie<'c, Conn, XIPassiveGrabDeviceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>, C: Into<u8>, D: Into<bool>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32 + 4 * mask.len() + 4 * modifiers.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let time_bytes = time.serialize();
    let grab_window_bytes = grab_window.serialize();
    let cursor_bytes = cursor.serialize();
    let detail_bytes = detail.serialize();
    let deviceid_bytes = deviceid.serialize();
    let num_modifiers: u16 = modifiers.len().try_into()?;
    let num_modifiers_bytes = num_modifiers.serialize();
    let mask_len: u16 = mask.len().try_into()?;
    let mask_len_bytes = mask_len.serialize();
    let grab_type = grab_type.into();
    let grab_type_bytes = grab_type.serialize();
    let grab_mode = grab_mode.into();
    let grab_mode_bytes = grab_mode.serialize();
    let paired_device_mode = paired_device_mode.into();
    let paired_device_mode_bytes = paired_device_mode.serialize();
    let owner_events = owner_events.into();
    let owner_events_bytes = (owner_events as u8).serialize();
    let mask_bytes = mask.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_PASSIVE_GRAB_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
        detail_bytes[0],
        detail_bytes[1],
        detail_bytes[2],
        detail_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        num_modifiers_bytes[0],
        num_modifiers_bytes[1],
        mask_len_bytes[0],
        mask_len_bytes[1],
        grab_type_bytes[0],
        grab_mode_bytes[0],
        paired_device_mode_bytes[0],
        owner_events_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&mask_bytes).len();
    let modifiers_bytes = modifiers.serialize();
    let length_so_far = length_so_far + (&modifiers_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&mask_bytes), IoSlice::new(&modifiers_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XIPassiveGrabDeviceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub modifiers: Vec<GrabModifierInfo>,
}
impl XIPassiveGrabDeviceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_modifiers, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (modifiers, remaining) = crate::x11_utils::parse_list::<GrabModifierInfo>(remaining, num_modifiers as usize)?;
        let result = XIPassiveGrabDeviceReply { response_type, sequence, length, modifiers };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIPassiveGrabDeviceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the XIPassiveUngrabDevice request
pub const XI_PASSIVE_UNGRAB_DEVICE_REQUEST: u8 = 55;
pub fn xi_passive_ungrab_device<'c, Conn, A>(conn: &'c Conn, grab_window: xproto::Window, detail: u32, deviceid: DeviceId, grab_type: A, modifiers: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20 + 4 * modifiers.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let detail_bytes = detail.serialize();
    let deviceid_bytes = deviceid.serialize();
    let num_modifiers: u16 = modifiers.len().try_into()?;
    let num_modifiers_bytes = num_modifiers.serialize();
    let grab_type = grab_type.into();
    let grab_type_bytes = grab_type.serialize();
    let modifiers_bytes = modifiers.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_PASSIVE_UNGRAB_DEVICE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        detail_bytes[0],
        detail_bytes[1],
        detail_bytes[2],
        detail_bytes[3],
        deviceid_bytes[0],
        deviceid_bytes[1],
        num_modifiers_bytes[0],
        num_modifiers_bytes[1],
        grab_type_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&modifiers_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&modifiers_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the XIListProperties request
pub const XI_LIST_PROPERTIES_REQUEST: u8 = 56;
pub fn xi_list_properties<Conn>(conn: &Conn, deviceid: DeviceId) -> Result<Cookie<'_, Conn, XIListPropertiesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let deviceid_bytes = deviceid.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_LIST_PROPERTIES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XIListPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub properties: Vec<xproto::Atom>,
}
impl XIListPropertiesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_properties, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (properties, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_properties as usize)?;
        let result = XIListPropertiesReply { response_type, sequence, length, properties };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIListPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the XIChangeProperty request
pub const XI_CHANGE_PROPERTY_REQUEST: u8 = 57;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XIChangePropertyAux {
    Data8(Vec<u8>),
    Data16(Vec<u16>),
    Data32(Vec<u32>),
}
impl Serialize for XIChangePropertyAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            XIChangePropertyAux::Data8(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3][..(4 - (result.len() % 4)) % 4]);
                result
            }
            XIChangePropertyAux::Data16(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3][..(4 - (result.len() % 4)) % 4]);
                result
            }
            XIChangePropertyAux::Data32(value) => value.serialize(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            XIChangePropertyAux::Data8(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            XIChangePropertyAux::Data16(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            XIChangePropertyAux::Data32(value) => value.serialize_into(bytes),
        }
    }
}
impl XIChangePropertyAux {
    fn format(&self) -> u8 {
        match self {
            XIChangePropertyAux::Data8(_) => PropertyFormat::M8Bits.into(),
            XIChangePropertyAux::Data16(_) => PropertyFormat::M16Bits.into(),
            XIChangePropertyAux::Data32(_) => PropertyFormat::M32Bits.into(),
        }
    }
}
pub fn xi_change_property<'c, Conn, A>(conn: &'c Conn, deviceid: DeviceId, mode: A, property: xproto::Atom, type_: xproto::Atom, num_items: u32, items: &XIChangePropertyAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let format = items.format();
    let items_bytes = items.serialize();
    let length: usize = (20 + items_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let deviceid_bytes = deviceid.serialize();
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let format_bytes = format.serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let num_items_bytes = num_items.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_CHANGE_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        deviceid_bytes[0],
        deviceid_bytes[1],
        mode_bytes[0],
        format_bytes[0],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        num_items_bytes[0],
        num_items_bytes[1],
        num_items_bytes[2],
        num_items_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&items_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&items_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the XIDeleteProperty request
pub const XI_DELETE_PROPERTY_REQUEST: u8 = 58;
pub fn xi_delete_property<Conn>(conn: &Conn, deviceid: DeviceId, property: xproto::Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let deviceid_bytes = deviceid.serialize();
    let property_bytes = property.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_DELETE_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        deviceid_bytes[0],
        deviceid_bytes[1],
        0,
        0,
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the XIGetProperty request
pub const XI_GET_PROPERTY_REQUEST: u8 = 59;
pub fn xi_get_property<Conn>(conn: &Conn, deviceid: DeviceId, delete: bool, property: xproto::Atom, type_: xproto::Atom, offset: u32, len: u32) -> Result<Cookie<'_, Conn, XIGetPropertyReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let deviceid_bytes = deviceid.serialize();
    let delete_bytes = (delete as u8).serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let offset_bytes = offset.serialize();
    let len_bytes = len.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_GET_PROPERTY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        deviceid_bytes[0],
        deviceid_bytes[1],
        delete_bytes[0],
        0,
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        offset_bytes[0],
        offset_bytes[1],
        offset_bytes[2],
        offset_bytes[3],
        len_bytes[0],
        len_bytes[1],
        len_bytes[2],
        len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XIGetPropertyItems {
    Data8(Vec<u8>),
    Data16(Vec<u16>),
    Data32(Vec<u32>),
}
impl XIGetPropertyItems {
    fn try_parse(value: &[u8], format: u8, num_items: u32) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let mut parse_result = None;
        if format == Into::<u8>::into(PropertyFormat::M8Bits) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data8, remaining) = crate::x11_utils::parse_list::<u8>(remaining, num_items as usize)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIGetPropertyItems::Data8(data8));
        }
        if format == Into::<u8>::into(PropertyFormat::M16Bits) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data16, remaining) = crate::x11_utils::parse_list::<u16>(remaining, num_items as usize)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIGetPropertyItems::Data16(data16));
        }
        if format == Into::<u8>::into(PropertyFormat::M32Bits) {
            let remaining = outer_remaining;
            let (data32, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_items as usize)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIGetPropertyItems::Data32(data32));
        }
        match parse_result {
            None => Err(ParseError::ParseError),
            Some(result) => Ok((result, outer_remaining))
        }
    }
    pub fn as_data8(&self) -> Option<&Vec<u8>> {
        match self {
            XIGetPropertyItems::Data8(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_data16(&self) -> Option<&Vec<u16>> {
        match self {
            XIGetPropertyItems::Data16(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_data32(&self) -> Option<&Vec<u32>> {
        match self {
            XIGetPropertyItems::Data32(value) => Some(value),
            _ => None,
        }
    }
}
impl Serialize for XIGetPropertyItems {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            XIGetPropertyItems::Data8(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3][..(4 - (result.len() % 4)) % 4]);
                result
            }
            XIGetPropertyItems::Data16(value) => {
                let mut result = Vec::new();
                value.serialize_into(&mut result);
                result.extend_from_slice(&[0; 3][..(4 - (result.len() % 4)) % 4]);
                result
            }
            XIGetPropertyItems::Data32(value) => value.serialize(),
        }
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        match self {
            XIGetPropertyItems::Data8(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            XIGetPropertyItems::Data16(value) => {
                value.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            XIGetPropertyItems::Data32(value) => value.serialize_into(bytes),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XIGetPropertyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xproto::Atom,
    pub bytes_after: u32,
    pub num_items: u32,
    pub format: PropertyFormat,
    pub items: XIGetPropertyItems,
}
impl XIGetPropertyReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::ParseError)?;
        let (items, remaining) = XIGetPropertyItems::try_parse(remaining, format, num_items)?;
        let format = format.try_into()?;
        let result = XIGetPropertyReply { response_type, sequence, length, type_, bytes_after, num_items, format, items };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIGetPropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the XIGetSelectedEvents request
pub const XI_GET_SELECTED_EVENTS_REQUEST: u8 = 60;
pub fn xi_get_selected_events<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, XIGetSelectedEventsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_GET_SELECTED_EVENTS_REQUEST,
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
pub struct XIGetSelectedEventsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub masks: Vec<EventMask>,
}
impl XIGetSelectedEventsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_masks, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (masks, remaining) = crate::x11_utils::parse_list::<EventMask>(remaining, num_masks as usize)?;
        let result = XIGetSelectedEventsReply { response_type, sequence, length, masks };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for XIGetSelectedEventsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarrierReleasePointerInfo {
    pub deviceid: DeviceId,
    pub barrier: xfixes::Barrier,
    pub eventid: u32,
}
impl TryParse for BarrierReleasePointerInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (barrier, remaining) = xfixes::Barrier::try_parse(remaining)?;
        let (eventid, remaining) = u32::try_parse(remaining)?;
        let result = BarrierReleasePointerInfo { deviceid, barrier, eventid };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BarrierReleasePointerInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for BarrierReleasePointerInfo {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let deviceid_bytes = self.deviceid.serialize();
        let barrier_bytes = self.barrier.serialize();
        let eventid_bytes = self.eventid.serialize();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
            barrier_bytes[0],
            barrier_bytes[1],
            barrier_bytes[2],
            barrier_bytes[3],
            eventid_bytes[0],
            eventid_bytes[1],
            eventid_bytes[2],
            eventid_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.deviceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.barrier.serialize_into(bytes);
        self.eventid.serialize_into(bytes);
    }
}

/// Opcode for the XIBarrierReleasePointer request
pub const XI_BARRIER_RELEASE_POINTER_REQUEST: u8 = 61;
pub fn xi_barrier_release_pointer<'c, Conn>(conn: &'c Conn, barriers: &[BarrierReleasePointerInfo]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 12 * barriers.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let num_barriers: u32 = barriers.len().try_into()?;
    let num_barriers_bytes = num_barriers.serialize();
    let barriers_bytes = barriers.serialize();
    let request0 = [
        extension_information.major_opcode,
        XI_BARRIER_RELEASE_POINTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        num_barriers_bytes[0],
        num_barriers_bytes[1],
        num_barriers_bytes[2],
        num_barriers_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&barriers_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&barriers_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeviceValuator event
pub const DEVICE_VALUATOR_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceValuatorEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub device_state: u16,
    pub num_valuators: u8,
    pub first_valuator: u8,
    pub valuators: [i32; 6],
}
impl DeviceValuatorEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (device_state, remaining) = u16::try_parse(remaining)?;
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let (first_valuator, remaining) = u8::try_parse(remaining)?;
        let (valuators_0, remaining) = i32::try_parse(remaining)?;
        let (valuators_1, remaining) = i32::try_parse(remaining)?;
        let (valuators_2, remaining) = i32::try_parse(remaining)?;
        let (valuators_3, remaining) = i32::try_parse(remaining)?;
        let (valuators_4, remaining) = i32::try_parse(remaining)?;
        let (valuators_5, remaining) = i32::try_parse(remaining)?;
        let valuators = [
            valuators_0,
            valuators_1,
            valuators_2,
            valuators_3,
            valuators_4,
            valuators_5,
        ];
        let result = DeviceValuatorEvent { response_type, device_id, sequence, device_state, num_valuators, first_valuator, valuators };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceValuatorEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceValuatorEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceValuatorEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceValuatorEvent> for [u8; 32] {
    fn from(input: &DeviceValuatorEvent) -> Self {
        let response_type = input.response_type.serialize();
        let device_id = input.device_id.serialize();
        let sequence = input.sequence.serialize();
        let device_state = input.device_state.serialize();
        let num_valuators = input.num_valuators.serialize();
        let first_valuator = input.first_valuator.serialize();
        let valuators_0 = input.valuators[0].serialize();
        let valuators_1 = input.valuators[1].serialize();
        let valuators_2 = input.valuators[2].serialize();
        let valuators_3 = input.valuators[3].serialize();
        let valuators_4 = input.valuators[4].serialize();
        let valuators_5 = input.valuators[5].serialize();
        [
            response_type[0], device_id[0], sequence[0], sequence[1], device_state[0], device_state[1], num_valuators[0], first_valuator[0],
            valuators_0[0], valuators_0[1], valuators_0[2], valuators_0[3], valuators_1[0], valuators_1[1], valuators_1[2], valuators_1[3],
            valuators_2[0], valuators_2[1], valuators_2[2], valuators_2[3], valuators_3[0], valuators_3[1], valuators_3[2], valuators_3[3],
            valuators_4[0], valuators_4[1], valuators_4[2], valuators_4[3], valuators_5[0], valuators_5[1], valuators_5[2], valuators_5[3]
        ]
    }
}
impl From<DeviceValuatorEvent> for [u8; 32] {
    fn from(input: DeviceValuatorEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MoreEventsMask {
    MoreEvents = 1 << 7,
}
impl From<MoreEventsMask> for u8 {
    fn from(input: MoreEventsMask) -> Self {
        match input {
            MoreEventsMask::MoreEvents => 1 << 7,
        }
    }
}
impl From<MoreEventsMask> for Option<u8> {
    fn from(input: MoreEventsMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<MoreEventsMask> for u16 {
    fn from(input: MoreEventsMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MoreEventsMask> for Option<u16> {
    fn from(input: MoreEventsMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<MoreEventsMask> for u32 {
    fn from(input: MoreEventsMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MoreEventsMask> for Option<u32> {
    fn from(input: MoreEventsMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for MoreEventsMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(MoreEventsMask::MoreEvents),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for MoreEventsMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for MoreEventsMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(MoreEventsMask, u8);

/// Opcode for the DeviceKeyPress event
pub const DEVICE_KEY_PRESS_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceKeyPressEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
    pub device_id: u8,
}
impl DeviceKeyPressEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let result = DeviceKeyPressEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceKeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceKeyPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceKeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceKeyPressEvent> for [u8; 32] {
    fn from(input: &DeviceKeyPressEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], device_id[0]
        ]
    }
}
impl From<DeviceKeyPressEvent> for [u8; 32] {
    fn from(input: DeviceKeyPressEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceKeyRelease event
pub const DEVICE_KEY_RELEASE_EVENT: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceKeyReleaseEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
    pub device_id: u8,
}
impl DeviceKeyReleaseEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let result = DeviceKeyReleaseEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceKeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceKeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceKeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceKeyReleaseEvent> for [u8; 32] {
    fn from(input: &DeviceKeyReleaseEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], device_id[0]
        ]
    }
}
impl From<DeviceKeyReleaseEvent> for [u8; 32] {
    fn from(input: DeviceKeyReleaseEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceButtonPress event
pub const DEVICE_BUTTON_PRESS_EVENT: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceButtonPressEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
    pub device_id: u8,
}
impl DeviceButtonPressEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let result = DeviceButtonPressEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceButtonPressEvent> for [u8; 32] {
    fn from(input: &DeviceButtonPressEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], device_id[0]
        ]
    }
}
impl From<DeviceButtonPressEvent> for [u8; 32] {
    fn from(input: DeviceButtonPressEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceButtonRelease event
pub const DEVICE_BUTTON_RELEASE_EVENT: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceButtonReleaseEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
    pub device_id: u8,
}
impl DeviceButtonReleaseEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let result = DeviceButtonReleaseEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceButtonReleaseEvent> for [u8; 32] {
    fn from(input: &DeviceButtonReleaseEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], device_id[0]
        ]
    }
}
impl From<DeviceButtonReleaseEvent> for [u8; 32] {
    fn from(input: DeviceButtonReleaseEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceMotionNotify event
pub const DEVICE_MOTION_NOTIFY_EVENT: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceMotionNotifyEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
    pub device_id: u8,
}
impl DeviceMotionNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let result = DeviceMotionNotifyEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceMotionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceMotionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceMotionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceMotionNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceMotionNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], device_id[0]
        ]
    }
}
impl From<DeviceMotionNotifyEvent> for [u8; 32] {
    fn from(input: DeviceMotionNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceFocusIn event
pub const DEVICE_FOCUS_IN_EVENT: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceFocusInEvent {
    pub response_type: u8,
    pub detail: xproto::NotifyDetail,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub window: xproto::Window,
    pub mode: xproto::NotifyMode,
    pub device_id: u8,
}
impl DeviceFocusInEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let detail = detail.try_into()?;
        let mode = mode.try_into()?;
        let result = DeviceFocusInEvent { response_type, detail, sequence, time, window, mode, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceFocusInEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceFocusInEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceFocusInEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceFocusInEvent> for [u8; 32] {
    fn from(input: &DeviceFocusInEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = Into::<u8>::into(input.detail).serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let window = input.window.serialize();
        let mode = Into::<u8>::into(input.mode).serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            window[0], window[1], window[2], window[3], mode[0], device_id[0], 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<DeviceFocusInEvent> for [u8; 32] {
    fn from(input: DeviceFocusInEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceFocusOut event
pub const DEVICE_FOCUS_OUT_EVENT: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceFocusOutEvent {
    pub response_type: u8,
    pub detail: xproto::NotifyDetail,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub window: xproto::Window,
    pub mode: xproto::NotifyMode,
    pub device_id: u8,
}
impl DeviceFocusOutEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let detail = detail.try_into()?;
        let mode = mode.try_into()?;
        let result = DeviceFocusOutEvent { response_type, detail, sequence, time, window, mode, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceFocusOutEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceFocusOutEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceFocusOutEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceFocusOutEvent> for [u8; 32] {
    fn from(input: &DeviceFocusOutEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = Into::<u8>::into(input.detail).serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let window = input.window.serialize();
        let mode = Into::<u8>::into(input.mode).serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            window[0], window[1], window[2], window[3], mode[0], device_id[0], 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<DeviceFocusOutEvent> for [u8; 32] {
    fn from(input: DeviceFocusOutEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ProximityIn event
pub const PROXIMITY_IN_EVENT: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProximityInEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
    pub device_id: u8,
}
impl ProximityInEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let result = ProximityInEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ProximityInEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ProximityInEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ProximityInEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ProximityInEvent> for [u8; 32] {
    fn from(input: &ProximityInEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], device_id[0]
        ]
    }
}
impl From<ProximityInEvent> for [u8; 32] {
    fn from(input: ProximityInEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ProximityOut event
pub const PROXIMITY_OUT_EVENT: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProximityOutEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
    pub device_id: u8,
}
impl ProximityOutEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let result = ProximityOutEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ProximityOutEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ProximityOutEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ProximityOutEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ProximityOutEvent> for [u8; 32] {
    fn from(input: &ProximityOutEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], device_id[0]
        ]
    }
}
impl From<ProximityOutEvent> for [u8; 32] {
    fn from(input: ProximityOutEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClassesReportedMask {
    OutOfProximity = 1 << 7,
    DeviceModeAbsolute = 1 << 6,
    ReportingValuators = 1 << 2,
    ReportingButtons = 1 << 1,
    ReportingKeys = 1 << 0,
}
impl From<ClassesReportedMask> for u8 {
    fn from(input: ClassesReportedMask) -> Self {
        match input {
            ClassesReportedMask::OutOfProximity => 1 << 7,
            ClassesReportedMask::DeviceModeAbsolute => 1 << 6,
            ClassesReportedMask::ReportingValuators => 1 << 2,
            ClassesReportedMask::ReportingButtons => 1 << 1,
            ClassesReportedMask::ReportingKeys => 1 << 0,
        }
    }
}
impl From<ClassesReportedMask> for Option<u8> {
    fn from(input: ClassesReportedMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<ClassesReportedMask> for u16 {
    fn from(input: ClassesReportedMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ClassesReportedMask> for Option<u16> {
    fn from(input: ClassesReportedMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<ClassesReportedMask> for u32 {
    fn from(input: ClassesReportedMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ClassesReportedMask> for Option<u32> {
    fn from(input: ClassesReportedMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ClassesReportedMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(ClassesReportedMask::OutOfProximity),
            64 => Ok(ClassesReportedMask::DeviceModeAbsolute),
            4 => Ok(ClassesReportedMask::ReportingValuators),
            2 => Ok(ClassesReportedMask::ReportingButtons),
            1 => Ok(ClassesReportedMask::ReportingKeys),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ClassesReportedMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ClassesReportedMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ClassesReportedMask, u8);

/// Opcode for the DeviceStateNotify event
pub const DEVICE_STATE_NOTIFY_EVENT: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceStateNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub num_keys: u8,
    pub num_buttons: u8,
    pub num_valuators: u8,
    pub classes_reported: u8,
    pub buttons: [u8; 4],
    pub keys: [u8; 4],
    pub valuators: [u32; 3],
}
impl DeviceStateNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_keys, remaining) = u8::try_parse(remaining)?;
        let (num_buttons, remaining) = u8::try_parse(remaining)?;
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let (classes_reported, remaining) = u8::try_parse(remaining)?;
        let (buttons_0, remaining) = u8::try_parse(remaining)?;
        let (buttons_1, remaining) = u8::try_parse(remaining)?;
        let (buttons_2, remaining) = u8::try_parse(remaining)?;
        let (buttons_3, remaining) = u8::try_parse(remaining)?;
        let buttons = [
            buttons_0,
            buttons_1,
            buttons_2,
            buttons_3,
        ];
        let (keys_0, remaining) = u8::try_parse(remaining)?;
        let (keys_1, remaining) = u8::try_parse(remaining)?;
        let (keys_2, remaining) = u8::try_parse(remaining)?;
        let (keys_3, remaining) = u8::try_parse(remaining)?;
        let keys = [
            keys_0,
            keys_1,
            keys_2,
            keys_3,
        ];
        let (valuators_0, remaining) = u32::try_parse(remaining)?;
        let (valuators_1, remaining) = u32::try_parse(remaining)?;
        let (valuators_2, remaining) = u32::try_parse(remaining)?;
        let valuators = [
            valuators_0,
            valuators_1,
            valuators_2,
        ];
        let result = DeviceStateNotifyEvent { response_type, device_id, sequence, time, num_keys, num_buttons, num_valuators, classes_reported, buttons, keys, valuators };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceStateNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceStateNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let device_id = input.device_id.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let num_keys = input.num_keys.serialize();
        let num_buttons = input.num_buttons.serialize();
        let num_valuators = input.num_valuators.serialize();
        let classes_reported = input.classes_reported.serialize();
        let valuators_0 = input.valuators[0].serialize();
        let valuators_1 = input.valuators[1].serialize();
        let valuators_2 = input.valuators[2].serialize();
        [
            response_type[0], device_id[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            num_keys[0], num_buttons[0], num_valuators[0], classes_reported[0], input.buttons[0], input.buttons[1], input.buttons[2], input.buttons[3],
            input.keys[0], input.keys[1], input.keys[2], input.keys[3], valuators_0[0], valuators_0[1], valuators_0[2], valuators_0[3],
            valuators_1[0], valuators_1[1], valuators_1[2], valuators_1[3], valuators_2[0], valuators_2[1], valuators_2[2], valuators_2[3]
        ]
    }
}
impl From<DeviceStateNotifyEvent> for [u8; 32] {
    fn from(input: DeviceStateNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceMappingNotify event
pub const DEVICE_MAPPING_NOTIFY_EVENT: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceMappingNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub request: xproto::Mapping,
    pub first_keycode: KeyCode,
    pub count: u8,
    pub time: xproto::Timestamp,
}
impl DeviceMappingNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (request, remaining) = u8::try_parse(remaining)?;
        let (first_keycode, remaining) = KeyCode::try_parse(remaining)?;
        let (count, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let request = request.try_into()?;
        let result = DeviceMappingNotifyEvent { response_type, device_id, sequence, request, first_keycode, count, time };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceMappingNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceMappingNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceMappingNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceMappingNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceMappingNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let device_id = input.device_id.serialize();
        let sequence = input.sequence.serialize();
        let request = Into::<u8>::into(input.request).serialize();
        let first_keycode = input.first_keycode.serialize();
        let count = input.count.serialize();
        let time = input.time.serialize();
        [
            response_type[0], device_id[0], sequence[0], sequence[1], request[0], first_keycode[0], count[0], 0,
            time[0], time[1], time[2], time[3], 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<DeviceMappingNotifyEvent> for [u8; 32] {
    fn from(input: DeviceMappingNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChangeDevice {
    NewPointer = 0,
    NewKeyboard = 1,
}
impl From<ChangeDevice> for u8 {
    fn from(input: ChangeDevice) -> Self {
        match input {
            ChangeDevice::NewPointer => 0,
            ChangeDevice::NewKeyboard => 1,
        }
    }
}
impl From<ChangeDevice> for Option<u8> {
    fn from(input: ChangeDevice) -> Self {
        Some(u8::from(input))
    }
}
impl From<ChangeDevice> for u16 {
    fn from(input: ChangeDevice) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ChangeDevice> for Option<u16> {
    fn from(input: ChangeDevice) -> Self {
        Some(u16::from(input))
    }
}
impl From<ChangeDevice> for u32 {
    fn from(input: ChangeDevice) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ChangeDevice> for Option<u32> {
    fn from(input: ChangeDevice) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ChangeDevice {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChangeDevice::NewPointer),
            1 => Ok(ChangeDevice::NewKeyboard),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ChangeDevice {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ChangeDevice {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ChangeDeviceNotify event
pub const CHANGE_DEVICE_NOTIFY_EVENT: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeDeviceNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub request: ChangeDevice,
}
impl ChangeDeviceNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (request, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let request = request.try_into()?;
        let result = ChangeDeviceNotifyEvent { response_type, device_id, sequence, time, request };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ChangeDeviceNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ChangeDeviceNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ChangeDeviceNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ChangeDeviceNotifyEvent> for [u8; 32] {
    fn from(input: &ChangeDeviceNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let device_id = input.device_id.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let request = Into::<u8>::into(input.request).serialize();
        [
            response_type[0], device_id[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            request[0], 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ChangeDeviceNotifyEvent> for [u8; 32] {
    fn from(input: ChangeDeviceNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceKeyStateNotify event
pub const DEVICE_KEY_STATE_NOTIFY_EVENT: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceKeyStateNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub keys: [u8; 28],
}
impl DeviceKeyStateNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (keys_0, remaining) = u8::try_parse(remaining)?;
        let (keys_1, remaining) = u8::try_parse(remaining)?;
        let (keys_2, remaining) = u8::try_parse(remaining)?;
        let (keys_3, remaining) = u8::try_parse(remaining)?;
        let (keys_4, remaining) = u8::try_parse(remaining)?;
        let (keys_5, remaining) = u8::try_parse(remaining)?;
        let (keys_6, remaining) = u8::try_parse(remaining)?;
        let (keys_7, remaining) = u8::try_parse(remaining)?;
        let (keys_8, remaining) = u8::try_parse(remaining)?;
        let (keys_9, remaining) = u8::try_parse(remaining)?;
        let (keys_10, remaining) = u8::try_parse(remaining)?;
        let (keys_11, remaining) = u8::try_parse(remaining)?;
        let (keys_12, remaining) = u8::try_parse(remaining)?;
        let (keys_13, remaining) = u8::try_parse(remaining)?;
        let (keys_14, remaining) = u8::try_parse(remaining)?;
        let (keys_15, remaining) = u8::try_parse(remaining)?;
        let (keys_16, remaining) = u8::try_parse(remaining)?;
        let (keys_17, remaining) = u8::try_parse(remaining)?;
        let (keys_18, remaining) = u8::try_parse(remaining)?;
        let (keys_19, remaining) = u8::try_parse(remaining)?;
        let (keys_20, remaining) = u8::try_parse(remaining)?;
        let (keys_21, remaining) = u8::try_parse(remaining)?;
        let (keys_22, remaining) = u8::try_parse(remaining)?;
        let (keys_23, remaining) = u8::try_parse(remaining)?;
        let (keys_24, remaining) = u8::try_parse(remaining)?;
        let (keys_25, remaining) = u8::try_parse(remaining)?;
        let (keys_26, remaining) = u8::try_parse(remaining)?;
        let (keys_27, remaining) = u8::try_parse(remaining)?;
        let keys = [
            keys_0,
            keys_1,
            keys_2,
            keys_3,
            keys_4,
            keys_5,
            keys_6,
            keys_7,
            keys_8,
            keys_9,
            keys_10,
            keys_11,
            keys_12,
            keys_13,
            keys_14,
            keys_15,
            keys_16,
            keys_17,
            keys_18,
            keys_19,
            keys_20,
            keys_21,
            keys_22,
            keys_23,
            keys_24,
            keys_25,
            keys_26,
            keys_27,
        ];
        let result = DeviceKeyStateNotifyEvent { response_type, device_id, sequence, keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceKeyStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceKeyStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceKeyStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceKeyStateNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceKeyStateNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let device_id = input.device_id.serialize();
        let sequence = input.sequence.serialize();
        [
            response_type[0], device_id[0], sequence[0], sequence[1], input.keys[0], input.keys[1], input.keys[2], input.keys[3],
            input.keys[4], input.keys[5], input.keys[6], input.keys[7], input.keys[8], input.keys[9], input.keys[10], input.keys[11],
            input.keys[12], input.keys[13], input.keys[14], input.keys[15], input.keys[16], input.keys[17], input.keys[18], input.keys[19],
            input.keys[20], input.keys[21], input.keys[22], input.keys[23], input.keys[24], input.keys[25], input.keys[26], input.keys[27]
        ]
    }
}
impl From<DeviceKeyStateNotifyEvent> for [u8; 32] {
    fn from(input: DeviceKeyStateNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceButtonStateNotify event
pub const DEVICE_BUTTON_STATE_NOTIFY_EVENT: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceButtonStateNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub buttons: [u8; 28],
}
impl DeviceButtonStateNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (buttons_0, remaining) = u8::try_parse(remaining)?;
        let (buttons_1, remaining) = u8::try_parse(remaining)?;
        let (buttons_2, remaining) = u8::try_parse(remaining)?;
        let (buttons_3, remaining) = u8::try_parse(remaining)?;
        let (buttons_4, remaining) = u8::try_parse(remaining)?;
        let (buttons_5, remaining) = u8::try_parse(remaining)?;
        let (buttons_6, remaining) = u8::try_parse(remaining)?;
        let (buttons_7, remaining) = u8::try_parse(remaining)?;
        let (buttons_8, remaining) = u8::try_parse(remaining)?;
        let (buttons_9, remaining) = u8::try_parse(remaining)?;
        let (buttons_10, remaining) = u8::try_parse(remaining)?;
        let (buttons_11, remaining) = u8::try_parse(remaining)?;
        let (buttons_12, remaining) = u8::try_parse(remaining)?;
        let (buttons_13, remaining) = u8::try_parse(remaining)?;
        let (buttons_14, remaining) = u8::try_parse(remaining)?;
        let (buttons_15, remaining) = u8::try_parse(remaining)?;
        let (buttons_16, remaining) = u8::try_parse(remaining)?;
        let (buttons_17, remaining) = u8::try_parse(remaining)?;
        let (buttons_18, remaining) = u8::try_parse(remaining)?;
        let (buttons_19, remaining) = u8::try_parse(remaining)?;
        let (buttons_20, remaining) = u8::try_parse(remaining)?;
        let (buttons_21, remaining) = u8::try_parse(remaining)?;
        let (buttons_22, remaining) = u8::try_parse(remaining)?;
        let (buttons_23, remaining) = u8::try_parse(remaining)?;
        let (buttons_24, remaining) = u8::try_parse(remaining)?;
        let (buttons_25, remaining) = u8::try_parse(remaining)?;
        let (buttons_26, remaining) = u8::try_parse(remaining)?;
        let (buttons_27, remaining) = u8::try_parse(remaining)?;
        let buttons = [
            buttons_0,
            buttons_1,
            buttons_2,
            buttons_3,
            buttons_4,
            buttons_5,
            buttons_6,
            buttons_7,
            buttons_8,
            buttons_9,
            buttons_10,
            buttons_11,
            buttons_12,
            buttons_13,
            buttons_14,
            buttons_15,
            buttons_16,
            buttons_17,
            buttons_18,
            buttons_19,
            buttons_20,
            buttons_21,
            buttons_22,
            buttons_23,
            buttons_24,
            buttons_25,
            buttons_26,
            buttons_27,
        ];
        let result = DeviceButtonStateNotifyEvent { response_type, device_id, sequence, buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceButtonStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceButtonStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceButtonStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DeviceButtonStateNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceButtonStateNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let device_id = input.device_id.serialize();
        let sequence = input.sequence.serialize();
        [
            response_type[0], device_id[0], sequence[0], sequence[1], input.buttons[0], input.buttons[1], input.buttons[2], input.buttons[3],
            input.buttons[4], input.buttons[5], input.buttons[6], input.buttons[7], input.buttons[8], input.buttons[9], input.buttons[10], input.buttons[11],
            input.buttons[12], input.buttons[13], input.buttons[14], input.buttons[15], input.buttons[16], input.buttons[17], input.buttons[18], input.buttons[19],
            input.buttons[20], input.buttons[21], input.buttons[22], input.buttons[23], input.buttons[24], input.buttons[25], input.buttons[26], input.buttons[27]
        ]
    }
}
impl From<DeviceButtonStateNotifyEvent> for [u8; 32] {
    fn from(input: DeviceButtonStateNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceChange {
    Added = 0,
    Removed = 1,
    Enabled = 2,
    Disabled = 3,
    Unrecoverable = 4,
    ControlChanged = 5,
}
impl From<DeviceChange> for u8 {
    fn from(input: DeviceChange) -> Self {
        match input {
            DeviceChange::Added => 0,
            DeviceChange::Removed => 1,
            DeviceChange::Enabled => 2,
            DeviceChange::Disabled => 3,
            DeviceChange::Unrecoverable => 4,
            DeviceChange::ControlChanged => 5,
        }
    }
}
impl From<DeviceChange> for Option<u8> {
    fn from(input: DeviceChange) -> Self {
        Some(u8::from(input))
    }
}
impl From<DeviceChange> for u16 {
    fn from(input: DeviceChange) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceChange> for Option<u16> {
    fn from(input: DeviceChange) -> Self {
        Some(u16::from(input))
    }
}
impl From<DeviceChange> for u32 {
    fn from(input: DeviceChange) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DeviceChange> for Option<u32> {
    fn from(input: DeviceChange) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DeviceChange {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeviceChange::Added),
            1 => Ok(DeviceChange::Removed),
            2 => Ok(DeviceChange::Enabled),
            3 => Ok(DeviceChange::Disabled),
            4 => Ok(DeviceChange::Unrecoverable),
            5 => Ok(DeviceChange::ControlChanged),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for DeviceChange {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DeviceChange {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the DevicePresenceNotify event
pub const DEVICE_PRESENCE_NOTIFY_EVENT: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DevicePresenceNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub devchange: DeviceChange,
    pub device_id: u8,
    pub control: u16,
}
impl DevicePresenceNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (devchange, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (control, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let devchange = devchange.try_into()?;
        let result = DevicePresenceNotifyEvent { response_type, sequence, time, devchange, device_id, control };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DevicePresenceNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DevicePresenceNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DevicePresenceNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DevicePresenceNotifyEvent> for [u8; 32] {
    fn from(input: &DevicePresenceNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let devchange = Into::<u8>::into(input.devchange).serialize();
        let device_id = input.device_id.serialize();
        let control = input.control.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            devchange[0], device_id[0], control[0], control[1], 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<DevicePresenceNotifyEvent> for [u8; 32] {
    fn from(input: DevicePresenceNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DevicePropertyNotify event
pub const DEVICE_PROPERTY_NOTIFY_EVENT: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DevicePropertyNotifyEvent {
    pub response_type: u8,
    pub state: xproto::Property,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub property: xproto::Atom,
    pub device_id: u8,
}
impl DevicePropertyNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let remaining = remaining.get(19..).ok_or(ParseError::ParseError)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let state = state.try_into()?;
        let result = DevicePropertyNotifyEvent { response_type, state, sequence, time, property, device_id };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DevicePropertyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DevicePropertyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DevicePropertyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DevicePropertyNotifyEvent> for [u8; 32] {
    fn from(input: &DevicePropertyNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let state = Into::<u8>::into(input.state).serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let property = input.property.serialize();
        let device_id = input.device_id.serialize();
        [
            response_type[0], state[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            property[0], property[1], property[2], property[3], 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, device_id[0]
        ]
    }
}
impl From<DevicePropertyNotifyEvent> for [u8; 32] {
    fn from(input: DevicePropertyNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChangeReason {
    SlaveSwitch = 1,
    DeviceChange = 2,
}
impl From<ChangeReason> for u8 {
    fn from(input: ChangeReason) -> Self {
        match input {
            ChangeReason::SlaveSwitch => 1,
            ChangeReason::DeviceChange => 2,
        }
    }
}
impl From<ChangeReason> for Option<u8> {
    fn from(input: ChangeReason) -> Self {
        Some(u8::from(input))
    }
}
impl From<ChangeReason> for u16 {
    fn from(input: ChangeReason) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ChangeReason> for Option<u16> {
    fn from(input: ChangeReason) -> Self {
        Some(u16::from(input))
    }
}
impl From<ChangeReason> for u32 {
    fn from(input: ChangeReason) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ChangeReason> for Option<u32> {
    fn from(input: ChangeReason) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ChangeReason {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ChangeReason::SlaveSwitch),
            2 => Ok(ChangeReason::DeviceChange),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ChangeReason {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ChangeReason {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the DeviceChanged event
pub const DEVICE_CHANGED_EVENT: u16 = 1;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceChangedEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub sourceid: DeviceId,
    pub reason: ChangeReason,
    pub classes: Vec<DeviceClass>,
}
impl DeviceChangedEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (reason, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::ParseError)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<DeviceClass>(remaining, num_classes as usize)?;
        let reason = reason.try_into()?;
        let result = DeviceChangedEvent { response_type, extension, sequence, length, event_type, deviceid, time, sourceid, reason, classes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceChangedEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DeviceChangedEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DeviceChangedEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum KeyEventFlags {
    KeyRepeat = 1 << 16,
}
impl From<KeyEventFlags> for u32 {
    fn from(input: KeyEventFlags) -> Self {
        match input {
            KeyEventFlags::KeyRepeat => 1 << 16,
        }
    }
}
impl From<KeyEventFlags> for Option<u32> {
    fn from(input: KeyEventFlags) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for KeyEventFlags {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            65536 => Ok(KeyEventFlags::KeyRepeat),
            _ => Err(ParseError::ParseError)
        }
    }
}
bitmask_binop!(KeyEventFlags, u32);

/// Opcode for the KeyPress event
pub const KEY_PRESS_EVENT: u16 = 2;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPressEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Vec<u32>,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
}
impl KeyPressEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = KeyPressEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the KeyRelease event
pub const KEY_RELEASE_EVENT: u16 = 3;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyReleaseEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Vec<u32>,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
}
impl KeyReleaseEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = KeyReleaseEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for KeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for KeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PointerEventFlags {
    PointerEmulated = 1 << 16,
}
impl From<PointerEventFlags> for u32 {
    fn from(input: PointerEventFlags) -> Self {
        match input {
            PointerEventFlags::PointerEmulated => 1 << 16,
        }
    }
}
impl From<PointerEventFlags> for Option<u32> {
    fn from(input: PointerEventFlags) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for PointerEventFlags {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            65536 => Ok(PointerEventFlags::PointerEmulated),
            _ => Err(ParseError::ParseError)
        }
    }
}
bitmask_binop!(PointerEventFlags, u32);

/// Opcode for the ButtonPress event
pub const BUTTON_PRESS_EVENT: u16 = 4;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonPressEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Vec<u32>,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
}
impl ButtonPressEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = ButtonPressEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the ButtonRelease event
pub const BUTTON_RELEASE_EVENT: u16 = 5;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonReleaseEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Vec<u32>,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
}
impl ButtonReleaseEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = ButtonReleaseEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the Motion event
pub const MOTION_EVENT: u16 = 6;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Vec<u32>,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
}
impl MotionEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = MotionEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MotionEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for MotionEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for MotionEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NotifyMode {
    Normal = 0,
    Grab = 1,
    Ungrab = 2,
    WhileGrabbed = 3,
    PassiveGrab = 4,
    PassiveUngrab = 5,
}
impl From<NotifyMode> for u8 {
    fn from(input: NotifyMode) -> Self {
        match input {
            NotifyMode::Normal => 0,
            NotifyMode::Grab => 1,
            NotifyMode::Ungrab => 2,
            NotifyMode::WhileGrabbed => 3,
            NotifyMode::PassiveGrab => 4,
            NotifyMode::PassiveUngrab => 5,
        }
    }
}
impl From<NotifyMode> for Option<u8> {
    fn from(input: NotifyMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<NotifyMode> for u16 {
    fn from(input: NotifyMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyMode> for Option<u16> {
    fn from(input: NotifyMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<NotifyMode> for u32 {
    fn from(input: NotifyMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyMode> for Option<u32> {
    fn from(input: NotifyMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for NotifyMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NotifyMode::Normal),
            1 => Ok(NotifyMode::Grab),
            2 => Ok(NotifyMode::Ungrab),
            3 => Ok(NotifyMode::WhileGrabbed),
            4 => Ok(NotifyMode::PassiveGrab),
            5 => Ok(NotifyMode::PassiveUngrab),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for NotifyMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for NotifyMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NotifyDetail {
    Ancestor = 0,
    Virtual = 1,
    Inferior = 2,
    Nonlinear = 3,
    NonlinearVirtual = 4,
    Pointer = 5,
    PointerRoot = 6,
    None = 7,
}
impl From<NotifyDetail> for u8 {
    fn from(input: NotifyDetail) -> Self {
        match input {
            NotifyDetail::Ancestor => 0,
            NotifyDetail::Virtual => 1,
            NotifyDetail::Inferior => 2,
            NotifyDetail::Nonlinear => 3,
            NotifyDetail::NonlinearVirtual => 4,
            NotifyDetail::Pointer => 5,
            NotifyDetail::PointerRoot => 6,
            NotifyDetail::None => 7,
        }
    }
}
impl From<NotifyDetail> for Option<u8> {
    fn from(input: NotifyDetail) -> Self {
        Some(u8::from(input))
    }
}
impl From<NotifyDetail> for u16 {
    fn from(input: NotifyDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyDetail> for Option<u16> {
    fn from(input: NotifyDetail) -> Self {
        Some(u16::from(input))
    }
}
impl From<NotifyDetail> for u32 {
    fn from(input: NotifyDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyDetail> for Option<u32> {
    fn from(input: NotifyDetail) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for NotifyDetail {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NotifyDetail::Ancestor),
            1 => Ok(NotifyDetail::Virtual),
            2 => Ok(NotifyDetail::Inferior),
            3 => Ok(NotifyDetail::Nonlinear),
            4 => Ok(NotifyDetail::NonlinearVirtual),
            5 => Ok(NotifyDetail::Pointer),
            6 => Ok(NotifyDetail::PointerRoot),
            7 => Ok(NotifyDetail::None),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for NotifyDetail {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for NotifyDetail {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the Enter event
pub const ENTER_EVENT: u16 = 7;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnterEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub sourceid: DeviceId,
    pub mode: NotifyMode,
    pub detail: NotifyDetail,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
}
impl EnterEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (focus, remaining) = bool::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (buttons, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let mode = mode.try_into()?;
        let detail = detail.try_into()?;
        let result = EnterEvent { response_type, extension, sequence, length, event_type, deviceid, time, sourceid, mode, detail, root, event, child, root_x, root_y, event_x, event_y, same_screen, focus, mods, group, buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for EnterEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for EnterEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for EnterEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the Leave event
pub const LEAVE_EVENT: u16 = 8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaveEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub sourceid: DeviceId,
    pub mode: NotifyMode,
    pub detail: NotifyDetail,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
}
impl LeaveEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (focus, remaining) = bool::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (buttons, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let mode = mode.try_into()?;
        let detail = detail.try_into()?;
        let result = LeaveEvent { response_type, extension, sequence, length, event_type, deviceid, time, sourceid, mode, detail, root, event, child, root_x, root_y, event_x, event_y, same_screen, focus, mods, group, buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for LeaveEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for LeaveEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for LeaveEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the FocusIn event
pub const FOCUS_IN_EVENT: u16 = 9;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusInEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub sourceid: DeviceId,
    pub mode: NotifyMode,
    pub detail: NotifyDetail,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
}
impl FocusInEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (focus, remaining) = bool::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (buttons, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let mode = mode.try_into()?;
        let detail = detail.try_into()?;
        let result = FocusInEvent { response_type, extension, sequence, length, event_type, deviceid, time, sourceid, mode, detail, root, event, child, root_x, root_y, event_x, event_y, same_screen, focus, mods, group, buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FocusInEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for FocusInEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for FocusInEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the FocusOut event
pub const FOCUS_OUT_EVENT: u16 = 10;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusOutEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub sourceid: DeviceId,
    pub mode: NotifyMode,
    pub detail: NotifyDetail,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
}
impl FocusOutEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (focus, remaining) = bool::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (buttons, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let mode = mode.try_into()?;
        let detail = detail.try_into()?;
        let result = FocusOutEvent { response_type, extension, sequence, length, event_type, deviceid, time, sourceid, mode, detail, root, event, child, root_x, root_y, event_x, event_y, same_screen, focus, mods, group, buttons };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FocusOutEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for FocusOutEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for FocusOutEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HierarchyMask {
    MasterAdded = 1 << 0,
    MasterRemoved = 1 << 1,
    SlaveAdded = 1 << 2,
    SlaveRemoved = 1 << 3,
    SlaveAttached = 1 << 4,
    SlaveDetached = 1 << 5,
    DeviceEnabled = 1 << 6,
    DeviceDisabled = 1 << 7,
}
impl From<HierarchyMask> for u8 {
    fn from(input: HierarchyMask) -> Self {
        match input {
            HierarchyMask::MasterAdded => 1 << 0,
            HierarchyMask::MasterRemoved => 1 << 1,
            HierarchyMask::SlaveAdded => 1 << 2,
            HierarchyMask::SlaveRemoved => 1 << 3,
            HierarchyMask::SlaveAttached => 1 << 4,
            HierarchyMask::SlaveDetached => 1 << 5,
            HierarchyMask::DeviceEnabled => 1 << 6,
            HierarchyMask::DeviceDisabled => 1 << 7,
        }
    }
}
impl From<HierarchyMask> for Option<u8> {
    fn from(input: HierarchyMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<HierarchyMask> for u16 {
    fn from(input: HierarchyMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<HierarchyMask> for Option<u16> {
    fn from(input: HierarchyMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<HierarchyMask> for u32 {
    fn from(input: HierarchyMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<HierarchyMask> for Option<u32> {
    fn from(input: HierarchyMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for HierarchyMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HierarchyMask::MasterAdded),
            2 => Ok(HierarchyMask::MasterRemoved),
            4 => Ok(HierarchyMask::SlaveAdded),
            8 => Ok(HierarchyMask::SlaveRemoved),
            16 => Ok(HierarchyMask::SlaveAttached),
            32 => Ok(HierarchyMask::SlaveDetached),
            64 => Ok(HierarchyMask::DeviceEnabled),
            128 => Ok(HierarchyMask::DeviceDisabled),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for HierarchyMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for HierarchyMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(HierarchyMask, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HierarchyInfo {
    pub deviceid: DeviceId,
    pub attachment: DeviceId,
    pub type_: DeviceType,
    pub enabled: bool,
    pub flags: u32,
}
impl TryParse for HierarchyInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (attachment, remaining) = DeviceId::try_parse(remaining)?;
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (enabled, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let result = HierarchyInfo { deviceid, attachment, type_, enabled, flags };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for HierarchyInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for HierarchyInfo {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let deviceid_bytes = self.deviceid.serialize();
        let attachment_bytes = self.attachment.serialize();
        let type_bytes = Into::<u8>::into(self.type_).serialize();
        let enabled_bytes = self.enabled.serialize();
        let flags_bytes = self.flags.serialize();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            attachment_bytes[0],
            attachment_bytes[1],
            type_bytes[0],
            enabled_bytes[0],
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.deviceid.serialize_into(bytes);
        self.attachment.serialize_into(bytes);
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.enabled.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
    }
}

/// Opcode for the Hierarchy event
pub const HIERARCHY_EVENT: u16 = 11;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub flags: u32,
    pub infos: Vec<HierarchyInfo>,
}
impl HierarchyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (num_infos, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(10..).ok_or(ParseError::ParseError)?;
        let (infos, remaining) = crate::x11_utils::parse_list::<HierarchyInfo>(remaining, num_infos as usize)?;
        let result = HierarchyEvent { response_type, extension, sequence, length, event_type, deviceid, time, flags, infos };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for HierarchyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for HierarchyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for HierarchyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PropertyFlag {
    Deleted = 0,
    Created = 1,
    Modified = 2,
}
impl From<PropertyFlag> for u8 {
    fn from(input: PropertyFlag) -> Self {
        match input {
            PropertyFlag::Deleted => 0,
            PropertyFlag::Created => 1,
            PropertyFlag::Modified => 2,
        }
    }
}
impl From<PropertyFlag> for Option<u8> {
    fn from(input: PropertyFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<PropertyFlag> for u16 {
    fn from(input: PropertyFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PropertyFlag> for Option<u16> {
    fn from(input: PropertyFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<PropertyFlag> for u32 {
    fn from(input: PropertyFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PropertyFlag> for Option<u32> {
    fn from(input: PropertyFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PropertyFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PropertyFlag::Deleted),
            1 => Ok(PropertyFlag::Created),
            2 => Ok(PropertyFlag::Modified),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for PropertyFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PropertyFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the Property event
pub const PROPERTY_EVENT: u16 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub property: xproto::Atom,
    pub what: PropertyFlag,
}
impl PropertyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (what, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::ParseError)?;
        let what = what.try_into()?;
        let result = PropertyEvent { response_type, extension, sequence, length, event_type, deviceid, time, property, what };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PropertyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for PropertyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for PropertyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RawKeyPress event
pub const RAW_KEY_PRESS_EVENT: u16 = 13;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawKeyPressEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
    pub axisvalues_raw: Vec<Fp3232>,
}
impl RawKeyPressEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = RawKeyPressEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RawKeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for RawKeyPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for RawKeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RawKeyRelease event
pub const RAW_KEY_RELEASE_EVENT: u16 = 14;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawKeyReleaseEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
    pub axisvalues_raw: Vec<Fp3232>,
}
impl RawKeyReleaseEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = RawKeyReleaseEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RawKeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for RawKeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for RawKeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RawButtonPress event
pub const RAW_BUTTON_PRESS_EVENT: u16 = 15;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawButtonPressEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
    pub axisvalues_raw: Vec<Fp3232>,
}
impl RawButtonPressEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = RawButtonPressEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RawButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for RawButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for RawButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RawButtonRelease event
pub const RAW_BUTTON_RELEASE_EVENT: u16 = 16;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawButtonReleaseEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
    pub axisvalues_raw: Vec<Fp3232>,
}
impl RawButtonReleaseEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = RawButtonReleaseEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RawButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for RawButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for RawButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RawMotion event
pub const RAW_MOTION_EVENT: u16 = 17;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawMotionEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
    pub axisvalues_raw: Vec<Fp3232>,
}
impl RawMotionEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = RawMotionEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RawMotionEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for RawMotionEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for RawMotionEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TouchEventFlags {
    TouchPendingEnd = 1 << 16,
    TouchEmulatingPointer = 1 << 17,
}
impl From<TouchEventFlags> for u32 {
    fn from(input: TouchEventFlags) -> Self {
        match input {
            TouchEventFlags::TouchPendingEnd => 1 << 16,
            TouchEventFlags::TouchEmulatingPointer => 1 << 17,
        }
    }
}
impl From<TouchEventFlags> for Option<u32> {
    fn from(input: TouchEventFlags) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for TouchEventFlags {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            65536 => Ok(TouchEventFlags::TouchPendingEnd),
            131_072 => Ok(TouchEventFlags::TouchEmulatingPointer),
            _ => Err(ParseError::ParseError)
        }
    }
}
bitmask_binop!(TouchEventFlags, u32);

/// Opcode for the TouchBegin event
pub const TOUCH_BEGIN_EVENT: u16 = 18;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TouchBeginEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Vec<u32>,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
}
impl TouchBeginEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = TouchBeginEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for TouchBeginEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for TouchBeginEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for TouchBeginEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the TouchUpdate event
pub const TOUCH_UPDATE_EVENT: u16 = 19;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TouchUpdateEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Vec<u32>,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
}
impl TouchUpdateEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = TouchUpdateEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for TouchUpdateEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for TouchUpdateEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for TouchUpdateEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the TouchEnd event
pub const TOUCH_END_EVENT: u16 = 20;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TouchEndEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Vec<u32>,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
}
impl TouchEndEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (event_x, remaining) = Fp1616::try_parse(remaining)?;
        let (event_y, remaining) = Fp1616::try_parse(remaining)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len as usize)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = TouchEndEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for TouchEndEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for TouchEndEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for TouchEndEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TouchOwnershipFlags {
    None = 0,
}
impl From<TouchOwnershipFlags> for u8 {
    fn from(input: TouchOwnershipFlags) -> Self {
        match input {
            TouchOwnershipFlags::None => 0,
        }
    }
}
impl From<TouchOwnershipFlags> for Option<u8> {
    fn from(input: TouchOwnershipFlags) -> Self {
        Some(u8::from(input))
    }
}
impl From<TouchOwnershipFlags> for u16 {
    fn from(input: TouchOwnershipFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<TouchOwnershipFlags> for Option<u16> {
    fn from(input: TouchOwnershipFlags) -> Self {
        Some(u16::from(input))
    }
}
impl From<TouchOwnershipFlags> for u32 {
    fn from(input: TouchOwnershipFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<TouchOwnershipFlags> for Option<u32> {
    fn from(input: TouchOwnershipFlags) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for TouchOwnershipFlags {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TouchOwnershipFlags::None),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for TouchOwnershipFlags {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for TouchOwnershipFlags {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the TouchOwnership event
pub const TOUCH_OWNERSHIP_EVENT: u16 = 21;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TouchOwnershipEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub touchid: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub child: xproto::Window,
    pub sourceid: DeviceId,
    pub flags: TouchOwnershipFlags,
}
impl TouchOwnershipEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (touchid, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let flags = flags.try_into()?;
        let result = TouchOwnershipEvent { response_type, extension, sequence, length, event_type, deviceid, time, touchid, root, event, child, sourceid, flags };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for TouchOwnershipEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for TouchOwnershipEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for TouchOwnershipEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RawTouchBegin event
pub const RAW_TOUCH_BEGIN_EVENT: u16 = 22;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawTouchBeginEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
    pub axisvalues_raw: Vec<Fp3232>,
}
impl RawTouchBeginEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = RawTouchBeginEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RawTouchBeginEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for RawTouchBeginEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for RawTouchBeginEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RawTouchUpdate event
pub const RAW_TOUCH_UPDATE_EVENT: u16 = 23;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawTouchUpdateEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
    pub axisvalues_raw: Vec<Fp3232>,
}
impl RawTouchUpdateEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = RawTouchUpdateEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RawTouchUpdateEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for RawTouchUpdateEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for RawTouchUpdateEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the RawTouchEnd event
pub const RAW_TOUCH_END_EVENT: u16 = 24;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawTouchEndEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub detail: u32,
    pub sourceid: DeviceId,
    pub flags: u32,
    pub valuator_mask: Vec<u32>,
    pub axisvalues: Vec<Fp3232>,
    pub axisvalues_raw: Vec<Fp3232>,
}
impl RawTouchEndEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (valuators_len, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len as usize)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().map(|x| TryInto::<usize>::try_into(x.count_ones()).unwrap()).sum())?;
        let result = RawTouchEndEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RawTouchEndEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for RawTouchEndEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for RawTouchEndEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BarrierFlags {
    PointerReleased = 1 << 0,
    DeviceIsGrabbed = 1 << 1,
}
impl From<BarrierFlags> for u8 {
    fn from(input: BarrierFlags) -> Self {
        match input {
            BarrierFlags::PointerReleased => 1 << 0,
            BarrierFlags::DeviceIsGrabbed => 1 << 1,
        }
    }
}
impl From<BarrierFlags> for Option<u8> {
    fn from(input: BarrierFlags) -> Self {
        Some(u8::from(input))
    }
}
impl From<BarrierFlags> for u16 {
    fn from(input: BarrierFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BarrierFlags> for Option<u16> {
    fn from(input: BarrierFlags) -> Self {
        Some(u16::from(input))
    }
}
impl From<BarrierFlags> for u32 {
    fn from(input: BarrierFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BarrierFlags> for Option<u32> {
    fn from(input: BarrierFlags) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BarrierFlags {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BarrierFlags::PointerReleased),
            2 => Ok(BarrierFlags::DeviceIsGrabbed),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for BarrierFlags {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for BarrierFlags {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(BarrierFlags, u8);

/// Opcode for the BarrierHit event
pub const BARRIER_HIT_EVENT: u16 = 25;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarrierHitEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub eventid: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub barrier: xfixes::Barrier,
    pub dtime: u32,
    pub flags: u32,
    pub sourceid: DeviceId,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub dx: Fp3232,
    pub dy: Fp3232,
}
impl BarrierHitEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (eventid, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (barrier, remaining) = xfixes::Barrier::try_parse(remaining)?;
        let (dtime, remaining) = u32::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (dx, remaining) = Fp3232::try_parse(remaining)?;
        let (dy, remaining) = Fp3232::try_parse(remaining)?;
        let result = BarrierHitEvent { response_type, extension, sequence, length, event_type, deviceid, time, eventid, root, event, barrier, dtime, flags, sourceid, root_x, root_y, dx, dy };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BarrierHitEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for BarrierHitEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for BarrierHitEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the BarrierLeave event
pub const BARRIER_LEAVE_EVENT: u16 = 26;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarrierLeaveEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: DeviceId,
    pub time: xproto::Timestamp,
    pub eventid: u32,
    pub root: xproto::Window,
    pub event: xproto::Window,
    pub barrier: xfixes::Barrier,
    pub dtime: u32,
    pub flags: u32,
    pub sourceid: DeviceId,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub dx: Fp3232,
    pub dy: Fp3232,
}
impl BarrierLeaveEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (eventid, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (event, remaining) = xproto::Window::try_parse(remaining)?;
        let (barrier, remaining) = xfixes::Barrier::try_parse(remaining)?;
        let (dtime, remaining) = u32::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (dx, remaining) = Fp3232::try_parse(remaining)?;
        let (dy, remaining) = Fp3232::try_parse(remaining)?;
        let result = BarrierLeaveEvent { response_type, extension, sequence, length, event_type, deviceid, time, eventid, root, event, barrier, dtime, flags, sourceid, root_x, root_y, dx, dy };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BarrierLeaveEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for BarrierLeaveEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for BarrierLeaveEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EventForSend([u8; 32]);
impl EventForSend {
    pub fn as_xproto_device_valuator(&self) -> DeviceValuatorEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceValuatorEvent, ParseError> {
            let (event, remaining) = DeviceValuatorEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_key_press(&self) -> DeviceKeyPressEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceKeyPressEvent, ParseError> {
            let (event, remaining) = DeviceKeyPressEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_key_release(&self) -> DeviceKeyReleaseEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceKeyReleaseEvent, ParseError> {
            let (event, remaining) = DeviceKeyReleaseEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_button_press(&self) -> DeviceButtonPressEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceButtonPressEvent, ParseError> {
            let (event, remaining) = DeviceButtonPressEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_button_release(&self) -> DeviceButtonReleaseEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceButtonReleaseEvent, ParseError> {
            let (event, remaining) = DeviceButtonReleaseEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_motion_notify(&self) -> DeviceMotionNotifyEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceMotionNotifyEvent, ParseError> {
            let (event, remaining) = DeviceMotionNotifyEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_focus_in(&self) -> DeviceFocusInEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceFocusInEvent, ParseError> {
            let (event, remaining) = DeviceFocusInEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_focus_out(&self) -> DeviceFocusOutEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceFocusOutEvent, ParseError> {
            let (event, remaining) = DeviceFocusOutEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_proximity_in(&self) -> ProximityInEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<ProximityInEvent, ParseError> {
            let (event, remaining) = ProximityInEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_proximity_out(&self) -> ProximityOutEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<ProximityOutEvent, ParseError> {
            let (event, remaining) = ProximityOutEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_state_notify(&self) -> DeviceStateNotifyEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceStateNotifyEvent, ParseError> {
            let (event, remaining) = DeviceStateNotifyEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_mapping_notify(&self) -> DeviceMappingNotifyEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceMappingNotifyEvent, ParseError> {
            let (event, remaining) = DeviceMappingNotifyEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_change_device_notify(&self) -> ChangeDeviceNotifyEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<ChangeDeviceNotifyEvent, ParseError> {
            let (event, remaining) = ChangeDeviceNotifyEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_key_state_notify(&self) -> DeviceKeyStateNotifyEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceKeyStateNotifyEvent, ParseError> {
            let (event, remaining) = DeviceKeyStateNotifyEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_button_state_notify(&self) -> DeviceButtonStateNotifyEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DeviceButtonStateNotifyEvent, ParseError> {
            let (event, remaining) = DeviceButtonStateNotifyEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_xproto_device_presence_notify(&self) -> DevicePresenceNotifyEvent {
        fn do_the_parse(remaining: &[u8]) -> Result<DevicePresenceNotifyEvent, ParseError> {
            let (event, remaining) = DevicePresenceNotifyEvent::try_parse(remaining)?;
            let _ = remaining;
            Ok(event)
        }
        do_the_parse(&self.0).unwrap()
    }
}
impl Serialize for EventForSend {
    type Bytes = [u8; 32];
    fn serialize(&self) -> Self::Bytes {
        self.0
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}
impl TryParse for EventForSend {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner: [u8; 32] = value.get(..32)
            .ok_or(ParseError::ParseError)?
            .try_into()
            .unwrap();
        let result = EventForSend(inner);
        Ok((result, &value[32..]))
    }
}
impl From<DeviceValuatorEvent> for EventForSend {
    fn from(value: DeviceValuatorEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}
impl From<DeviceKeyPressEvent> for EventForSend {
    fn from(value: DeviceKeyPressEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}
impl From<DeviceFocusInEvent> for EventForSend {
    fn from(value: DeviceFocusInEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}
impl From<DeviceStateNotifyEvent> for EventForSend {
    fn from(value: DeviceStateNotifyEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}
impl From<DeviceMappingNotifyEvent> for EventForSend {
    fn from(value: DeviceMappingNotifyEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}
impl From<ChangeDeviceNotifyEvent> for EventForSend {
    fn from(value: ChangeDeviceNotifyEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}
impl From<DeviceKeyStateNotifyEvent> for EventForSend {
    fn from(value: DeviceKeyStateNotifyEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}
impl From<DeviceButtonStateNotifyEvent> for EventForSend {
    fn from(value: DeviceButtonStateNotifyEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}
impl From<DevicePresenceNotifyEvent> for EventForSend {
    fn from(value: DevicePresenceNotifyEvent) -> Self {
        Self(Into::<[u8; 32]>::into(value))
    }
}

/// Opcode for the SendExtensionEvent request
pub const SEND_EXTENSION_EVENT_REQUEST: u8 = 31;
pub fn send_extension_event<'c, Conn>(conn: &'c Conn, destination: xproto::Window, device_id: u8, propagate: bool, events: &[EventForSend], classes: &[EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16 + 32 * events.len() + 4 * classes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let destination_bytes = destination.serialize();
    let device_id_bytes = device_id.serialize();
    let propagate_bytes = (propagate as u8).serialize();
    let num_classes: u16 = classes.len().try_into()?;
    let num_classes_bytes = num_classes.serialize();
    let num_events: u8 = events.len().try_into()?;
    let num_events_bytes = num_events.serialize();
    let events_bytes = events.serialize();
    let request0 = [
        extension_information.major_opcode,
        SEND_EXTENSION_EVENT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
        device_id_bytes[0],
        propagate_bytes[0],
        num_classes_bytes[0],
        num_classes_bytes[1],
        num_events_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&events_bytes).len();
    let classes_bytes = classes.serialize();
    let length_so_far = length_so_far + (&classes_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&events_bytes), IoSlice::new(&classes_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the Device error
pub const DEVICE_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl DeviceError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = DeviceError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for DeviceError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for DeviceError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&DeviceError> for [u8; 32] {
    fn from(input: &DeviceError) -> Self {
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
impl From<DeviceError> for [u8; 32] {
    fn from(input: DeviceError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Event error
pub const EVENT_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl EventError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = EventError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for EventError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for EventError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for EventError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&EventError> for [u8; 32] {
    fn from(input: &EventError) -> Self {
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
impl From<EventError> for [u8; 32] {
    fn from(input: EventError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Mode error
pub const MODE_ERROR: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl ModeError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = ModeError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ModeError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ModeError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ModeError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ModeError> for [u8; 32] {
    fn from(input: &ModeError) -> Self {
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
impl From<ModeError> for [u8; 32] {
    fn from(input: ModeError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceBusy error
pub const DEVICE_BUSY_ERROR: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceBusyError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl DeviceBusyError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = DeviceBusyError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceBusyError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for DeviceBusyError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for DeviceBusyError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&DeviceBusyError> for [u8; 32] {
    fn from(input: &DeviceBusyError) -> Self {
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
impl From<DeviceBusyError> for [u8; 32] {
    fn from(input: DeviceBusyError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Class error
pub const CLASS_ERROR: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl ClassError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = ClassError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ClassError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ClassError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ClassError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ClassError> for [u8; 32] {
    fn from(input: &ClassError) -> Self {
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
impl From<ClassError> for [u8; 32] {
    fn from(input: ClassError) -> Self {
        Self::from(&input)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xinput_get_extension_version<'c>(&'c self, name: &[u8]) -> Result<Cookie<'c, Self, GetExtensionVersionReply>, ConnectionError>
    {
        get_extension_version(self, name)
    }

    fn xinput_list_input_devices(&self) -> Result<Cookie<'_, Self, ListInputDevicesReply>, ConnectionError>
    {
        list_input_devices(self)
    }

    fn xinput_open_device(&self, device_id: u8) -> Result<Cookie<'_, Self, OpenDeviceReply>, ConnectionError>
    {
        open_device(self, device_id)
    }

    fn xinput_close_device(&self, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        close_device(self, device_id)
    }

    fn xinput_set_device_mode<A>(&self, device_id: u8, mode: A) -> Result<Cookie<'_, Self, SetDeviceModeReply>, ConnectionError>
    where A: Into<u8>
    {
        set_device_mode(self, device_id, mode)
    }

    fn xinput_select_extension_event<'c>(&'c self, window: xproto::Window, classes: &[EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        select_extension_event(self, window, classes)
    }

    fn xinput_get_selected_extension_events(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetSelectedExtensionEventsReply>, ConnectionError>
    {
        get_selected_extension_events(self, window)
    }

    fn xinput_change_device_dont_propagate_list<'c, A>(&'c self, window: xproto::Window, mode: A, classes: &[EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        change_device_dont_propagate_list(self, window, mode, classes)
    }

    fn xinput_get_device_dont_propagate_list(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetDeviceDontPropagateListReply>, ConnectionError>
    {
        get_device_dont_propagate_list(self, window)
    }

    fn xinput_get_device_motion_events(&self, start: xproto::Timestamp, stop: xproto::Timestamp, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceMotionEventsReply>, ConnectionError>
    {
        get_device_motion_events(self, start, stop, device_id)
    }

    fn xinput_change_keyboard_device(&self, device_id: u8) -> Result<Cookie<'_, Self, ChangeKeyboardDeviceReply>, ConnectionError>
    {
        change_keyboard_device(self, device_id)
    }

    fn xinput_change_pointer_device(&self, x_axis: u8, y_axis: u8, device_id: u8) -> Result<Cookie<'_, Self, ChangePointerDeviceReply>, ConnectionError>
    {
        change_pointer_device(self, x_axis, y_axis, device_id)
    }

    fn xinput_grab_device<'c, A, B>(&'c self, grab_window: xproto::Window, time: xproto::Timestamp, this_device_mode: A, other_device_mode: B, owner_events: bool, device_id: u8, classes: &[EventClass]) -> Result<Cookie<'c, Self, GrabDeviceReply>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        grab_device(self, grab_window, time, this_device_mode, other_device_mode, owner_events, device_id, classes)
    }

    fn xinput_ungrab_device(&self, time: xproto::Timestamp, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        ungrab_device(self, time, device_id)
    }

    fn xinput_grab_device_key<'c, A, B>(&'c self, grab_window: xproto::Window, modifiers: u16, modifier_device: u8, grabbed_device: u8, key: u8, this_device_mode: A, other_device_mode: B, owner_events: bool, classes: &[EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        grab_device_key(self, grab_window, modifiers, modifier_device, grabbed_device, key, this_device_mode, other_device_mode, owner_events, classes)
    }

    fn xinput_ungrab_device_key(&self, grab_window: xproto::Window, modifiers: u16, modifier_device: u8, key: u8, grabbed_device: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        ungrab_device_key(self, grab_window, modifiers, modifier_device, key, grabbed_device)
    }

    fn xinput_grab_device_button<'c, A, B>(&'c self, grab_window: xproto::Window, grabbed_device: u8, modifier_device: u8, modifiers: u16, this_device_mode: A, other_device_mode: B, button: u8, owner_events: bool, classes: &[EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        grab_device_button(self, grab_window, grabbed_device, modifier_device, modifiers, this_device_mode, other_device_mode, button, owner_events, classes)
    }

    fn xinput_ungrab_device_button(&self, grab_window: xproto::Window, modifiers: u16, modifier_device: u8, button: u8, grabbed_device: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        ungrab_device_button(self, grab_window, modifiers, modifier_device, button, grabbed_device)
    }

    fn xinput_allow_device_events<A>(&self, time: xproto::Timestamp, mode: A, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        allow_device_events(self, time, mode, device_id)
    }

    fn xinput_get_device_focus(&self, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceFocusReply>, ConnectionError>
    {
        get_device_focus(self, device_id)
    }

    fn xinput_set_device_focus<A>(&self, focus: xproto::Window, time: xproto::Timestamp, revert_to: A, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        set_device_focus(self, focus, time, revert_to, device_id)
    }

    fn xinput_get_feedback_control(&self, device_id: u8) -> Result<Cookie<'_, Self, GetFeedbackControlReply>, ConnectionError>
    {
        get_feedback_control(self, device_id)
    }

    fn xinput_change_feedback_control(&self, mask: u32, device_id: u8, feedback_id: u8, feedback: FeedbackCtl) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        change_feedback_control(self, mask, device_id, feedback_id, feedback)
    }

    fn xinput_get_device_key_mapping(&self, device_id: u8, first_keycode: KeyCode, count: u8) -> Result<Cookie<'_, Self, GetDeviceKeyMappingReply>, ConnectionError>
    {
        get_device_key_mapping(self, device_id, first_keycode, count)
    }

    fn xinput_change_device_key_mapping<'c>(&'c self, device_id: u8, first_keycode: KeyCode, keysyms_per_keycode: u8, keycode_count: u8, keysyms: &[xproto::Keysym]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_device_key_mapping(self, device_id, first_keycode, keysyms_per_keycode, keycode_count, keysyms)
    }

    fn xinput_get_device_modifier_mapping(&self, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceModifierMappingReply>, ConnectionError>
    {
        get_device_modifier_mapping(self, device_id)
    }

    fn xinput_set_device_modifier_mapping<'c>(&'c self, device_id: u8, keymaps: &[u8]) -> Result<Cookie<'c, Self, SetDeviceModifierMappingReply>, ConnectionError>
    {
        set_device_modifier_mapping(self, device_id, keymaps)
    }

    fn xinput_get_device_button_mapping(&self, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceButtonMappingReply>, ConnectionError>
    {
        get_device_button_mapping(self, device_id)
    }

    fn xinput_set_device_button_mapping<'c>(&'c self, device_id: u8, map: &[u8]) -> Result<Cookie<'c, Self, SetDeviceButtonMappingReply>, ConnectionError>
    {
        set_device_button_mapping(self, device_id, map)
    }

    fn xinput_query_device_state(&self, device_id: u8) -> Result<Cookie<'_, Self, QueryDeviceStateReply>, ConnectionError>
    {
        query_device_state(self, device_id)
    }

    fn xinput_device_bell(&self, device_id: u8, feedback_id: u8, feedback_class: u8, percent: i8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        device_bell(self, device_id, feedback_id, feedback_class, percent)
    }

    fn xinput_set_device_valuators<'c>(&'c self, device_id: u8, first_valuator: u8, valuators: &[i32]) -> Result<Cookie<'c, Self, SetDeviceValuatorsReply>, ConnectionError>
    {
        set_device_valuators(self, device_id, first_valuator, valuators)
    }

    fn xinput_get_device_control<A>(&self, control_id: A, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceControlReply>, ConnectionError>
    where A: Into<u16>
    {
        get_device_control(self, control_id, device_id)
    }

    fn xinput_change_device_control<A>(&self, control_id: A, device_id: u8, control: DeviceCtl) -> Result<Cookie<'_, Self, ChangeDeviceControlReply>, ConnectionError>
    where A: Into<u16>
    {
        change_device_control(self, control_id, device_id, control)
    }

    fn xinput_list_device_properties(&self, device_id: u8) -> Result<Cookie<'_, Self, ListDevicePropertiesReply>, ConnectionError>
    {
        list_device_properties(self, device_id)
    }

    fn xinput_change_device_property<'c, A>(&'c self, property: xproto::Atom, type_: xproto::Atom, device_id: u8, mode: A, num_items: u32, items: &ChangeDevicePropertyAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        change_device_property(self, property, type_, device_id, mode, num_items, items)
    }

    fn xinput_delete_device_property(&self, property: xproto::Atom, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_device_property(self, property, device_id)
    }

    fn xinput_get_device_property(&self, property: xproto::Atom, type_: xproto::Atom, offset: u32, len: u32, device_id: u8, delete: bool) -> Result<Cookie<'_, Self, GetDevicePropertyReply>, ConnectionError>
    {
        get_device_property(self, property, type_, offset, len, device_id, delete)
    }

    fn xinput_xi_query_pointer(&self, window: xproto::Window, deviceid: DeviceId) -> Result<Cookie<'_, Self, XIQueryPointerReply>, ConnectionError>
    {
        xi_query_pointer(self, window, deviceid)
    }

    fn xinput_xi_warp_pointer(&self, src_win: xproto::Window, dst_win: xproto::Window, src_x: Fp1616, src_y: Fp1616, src_width: u16, src_height: u16, dst_x: Fp1616, dst_y: Fp1616, deviceid: DeviceId) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        xi_warp_pointer(self, src_win, dst_win, src_x, src_y, src_width, src_height, dst_x, dst_y, deviceid)
    }

    fn xinput_xi_change_cursor(&self, window: xproto::Window, cursor: xproto::Cursor, deviceid: DeviceId) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        xi_change_cursor(self, window, cursor, deviceid)
    }

    fn xinput_xi_change_hierarchy<'c>(&'c self, changes: &[HierarchyChange]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        xi_change_hierarchy(self, changes)
    }

    fn xinput_xi_set_client_pointer(&self, window: xproto::Window, deviceid: DeviceId) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        xi_set_client_pointer(self, window, deviceid)
    }

    fn xinput_xi_get_client_pointer(&self, window: xproto::Window) -> Result<Cookie<'_, Self, XIGetClientPointerReply>, ConnectionError>
    {
        xi_get_client_pointer(self, window)
    }

    fn xinput_xi_select_events<'c>(&'c self, window: xproto::Window, masks: &[EventMask]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        xi_select_events(self, window, masks)
    }

    fn xinput_xi_query_version(&self, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Self, XIQueryVersionReply>, ConnectionError>
    {
        xi_query_version(self, major_version, minor_version)
    }

    fn xinput_xi_query_device(&self, deviceid: DeviceId) -> Result<Cookie<'_, Self, XIQueryDeviceReply>, ConnectionError>
    {
        xi_query_device(self, deviceid)
    }

    fn xinput_xi_set_focus(&self, window: xproto::Window, time: xproto::Timestamp, deviceid: DeviceId) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        xi_set_focus(self, window, time, deviceid)
    }

    fn xinput_xi_get_focus(&self, deviceid: DeviceId) -> Result<Cookie<'_, Self, XIGetFocusReply>, ConnectionError>
    {
        xi_get_focus(self, deviceid)
    }

    fn xinput_xi_grab_device<'c, A, B, C>(&'c self, window: xproto::Window, time: xproto::Timestamp, cursor: xproto::Cursor, deviceid: DeviceId, mode: A, paired_device_mode: B, owner_events: C, mask: &[u32]) -> Result<Cookie<'c, Self, XIGrabDeviceReply>, ConnectionError>
    where A: Into<u8>, B: Into<u8>, C: Into<bool>
    {
        xi_grab_device(self, window, time, cursor, deviceid, mode, paired_device_mode, owner_events, mask)
    }

    fn xinput_xi_ungrab_device(&self, time: xproto::Timestamp, deviceid: DeviceId) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        xi_ungrab_device(self, time, deviceid)
    }

    fn xinput_xi_allow_events<A>(&self, time: xproto::Timestamp, deviceid: DeviceId, event_mode: A, touchid: u32, grab_window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        xi_allow_events(self, time, deviceid, event_mode, touchid, grab_window)
    }

    fn xinput_xi_passive_grab_device<'c, A, B, C, D>(&'c self, time: xproto::Timestamp, grab_window: xproto::Window, cursor: xproto::Cursor, detail: u32, deviceid: DeviceId, grab_type: A, grab_mode: B, paired_device_mode: C, owner_events: D, mask: &[u32], modifiers: &[u32]) -> Result<Cookie<'c, Self, XIPassiveGrabDeviceReply>, ConnectionError>
    where A: Into<u8>, B: Into<u8>, C: Into<u8>, D: Into<bool>
    {
        xi_passive_grab_device(self, time, grab_window, cursor, detail, deviceid, grab_type, grab_mode, paired_device_mode, owner_events, mask, modifiers)
    }

    fn xinput_xi_passive_ungrab_device<'c, A>(&'c self, grab_window: xproto::Window, detail: u32, deviceid: DeviceId, grab_type: A, modifiers: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        xi_passive_ungrab_device(self, grab_window, detail, deviceid, grab_type, modifiers)
    }

    fn xinput_xi_list_properties(&self, deviceid: DeviceId) -> Result<Cookie<'_, Self, XIListPropertiesReply>, ConnectionError>
    {
        xi_list_properties(self, deviceid)
    }

    fn xinput_xi_change_property<'c, A>(&'c self, deviceid: DeviceId, mode: A, property: xproto::Atom, type_: xproto::Atom, num_items: u32, items: &XIChangePropertyAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        xi_change_property(self, deviceid, mode, property, type_, num_items, items)
    }

    fn xinput_xi_delete_property(&self, deviceid: DeviceId, property: xproto::Atom) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        xi_delete_property(self, deviceid, property)
    }

    fn xinput_xi_get_property(&self, deviceid: DeviceId, delete: bool, property: xproto::Atom, type_: xproto::Atom, offset: u32, len: u32) -> Result<Cookie<'_, Self, XIGetPropertyReply>, ConnectionError>
    {
        xi_get_property(self, deviceid, delete, property, type_, offset, len)
    }

    fn xinput_xi_get_selected_events(&self, window: xproto::Window) -> Result<Cookie<'_, Self, XIGetSelectedEventsReply>, ConnectionError>
    {
        xi_get_selected_events(self, window)
    }

    fn xinput_xi_barrier_release_pointer<'c>(&'c self, barriers: &[BarrierReleasePointerInfo]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        xi_barrier_release_pointer(self, barriers)
    }

    fn xinput_send_extension_event<'c>(&'c self, destination: xproto::Window, device_id: u8, propagate: bool, events: &[EventForSend], classes: &[EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        send_extension_event(self, destination, device_id, propagate, events, classes)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
