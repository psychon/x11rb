// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Input` X11 extension.

#![allow(clippy::too_many_arguments)]

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
use crate::{BufWithFds, PiecewiseBuf};
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
#[allow(unused_imports)]
use super::xfixes;
#[allow(unused_imports)]
use super::xproto;

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for Fp3232 {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod fp3232 {
    use super::Fp3232;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for Fp3232 {
        fn generate(rng: &Rng) -> Self {
            Self {
                integral: GenRandom::generate(rng),
                frac: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(50979600000);
        let value = Fp3232::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetExtensionVersion request
pub const GET_EXTENSION_VERSION_REQUEST: u8 = 1;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetExtensionVersionRequest<'input> {
    pub name: Cow<'input, [u8]>,
}
impl<'input> GetExtensionVersionRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let name_len = u16::try_from(self.name.len()).expect("`name` has too many elements");
        let name_len_bytes = name_len.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_EXTENSION_VERSION_REQUEST,
            0,
            0,
            name_len_bytes[0],
            name_len_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.name.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.name, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_EXTENSION_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (name_len, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(GetExtensionVersionRequest {
            name: Cow::Borrowed(name),
        })
    }
    /// Clone all borrowed data in this GetExtensionVersionRequest.
    pub fn into_owned(self) -> GetExtensionVersionRequest<'static> {
        GetExtensionVersionRequest {
            name: Cow::Owned(self.name.into_owned()),
        }
    }
}
impl<'input> Request for GetExtensionVersionRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for GetExtensionVersionRequest<'input> {
    type Reply = GetExtensionVersionReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetExtensionVersionReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
    pub present: bool,
}
impl TryParse for GetExtensionVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        let (present, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(19..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetExtensionVersionReply { xi_reply_type, sequence, length, server_major, server_minor, present };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetExtensionVersionReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let server_major_bytes = self.server_major.serialize();
        let server_minor_bytes = self.server_minor.serialize();
        let present_bytes = self.present.serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            server_major_bytes[0],
            server_major_bytes[1],
            server_minor_bytes[0],
            server_minor_bytes[1],
            present_bytes[0],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.server_major.serialize_into(bytes);
        self.server_minor.serialize_into(bytes);
        self.present.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 19]);
    }
}
#[cfg(test)]
mod get_extension_version_reply {
    use super::GetExtensionVersionReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetExtensionVersionReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                server_major: GenRandom::generate(rng),
                server_minor: GenRandom::generate(rng),
                present: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(943893896866824192);
        let value = GetExtensionVersionReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceUse(u8);
impl DeviceUse {
    pub const IS_X_POINTER: Self = Self(0);
    pub const IS_X_KEYBOARD: Self = Self(1);
    pub const IS_X_EXTENSION_DEVICE: Self = Self(2);
    pub const IS_X_EXTENSION_KEYBOARD: Self = Self(3);
    pub const IS_X_EXTENSION_POINTER: Self = Self(4);
}
impl From<DeviceUse> for u8 {
    #[inline]
    fn from(input: DeviceUse) -> Self {
        input.0
    }
}
impl From<DeviceUse> for Option<u8> {
    #[inline]
    fn from(input: DeviceUse) -> Self {
        Some(input.0)
    }
}
impl From<DeviceUse> for u16 {
    #[inline]
    fn from(input: DeviceUse) -> Self {
        u16::from(input.0)
    }
}
impl From<DeviceUse> for Option<u16> {
    #[inline]
    fn from(input: DeviceUse) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<DeviceUse> for u32 {
    #[inline]
    fn from(input: DeviceUse) -> Self {
        u32::from(input.0)
    }
}
impl From<DeviceUse> for Option<u32> {
    #[inline]
    fn from(input: DeviceUse) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for DeviceUse {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for DeviceUse  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::IS_X_POINTER.0.into(), "IS_X_POINTER", "IsXPointer"),
            (Self::IS_X_KEYBOARD.0.into(), "IS_X_KEYBOARD", "IsXKeyboard"),
            (Self::IS_X_EXTENSION_DEVICE.0.into(), "IS_X_EXTENSION_DEVICE", "IsXExtensionDevice"),
            (Self::IS_X_EXTENSION_KEYBOARD.0.into(), "IS_X_EXTENSION_KEYBOARD", "IsXExtensionKeyboard"),
            (Self::IS_X_EXTENSION_POINTER.0.into(), "IS_X_EXTENSION_POINTER", "IsXExtensionPointer"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for DeviceUse {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::IS_X_POINTER.0,
            Self::IS_X_KEYBOARD.0,
            Self::IS_X_EXTENSION_DEVICE.0,
            Self::IS_X_EXTENSION_KEYBOARD.0,
            Self::IS_X_EXTENSION_POINTER.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputClass(u8);
impl InputClass {
    pub const KEY: Self = Self(0);
    pub const BUTTON: Self = Self(1);
    pub const VALUATOR: Self = Self(2);
    pub const FEEDBACK: Self = Self(3);
    pub const PROXIMITY: Self = Self(4);
    pub const FOCUS: Self = Self(5);
    pub const OTHER: Self = Self(6);
}
impl From<InputClass> for u8 {
    #[inline]
    fn from(input: InputClass) -> Self {
        input.0
    }
}
impl From<InputClass> for Option<u8> {
    #[inline]
    fn from(input: InputClass) -> Self {
        Some(input.0)
    }
}
impl From<InputClass> for u16 {
    #[inline]
    fn from(input: InputClass) -> Self {
        u16::from(input.0)
    }
}
impl From<InputClass> for Option<u16> {
    #[inline]
    fn from(input: InputClass) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<InputClass> for u32 {
    #[inline]
    fn from(input: InputClass) -> Self {
        u32::from(input.0)
    }
}
impl From<InputClass> for Option<u32> {
    #[inline]
    fn from(input: InputClass) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for InputClass {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for InputClass  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::KEY.0.into(), "KEY", "Key"),
            (Self::BUTTON.0.into(), "BUTTON", "Button"),
            (Self::VALUATOR.0.into(), "VALUATOR", "Valuator"),
            (Self::FEEDBACK.0.into(), "FEEDBACK", "Feedback"),
            (Self::PROXIMITY.0.into(), "PROXIMITY", "Proximity"),
            (Self::FOCUS.0.into(), "FOCUS", "Focus"),
            (Self::OTHER.0.into(), "OTHER", "Other"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for InputClass {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::KEY.0,
            Self::BUTTON.0,
            Self::VALUATOR.0,
            Self::FEEDBACK.0,
            Self::PROXIMITY.0,
            Self::FOCUS.0,
            Self::OTHER.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValuatorMode(u8);
impl ValuatorMode {
    pub const RELATIVE: Self = Self(0);
    pub const ABSOLUTE: Self = Self(1);
}
impl From<ValuatorMode> for u8 {
    #[inline]
    fn from(input: ValuatorMode) -> Self {
        input.0
    }
}
impl From<ValuatorMode> for Option<u8> {
    #[inline]
    fn from(input: ValuatorMode) -> Self {
        Some(input.0)
    }
}
impl From<ValuatorMode> for u16 {
    #[inline]
    fn from(input: ValuatorMode) -> Self {
        u16::from(input.0)
    }
}
impl From<ValuatorMode> for Option<u16> {
    #[inline]
    fn from(input: ValuatorMode) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ValuatorMode> for u32 {
    #[inline]
    fn from(input: ValuatorMode) -> Self {
        u32::from(input.0)
    }
}
impl From<ValuatorMode> for Option<u32> {
    #[inline]
    fn from(input: ValuatorMode) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ValuatorMode {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ValuatorMode  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::RELATIVE.0.into(), "RELATIVE", "Relative"),
            (Self::ABSOLUTE.0.into(), "ABSOLUTE", "Absolute"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for ValuatorMode {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::RELATIVE.0,
            Self::ABSOLUTE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let device_use = device_use.into();
        let result = DeviceInfo { device_type, device_id, num_class_info, device_use };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceInfo {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let device_type_bytes = self.device_type.serialize();
        let device_id_bytes = self.device_id.serialize();
        let num_class_info_bytes = self.num_class_info.serialize();
        let device_use_bytes = u8::from(self.device_use).serialize();
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
        u8::from(self.device_use).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
    }
}
#[cfg(test)]
mod device_info {
    use super::DeviceInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                device_type: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                num_class_info: GenRandom::generate(rng),
                device_use: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3569531762545610336);
        let value = DeviceInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let class_id = class_id.into();
        let result = KeyInfo { class_id, len, min_keycode, max_keycode, num_keys };
        Ok((result, remaining))
    }
}
impl Serialize for KeyInfo {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.min_keycode.serialize_into(bytes);
        self.max_keycode.serialize_into(bytes);
        self.num_keys.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}
#[cfg(test)]
mod key_info {
    use super::KeyInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for KeyInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                min_keycode: GenRandom::generate(rng),
                max_keycode: GenRandom::generate(rng),
                num_keys: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(83331021064500);
        let value = KeyInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let class_id = class_id.into();
        let result = ButtonInfo { class_id, len, num_buttons };
        Ok((result, remaining))
    }
}
impl Serialize for ButtonInfo {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.num_buttons.serialize_into(bytes);
    }
}
#[cfg(test)]
mod button_info {
    use super::ButtonInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ButtonInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                num_buttons: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4664897331351285504);
        let value = ButtonInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for AxisInfo {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
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
#[cfg(test)]
mod axis_info {
    use super::AxisInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for AxisInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                resolution: GenRandom::generate(rng),
                minimum: GenRandom::generate(rng),
                maximum: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8562891437100000);
        let value = AxisInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let (axes, remaining) = crate::x11_utils::parse_list::<AxisInfo>(remaining, axes_len.try_to_usize()?)?;
        let class_id = class_id.into();
        let mode = mode.into();
        let result = ValuatorInfo { class_id, len, mode, motion_size, axes };
        Ok((result, remaining))
    }
}
impl Serialize for ValuatorInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        let axes_len = u8::try_from(self.axes.len()).expect("`axes` has too many elements");
        axes_len.serialize_into(bytes);
        u8::from(self.mode).serialize_into(bytes);
        self.motion_size.serialize_into(bytes);
        self.axes.serialize_into(bytes);
    }
}
impl ValuatorInfo {
    /// Get the value of the `axes_len` field.
    ///
    /// The `axes_len` field is used as the length field of the `axes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn axes_len(&self) -> u8 {
        self.axes.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod valuator_info {
    use super::ValuatorInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ValuatorInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                mode: GenRandom::generate(rng),
                motion_size: GenRandom::generate(rng),
                axes: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2233853750468803840);
        let value = ValuatorInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let result = InputInfoInfoKey { min_keycode, max_keycode, num_keys };
        Ok((result, remaining))
    }
}
impl Serialize for InputInfoInfoKey {
    type Bytes = [u8; 6];
    fn serialize(&self) -> [u8; 6] {
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
#[cfg(test)]
mod input_info_info_key {
    use super::InputInfoInfoKey;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputInfoInfoKey {
        fn generate(rng: &Rng) -> Self {
            Self {
                min_keycode: GenRandom::generate(rng),
                max_keycode: GenRandom::generate(rng),
                num_keys: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14688433154476308480);
        let value = InputInfoInfoKey::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputInfoInfoButton {
    pub num_buttons: u16,
}
impl TryParse for InputInfoInfoButton {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_buttons, remaining) = u16::try_parse(remaining)?;
        let result = InputInfoInfoButton { num_buttons };
        Ok((result, remaining))
    }
}
impl Serialize for InputInfoInfoButton {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let num_buttons_bytes = self.num_buttons.serialize();
        [
            num_buttons_bytes[0],
            num_buttons_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.num_buttons.serialize_into(bytes);
    }
}
#[cfg(test)]
mod input_info_info_button {
    use super::InputInfoInfoButton;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputInfoInfoButton {
        fn generate(rng: &Rng) -> Self {
            Self {
                num_buttons: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(463264665377046528);
        let value = InputInfoInfoButton::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let (axes, remaining) = crate::x11_utils::parse_list::<AxisInfo>(remaining, axes_len.try_to_usize()?)?;
        let mode = mode.into();
        let result = InputInfoInfoValuator { mode, motion_size, axes };
        Ok((result, remaining))
    }
}
impl Serialize for InputInfoInfoValuator {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(6);
        let axes_len = u8::try_from(self.axes.len()).expect("`axes` has too many elements");
        axes_len.serialize_into(bytes);
        u8::from(self.mode).serialize_into(bytes);
        self.motion_size.serialize_into(bytes);
        self.axes.serialize_into(bytes);
    }
}
impl InputInfoInfoValuator {
    /// Get the value of the `axes_len` field.
    ///
    /// The `axes_len` field is used as the length field of the `axes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn axes_len(&self) -> u8 {
        self.axes.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod input_info_info_valuator {
    use super::InputInfoInfoValuator;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputInfoInfoValuator {
        fn generate(rng: &Rng) -> Self {
            Self {
                mode: GenRandom::generate(rng),
                motion_size: GenRandom::generate(rng),
                axes: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14608188777479143424);
        let value = InputInfoInfoValuator::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputInfoInfo {
    Key(InputInfoInfoKey),
    Button(InputInfoInfoButton),
    Valuator(InputInfoInfoValuator),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u8),
}
impl InputInfoInfo {
    fn try_parse(value: &[u8], class_id: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = class_id;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u8::from(InputClass::KEY) {
            let (key, new_remaining) = InputInfoInfoKey::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputInfoInfo::Key(key));
        }
        if switch_expr == u8::from(InputClass::BUTTON) {
            let (button, new_remaining) = InputInfoInfoButton::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputInfoInfo::Button(button));
        }
        if switch_expr == u8::from(InputClass::VALUATOR) {
            let (valuator, new_remaining) = InputInfoInfoValuator::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputInfoInfo::Valuator(valuator));
        }
        match parse_result {
            None => Ok((InputInfoInfo::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl InputInfoInfo {
    pub fn as_key(&self) -> Option<&InputInfoInfoKey> {
        match self {
            InputInfoInfo::Key(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_button(&self) -> Option<&InputInfoInfoButton> {
        match self {
            InputInfoInfo::Button(value) => Some(value),
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
impl InputInfoInfo {
    #[allow(dead_code)]
    fn serialize(&self, class_id: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, class_id);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, class_id: u8) {
        let _ = class_id;
        match self {
            InputInfoInfo::Key(key) => key.serialize_into(bytes),
            InputInfoInfo::Button(button) => button.serialize_into(bytes),
            InputInfoInfo::Valuator(valuator) => valuator.serialize_into(bytes),
            InputInfoInfo::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl InputInfoInfo {
    fn switch_expr(&self) -> u8 {
        match self {
            InputInfoInfo::Key(_) => u8::from(InputClass::KEY),
            InputInfoInfo::Button(_) => u8::from(InputClass::BUTTON),
            InputInfoInfo::Valuator(_) => u8::from(InputClass::VALUATOR),
            InputInfoInfo::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod input_info_info {
    use super::InputInfoInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputInfoInfo {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..3) {
                0 => Self::Key(GenRandom::generate(rng)),
                1 => Self::Button(GenRandom::generate(rng)),
                _ => Self::Valuator(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<InputInfoInfo> {
        alloc::vec![
            InputInfoInfo::Key(GenRandom::generate(rng)),
            InputInfoInfo::Button(GenRandom::generate(rng)),
            InputInfoInfo::Valuator(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputInfo {
    pub len: u8,
    pub info: InputInfoInfo,
}
impl TryParse for InputInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (info, remaining) = InputInfoInfo::try_parse(remaining, class_id)?;
        let result = InputInfo { len, info };
        Ok((result, remaining))
    }
}
impl Serialize for InputInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        let class_id: u8 = self.info.switch_expr();
        class_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.info.serialize_into(bytes, class_id);
    }
}
#[cfg(test)]
mod input_info {
    use super::InputInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                len: GenRandom::generate(rng),
                info: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1109726903071987200);
        let value = InputInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceName {
    pub string: Vec<u8>,
}
impl TryParse for DeviceName {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (len, remaining) = u8::try_parse(remaining)?;
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, len.try_to_usize()?)?;
        let string = string.to_vec();
        let result = DeviceName { string };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceName {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        let len = u8::try_from(self.string.len()).expect("`string` has too many elements");
        len.serialize_into(bytes);
        bytes.extend_from_slice(&self.string);
    }
}
#[allow(clippy::len_without_is_empty)] // This is not a container and is_empty() makes no sense
impl DeviceName {
    /// Get the value of the `len` field.
    ///
    /// The `len` field is used as the length field of the `string` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn len(&self) -> u8 {
        self.string.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod device_name {
    use super::DeviceName;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceName {
        fn generate(rng: &Rng) -> Self {
            Self {
                string: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15531389385572380272);
        let value = DeviceName::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ListInputDevices request
pub const LIST_INPUT_DEVICES_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListInputDevicesRequest;
impl ListInputDevicesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            LIST_INPUT_DEVICES_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_INPUT_DEVICES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(ListInputDevicesRequest
        )
    }
}
impl Request for ListInputDevicesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for ListInputDevicesRequest {
    type Reply = ListInputDevicesReply;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListInputDevicesReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub devices: Vec<DeviceInfo>,
    pub infos: Vec<InputInfo>,
    pub names: Vec<xproto::Str>,
}
impl TryParse for ListInputDevicesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (devices_len, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        let (devices, remaining) = crate::x11_utils::parse_list::<DeviceInfo>(remaining, devices_len.try_to_usize()?)?;
        let (infos, remaining) = crate::x11_utils::parse_list::<InputInfo>(remaining, devices.iter().try_fold(0u32, |acc, x| acc.checked_add(u32::from(x.num_class_info)).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let (names, remaining) = crate::x11_utils::parse_list::<xproto::Str>(remaining, devices_len.try_to_usize()?)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListInputDevicesReply { xi_reply_type, sequence, length, devices, infos, names };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ListInputDevicesReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let devices_len = u8::try_from(self.devices.len()).expect("`devices` has too many elements");
        devices_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
        self.devices.serialize_into(bytes);
        assert_eq!(self.infos.len(), usize::try_from(self.devices.iter().fold(0u32, |acc, x| acc.checked_add(u32::from(x.num_class_info)).unwrap())).unwrap(), "`infos` has an incorrect length");
        self.infos.serialize_into(bytes);
        assert_eq!(self.names.len(), usize::try_from(devices_len).unwrap(), "`names` has an incorrect length");
        self.names.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl ListInputDevicesReply {
    /// Get the value of the `devices_len` field.
    ///
    /// The `devices_len` field is used as the length field of the `devices` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn devices_len(&self) -> u8 {
        self.devices.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod list_input_devices_reply {
    use super::ListInputDevicesReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ListInputDevicesReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                devices: GenRandom::generate(rng),
                infos: GenRandom::generate(rng),
                names: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3227960779179294720);
        let value = ListInputDevicesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

pub type EventTypeBase = u8;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputClassInfo {
    pub class_id: InputClass,
    pub event_type_base: EventTypeBase,
}
impl TryParse for InputClassInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (event_type_base, remaining) = EventTypeBase::try_parse(remaining)?;
        let class_id = class_id.into();
        let result = InputClassInfo { class_id, event_type_base };
        Ok((result, remaining))
    }
}
impl Serialize for InputClassInfo {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let class_id_bytes = u8::from(self.class_id).serialize();
        let event_type_base_bytes = self.event_type_base.serialize();
        [
            class_id_bytes[0],
            event_type_base_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        u8::from(self.class_id).serialize_into(bytes);
        self.event_type_base.serialize_into(bytes);
    }
}
#[cfg(test)]
mod input_class_info {
    use super::InputClassInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputClassInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                event_type_base: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10612553281813059584);
        let value = InputClassInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the OpenDevice request
pub const OPEN_DEVICE_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenDeviceRequest {
    pub device_id: u8,
}
impl OpenDeviceRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            OPEN_DEVICE_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != OPEN_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(OpenDeviceRequest {
            device_id,
        })
    }
}
impl Request for OpenDeviceRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for OpenDeviceRequest {
    type Reply = OpenDeviceReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenDeviceReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub class_info: Vec<InputClassInfo>,
}
impl TryParse for OpenDeviceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_classes, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        let (class_info, remaining) = crate::x11_utils::parse_list::<InputClassInfo>(remaining, num_classes.try_to_usize()?)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = OpenDeviceReply { xi_reply_type, sequence, length, class_info };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for OpenDeviceReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_classes = u8::try_from(self.class_info.len()).expect("`class_info` has too many elements");
        num_classes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
        self.class_info.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl OpenDeviceReply {
    /// Get the value of the `num_classes` field.
    ///
    /// The `num_classes` field is used as the length field of the `class_info` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_classes(&self) -> u8 {
        self.class_info.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod open_device_reply {
    use super::OpenDeviceReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for OpenDeviceReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                class_info: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2777447594204168192);
        let value = OpenDeviceReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the CloseDevice request
pub const CLOSE_DEVICE_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CloseDeviceRequest {
    pub device_id: u8,
}
impl CloseDeviceRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            CLOSE_DEVICE_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CLOSE_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(CloseDeviceRequest {
            device_id,
        })
    }
}
impl Request for CloseDeviceRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CloseDeviceRequest {
}

/// Opcode for the SetDeviceMode request
pub const SET_DEVICE_MODE_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceModeRequest {
    pub device_id: u8,
    pub mode: ValuatorMode,
}
impl SetDeviceModeRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_MODE_REQUEST,
            0,
            0,
            device_id_bytes[0],
            mode_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_MODE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let mode = mode.into();
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(SetDeviceModeRequest {
            device_id,
            mode,
        })
    }
}
impl Request for SetDeviceModeRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for SetDeviceModeRequest {
    type Reply = SetDeviceModeReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceModeReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl TryParse for SetDeviceModeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.into();
        let result = SetDeviceModeReply { xi_reply_type, sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for SetDeviceModeReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = u8::from(self.status).serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod set_device_mode_reply {
    use super::SetDeviceModeReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for SetDeviceModeReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10542920688995745792);
        let value = SetDeviceModeReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SelectExtensionEvent request
pub const SELECT_EXTENSION_EVENT_REQUEST: u8 = 6;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectExtensionEventRequest<'input> {
    pub window: xproto::Window,
    pub classes: Cow<'input, [EventClass]>,
}
impl<'input> SelectExtensionEventRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        let num_classes_bytes = num_classes.serialize();
        let mut request0 = vec![
            major_opcode,
            SELECT_EXTENSION_EVENT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            num_classes_bytes[0],
            num_classes_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let classes_bytes = self.classes.serialize();
        let length_so_far = length_so_far + classes_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), classes_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SELECT_EXTENSION_EVENT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_classes.try_to_usize()?)?;
        let _ = remaining;
        Ok(SelectExtensionEventRequest {
            window,
            classes: Cow::Owned(classes),
        })
    }
    /// Clone all borrowed data in this SelectExtensionEventRequest.
    pub fn into_owned(self) -> SelectExtensionEventRequest<'static> {
        SelectExtensionEventRequest {
            window: self.window,
            classes: Cow::Owned(self.classes.into_owned()),
        }
    }
}
impl<'input> Request for SelectExtensionEventRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SelectExtensionEventRequest<'input> {
}

/// Opcode for the GetSelectedExtensionEvents request
pub const GET_SELECTED_EXTENSION_EVENTS_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectedExtensionEventsRequest {
    pub window: xproto::Window,
}
impl GetSelectedExtensionEventsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_SELECTED_EXTENSION_EVENTS_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SELECTED_EXTENSION_EVENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetSelectedExtensionEventsRequest {
            window,
        })
    }
}
impl Request for GetSelectedExtensionEventsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetSelectedExtensionEventsRequest {
    type Reply = GetSelectedExtensionEventsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectedExtensionEventsReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub this_classes: Vec<EventClass>,
    pub all_classes: Vec<EventClass>,
}
impl TryParse for GetSelectedExtensionEventsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_this_classes, remaining) = u16::try_parse(remaining)?;
        let (num_all_classes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (this_classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_this_classes.try_to_usize()?)?;
        let (all_classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_all_classes.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectedExtensionEventsReply { xi_reply_type, sequence, length, this_classes, all_classes };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetSelectedExtensionEventsReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_this_classes = u16::try_from(self.this_classes.len()).expect("`this_classes` has too many elements");
        num_this_classes.serialize_into(bytes);
        let num_all_classes = u16::try_from(self.all_classes.len()).expect("`all_classes` has too many elements");
        num_all_classes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.this_classes.serialize_into(bytes);
        self.all_classes.serialize_into(bytes);
    }
}
impl GetSelectedExtensionEventsReply {
    /// Get the value of the `num_this_classes` field.
    ///
    /// The `num_this_classes` field is used as the length field of the `this_classes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_this_classes(&self) -> u16 {
        self.this_classes.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_all_classes` field.
    ///
    /// The `num_all_classes` field is used as the length field of the `all_classes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_all_classes(&self) -> u16 {
        self.all_classes.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_selected_extension_events_reply {
    use super::GetSelectedExtensionEventsReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetSelectedExtensionEventsReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                this_classes: GenRandom::generate(rng),
                all_classes: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8369154605514227712);
        let value = GetSelectedExtensionEventsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PropagateMode(u8);
impl PropagateMode {
    pub const ADD_TO_LIST: Self = Self(0);
    pub const DELETE_FROM_LIST: Self = Self(1);
}
impl From<PropagateMode> for u8 {
    #[inline]
    fn from(input: PropagateMode) -> Self {
        input.0
    }
}
impl From<PropagateMode> for Option<u8> {
    #[inline]
    fn from(input: PropagateMode) -> Self {
        Some(input.0)
    }
}
impl From<PropagateMode> for u16 {
    #[inline]
    fn from(input: PropagateMode) -> Self {
        u16::from(input.0)
    }
}
impl From<PropagateMode> for Option<u16> {
    #[inline]
    fn from(input: PropagateMode) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<PropagateMode> for u32 {
    #[inline]
    fn from(input: PropagateMode) -> Self {
        u32::from(input.0)
    }
}
impl From<PropagateMode> for Option<u32> {
    #[inline]
    fn from(input: PropagateMode) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for PropagateMode {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for PropagateMode  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ADD_TO_LIST.0.into(), "ADD_TO_LIST", "AddToList"),
            (Self::DELETE_FROM_LIST.0.into(), "DELETE_FROM_LIST", "DeleteFromList"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for PropagateMode {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ADD_TO_LIST.0,
            Self::DELETE_FROM_LIST.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the ChangeDeviceDontPropagateList request
pub const CHANGE_DEVICE_DONT_PROPAGATE_LIST_REQUEST: u8 = 8;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDeviceDontPropagateListRequest<'input> {
    pub window: xproto::Window,
    pub mode: PropagateMode,
    pub classes: Cow<'input, [EventClass]>,
}
impl<'input> ChangeDeviceDontPropagateListRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        let num_classes_bytes = num_classes.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_DEVICE_DONT_PROPAGATE_LIST_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            num_classes_bytes[0],
            num_classes_bytes[1],
            mode_bytes[0],
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let classes_bytes = self.classes.serialize();
        let length_so_far = length_so_far + classes_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), classes_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_DEVICE_DONT_PROPAGATE_LIST_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let mode = mode.into();
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_classes.try_to_usize()?)?;
        let _ = remaining;
        Ok(ChangeDeviceDontPropagateListRequest {
            window,
            mode,
            classes: Cow::Owned(classes),
        })
    }
    /// Clone all borrowed data in this ChangeDeviceDontPropagateListRequest.
    pub fn into_owned(self) -> ChangeDeviceDontPropagateListRequest<'static> {
        ChangeDeviceDontPropagateListRequest {
            window: self.window,
            mode: self.mode,
            classes: Cow::Owned(self.classes.into_owned()),
        }
    }
}
impl<'input> Request for ChangeDeviceDontPropagateListRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for ChangeDeviceDontPropagateListRequest<'input> {
}

/// Opcode for the GetDeviceDontPropagateList request
pub const GET_DEVICE_DONT_PROPAGATE_LIST_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceDontPropagateListRequest {
    pub window: xproto::Window,
}
impl GetDeviceDontPropagateListRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_DONT_PROPAGATE_LIST_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_DONT_PROPAGATE_LIST_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetDeviceDontPropagateListRequest {
            window,
        })
    }
}
impl Request for GetDeviceDontPropagateListRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceDontPropagateListRequest {
    type Reply = GetDeviceDontPropagateListReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceDontPropagateListReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub classes: Vec<EventClass>,
}
impl TryParse for GetDeviceDontPropagateListReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_classes.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceDontPropagateListReply { xi_reply_type, sequence, length, classes };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceDontPropagateListReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        num_classes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 22]);
        self.classes.serialize_into(bytes);
    }
}
impl GetDeviceDontPropagateListReply {
    /// Get the value of the `num_classes` field.
    ///
    /// The `num_classes` field is used as the length field of the `classes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_classes(&self) -> u16 {
        self.classes.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_device_dont_propagate_list_reply {
    use super::GetDeviceDontPropagateListReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDeviceDontPropagateListReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                classes: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4984399178680500224);
        let value = GetDeviceDontPropagateListReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceTimeCoord {
    pub time: xproto::Timestamp,
    pub axisvalues: Vec<i32>,
}
impl DeviceTimeCoord {
    pub fn try_parse(remaining: &[u8], num_axes: u8) -> Result<(Self, &[u8]), ParseError> {
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<i32>(remaining, num_axes.try_to_usize()?)?;
        let result = DeviceTimeCoord { time, axisvalues };
        Ok((result, remaining))
    }
}
impl DeviceTimeCoord {
    #[allow(dead_code)]
    fn serialize(&self, num_axes: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, num_axes);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, num_axes: u8) {
        self.time.serialize_into(bytes);
        assert_eq!(self.axisvalues.len(), usize::try_from(num_axes).unwrap(), "`axisvalues` has an incorrect length");
        self.axisvalues.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_time_coord {
    use super::DeviceTimeCoord;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceTimeCoord {
        fn generate(rng: &Rng) -> Self {
            Self {
                time: GenRandom::generate(rng),
                axisvalues: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1971480954127386368);
        let value = DeviceTimeCoord::generate(&rng);
        let left = value.serialize(GenRandom::generate(&rng));
        let mut right = alloc::vec![];
        value.serialize_into(&mut right, GenRandom::generate(&rng));
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetDeviceMotionEvents request
pub const GET_DEVICE_MOTION_EVENTS_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceMotionEventsRequest {
    pub start: xproto::Timestamp,
    pub stop: xproto::Timestamp,
    pub device_id: u8,
}
impl GetDeviceMotionEventsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let start_bytes = self.start.serialize();
        let stop_bytes = self.stop.serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_MOTION_EVENTS_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_MOTION_EVENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (start, remaining) = xproto::Timestamp::try_parse(value)?;
        let (stop, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetDeviceMotionEventsRequest {
            start,
            stop,
            device_id,
        })
    }
}
impl Request for GetDeviceMotionEventsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceMotionEventsRequest {
    type Reply = GetDeviceMotionEventsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceMotionEventsReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_axes: u8,
    pub device_mode: ValuatorMode,
    pub events: Vec<DeviceTimeCoord>,
}
impl TryParse for GetDeviceMotionEventsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_events, remaining) = u32::try_parse(remaining)?;
        let (num_axes, remaining) = u8::try_parse(remaining)?;
        let (device_mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::InsufficientData)?;
        let mut remaining = remaining;
        let list_length = num_events.try_to_usize()?;
        let mut events = Vec::with_capacity(list_length);
        for _ in 0..list_length {
            let (v, new_remaining) = DeviceTimeCoord::try_parse(remaining, num_axes)?;
            remaining = new_remaining;
            events.push(v);
        }
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let device_mode = device_mode.into();
        let result = GetDeviceMotionEventsReply { xi_reply_type, sequence, length, num_axes, device_mode, events };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceMotionEventsReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_events = u32::try_from(self.events.len()).expect("`events` has too many elements");
        num_events.serialize_into(bytes);
        self.num_axes.serialize_into(bytes);
        u8::from(self.device_mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 18]);
        for element in self.events.iter() {
            element.serialize_into(bytes, self.num_axes);
        }
    }
}
impl GetDeviceMotionEventsReply {
    /// Get the value of the `num_events` field.
    ///
    /// The `num_events` field is used as the length field of the `events` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_events(&self) -> u32 {
        self.events.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_device_motion_events_reply {
    use super::GetDeviceMotionEventsReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDeviceMotionEventsReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                num_axes: GenRandom::generate(rng),
                device_mode: GenRandom::generate(rng),
                events: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2051627056346169344);
        let value = GetDeviceMotionEventsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ChangeKeyboardDevice request
pub const CHANGE_KEYBOARD_DEVICE_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeKeyboardDeviceRequest {
    pub device_id: u8,
}
impl ChangeKeyboardDeviceRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_KEYBOARD_DEVICE_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_KEYBOARD_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(ChangeKeyboardDeviceRequest {
            device_id,
        })
    }
}
impl Request for ChangeKeyboardDeviceRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for ChangeKeyboardDeviceRequest {
    type Reply = ChangeKeyboardDeviceReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeKeyboardDeviceReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl TryParse for ChangeKeyboardDeviceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.into();
        let result = ChangeKeyboardDeviceReply { xi_reply_type, sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ChangeKeyboardDeviceReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = u8::from(self.status).serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod change_keyboard_device_reply {
    use super::ChangeKeyboardDeviceReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ChangeKeyboardDeviceReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4793584832704413696);
        let value = ChangeKeyboardDeviceReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ChangePointerDevice request
pub const CHANGE_POINTER_DEVICE_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangePointerDeviceRequest {
    pub x_axis: u8,
    pub y_axis: u8,
    pub device_id: u8,
}
impl ChangePointerDeviceRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let x_axis_bytes = self.x_axis.serialize();
        let y_axis_bytes = self.y_axis.serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_POINTER_DEVICE_REQUEST,
            0,
            0,
            x_axis_bytes[0],
            y_axis_bytes[0],
            device_id_bytes[0],
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_POINTER_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (x_axis, remaining) = u8::try_parse(value)?;
        let (y_axis, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(ChangePointerDeviceRequest {
            x_axis,
            y_axis,
            device_id,
        })
    }
}
impl Request for ChangePointerDeviceRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for ChangePointerDeviceRequest {
    type Reply = ChangePointerDeviceReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangePointerDeviceReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl TryParse for ChangePointerDeviceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.into();
        let result = ChangePointerDeviceReply { xi_reply_type, sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ChangePointerDeviceReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = u8::from(self.status).serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod change_pointer_device_reply {
    use super::ChangePointerDeviceReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ChangePointerDeviceReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5263867155522256896);
        let value = ChangePointerDeviceReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GrabDevice request
pub const GRAB_DEVICE_REQUEST: u8 = 13;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrabDeviceRequest<'input> {
    pub grab_window: xproto::Window,
    pub time: xproto::Timestamp,
    pub this_device_mode: xproto::GrabMode,
    pub other_device_mode: xproto::GrabMode,
    pub owner_events: bool,
    pub device_id: u8,
    pub classes: Cow<'input, [EventClass]>,
}
impl<'input> GrabDeviceRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let grab_window_bytes = self.grab_window.serialize();
        let time_bytes = self.time.serialize();
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        let num_classes_bytes = num_classes.serialize();
        let this_device_mode_bytes = u8::from(self.this_device_mode).serialize();
        let other_device_mode_bytes = u8::from(self.other_device_mode).serialize();
        let owner_events_bytes = self.owner_events.serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            GRAB_DEVICE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let classes_bytes = self.classes.serialize();
        let length_so_far = length_so_far + classes_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), classes_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GRAB_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (grab_window, remaining) = xproto::Window::try_parse(value)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let (this_device_mode, remaining) = u8::try_parse(remaining)?;
        let this_device_mode = this_device_mode.into();
        let (other_device_mode, remaining) = u8::try_parse(remaining)?;
        let other_device_mode = other_device_mode.into();
        let (owner_events, remaining) = bool::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_classes.try_to_usize()?)?;
        let _ = remaining;
        Ok(GrabDeviceRequest {
            grab_window,
            time,
            this_device_mode,
            other_device_mode,
            owner_events,
            device_id,
            classes: Cow::Owned(classes),
        })
    }
    /// Clone all borrowed data in this GrabDeviceRequest.
    pub fn into_owned(self) -> GrabDeviceRequest<'static> {
        GrabDeviceRequest {
            grab_window: self.grab_window,
            time: self.time,
            this_device_mode: self.this_device_mode,
            other_device_mode: self.other_device_mode,
            owner_events: self.owner_events,
            device_id: self.device_id,
            classes: Cow::Owned(self.classes.into_owned()),
        }
    }
}
impl<'input> Request for GrabDeviceRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for GrabDeviceRequest<'input> {
    type Reply = GrabDeviceReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrabDeviceReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl TryParse for GrabDeviceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.into();
        let result = GrabDeviceReply { xi_reply_type, sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GrabDeviceReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = u8::from(self.status).serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod grab_device_reply {
    use super::GrabDeviceReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GrabDeviceReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6281887498973409280);
        let value = GrabDeviceReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the UngrabDevice request
pub const UNGRAB_DEVICE_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UngrabDeviceRequest {
    pub time: xproto::Timestamp,
    pub device_id: u8,
}
impl UngrabDeviceRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let time_bytes = self.time.serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            UNGRAB_DEVICE_REQUEST,
            0,
            0,
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != UNGRAB_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (time, remaining) = xproto::Timestamp::try_parse(value)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(UngrabDeviceRequest {
            time,
            device_id,
        })
    }
}
impl Request for UngrabDeviceRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for UngrabDeviceRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifierDevice(u8);
impl ModifierDevice {
    pub const USE_X_KEYBOARD: Self = Self(255);
}
impl From<ModifierDevice> for u8 {
    #[inline]
    fn from(input: ModifierDevice) -> Self {
        input.0
    }
}
impl From<ModifierDevice> for Option<u8> {
    #[inline]
    fn from(input: ModifierDevice) -> Self {
        Some(input.0)
    }
}
impl From<ModifierDevice> for u16 {
    #[inline]
    fn from(input: ModifierDevice) -> Self {
        u16::from(input.0)
    }
}
impl From<ModifierDevice> for Option<u16> {
    #[inline]
    fn from(input: ModifierDevice) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ModifierDevice> for u32 {
    #[inline]
    fn from(input: ModifierDevice) -> Self {
        u32::from(input.0)
    }
}
impl From<ModifierDevice> for Option<u32> {
    #[inline]
    fn from(input: ModifierDevice) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ModifierDevice {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ModifierDevice  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::USE_X_KEYBOARD.0.into(), "USE_X_KEYBOARD", "UseXKeyboard"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for ModifierDevice {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::USE_X_KEYBOARD.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the GrabDeviceKey request
pub const GRAB_DEVICE_KEY_REQUEST: u8 = 15;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrabDeviceKeyRequest<'input> {
    pub grab_window: xproto::Window,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub grabbed_device: u8,
    pub key: u8,
    pub this_device_mode: xproto::GrabMode,
    pub other_device_mode: xproto::GrabMode,
    pub owner_events: bool,
    pub classes: Cow<'input, [EventClass]>,
}
impl<'input> GrabDeviceKeyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let grab_window_bytes = self.grab_window.serialize();
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        let num_classes_bytes = num_classes.serialize();
        let modifiers_bytes = self.modifiers.serialize();
        let modifier_device_bytes = self.modifier_device.serialize();
        let grabbed_device_bytes = self.grabbed_device.serialize();
        let key_bytes = self.key.serialize();
        let this_device_mode_bytes = u8::from(self.this_device_mode).serialize();
        let other_device_mode_bytes = u8::from(self.other_device_mode).serialize();
        let owner_events_bytes = self.owner_events.serialize();
        let mut request0 = vec![
            major_opcode,
            GRAB_DEVICE_KEY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let classes_bytes = self.classes.serialize();
        let length_so_far = length_so_far + classes_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), classes_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GRAB_DEVICE_KEY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (grab_window, remaining) = xproto::Window::try_parse(value)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let (modifiers, remaining) = u16::try_parse(remaining)?;
        let (modifier_device, remaining) = u8::try_parse(remaining)?;
        let (grabbed_device, remaining) = u8::try_parse(remaining)?;
        let (key, remaining) = u8::try_parse(remaining)?;
        let (this_device_mode, remaining) = u8::try_parse(remaining)?;
        let this_device_mode = this_device_mode.into();
        let (other_device_mode, remaining) = u8::try_parse(remaining)?;
        let other_device_mode = other_device_mode.into();
        let (owner_events, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_classes.try_to_usize()?)?;
        let _ = remaining;
        Ok(GrabDeviceKeyRequest {
            grab_window,
            modifiers,
            modifier_device,
            grabbed_device,
            key,
            this_device_mode,
            other_device_mode,
            owner_events,
            classes: Cow::Owned(classes),
        })
    }
    /// Clone all borrowed data in this GrabDeviceKeyRequest.
    pub fn into_owned(self) -> GrabDeviceKeyRequest<'static> {
        GrabDeviceKeyRequest {
            grab_window: self.grab_window,
            modifiers: self.modifiers,
            modifier_device: self.modifier_device,
            grabbed_device: self.grabbed_device,
            key: self.key,
            this_device_mode: self.this_device_mode,
            other_device_mode: self.other_device_mode,
            owner_events: self.owner_events,
            classes: Cow::Owned(self.classes.into_owned()),
        }
    }
}
impl<'input> Request for GrabDeviceKeyRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for GrabDeviceKeyRequest<'input> {
}

/// Opcode for the UngrabDeviceKey request
pub const UNGRAB_DEVICE_KEY_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UngrabDeviceKeyRequest {
    pub grab_window: xproto::Window,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub key: u8,
    pub grabbed_device: u8,
}
impl UngrabDeviceKeyRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let grab_window_bytes = self.grab_window.serialize();
        let modifiers_bytes = self.modifiers.serialize();
        let modifier_device_bytes = self.modifier_device.serialize();
        let key_bytes = self.key.serialize();
        let grabbed_device_bytes = self.grabbed_device.serialize();
        let mut request0 = vec![
            major_opcode,
            UNGRAB_DEVICE_KEY_REQUEST,
            0,
            0,
            grab_window_bytes[0],
            grab_window_bytes[1],
            grab_window_bytes[2],
            grab_window_bytes[3],
            modifiers_bytes[0],
            modifiers_bytes[1],
            modifier_device_bytes[0],
            key_bytes[0],
            grabbed_device_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != UNGRAB_DEVICE_KEY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (grab_window, remaining) = xproto::Window::try_parse(value)?;
        let (modifiers, remaining) = u16::try_parse(remaining)?;
        let (modifier_device, remaining) = u8::try_parse(remaining)?;
        let (key, remaining) = u8::try_parse(remaining)?;
        let (grabbed_device, remaining) = u8::try_parse(remaining)?;
        let _ = remaining;
        Ok(UngrabDeviceKeyRequest {
            grab_window,
            modifiers,
            modifier_device,
            key,
            grabbed_device,
        })
    }
}
impl Request for UngrabDeviceKeyRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for UngrabDeviceKeyRequest {
}

/// Opcode for the GrabDeviceButton request
pub const GRAB_DEVICE_BUTTON_REQUEST: u8 = 17;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrabDeviceButtonRequest<'input> {
    pub grab_window: xproto::Window,
    pub grabbed_device: u8,
    pub modifier_device: u8,
    pub modifiers: u16,
    pub this_device_mode: xproto::GrabMode,
    pub other_device_mode: xproto::GrabMode,
    pub button: u8,
    pub owner_events: bool,
    pub classes: Cow<'input, [EventClass]>,
}
impl<'input> GrabDeviceButtonRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let grab_window_bytes = self.grab_window.serialize();
        let grabbed_device_bytes = self.grabbed_device.serialize();
        let modifier_device_bytes = self.modifier_device.serialize();
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        let num_classes_bytes = num_classes.serialize();
        let modifiers_bytes = self.modifiers.serialize();
        let this_device_mode_bytes = u8::from(self.this_device_mode).serialize();
        let other_device_mode_bytes = u8::from(self.other_device_mode).serialize();
        let button_bytes = self.button.serialize();
        let owner_events_bytes = self.owner_events.serialize();
        let mut request0 = vec![
            major_opcode,
            GRAB_DEVICE_BUTTON_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let classes_bytes = self.classes.serialize();
        let length_so_far = length_so_far + classes_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), classes_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GRAB_DEVICE_BUTTON_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (grab_window, remaining) = xproto::Window::try_parse(value)?;
        let (grabbed_device, remaining) = u8::try_parse(remaining)?;
        let (modifier_device, remaining) = u8::try_parse(remaining)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let (modifiers, remaining) = u16::try_parse(remaining)?;
        let (this_device_mode, remaining) = u8::try_parse(remaining)?;
        let this_device_mode = this_device_mode.into();
        let (other_device_mode, remaining) = u8::try_parse(remaining)?;
        let other_device_mode = other_device_mode.into();
        let (button, remaining) = u8::try_parse(remaining)?;
        let (owner_events, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_classes.try_to_usize()?)?;
        let _ = remaining;
        Ok(GrabDeviceButtonRequest {
            grab_window,
            grabbed_device,
            modifier_device,
            modifiers,
            this_device_mode,
            other_device_mode,
            button,
            owner_events,
            classes: Cow::Owned(classes),
        })
    }
    /// Clone all borrowed data in this GrabDeviceButtonRequest.
    pub fn into_owned(self) -> GrabDeviceButtonRequest<'static> {
        GrabDeviceButtonRequest {
            grab_window: self.grab_window,
            grabbed_device: self.grabbed_device,
            modifier_device: self.modifier_device,
            modifiers: self.modifiers,
            this_device_mode: self.this_device_mode,
            other_device_mode: self.other_device_mode,
            button: self.button,
            owner_events: self.owner_events,
            classes: Cow::Owned(self.classes.into_owned()),
        }
    }
}
impl<'input> Request for GrabDeviceButtonRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for GrabDeviceButtonRequest<'input> {
}

/// Opcode for the UngrabDeviceButton request
pub const UNGRAB_DEVICE_BUTTON_REQUEST: u8 = 18;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UngrabDeviceButtonRequest {
    pub grab_window: xproto::Window,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub button: u8,
    pub grabbed_device: u8,
}
impl UngrabDeviceButtonRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let grab_window_bytes = self.grab_window.serialize();
        let modifiers_bytes = self.modifiers.serialize();
        let modifier_device_bytes = self.modifier_device.serialize();
        let button_bytes = self.button.serialize();
        let grabbed_device_bytes = self.grabbed_device.serialize();
        let mut request0 = vec![
            major_opcode,
            UNGRAB_DEVICE_BUTTON_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != UNGRAB_DEVICE_BUTTON_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (grab_window, remaining) = xproto::Window::try_parse(value)?;
        let (modifiers, remaining) = u16::try_parse(remaining)?;
        let (modifier_device, remaining) = u8::try_parse(remaining)?;
        let (button, remaining) = u8::try_parse(remaining)?;
        let (grabbed_device, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(UngrabDeviceButtonRequest {
            grab_window,
            modifiers,
            modifier_device,
            button,
            grabbed_device,
        })
    }
}
impl Request for UngrabDeviceButtonRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for UngrabDeviceButtonRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceInputMode(u8);
impl DeviceInputMode {
    pub const ASYNC_THIS_DEVICE: Self = Self(0);
    pub const SYNC_THIS_DEVICE: Self = Self(1);
    pub const REPLAY_THIS_DEVICE: Self = Self(2);
    pub const ASYNC_OTHER_DEVICES: Self = Self(3);
    pub const ASYNC_ALL: Self = Self(4);
    pub const SYNC_ALL: Self = Self(5);
}
impl From<DeviceInputMode> for u8 {
    #[inline]
    fn from(input: DeviceInputMode) -> Self {
        input.0
    }
}
impl From<DeviceInputMode> for Option<u8> {
    #[inline]
    fn from(input: DeviceInputMode) -> Self {
        Some(input.0)
    }
}
impl From<DeviceInputMode> for u16 {
    #[inline]
    fn from(input: DeviceInputMode) -> Self {
        u16::from(input.0)
    }
}
impl From<DeviceInputMode> for Option<u16> {
    #[inline]
    fn from(input: DeviceInputMode) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<DeviceInputMode> for u32 {
    #[inline]
    fn from(input: DeviceInputMode) -> Self {
        u32::from(input.0)
    }
}
impl From<DeviceInputMode> for Option<u32> {
    #[inline]
    fn from(input: DeviceInputMode) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for DeviceInputMode {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for DeviceInputMode  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ASYNC_THIS_DEVICE.0.into(), "ASYNC_THIS_DEVICE", "AsyncThisDevice"),
            (Self::SYNC_THIS_DEVICE.0.into(), "SYNC_THIS_DEVICE", "SyncThisDevice"),
            (Self::REPLAY_THIS_DEVICE.0.into(), "REPLAY_THIS_DEVICE", "ReplayThisDevice"),
            (Self::ASYNC_OTHER_DEVICES.0.into(), "ASYNC_OTHER_DEVICES", "AsyncOtherDevices"),
            (Self::ASYNC_ALL.0.into(), "ASYNC_ALL", "AsyncAll"),
            (Self::SYNC_ALL.0.into(), "SYNC_ALL", "SyncAll"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for DeviceInputMode {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ASYNC_THIS_DEVICE.0,
            Self::SYNC_THIS_DEVICE.0,
            Self::REPLAY_THIS_DEVICE.0,
            Self::ASYNC_OTHER_DEVICES.0,
            Self::ASYNC_ALL.0,
            Self::SYNC_ALL.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the AllowDeviceEvents request
pub const ALLOW_DEVICE_EVENTS_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllowDeviceEventsRequest {
    pub time: xproto::Timestamp,
    pub mode: DeviceInputMode,
    pub device_id: u8,
}
impl AllowDeviceEventsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let time_bytes = self.time.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            ALLOW_DEVICE_EVENTS_REQUEST,
            0,
            0,
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            mode_bytes[0],
            device_id_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != ALLOW_DEVICE_EVENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (time, remaining) = xproto::Timestamp::try_parse(value)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let mode = mode.into();
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(AllowDeviceEventsRequest {
            time,
            mode,
            device_id,
        })
    }
}
impl Request for AllowDeviceEventsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for AllowDeviceEventsRequest {
}

/// Opcode for the GetDeviceFocus request
pub const GET_DEVICE_FOCUS_REQUEST: u8 = 20;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceFocusRequest {
    pub device_id: u8,
}
impl GetDeviceFocusRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_FOCUS_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_FOCUS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetDeviceFocusRequest {
            device_id,
        })
    }
}
impl Request for GetDeviceFocusRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceFocusRequest {
    type Reply = GetDeviceFocusReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceFocusReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xproto::Window,
    pub time: xproto::Timestamp,
    pub revert_to: xproto::InputFocus,
}
impl TryParse for GetDeviceFocusReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (focus, remaining) = xproto::Window::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (revert_to, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let revert_to = revert_to.into();
        let result = GetDeviceFocusReply { xi_reply_type, sequence, length, focus, time, revert_to };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceFocusReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let focus_bytes = self.focus.serialize();
        let time_bytes = self.time.serialize();
        let revert_to_bytes = u8::from(self.revert_to).serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            focus_bytes[0],
            focus_bytes[1],
            focus_bytes[2],
            focus_bytes[3],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            revert_to_bytes[0],
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.focus.serialize_into(bytes);
        self.time.serialize_into(bytes);
        u8::from(self.revert_to).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 15]);
    }
}
#[cfg(test)]
mod get_device_focus_reply {
    use super::GetDeviceFocusReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDeviceFocusReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                focus: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                revert_to: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7562862494836318208);
        let value = GetDeviceFocusReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetDeviceFocus request
pub const SET_DEVICE_FOCUS_REQUEST: u8 = 21;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceFocusRequest {
    pub focus: xproto::Window,
    pub time: xproto::Timestamp,
    pub revert_to: xproto::InputFocus,
    pub device_id: u8,
}
impl SetDeviceFocusRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let focus_bytes = self.focus.serialize();
        let time_bytes = self.time.serialize();
        let revert_to_bytes = u8::from(self.revert_to).serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_FOCUS_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_FOCUS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (focus, remaining) = xproto::Window::try_parse(value)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (revert_to, remaining) = u8::try_parse(remaining)?;
        let revert_to = revert_to.into();
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(SetDeviceFocusRequest {
            focus,
            time,
            revert_to,
            device_id,
        })
    }
}
impl Request for SetDeviceFocusRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SetDeviceFocusRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackClass(u8);
impl FeedbackClass {
    pub const KEYBOARD: Self = Self(0);
    pub const POINTER: Self = Self(1);
    pub const STRING: Self = Self(2);
    pub const INTEGER: Self = Self(3);
    pub const LED: Self = Self(4);
    pub const BELL: Self = Self(5);
}
impl From<FeedbackClass> for u8 {
    #[inline]
    fn from(input: FeedbackClass) -> Self {
        input.0
    }
}
impl From<FeedbackClass> for Option<u8> {
    #[inline]
    fn from(input: FeedbackClass) -> Self {
        Some(input.0)
    }
}
impl From<FeedbackClass> for u16 {
    #[inline]
    fn from(input: FeedbackClass) -> Self {
        u16::from(input.0)
    }
}
impl From<FeedbackClass> for Option<u16> {
    #[inline]
    fn from(input: FeedbackClass) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<FeedbackClass> for u32 {
    #[inline]
    fn from(input: FeedbackClass) -> Self {
        u32::from(input.0)
    }
}
impl From<FeedbackClass> for Option<u32> {
    #[inline]
    fn from(input: FeedbackClass) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for FeedbackClass {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for FeedbackClass  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::KEYBOARD.0.into(), "KEYBOARD", "Keyboard"),
            (Self::POINTER.0.into(), "POINTER", "Pointer"),
            (Self::STRING.0.into(), "STRING", "String"),
            (Self::INTEGER.0.into(), "INTEGER", "Integer"),
            (Self::LED.0.into(), "LED", "Led"),
            (Self::BELL.0.into(), "BELL", "Bell"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for FeedbackClass {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::KEYBOARD.0,
            Self::POINTER.0,
            Self::STRING.0,
            Self::INTEGER.0,
            Self::LED.0,
            Self::BELL.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (auto_repeats, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let auto_repeats = <[u8; 32]>::try_from(auto_repeats).unwrap();
        let class_id = class_id.into();
        let result = KbdFeedbackState { class_id, feedback_id, len, pitch, duration, led_mask, led_values, global_auto_repeat, click, percent, auto_repeats };
        Ok((result, remaining))
    }
}
impl Serialize for KbdFeedbackState {
    type Bytes = [u8; 52];
    fn serialize(&self) -> [u8; 52] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
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
        bytes.extend_from_slice(&self.auto_repeats);
    }
}
#[cfg(test)]
mod kbd_feedback_state {
    use super::KbdFeedbackState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for KbdFeedbackState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                pitch: GenRandom::generate(rng),
                duration: GenRandom::generate(rng),
                led_mask: GenRandom::generate(rng),
                led_values: GenRandom::generate(rng),
                global_auto_repeat: GenRandom::generate(rng),
                click: GenRandom::generate(rng),
                percent: GenRandom::generate(rng),
                auto_repeats: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(18134747517575067648);
        let value = KbdFeedbackState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (accel_num, remaining) = u16::try_parse(remaining)?;
        let (accel_denom, remaining) = u16::try_parse(remaining)?;
        let (threshold, remaining) = u16::try_parse(remaining)?;
        let class_id = class_id.into();
        let result = PtrFeedbackState { class_id, feedback_id, len, accel_num, accel_denom, threshold };
        Ok((result, remaining))
    }
}
impl Serialize for PtrFeedbackState {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.accel_num.serialize_into(bytes);
        self.accel_denom.serialize_into(bytes);
        self.threshold.serialize_into(bytes);
    }
}
#[cfg(test)]
mod ptr_feedback_state {
    use super::PtrFeedbackState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for PtrFeedbackState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                accel_num: GenRandom::generate(rng),
                accel_denom: GenRandom::generate(rng),
                threshold: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11426114325511045120);
        let value = PtrFeedbackState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let class_id = class_id.into();
        let result = IntegerFeedbackState { class_id, feedback_id, len, resolution, min_value, max_value };
        Ok((result, remaining))
    }
}
impl Serialize for IntegerFeedbackState {
    type Bytes = [u8; 16];
    fn serialize(&self) -> [u8; 16] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.resolution.serialize_into(bytes);
        self.min_value.serialize_into(bytes);
        self.max_value.serialize_into(bytes);
    }
}
#[cfg(test)]
mod integer_feedback_state {
    use super::IntegerFeedbackState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for IntegerFeedbackState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                resolution: GenRandom::generate(rng),
                min_value: GenRandom::generate(rng),
                max_value: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7607233837219745792);
        let value = IntegerFeedbackState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, num_keysyms.try_to_usize()?)?;
        let class_id = class_id.into();
        let result = StringFeedbackState { class_id, feedback_id, len, max_symbols, keysyms };
        Ok((result, remaining))
    }
}
impl Serialize for StringFeedbackState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.max_symbols.serialize_into(bytes);
        let num_keysyms = u16::try_from(self.keysyms.len()).expect("`keysyms` has too many elements");
        num_keysyms.serialize_into(bytes);
        self.keysyms.serialize_into(bytes);
    }
}
impl StringFeedbackState {
    /// Get the value of the `num_keysyms` field.
    ///
    /// The `num_keysyms` field is used as the length field of the `keysyms` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_keysyms(&self) -> u16 {
        self.keysyms.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod string_feedback_state {
    use super::StringFeedbackState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for StringFeedbackState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                max_symbols: GenRandom::generate(rng),
                keysyms: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11663386592670396416);
        let value = StringFeedbackState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (pitch, remaining) = u16::try_parse(remaining)?;
        let (duration, remaining) = u16::try_parse(remaining)?;
        let class_id = class_id.into();
        let result = BellFeedbackState { class_id, feedback_id, len, percent, pitch, duration };
        Ok((result, remaining))
    }
}
impl Serialize for BellFeedbackState {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.percent.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.pitch.serialize_into(bytes);
        self.duration.serialize_into(bytes);
    }
}
#[cfg(test)]
mod bell_feedback_state {
    use super::BellFeedbackState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for BellFeedbackState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                percent: GenRandom::generate(rng),
                pitch: GenRandom::generate(rng),
                duration: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6808983993224929280);
        let value = BellFeedbackState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let class_id = class_id.into();
        let result = LedFeedbackState { class_id, feedback_id, len, led_mask, led_values };
        Ok((result, remaining))
    }
}
impl Serialize for LedFeedbackState {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
    }
}
#[cfg(test)]
mod led_feedback_state {
    use super::LedFeedbackState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for LedFeedbackState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                led_mask: GenRandom::generate(rng),
                led_values: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1425978475994689536);
        let value = LedFeedbackState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (auto_repeats, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let auto_repeats = <[u8; 32]>::try_from(auto_repeats).unwrap();
        let result = FeedbackStateDataKeyboard { pitch, duration, led_mask, led_values, global_auto_repeat, click, percent, auto_repeats };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackStateDataKeyboard {
    type Bytes = [u8; 48];
    fn serialize(&self) -> [u8; 48] {
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
        bytes.extend_from_slice(&self.auto_repeats);
    }
}
#[cfg(test)]
mod feedback_state_data_keyboard {
    use super::FeedbackStateDataKeyboard;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackStateDataKeyboard {
        fn generate(rng: &Rng) -> Self {
            Self {
                pitch: GenRandom::generate(rng),
                duration: GenRandom::generate(rng),
                led_mask: GenRandom::generate(rng),
                led_values: GenRandom::generate(rng),
                global_auto_repeat: GenRandom::generate(rng),
                click: GenRandom::generate(rng),
                percent: GenRandom::generate(rng),
                auto_repeats: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13870198378932011008);
        let value = FeedbackStateDataKeyboard::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackStateDataPointer {
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}
impl TryParse for FeedbackStateDataPointer {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (accel_num, remaining) = u16::try_parse(remaining)?;
        let (accel_denom, remaining) = u16::try_parse(remaining)?;
        let (threshold, remaining) = u16::try_parse(remaining)?;
        let result = FeedbackStateDataPointer { accel_num, accel_denom, threshold };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackStateDataPointer {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod feedback_state_data_pointer {
    use super::FeedbackStateDataPointer;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackStateDataPointer {
        fn generate(rng: &Rng) -> Self {
            Self {
                accel_num: GenRandom::generate(rng),
                accel_denom: GenRandom::generate(rng),
                threshold: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12102824518948487168);
        let value = FeedbackStateDataPointer::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackStateDataString {
    pub max_symbols: u16,
    pub keysyms: Vec<xproto::Keysym>,
}
impl TryParse for FeedbackStateDataString {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (max_symbols, remaining) = u16::try_parse(remaining)?;
        let (num_keysyms, remaining) = u16::try_parse(remaining)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, num_keysyms.try_to_usize()?)?;
        let result = FeedbackStateDataString { max_symbols, keysyms };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackStateDataString {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.max_symbols.serialize_into(bytes);
        let num_keysyms = u16::try_from(self.keysyms.len()).expect("`keysyms` has too many elements");
        num_keysyms.serialize_into(bytes);
        self.keysyms.serialize_into(bytes);
    }
}
impl FeedbackStateDataString {
    /// Get the value of the `num_keysyms` field.
    ///
    /// The `num_keysyms` field is used as the length field of the `keysyms` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_keysyms(&self) -> u16 {
        self.keysyms.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod feedback_state_data_string {
    use super::FeedbackStateDataString;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackStateDataString {
        fn generate(rng: &Rng) -> Self {
            Self {
                max_symbols: GenRandom::generate(rng),
                keysyms: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11327280702552473600);
        let value = FeedbackStateDataString::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for FeedbackStateDataInteger {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
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
#[cfg(test)]
mod feedback_state_data_integer {
    use super::FeedbackStateDataInteger;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackStateDataInteger {
        fn generate(rng: &Rng) -> Self {
            Self {
                resolution: GenRandom::generate(rng),
                min_value: GenRandom::generate(rng),
                max_value: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17533989062234669056);
        let value = FeedbackStateDataInteger::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for FeedbackStateDataLed {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod feedback_state_data_led {
    use super::FeedbackStateDataLed;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackStateDataLed {
        fn generate(rng: &Rng) -> Self {
            Self {
                led_mask: GenRandom::generate(rng),
                led_values: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17222583355082997760);
        let value = FeedbackStateDataLed::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackStateDataBell {
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
}
impl TryParse for FeedbackStateDataBell {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (percent, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (pitch, remaining) = u16::try_parse(remaining)?;
        let (duration, remaining) = u16::try_parse(remaining)?;
        let result = FeedbackStateDataBell { percent, pitch, duration };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackStateDataBell {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod feedback_state_data_bell {
    use super::FeedbackStateDataBell;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackStateDataBell {
        fn generate(rng: &Rng) -> Self {
            Self {
                percent: GenRandom::generate(rng),
                pitch: GenRandom::generate(rng),
                duration: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8973437668175773696);
        let value = FeedbackStateDataBell::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeedbackStateData {
    Keyboard(FeedbackStateDataKeyboard),
    Pointer(FeedbackStateDataPointer),
    String(FeedbackStateDataString),
    Integer(FeedbackStateDataInteger),
    Led(FeedbackStateDataLed),
    Bell(FeedbackStateDataBell),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u8),
}
impl FeedbackStateData {
    fn try_parse(value: &[u8], class_id: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = class_id;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u8::from(FeedbackClass::KEYBOARD) {
            let (keyboard, new_remaining) = FeedbackStateDataKeyboard::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Keyboard(keyboard));
        }
        if switch_expr == u8::from(FeedbackClass::POINTER) {
            let (pointer, new_remaining) = FeedbackStateDataPointer::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Pointer(pointer));
        }
        if switch_expr == u8::from(FeedbackClass::STRING) {
            let (string, new_remaining) = FeedbackStateDataString::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::String(string));
        }
        if switch_expr == u8::from(FeedbackClass::INTEGER) {
            let (integer, new_remaining) = FeedbackStateDataInteger::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Integer(integer));
        }
        if switch_expr == u8::from(FeedbackClass::LED) {
            let (led, new_remaining) = FeedbackStateDataLed::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Led(led));
        }
        if switch_expr == u8::from(FeedbackClass::BELL) {
            let (bell, new_remaining) = FeedbackStateDataBell::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackStateData::Bell(bell));
        }
        match parse_result {
            None => Ok((FeedbackStateData::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl FeedbackStateData {
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
impl FeedbackStateData {
    #[allow(dead_code)]
    fn serialize(&self, class_id: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, class_id);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, class_id: u8) {
        let _ = class_id;
        match self {
            FeedbackStateData::Keyboard(keyboard) => keyboard.serialize_into(bytes),
            FeedbackStateData::Pointer(pointer) => pointer.serialize_into(bytes),
            FeedbackStateData::String(string) => string.serialize_into(bytes),
            FeedbackStateData::Integer(integer) => integer.serialize_into(bytes),
            FeedbackStateData::Led(led) => led.serialize_into(bytes),
            FeedbackStateData::Bell(bell) => bell.serialize_into(bytes),
            FeedbackStateData::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl FeedbackStateData {
    fn switch_expr(&self) -> u8 {
        match self {
            FeedbackStateData::Keyboard(_) => u8::from(FeedbackClass::KEYBOARD),
            FeedbackStateData::Pointer(_) => u8::from(FeedbackClass::POINTER),
            FeedbackStateData::String(_) => u8::from(FeedbackClass::STRING),
            FeedbackStateData::Integer(_) => u8::from(FeedbackClass::INTEGER),
            FeedbackStateData::Led(_) => u8::from(FeedbackClass::LED),
            FeedbackStateData::Bell(_) => u8::from(FeedbackClass::BELL),
            FeedbackStateData::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod feedback_state_data {
    use super::FeedbackStateData;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackStateData {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..6) {
                0 => Self::Keyboard(GenRandom::generate(rng)),
                1 => Self::Pointer(GenRandom::generate(rng)),
                2 => Self::String(GenRandom::generate(rng)),
                3 => Self::Integer(GenRandom::generate(rng)),
                4 => Self::Led(GenRandom::generate(rng)),
                _ => Self::Bell(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<FeedbackStateData> {
        alloc::vec![
            FeedbackStateData::Keyboard(GenRandom::generate(rng)),
            FeedbackStateData::Pointer(GenRandom::generate(rng)),
            FeedbackStateData::String(GenRandom::generate(rng)),
            FeedbackStateData::Integer(GenRandom::generate(rng)),
            FeedbackStateData::Led(GenRandom::generate(rng)),
            FeedbackStateData::Bell(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackState {
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
        let result = FeedbackState { feedback_id, len, data };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let class_id: u8 = self.data.switch_expr();
        class_id.serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes, class_id);
    }
}
#[cfg(test)]
mod feedback_state {
    use super::FeedbackState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackState {
        fn generate(rng: &Rng) -> Self {
            Self {
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                data: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15352158019596502272);
        let value = FeedbackState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetFeedbackControl request
pub const GET_FEEDBACK_CONTROL_REQUEST: u8 = 22;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetFeedbackControlRequest {
    pub device_id: u8,
}
impl GetFeedbackControlRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_FEEDBACK_CONTROL_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_FEEDBACK_CONTROL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetFeedbackControlRequest {
            device_id,
        })
    }
}
impl Request for GetFeedbackControlRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetFeedbackControlRequest {
    type Reply = GetFeedbackControlReply;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetFeedbackControlReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub feedbacks: Vec<FeedbackState>,
}
impl TryParse for GetFeedbackControlReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_feedbacks, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (feedbacks, remaining) = crate::x11_utils::parse_list::<FeedbackState>(remaining, num_feedbacks.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetFeedbackControlReply { xi_reply_type, sequence, length, feedbacks };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetFeedbackControlReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_feedbacks = u16::try_from(self.feedbacks.len()).expect("`feedbacks` has too many elements");
        num_feedbacks.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 22]);
        self.feedbacks.serialize_into(bytes);
    }
}
impl GetFeedbackControlReply {
    /// Get the value of the `num_feedbacks` field.
    ///
    /// The `num_feedbacks` field is used as the length field of the `feedbacks` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_feedbacks(&self) -> u16 {
        self.feedbacks.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_feedback_control_reply {
    use super::GetFeedbackControlReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetFeedbackControlReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                feedbacks: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(431505973022556160);
        let value = GetFeedbackControlReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let class_id = class_id.into();
        let result = KbdFeedbackCtl { class_id, feedback_id, len, key, auto_repeat_mode, key_click_percent, bell_percent, bell_pitch, bell_duration, led_mask, led_values };
        Ok((result, remaining))
    }
}
impl Serialize for KbdFeedbackCtl {
    type Bytes = [u8; 20];
    fn serialize(&self) -> [u8; 20] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
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
#[cfg(test)]
mod kbd_feedback_ctl {
    use super::KbdFeedbackCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for KbdFeedbackCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                key: GenRandom::generate(rng),
                auto_repeat_mode: GenRandom::generate(rng),
                key_click_percent: GenRandom::generate(rng),
                bell_percent: GenRandom::generate(rng),
                bell_pitch: GenRandom::generate(rng),
                bell_duration: GenRandom::generate(rng),
                led_mask: GenRandom::generate(rng),
                led_values: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2719339621092141056);
        let value = KbdFeedbackCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (num, remaining) = i16::try_parse(remaining)?;
        let (denom, remaining) = i16::try_parse(remaining)?;
        let (threshold, remaining) = i16::try_parse(remaining)?;
        let class_id = class_id.into();
        let result = PtrFeedbackCtl { class_id, feedback_id, len, num, denom, threshold };
        Ok((result, remaining))
    }
}
impl Serialize for PtrFeedbackCtl {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.num.serialize_into(bytes);
        self.denom.serialize_into(bytes);
        self.threshold.serialize_into(bytes);
    }
}
#[cfg(test)]
mod ptr_feedback_ctl {
    use super::PtrFeedbackCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for PtrFeedbackCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                num: GenRandom::generate(rng),
                denom: GenRandom::generate(rng),
                threshold: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(131361231505162240);
        let value = PtrFeedbackCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let class_id = class_id.into();
        let result = IntegerFeedbackCtl { class_id, feedback_id, len, int_to_display };
        Ok((result, remaining))
    }
}
impl Serialize for IntegerFeedbackCtl {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.int_to_display.serialize_into(bytes);
    }
}
#[cfg(test)]
mod integer_feedback_ctl {
    use super::IntegerFeedbackCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for IntegerFeedbackCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                int_to_display: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11489901166412115968);
        let value = IntegerFeedbackCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (num_keysyms, remaining) = u16::try_parse(remaining)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, num_keysyms.try_to_usize()?)?;
        let class_id = class_id.into();
        let result = StringFeedbackCtl { class_id, feedback_id, len, keysyms };
        Ok((result, remaining))
    }
}
impl Serialize for StringFeedbackCtl {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        let num_keysyms = u16::try_from(self.keysyms.len()).expect("`keysyms` has too many elements");
        num_keysyms.serialize_into(bytes);
        self.keysyms.serialize_into(bytes);
    }
}
impl StringFeedbackCtl {
    /// Get the value of the `num_keysyms` field.
    ///
    /// The `num_keysyms` field is used as the length field of the `keysyms` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_keysyms(&self) -> u16 {
        self.keysyms.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod string_feedback_ctl {
    use super::StringFeedbackCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for StringFeedbackCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                keysyms: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5726236037369368576);
        let value = StringFeedbackCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (pitch, remaining) = i16::try_parse(remaining)?;
        let (duration, remaining) = i16::try_parse(remaining)?;
        let class_id = class_id.into();
        let result = BellFeedbackCtl { class_id, feedback_id, len, percent, pitch, duration };
        Ok((result, remaining))
    }
}
impl Serialize for BellFeedbackCtl {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.percent.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.pitch.serialize_into(bytes);
        self.duration.serialize_into(bytes);
    }
}
#[cfg(test)]
mod bell_feedback_ctl {
    use super::BellFeedbackCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for BellFeedbackCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                percent: GenRandom::generate(rng),
                pitch: GenRandom::generate(rng),
                duration: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(715639192905670656);
        let value = BellFeedbackCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let class_id = class_id.into();
        let result = LedFeedbackCtl { class_id, feedback_id, len, led_mask, led_values };
        Ok((result, remaining))
    }
}
impl Serialize for LedFeedbackCtl {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.led_mask.serialize_into(bytes);
        self.led_values.serialize_into(bytes);
    }
}
#[cfg(test)]
mod led_feedback_ctl {
    use super::LedFeedbackCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for LedFeedbackCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                led_mask: GenRandom::generate(rng),
                led_values: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17481898756125822976);
        let value = LedFeedbackCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for FeedbackCtlDataKeyboard {
    type Bytes = [u8; 16];
    fn serialize(&self) -> [u8; 16] {
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
#[cfg(test)]
mod feedback_ctl_data_keyboard {
    use super::FeedbackCtlDataKeyboard;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackCtlDataKeyboard {
        fn generate(rng: &Rng) -> Self {
            Self {
                key: GenRandom::generate(rng),
                auto_repeat_mode: GenRandom::generate(rng),
                key_click_percent: GenRandom::generate(rng),
                bell_percent: GenRandom::generate(rng),
                bell_pitch: GenRandom::generate(rng),
                bell_duration: GenRandom::generate(rng),
                led_mask: GenRandom::generate(rng),
                led_values: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14515145313484472320);
        let value = FeedbackCtlDataKeyboard::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackCtlDataPointer {
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}
impl TryParse for FeedbackCtlDataPointer {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (num, remaining) = i16::try_parse(remaining)?;
        let (denom, remaining) = i16::try_parse(remaining)?;
        let (threshold, remaining) = i16::try_parse(remaining)?;
        let result = FeedbackCtlDataPointer { num, denom, threshold };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackCtlDataPointer {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod feedback_ctl_data_pointer {
    use super::FeedbackCtlDataPointer;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackCtlDataPointer {
        fn generate(rng: &Rng) -> Self {
            Self {
                num: GenRandom::generate(rng),
                denom: GenRandom::generate(rng),
                threshold: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13793741976084938752);
        let value = FeedbackCtlDataPointer::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackCtlDataString {
    pub keysyms: Vec<xproto::Keysym>,
}
impl TryParse for FeedbackCtlDataString {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (num_keysyms, remaining) = u16::try_parse(remaining)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, num_keysyms.try_to_usize()?)?;
        let result = FeedbackCtlDataString { keysyms };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackCtlDataString {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        bytes.extend_from_slice(&[0; 2]);
        let num_keysyms = u16::try_from(self.keysyms.len()).expect("`keysyms` has too many elements");
        num_keysyms.serialize_into(bytes);
        self.keysyms.serialize_into(bytes);
    }
}
impl FeedbackCtlDataString {
    /// Get the value of the `num_keysyms` field.
    ///
    /// The `num_keysyms` field is used as the length field of the `keysyms` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_keysyms(&self) -> u16 {
        self.keysyms.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod feedback_ctl_data_string {
    use super::FeedbackCtlDataString;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackCtlDataString {
        fn generate(rng: &Rng) -> Self {
            Self {
                keysyms: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1399931206702530560);
        let value = FeedbackCtlDataString::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackCtlDataInteger {
    pub int_to_display: i32,
}
impl TryParse for FeedbackCtlDataInteger {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (int_to_display, remaining) = i32::try_parse(remaining)?;
        let result = FeedbackCtlDataInteger { int_to_display };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackCtlDataInteger {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let int_to_display_bytes = self.int_to_display.serialize();
        [
            int_to_display_bytes[0],
            int_to_display_bytes[1],
            int_to_display_bytes[2],
            int_to_display_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.int_to_display.serialize_into(bytes);
    }
}
#[cfg(test)]
mod feedback_ctl_data_integer {
    use super::FeedbackCtlDataInteger;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackCtlDataInteger {
        fn generate(rng: &Rng) -> Self {
            Self {
                int_to_display: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8675258848572276736);
        let value = FeedbackCtlDataInteger::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for FeedbackCtlDataLed {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod feedback_ctl_data_led {
    use super::FeedbackCtlDataLed;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackCtlDataLed {
        fn generate(rng: &Rng) -> Self {
            Self {
                led_mask: GenRandom::generate(rng),
                led_values: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4871789613682589696);
        let value = FeedbackCtlDataLed::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackCtlDataBell {
    pub percent: i8,
    pub pitch: i16,
    pub duration: i16,
}
impl TryParse for FeedbackCtlDataBell {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (percent, remaining) = i8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (pitch, remaining) = i16::try_parse(remaining)?;
        let (duration, remaining) = i16::try_parse(remaining)?;
        let result = FeedbackCtlDataBell { percent, pitch, duration };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackCtlDataBell {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod feedback_ctl_data_bell {
    use super::FeedbackCtlDataBell;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackCtlDataBell {
        fn generate(rng: &Rng) -> Self {
            Self {
                percent: GenRandom::generate(rng),
                pitch: GenRandom::generate(rng),
                duration: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13511527296311099392);
        let value = FeedbackCtlDataBell::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeedbackCtlData {
    Keyboard(FeedbackCtlDataKeyboard),
    Pointer(FeedbackCtlDataPointer),
    String(FeedbackCtlDataString),
    Integer(FeedbackCtlDataInteger),
    Led(FeedbackCtlDataLed),
    Bell(FeedbackCtlDataBell),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u8),
}
impl FeedbackCtlData {
    fn try_parse(value: &[u8], class_id: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = class_id;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u8::from(FeedbackClass::KEYBOARD) {
            let (keyboard, new_remaining) = FeedbackCtlDataKeyboard::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Keyboard(keyboard));
        }
        if switch_expr == u8::from(FeedbackClass::POINTER) {
            let (pointer, new_remaining) = FeedbackCtlDataPointer::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Pointer(pointer));
        }
        if switch_expr == u8::from(FeedbackClass::STRING) {
            let (string, new_remaining) = FeedbackCtlDataString::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::String(string));
        }
        if switch_expr == u8::from(FeedbackClass::INTEGER) {
            let (integer, new_remaining) = FeedbackCtlDataInteger::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Integer(integer));
        }
        if switch_expr == u8::from(FeedbackClass::LED) {
            let (led, new_remaining) = FeedbackCtlDataLed::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Led(led));
        }
        if switch_expr == u8::from(FeedbackClass::BELL) {
            let (bell, new_remaining) = FeedbackCtlDataBell::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(FeedbackCtlData::Bell(bell));
        }
        match parse_result {
            None => Ok((FeedbackCtlData::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl FeedbackCtlData {
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
    pub fn as_integer(&self) -> Option<&FeedbackCtlDataInteger> {
        match self {
            FeedbackCtlData::Integer(value) => Some(value),
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
impl FeedbackCtlData {
    #[allow(dead_code)]
    fn serialize(&self, class_id: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, class_id);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, class_id: u8) {
        let _ = class_id;
        match self {
            FeedbackCtlData::Keyboard(keyboard) => keyboard.serialize_into(bytes),
            FeedbackCtlData::Pointer(pointer) => pointer.serialize_into(bytes),
            FeedbackCtlData::String(string) => string.serialize_into(bytes),
            FeedbackCtlData::Integer(integer) => integer.serialize_into(bytes),
            FeedbackCtlData::Led(led) => led.serialize_into(bytes),
            FeedbackCtlData::Bell(bell) => bell.serialize_into(bytes),
            FeedbackCtlData::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl FeedbackCtlData {
    fn switch_expr(&self) -> u8 {
        match self {
            FeedbackCtlData::Keyboard(_) => u8::from(FeedbackClass::KEYBOARD),
            FeedbackCtlData::Pointer(_) => u8::from(FeedbackClass::POINTER),
            FeedbackCtlData::String(_) => u8::from(FeedbackClass::STRING),
            FeedbackCtlData::Integer(_) => u8::from(FeedbackClass::INTEGER),
            FeedbackCtlData::Led(_) => u8::from(FeedbackClass::LED),
            FeedbackCtlData::Bell(_) => u8::from(FeedbackClass::BELL),
            FeedbackCtlData::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod feedback_ctl_data {
    use super::FeedbackCtlData;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackCtlData {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..6) {
                0 => Self::Keyboard(GenRandom::generate(rng)),
                1 => Self::Pointer(GenRandom::generate(rng)),
                2 => Self::String(GenRandom::generate(rng)),
                3 => Self::Integer(GenRandom::generate(rng)),
                4 => Self::Led(GenRandom::generate(rng)),
                _ => Self::Bell(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<FeedbackCtlData> {
        alloc::vec![
            FeedbackCtlData::Keyboard(GenRandom::generate(rng)),
            FeedbackCtlData::Pointer(GenRandom::generate(rng)),
            FeedbackCtlData::String(GenRandom::generate(rng)),
            FeedbackCtlData::Integer(GenRandom::generate(rng)),
            FeedbackCtlData::Led(GenRandom::generate(rng)),
            FeedbackCtlData::Bell(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackCtl {
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
        let result = FeedbackCtl { feedback_id, len, data };
        Ok((result, remaining))
    }
}
impl Serialize for FeedbackCtl {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let class_id: u8 = self.data.switch_expr();
        class_id.serialize_into(bytes);
        self.feedback_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes, class_id);
    }
}
#[cfg(test)]
mod feedback_ctl {
    use super::FeedbackCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for FeedbackCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                feedback_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                data: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3426680917958477568);
        let value = FeedbackCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeFeedbackControlMask(u8);
impl ChangeFeedbackControlMask {
    pub const KEY_CLICK_PERCENT: Self = Self(1 << 0);
    pub const PERCENT: Self = Self(1 << 1);
    pub const PITCH: Self = Self(1 << 2);
    pub const DURATION: Self = Self(1 << 3);
    pub const LED: Self = Self(1 << 4);
    pub const LED_MODE: Self = Self(1 << 5);
    pub const KEY: Self = Self(1 << 6);
    pub const AUTO_REPEAT_MODE: Self = Self(1 << 7);
    pub const STRING: Self = Self(1 << 0);
    pub const INTEGER: Self = Self(1 << 0);
    pub const ACCEL_NUM: Self = Self(1 << 0);
    pub const ACCEL_DENOM: Self = Self(1 << 1);
    pub const THRESHOLD: Self = Self(1 << 2);
}
impl From<ChangeFeedbackControlMask> for u8 {
    #[inline]
    fn from(input: ChangeFeedbackControlMask) -> Self {
        input.0
    }
}
impl From<ChangeFeedbackControlMask> for Option<u8> {
    #[inline]
    fn from(input: ChangeFeedbackControlMask) -> Self {
        Some(input.0)
    }
}
impl From<ChangeFeedbackControlMask> for u16 {
    #[inline]
    fn from(input: ChangeFeedbackControlMask) -> Self {
        u16::from(input.0)
    }
}
impl From<ChangeFeedbackControlMask> for Option<u16> {
    #[inline]
    fn from(input: ChangeFeedbackControlMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ChangeFeedbackControlMask> for u32 {
    #[inline]
    fn from(input: ChangeFeedbackControlMask) -> Self {
        u32::from(input.0)
    }
}
impl From<ChangeFeedbackControlMask> for Option<u32> {
    #[inline]
    fn from(input: ChangeFeedbackControlMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ChangeFeedbackControlMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ChangeFeedbackControlMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::KEY_CLICK_PERCENT.0.into(), "KEY_CLICK_PERCENT", "KeyClickPercent"),
            (Self::PERCENT.0.into(), "PERCENT", "Percent"),
            (Self::PITCH.0.into(), "PITCH", "Pitch"),
            (Self::DURATION.0.into(), "DURATION", "Duration"),
            (Self::LED.0.into(), "LED", "Led"),
            (Self::LED_MODE.0.into(), "LED_MODE", "LedMode"),
            (Self::KEY.0.into(), "KEY", "Key"),
            (Self::AUTO_REPEAT_MODE.0.into(), "AUTO_REPEAT_MODE", "AutoRepeatMode"),
            (Self::STRING.0.into(), "STRING", "String"),
            (Self::INTEGER.0.into(), "INTEGER", "Integer"),
            (Self::ACCEL_NUM.0.into(), "ACCEL_NUM", "AccelNum"),
            (Self::ACCEL_DENOM.0.into(), "ACCEL_DENOM", "AccelDenom"),
            (Self::THRESHOLD.0.into(), "THRESHOLD", "Threshold"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(ChangeFeedbackControlMask, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for ChangeFeedbackControlMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::KEY_CLICK_PERCENT.0,
            Self::PERCENT.0,
            Self::PITCH.0,
            Self::DURATION.0,
            Self::LED.0,
            Self::LED_MODE.0,
            Self::KEY.0,
            Self::AUTO_REPEAT_MODE.0,
            Self::STRING.0,
            Self::INTEGER.0,
            Self::ACCEL_NUM.0,
            Self::ACCEL_DENOM.0,
            Self::THRESHOLD.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the ChangeFeedbackControl request
pub const CHANGE_FEEDBACK_CONTROL_REQUEST: u8 = 23;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeFeedbackControlRequest {
    pub mask: u32,
    pub device_id: u8,
    pub feedback_id: u8,
    pub feedback: FeedbackCtl,
}
impl ChangeFeedbackControlRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mask_bytes = self.mask.serialize();
        let device_id_bytes = self.device_id.serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_FEEDBACK_CONTROL_REQUEST,
            0,
            0,
            mask_bytes[0],
            mask_bytes[1],
            mask_bytes[2],
            mask_bytes[3],
            device_id_bytes[0],
            feedback_id_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let feedback_bytes = self.feedback.serialize();
        let length_so_far = length_so_far + feedback_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), feedback_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_FEEDBACK_CONTROL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (mask, remaining) = u32::try_parse(value)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (feedback, remaining) = FeedbackCtl::try_parse(remaining)?;
        let _ = remaining;
        Ok(ChangeFeedbackControlRequest {
            mask,
            device_id,
            feedback_id,
            feedback,
        })
    }
}
impl Request for ChangeFeedbackControlRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for ChangeFeedbackControlRequest {
}

/// Opcode for the GetDeviceKeyMapping request
pub const GET_DEVICE_KEY_MAPPING_REQUEST: u8 = 24;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceKeyMappingRequest {
    pub device_id: u8,
    pub first_keycode: KeyCode,
    pub count: u8,
}
impl GetDeviceKeyMappingRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let first_keycode_bytes = self.first_keycode.serialize();
        let count_bytes = self.count.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_KEY_MAPPING_REQUEST,
            0,
            0,
            device_id_bytes[0],
            first_keycode_bytes[0],
            count_bytes[0],
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_KEY_MAPPING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let (first_keycode, remaining) = KeyCode::try_parse(remaining)?;
        let (count, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetDeviceKeyMappingRequest {
            device_id,
            first_keycode,
            count,
        })
    }
}
impl Request for GetDeviceKeyMappingRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceKeyMappingRequest {
    type Reply = GetDeviceKeyMappingReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceKeyMappingReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub keysyms_per_keycode: u8,
    pub keysyms: Vec<xproto::Keysym>,
}
impl TryParse for GetDeviceKeyMappingReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (keysyms_per_keycode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, length.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceKeyMappingReply { xi_reply_type, sequence, keysyms_per_keycode, keysyms };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceKeyMappingReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        let length = u32::try_from(self.keysyms.len()).expect("`keysyms` has too many elements");
        length.serialize_into(bytes);
        self.keysyms_per_keycode.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
        self.keysyms.serialize_into(bytes);
    }
}
impl GetDeviceKeyMappingReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `keysyms` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.keysyms.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_device_key_mapping_reply {
    use super::GetDeviceKeyMappingReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDeviceKeyMappingReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                keysyms_per_keycode: GenRandom::generate(rng),
                keysyms: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(981399017275523072);
        let value = GetDeviceKeyMappingReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ChangeDeviceKeyMapping request
pub const CHANGE_DEVICE_KEY_MAPPING_REQUEST: u8 = 25;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDeviceKeyMappingRequest<'input> {
    pub device_id: u8,
    pub first_keycode: KeyCode,
    pub keysyms_per_keycode: u8,
    pub keycode_count: u8,
    pub keysyms: Cow<'input, [xproto::Keysym]>,
}
impl<'input> ChangeDeviceKeyMappingRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let first_keycode_bytes = self.first_keycode.serialize();
        let keysyms_per_keycode_bytes = self.keysyms_per_keycode.serialize();
        let keycode_count_bytes = self.keycode_count.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_DEVICE_KEY_MAPPING_REQUEST,
            0,
            0,
            device_id_bytes[0],
            first_keycode_bytes[0],
            keysyms_per_keycode_bytes[0],
            keycode_count_bytes[0],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(self.keysyms.len(), usize::try_from(u32::from(self.keycode_count).checked_mul(u32::from(self.keysyms_per_keycode)).unwrap()).unwrap(), "`keysyms` has an incorrect length");
        let keysyms_bytes = self.keysyms.serialize();
        let length_so_far = length_so_far + keysyms_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), keysyms_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_DEVICE_KEY_MAPPING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let (first_keycode, remaining) = KeyCode::try_parse(remaining)?;
        let (keysyms_per_keycode, remaining) = u8::try_parse(remaining)?;
        let (keycode_count, remaining) = u8::try_parse(remaining)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, u32::from(keycode_count).checked_mul(u32::from(keysyms_per_keycode)).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let _ = remaining;
        Ok(ChangeDeviceKeyMappingRequest {
            device_id,
            first_keycode,
            keysyms_per_keycode,
            keycode_count,
            keysyms: Cow::Owned(keysyms),
        })
    }
    /// Clone all borrowed data in this ChangeDeviceKeyMappingRequest.
    pub fn into_owned(self) -> ChangeDeviceKeyMappingRequest<'static> {
        ChangeDeviceKeyMappingRequest {
            device_id: self.device_id,
            first_keycode: self.first_keycode,
            keysyms_per_keycode: self.keysyms_per_keycode,
            keycode_count: self.keycode_count,
            keysyms: Cow::Owned(self.keysyms.into_owned()),
        }
    }
}
impl<'input> Request for ChangeDeviceKeyMappingRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for ChangeDeviceKeyMappingRequest<'input> {
}

/// Opcode for the GetDeviceModifierMapping request
pub const GET_DEVICE_MODIFIER_MAPPING_REQUEST: u8 = 26;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceModifierMappingRequest {
    pub device_id: u8,
}
impl GetDeviceModifierMappingRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_MODIFIER_MAPPING_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_MODIFIER_MAPPING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetDeviceModifierMappingRequest {
            device_id,
        })
    }
}
impl Request for GetDeviceModifierMappingRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceModifierMappingRequest {
    type Reply = GetDeviceModifierMappingReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceModifierMappingReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keymaps: Vec<u8>,
}
impl TryParse for GetDeviceModifierMappingReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (keycodes_per_modifier, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        let (keymaps, remaining) = crate::x11_utils::parse_u8_list(remaining, u32::from(keycodes_per_modifier).checked_mul(8u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let keymaps = keymaps.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceModifierMappingReply { xi_reply_type, sequence, length, keymaps };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceModifierMappingReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        assert_eq!(self.keymaps.len() % 8, 0, "`keymaps` has an incorrect length, must be a multiple of 8");
        let keycodes_per_modifier = u8::try_from(self.keymaps.len() / 8).expect("`keymaps` has too many elements");
        keycodes_per_modifier.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
        bytes.extend_from_slice(&self.keymaps);
    }
}
impl GetDeviceModifierMappingReply {
    /// Get the value of the `keycodes_per_modifier` field.
    ///
    /// The `keycodes_per_modifier` field is used as the length field of the `keymaps` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn keycodes_per_modifier(&self) -> u8 {
        self.keymaps.len()
            .checked_div(8).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_device_modifier_mapping_reply {
    use super::GetDeviceModifierMappingReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDeviceModifierMappingReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                keymaps: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14933561739831148544);
        let value = GetDeviceModifierMappingReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetDeviceModifierMapping request
pub const SET_DEVICE_MODIFIER_MAPPING_REQUEST: u8 = 27;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceModifierMappingRequest<'input> {
    pub device_id: u8,
    pub keymaps: Cow<'input, [u8]>,
}
impl<'input> SetDeviceModifierMappingRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        assert_eq!(self.keymaps.len() % 8, 0, "`keymaps` has an incorrect length, must be a multiple of 8");
        let keycodes_per_modifier = u8::try_from(self.keymaps.len() / 8).expect("`keymaps` has too many elements");
        let keycodes_per_modifier_bytes = keycodes_per_modifier.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_MODIFIER_MAPPING_REQUEST,
            0,
            0,
            device_id_bytes[0],
            keycodes_per_modifier_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.keymaps.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.keymaps, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_MODIFIER_MAPPING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let (keycodes_per_modifier, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (keymaps, remaining) = crate::x11_utils::parse_u8_list(remaining, u32::from(keycodes_per_modifier).checked_mul(8u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetDeviceModifierMappingRequest {
            device_id,
            keymaps: Cow::Borrowed(keymaps),
        })
    }
    /// Clone all borrowed data in this SetDeviceModifierMappingRequest.
    pub fn into_owned(self) -> SetDeviceModifierMappingRequest<'static> {
        SetDeviceModifierMappingRequest {
            device_id: self.device_id,
            keymaps: Cow::Owned(self.keymaps.into_owned()),
        }
    }
}
impl<'input> Request for SetDeviceModifierMappingRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for SetDeviceModifierMappingRequest<'input> {
    type Reply = SetDeviceModifierMappingReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceModifierMappingReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::MappingStatus,
}
impl TryParse for SetDeviceModifierMappingReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.into();
        let result = SetDeviceModifierMappingReply { xi_reply_type, sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for SetDeviceModifierMappingReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = u8::from(self.status).serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod set_device_modifier_mapping_reply {
    use super::SetDeviceModifierMappingReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for SetDeviceModifierMappingReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7324825430018490368);
        let value = SetDeviceModifierMappingReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetDeviceButtonMapping request
pub const GET_DEVICE_BUTTON_MAPPING_REQUEST: u8 = 28;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceButtonMappingRequest {
    pub device_id: u8,
}
impl GetDeviceButtonMappingRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_BUTTON_MAPPING_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_BUTTON_MAPPING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetDeviceButtonMappingRequest {
            device_id,
        })
    }
}
impl Request for GetDeviceButtonMappingRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceButtonMappingRequest {
    type Reply = GetDeviceButtonMappingReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceButtonMappingReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub map: Vec<u8>,
}
impl TryParse for GetDeviceButtonMappingReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (map_size, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        let (map, remaining) = crate::x11_utils::parse_u8_list(remaining, map_size.try_to_usize()?)?;
        let map = map.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceButtonMappingReply { xi_reply_type, sequence, length, map };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceButtonMappingReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let map_size = u8::try_from(self.map.len()).expect("`map` has too many elements");
        map_size.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
        bytes.extend_from_slice(&self.map);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl GetDeviceButtonMappingReply {
    /// Get the value of the `map_size` field.
    ///
    /// The `map_size` field is used as the length field of the `map` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn map_size(&self) -> u8 {
        self.map.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_device_button_mapping_reply {
    use super::GetDeviceButtonMappingReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDeviceButtonMappingReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                map: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12570339239020986368);
        let value = GetDeviceButtonMappingReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetDeviceButtonMapping request
pub const SET_DEVICE_BUTTON_MAPPING_REQUEST: u8 = 29;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceButtonMappingRequest<'input> {
    pub device_id: u8,
    pub map: Cow<'input, [u8]>,
}
impl<'input> SetDeviceButtonMappingRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let map_size = u8::try_from(self.map.len()).expect("`map` has too many elements");
        let map_size_bytes = map_size.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_BUTTON_MAPPING_REQUEST,
            0,
            0,
            device_id_bytes[0],
            map_size_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.map.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.map, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_BUTTON_MAPPING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let (map_size, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (map, remaining) = crate::x11_utils::parse_u8_list(remaining, map_size.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetDeviceButtonMappingRequest {
            device_id,
            map: Cow::Borrowed(map),
        })
    }
    /// Clone all borrowed data in this SetDeviceButtonMappingRequest.
    pub fn into_owned(self) -> SetDeviceButtonMappingRequest<'static> {
        SetDeviceButtonMappingRequest {
            device_id: self.device_id,
            map: Cow::Owned(self.map.into_owned()),
        }
    }
}
impl<'input> Request for SetDeviceButtonMappingRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for SetDeviceButtonMappingRequest<'input> {
    type Reply = SetDeviceButtonMappingReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceButtonMappingReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::MappingStatus,
}
impl TryParse for SetDeviceButtonMappingReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.into();
        let result = SetDeviceButtonMappingReply { xi_reply_type, sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for SetDeviceButtonMappingReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = u8::from(self.status).serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod set_device_button_mapping_reply {
    use super::SetDeviceButtonMappingReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for SetDeviceButtonMappingReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1704238776806539264);
        let value = SetDeviceButtonMappingReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (keys, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let keys = <[u8; 32]>::try_from(keys).unwrap();
        let class_id = class_id.into();
        let result = KeyState { class_id, len, num_keys, keys };
        Ok((result, remaining))
    }
}
impl Serialize for KeyState {
    type Bytes = [u8; 36];
    fn serialize(&self) -> [u8; 36] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.num_keys.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        bytes.extend_from_slice(&self.keys);
    }
}
#[cfg(test)]
mod key_state {
    use super::KeyState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for KeyState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                num_keys: GenRandom::generate(rng),
                keys: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10028943540013200);
        let value = KeyState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (buttons, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let buttons = <[u8; 32]>::try_from(buttons).unwrap();
        let class_id = class_id.into();
        let result = ButtonState { class_id, len, num_buttons, buttons };
        Ok((result, remaining))
    }
}
impl Serialize for ButtonState {
    type Bytes = [u8; 36];
    fn serialize(&self) -> [u8; 36] {
        let class_id_bytes = u8::from(self.class_id).serialize();
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
        u8::from(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.num_buttons.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        bytes.extend_from_slice(&self.buttons);
    }
}
#[cfg(test)]
mod button_state {
    use super::ButtonState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ButtonState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                num_buttons: GenRandom::generate(rng),
                buttons: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9939090486152793088);
        let value = ButtonState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValuatorStateModeMask(u8);
impl ValuatorStateModeMask {
    pub const DEVICE_MODE_ABSOLUTE: Self = Self(1 << 0);
    pub const OUT_OF_PROXIMITY: Self = Self(1 << 1);
}
impl From<ValuatorStateModeMask> for u8 {
    #[inline]
    fn from(input: ValuatorStateModeMask) -> Self {
        input.0
    }
}
impl From<ValuatorStateModeMask> for Option<u8> {
    #[inline]
    fn from(input: ValuatorStateModeMask) -> Self {
        Some(input.0)
    }
}
impl From<ValuatorStateModeMask> for u16 {
    #[inline]
    fn from(input: ValuatorStateModeMask) -> Self {
        u16::from(input.0)
    }
}
impl From<ValuatorStateModeMask> for Option<u16> {
    #[inline]
    fn from(input: ValuatorStateModeMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ValuatorStateModeMask> for u32 {
    #[inline]
    fn from(input: ValuatorStateModeMask) -> Self {
        u32::from(input.0)
    }
}
impl From<ValuatorStateModeMask> for Option<u32> {
    #[inline]
    fn from(input: ValuatorStateModeMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ValuatorStateModeMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ValuatorStateModeMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::DEVICE_MODE_ABSOLUTE.0.into(), "DEVICE_MODE_ABSOLUTE", "DeviceModeAbsolute"),
            (Self::OUT_OF_PROXIMITY.0.into(), "OUT_OF_PROXIMITY", "OutOfProximity"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(ValuatorStateModeMask, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for ValuatorStateModeMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::DEVICE_MODE_ABSOLUTE.0,
            Self::OUT_OF_PROXIMITY.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let (valuators, remaining) = crate::x11_utils::parse_list::<i32>(remaining, num_valuators.try_to_usize()?)?;
        let class_id = class_id.into();
        let result = ValuatorState { class_id, len, mode, valuators };
        Ok((result, remaining))
    }
}
impl Serialize for ValuatorState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        u8::from(self.class_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        let num_valuators = u8::try_from(self.valuators.len()).expect("`valuators` has too many elements");
        num_valuators.serialize_into(bytes);
        self.mode.serialize_into(bytes);
        self.valuators.serialize_into(bytes);
    }
}
impl ValuatorState {
    /// Get the value of the `num_valuators` field.
    ///
    /// The `num_valuators` field is used as the length field of the `valuators` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_valuators(&self) -> u8 {
        self.valuators.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod valuator_state {
    use super::ValuatorState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ValuatorState {
        fn generate(rng: &Rng) -> Self {
            Self {
                class_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                mode: GenRandom::generate(rng),
                valuators: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(612676600527971328);
        let value = ValuatorState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputStateDataKey {
    pub num_keys: u8,
    pub keys: [u8; 32],
}
impl TryParse for InputStateDataKey {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_keys, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (keys, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let keys = <[u8; 32]>::try_from(keys).unwrap();
        let result = InputStateDataKey { num_keys, keys };
        Ok((result, remaining))
    }
}
impl Serialize for InputStateDataKey {
    type Bytes = [u8; 34];
    fn serialize(&self) -> [u8; 34] {
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
        bytes.extend_from_slice(&self.keys);
    }
}
#[cfg(test)]
mod input_state_data_key {
    use super::InputStateDataKey;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputStateDataKey {
        fn generate(rng: &Rng) -> Self {
            Self {
                num_keys: GenRandom::generate(rng),
                keys: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5527058776040833024);
        let value = InputStateDataKey::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputStateDataButton {
    pub num_buttons: u8,
    pub buttons: [u8; 32],
}
impl TryParse for InputStateDataButton {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_buttons, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (buttons, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let buttons = <[u8; 32]>::try_from(buttons).unwrap();
        let result = InputStateDataButton { num_buttons, buttons };
        Ok((result, remaining))
    }
}
impl Serialize for InputStateDataButton {
    type Bytes = [u8; 34];
    fn serialize(&self) -> [u8; 34] {
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
        bytes.extend_from_slice(&self.buttons);
    }
}
#[cfg(test)]
mod input_state_data_button {
    use super::InputStateDataButton;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputStateDataButton {
        fn generate(rng: &Rng) -> Self {
            Self {
                num_buttons: GenRandom::generate(rng),
                buttons: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13038358670073659392);
        let value = InputStateDataButton::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputStateDataValuator {
    pub mode: u8,
    pub valuators: Vec<i32>,
}
impl TryParse for InputStateDataValuator {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (valuators, remaining) = crate::x11_utils::parse_list::<i32>(remaining, num_valuators.try_to_usize()?)?;
        let result = InputStateDataValuator { mode, valuators };
        Ok((result, remaining))
    }
}
impl Serialize for InputStateDataValuator {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        let num_valuators = u8::try_from(self.valuators.len()).expect("`valuators` has too many elements");
        num_valuators.serialize_into(bytes);
        self.mode.serialize_into(bytes);
        self.valuators.serialize_into(bytes);
    }
}
impl InputStateDataValuator {
    /// Get the value of the `num_valuators` field.
    ///
    /// The `num_valuators` field is used as the length field of the `valuators` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_valuators(&self) -> u8 {
        self.valuators.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod input_state_data_valuator {
    use super::InputStateDataValuator;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputStateDataValuator {
        fn generate(rng: &Rng) -> Self {
            Self {
                mode: GenRandom::generate(rng),
                valuators: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14305108348161228800);
        let value = InputStateDataValuator::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputStateData {
    Key(InputStateDataKey),
    Button(InputStateDataButton),
    Valuator(InputStateDataValuator),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u8),
}
impl InputStateData {
    fn try_parse(value: &[u8], class_id: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = class_id;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u8::from(InputClass::KEY) {
            let (key, new_remaining) = InputStateDataKey::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputStateData::Key(key));
        }
        if switch_expr == u8::from(InputClass::BUTTON) {
            let (button, new_remaining) = InputStateDataButton::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputStateData::Button(button));
        }
        if switch_expr == u8::from(InputClass::VALUATOR) {
            let (valuator, new_remaining) = InputStateDataValuator::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(InputStateData::Valuator(valuator));
        }
        match parse_result {
            None => Ok((InputStateData::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl InputStateData {
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
impl InputStateData {
    #[allow(dead_code)]
    fn serialize(&self, class_id: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, class_id);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, class_id: u8) {
        let _ = class_id;
        match self {
            InputStateData::Key(key) => key.serialize_into(bytes),
            InputStateData::Button(button) => button.serialize_into(bytes),
            InputStateData::Valuator(valuator) => valuator.serialize_into(bytes),
            InputStateData::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl InputStateData {
    fn switch_expr(&self) -> u8 {
        match self {
            InputStateData::Key(_) => u8::from(InputClass::KEY),
            InputStateData::Button(_) => u8::from(InputClass::BUTTON),
            InputStateData::Valuator(_) => u8::from(InputClass::VALUATOR),
            InputStateData::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod input_state_data {
    use super::InputStateData;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputStateData {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..3) {
                0 => Self::Key(GenRandom::generate(rng)),
                1 => Self::Button(GenRandom::generate(rng)),
                _ => Self::Valuator(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<InputStateData> {
        alloc::vec![
            InputStateData::Key(GenRandom::generate(rng)),
            InputStateData::Button(GenRandom::generate(rng)),
            InputStateData::Valuator(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputState {
    pub len: u8,
    pub data: InputStateData,
}
impl TryParse for InputState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (class_id, remaining) = u8::try_parse(remaining)?;
        let (len, remaining) = u8::try_parse(remaining)?;
        let (data, remaining) = InputStateData::try_parse(remaining, class_id)?;
        let result = InputState { len, data };
        Ok((result, remaining))
    }
}
impl Serialize for InputState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        let class_id: u8 = self.data.switch_expr();
        class_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes, class_id);
    }
}
#[cfg(test)]
mod input_state {
    use super::InputState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for InputState {
        fn generate(rng: &Rng) -> Self {
            Self {
                len: GenRandom::generate(rng),
                data: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4429158771651934208);
        let value = InputState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryDeviceState request
pub const QUERY_DEVICE_STATE_REQUEST: u8 = 30;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryDeviceStateRequest {
    pub device_id: u8,
}
impl QueryDeviceStateRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_DEVICE_STATE_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_DEVICE_STATE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(QueryDeviceStateRequest {
            device_id,
        })
    }
}
impl Request for QueryDeviceStateRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryDeviceStateRequest {
    type Reply = QueryDeviceStateReply;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryDeviceStateReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub classes: Vec<InputState>,
}
impl TryParse for QueryDeviceStateReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_classes, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<InputState>(remaining, num_classes.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryDeviceStateReply { xi_reply_type, sequence, length, classes };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryDeviceStateReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_classes = u8::try_from(self.classes.len()).expect("`classes` has too many elements");
        num_classes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
        self.classes.serialize_into(bytes);
    }
}
impl QueryDeviceStateReply {
    /// Get the value of the `num_classes` field.
    ///
    /// The `num_classes` field is used as the length field of the `classes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_classes(&self) -> u8 {
        self.classes.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod query_device_state_reply {
    use super::QueryDeviceStateReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for QueryDeviceStateReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                classes: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4631255388678750208);
        let value = QueryDeviceStateReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the DeviceBell request
pub const DEVICE_BELL_REQUEST: u8 = 32;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceBellRequest {
    pub device_id: u8,
    pub feedback_id: u8,
    pub feedback_class: u8,
    pub percent: i8,
}
impl DeviceBellRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let feedback_id_bytes = self.feedback_id.serialize();
        let feedback_class_bytes = self.feedback_class.serialize();
        let percent_bytes = self.percent.serialize();
        let mut request0 = vec![
            major_opcode,
            DEVICE_BELL_REQUEST,
            0,
            0,
            device_id_bytes[0],
            feedback_id_bytes[0],
            feedback_class_bytes[0],
            percent_bytes[0],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DEVICE_BELL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let (feedback_id, remaining) = u8::try_parse(remaining)?;
        let (feedback_class, remaining) = u8::try_parse(remaining)?;
        let (percent, remaining) = i8::try_parse(remaining)?;
        let _ = remaining;
        Ok(DeviceBellRequest {
            device_id,
            feedback_id,
            feedback_class,
            percent,
        })
    }
}
impl Request for DeviceBellRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DeviceBellRequest {
}

/// Opcode for the SetDeviceValuators request
pub const SET_DEVICE_VALUATORS_REQUEST: u8 = 33;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceValuatorsRequest<'input> {
    pub device_id: u8,
    pub first_valuator: u8,
    pub valuators: Cow<'input, [i32]>,
}
impl<'input> SetDeviceValuatorsRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let first_valuator_bytes = self.first_valuator.serialize();
        let num_valuators = u8::try_from(self.valuators.len()).expect("`valuators` has too many elements");
        let num_valuators_bytes = num_valuators.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_VALUATORS_REQUEST,
            0,
            0,
            device_id_bytes[0],
            first_valuator_bytes[0],
            num_valuators_bytes[0],
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let valuators_bytes = self.valuators.serialize();
        let length_so_far = length_so_far + valuators_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), valuators_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_VALUATORS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let (first_valuator, remaining) = u8::try_parse(remaining)?;
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (valuators, remaining) = crate::x11_utils::parse_list::<i32>(remaining, num_valuators.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetDeviceValuatorsRequest {
            device_id,
            first_valuator,
            valuators: Cow::Owned(valuators),
        })
    }
    /// Clone all borrowed data in this SetDeviceValuatorsRequest.
    pub fn into_owned(self) -> SetDeviceValuatorsRequest<'static> {
        SetDeviceValuatorsRequest {
            device_id: self.device_id,
            first_valuator: self.first_valuator,
            valuators: Cow::Owned(self.valuators.into_owned()),
        }
    }
}
impl<'input> Request for SetDeviceValuatorsRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for SetDeviceValuatorsRequest<'input> {
    type Reply = SetDeviceValuatorsReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceValuatorsReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl TryParse for SetDeviceValuatorsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.into();
        let result = SetDeviceValuatorsReply { xi_reply_type, sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for SetDeviceValuatorsReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = u8::from(self.status).serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod set_device_valuators_reply {
    use super::SetDeviceValuatorsReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for SetDeviceValuatorsReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14904814533756059648);
        let value = SetDeviceValuatorsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceControl(u16);
impl DeviceControl {
    pub const RESOLUTION: Self = Self(1);
    pub const ABSCALIB: Self = Self(2);
    pub const CORE: Self = Self(3);
    pub const ENABLE: Self = Self(4);
    pub const ABSAREA: Self = Self(5);
}
impl From<DeviceControl> for u16 {
    #[inline]
    fn from(input: DeviceControl) -> Self {
        input.0
    }
}
impl From<DeviceControl> for Option<u16> {
    #[inline]
    fn from(input: DeviceControl) -> Self {
        Some(input.0)
    }
}
impl From<DeviceControl> for u32 {
    #[inline]
    fn from(input: DeviceControl) -> Self {
        u32::from(input.0)
    }
}
impl From<DeviceControl> for Option<u32> {
    #[inline]
    fn from(input: DeviceControl) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for DeviceControl {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for DeviceControl {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for DeviceControl  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::RESOLUTION.0.into(), "RESOLUTION", "Resolution"),
            (Self::ABSCALIB.0.into(), "ABSCALIB", "Abscalib"),
            (Self::CORE.0.into(), "CORE", "Core"),
            (Self::ENABLE.0.into(), "ENABLE", "Enable"),
            (Self::ABSAREA.0.into(), "ABSAREA", "Absarea"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for DeviceControl {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::RESOLUTION.0,
            Self::ABSCALIB.0,
            Self::CORE.0,
            Self::ENABLE.0,
            Self::ABSAREA.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceResolutionState {
    pub control_id: DeviceControl,
    pub len: u16,
    pub resolution_values: Vec<u32>,
    pub resolution_min: Vec<u32>,
    pub resolution_max: Vec<u32>,
}
impl TryParse for DeviceResolutionState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (num_valuators, remaining) = u32::try_parse(remaining)?;
        let (resolution_values, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators.try_to_usize()?)?;
        let (resolution_min, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators.try_to_usize()?)?;
        let (resolution_max, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators.try_to_usize()?)?;
        let control_id = control_id.into();
        let result = DeviceResolutionState { control_id, len, resolution_values, resolution_min, resolution_max };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceResolutionState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u16::from(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        let num_valuators = u32::try_from(self.resolution_values.len()).expect("`resolution_values` has too many elements");
        num_valuators.serialize_into(bytes);
        self.resolution_values.serialize_into(bytes);
        assert_eq!(self.resolution_min.len(), usize::try_from(num_valuators).unwrap(), "`resolution_min` has an incorrect length");
        self.resolution_min.serialize_into(bytes);
        assert_eq!(self.resolution_max.len(), usize::try_from(num_valuators).unwrap(), "`resolution_max` has an incorrect length");
        self.resolution_max.serialize_into(bytes);
    }
}
impl DeviceResolutionState {
    /// Get the value of the `num_valuators` field.
    ///
    /// The `num_valuators` field is used as the length field of the `resolution_values` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_valuators(&self) -> u32 {
        self.resolution_values.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod device_resolution_state {
    use super::DeviceResolutionState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceResolutionState {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                resolution_values: GenRandom::generate(rng),
                resolution_min: GenRandom::generate(rng),
                resolution_max: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5372691805322125312);
        let value = DeviceResolutionState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let control_id = control_id.into();
        let result = DeviceAbsCalibState { control_id, len, min_x, max_x, min_y, max_y, flip_x, flip_y, rotation, button_threshold };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceAbsCalibState {
    type Bytes = [u8; 36];
    fn serialize(&self) -> [u8; 36] {
        let control_id_bytes = u16::from(self.control_id).serialize();
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
        u16::from(self.control_id).serialize_into(bytes);
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
#[cfg(test)]
mod device_abs_calib_state {
    use super::DeviceAbsCalibState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceAbsCalibState {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                min_x: GenRandom::generate(rng),
                max_x: GenRandom::generate(rng),
                min_y: GenRandom::generate(rng),
                max_y: GenRandom::generate(rng),
                flip_x: GenRandom::generate(rng),
                flip_y: GenRandom::generate(rng),
                rotation: GenRandom::generate(rng),
                button_threshold: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(954135656690378752);
        let value = DeviceAbsCalibState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let control_id = control_id.into();
        let result = DeviceAbsAreaState { control_id, len, offset_x, offset_y, width, height, screen, following };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceAbsAreaState {
    type Bytes = [u8; 28];
    fn serialize(&self) -> [u8; 28] {
        let control_id_bytes = u16::from(self.control_id).serialize();
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
        u16::from(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.offset_x.serialize_into(bytes);
        self.offset_y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.screen.serialize_into(bytes);
        self.following.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_abs_area_state {
    use super::DeviceAbsAreaState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceAbsAreaState {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                offset_x: GenRandom::generate(rng),
                offset_y: GenRandom::generate(rng),
                width: GenRandom::generate(rng),
                height: GenRandom::generate(rng),
                screen: GenRandom::generate(rng),
                following: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(18038828767116059136);
        let value = DeviceAbsAreaState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let control_id = control_id.into();
        let result = DeviceCoreState { control_id, len, status, iscore };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceCoreState {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let control_id_bytes = u16::from(self.control_id).serialize();
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
        u16::from(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.status.serialize_into(bytes);
        self.iscore.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}
#[cfg(test)]
mod device_core_state {
    use super::DeviceCoreState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceCoreState {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
                iscore: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10067687606458982144);
        let value = DeviceCoreState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let control_id = control_id.into();
        let result = DeviceEnableState { control_id, len, enable };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceEnableState {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let control_id_bytes = u16::from(self.control_id).serialize();
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
        u16::from(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.enable.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}
#[cfg(test)]
mod device_enable_state {
    use super::DeviceEnableState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceEnableState {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                enable: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(502957766914152448);
        let value = DeviceEnableState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceStateDataResolution {
    pub resolution_values: Vec<u32>,
    pub resolution_min: Vec<u32>,
    pub resolution_max: Vec<u32>,
}
impl TryParse for DeviceStateDataResolution {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_valuators, remaining) = u32::try_parse(remaining)?;
        let (resolution_values, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators.try_to_usize()?)?;
        let (resolution_min, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators.try_to_usize()?)?;
        let (resolution_max, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators.try_to_usize()?)?;
        let result = DeviceStateDataResolution { resolution_values, resolution_min, resolution_max };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceStateDataResolution {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        let num_valuators = u32::try_from(self.resolution_values.len()).expect("`resolution_values` has too many elements");
        num_valuators.serialize_into(bytes);
        self.resolution_values.serialize_into(bytes);
        assert_eq!(self.resolution_min.len(), usize::try_from(num_valuators).unwrap(), "`resolution_min` has an incorrect length");
        self.resolution_min.serialize_into(bytes);
        assert_eq!(self.resolution_max.len(), usize::try_from(num_valuators).unwrap(), "`resolution_max` has an incorrect length");
        self.resolution_max.serialize_into(bytes);
    }
}
impl DeviceStateDataResolution {
    /// Get the value of the `num_valuators` field.
    ///
    /// The `num_valuators` field is used as the length field of the `resolution_values` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_valuators(&self) -> u32 {
        self.resolution_values.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod device_state_data_resolution {
    use super::DeviceStateDataResolution;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceStateDataResolution {
        fn generate(rng: &Rng) -> Self {
            Self {
                resolution_values: GenRandom::generate(rng),
                resolution_min: GenRandom::generate(rng),
                resolution_max: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11239051915084038144);
        let value = DeviceStateDataResolution::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for DeviceStateDataAbsCalib {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
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
#[cfg(test)]
mod device_state_data_abs_calib {
    use super::DeviceStateDataAbsCalib;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceStateDataAbsCalib {
        fn generate(rng: &Rng) -> Self {
            Self {
                min_x: GenRandom::generate(rng),
                max_x: GenRandom::generate(rng),
                min_y: GenRandom::generate(rng),
                max_y: GenRandom::generate(rng),
                flip_x: GenRandom::generate(rng),
                flip_y: GenRandom::generate(rng),
                rotation: GenRandom::generate(rng),
                button_threshold: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13661908997385781248);
        let value = DeviceStateDataAbsCalib::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceStateDataCore {
    pub status: u8,
    pub iscore: u8,
}
impl TryParse for DeviceStateDataCore {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (status, remaining) = u8::try_parse(remaining)?;
        let (iscore, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let result = DeviceStateDataCore { status, iscore };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceStateDataCore {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
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
#[cfg(test)]
mod device_state_data_core {
    use super::DeviceStateDataCore;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceStateDataCore {
        fn generate(rng: &Rng) -> Self {
            Self {
                status: GenRandom::generate(rng),
                iscore: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3881827076722864128);
        let value = DeviceStateDataCore::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for DeviceStateDataAbsArea {
    type Bytes = [u8; 24];
    fn serialize(&self) -> [u8; 24] {
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
#[cfg(test)]
mod device_state_data_abs_area {
    use super::DeviceStateDataAbsArea;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceStateDataAbsArea {
        fn generate(rng: &Rng) -> Self {
            Self {
                offset_x: GenRandom::generate(rng),
                offset_y: GenRandom::generate(rng),
                width: GenRandom::generate(rng),
                height: GenRandom::generate(rng),
                screen: GenRandom::generate(rng),
                following: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4489045204152328192);
        let value = DeviceStateDataAbsArea::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceStateData {
    Resolution(DeviceStateDataResolution),
    AbsCalib(DeviceStateDataAbsCalib),
    Core(DeviceStateDataCore),
    Enable(u8),
    AbsArea(DeviceStateDataAbsArea),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u16),
}
impl DeviceStateData {
    fn try_parse(value: &[u8], control_id: u16) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = control_id;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u16::from(DeviceControl::RESOLUTION) {
            let (resolution, new_remaining) = DeviceStateDataResolution::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::Resolution(resolution));
        }
        if switch_expr == u16::from(DeviceControl::ABSCALIB) {
            let (abs_calib, new_remaining) = DeviceStateDataAbsCalib::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::AbsCalib(abs_calib));
        }
        if switch_expr == u16::from(DeviceControl::CORE) {
            let (core, new_remaining) = DeviceStateDataCore::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::Core(core));
        }
        if switch_expr == u16::from(DeviceControl::ENABLE) {
            let remaining = outer_remaining;
            let (enable, remaining) = u8::try_parse(remaining)?;
            let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::Enable(enable));
        }
        if switch_expr == u16::from(DeviceControl::ABSAREA) {
            let (abs_area, new_remaining) = DeviceStateDataAbsArea::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceStateData::AbsArea(abs_area));
        }
        match parse_result {
            None => Ok((DeviceStateData::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl DeviceStateData {
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
impl DeviceStateData {
    #[allow(dead_code)]
    fn serialize(&self, control_id: u16) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, control_id);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, control_id: u16) {
        let _ = control_id;
        match self {
            DeviceStateData::Resolution(resolution) => resolution.serialize_into(bytes),
            DeviceStateData::AbsCalib(abs_calib) => abs_calib.serialize_into(bytes),
            DeviceStateData::Core(core) => core.serialize_into(bytes),
            DeviceStateData::Enable(enable) => {
                bytes.reserve(4);
                enable.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3]);
            }
            DeviceStateData::AbsArea(abs_area) => abs_area.serialize_into(bytes),
            DeviceStateData::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl DeviceStateData {
    fn switch_expr(&self) -> u16 {
        match self {
            DeviceStateData::Resolution(_) => u16::from(DeviceControl::RESOLUTION),
            DeviceStateData::AbsCalib(_) => u16::from(DeviceControl::ABSCALIB),
            DeviceStateData::Core(_) => u16::from(DeviceControl::CORE),
            DeviceStateData::Enable(_) => u16::from(DeviceControl::ENABLE),
            DeviceStateData::AbsArea(_) => u16::from(DeviceControl::ABSAREA),
            DeviceStateData::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod device_state_data {
    use super::DeviceStateData;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceStateData {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..5) {
                0 => Self::Resolution(GenRandom::generate(rng)),
                1 => Self::AbsCalib(GenRandom::generate(rng)),
                2 => Self::Core(GenRandom::generate(rng)),
                3 => Self::Enable(GenRandom::generate(rng)),
                _ => Self::AbsArea(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<DeviceStateData> {
        alloc::vec![
            DeviceStateData::Resolution(GenRandom::generate(rng)),
            DeviceStateData::AbsCalib(GenRandom::generate(rng)),
            DeviceStateData::Core(GenRandom::generate(rng)),
            DeviceStateData::Enable(GenRandom::generate(rng)),
            DeviceStateData::AbsArea(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceState {
    pub len: u16,
    pub data: DeviceStateData,
}
impl TryParse for DeviceState {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = DeviceStateData::try_parse(remaining, control_id)?;
        let result = DeviceState { len, data };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceState {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let control_id: u16 = self.data.switch_expr();
        control_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes, control_id);
    }
}
#[cfg(test)]
mod device_state {
    use super::DeviceState;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceState {
        fn generate(rng: &Rng) -> Self {
            Self {
                len: GenRandom::generate(rng),
                data: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12747932063375596416);
        let value = DeviceState::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetDeviceControl request
pub const GET_DEVICE_CONTROL_REQUEST: u8 = 34;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceControlRequest {
    pub control_id: DeviceControl,
    pub device_id: u8,
}
impl GetDeviceControlRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let control_id_bytes = u16::from(self.control_id).serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_CONTROL_REQUEST,
            0,
            0,
            control_id_bytes[0],
            control_id_bytes[1],
            device_id_bytes[0],
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_CONTROL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (control_id, remaining) = u16::try_parse(value)?;
        let control_id = control_id.into();
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetDeviceControlRequest {
            control_id,
            device_id,
        })
    }
}
impl Request for GetDeviceControlRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceControlRequest {
    type Reply = GetDeviceControlReply;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceControlReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub control: DeviceState,
}
impl TryParse for GetDeviceControlReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        let (control, remaining) = DeviceState::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceControlReply { xi_reply_type, sequence, length, status, control };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceControlReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.status.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
        self.control.serialize_into(bytes);
    }
}
#[cfg(test)]
mod get_device_control_reply {
    use super::GetDeviceControlReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDeviceControlReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
                control: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11541465192518123520);
        let value = GetDeviceControlReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (resolution_values, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators.try_to_usize()?)?;
        let control_id = control_id.into();
        let result = DeviceResolutionCtl { control_id, len, first_valuator, resolution_values };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceResolutionCtl {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u16::from(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.first_valuator.serialize_into(bytes);
        let num_valuators = u8::try_from(self.resolution_values.len()).expect("`resolution_values` has too many elements");
        num_valuators.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.resolution_values.serialize_into(bytes);
    }
}
impl DeviceResolutionCtl {
    /// Get the value of the `num_valuators` field.
    ///
    /// The `num_valuators` field is used as the length field of the `resolution_values` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_valuators(&self) -> u8 {
        self.resolution_values.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod device_resolution_ctl {
    use super::DeviceResolutionCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceResolutionCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                first_valuator: GenRandom::generate(rng),
                resolution_values: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15286755877133017088);
        let value = DeviceResolutionCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let control_id = control_id.into();
        let result = DeviceAbsCalibCtl { control_id, len, min_x, max_x, min_y, max_y, flip_x, flip_y, rotation, button_threshold };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceAbsCalibCtl {
    type Bytes = [u8; 36];
    fn serialize(&self) -> [u8; 36] {
        let control_id_bytes = u16::from(self.control_id).serialize();
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
        u16::from(self.control_id).serialize_into(bytes);
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
#[cfg(test)]
mod device_abs_calib_ctl {
    use super::DeviceAbsCalibCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceAbsCalibCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                min_x: GenRandom::generate(rng),
                max_x: GenRandom::generate(rng),
                min_y: GenRandom::generate(rng),
                max_y: GenRandom::generate(rng),
                flip_x: GenRandom::generate(rng),
                flip_y: GenRandom::generate(rng),
                rotation: GenRandom::generate(rng),
                button_threshold: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3509212396161267712);
        let value = DeviceAbsCalibCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let control_id = control_id.into();
        let result = DeviceAbsAreaCtrl { control_id, len, offset_x, offset_y, width, height, screen, following };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceAbsAreaCtrl {
    type Bytes = [u8; 28];
    fn serialize(&self) -> [u8; 28] {
        let control_id_bytes = u16::from(self.control_id).serialize();
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
        u16::from(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.offset_x.serialize_into(bytes);
        self.offset_y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.screen.serialize_into(bytes);
        self.following.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_abs_area_ctrl {
    use super::DeviceAbsAreaCtrl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceAbsAreaCtrl {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                offset_x: GenRandom::generate(rng),
                offset_y: GenRandom::generate(rng),
                width: GenRandom::generate(rng),
                height: GenRandom::generate(rng),
                screen: GenRandom::generate(rng),
                following: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1432943217556016128);
        let value = DeviceAbsAreaCtrl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let control_id = control_id.into();
        let result = DeviceCoreCtrl { control_id, len, status };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceCoreCtrl {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let control_id_bytes = u16::from(self.control_id).serialize();
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
        u16::from(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.status.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}
#[cfg(test)]
mod device_core_ctrl {
    use super::DeviceCoreCtrl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceCoreCtrl {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9451725583311682048);
        let value = DeviceCoreCtrl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let control_id = control_id.into();
        let result = DeviceEnableCtrl { control_id, len, enable };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceEnableCtrl {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let control_id_bytes = u16::from(self.control_id).serialize();
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
        u16::from(self.control_id).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.enable.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}
#[cfg(test)]
mod device_enable_ctrl {
    use super::DeviceEnableCtrl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceEnableCtrl {
        fn generate(rng: &Rng) -> Self {
            Self {
                control_id: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                enable: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5707698105144479744);
        let value = DeviceEnableCtrl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceCtlDataResolution {
    pub first_valuator: u8,
    pub resolution_values: Vec<u32>,
}
impl TryParse for DeviceCtlDataResolution {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (first_valuator, remaining) = u8::try_parse(remaining)?;
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (resolution_values, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_valuators.try_to_usize()?)?;
        let result = DeviceCtlDataResolution { first_valuator, resolution_values };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceCtlDataResolution {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.first_valuator.serialize_into(bytes);
        let num_valuators = u8::try_from(self.resolution_values.len()).expect("`resolution_values` has too many elements");
        num_valuators.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.resolution_values.serialize_into(bytes);
    }
}
impl DeviceCtlDataResolution {
    /// Get the value of the `num_valuators` field.
    ///
    /// The `num_valuators` field is used as the length field of the `resolution_values` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_valuators(&self) -> u8 {
        self.resolution_values.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod device_ctl_data_resolution {
    use super::DeviceCtlDataResolution;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceCtlDataResolution {
        fn generate(rng: &Rng) -> Self {
            Self {
                first_valuator: GenRandom::generate(rng),
                resolution_values: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9941447136651902976);
        let value = DeviceCtlDataResolution::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for DeviceCtlDataAbsCalib {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
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
#[cfg(test)]
mod device_ctl_data_abs_calib {
    use super::DeviceCtlDataAbsCalib;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceCtlDataAbsCalib {
        fn generate(rng: &Rng) -> Self {
            Self {
                min_x: GenRandom::generate(rng),
                max_x: GenRandom::generate(rng),
                min_y: GenRandom::generate(rng),
                max_y: GenRandom::generate(rng),
                flip_x: GenRandom::generate(rng),
                flip_y: GenRandom::generate(rng),
                rotation: GenRandom::generate(rng),
                button_threshold: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14972916642182955008);
        let value = DeviceCtlDataAbsCalib::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceCtlDataCore {
    pub status: u8,
}
impl TryParse for DeviceCtlDataCore {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let result = DeviceCtlDataCore { status };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceCtlDataCore {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let status_bytes = self.status.serialize();
        [
            status_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.status.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}
#[cfg(test)]
mod device_ctl_data_core {
    use super::DeviceCtlDataCore;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceCtlDataCore {
        fn generate(rng: &Rng) -> Self {
            Self {
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1008983116064362496);
        let value = DeviceCtlDataCore::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for DeviceCtlDataAbsArea {
    type Bytes = [u8; 24];
    fn serialize(&self) -> [u8; 24] {
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
#[cfg(test)]
mod device_ctl_data_abs_area {
    use super::DeviceCtlDataAbsArea;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceCtlDataAbsArea {
        fn generate(rng: &Rng) -> Self {
            Self {
                offset_x: GenRandom::generate(rng),
                offset_y: GenRandom::generate(rng),
                width: GenRandom::generate(rng),
                height: GenRandom::generate(rng),
                screen: GenRandom::generate(rng),
                following: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10309047854618501120);
        let value = DeviceCtlDataAbsArea::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceCtlData {
    Resolution(DeviceCtlDataResolution),
    AbsCalib(DeviceCtlDataAbsCalib),
    Core(DeviceCtlDataCore),
    Enable(u8),
    AbsArea(DeviceCtlDataAbsArea),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u16),
}
impl DeviceCtlData {
    fn try_parse(value: &[u8], control_id: u16) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = control_id;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u16::from(DeviceControl::RESOLUTION) {
            let (resolution, new_remaining) = DeviceCtlDataResolution::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::Resolution(resolution));
        }
        if switch_expr == u16::from(DeviceControl::ABSCALIB) {
            let (abs_calib, new_remaining) = DeviceCtlDataAbsCalib::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::AbsCalib(abs_calib));
        }
        if switch_expr == u16::from(DeviceControl::CORE) {
            let (core, new_remaining) = DeviceCtlDataCore::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::Core(core));
        }
        if switch_expr == u16::from(DeviceControl::ENABLE) {
            let remaining = outer_remaining;
            let (enable, remaining) = u8::try_parse(remaining)?;
            let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::Enable(enable));
        }
        if switch_expr == u16::from(DeviceControl::ABSAREA) {
            let (abs_area, new_remaining) = DeviceCtlDataAbsArea::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceCtlData::AbsArea(abs_area));
        }
        match parse_result {
            None => Ok((DeviceCtlData::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl DeviceCtlData {
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
    pub fn as_core(&self) -> Option<&DeviceCtlDataCore> {
        match self {
            DeviceCtlData::Core(value) => Some(value),
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
impl DeviceCtlData {
    #[allow(dead_code)]
    fn serialize(&self, control_id: u16) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, control_id);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, control_id: u16) {
        let _ = control_id;
        match self {
            DeviceCtlData::Resolution(resolution) => resolution.serialize_into(bytes),
            DeviceCtlData::AbsCalib(abs_calib) => abs_calib.serialize_into(bytes),
            DeviceCtlData::Core(core) => core.serialize_into(bytes),
            DeviceCtlData::Enable(enable) => {
                bytes.reserve(4);
                enable.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3]);
            }
            DeviceCtlData::AbsArea(abs_area) => abs_area.serialize_into(bytes),
            DeviceCtlData::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl DeviceCtlData {
    fn switch_expr(&self) -> u16 {
        match self {
            DeviceCtlData::Resolution(_) => u16::from(DeviceControl::RESOLUTION),
            DeviceCtlData::AbsCalib(_) => u16::from(DeviceControl::ABSCALIB),
            DeviceCtlData::Core(_) => u16::from(DeviceControl::CORE),
            DeviceCtlData::Enable(_) => u16::from(DeviceControl::ENABLE),
            DeviceCtlData::AbsArea(_) => u16::from(DeviceControl::ABSAREA),
            DeviceCtlData::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod device_ctl_data {
    use super::DeviceCtlData;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceCtlData {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..5) {
                0 => Self::Resolution(GenRandom::generate(rng)),
                1 => Self::AbsCalib(GenRandom::generate(rng)),
                2 => Self::Core(GenRandom::generate(rng)),
                3 => Self::Enable(GenRandom::generate(rng)),
                _ => Self::AbsArea(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<DeviceCtlData> {
        alloc::vec![
            DeviceCtlData::Resolution(GenRandom::generate(rng)),
            DeviceCtlData::AbsCalib(GenRandom::generate(rng)),
            DeviceCtlData::Core(GenRandom::generate(rng)),
            DeviceCtlData::Enable(GenRandom::generate(rng)),
            DeviceCtlData::AbsArea(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceCtl {
    pub len: u16,
    pub data: DeviceCtlData,
}
impl TryParse for DeviceCtl {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (control_id, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = DeviceCtlData::try_parse(remaining, control_id)?;
        let result = DeviceCtl { len, data };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceCtl {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let control_id: u16 = self.data.switch_expr();
        control_id.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes, control_id);
    }
}
#[cfg(test)]
mod device_ctl {
    use super::DeviceCtl;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceCtl {
        fn generate(rng: &Rng) -> Self {
            Self {
                len: GenRandom::generate(rng),
                data: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(714191551897380480);
        let value = DeviceCtl::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ChangeDeviceControl request
pub const CHANGE_DEVICE_CONTROL_REQUEST: u8 = 35;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDeviceControlRequest {
    pub control_id: DeviceControl,
    pub device_id: u8,
    pub control: DeviceCtl,
}
impl ChangeDeviceControlRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let control_id_bytes = u16::from(self.control_id).serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_DEVICE_CONTROL_REQUEST,
            0,
            0,
            control_id_bytes[0],
            control_id_bytes[1],
            device_id_bytes[0],
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let control_bytes = self.control.serialize();
        let length_so_far = length_so_far + control_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), control_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_DEVICE_CONTROL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (control_id, remaining) = u16::try_parse(value)?;
        let control_id = control_id.into();
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (control, remaining) = DeviceCtl::try_parse(remaining)?;
        let _ = remaining;
        Ok(ChangeDeviceControlRequest {
            control_id,
            device_id,
            control,
        })
    }
}
impl Request for ChangeDeviceControlRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for ChangeDeviceControlRequest {
    type Reply = ChangeDeviceControlReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDeviceControlReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
}
impl TryParse for ChangeDeviceControlReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ChangeDeviceControlReply { xi_reply_type, sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ChangeDeviceControlReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let xi_reply_type_bytes = self.xi_reply_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = self.status.serialize();
        [
            response_type_bytes[0],
            xi_reply_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.status.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod change_device_control_reply {
    use super::ChangeDeviceControlReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ChangeDeviceControlReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10658840133124489216);
        let value = ChangeDeviceControlReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ListDeviceProperties request
pub const LIST_DEVICE_PROPERTIES_REQUEST: u8 = 36;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListDevicePropertiesRequest {
    pub device_id: u8,
}
impl ListDevicePropertiesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            LIST_DEVICE_PROPERTIES_REQUEST,
            0,
            0,
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_DEVICE_PROPERTIES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_id, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(ListDevicePropertiesRequest {
            device_id,
        })
    }
}
impl Request for ListDevicePropertiesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for ListDevicePropertiesRequest {
    type Reply = ListDevicePropertiesReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListDevicePropertiesReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Vec<xproto::Atom>,
}
impl TryParse for ListDevicePropertiesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_atoms, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (atoms, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_atoms.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListDevicePropertiesReply { xi_reply_type, sequence, length, atoms };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ListDevicePropertiesReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_atoms = u16::try_from(self.atoms.len()).expect("`atoms` has too many elements");
        num_atoms.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 22]);
        self.atoms.serialize_into(bytes);
    }
}
impl ListDevicePropertiesReply {
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
#[cfg(test)]
mod list_device_properties_reply {
    use super::ListDevicePropertiesReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ListDevicePropertiesReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                atoms: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15993096344826806272);
        let value = ListDevicePropertiesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PropertyFormat(u8);
impl PropertyFormat {
    pub const M8_BITS: Self = Self(8);
    pub const M16_BITS: Self = Self(16);
    pub const M32_BITS: Self = Self(32);
}
impl From<PropertyFormat> for u8 {
    #[inline]
    fn from(input: PropertyFormat) -> Self {
        input.0
    }
}
impl From<PropertyFormat> for Option<u8> {
    #[inline]
    fn from(input: PropertyFormat) -> Self {
        Some(input.0)
    }
}
impl From<PropertyFormat> for u16 {
    #[inline]
    fn from(input: PropertyFormat) -> Self {
        u16::from(input.0)
    }
}
impl From<PropertyFormat> for Option<u16> {
    #[inline]
    fn from(input: PropertyFormat) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<PropertyFormat> for u32 {
    #[inline]
    fn from(input: PropertyFormat) -> Self {
        u32::from(input.0)
    }
}
impl From<PropertyFormat> for Option<u32> {
    #[inline]
    fn from(input: PropertyFormat) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for PropertyFormat {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for PropertyFormat  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::M8_BITS.0.into(), "M8_BITS", "M8Bits"),
            (Self::M16_BITS.0.into(), "M16_BITS", "M16Bits"),
            (Self::M32_BITS.0.into(), "M32_BITS", "M32Bits"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for PropertyFormat {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::M8_BITS.0,
            Self::M16_BITS.0,
            Self::M32_BITS.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChangeDevicePropertyAux {
    Data8(Vec<u8>),
    Data16(Vec<u16>),
    Data32(Vec<u32>),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u8),
}
impl ChangeDevicePropertyAux {
    fn try_parse(value: &[u8], format: u8, num_items: u32) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = format;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u8::from(PropertyFormat::M8_BITS) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data8, remaining) = crate::x11_utils::parse_u8_list(remaining, num_items.try_to_usize()?)?;
            let data8 = data8.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(ChangeDevicePropertyAux::Data8(data8));
        }
        if switch_expr == u8::from(PropertyFormat::M16_BITS) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data16, remaining) = crate::x11_utils::parse_list::<u16>(remaining, num_items.try_to_usize()?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(ChangeDevicePropertyAux::Data16(data16));
        }
        if switch_expr == u8::from(PropertyFormat::M32_BITS) {
            let remaining = outer_remaining;
            let (data32, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_items.try_to_usize()?)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(ChangeDevicePropertyAux::Data32(data32));
        }
        match parse_result {
            None => Ok((ChangeDevicePropertyAux::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl ChangeDevicePropertyAux {
    pub fn as_data8(&self) -> Option<&Vec<u8>> {
        match self {
            ChangeDevicePropertyAux::Data8(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_data16(&self) -> Option<&Vec<u16>> {
        match self {
            ChangeDevicePropertyAux::Data16(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_data32(&self) -> Option<&Vec<u32>> {
        match self {
            ChangeDevicePropertyAux::Data32(value) => Some(value),
            _ => None,
        }
    }
}
impl ChangeDevicePropertyAux {
    #[allow(dead_code)]
    fn serialize(&self, format: u8, num_items: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, format, num_items);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, format: u8, num_items: u32) {
        let _ = format;
        let _ = num_items;
        match self {
            ChangeDevicePropertyAux::Data8(data8) => {
                assert_eq!(data8.len(), usize::try_from(num_items).unwrap(), "`data8` has an incorrect length");
                bytes.extend_from_slice(&data8);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            ChangeDevicePropertyAux::Data16(data16) => {
                assert_eq!(data16.len(), usize::try_from(num_items).unwrap(), "`data16` has an incorrect length");
                data16.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            ChangeDevicePropertyAux::Data32(data32) => {
                assert_eq!(data32.len(), usize::try_from(num_items).unwrap(), "`data32` has an incorrect length");
                data32.serialize_into(bytes);
            }
            ChangeDevicePropertyAux::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl ChangeDevicePropertyAux {
    fn switch_expr(&self) -> u8 {
        match self {
            ChangeDevicePropertyAux::Data8(_) => u8::from(PropertyFormat::M8_BITS),
            ChangeDevicePropertyAux::Data16(_) => u8::from(PropertyFormat::M16_BITS),
            ChangeDevicePropertyAux::Data32(_) => u8::from(PropertyFormat::M32_BITS),
            ChangeDevicePropertyAux::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod change_device_property_aux {
    use super::ChangeDevicePropertyAux;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ChangeDevicePropertyAux {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..3) {
                0 => Self::Data8(GenRandom::generate(rng)),
                1 => Self::Data16(GenRandom::generate(rng)),
                _ => Self::Data32(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<ChangeDevicePropertyAux> {
        alloc::vec![
            ChangeDevicePropertyAux::Data8(GenRandom::generate(rng)),
            ChangeDevicePropertyAux::Data16(GenRandom::generate(rng)),
            ChangeDevicePropertyAux::Data32(GenRandom::generate(rng)),
        ]
    }
}

/// Opcode for the ChangeDeviceProperty request
pub const CHANGE_DEVICE_PROPERTY_REQUEST: u8 = 37;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDevicePropertyRequest<'input> {
    pub property: xproto::Atom,
    pub type_: xproto::Atom,
    pub device_id: u8,
    pub mode: xproto::PropMode,
    pub num_items: u32,
    pub items: Cow<'input, ChangeDevicePropertyAux>,
}
impl<'input> ChangeDevicePropertyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let property_bytes = self.property.serialize();
        let type_bytes = self.type_.serialize();
        let device_id_bytes = self.device_id.serialize();
        let format: u8 = self.items.switch_expr();
        let format_bytes = format.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
        let num_items_bytes = self.num_items.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_DEVICE_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let items_bytes = self.items.serialize(format, self.num_items);
        let length_so_far = length_so_far + items_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), items_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_DEVICE_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (property, remaining) = xproto::Atom::try_parse(value)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let mode = mode.into();
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let (items, remaining) = ChangeDevicePropertyAux::try_parse(remaining, format, num_items)?;
        let _ = remaining;
        Ok(ChangeDevicePropertyRequest {
            property,
            type_,
            device_id,
            mode,
            num_items,
            items: Cow::Owned(items),
        })
    }
    /// Clone all borrowed data in this ChangeDevicePropertyRequest.
    pub fn into_owned(self) -> ChangeDevicePropertyRequest<'static> {
        ChangeDevicePropertyRequest {
            property: self.property,
            type_: self.type_,
            device_id: self.device_id,
            mode: self.mode,
            num_items: self.num_items,
            items: Cow::Owned(self.items.into_owned()),
        }
    }
}
impl<'input> Request for ChangeDevicePropertyRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for ChangeDevicePropertyRequest<'input> {
}

/// Opcode for the DeleteDeviceProperty request
pub const DELETE_DEVICE_PROPERTY_REQUEST: u8 = 38;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteDevicePropertyRequest {
    pub property: xproto::Atom,
    pub device_id: u8,
}
impl DeleteDevicePropertyRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let property_bytes = self.property.serialize();
        let device_id_bytes = self.device_id.serialize();
        let mut request0 = vec![
            major_opcode,
            DELETE_DEVICE_PROPERTY_REQUEST,
            0,
            0,
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
            device_id_bytes[0],
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
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DELETE_DEVICE_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (property, remaining) = xproto::Atom::try_parse(value)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(DeleteDevicePropertyRequest {
            property,
            device_id,
        })
    }
}
impl Request for DeleteDevicePropertyRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DeleteDevicePropertyRequest {
}

/// Opcode for the GetDeviceProperty request
pub const GET_DEVICE_PROPERTY_REQUEST: u8 = 39;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDevicePropertyRequest {
    pub property: xproto::Atom,
    pub type_: xproto::Atom,
    pub offset: u32,
    pub len: u32,
    pub device_id: u8,
    pub delete: bool,
}
impl GetDevicePropertyRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let property_bytes = self.property.serialize();
        let type_bytes = self.type_.serialize();
        let offset_bytes = self.offset.serialize();
        let len_bytes = self.len.serialize();
        let device_id_bytes = self.device_id.serialize();
        let delete_bytes = self.delete.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_PROPERTY_REQUEST,
            0,
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
            device_id_bytes[0],
            delete_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (property, remaining) = xproto::Atom::try_parse(value)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let (len, remaining) = u32::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (delete, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetDevicePropertyRequest {
            property,
            type_,
            offset,
            len,
            device_id,
            delete,
        })
    }
}
impl Request for GetDevicePropertyRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDevicePropertyRequest {
    type Reply = GetDevicePropertyReply;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GetDevicePropertyItems {
    Data8(Vec<u8>),
    Data16(Vec<u16>),
    Data32(Vec<u32>),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u8),
}
impl GetDevicePropertyItems {
    fn try_parse(value: &[u8], format: u8, num_items: u32) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = format;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u8::from(PropertyFormat::M8_BITS) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data8, remaining) = crate::x11_utils::parse_u8_list(remaining, num_items.try_to_usize()?)?;
            let data8 = data8.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(GetDevicePropertyItems::Data8(data8));
        }
        if switch_expr == u8::from(PropertyFormat::M16_BITS) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data16, remaining) = crate::x11_utils::parse_list::<u16>(remaining, num_items.try_to_usize()?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(GetDevicePropertyItems::Data16(data16));
        }
        if switch_expr == u8::from(PropertyFormat::M32_BITS) {
            let remaining = outer_remaining;
            let (data32, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_items.try_to_usize()?)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(GetDevicePropertyItems::Data32(data32));
        }
        match parse_result {
            None => Ok((GetDevicePropertyItems::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl GetDevicePropertyItems {
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
impl GetDevicePropertyItems {
    #[allow(dead_code)]
    fn serialize(&self, format: u8, num_items: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, format, num_items);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, format: u8, num_items: u32) {
        let _ = format;
        let _ = num_items;
        match self {
            GetDevicePropertyItems::Data8(data8) => {
                assert_eq!(data8.len(), usize::try_from(num_items).unwrap(), "`data8` has an incorrect length");
                bytes.extend_from_slice(&data8);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            GetDevicePropertyItems::Data16(data16) => {
                assert_eq!(data16.len(), usize::try_from(num_items).unwrap(), "`data16` has an incorrect length");
                data16.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            GetDevicePropertyItems::Data32(data32) => {
                assert_eq!(data32.len(), usize::try_from(num_items).unwrap(), "`data32` has an incorrect length");
                data32.serialize_into(bytes);
            }
            GetDevicePropertyItems::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl GetDevicePropertyItems {
    fn switch_expr(&self) -> u8 {
        match self {
            GetDevicePropertyItems::Data8(_) => u8::from(PropertyFormat::M8_BITS),
            GetDevicePropertyItems::Data16(_) => u8::from(PropertyFormat::M16_BITS),
            GetDevicePropertyItems::Data32(_) => u8::from(PropertyFormat::M32_BITS),
            GetDevicePropertyItems::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod get_device_property_items {
    use super::GetDevicePropertyItems;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDevicePropertyItems {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..3) {
                0 => Self::Data8(GenRandom::generate(rng)),
                1 => Self::Data16(GenRandom::generate(rng)),
                _ => Self::Data32(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<GetDevicePropertyItems> {
        alloc::vec![
            GetDevicePropertyItems::Data8(GenRandom::generate(rng)),
            GetDevicePropertyItems::Data16(GenRandom::generate(rng)),
            GetDevicePropertyItems::Data32(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDevicePropertyReply {
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xproto::Atom,
    pub bytes_after: u32,
    pub num_items: u32,
    pub device_id: u8,
    pub items: GetDevicePropertyItems,
}
impl TryParse for GetDevicePropertyReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xi_reply_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(10..).ok_or(ParseError::InsufficientData)?;
        let (items, remaining) = GetDevicePropertyItems::try_parse(remaining, format, num_items)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDevicePropertyReply { xi_reply_type, sequence, length, type_, bytes_after, num_items, device_id, items };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDevicePropertyReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        self.xi_reply_type.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.type_.serialize_into(bytes);
        self.bytes_after.serialize_into(bytes);
        self.num_items.serialize_into(bytes);
        let format: u8 = self.items.switch_expr();
        format.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 10]);
        self.items.serialize_into(bytes, format, self.num_items);
    }
}
#[cfg(test)]
mod get_device_property_reply {
    use super::GetDevicePropertyReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GetDevicePropertyReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                xi_reply_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                type_: GenRandom::generate(rng),
                bytes_after: GenRandom::generate(rng),
                num_items: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                items: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5555853434759938048);
        let value = GetDevicePropertyReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Device(bool);
impl Device {
    pub const ALL: Self = Self(false);
    pub const ALL_MASTER: Self = Self(true);
}
impl From<Device> for bool {
    #[inline]
    fn from(input: Device) -> Self {
        input.0
    }
}
impl From<Device> for Option<bool> {
    #[inline]
    fn from(input: Device) -> Self {
        Some(input.0)
    }
}
impl From<Device> for u8 {
    #[inline]
    fn from(input: Device) -> Self {
        u8::from(input.0)
    }
}
impl From<Device> for Option<u8> {
    #[inline]
    fn from(input: Device) -> Self {
        Some(u8::from(input.0))
    }
}
impl From<Device> for u16 {
    #[inline]
    fn from(input: Device) -> Self {
        u16::from(input.0)
    }
}
impl From<Device> for Option<u16> {
    #[inline]
    fn from(input: Device) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Device> for u32 {
    #[inline]
    fn from(input: Device) -> Self {
        u32::from(input.0)
    }
}
impl From<Device> for Option<u32> {
    #[inline]
    fn from(input: Device) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<bool> for Device {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for Device  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ALL.0.into(), "ALL", "All"),
            (Self::ALL_MASTER.0.into(), "ALL_MASTER", "AllMaster"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for Device {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ALL.0,
            Self::ALL_MASTER.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for GroupInfo {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
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
#[cfg(test)]
mod group_info {
    use super::GroupInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GroupInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                base: GenRandom::generate(rng),
                latched: GenRandom::generate(rng),
                locked: GenRandom::generate(rng),
                effective: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1070357259881669760);
        let value = GroupInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for ModifierInfo {
    type Bytes = [u8; 16];
    fn serialize(&self) -> [u8; 16] {
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
#[cfg(test)]
mod modifier_info {
    use super::ModifierInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ModifierInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                base: GenRandom::generate(rng),
                latched: GenRandom::generate(rng),
                locked: GenRandom::generate(rng),
                effective: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15320974152793160128);
        let value = ModifierInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIQueryPointer request
pub const XI_QUERY_POINTER_REQUEST: u8 = 40;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIQueryPointerRequest {
    pub window: xproto::Window,
    pub deviceid: DeviceId,
}
impl XIQueryPointerRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_QUERY_POINTER_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_QUERY_POINTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XIQueryPointerRequest {
            window,
            deviceid,
        })
    }
}
impl Request for XIQueryPointerRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for XIQueryPointerRequest {
    type Reply = XIQueryPointerReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIQueryPointerReply {
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
impl TryParse for XIQueryPointerReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let (child, remaining) = xproto::Window::try_parse(remaining)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (win_x, remaining) = Fp1616::try_parse(remaining)?;
        let (win_y, remaining) = Fp1616::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (buttons_len, remaining) = u16::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (buttons, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIQueryPointerReply { sequence, length, root, child, root_x, root_y, win_x, win_y, same_screen, mods, group, buttons };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIQueryPointerReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(56);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.root.serialize_into(bytes);
        self.child.serialize_into(bytes);
        self.root_x.serialize_into(bytes);
        self.root_y.serialize_into(bytes);
        self.win_x.serialize_into(bytes);
        self.win_y.serialize_into(bytes);
        self.same_screen.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        let buttons_len = u16::try_from(self.buttons.len()).expect("`buttons` has too many elements");
        buttons_len.serialize_into(bytes);
        self.mods.serialize_into(bytes);
        self.group.serialize_into(bytes);
        self.buttons.serialize_into(bytes);
    }
}
impl XIQueryPointerReply {
    /// Get the value of the `buttons_len` field.
    ///
    /// The `buttons_len` field is used as the length field of the `buttons` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn buttons_len(&self) -> u16 {
        self.buttons.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod xi_query_pointer_reply {
    use super::XIQueryPointerReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIQueryPointerReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                root: GenRandom::generate(rng),
                child: GenRandom::generate(rng),
                root_x: GenRandom::generate(rng),
                root_y: GenRandom::generate(rng),
                win_x: GenRandom::generate(rng),
                win_y: GenRandom::generate(rng),
                same_screen: GenRandom::generate(rng),
                mods: GenRandom::generate(rng),
                group: GenRandom::generate(rng),
                buttons: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2281982500122656768);
        let value = XIQueryPointerReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIWarpPointer request
pub const XI_WARP_POINTER_REQUEST: u8 = 41;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIWarpPointerRequest {
    pub src_win: xproto::Window,
    pub dst_win: xproto::Window,
    pub src_x: Fp1616,
    pub src_y: Fp1616,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: Fp1616,
    pub dst_y: Fp1616,
    pub deviceid: DeviceId,
}
impl XIWarpPointerRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let src_win_bytes = self.src_win.serialize();
        let dst_win_bytes = self.dst_win.serialize();
        let src_x_bytes = self.src_x.serialize();
        let src_y_bytes = self.src_y.serialize();
        let src_width_bytes = self.src_width.serialize();
        let src_height_bytes = self.src_height.serialize();
        let dst_x_bytes = self.dst_x.serialize();
        let dst_y_bytes = self.dst_y.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_WARP_POINTER_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_WARP_POINTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (src_win, remaining) = xproto::Window::try_parse(value)?;
        let (dst_win, remaining) = xproto::Window::try_parse(remaining)?;
        let (src_x, remaining) = Fp1616::try_parse(remaining)?;
        let (src_y, remaining) = Fp1616::try_parse(remaining)?;
        let (src_width, remaining) = u16::try_parse(remaining)?;
        let (src_height, remaining) = u16::try_parse(remaining)?;
        let (dst_x, remaining) = Fp1616::try_parse(remaining)?;
        let (dst_y, remaining) = Fp1616::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XIWarpPointerRequest {
            src_win,
            dst_win,
            src_x,
            src_y,
            src_width,
            src_height,
            dst_x,
            dst_y,
            deviceid,
        })
    }
}
impl Request for XIWarpPointerRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for XIWarpPointerRequest {
}

/// Opcode for the XIChangeCursor request
pub const XI_CHANGE_CURSOR_REQUEST: u8 = 42;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIChangeCursorRequest {
    pub window: xproto::Window,
    pub cursor: xproto::Cursor,
    pub deviceid: DeviceId,
}
impl XIChangeCursorRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let cursor_bytes = self.cursor.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_CHANGE_CURSOR_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_CHANGE_CURSOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (cursor, remaining) = xproto::Cursor::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XIChangeCursorRequest {
            window,
            cursor,
            deviceid,
        })
    }
}
impl Request for XIChangeCursorRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for XIChangeCursorRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HierarchyChangeType(u16);
impl HierarchyChangeType {
    pub const ADD_MASTER: Self = Self(1);
    pub const REMOVE_MASTER: Self = Self(2);
    pub const ATTACH_SLAVE: Self = Self(3);
    pub const DETACH_SLAVE: Self = Self(4);
}
impl From<HierarchyChangeType> for u16 {
    #[inline]
    fn from(input: HierarchyChangeType) -> Self {
        input.0
    }
}
impl From<HierarchyChangeType> for Option<u16> {
    #[inline]
    fn from(input: HierarchyChangeType) -> Self {
        Some(input.0)
    }
}
impl From<HierarchyChangeType> for u32 {
    #[inline]
    fn from(input: HierarchyChangeType) -> Self {
        u32::from(input.0)
    }
}
impl From<HierarchyChangeType> for Option<u32> {
    #[inline]
    fn from(input: HierarchyChangeType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for HierarchyChangeType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for HierarchyChangeType {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for HierarchyChangeType  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ADD_MASTER.0.into(), "ADD_MASTER", "AddMaster"),
            (Self::REMOVE_MASTER.0.into(), "REMOVE_MASTER", "RemoveMaster"),
            (Self::ATTACH_SLAVE.0.into(), "ATTACH_SLAVE", "AttachSlave"),
            (Self::DETACH_SLAVE.0.into(), "DETACH_SLAVE", "DetachSlave"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for HierarchyChangeType {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ADD_MASTER.0,
            Self::REMOVE_MASTER.0,
            Self::ATTACH_SLAVE.0,
            Self::DETACH_SLAVE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeMode(u8);
impl ChangeMode {
    pub const ATTACH: Self = Self(1);
    pub const FLOAT: Self = Self(2);
}
impl From<ChangeMode> for u8 {
    #[inline]
    fn from(input: ChangeMode) -> Self {
        input.0
    }
}
impl From<ChangeMode> for Option<u8> {
    #[inline]
    fn from(input: ChangeMode) -> Self {
        Some(input.0)
    }
}
impl From<ChangeMode> for u16 {
    #[inline]
    fn from(input: ChangeMode) -> Self {
        u16::from(input.0)
    }
}
impl From<ChangeMode> for Option<u16> {
    #[inline]
    fn from(input: ChangeMode) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ChangeMode> for u32 {
    #[inline]
    fn from(input: ChangeMode) -> Self {
        u32::from(input.0)
    }
}
impl From<ChangeMode> for Option<u32> {
    #[inline]
    fn from(input: ChangeMode) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ChangeMode {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ChangeMode  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ATTACH.0.into(), "ATTACH", "Attach"),
            (Self::FLOAT.0.into(), "FLOAT", "Float"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for ChangeMode {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ATTACH.0,
            Self::FLOAT.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = AddMaster { type_, len, send_core, enable, name };
        Ok((result, remaining))
    }
}
impl Serialize for AddMaster {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        let name_len = u16::try_from(self.name.len()).expect("`name` has too many elements");
        name_len.serialize_into(bytes);
        self.send_core.serialize_into(bytes);
        self.enable.serialize_into(bytes);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl AddMaster {
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
#[cfg(test)]
mod add_master {
    use super::AddMaster;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for AddMaster {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                send_core: GenRandom::generate(rng),
                enable: GenRandom::generate(rng),
                name: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(745689230286000000);
        let value = AddMaster::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (return_pointer, remaining) = DeviceId::try_parse(remaining)?;
        let (return_keyboard, remaining) = DeviceId::try_parse(remaining)?;
        let type_ = type_.into();
        let return_mode = return_mode.into();
        let result = RemoveMaster { type_, len, deviceid, return_mode, return_pointer, return_keyboard };
        Ok((result, remaining))
    }
}
impl Serialize for RemoveMaster {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let type_bytes = u16::from(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let return_mode_bytes = u8::from(self.return_mode).serialize();
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
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        u8::from(self.return_mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.return_pointer.serialize_into(bytes);
        self.return_keyboard.serialize_into(bytes);
    }
}
#[cfg(test)]
mod remove_master {
    use super::RemoveMaster;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for RemoveMaster {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                return_mode: GenRandom::generate(rng),
                return_pointer: GenRandom::generate(rng),
                return_keyboard: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(16717092684521509856);
        let value = RemoveMaster::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let type_ = type_.into();
        let result = AttachSlave { type_, len, deviceid, master };
        Ok((result, remaining))
    }
}
impl Serialize for AttachSlave {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u16::from(self.type_).serialize();
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
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.master.serialize_into(bytes);
    }
}
#[cfg(test)]
mod attach_slave {
    use super::AttachSlave;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for AttachSlave {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                master: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13137531740200494080);
        let value = AttachSlave::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = DetachSlave { type_, len, deviceid };
        Ok((result, remaining))
    }
}
impl Serialize for DetachSlave {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u16::from(self.type_).serialize();
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
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}
#[cfg(test)]
mod detach_slave {
    use super::DetachSlave;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DetachSlave {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(18034017678315209728);
        let value = DetachSlave::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let result = HierarchyChangeDataAddMaster { send_core, enable, name };
        Ok((result, remaining))
    }
}
impl Serialize for HierarchyChangeDataAddMaster {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let name_len = u16::try_from(self.name.len()).expect("`name` has too many elements");
        name_len.serialize_into(bytes);
        self.send_core.serialize_into(bytes);
        self.enable.serialize_into(bytes);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl HierarchyChangeDataAddMaster {
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
#[cfg(test)]
mod hierarchy_change_data_add_master {
    use super::HierarchyChangeDataAddMaster;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for HierarchyChangeDataAddMaster {
        fn generate(rng: &Rng) -> Self {
            Self {
                send_core: GenRandom::generate(rng),
                enable: GenRandom::generate(rng),
                name: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2437369091569221632);
        let value = HierarchyChangeDataAddMaster::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (return_pointer, remaining) = DeviceId::try_parse(remaining)?;
        let (return_keyboard, remaining) = DeviceId::try_parse(remaining)?;
        let return_mode = return_mode.into();
        let result = HierarchyChangeDataRemoveMaster { deviceid, return_mode, return_pointer, return_keyboard };
        Ok((result, remaining))
    }
}
impl Serialize for HierarchyChangeDataRemoveMaster {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let deviceid_bytes = self.deviceid.serialize();
        let return_mode_bytes = u8::from(self.return_mode).serialize();
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
        u8::from(self.return_mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.return_pointer.serialize_into(bytes);
        self.return_keyboard.serialize_into(bytes);
    }
}
#[cfg(test)]
mod hierarchy_change_data_remove_master {
    use super::HierarchyChangeDataRemoveMaster;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for HierarchyChangeDataRemoveMaster {
        fn generate(rng: &Rng) -> Self {
            Self {
                deviceid: GenRandom::generate(rng),
                return_mode: GenRandom::generate(rng),
                return_pointer: GenRandom::generate(rng),
                return_keyboard: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9958354662981632000);
        let value = HierarchyChangeDataRemoveMaster::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for HierarchyChangeDataAttachSlave {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
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
#[cfg(test)]
mod hierarchy_change_data_attach_slave {
    use super::HierarchyChangeDataAttachSlave;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for HierarchyChangeDataAttachSlave {
        fn generate(rng: &Rng) -> Self {
            Self {
                deviceid: GenRandom::generate(rng),
                master: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14361225236953694208);
        let value = HierarchyChangeDataAttachSlave::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HierarchyChangeDataDetachSlave {
    pub deviceid: DeviceId,
}
impl TryParse for HierarchyChangeDataDetachSlave {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let result = HierarchyChangeDataDetachSlave { deviceid };
        Ok((result, remaining))
    }
}
impl Serialize for HierarchyChangeDataDetachSlave {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let deviceid_bytes = self.deviceid.serialize();
        [
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.deviceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}
#[cfg(test)]
mod hierarchy_change_data_detach_slave {
    use super::HierarchyChangeDataDetachSlave;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for HierarchyChangeDataDetachSlave {
        fn generate(rng: &Rng) -> Self {
            Self {
                deviceid: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11750380126167105536);
        let value = HierarchyChangeDataDetachSlave::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HierarchyChangeData {
    AddMaster(HierarchyChangeDataAddMaster),
    RemoveMaster(HierarchyChangeDataRemoveMaster),
    AttachSlave(HierarchyChangeDataAttachSlave),
    DetachSlave(HierarchyChangeDataDetachSlave),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u16),
}
impl HierarchyChangeData {
    fn try_parse(value: &[u8], type_: u16) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = type_;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u16::from(HierarchyChangeType::ADD_MASTER) {
            let (add_master, new_remaining) = HierarchyChangeDataAddMaster::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(HierarchyChangeData::AddMaster(add_master));
        }
        if switch_expr == u16::from(HierarchyChangeType::REMOVE_MASTER) {
            let (remove_master, new_remaining) = HierarchyChangeDataRemoveMaster::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(HierarchyChangeData::RemoveMaster(remove_master));
        }
        if switch_expr == u16::from(HierarchyChangeType::ATTACH_SLAVE) {
            let (attach_slave, new_remaining) = HierarchyChangeDataAttachSlave::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(HierarchyChangeData::AttachSlave(attach_slave));
        }
        if switch_expr == u16::from(HierarchyChangeType::DETACH_SLAVE) {
            let (detach_slave, new_remaining) = HierarchyChangeDataDetachSlave::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(HierarchyChangeData::DetachSlave(detach_slave));
        }
        match parse_result {
            None => Ok((HierarchyChangeData::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl HierarchyChangeData {
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
    pub fn as_detach_slave(&self) -> Option<&HierarchyChangeDataDetachSlave> {
        match self {
            HierarchyChangeData::DetachSlave(value) => Some(value),
            _ => None,
        }
    }
}
impl HierarchyChangeData {
    #[allow(dead_code)]
    fn serialize(&self, type_: u16) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, type_);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, type_: u16) {
        let _ = type_;
        match self {
            HierarchyChangeData::AddMaster(add_master) => add_master.serialize_into(bytes),
            HierarchyChangeData::RemoveMaster(remove_master) => remove_master.serialize_into(bytes),
            HierarchyChangeData::AttachSlave(attach_slave) => attach_slave.serialize_into(bytes),
            HierarchyChangeData::DetachSlave(detach_slave) => detach_slave.serialize_into(bytes),
            HierarchyChangeData::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl HierarchyChangeData {
    fn switch_expr(&self) -> u16 {
        match self {
            HierarchyChangeData::AddMaster(_) => u16::from(HierarchyChangeType::ADD_MASTER),
            HierarchyChangeData::RemoveMaster(_) => u16::from(HierarchyChangeType::REMOVE_MASTER),
            HierarchyChangeData::AttachSlave(_) => u16::from(HierarchyChangeType::ATTACH_SLAVE),
            HierarchyChangeData::DetachSlave(_) => u16::from(HierarchyChangeType::DETACH_SLAVE),
            HierarchyChangeData::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod hierarchy_change_data {
    use super::HierarchyChangeData;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for HierarchyChangeData {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..4) {
                0 => Self::AddMaster(GenRandom::generate(rng)),
                1 => Self::RemoveMaster(GenRandom::generate(rng)),
                2 => Self::AttachSlave(GenRandom::generate(rng)),
                _ => Self::DetachSlave(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<HierarchyChangeData> {
        alloc::vec![
            HierarchyChangeData::AddMaster(GenRandom::generate(rng)),
            HierarchyChangeData::RemoveMaster(GenRandom::generate(rng)),
            HierarchyChangeData::AttachSlave(GenRandom::generate(rng)),
            HierarchyChangeData::DetachSlave(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HierarchyChange {
    pub len: u16,
    pub data: HierarchyChangeData,
}
impl TryParse for HierarchyChange {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = HierarchyChangeData::try_parse(remaining, type_)?;
        let result = HierarchyChange { len, data };
        Ok((result, remaining))
    }
}
impl Serialize for HierarchyChange {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let type_: u16 = self.data.switch_expr();
        type_.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.data.serialize_into(bytes, type_);
    }
}
#[cfg(test)]
mod hierarchy_change {
    use super::HierarchyChange;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for HierarchyChange {
        fn generate(rng: &Rng) -> Self {
            Self {
                len: GenRandom::generate(rng),
                data: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14652501235968118784);
        let value = HierarchyChange::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIChangeHierarchy request
pub const XI_CHANGE_HIERARCHY_REQUEST: u8 = 43;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIChangeHierarchyRequest<'input> {
    pub changes: Cow<'input, [HierarchyChange]>,
}
impl<'input> XIChangeHierarchyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let num_changes = u8::try_from(self.changes.len()).expect("`changes` has too many elements");
        let num_changes_bytes = num_changes.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_CHANGE_HIERARCHY_REQUEST,
            0,
            0,
            num_changes_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let changes_bytes = self.changes.serialize();
        let length_so_far = length_so_far + changes_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), changes_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_CHANGE_HIERARCHY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (num_changes, remaining) = u8::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (changes, remaining) = crate::x11_utils::parse_list::<HierarchyChange>(remaining, num_changes.try_to_usize()?)?;
        let _ = remaining;
        Ok(XIChangeHierarchyRequest {
            changes: Cow::Owned(changes),
        })
    }
    /// Clone all borrowed data in this XIChangeHierarchyRequest.
    pub fn into_owned(self) -> XIChangeHierarchyRequest<'static> {
        XIChangeHierarchyRequest {
            changes: Cow::Owned(self.changes.into_owned()),
        }
    }
}
impl<'input> Request for XIChangeHierarchyRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for XIChangeHierarchyRequest<'input> {
}

/// Opcode for the XISetClientPointer request
pub const XI_SET_CLIENT_POINTER_REQUEST: u8 = 44;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XISetClientPointerRequest {
    pub window: xproto::Window,
    pub deviceid: DeviceId,
}
impl XISetClientPointerRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_SET_CLIENT_POINTER_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_SET_CLIENT_POINTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XISetClientPointerRequest {
            window,
            deviceid,
        })
    }
}
impl Request for XISetClientPointerRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for XISetClientPointerRequest {
}

/// Opcode for the XIGetClientPointer request
pub const XI_GET_CLIENT_POINTER_REQUEST: u8 = 45;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGetClientPointerRequest {
    pub window: xproto::Window,
}
impl XIGetClientPointerRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_GET_CLIENT_POINTER_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_GET_CLIENT_POINTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(XIGetClientPointerRequest {
            window,
        })
    }
}
impl Request for XIGetClientPointerRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for XIGetClientPointerRequest {
    type Reply = XIGetClientPointerReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGetClientPointerReply {
    pub sequence: u16,
    pub length: u32,
    pub set: bool,
    pub deviceid: DeviceId,
}
impl TryParse for XIGetClientPointerReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (set, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIGetClientPointerReply { sequence, length, set, deviceid };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIGetClientPointerReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let set_bytes = self.set.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            set_bytes[0],
            0,
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.set.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.deviceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}
#[cfg(test)]
mod xi_get_client_pointer_reply {
    use super::XIGetClientPointerReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIGetClientPointerReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                set: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2207258019937386496);
        let value = XIGetClientPointerReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIEventMask(u32);
impl XIEventMask {
    pub const DEVICE_CHANGED: Self = Self(1 << 1);
    pub const KEY_PRESS: Self = Self(1 << 2);
    pub const KEY_RELEASE: Self = Self(1 << 3);
    pub const BUTTON_PRESS: Self = Self(1 << 4);
    pub const BUTTON_RELEASE: Self = Self(1 << 5);
    pub const MOTION: Self = Self(1 << 6);
    pub const ENTER: Self = Self(1 << 7);
    pub const LEAVE: Self = Self(1 << 8);
    pub const FOCUS_IN: Self = Self(1 << 9);
    pub const FOCUS_OUT: Self = Self(1 << 10);
    pub const HIERARCHY: Self = Self(1 << 11);
    pub const PROPERTY: Self = Self(1 << 12);
    pub const RAW_KEY_PRESS: Self = Self(1 << 13);
    pub const RAW_KEY_RELEASE: Self = Self(1 << 14);
    pub const RAW_BUTTON_PRESS: Self = Self(1 << 15);
    pub const RAW_BUTTON_RELEASE: Self = Self(1 << 16);
    pub const RAW_MOTION: Self = Self(1 << 17);
    pub const TOUCH_BEGIN: Self = Self(1 << 18);
    pub const TOUCH_UPDATE: Self = Self(1 << 19);
    pub const TOUCH_END: Self = Self(1 << 20);
    pub const TOUCH_OWNERSHIP: Self = Self(1 << 21);
    pub const RAW_TOUCH_BEGIN: Self = Self(1 << 22);
    pub const RAW_TOUCH_UPDATE: Self = Self(1 << 23);
    pub const RAW_TOUCH_END: Self = Self(1 << 24);
    pub const BARRIER_HIT: Self = Self(1 << 25);
    pub const BARRIER_LEAVE: Self = Self(1 << 26);
}
impl From<XIEventMask> for u32 {
    #[inline]
    fn from(input: XIEventMask) -> Self {
        input.0
    }
}
impl From<XIEventMask> for Option<u32> {
    #[inline]
    fn from(input: XIEventMask) -> Self {
        Some(input.0)
    }
}
impl From<u8> for XIEventMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for XIEventMask {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for XIEventMask {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for XIEventMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::DEVICE_CHANGED.0, "DEVICE_CHANGED", "DeviceChanged"),
            (Self::KEY_PRESS.0, "KEY_PRESS", "KeyPress"),
            (Self::KEY_RELEASE.0, "KEY_RELEASE", "KeyRelease"),
            (Self::BUTTON_PRESS.0, "BUTTON_PRESS", "ButtonPress"),
            (Self::BUTTON_RELEASE.0, "BUTTON_RELEASE", "ButtonRelease"),
            (Self::MOTION.0, "MOTION", "Motion"),
            (Self::ENTER.0, "ENTER", "Enter"),
            (Self::LEAVE.0, "LEAVE", "Leave"),
            (Self::FOCUS_IN.0, "FOCUS_IN", "FocusIn"),
            (Self::FOCUS_OUT.0, "FOCUS_OUT", "FocusOut"),
            (Self::HIERARCHY.0, "HIERARCHY", "Hierarchy"),
            (Self::PROPERTY.0, "PROPERTY", "Property"),
            (Self::RAW_KEY_PRESS.0, "RAW_KEY_PRESS", "RawKeyPress"),
            (Self::RAW_KEY_RELEASE.0, "RAW_KEY_RELEASE", "RawKeyRelease"),
            (Self::RAW_BUTTON_PRESS.0, "RAW_BUTTON_PRESS", "RawButtonPress"),
            (Self::RAW_BUTTON_RELEASE.0, "RAW_BUTTON_RELEASE", "RawButtonRelease"),
            (Self::RAW_MOTION.0, "RAW_MOTION", "RawMotion"),
            (Self::TOUCH_BEGIN.0, "TOUCH_BEGIN", "TouchBegin"),
            (Self::TOUCH_UPDATE.0, "TOUCH_UPDATE", "TouchUpdate"),
            (Self::TOUCH_END.0, "TOUCH_END", "TouchEnd"),
            (Self::TOUCH_OWNERSHIP.0, "TOUCH_OWNERSHIP", "TouchOwnership"),
            (Self::RAW_TOUCH_BEGIN.0, "RAW_TOUCH_BEGIN", "RawTouchBegin"),
            (Self::RAW_TOUCH_UPDATE.0, "RAW_TOUCH_UPDATE", "RawTouchUpdate"),
            (Self::RAW_TOUCH_END.0, "RAW_TOUCH_END", "RawTouchEnd"),
            (Self::BARRIER_HIT.0, "BARRIER_HIT", "BarrierHit"),
            (Self::BARRIER_LEAVE.0, "BARRIER_LEAVE", "BarrierLeave"),
        ];
        pretty_print_bitmask(fmt, self.0, &variants)
    }
}
bitmask_binop!(XIEventMask, u32);
#[cfg(test)]
impl crate::x11_utils::GenRandom for XIEventMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::DEVICE_CHANGED.0,
            Self::KEY_PRESS.0,
            Self::KEY_RELEASE.0,
            Self::BUTTON_PRESS.0,
            Self::BUTTON_RELEASE.0,
            Self::MOTION.0,
            Self::ENTER.0,
            Self::LEAVE.0,
            Self::FOCUS_IN.0,
            Self::FOCUS_OUT.0,
            Self::HIERARCHY.0,
            Self::PROPERTY.0,
            Self::RAW_KEY_PRESS.0,
            Self::RAW_KEY_RELEASE.0,
            Self::RAW_BUTTON_PRESS.0,
            Self::RAW_BUTTON_RELEASE.0,
            Self::RAW_MOTION.0,
            Self::TOUCH_BEGIN.0,
            Self::TOUCH_UPDATE.0,
            Self::TOUCH_END.0,
            Self::TOUCH_OWNERSHIP.0,
            Self::RAW_TOUCH_BEGIN.0,
            Self::RAW_TOUCH_UPDATE.0,
            Self::RAW_TOUCH_END.0,
            Self::BARRIER_HIT.0,
            Self::BARRIER_LEAVE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventMask {
    pub deviceid: DeviceId,
    pub mask: Vec<u32>,
}
impl TryParse for EventMask {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (mask_len, remaining) = u16::try_parse(remaining)?;
        let (mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, mask_len.try_to_usize()?)?;
        let result = EventMask { deviceid, mask };
        Ok((result, remaining))
    }
}
impl Serialize for EventMask {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.deviceid.serialize_into(bytes);
        let mask_len = u16::try_from(self.mask.len()).expect("`mask` has too many elements");
        mask_len.serialize_into(bytes);
        self.mask.serialize_into(bytes);
    }
}
impl EventMask {
    /// Get the value of the `mask_len` field.
    ///
    /// The `mask_len` field is used as the length field of the `mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn mask_len(&self) -> u16 {
        self.mask.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod event_mask {
    use super::EventMask;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for EventMask {
        fn generate(rng: &Rng) -> Self {
            Self {
                deviceid: GenRandom::generate(rng),
                mask: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(964377842940296400);
        let value = EventMask::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XISelectEvents request
pub const XI_SELECT_EVENTS_REQUEST: u8 = 46;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XISelectEventsRequest<'input> {
    pub window: xproto::Window,
    pub masks: Cow<'input, [EventMask]>,
}
impl<'input> XISelectEventsRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let num_mask = u16::try_from(self.masks.len()).expect("`masks` has too many elements");
        let num_mask_bytes = num_mask.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_SELECT_EVENTS_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            num_mask_bytes[0],
            num_mask_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let masks_bytes = self.masks.serialize();
        let length_so_far = length_so_far + masks_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), masks_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_SELECT_EVENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (num_mask, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (masks, remaining) = crate::x11_utils::parse_list::<EventMask>(remaining, num_mask.try_to_usize()?)?;
        let _ = remaining;
        Ok(XISelectEventsRequest {
            window,
            masks: Cow::Owned(masks),
        })
    }
    /// Clone all borrowed data in this XISelectEventsRequest.
    pub fn into_owned(self) -> XISelectEventsRequest<'static> {
        XISelectEventsRequest {
            window: self.window,
            masks: Cow::Owned(self.masks.into_owned()),
        }
    }
}
impl<'input> Request for XISelectEventsRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for XISelectEventsRequest<'input> {
}

/// Opcode for the XIQueryVersion request
pub const XI_QUERY_VERSION_REQUEST: u8 = 47;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIQueryVersionRequest {
    pub major_version: u16,
    pub minor_version: u16,
}
impl XIQueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_QUERY_VERSION_REQUEST,
            0,
            0,
            major_version_bytes[0],
            major_version_bytes[1],
            minor_version_bytes[0],
            minor_version_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (major_version, remaining) = u16::try_parse(value)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(XIQueryVersionRequest {
            major_version,
            minor_version,
        })
    }
}
impl Request for XIQueryVersionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for XIQueryVersionRequest {
    type Reply = XIQueryVersionReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIQueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl TryParse for XIQueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIQueryVersionReply { sequence, length, major_version, minor_version };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIQueryVersionReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            major_version_bytes[0],
            major_version_bytes[1],
            minor_version_bytes[0],
            minor_version_bytes[1],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.major_version.serialize_into(bytes);
        self.minor_version.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}
#[cfg(test)]
mod xi_query_version_reply {
    use super::XIQueryVersionReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIQueryVersionReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                major_version: GenRandom::generate(rng),
                minor_version: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7421350705950867456);
        let value = XIQueryVersionReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceClassType(u16);
impl DeviceClassType {
    pub const KEY: Self = Self(0);
    pub const BUTTON: Self = Self(1);
    pub const VALUATOR: Self = Self(2);
    pub const SCROLL: Self = Self(3);
    pub const TOUCH: Self = Self(8);
}
impl From<DeviceClassType> for u16 {
    #[inline]
    fn from(input: DeviceClassType) -> Self {
        input.0
    }
}
impl From<DeviceClassType> for Option<u16> {
    #[inline]
    fn from(input: DeviceClassType) -> Self {
        Some(input.0)
    }
}
impl From<DeviceClassType> for u32 {
    #[inline]
    fn from(input: DeviceClassType) -> Self {
        u32::from(input.0)
    }
}
impl From<DeviceClassType> for Option<u32> {
    #[inline]
    fn from(input: DeviceClassType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for DeviceClassType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for DeviceClassType {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for DeviceClassType  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::KEY.0.into(), "KEY", "Key"),
            (Self::BUTTON.0.into(), "BUTTON", "Button"),
            (Self::VALUATOR.0.into(), "VALUATOR", "Valuator"),
            (Self::SCROLL.0.into(), "SCROLL", "Scroll"),
            (Self::TOUCH.0.into(), "TOUCH", "Touch"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for DeviceClassType {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::KEY.0,
            Self::BUTTON.0,
            Self::VALUATOR.0,
            Self::SCROLL.0,
            Self::TOUCH.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceType(u16);
impl DeviceType {
    pub const MASTER_POINTER: Self = Self(1);
    pub const MASTER_KEYBOARD: Self = Self(2);
    pub const SLAVE_POINTER: Self = Self(3);
    pub const SLAVE_KEYBOARD: Self = Self(4);
    pub const FLOATING_SLAVE: Self = Self(5);
}
impl From<DeviceType> for u16 {
    #[inline]
    fn from(input: DeviceType) -> Self {
        input.0
    }
}
impl From<DeviceType> for Option<u16> {
    #[inline]
    fn from(input: DeviceType) -> Self {
        Some(input.0)
    }
}
impl From<DeviceType> for u32 {
    #[inline]
    fn from(input: DeviceType) -> Self {
        u32::from(input.0)
    }
}
impl From<DeviceType> for Option<u32> {
    #[inline]
    fn from(input: DeviceType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for DeviceType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for DeviceType {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for DeviceType  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::MASTER_POINTER.0.into(), "MASTER_POINTER", "MasterPointer"),
            (Self::MASTER_KEYBOARD.0.into(), "MASTER_KEYBOARD", "MasterKeyboard"),
            (Self::SLAVE_POINTER.0.into(), "SLAVE_POINTER", "SlavePointer"),
            (Self::SLAVE_KEYBOARD.0.into(), "SLAVE_KEYBOARD", "SlaveKeyboard"),
            (Self::FLOATING_SLAVE.0.into(), "FLOATING_SLAVE", "FloatingSlave"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for DeviceType {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::MASTER_POINTER.0,
            Self::MASTER_KEYBOARD.0,
            Self::SLAVE_POINTER.0,
            Self::SLAVE_KEYBOARD.0,
            Self::FLOATING_SLAVE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScrollFlags(u8);
impl ScrollFlags {
    pub const NO_EMULATION: Self = Self(1 << 0);
    pub const PREFERRED: Self = Self(1 << 1);
}
impl From<ScrollFlags> for u8 {
    #[inline]
    fn from(input: ScrollFlags) -> Self {
        input.0
    }
}
impl From<ScrollFlags> for Option<u8> {
    #[inline]
    fn from(input: ScrollFlags) -> Self {
        Some(input.0)
    }
}
impl From<ScrollFlags> for u16 {
    #[inline]
    fn from(input: ScrollFlags) -> Self {
        u16::from(input.0)
    }
}
impl From<ScrollFlags> for Option<u16> {
    #[inline]
    fn from(input: ScrollFlags) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ScrollFlags> for u32 {
    #[inline]
    fn from(input: ScrollFlags) -> Self {
        u32::from(input.0)
    }
}
impl From<ScrollFlags> for Option<u32> {
    #[inline]
    fn from(input: ScrollFlags) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ScrollFlags {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ScrollFlags  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::NO_EMULATION.0.into(), "NO_EMULATION", "NoEmulation"),
            (Self::PREFERRED.0.into(), "PREFERRED", "Preferred"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(ScrollFlags, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for ScrollFlags {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::NO_EMULATION.0,
            Self::PREFERRED.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScrollType(u16);
impl ScrollType {
    pub const VERTICAL: Self = Self(1);
    pub const HORIZONTAL: Self = Self(2);
}
impl From<ScrollType> for u16 {
    #[inline]
    fn from(input: ScrollType) -> Self {
        input.0
    }
}
impl From<ScrollType> for Option<u16> {
    #[inline]
    fn from(input: ScrollType) -> Self {
        Some(input.0)
    }
}
impl From<ScrollType> for u32 {
    #[inline]
    fn from(input: ScrollType) -> Self {
        u32::from(input.0)
    }
}
impl From<ScrollType> for Option<u32> {
    #[inline]
    fn from(input: ScrollType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ScrollType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for ScrollType {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ScrollType  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::VERTICAL.0.into(), "VERTICAL", "Vertical"),
            (Self::HORIZONTAL.0.into(), "HORIZONTAL", "Horizontal"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for ScrollType {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::VERTICAL.0,
            Self::HORIZONTAL.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TouchMode(u8);
impl TouchMode {
    pub const DIRECT: Self = Self(1);
    pub const DEPENDENT: Self = Self(2);
}
impl From<TouchMode> for u8 {
    #[inline]
    fn from(input: TouchMode) -> Self {
        input.0
    }
}
impl From<TouchMode> for Option<u8> {
    #[inline]
    fn from(input: TouchMode) -> Self {
        Some(input.0)
    }
}
impl From<TouchMode> for u16 {
    #[inline]
    fn from(input: TouchMode) -> Self {
        u16::from(input.0)
    }
}
impl From<TouchMode> for Option<u16> {
    #[inline]
    fn from(input: TouchMode) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<TouchMode> for u32 {
    #[inline]
    fn from(input: TouchMode) -> Self {
        u32::from(input.0)
    }
}
impl From<TouchMode> for Option<u32> {
    #[inline]
    fn from(input: TouchMode) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for TouchMode {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for TouchMode  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::DIRECT.0.into(), "DIRECT", "Direct"),
            (Self::DEPENDENT.0.into(), "DEPENDENT", "Dependent"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for TouchMode {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::DIRECT.0,
            Self::DEPENDENT.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ButtonClass {
    pub type_: DeviceClassType,
    pub len: u16,
    pub sourceid: DeviceId,
    pub state: Vec<u32>,
    pub labels: Vec<xproto::Atom>,
}
impl TryParse for ButtonClass {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u16::try_parse(remaining)?;
        let (len, remaining) = u16::try_parse(remaining)?;
        let (sourceid, remaining) = DeviceId::try_parse(remaining)?;
        let (num_buttons, remaining) = u16::try_parse(remaining)?;
        let (state, remaining) = crate::x11_utils::parse_list::<u32>(remaining, u32::from(num_buttons).checked_add(31u32).ok_or(ParseError::InvalidExpression)?.checked_div(32u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let (labels, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_buttons.try_to_usize()?)?;
        let type_ = type_.into();
        let result = ButtonClass { type_, len, sourceid, state, labels };
        Ok((result, remaining))
    }
}
impl Serialize for ButtonClass {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        let num_buttons = u16::try_from(self.labels.len()).expect("`labels` has too many elements");
        num_buttons.serialize_into(bytes);
        assert_eq!(self.state.len(), usize::try_from(u32::from(num_buttons).checked_add(31u32).unwrap().checked_div(32u32).unwrap()).unwrap(), "`state` has an incorrect length");
        self.state.serialize_into(bytes);
        self.labels.serialize_into(bytes);
    }
}
impl ButtonClass {
    /// Get the value of the `num_buttons` field.
    ///
    /// The `num_buttons` field is used as the length field of the `labels` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_buttons(&self) -> u16 {
        self.labels.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod button_class {
    use super::ButtonClass;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ButtonClass {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                state: GenRandom::generate(rng),
                labels: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7780349420175092992);
        let value = ButtonClass::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let (keys, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_keys.try_to_usize()?)?;
        let type_ = type_.into();
        let result = KeyClass { type_, len, sourceid, keys };
        Ok((result, remaining))
    }
}
impl Serialize for KeyClass {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        let num_keys = u16::try_from(self.keys.len()).expect("`keys` has too many elements");
        num_keys.serialize_into(bytes);
        self.keys.serialize_into(bytes);
    }
}
impl KeyClass {
    /// Get the value of the `num_keys` field.
    ///
    /// The `num_keys` field is used as the length field of the `keys` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_keys(&self) -> u16 {
        self.keys.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod key_class {
    use super::KeyClass;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for KeyClass {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                keys: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8508127327177500);
        let value = KeyClass::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (increment, remaining) = Fp3232::try_parse(remaining)?;
        let type_ = type_.into();
        let scroll_type = scroll_type.into();
        let result = ScrollClass { type_, len, sourceid, number, scroll_type, flags, increment };
        Ok((result, remaining))
    }
}
impl Serialize for ScrollClass {
    type Bytes = [u8; 24];
    fn serialize(&self) -> [u8; 24] {
        let type_bytes = u16::from(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let sourceid_bytes = self.sourceid.serialize();
        let number_bytes = self.number.serialize();
        let scroll_type_bytes = u16::from(self.scroll_type).serialize();
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
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        self.number.serialize_into(bytes);
        u16::from(self.scroll_type).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
        self.increment.serialize_into(bytes);
    }
}
#[cfg(test)]
mod scroll_class {
    use super::ScrollClass;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ScrollClass {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                number: GenRandom::generate(rng),
                scroll_type: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                increment: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5313910774664992640);
        let value = ScrollClass::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let type_ = type_.into();
        let mode = mode.into();
        let result = TouchClass { type_, len, sourceid, mode, num_touches };
        Ok((result, remaining))
    }
}
impl Serialize for TouchClass {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u16::from(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let sourceid_bytes = self.sourceid.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
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
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        u8::from(self.mode).serialize_into(bytes);
        self.num_touches.serialize_into(bytes);
    }
}
#[cfg(test)]
mod touch_class {
    use super::TouchClass;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for TouchClass {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                mode: GenRandom::generate(rng),
                num_touches: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12027459104568507520);
        let value = TouchClass::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let mode = mode.into();
        let result = ValuatorClass { type_, len, sourceid, number, label, min, max, value, resolution, mode };
        Ok((result, remaining))
    }
}
impl Serialize for ValuatorClass {
    type Bytes = [u8; 44];
    fn serialize(&self) -> [u8; 44] {
        let type_bytes = u16::from(self.type_).serialize();
        let len_bytes = self.len.serialize();
        let sourceid_bytes = self.sourceid.serialize();
        let number_bytes = self.number.serialize();
        let label_bytes = self.label.serialize();
        let min_bytes = self.min.serialize();
        let max_bytes = self.max.serialize();
        let value_bytes = self.value.serialize();
        let resolution_bytes = self.resolution.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
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
        u16::from(self.type_).serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        self.number.serialize_into(bytes);
        self.label.serialize_into(bytes);
        self.min.serialize_into(bytes);
        self.max.serialize_into(bytes);
        self.value.serialize_into(bytes);
        self.resolution.serialize_into(bytes);
        u8::from(self.mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}
#[cfg(test)]
mod valuator_class {
    use super::ValuatorClass;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ValuatorClass {
        fn generate(rng: &Rng) -> Self {
            Self {
                type_: GenRandom::generate(rng),
                len: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                number: GenRandom::generate(rng),
                label: GenRandom::generate(rng),
                min: GenRandom::generate(rng),
                max: GenRandom::generate(rng),
                value: GenRandom::generate(rng),
                resolution: GenRandom::generate(rng),
                mode: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1142443798316963584);
        let value = ValuatorClass::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceClassDataKey {
    pub keys: Vec<u32>,
}
impl TryParse for DeviceClassDataKey {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_keys, remaining) = u16::try_parse(remaining)?;
        let (keys, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_keys.try_to_usize()?)?;
        let result = DeviceClassDataKey { keys };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceClassDataKey {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        let num_keys = u16::try_from(self.keys.len()).expect("`keys` has too many elements");
        num_keys.serialize_into(bytes);
        self.keys.serialize_into(bytes);
    }
}
impl DeviceClassDataKey {
    /// Get the value of the `num_keys` field.
    ///
    /// The `num_keys` field is used as the length field of the `keys` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_keys(&self) -> u16 {
        self.keys.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod device_class_data_key {
    use super::DeviceClassDataKey;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceClassDataKey {
        fn generate(rng: &Rng) -> Self {
            Self {
                keys: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2224012988422829568);
        let value = DeviceClassDataKey::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceClassDataButton {
    pub state: Vec<u32>,
    pub labels: Vec<xproto::Atom>,
}
impl TryParse for DeviceClassDataButton {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_buttons, remaining) = u16::try_parse(remaining)?;
        let (state, remaining) = crate::x11_utils::parse_list::<u32>(remaining, u32::from(num_buttons).checked_add(31u32).ok_or(ParseError::InvalidExpression)?.checked_div(32u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let (labels, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_buttons.try_to_usize()?)?;
        let result = DeviceClassDataButton { state, labels };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceClassDataButton {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        let num_buttons = u16::try_from(self.labels.len()).expect("`labels` has too many elements");
        num_buttons.serialize_into(bytes);
        assert_eq!(self.state.len(), usize::try_from(u32::from(num_buttons).checked_add(31u32).unwrap().checked_div(32u32).unwrap()).unwrap(), "`state` has an incorrect length");
        self.state.serialize_into(bytes);
        self.labels.serialize_into(bytes);
    }
}
impl DeviceClassDataButton {
    /// Get the value of the `num_buttons` field.
    ///
    /// The `num_buttons` field is used as the length field of the `labels` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_buttons(&self) -> u16 {
        self.labels.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod device_class_data_button {
    use super::DeviceClassDataButton;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceClassDataButton {
        fn generate(rng: &Rng) -> Self {
            Self {
                state: GenRandom::generate(rng),
                labels: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6702323549547560960);
        let value = DeviceClassDataButton::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let mode = mode.into();
        let result = DeviceClassDataValuator { number, label, min, max, value, resolution, mode };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceClassDataValuator {
    type Bytes = [u8; 38];
    fn serialize(&self) -> [u8; 38] {
        let number_bytes = self.number.serialize();
        let label_bytes = self.label.serialize();
        let min_bytes = self.min.serialize();
        let max_bytes = self.max.serialize();
        let value_bytes = self.value.serialize();
        let resolution_bytes = self.resolution.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
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
        u8::from(self.mode).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}
#[cfg(test)]
mod device_class_data_valuator {
    use super::DeviceClassDataValuator;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceClassDataValuator {
        fn generate(rng: &Rng) -> Self {
            Self {
                number: GenRandom::generate(rng),
                label: GenRandom::generate(rng),
                min: GenRandom::generate(rng),
                max: GenRandom::generate(rng),
                value: GenRandom::generate(rng),
                resolution: GenRandom::generate(rng),
                mode: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5230139036893806592);
        let value = DeviceClassDataValuator::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (increment, remaining) = Fp3232::try_parse(remaining)?;
        let scroll_type = scroll_type.into();
        let result = DeviceClassDataScroll { number, scroll_type, flags, increment };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceClassDataScroll {
    type Bytes = [u8; 18];
    fn serialize(&self) -> [u8; 18] {
        let number_bytes = self.number.serialize();
        let scroll_type_bytes = u16::from(self.scroll_type).serialize();
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
        u16::from(self.scroll_type).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
        self.increment.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_class_data_scroll {
    use super::DeviceClassDataScroll;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceClassDataScroll {
        fn generate(rng: &Rng) -> Self {
            Self {
                number: GenRandom::generate(rng),
                scroll_type: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                increment: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17180849776415522816);
        let value = DeviceClassDataScroll::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceClassDataTouch {
    pub mode: TouchMode,
    pub num_touches: u8,
}
impl TryParse for DeviceClassDataTouch {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (num_touches, remaining) = u8::try_parse(remaining)?;
        let mode = mode.into();
        let result = DeviceClassDataTouch { mode, num_touches };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceClassDataTouch {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let mode_bytes = u8::from(self.mode).serialize();
        let num_touches_bytes = self.num_touches.serialize();
        [
            mode_bytes[0],
            num_touches_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        u8::from(self.mode).serialize_into(bytes);
        self.num_touches.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_class_data_touch {
    use super::DeviceClassDataTouch;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceClassDataTouch {
        fn generate(rng: &Rng) -> Self {
            Self {
                mode: GenRandom::generate(rng),
                num_touches: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1907799378990022656);
        let value = DeviceClassDataTouch::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceClassData {
    Key(DeviceClassDataKey),
    Button(DeviceClassDataButton),
    Valuator(DeviceClassDataValuator),
    Scroll(DeviceClassDataScroll),
    Touch(DeviceClassDataTouch),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u16),
}
impl DeviceClassData {
    fn try_parse(value: &[u8], type_: u16) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = type_;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u16::from(DeviceClassType::KEY) {
            let (key, new_remaining) = DeviceClassDataKey::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Key(key));
        }
        if switch_expr == u16::from(DeviceClassType::BUTTON) {
            let (button, new_remaining) = DeviceClassDataButton::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Button(button));
        }
        if switch_expr == u16::from(DeviceClassType::VALUATOR) {
            let (valuator, new_remaining) = DeviceClassDataValuator::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Valuator(valuator));
        }
        if switch_expr == u16::from(DeviceClassType::SCROLL) {
            let (scroll, new_remaining) = DeviceClassDataScroll::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Scroll(scroll));
        }
        if switch_expr == u16::from(DeviceClassType::TOUCH) {
            let (touch, new_remaining) = DeviceClassDataTouch::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(DeviceClassData::Touch(touch));
        }
        match parse_result {
            None => Ok((DeviceClassData::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl DeviceClassData {
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
impl DeviceClassData {
    #[allow(dead_code)]
    fn serialize(&self, type_: u16) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, type_);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, type_: u16) {
        let _ = type_;
        match self {
            DeviceClassData::Key(key) => key.serialize_into(bytes),
            DeviceClassData::Button(button) => button.serialize_into(bytes),
            DeviceClassData::Valuator(valuator) => valuator.serialize_into(bytes),
            DeviceClassData::Scroll(scroll) => scroll.serialize_into(bytes),
            DeviceClassData::Touch(touch) => touch.serialize_into(bytes),
            DeviceClassData::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl DeviceClassData {
    fn switch_expr(&self) -> u16 {
        match self {
            DeviceClassData::Key(_) => u16::from(DeviceClassType::KEY),
            DeviceClassData::Button(_) => u16::from(DeviceClassType::BUTTON),
            DeviceClassData::Valuator(_) => u16::from(DeviceClassType::VALUATOR),
            DeviceClassData::Scroll(_) => u16::from(DeviceClassType::SCROLL),
            DeviceClassData::Touch(_) => u16::from(DeviceClassType::TOUCH),
            DeviceClassData::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod device_class_data {
    use super::DeviceClassData;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceClassData {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..5) {
                0 => Self::Key(GenRandom::generate(rng)),
                1 => Self::Button(GenRandom::generate(rng)),
                2 => Self::Valuator(GenRandom::generate(rng)),
                3 => Self::Scroll(GenRandom::generate(rng)),
                _ => Self::Touch(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<DeviceClassData> {
        alloc::vec![
            DeviceClassData::Key(GenRandom::generate(rng)),
            DeviceClassData::Button(GenRandom::generate(rng)),
            DeviceClassData::Valuator(GenRandom::generate(rng)),
            DeviceClassData::Scroll(GenRandom::generate(rng)),
            DeviceClassData::Touch(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceClass {
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
        let result = DeviceClass { len, sourceid, data };
        Ok((result, remaining))
    }
}
impl Serialize for DeviceClass {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(6);
        let type_: u16 = self.data.switch_expr();
        type_.serialize_into(bytes);
        self.len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        self.data.serialize_into(bytes, type_);
    }
}
#[cfg(test)]
mod device_class {
    use super::DeviceClass;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceClass {
        fn generate(rng: &Rng) -> Self {
            Self {
                len: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                data: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2920929234700824352);
        let value = DeviceClass::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<DeviceClass>(remaining, num_classes.try_to_usize()?)?;
        let type_ = type_.into();
        let result = XIDeviceInfo { deviceid, type_, attachment, enabled, name, classes };
        Ok((result, remaining))
    }
}
impl Serialize for XIDeviceInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.deviceid.serialize_into(bytes);
        u16::from(self.type_).serialize_into(bytes);
        self.attachment.serialize_into(bytes);
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        num_classes.serialize_into(bytes);
        let name_len = u16::try_from(self.name.len()).expect("`name` has too many elements");
        name_len.serialize_into(bytes);
        self.enabled.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.classes.serialize_into(bytes);
    }
}
impl XIDeviceInfo {
    /// Get the value of the `num_classes` field.
    ///
    /// The `num_classes` field is used as the length field of the `classes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_classes(&self) -> u16 {
        self.classes.len()
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
#[cfg(test)]
mod xi_device_info {
    use super::XIDeviceInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIDeviceInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                deviceid: GenRandom::generate(rng),
                type_: GenRandom::generate(rng),
                attachment: GenRandom::generate(rng),
                enabled: GenRandom::generate(rng),
                name: GenRandom::generate(rng),
                classes: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1369158972028139776);
        let value = XIDeviceInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIQueryDevice request
pub const XI_QUERY_DEVICE_REQUEST: u8 = 48;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIQueryDeviceRequest {
    pub deviceid: DeviceId,
}
impl XIQueryDeviceRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_QUERY_DEVICE_REQUEST,
            0,
            0,
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_QUERY_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (deviceid, remaining) = DeviceId::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XIQueryDeviceRequest {
            deviceid,
        })
    }
}
impl Request for XIQueryDeviceRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for XIQueryDeviceRequest {
    type Reply = XIQueryDeviceReply;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIQueryDeviceReply {
    pub sequence: u16,
    pub length: u32,
    pub infos: Vec<XIDeviceInfo>,
}
impl TryParse for XIQueryDeviceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_infos, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (infos, remaining) = crate::x11_utils::parse_list::<XIDeviceInfo>(remaining, num_infos.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIQueryDeviceReply { sequence, length, infos };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIQueryDeviceReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_infos = u16::try_from(self.infos.len()).expect("`infos` has too many elements");
        num_infos.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 22]);
        self.infos.serialize_into(bytes);
    }
}
impl XIQueryDeviceReply {
    /// Get the value of the `num_infos` field.
    ///
    /// The `num_infos` field is used as the length field of the `infos` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_infos(&self) -> u16 {
        self.infos.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod xi_query_device_reply {
    use super::XIQueryDeviceReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIQueryDeviceReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                infos: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8468985576548384768);
        let value = XIQueryDeviceReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XISetFocus request
pub const XI_SET_FOCUS_REQUEST: u8 = 49;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XISetFocusRequest {
    pub window: xproto::Window,
    pub time: xproto::Timestamp,
    pub deviceid: DeviceId,
}
impl XISetFocusRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let time_bytes = self.time.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_SET_FOCUS_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_SET_FOCUS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XISetFocusRequest {
            window,
            time,
            deviceid,
        })
    }
}
impl Request for XISetFocusRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for XISetFocusRequest {
}

/// Opcode for the XIGetFocus request
pub const XI_GET_FOCUS_REQUEST: u8 = 50;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGetFocusRequest {
    pub deviceid: DeviceId,
}
impl XIGetFocusRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_GET_FOCUS_REQUEST,
            0,
            0,
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_GET_FOCUS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (deviceid, remaining) = DeviceId::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XIGetFocusRequest {
            deviceid,
        })
    }
}
impl Request for XIGetFocusRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for XIGetFocusRequest {
    type Reply = XIGetFocusReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGetFocusReply {
    pub sequence: u16,
    pub length: u32,
    pub focus: xproto::Window,
}
impl TryParse for XIGetFocusReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (focus, remaining) = xproto::Window::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIGetFocusReply { sequence, length, focus };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIGetFocusReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let focus_bytes = self.focus.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            focus_bytes[0],
            focus_bytes[1],
            focus_bytes[2],
            focus_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.focus.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}
#[cfg(test)]
mod xi_get_focus_reply {
    use super::XIGetFocusReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIGetFocusReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                focus: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2029643296113401856);
        let value = XIGetFocusReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrabOwner(bool);
impl GrabOwner {
    pub const NO_OWNER: Self = Self(false);
    pub const OWNER: Self = Self(true);
}
impl From<GrabOwner> for bool {
    #[inline]
    fn from(input: GrabOwner) -> Self {
        input.0
    }
}
impl From<GrabOwner> for Option<bool> {
    #[inline]
    fn from(input: GrabOwner) -> Self {
        Some(input.0)
    }
}
impl From<GrabOwner> for u8 {
    #[inline]
    fn from(input: GrabOwner) -> Self {
        u8::from(input.0)
    }
}
impl From<GrabOwner> for Option<u8> {
    #[inline]
    fn from(input: GrabOwner) -> Self {
        Some(u8::from(input.0))
    }
}
impl From<GrabOwner> for u16 {
    #[inline]
    fn from(input: GrabOwner) -> Self {
        u16::from(input.0)
    }
}
impl From<GrabOwner> for Option<u16> {
    #[inline]
    fn from(input: GrabOwner) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<GrabOwner> for u32 {
    #[inline]
    fn from(input: GrabOwner) -> Self {
        u32::from(input.0)
    }
}
impl From<GrabOwner> for Option<u32> {
    #[inline]
    fn from(input: GrabOwner) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<bool> for GrabOwner {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for GrabOwner  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::NO_OWNER.0.into(), "NO_OWNER", "NoOwner"),
            (Self::OWNER.0.into(), "OWNER", "Owner"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for GrabOwner {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::NO_OWNER.0,
            Self::OWNER.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the XIGrabDevice request
pub const XI_GRAB_DEVICE_REQUEST: u8 = 51;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGrabDeviceRequest<'input> {
    pub window: xproto::Window,
    pub time: xproto::Timestamp,
    pub cursor: xproto::Cursor,
    pub deviceid: DeviceId,
    pub mode: xproto::GrabMode,
    pub paired_device_mode: xproto::GrabMode,
    pub owner_events: GrabOwner,
    pub mask: Cow<'input, [u32]>,
}
impl<'input> XIGrabDeviceRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let time_bytes = self.time.serialize();
        let cursor_bytes = self.cursor.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
        let paired_device_mode_bytes = u8::from(self.paired_device_mode).serialize();
        let owner_events_bytes = bool::from(self.owner_events).serialize();
        let mask_len = u16::try_from(self.mask.len()).expect("`mask` has too many elements");
        let mask_len_bytes = mask_len.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_GRAB_DEVICE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let mask_bytes = self.mask.serialize();
        let length_so_far = length_so_far + mask_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), mask_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_GRAB_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (cursor, remaining) = xproto::Cursor::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let mode = mode.into();
        let (paired_device_mode, remaining) = u8::try_parse(remaining)?;
        let paired_device_mode = paired_device_mode.into();
        let (owner_events, remaining) = bool::try_parse(remaining)?;
        let owner_events = owner_events.into();
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (mask_len, remaining) = u16::try_parse(remaining)?;
        let (mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, mask_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(XIGrabDeviceRequest {
            window,
            time,
            cursor,
            deviceid,
            mode,
            paired_device_mode,
            owner_events,
            mask: Cow::Owned(mask),
        })
    }
    /// Clone all borrowed data in this XIGrabDeviceRequest.
    pub fn into_owned(self) -> XIGrabDeviceRequest<'static> {
        XIGrabDeviceRequest {
            window: self.window,
            time: self.time,
            cursor: self.cursor,
            deviceid: self.deviceid,
            mode: self.mode,
            paired_device_mode: self.paired_device_mode,
            owner_events: self.owner_events,
            mask: Cow::Owned(self.mask.into_owned()),
        }
    }
}
impl<'input> Request for XIGrabDeviceRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for XIGrabDeviceRequest<'input> {
    type Reply = XIGrabDeviceReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGrabDeviceReply {
    pub sequence: u16,
    pub length: u32,
    pub status: xproto::GrabStatus,
}
impl TryParse for XIGrabDeviceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let status = status.into();
        let result = XIGrabDeviceReply { sequence, length, status };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIGrabDeviceReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let status_bytes = u8::from(self.status).serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
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
            0,
            0,
            0,
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
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod xi_grab_device_reply {
    use super::XIGrabDeviceReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIGrabDeviceReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11816004202391830528);
        let value = XIGrabDeviceReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIUngrabDevice request
pub const XI_UNGRAB_DEVICE_REQUEST: u8 = 52;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIUngrabDeviceRequest {
    pub time: xproto::Timestamp,
    pub deviceid: DeviceId,
}
impl XIUngrabDeviceRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let time_bytes = self.time.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_UNGRAB_DEVICE_REQUEST,
            0,
            0,
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_UNGRAB_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (time, remaining) = xproto::Timestamp::try_parse(value)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XIUngrabDeviceRequest {
            time,
            deviceid,
        })
    }
}
impl Request for XIUngrabDeviceRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for XIUngrabDeviceRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventMode(u8);
impl EventMode {
    pub const ASYNC_DEVICE: Self = Self(0);
    pub const SYNC_DEVICE: Self = Self(1);
    pub const REPLAY_DEVICE: Self = Self(2);
    pub const ASYNC_PAIRED_DEVICE: Self = Self(3);
    pub const ASYNC_PAIR: Self = Self(4);
    pub const SYNC_PAIR: Self = Self(5);
    pub const ACCEPT_TOUCH: Self = Self(6);
    pub const REJECT_TOUCH: Self = Self(7);
}
impl From<EventMode> for u8 {
    #[inline]
    fn from(input: EventMode) -> Self {
        input.0
    }
}
impl From<EventMode> for Option<u8> {
    #[inline]
    fn from(input: EventMode) -> Self {
        Some(input.0)
    }
}
impl From<EventMode> for u16 {
    #[inline]
    fn from(input: EventMode) -> Self {
        u16::from(input.0)
    }
}
impl From<EventMode> for Option<u16> {
    #[inline]
    fn from(input: EventMode) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<EventMode> for u32 {
    #[inline]
    fn from(input: EventMode) -> Self {
        u32::from(input.0)
    }
}
impl From<EventMode> for Option<u32> {
    #[inline]
    fn from(input: EventMode) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for EventMode {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for EventMode  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ASYNC_DEVICE.0.into(), "ASYNC_DEVICE", "AsyncDevice"),
            (Self::SYNC_DEVICE.0.into(), "SYNC_DEVICE", "SyncDevice"),
            (Self::REPLAY_DEVICE.0.into(), "REPLAY_DEVICE", "ReplayDevice"),
            (Self::ASYNC_PAIRED_DEVICE.0.into(), "ASYNC_PAIRED_DEVICE", "AsyncPairedDevice"),
            (Self::ASYNC_PAIR.0.into(), "ASYNC_PAIR", "AsyncPair"),
            (Self::SYNC_PAIR.0.into(), "SYNC_PAIR", "SyncPair"),
            (Self::ACCEPT_TOUCH.0.into(), "ACCEPT_TOUCH", "AcceptTouch"),
            (Self::REJECT_TOUCH.0.into(), "REJECT_TOUCH", "RejectTouch"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for EventMode {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ASYNC_DEVICE.0,
            Self::SYNC_DEVICE.0,
            Self::REPLAY_DEVICE.0,
            Self::ASYNC_PAIRED_DEVICE.0,
            Self::ASYNC_PAIR.0,
            Self::SYNC_PAIR.0,
            Self::ACCEPT_TOUCH.0,
            Self::REJECT_TOUCH.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the XIAllowEvents request
pub const XI_ALLOW_EVENTS_REQUEST: u8 = 53;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIAllowEventsRequest {
    pub time: xproto::Timestamp,
    pub deviceid: DeviceId,
    pub event_mode: EventMode,
    pub touchid: u32,
    pub grab_window: xproto::Window,
}
impl XIAllowEventsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let time_bytes = self.time.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let event_mode_bytes = u8::from(self.event_mode).serialize();
        let touchid_bytes = self.touchid.serialize();
        let grab_window_bytes = self.grab_window.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_ALLOW_EVENTS_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_ALLOW_EVENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (time, remaining) = xproto::Timestamp::try_parse(value)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (event_mode, remaining) = u8::try_parse(remaining)?;
        let event_mode = event_mode.into();
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (touchid, remaining) = u32::try_parse(remaining)?;
        let (grab_window, remaining) = xproto::Window::try_parse(remaining)?;
        let _ = remaining;
        Ok(XIAllowEventsRequest {
            time,
            deviceid,
            event_mode,
            touchid,
            grab_window,
        })
    }
}
impl Request for XIAllowEventsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for XIAllowEventsRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrabMode22(u8);
impl GrabMode22 {
    pub const SYNC: Self = Self(0);
    pub const ASYNC: Self = Self(1);
    pub const TOUCH: Self = Self(2);
}
impl From<GrabMode22> for u8 {
    #[inline]
    fn from(input: GrabMode22) -> Self {
        input.0
    }
}
impl From<GrabMode22> for Option<u8> {
    #[inline]
    fn from(input: GrabMode22) -> Self {
        Some(input.0)
    }
}
impl From<GrabMode22> for u16 {
    #[inline]
    fn from(input: GrabMode22) -> Self {
        u16::from(input.0)
    }
}
impl From<GrabMode22> for Option<u16> {
    #[inline]
    fn from(input: GrabMode22) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<GrabMode22> for u32 {
    #[inline]
    fn from(input: GrabMode22) -> Self {
        u32::from(input.0)
    }
}
impl From<GrabMode22> for Option<u32> {
    #[inline]
    fn from(input: GrabMode22) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for GrabMode22 {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for GrabMode22  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::SYNC.0.into(), "SYNC", "Sync"),
            (Self::ASYNC.0.into(), "ASYNC", "Async"),
            (Self::TOUCH.0.into(), "TOUCH", "Touch"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for GrabMode22 {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::SYNC.0,
            Self::ASYNC.0,
            Self::TOUCH.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrabType(u8);
impl GrabType {
    pub const BUTTON: Self = Self(0);
    pub const KEYCODE: Self = Self(1);
    pub const ENTER: Self = Self(2);
    pub const FOCUS_IN: Self = Self(3);
    pub const TOUCH_BEGIN: Self = Self(4);
}
impl From<GrabType> for u8 {
    #[inline]
    fn from(input: GrabType) -> Self {
        input.0
    }
}
impl From<GrabType> for Option<u8> {
    #[inline]
    fn from(input: GrabType) -> Self {
        Some(input.0)
    }
}
impl From<GrabType> for u16 {
    #[inline]
    fn from(input: GrabType) -> Self {
        u16::from(input.0)
    }
}
impl From<GrabType> for Option<u16> {
    #[inline]
    fn from(input: GrabType) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<GrabType> for u32 {
    #[inline]
    fn from(input: GrabType) -> Self {
        u32::from(input.0)
    }
}
impl From<GrabType> for Option<u32> {
    #[inline]
    fn from(input: GrabType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for GrabType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for GrabType  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::BUTTON.0.into(), "BUTTON", "Button"),
            (Self::KEYCODE.0.into(), "KEYCODE", "Keycode"),
            (Self::ENTER.0.into(), "ENTER", "Enter"),
            (Self::FOCUS_IN.0.into(), "FOCUS_IN", "FocusIn"),
            (Self::TOUCH_BEGIN.0.into(), "TOUCH_BEGIN", "TouchBegin"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for GrabType {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::BUTTON.0,
            Self::KEYCODE.0,
            Self::ENTER.0,
            Self::FOCUS_IN.0,
            Self::TOUCH_BEGIN.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifierMask(u32);
impl ModifierMask {
    pub const ANY: Self = Self(1 << 31);
}
impl From<ModifierMask> for u32 {
    #[inline]
    fn from(input: ModifierMask) -> Self {
        input.0
    }
}
impl From<ModifierMask> for Option<u32> {
    #[inline]
    fn from(input: ModifierMask) -> Self {
        Some(input.0)
    }
}
impl From<u8> for ModifierMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for ModifierMask {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for ModifierMask {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ModifierMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ANY.0, "ANY", "Any"),
        ];
        pretty_print_bitmask(fmt, self.0, &variants)
    }
}
bitmask_binop!(ModifierMask, u32);
#[cfg(test)]
impl crate::x11_utils::GenRandom for ModifierMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ANY.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrabModifierInfo {
    pub modifiers: u32,
    pub status: xproto::GrabStatus,
}
impl TryParse for GrabModifierInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (modifiers, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let status = status.into();
        let result = GrabModifierInfo { modifiers, status };
        Ok((result, remaining))
    }
}
impl Serialize for GrabModifierInfo {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let modifiers_bytes = self.modifiers.serialize();
        let status_bytes = u8::from(self.status).serialize();
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
        u8::from(self.status).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}
#[cfg(test)]
mod grab_modifier_info {
    use super::GrabModifierInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for GrabModifierInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                modifiers: GenRandom::generate(rng),
                status: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13050813650016786688);
        let value = GrabModifierInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIPassiveGrabDevice request
pub const XI_PASSIVE_GRAB_DEVICE_REQUEST: u8 = 54;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIPassiveGrabDeviceRequest<'input> {
    pub time: xproto::Timestamp,
    pub grab_window: xproto::Window,
    pub cursor: xproto::Cursor,
    pub detail: u32,
    pub deviceid: DeviceId,
    pub grab_type: GrabType,
    pub grab_mode: GrabMode22,
    pub paired_device_mode: xproto::GrabMode,
    pub owner_events: GrabOwner,
    pub mask: Cow<'input, [u32]>,
    pub modifiers: Cow<'input, [u32]>,
}
impl<'input> XIPassiveGrabDeviceRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let time_bytes = self.time.serialize();
        let grab_window_bytes = self.grab_window.serialize();
        let cursor_bytes = self.cursor.serialize();
        let detail_bytes = self.detail.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let num_modifiers = u16::try_from(self.modifiers.len()).expect("`modifiers` has too many elements");
        let num_modifiers_bytes = num_modifiers.serialize();
        let mask_len = u16::try_from(self.mask.len()).expect("`mask` has too many elements");
        let mask_len_bytes = mask_len.serialize();
        let grab_type_bytes = u8::from(self.grab_type).serialize();
        let grab_mode_bytes = u8::from(self.grab_mode).serialize();
        let paired_device_mode_bytes = u8::from(self.paired_device_mode).serialize();
        let owner_events_bytes = bool::from(self.owner_events).serialize();
        let mut request0 = vec![
            major_opcode,
            XI_PASSIVE_GRAB_DEVICE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let mask_bytes = self.mask.serialize();
        let length_so_far = length_so_far + mask_bytes.len();
        let modifiers_bytes = self.modifiers.serialize();
        let length_so_far = length_so_far + modifiers_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), mask_bytes.into(), modifiers_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_PASSIVE_GRAB_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (time, remaining) = xproto::Timestamp::try_parse(value)?;
        let (grab_window, remaining) = xproto::Window::try_parse(remaining)?;
        let (cursor, remaining) = xproto::Cursor::try_parse(remaining)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (num_modifiers, remaining) = u16::try_parse(remaining)?;
        let (mask_len, remaining) = u16::try_parse(remaining)?;
        let (grab_type, remaining) = u8::try_parse(remaining)?;
        let grab_type = grab_type.into();
        let (grab_mode, remaining) = u8::try_parse(remaining)?;
        let grab_mode = grab_mode.into();
        let (paired_device_mode, remaining) = u8::try_parse(remaining)?;
        let paired_device_mode = paired_device_mode.into();
        let (owner_events, remaining) = bool::try_parse(remaining)?;
        let owner_events = owner_events.into();
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, mask_len.try_to_usize()?)?;
        let (modifiers, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_modifiers.try_to_usize()?)?;
        let _ = remaining;
        Ok(XIPassiveGrabDeviceRequest {
            time,
            grab_window,
            cursor,
            detail,
            deviceid,
            grab_type,
            grab_mode,
            paired_device_mode,
            owner_events,
            mask: Cow::Owned(mask),
            modifiers: Cow::Owned(modifiers),
        })
    }
    /// Clone all borrowed data in this XIPassiveGrabDeviceRequest.
    pub fn into_owned(self) -> XIPassiveGrabDeviceRequest<'static> {
        XIPassiveGrabDeviceRequest {
            time: self.time,
            grab_window: self.grab_window,
            cursor: self.cursor,
            detail: self.detail,
            deviceid: self.deviceid,
            grab_type: self.grab_type,
            grab_mode: self.grab_mode,
            paired_device_mode: self.paired_device_mode,
            owner_events: self.owner_events,
            mask: Cow::Owned(self.mask.into_owned()),
            modifiers: Cow::Owned(self.modifiers.into_owned()),
        }
    }
}
impl<'input> Request for XIPassiveGrabDeviceRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for XIPassiveGrabDeviceRequest<'input> {
    type Reply = XIPassiveGrabDeviceReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIPassiveGrabDeviceReply {
    pub sequence: u16,
    pub length: u32,
    pub modifiers: Vec<GrabModifierInfo>,
}
impl TryParse for XIPassiveGrabDeviceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_modifiers, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (modifiers, remaining) = crate::x11_utils::parse_list::<GrabModifierInfo>(remaining, num_modifiers.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIPassiveGrabDeviceReply { sequence, length, modifiers };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIPassiveGrabDeviceReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_modifiers = u16::try_from(self.modifiers.len()).expect("`modifiers` has too many elements");
        num_modifiers.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 22]);
        self.modifiers.serialize_into(bytes);
    }
}
impl XIPassiveGrabDeviceReply {
    /// Get the value of the `num_modifiers` field.
    ///
    /// The `num_modifiers` field is used as the length field of the `modifiers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_modifiers(&self) -> u16 {
        self.modifiers.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod xi_passive_grab_device_reply {
    use super::XIPassiveGrabDeviceReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIPassiveGrabDeviceReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                modifiers: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12173974932621885440);
        let value = XIPassiveGrabDeviceReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIPassiveUngrabDevice request
pub const XI_PASSIVE_UNGRAB_DEVICE_REQUEST: u8 = 55;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIPassiveUngrabDeviceRequest<'input> {
    pub grab_window: xproto::Window,
    pub detail: u32,
    pub deviceid: DeviceId,
    pub grab_type: GrabType,
    pub modifiers: Cow<'input, [u32]>,
}
impl<'input> XIPassiveUngrabDeviceRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let grab_window_bytes = self.grab_window.serialize();
        let detail_bytes = self.detail.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let num_modifiers = u16::try_from(self.modifiers.len()).expect("`modifiers` has too many elements");
        let num_modifiers_bytes = num_modifiers.serialize();
        let grab_type_bytes = u8::from(self.grab_type).serialize();
        let mut request0 = vec![
            major_opcode,
            XI_PASSIVE_UNGRAB_DEVICE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let modifiers_bytes = self.modifiers.serialize();
        let length_so_far = length_so_far + modifiers_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), modifiers_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_PASSIVE_UNGRAB_DEVICE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (grab_window, remaining) = xproto::Window::try_parse(value)?;
        let (detail, remaining) = u32::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (num_modifiers, remaining) = u16::try_parse(remaining)?;
        let (grab_type, remaining) = u8::try_parse(remaining)?;
        let grab_type = grab_type.into();
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (modifiers, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_modifiers.try_to_usize()?)?;
        let _ = remaining;
        Ok(XIPassiveUngrabDeviceRequest {
            grab_window,
            detail,
            deviceid,
            grab_type,
            modifiers: Cow::Owned(modifiers),
        })
    }
    /// Clone all borrowed data in this XIPassiveUngrabDeviceRequest.
    pub fn into_owned(self) -> XIPassiveUngrabDeviceRequest<'static> {
        XIPassiveUngrabDeviceRequest {
            grab_window: self.grab_window,
            detail: self.detail,
            deviceid: self.deviceid,
            grab_type: self.grab_type,
            modifiers: Cow::Owned(self.modifiers.into_owned()),
        }
    }
}
impl<'input> Request for XIPassiveUngrabDeviceRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for XIPassiveUngrabDeviceRequest<'input> {
}

/// Opcode for the XIListProperties request
pub const XI_LIST_PROPERTIES_REQUEST: u8 = 56;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIListPropertiesRequest {
    pub deviceid: DeviceId,
}
impl XIListPropertiesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let deviceid_bytes = self.deviceid.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_LIST_PROPERTIES_REQUEST,
            0,
            0,
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_LIST_PROPERTIES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (deviceid, remaining) = DeviceId::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(XIListPropertiesRequest {
            deviceid,
        })
    }
}
impl Request for XIListPropertiesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for XIListPropertiesRequest {
    type Reply = XIListPropertiesReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIListPropertiesReply {
    pub sequence: u16,
    pub length: u32,
    pub properties: Vec<xproto::Atom>,
}
impl TryParse for XIListPropertiesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_properties, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (properties, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, num_properties.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIListPropertiesReply { sequence, length, properties };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIListPropertiesReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_properties = u16::try_from(self.properties.len()).expect("`properties` has too many elements");
        num_properties.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 22]);
        self.properties.serialize_into(bytes);
    }
}
impl XIListPropertiesReply {
    /// Get the value of the `num_properties` field.
    ///
    /// The `num_properties` field is used as the length field of the `properties` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_properties(&self) -> u16 {
        self.properties.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod xi_list_properties_reply {
    use super::XIListPropertiesReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIListPropertiesReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                properties: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17411590907165147136);
        let value = XIListPropertiesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum XIChangePropertyAux {
    Data8(Vec<u8>),
    Data16(Vec<u16>),
    Data32(Vec<u32>),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u8),
}
impl XIChangePropertyAux {
    fn try_parse(value: &[u8], format: u8, num_items: u32) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = format;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u8::from(PropertyFormat::M8_BITS) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data8, remaining) = crate::x11_utils::parse_u8_list(remaining, num_items.try_to_usize()?)?;
            let data8 = data8.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIChangePropertyAux::Data8(data8));
        }
        if switch_expr == u8::from(PropertyFormat::M16_BITS) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data16, remaining) = crate::x11_utils::parse_list::<u16>(remaining, num_items.try_to_usize()?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIChangePropertyAux::Data16(data16));
        }
        if switch_expr == u8::from(PropertyFormat::M32_BITS) {
            let remaining = outer_remaining;
            let (data32, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_items.try_to_usize()?)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIChangePropertyAux::Data32(data32));
        }
        match parse_result {
            None => Ok((XIChangePropertyAux::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl XIChangePropertyAux {
    pub fn as_data8(&self) -> Option<&Vec<u8>> {
        match self {
            XIChangePropertyAux::Data8(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_data16(&self) -> Option<&Vec<u16>> {
        match self {
            XIChangePropertyAux::Data16(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_data32(&self) -> Option<&Vec<u32>> {
        match self {
            XIChangePropertyAux::Data32(value) => Some(value),
            _ => None,
        }
    }
}
impl XIChangePropertyAux {
    #[allow(dead_code)]
    fn serialize(&self, format: u8, num_items: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, format, num_items);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, format: u8, num_items: u32) {
        let _ = format;
        let _ = num_items;
        match self {
            XIChangePropertyAux::Data8(data8) => {
                assert_eq!(data8.len(), usize::try_from(num_items).unwrap(), "`data8` has an incorrect length");
                bytes.extend_from_slice(&data8);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            XIChangePropertyAux::Data16(data16) => {
                assert_eq!(data16.len(), usize::try_from(num_items).unwrap(), "`data16` has an incorrect length");
                data16.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            XIChangePropertyAux::Data32(data32) => {
                assert_eq!(data32.len(), usize::try_from(num_items).unwrap(), "`data32` has an incorrect length");
                data32.serialize_into(bytes);
            }
            XIChangePropertyAux::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl XIChangePropertyAux {
    fn switch_expr(&self) -> u8 {
        match self {
            XIChangePropertyAux::Data8(_) => u8::from(PropertyFormat::M8_BITS),
            XIChangePropertyAux::Data16(_) => u8::from(PropertyFormat::M16_BITS),
            XIChangePropertyAux::Data32(_) => u8::from(PropertyFormat::M32_BITS),
            XIChangePropertyAux::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod xi_change_property_aux {
    use super::XIChangePropertyAux;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIChangePropertyAux {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..3) {
                0 => Self::Data8(GenRandom::generate(rng)),
                1 => Self::Data16(GenRandom::generate(rng)),
                _ => Self::Data32(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<XIChangePropertyAux> {
        alloc::vec![
            XIChangePropertyAux::Data8(GenRandom::generate(rng)),
            XIChangePropertyAux::Data16(GenRandom::generate(rng)),
            XIChangePropertyAux::Data32(GenRandom::generate(rng)),
        ]
    }
}

/// Opcode for the XIChangeProperty request
pub const XI_CHANGE_PROPERTY_REQUEST: u8 = 57;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIChangePropertyRequest<'input> {
    pub deviceid: DeviceId,
    pub mode: xproto::PropMode,
    pub property: xproto::Atom,
    pub type_: xproto::Atom,
    pub num_items: u32,
    pub items: Cow<'input, XIChangePropertyAux>,
}
impl<'input> XIChangePropertyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let deviceid_bytes = self.deviceid.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
        let format: u8 = self.items.switch_expr();
        let format_bytes = format.serialize();
        let property_bytes = self.property.serialize();
        let type_bytes = self.type_.serialize();
        let num_items_bytes = self.num_items.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_CHANGE_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let items_bytes = self.items.serialize(format, self.num_items);
        let length_so_far = length_so_far + items_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), items_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_CHANGE_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (deviceid, remaining) = DeviceId::try_parse(value)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let mode = mode.into();
        let (format, remaining) = u8::try_parse(remaining)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let (items, remaining) = XIChangePropertyAux::try_parse(remaining, format, num_items)?;
        let _ = remaining;
        Ok(XIChangePropertyRequest {
            deviceid,
            mode,
            property,
            type_,
            num_items,
            items: Cow::Owned(items),
        })
    }
    /// Clone all borrowed data in this XIChangePropertyRequest.
    pub fn into_owned(self) -> XIChangePropertyRequest<'static> {
        XIChangePropertyRequest {
            deviceid: self.deviceid,
            mode: self.mode,
            property: self.property,
            type_: self.type_,
            num_items: self.num_items,
            items: Cow::Owned(self.items.into_owned()),
        }
    }
}
impl<'input> Request for XIChangePropertyRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for XIChangePropertyRequest<'input> {
}

/// Opcode for the XIDeleteProperty request
pub const XI_DELETE_PROPERTY_REQUEST: u8 = 58;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIDeletePropertyRequest {
    pub deviceid: DeviceId,
    pub property: xproto::Atom,
}
impl XIDeletePropertyRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let deviceid_bytes = self.deviceid.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_DELETE_PROPERTY_REQUEST,
            0,
            0,
            deviceid_bytes[0],
            deviceid_bytes[1],
            0,
            0,
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_DELETE_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (deviceid, remaining) = DeviceId::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(XIDeletePropertyRequest {
            deviceid,
            property,
        })
    }
}
impl Request for XIDeletePropertyRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for XIDeletePropertyRequest {
}

/// Opcode for the XIGetProperty request
pub const XI_GET_PROPERTY_REQUEST: u8 = 59;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGetPropertyRequest {
    pub deviceid: DeviceId,
    pub delete: bool,
    pub property: xproto::Atom,
    pub type_: xproto::Atom,
    pub offset: u32,
    pub len: u32,
}
impl XIGetPropertyRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let deviceid_bytes = self.deviceid.serialize();
        let delete_bytes = self.delete.serialize();
        let property_bytes = self.property.serialize();
        let type_bytes = self.type_.serialize();
        let offset_bytes = self.offset.serialize();
        let len_bytes = self.len.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_GET_PROPERTY_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_GET_PROPERTY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (deviceid, remaining) = DeviceId::try_parse(value)?;
        let (delete, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let (len, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(XIGetPropertyRequest {
            deviceid,
            delete,
            property,
            type_,
            offset,
            len,
        })
    }
}
impl Request for XIGetPropertyRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for XIGetPropertyRequest {
    type Reply = XIGetPropertyReply;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum XIGetPropertyItems {
    Data8(Vec<u8>),
    Data16(Vec<u16>),
    Data32(Vec<u32>),
    /// This variant is returned when the server sends a discriminant
    /// value that does not match any of the defined by the protocol.
    ///
    /// Usually, this should be considered a parsing error, but there
    /// are some cases where the server violates the protocol.
    ///
    /// Trying to use `serialize` or `serialize_into` with this variant
    /// will raise a panic.
    InvalidValue(u8),
}
impl XIGetPropertyItems {
    fn try_parse(value: &[u8], format: u8, num_items: u32) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = format;
        let mut outer_remaining = value;
        let mut parse_result = None;
        if switch_expr == u8::from(PropertyFormat::M8_BITS) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data8, remaining) = crate::x11_utils::parse_u8_list(remaining, num_items.try_to_usize()?)?;
            let data8 = data8.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIGetPropertyItems::Data8(data8));
        }
        if switch_expr == u8::from(PropertyFormat::M16_BITS) {
            let remaining = outer_remaining;
            let value = remaining;
            let (data16, remaining) = crate::x11_utils::parse_list::<u16>(remaining, num_items.try_to_usize()?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIGetPropertyItems::Data16(data16));
        }
        if switch_expr == u8::from(PropertyFormat::M32_BITS) {
            let remaining = outer_remaining;
            let (data32, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_items.try_to_usize()?)?;
            outer_remaining = remaining;
            assert!(parse_result.is_none(), "The XML should prevent more than one 'if' from matching");
            parse_result = Some(XIGetPropertyItems::Data32(data32));
        }
        match parse_result {
            None => Ok((XIGetPropertyItems::InvalidValue(switch_expr), &[])),
            Some(result) => Ok((result, outer_remaining)),
        }
    }
}
impl XIGetPropertyItems {
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
impl XIGetPropertyItems {
    #[allow(dead_code)]
    fn serialize(&self, format: u8, num_items: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, format, num_items);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, format: u8, num_items: u32) {
        let _ = format;
        let _ = num_items;
        match self {
            XIGetPropertyItems::Data8(data8) => {
                assert_eq!(data8.len(), usize::try_from(num_items).unwrap(), "`data8` has an incorrect length");
                bytes.extend_from_slice(&data8);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            XIGetPropertyItems::Data16(data16) => {
                assert_eq!(data16.len(), usize::try_from(num_items).unwrap(), "`data16` has an incorrect length");
                data16.serialize_into(bytes);
                bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
            }
            XIGetPropertyItems::Data32(data32) => {
                assert_eq!(data32.len(), usize::try_from(num_items).unwrap(), "`data32` has an incorrect length");
                data32.serialize_into(bytes);
            }
            XIGetPropertyItems::InvalidValue(_) => panic!("attempted to serialize invalid switch case"),
        }
    }
}
impl XIGetPropertyItems {
    fn switch_expr(&self) -> u8 {
        match self {
            XIGetPropertyItems::Data8(_) => u8::from(PropertyFormat::M8_BITS),
            XIGetPropertyItems::Data16(_) => u8::from(PropertyFormat::M16_BITS),
            XIGetPropertyItems::Data32(_) => u8::from(PropertyFormat::M32_BITS),
            XIGetPropertyItems::InvalidValue(switch_expr) => *switch_expr,
        }
    }
}
#[cfg(test)]
mod xi_get_property_items {
    use super::XIGetPropertyItems;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIGetPropertyItems {
        fn generate(rng: &Rng) -> Self {
            match rng.usize(..3) {
                0 => Self::Data8(GenRandom::generate(rng)),
                1 => Self::Data16(GenRandom::generate(rng)),
                _ => Self::Data32(GenRandom::generate(rng)),
            }
        }
    }
    fn generate_values(rng: &Rng) -> alloc::vec::Vec<XIGetPropertyItems> {
        alloc::vec![
            XIGetPropertyItems::Data8(GenRandom::generate(rng)),
            XIGetPropertyItems::Data16(GenRandom::generate(rng)),
            XIGetPropertyItems::Data32(GenRandom::generate(rng)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGetPropertyReply {
    pub sequence: u16,
    pub length: u32,
    pub type_: xproto::Atom,
    pub bytes_after: u32,
    pub num_items: u32,
    pub items: XIGetPropertyItems,
}
impl TryParse for XIGetPropertyReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = xproto::Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (num_items, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::InsufficientData)?;
        let (items, remaining) = XIGetPropertyItems::try_parse(remaining, format, num_items)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIGetPropertyReply { sequence, length, type_, bytes_after, num_items, items };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIGetPropertyReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.type_.serialize_into(bytes);
        self.bytes_after.serialize_into(bytes);
        self.num_items.serialize_into(bytes);
        let format: u8 = self.items.switch_expr();
        format.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 11]);
        self.items.serialize_into(bytes, format, self.num_items);
    }
}
#[cfg(test)]
mod xi_get_property_reply {
    use super::XIGetPropertyReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIGetPropertyReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                type_: GenRandom::generate(rng),
                bytes_after: GenRandom::generate(rng),
                num_items: GenRandom::generate(rng),
                items: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2704719749954142208);
        let value = XIGetPropertyReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIGetSelectedEvents request
pub const XI_GET_SELECTED_EVENTS_REQUEST: u8 = 60;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGetSelectedEventsRequest {
    pub window: xproto::Window,
}
impl XIGetSelectedEventsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_GET_SELECTED_EVENTS_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_GET_SELECTED_EVENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(XIGetSelectedEventsRequest {
            window,
        })
    }
}
impl Request for XIGetSelectedEventsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for XIGetSelectedEventsRequest {
    type Reply = XIGetSelectedEventsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIGetSelectedEventsReply {
    pub sequence: u16,
    pub length: u32,
    pub masks: Vec<EventMask>,
}
impl TryParse for XIGetSelectedEventsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_masks, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (masks, remaining) = crate::x11_utils::parse_list::<EventMask>(remaining, num_masks.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = XIGetSelectedEventsReply { sequence, length, masks };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for XIGetSelectedEventsReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_masks = u16::try_from(self.masks.len()).expect("`masks` has too many elements");
        num_masks.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 22]);
        self.masks.serialize_into(bytes);
    }
}
impl XIGetSelectedEventsReply {
    /// Get the value of the `num_masks` field.
    ///
    /// The `num_masks` field is used as the length field of the `masks` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_masks(&self) -> u16 {
        self.masks.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod xi_get_selected_events_reply {
    use super::XIGetSelectedEventsReply;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for XIGetSelectedEventsReply {
        fn generate(rng: &Rng) -> Self {
            Self {
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                masks: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11444888129673101312);
        let value = XIGetSelectedEventsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarrierReleasePointerInfo {
    pub deviceid: DeviceId,
    pub barrier: xfixes::Barrier,
    pub eventid: u32,
}
impl TryParse for BarrierReleasePointerInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (barrier, remaining) = xfixes::Barrier::try_parse(remaining)?;
        let (eventid, remaining) = u32::try_parse(remaining)?;
        let result = BarrierReleasePointerInfo { deviceid, barrier, eventid };
        Ok((result, remaining))
    }
}
impl Serialize for BarrierReleasePointerInfo {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
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
#[cfg(test)]
mod barrier_release_pointer_info {
    use super::BarrierReleasePointerInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for BarrierReleasePointerInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                deviceid: GenRandom::generate(rng),
                barrier: GenRandom::generate(rng),
                eventid: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(169789029248532480);
        let value = BarrierReleasePointerInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the XIBarrierReleasePointer request
pub const XI_BARRIER_RELEASE_POINTER_REQUEST: u8 = 61;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XIBarrierReleasePointerRequest<'input> {
    pub barriers: Cow<'input, [BarrierReleasePointerInfo]>,
}
impl<'input> XIBarrierReleasePointerRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let num_barriers = u32::try_from(self.barriers.len()).expect("`barriers` has too many elements");
        let num_barriers_bytes = num_barriers.serialize();
        let mut request0 = vec![
            major_opcode,
            XI_BARRIER_RELEASE_POINTER_REQUEST,
            0,
            0,
            num_barriers_bytes[0],
            num_barriers_bytes[1],
            num_barriers_bytes[2],
            num_barriers_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let barriers_bytes = self.barriers.serialize();
        let length_so_far = length_so_far + barriers_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), barriers_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != XI_BARRIER_RELEASE_POINTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (num_barriers, remaining) = u32::try_parse(value)?;
        let (barriers, remaining) = crate::x11_utils::parse_list::<BarrierReleasePointerInfo>(remaining, num_barriers.try_to_usize()?)?;
        let _ = remaining;
        Ok(XIBarrierReleasePointerRequest {
            barriers: Cow::Owned(barriers),
        })
    }
    /// Clone all borrowed data in this XIBarrierReleasePointerRequest.
    pub fn into_owned(self) -> XIBarrierReleasePointerRequest<'static> {
        XIBarrierReleasePointerRequest {
            barriers: Cow::Owned(self.barriers.into_owned()),
        }
    }
}
impl<'input> Request for XIBarrierReleasePointerRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for XIBarrierReleasePointerRequest<'input> {
}

/// Opcode for the DeviceValuator event
pub const DEVICE_VALUATOR_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceValuatorEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub device_state: u16,
    pub num_valuators: u8,
    pub first_valuator: u8,
    pub valuators: [i32; 6],
}
impl TryParse for DeviceValuatorEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DeviceValuatorEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let device_id_bytes = self.device_id.serialize();
        let sequence_bytes = self.sequence.serialize();
        let device_state_bytes = self.device_state.serialize();
        let num_valuators_bytes = self.num_valuators.serialize();
        let first_valuator_bytes = self.first_valuator.serialize();
        let valuators_0_bytes = self.valuators[0].serialize();
        let valuators_1_bytes = self.valuators[1].serialize();
        let valuators_2_bytes = self.valuators[2].serialize();
        let valuators_3_bytes = self.valuators[3].serialize();
        let valuators_4_bytes = self.valuators[4].serialize();
        let valuators_5_bytes = self.valuators[5].serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            device_state_bytes[0],
            device_state_bytes[1],
            num_valuators_bytes[0],
            first_valuator_bytes[0],
            valuators_0_bytes[0],
            valuators_0_bytes[1],
            valuators_0_bytes[2],
            valuators_0_bytes[3],
            valuators_1_bytes[0],
            valuators_1_bytes[1],
            valuators_1_bytes[2],
            valuators_1_bytes[3],
            valuators_2_bytes[0],
            valuators_2_bytes[1],
            valuators_2_bytes[2],
            valuators_2_bytes[3],
            valuators_3_bytes[0],
            valuators_3_bytes[1],
            valuators_3_bytes[2],
            valuators_3_bytes[3],
            valuators_4_bytes[0],
            valuators_4_bytes[1],
            valuators_4_bytes[2],
            valuators_4_bytes[3],
            valuators_5_bytes[0],
            valuators_5_bytes[1],
            valuators_5_bytes[2],
            valuators_5_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.device_state.serialize_into(bytes);
        self.num_valuators.serialize_into(bytes);
        self.first_valuator.serialize_into(bytes);
        self.valuators.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_valuator_event {
    use super::DeviceValuatorEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceValuatorEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                device_state: GenRandom::generate(rng),
                num_valuators: GenRandom::generate(rng),
                first_valuator: GenRandom::generate(rng),
                valuators: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1718806423994884096);
        let value = DeviceValuatorEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DeviceValuatorEvent> for [u8; 32] {
    fn from(input: &DeviceValuatorEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let device_id_bytes = input.device_id.serialize();
        let sequence_bytes = input.sequence.serialize();
        let device_state_bytes = input.device_state.serialize();
        let num_valuators_bytes = input.num_valuators.serialize();
        let first_valuator_bytes = input.first_valuator.serialize();
        let valuators_0_bytes = input.valuators[0].serialize();
        let valuators_1_bytes = input.valuators[1].serialize();
        let valuators_2_bytes = input.valuators[2].serialize();
        let valuators_3_bytes = input.valuators[3].serialize();
        let valuators_4_bytes = input.valuators[4].serialize();
        let valuators_5_bytes = input.valuators[5].serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            device_state_bytes[0],
            device_state_bytes[1],
            num_valuators_bytes[0],
            first_valuator_bytes[0],
            valuators_0_bytes[0],
            valuators_0_bytes[1],
            valuators_0_bytes[2],
            valuators_0_bytes[3],
            valuators_1_bytes[0],
            valuators_1_bytes[1],
            valuators_1_bytes[2],
            valuators_1_bytes[3],
            valuators_2_bytes[0],
            valuators_2_bytes[1],
            valuators_2_bytes[2],
            valuators_2_bytes[3],
            valuators_3_bytes[0],
            valuators_3_bytes[1],
            valuators_3_bytes[2],
            valuators_3_bytes[3],
            valuators_4_bytes[0],
            valuators_4_bytes[1],
            valuators_4_bytes[2],
            valuators_4_bytes[3],
            valuators_5_bytes[0],
            valuators_5_bytes[1],
            valuators_5_bytes[2],
            valuators_5_bytes[3],
        ]
    }
}
impl From<DeviceValuatorEvent> for [u8; 32] {
    fn from(input: DeviceValuatorEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MoreEventsMask(u8);
impl MoreEventsMask {
    pub const MORE_EVENTS: Self = Self(1 << 7);
}
impl From<MoreEventsMask> for u8 {
    #[inline]
    fn from(input: MoreEventsMask) -> Self {
        input.0
    }
}
impl From<MoreEventsMask> for Option<u8> {
    #[inline]
    fn from(input: MoreEventsMask) -> Self {
        Some(input.0)
    }
}
impl From<MoreEventsMask> for u16 {
    #[inline]
    fn from(input: MoreEventsMask) -> Self {
        u16::from(input.0)
    }
}
impl From<MoreEventsMask> for Option<u16> {
    #[inline]
    fn from(input: MoreEventsMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<MoreEventsMask> for u32 {
    #[inline]
    fn from(input: MoreEventsMask) -> Self {
        u32::from(input.0)
    }
}
impl From<MoreEventsMask> for Option<u32> {
    #[inline]
    fn from(input: MoreEventsMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for MoreEventsMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for MoreEventsMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::MORE_EVENTS.0.into(), "MORE_EVENTS", "MoreEvents"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(MoreEventsMask, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for MoreEventsMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::MORE_EVENTS.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the DeviceKeyPress event
pub const DEVICE_KEY_PRESS_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for DeviceKeyPressEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DeviceKeyPressEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let detail_bytes = self.detail.serialize();
        let sequence_bytes = self.sequence.serialize();
        let time_bytes = self.time.serialize();
        let root_bytes = self.root.serialize();
        let event_bytes = self.event.serialize();
        let child_bytes = self.child.serialize();
        let root_x_bytes = self.root_x.serialize();
        let root_y_bytes = self.root_y.serialize();
        let event_x_bytes = self.event_x.serialize();
        let event_y_bytes = self.event_y.serialize();
        let state_bytes = self.state.serialize();
        let same_screen_bytes = self.same_screen.serialize();
        let device_id_bytes = self.device_id.serialize();
        [
            response_type_bytes[0],
            detail_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            root_bytes[0],
            root_bytes[1],
            root_bytes[2],
            root_bytes[3],
            event_bytes[0],
            event_bytes[1],
            event_bytes[2],
            event_bytes[3],
            child_bytes[0],
            child_bytes[1],
            child_bytes[2],
            child_bytes[3],
            root_x_bytes[0],
            root_x_bytes[1],
            root_y_bytes[0],
            root_y_bytes[1],
            event_x_bytes[0],
            event_x_bytes[1],
            event_y_bytes[0],
            event_y_bytes[1],
            state_bytes[0],
            state_bytes[1],
            same_screen_bytes[0],
            device_id_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.detail.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.root.serialize_into(bytes);
        self.event.serialize_into(bytes);
        self.child.serialize_into(bytes);
        self.root_x.serialize_into(bytes);
        self.root_y.serialize_into(bytes);
        self.event_x.serialize_into(bytes);
        self.event_y.serialize_into(bytes);
        self.state.serialize_into(bytes);
        self.same_screen.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_key_press_event {
    use super::DeviceKeyPressEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceKeyPressEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                root: GenRandom::generate(rng),
                event: GenRandom::generate(rng),
                child: GenRandom::generate(rng),
                root_x: GenRandom::generate(rng),
                root_y: GenRandom::generate(rng),
                event_x: GenRandom::generate(rng),
                event_y: GenRandom::generate(rng),
                state: GenRandom::generate(rng),
                same_screen: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(726094504554491904);
        let value = DeviceKeyPressEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DeviceKeyPressEvent> for [u8; 32] {
    fn from(input: &DeviceKeyPressEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let detail_bytes = input.detail.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let root_bytes = input.root.serialize();
        let event_bytes = input.event.serialize();
        let child_bytes = input.child.serialize();
        let root_x_bytes = input.root_x.serialize();
        let root_y_bytes = input.root_y.serialize();
        let event_x_bytes = input.event_x.serialize();
        let event_y_bytes = input.event_y.serialize();
        let state_bytes = input.state.serialize();
        let same_screen_bytes = input.same_screen.serialize();
        let device_id_bytes = input.device_id.serialize();
        [
            response_type_bytes[0],
            detail_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            root_bytes[0],
            root_bytes[1],
            root_bytes[2],
            root_bytes[3],
            event_bytes[0],
            event_bytes[1],
            event_bytes[2],
            event_bytes[3],
            child_bytes[0],
            child_bytes[1],
            child_bytes[2],
            child_bytes[3],
            root_x_bytes[0],
            root_x_bytes[1],
            root_y_bytes[0],
            root_y_bytes[1],
            event_x_bytes[0],
            event_x_bytes[1],
            event_y_bytes[0],
            event_y_bytes[1],
            state_bytes[0],
            state_bytes[1],
            same_screen_bytes[0],
            device_id_bytes[0],
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
pub type DeviceKeyReleaseEvent = DeviceKeyPressEvent;

/// Opcode for the DeviceButtonPress event
pub const DEVICE_BUTTON_PRESS_EVENT: u8 = 3;
pub type DeviceButtonPressEvent = DeviceKeyPressEvent;

/// Opcode for the DeviceButtonRelease event
pub const DEVICE_BUTTON_RELEASE_EVENT: u8 = 4;
pub type DeviceButtonReleaseEvent = DeviceKeyPressEvent;

/// Opcode for the DeviceMotionNotify event
pub const DEVICE_MOTION_NOTIFY_EVENT: u8 = 5;
pub type DeviceMotionNotifyEvent = DeviceKeyPressEvent;

/// Opcode for the DeviceFocusIn event
pub const DEVICE_FOCUS_IN_EVENT: u8 = 6;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceFocusInEvent {
    pub response_type: u8,
    pub detail: xproto::NotifyDetail,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub window: xproto::Window,
    pub mode: xproto::NotifyMode,
    pub device_id: u8,
}
impl TryParse for DeviceFocusInEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::InsufficientData)?;
        let detail = detail.into();
        let mode = mode.into();
        let result = DeviceFocusInEvent { response_type, detail, sequence, time, window, mode, device_id };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DeviceFocusInEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let detail_bytes = u8::from(self.detail).serialize();
        let sequence_bytes = self.sequence.serialize();
        let time_bytes = self.time.serialize();
        let window_bytes = self.window.serialize();
        let mode_bytes = u8::from(self.mode).serialize();
        let device_id_bytes = self.device_id.serialize();
        [
            response_type_bytes[0],
            detail_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            mode_bytes[0],
            device_id_bytes[0],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.response_type.serialize_into(bytes);
        u8::from(self.detail).serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.window.serialize_into(bytes);
        u8::from(self.mode).serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 18]);
    }
}
#[cfg(test)]
mod device_focus_in_event {
    use super::DeviceFocusInEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceFocusInEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                window: GenRandom::generate(rng),
                mode: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5443457177368555008);
        let value = DeviceFocusInEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DeviceFocusInEvent> for [u8; 32] {
    fn from(input: &DeviceFocusInEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let detail_bytes = u8::from(input.detail).serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let window_bytes = input.window.serialize();
        let mode_bytes = u8::from(input.mode).serialize();
        let device_id_bytes = input.device_id.serialize();
        [
            response_type_bytes[0],
            detail_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            mode_bytes[0],
            device_id_bytes[0],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
impl From<DeviceFocusInEvent> for [u8; 32] {
    fn from(input: DeviceFocusInEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceFocusOut event
pub const DEVICE_FOCUS_OUT_EVENT: u8 = 7;
pub type DeviceFocusOutEvent = DeviceFocusInEvent;

/// Opcode for the ProximityIn event
pub const PROXIMITY_IN_EVENT: u8 = 8;
pub type ProximityInEvent = DeviceKeyPressEvent;

/// Opcode for the ProximityOut event
pub const PROXIMITY_OUT_EVENT: u8 = 9;
pub type ProximityOutEvent = DeviceKeyPressEvent;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassesReportedMask(u8);
impl ClassesReportedMask {
    pub const OUT_OF_PROXIMITY: Self = Self(1 << 7);
    pub const DEVICE_MODE_ABSOLUTE: Self = Self(1 << 6);
    pub const REPORTING_VALUATORS: Self = Self(1 << 2);
    pub const REPORTING_BUTTONS: Self = Self(1 << 1);
    pub const REPORTING_KEYS: Self = Self(1 << 0);
}
impl From<ClassesReportedMask> for u8 {
    #[inline]
    fn from(input: ClassesReportedMask) -> Self {
        input.0
    }
}
impl From<ClassesReportedMask> for Option<u8> {
    #[inline]
    fn from(input: ClassesReportedMask) -> Self {
        Some(input.0)
    }
}
impl From<ClassesReportedMask> for u16 {
    #[inline]
    fn from(input: ClassesReportedMask) -> Self {
        u16::from(input.0)
    }
}
impl From<ClassesReportedMask> for Option<u16> {
    #[inline]
    fn from(input: ClassesReportedMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ClassesReportedMask> for u32 {
    #[inline]
    fn from(input: ClassesReportedMask) -> Self {
        u32::from(input.0)
    }
}
impl From<ClassesReportedMask> for Option<u32> {
    #[inline]
    fn from(input: ClassesReportedMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ClassesReportedMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ClassesReportedMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::OUT_OF_PROXIMITY.0.into(), "OUT_OF_PROXIMITY", "OutOfProximity"),
            (Self::DEVICE_MODE_ABSOLUTE.0.into(), "DEVICE_MODE_ABSOLUTE", "DeviceModeAbsolute"),
            (Self::REPORTING_VALUATORS.0.into(), "REPORTING_VALUATORS", "ReportingValuators"),
            (Self::REPORTING_BUTTONS.0.into(), "REPORTING_BUTTONS", "ReportingButtons"),
            (Self::REPORTING_KEYS.0.into(), "REPORTING_KEYS", "ReportingKeys"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(ClassesReportedMask, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for ClassesReportedMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::OUT_OF_PROXIMITY.0,
            Self::DEVICE_MODE_ABSOLUTE.0,
            Self::REPORTING_VALUATORS.0,
            Self::REPORTING_BUTTONS.0,
            Self::REPORTING_KEYS.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the DeviceStateNotify event
pub const DEVICE_STATE_NOTIFY_EVENT: u8 = 10;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for DeviceStateNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (num_keys, remaining) = u8::try_parse(remaining)?;
        let (num_buttons, remaining) = u8::try_parse(remaining)?;
        let (num_valuators, remaining) = u8::try_parse(remaining)?;
        let (classes_reported, remaining) = u8::try_parse(remaining)?;
        let (buttons, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let buttons = <[u8; 4]>::try_from(buttons).unwrap();
        let (keys, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let keys = <[u8; 4]>::try_from(keys).unwrap();
        let (valuators_0, remaining) = u32::try_parse(remaining)?;
        let (valuators_1, remaining) = u32::try_parse(remaining)?;
        let (valuators_2, remaining) = u32::try_parse(remaining)?;
        let valuators = [
            valuators_0,
            valuators_1,
            valuators_2,
        ];
        let result = DeviceStateNotifyEvent { response_type, device_id, sequence, time, num_keys, num_buttons, num_valuators, classes_reported, buttons, keys, valuators };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DeviceStateNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let device_id_bytes = self.device_id.serialize();
        let sequence_bytes = self.sequence.serialize();
        let time_bytes = self.time.serialize();
        let num_keys_bytes = self.num_keys.serialize();
        let num_buttons_bytes = self.num_buttons.serialize();
        let num_valuators_bytes = self.num_valuators.serialize();
        let classes_reported_bytes = self.classes_reported.serialize();
        let valuators_0_bytes = self.valuators[0].serialize();
        let valuators_1_bytes = self.valuators[1].serialize();
        let valuators_2_bytes = self.valuators[2].serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            num_keys_bytes[0],
            num_buttons_bytes[0],
            num_valuators_bytes[0],
            classes_reported_bytes[0],
            self.buttons[0],
            self.buttons[1],
            self.buttons[2],
            self.buttons[3],
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            valuators_0_bytes[0],
            valuators_0_bytes[1],
            valuators_0_bytes[2],
            valuators_0_bytes[3],
            valuators_1_bytes[0],
            valuators_1_bytes[1],
            valuators_1_bytes[2],
            valuators_1_bytes[3],
            valuators_2_bytes[0],
            valuators_2_bytes[1],
            valuators_2_bytes[2],
            valuators_2_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.num_keys.serialize_into(bytes);
        self.num_buttons.serialize_into(bytes);
        self.num_valuators.serialize_into(bytes);
        self.classes_reported.serialize_into(bytes);
        bytes.extend_from_slice(&self.buttons);
        bytes.extend_from_slice(&self.keys);
        self.valuators.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_state_notify_event {
    use super::DeviceStateNotifyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceStateNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                num_keys: GenRandom::generate(rng),
                num_buttons: GenRandom::generate(rng),
                num_valuators: GenRandom::generate(rng),
                classes_reported: GenRandom::generate(rng),
                buttons: GenRandom::generate(rng),
                keys: GenRandom::generate(rng),
                valuators: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10988796715819892736);
        let value = DeviceStateNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DeviceStateNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceStateNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let device_id_bytes = input.device_id.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let num_keys_bytes = input.num_keys.serialize();
        let num_buttons_bytes = input.num_buttons.serialize();
        let num_valuators_bytes = input.num_valuators.serialize();
        let classes_reported_bytes = input.classes_reported.serialize();
        let valuators_0_bytes = input.valuators[0].serialize();
        let valuators_1_bytes = input.valuators[1].serialize();
        let valuators_2_bytes = input.valuators[2].serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            num_keys_bytes[0],
            num_buttons_bytes[0],
            num_valuators_bytes[0],
            classes_reported_bytes[0],
            input.buttons[0],
            input.buttons[1],
            input.buttons[2],
            input.buttons[3],
            input.keys[0],
            input.keys[1],
            input.keys[2],
            input.keys[3],
            valuators_0_bytes[0],
            valuators_0_bytes[1],
            valuators_0_bytes[2],
            valuators_0_bytes[3],
            valuators_1_bytes[0],
            valuators_1_bytes[1],
            valuators_1_bytes[2],
            valuators_1_bytes[3],
            valuators_2_bytes[0],
            valuators_2_bytes[1],
            valuators_2_bytes[2],
            valuators_2_bytes[3],
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceMappingNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub request: xproto::Mapping,
    pub first_keycode: KeyCode,
    pub count: u8,
    pub time: xproto::Timestamp,
}
impl TryParse for DeviceMappingNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (request, remaining) = u8::try_parse(remaining)?;
        let (first_keycode, remaining) = KeyCode::try_parse(remaining)?;
        let (count, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let request = request.into();
        let result = DeviceMappingNotifyEvent { response_type, device_id, sequence, request, first_keycode, count, time };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DeviceMappingNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let device_id_bytes = self.device_id.serialize();
        let sequence_bytes = self.sequence.serialize();
        let request_bytes = u8::from(self.request).serialize();
        let first_keycode_bytes = self.first_keycode.serialize();
        let count_bytes = self.count.serialize();
        let time_bytes = self.time.serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            request_bytes[0],
            first_keycode_bytes[0],
            count_bytes[0],
            0,
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.response_type.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        u8::from(self.request).serialize_into(bytes);
        self.first_keycode.serialize_into(bytes);
        self.count.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.time.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}
#[cfg(test)]
mod device_mapping_notify_event {
    use super::DeviceMappingNotifyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceMappingNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                request: GenRandom::generate(rng),
                first_keycode: GenRandom::generate(rng),
                count: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1599943193488523264);
        let value = DeviceMappingNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DeviceMappingNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceMappingNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let device_id_bytes = input.device_id.serialize();
        let sequence_bytes = input.sequence.serialize();
        let request_bytes = u8::from(input.request).serialize();
        let first_keycode_bytes = input.first_keycode.serialize();
        let count_bytes = input.count.serialize();
        let time_bytes = input.time.serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            request_bytes[0],
            first_keycode_bytes[0],
            count_bytes[0],
            0,
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
impl From<DeviceMappingNotifyEvent> for [u8; 32] {
    fn from(input: DeviceMappingNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDevice(u8);
impl ChangeDevice {
    pub const NEW_POINTER: Self = Self(0);
    pub const NEW_KEYBOARD: Self = Self(1);
}
impl From<ChangeDevice> for u8 {
    #[inline]
    fn from(input: ChangeDevice) -> Self {
        input.0
    }
}
impl From<ChangeDevice> for Option<u8> {
    #[inline]
    fn from(input: ChangeDevice) -> Self {
        Some(input.0)
    }
}
impl From<ChangeDevice> for u16 {
    #[inline]
    fn from(input: ChangeDevice) -> Self {
        u16::from(input.0)
    }
}
impl From<ChangeDevice> for Option<u16> {
    #[inline]
    fn from(input: ChangeDevice) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ChangeDevice> for u32 {
    #[inline]
    fn from(input: ChangeDevice) -> Self {
        u32::from(input.0)
    }
}
impl From<ChangeDevice> for Option<u32> {
    #[inline]
    fn from(input: ChangeDevice) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ChangeDevice {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ChangeDevice  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::NEW_POINTER.0.into(), "NEW_POINTER", "NewPointer"),
            (Self::NEW_KEYBOARD.0.into(), "NEW_KEYBOARD", "NewKeyboard"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for ChangeDevice {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::NEW_POINTER.0,
            Self::NEW_KEYBOARD.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the ChangeDeviceNotify event
pub const CHANGE_DEVICE_NOTIFY_EVENT: u8 = 12;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDeviceNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub request: ChangeDevice,
}
impl TryParse for ChangeDeviceNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (request, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        let request = request.into();
        let result = ChangeDeviceNotifyEvent { response_type, device_id, sequence, time, request };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ChangeDeviceNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let device_id_bytes = self.device_id.serialize();
        let sequence_bytes = self.sequence.serialize();
        let time_bytes = self.time.serialize();
        let request_bytes = u8::from(self.request).serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            request_bytes[0],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.response_type.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.time.serialize_into(bytes);
        u8::from(self.request).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod change_device_notify_event {
    use super::ChangeDeviceNotifyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ChangeDeviceNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                request: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9866718034532925440);
        let value = ChangeDeviceNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&ChangeDeviceNotifyEvent> for [u8; 32] {
    fn from(input: &ChangeDeviceNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let device_id_bytes = input.device_id.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let request_bytes = u8::from(input.request).serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            request_bytes[0],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
impl From<ChangeDeviceNotifyEvent> for [u8; 32] {
    fn from(input: ChangeDeviceNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DeviceKeyStateNotify event
pub const DEVICE_KEY_STATE_NOTIFY_EVENT: u8 = 13;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceKeyStateNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub keys: [u8; 28],
}
impl TryParse for DeviceKeyStateNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (keys, remaining) = crate::x11_utils::parse_u8_list(remaining, 28)?;
        let keys = <[u8; 28]>::try_from(keys).unwrap();
        let result = DeviceKeyStateNotifyEvent { response_type, device_id, sequence, keys };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DeviceKeyStateNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let device_id_bytes = self.device_id.serialize();
        let sequence_bytes = self.sequence.serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
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
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        bytes.extend_from_slice(&self.keys);
    }
}
#[cfg(test)]
mod device_key_state_notify_event {
    use super::DeviceKeyStateNotifyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceKeyStateNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                keys: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4958348687035301888);
        let value = DeviceKeyStateNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DeviceKeyStateNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceKeyStateNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let device_id_bytes = input.device_id.serialize();
        let sequence_bytes = input.sequence.serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            input.keys[0],
            input.keys[1],
            input.keys[2],
            input.keys[3],
            input.keys[4],
            input.keys[5],
            input.keys[6],
            input.keys[7],
            input.keys[8],
            input.keys[9],
            input.keys[10],
            input.keys[11],
            input.keys[12],
            input.keys[13],
            input.keys[14],
            input.keys[15],
            input.keys[16],
            input.keys[17],
            input.keys[18],
            input.keys[19],
            input.keys[20],
            input.keys[21],
            input.keys[22],
            input.keys[23],
            input.keys[24],
            input.keys[25],
            input.keys[26],
            input.keys[27],
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceButtonStateNotifyEvent {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub buttons: [u8; 28],
}
impl TryParse for DeviceButtonStateNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (buttons, remaining) = crate::x11_utils::parse_u8_list(remaining, 28)?;
        let buttons = <[u8; 28]>::try_from(buttons).unwrap();
        let result = DeviceButtonStateNotifyEvent { response_type, device_id, sequence, buttons };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DeviceButtonStateNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let device_id_bytes = self.device_id.serialize();
        let sequence_bytes = self.sequence.serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
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
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        bytes.extend_from_slice(&self.buttons);
    }
}
#[cfg(test)]
mod device_button_state_notify_event {
    use super::DeviceButtonStateNotifyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceButtonStateNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                buttons: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8436103572204027904);
        let value = DeviceButtonStateNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DeviceButtonStateNotifyEvent> for [u8; 32] {
    fn from(input: &DeviceButtonStateNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let device_id_bytes = input.device_id.serialize();
        let sequence_bytes = input.sequence.serialize();
        [
            response_type_bytes[0],
            device_id_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            input.buttons[0],
            input.buttons[1],
            input.buttons[2],
            input.buttons[3],
            input.buttons[4],
            input.buttons[5],
            input.buttons[6],
            input.buttons[7],
            input.buttons[8],
            input.buttons[9],
            input.buttons[10],
            input.buttons[11],
            input.buttons[12],
            input.buttons[13],
            input.buttons[14],
            input.buttons[15],
            input.buttons[16],
            input.buttons[17],
            input.buttons[18],
            input.buttons[19],
            input.buttons[20],
            input.buttons[21],
            input.buttons[22],
            input.buttons[23],
            input.buttons[24],
            input.buttons[25],
            input.buttons[26],
            input.buttons[27],
        ]
    }
}
impl From<DeviceButtonStateNotifyEvent> for [u8; 32] {
    fn from(input: DeviceButtonStateNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceChange(u8);
impl DeviceChange {
    pub const ADDED: Self = Self(0);
    pub const REMOVED: Self = Self(1);
    pub const ENABLED: Self = Self(2);
    pub const DISABLED: Self = Self(3);
    pub const UNRECOVERABLE: Self = Self(4);
    pub const CONTROL_CHANGED: Self = Self(5);
}
impl From<DeviceChange> for u8 {
    #[inline]
    fn from(input: DeviceChange) -> Self {
        input.0
    }
}
impl From<DeviceChange> for Option<u8> {
    #[inline]
    fn from(input: DeviceChange) -> Self {
        Some(input.0)
    }
}
impl From<DeviceChange> for u16 {
    #[inline]
    fn from(input: DeviceChange) -> Self {
        u16::from(input.0)
    }
}
impl From<DeviceChange> for Option<u16> {
    #[inline]
    fn from(input: DeviceChange) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<DeviceChange> for u32 {
    #[inline]
    fn from(input: DeviceChange) -> Self {
        u32::from(input.0)
    }
}
impl From<DeviceChange> for Option<u32> {
    #[inline]
    fn from(input: DeviceChange) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for DeviceChange {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for DeviceChange  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ADDED.0.into(), "ADDED", "Added"),
            (Self::REMOVED.0.into(), "REMOVED", "Removed"),
            (Self::ENABLED.0.into(), "ENABLED", "Enabled"),
            (Self::DISABLED.0.into(), "DISABLED", "Disabled"),
            (Self::UNRECOVERABLE.0.into(), "UNRECOVERABLE", "Unrecoverable"),
            (Self::CONTROL_CHANGED.0.into(), "CONTROL_CHANGED", "ControlChanged"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for DeviceChange {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ADDED.0,
            Self::REMOVED.0,
            Self::ENABLED.0,
            Self::DISABLED.0,
            Self::UNRECOVERABLE.0,
            Self::CONTROL_CHANGED.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the DevicePresenceNotify event
pub const DEVICE_PRESENCE_NOTIFY_EVENT: u8 = 15;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DevicePresenceNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub devchange: DeviceChange,
    pub device_id: u8,
    pub control: u16,
}
impl TryParse for DevicePresenceNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (devchange, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (control, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let devchange = devchange.into();
        let result = DevicePresenceNotifyEvent { response_type, sequence, time, devchange, device_id, control };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DevicePresenceNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let time_bytes = self.time.serialize();
        let devchange_bytes = u8::from(self.devchange).serialize();
        let device_id_bytes = self.device_id.serialize();
        let control_bytes = self.control.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            devchange_bytes[0],
            device_id_bytes[0],
            control_bytes[0],
            control_bytes[1],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        self.response_type.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.time.serialize_into(bytes);
        u8::from(self.devchange).serialize_into(bytes);
        self.device_id.serialize_into(bytes);
        self.control.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}
#[cfg(test)]
mod device_presence_notify_event {
    use super::DevicePresenceNotifyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DevicePresenceNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                devchange: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
                control: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10616172630236069888);
        let value = DevicePresenceNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DevicePresenceNotifyEvent> for [u8; 32] {
    fn from(input: &DevicePresenceNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let devchange_bytes = u8::from(input.devchange).serialize();
        let device_id_bytes = input.device_id.serialize();
        let control_bytes = input.control.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            devchange_bytes[0],
            device_id_bytes[0],
            control_bytes[0],
            control_bytes[1],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
impl From<DevicePresenceNotifyEvent> for [u8; 32] {
    fn from(input: DevicePresenceNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DevicePropertyNotify event
pub const DEVICE_PROPERTY_NOTIFY_EVENT: u8 = 16;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DevicePropertyNotifyEvent {
    pub response_type: u8,
    pub state: xproto::Property,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub property: xproto::Atom,
    pub device_id: u8,
}
impl TryParse for DevicePropertyNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let remaining = remaining.get(19..).ok_or(ParseError::InsufficientData)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let state = state.into();
        let result = DevicePropertyNotifyEvent { response_type, state, sequence, time, property, device_id };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DevicePropertyNotifyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let state_bytes = u8::from(self.state).serialize();
        let sequence_bytes = self.sequence.serialize();
        let time_bytes = self.time.serialize();
        let property_bytes = self.property.serialize();
        let device_id_bytes = self.device_id.serialize();
        [
            response_type_bytes[0],
            state_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            device_id_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        u8::from(self.state).serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.property.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 19]);
        self.device_id.serialize_into(bytes);
    }
}
#[cfg(test)]
mod device_property_notify_event {
    use super::DevicePropertyNotifyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DevicePropertyNotifyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                state: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                property: GenRandom::generate(rng),
                device_id: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12694692425680027648);
        let value = DevicePropertyNotifyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&DevicePropertyNotifyEvent> for [u8; 32] {
    fn from(input: &DevicePropertyNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let state_bytes = u8::from(input.state).serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let property_bytes = input.property.serialize();
        let device_id_bytes = input.device_id.serialize();
        [
            response_type_bytes[0],
            state_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            device_id_bytes[0],
        ]
    }
}
impl From<DevicePropertyNotifyEvent> for [u8; 32] {
    fn from(input: DevicePropertyNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeReason(u8);
impl ChangeReason {
    pub const SLAVE_SWITCH: Self = Self(1);
    pub const DEVICE_CHANGE: Self = Self(2);
}
impl From<ChangeReason> for u8 {
    #[inline]
    fn from(input: ChangeReason) -> Self {
        input.0
    }
}
impl From<ChangeReason> for Option<u8> {
    #[inline]
    fn from(input: ChangeReason) -> Self {
        Some(input.0)
    }
}
impl From<ChangeReason> for u16 {
    #[inline]
    fn from(input: ChangeReason) -> Self {
        u16::from(input.0)
    }
}
impl From<ChangeReason> for Option<u16> {
    #[inline]
    fn from(input: ChangeReason) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ChangeReason> for u32 {
    #[inline]
    fn from(input: ChangeReason) -> Self {
        u32::from(input.0)
    }
}
impl From<ChangeReason> for Option<u32> {
    #[inline]
    fn from(input: ChangeReason) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ChangeReason {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ChangeReason  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::SLAVE_SWITCH.0.into(), "SLAVE_SWITCH", "SlaveSwitch"),
            (Self::DEVICE_CHANGE.0.into(), "DEVICE_CHANGE", "DeviceChange"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for ChangeReason {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::SLAVE_SWITCH.0,
            Self::DEVICE_CHANGE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the DeviceChanged event
pub const DEVICE_CHANGED_EVENT: u16 = 1;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for DeviceChangedEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(11..).ok_or(ParseError::InsufficientData)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<DeviceClass>(remaining, num_classes.try_to_usize()?)?;
        let reason = reason.into();
        let result = DeviceChangedEvent { response_type, extension, sequence, length, event_type, deviceid, time, sourceid, reason, classes };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for DeviceChangedEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        num_classes.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        u8::from(self.reason).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 11]);
        self.classes.serialize_into(bytes);
    }
}
impl DeviceChangedEvent {
    /// Get the value of the `num_classes` field.
    ///
    /// The `num_classes` field is used as the length field of the `classes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_classes(&self) -> u16 {
        self.classes.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod device_changed_event {
    use super::DeviceChangedEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for DeviceChangedEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                reason: GenRandom::generate(rng),
                classes: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11666426160395804672);
        let value = DeviceChangedEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyEventFlags(u32);
impl KeyEventFlags {
    pub const KEY_REPEAT: Self = Self(1 << 16);
}
impl From<KeyEventFlags> for u32 {
    #[inline]
    fn from(input: KeyEventFlags) -> Self {
        input.0
    }
}
impl From<KeyEventFlags> for Option<u32> {
    #[inline]
    fn from(input: KeyEventFlags) -> Self {
        Some(input.0)
    }
}
impl From<u8> for KeyEventFlags {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for KeyEventFlags {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for KeyEventFlags {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for KeyEventFlags  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::KEY_REPEAT.0, "KEY_REPEAT", "KeyRepeat"),
        ];
        pretty_print_bitmask(fmt, self.0, &variants)
    }
}
bitmask_binop!(KeyEventFlags, u32);
#[cfg(test)]
impl crate::x11_utils::GenRandom for KeyEventFlags {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::KEY_REPEAT.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the KeyPress event
pub const KEY_PRESS_EVENT: u16 = 2;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for KeyPressEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len.try_to_usize()?)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len.try_to_usize()?)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = KeyPressEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for KeyPressEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(80);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.detail.serialize_into(bytes);
        self.root.serialize_into(bytes);
        self.event.serialize_into(bytes);
        self.child.serialize_into(bytes);
        self.root_x.serialize_into(bytes);
        self.root_y.serialize_into(bytes);
        self.event_x.serialize_into(bytes);
        self.event_y.serialize_into(bytes);
        let buttons_len = u16::try_from(self.button_mask.len()).expect("`button_mask` has too many elements");
        buttons_len.serialize_into(bytes);
        let valuators_len = u16::try_from(self.valuator_mask.len()).expect("`valuator_mask` has too many elements");
        valuators_len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
        self.mods.serialize_into(bytes);
        self.group.serialize_into(bytes);
        self.button_mask.serialize_into(bytes);
        self.valuator_mask.serialize_into(bytes);
        assert_eq!(self.axisvalues.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues` has an incorrect length");
        self.axisvalues.serialize_into(bytes);
    }
}
impl KeyPressEvent {
    /// Get the value of the `buttons_len` field.
    ///
    /// The `buttons_len` field is used as the length field of the `button_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn buttons_len(&self) -> u16 {
        self.button_mask.len()
            .try_into().unwrap()
    }
    /// Get the value of the `valuators_len` field.
    ///
    /// The `valuators_len` field is used as the length field of the `valuator_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn valuators_len(&self) -> u16 {
        self.valuator_mask.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod key_press_event {
    use super::KeyPressEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for KeyPressEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                root: GenRandom::generate(rng),
                event: GenRandom::generate(rng),
                child: GenRandom::generate(rng),
                root_x: GenRandom::generate(rng),
                root_y: GenRandom::generate(rng),
                event_x: GenRandom::generate(rng),
                event_y: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                mods: GenRandom::generate(rng),
                group: GenRandom::generate(rng),
                button_mask: GenRandom::generate(rng),
                valuator_mask: GenRandom::generate(rng),
                axisvalues: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3605300988613783040);
        let value = KeyPressEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the KeyRelease event
pub const KEY_RELEASE_EVENT: u16 = 3;
pub type KeyReleaseEvent = KeyPressEvent;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerEventFlags(u32);
impl PointerEventFlags {
    pub const POINTER_EMULATED: Self = Self(1 << 16);
}
impl From<PointerEventFlags> for u32 {
    #[inline]
    fn from(input: PointerEventFlags) -> Self {
        input.0
    }
}
impl From<PointerEventFlags> for Option<u32> {
    #[inline]
    fn from(input: PointerEventFlags) -> Self {
        Some(input.0)
    }
}
impl From<u8> for PointerEventFlags {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for PointerEventFlags {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for PointerEventFlags {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for PointerEventFlags  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::POINTER_EMULATED.0, "POINTER_EMULATED", "PointerEmulated"),
        ];
        pretty_print_bitmask(fmt, self.0, &variants)
    }
}
bitmask_binop!(PointerEventFlags, u32);
#[cfg(test)]
impl crate::x11_utils::GenRandom for PointerEventFlags {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::POINTER_EMULATED.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the ButtonPress event
pub const BUTTON_PRESS_EVENT: u16 = 4;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for ButtonPressEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len.try_to_usize()?)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len.try_to_usize()?)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = ButtonPressEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ButtonPressEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(80);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.detail.serialize_into(bytes);
        self.root.serialize_into(bytes);
        self.event.serialize_into(bytes);
        self.child.serialize_into(bytes);
        self.root_x.serialize_into(bytes);
        self.root_y.serialize_into(bytes);
        self.event_x.serialize_into(bytes);
        self.event_y.serialize_into(bytes);
        let buttons_len = u16::try_from(self.button_mask.len()).expect("`button_mask` has too many elements");
        buttons_len.serialize_into(bytes);
        let valuators_len = u16::try_from(self.valuator_mask.len()).expect("`valuator_mask` has too many elements");
        valuators_len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
        self.mods.serialize_into(bytes);
        self.group.serialize_into(bytes);
        self.button_mask.serialize_into(bytes);
        self.valuator_mask.serialize_into(bytes);
        assert_eq!(self.axisvalues.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues` has an incorrect length");
        self.axisvalues.serialize_into(bytes);
    }
}
impl ButtonPressEvent {
    /// Get the value of the `buttons_len` field.
    ///
    /// The `buttons_len` field is used as the length field of the `button_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn buttons_len(&self) -> u16 {
        self.button_mask.len()
            .try_into().unwrap()
    }
    /// Get the value of the `valuators_len` field.
    ///
    /// The `valuators_len` field is used as the length field of the `valuator_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn valuators_len(&self) -> u16 {
        self.valuator_mask.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod button_press_event {
    use super::ButtonPressEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for ButtonPressEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                root: GenRandom::generate(rng),
                event: GenRandom::generate(rng),
                child: GenRandom::generate(rng),
                root_x: GenRandom::generate(rng),
                root_y: GenRandom::generate(rng),
                event_x: GenRandom::generate(rng),
                event_y: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                mods: GenRandom::generate(rng),
                group: GenRandom::generate(rng),
                button_mask: GenRandom::generate(rng),
                valuator_mask: GenRandom::generate(rng),
                axisvalues: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13647151740139503616);
        let value = ButtonPressEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ButtonRelease event
pub const BUTTON_RELEASE_EVENT: u16 = 5;
pub type ButtonReleaseEvent = ButtonPressEvent;

/// Opcode for the Motion event
pub const MOTION_EVENT: u16 = 6;
pub type MotionEvent = ButtonPressEvent;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotifyMode(u8);
impl NotifyMode {
    pub const NORMAL: Self = Self(0);
    pub const GRAB: Self = Self(1);
    pub const UNGRAB: Self = Self(2);
    pub const WHILE_GRABBED: Self = Self(3);
    pub const PASSIVE_GRAB: Self = Self(4);
    pub const PASSIVE_UNGRAB: Self = Self(5);
}
impl From<NotifyMode> for u8 {
    #[inline]
    fn from(input: NotifyMode) -> Self {
        input.0
    }
}
impl From<NotifyMode> for Option<u8> {
    #[inline]
    fn from(input: NotifyMode) -> Self {
        Some(input.0)
    }
}
impl From<NotifyMode> for u16 {
    #[inline]
    fn from(input: NotifyMode) -> Self {
        u16::from(input.0)
    }
}
impl From<NotifyMode> for Option<u16> {
    #[inline]
    fn from(input: NotifyMode) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<NotifyMode> for u32 {
    #[inline]
    fn from(input: NotifyMode) -> Self {
        u32::from(input.0)
    }
}
impl From<NotifyMode> for Option<u32> {
    #[inline]
    fn from(input: NotifyMode) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for NotifyMode {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for NotifyMode  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::NORMAL.0.into(), "NORMAL", "Normal"),
            (Self::GRAB.0.into(), "GRAB", "Grab"),
            (Self::UNGRAB.0.into(), "UNGRAB", "Ungrab"),
            (Self::WHILE_GRABBED.0.into(), "WHILE_GRABBED", "WhileGrabbed"),
            (Self::PASSIVE_GRAB.0.into(), "PASSIVE_GRAB", "PassiveGrab"),
            (Self::PASSIVE_UNGRAB.0.into(), "PASSIVE_UNGRAB", "PassiveUngrab"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for NotifyMode {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::NORMAL.0,
            Self::GRAB.0,
            Self::UNGRAB.0,
            Self::WHILE_GRABBED.0,
            Self::PASSIVE_GRAB.0,
            Self::PASSIVE_UNGRAB.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotifyDetail(u8);
impl NotifyDetail {
    pub const ANCESTOR: Self = Self(0);
    pub const VIRTUAL: Self = Self(1);
    pub const INFERIOR: Self = Self(2);
    pub const NONLINEAR: Self = Self(3);
    pub const NONLINEAR_VIRTUAL: Self = Self(4);
    pub const POINTER: Self = Self(5);
    pub const POINTER_ROOT: Self = Self(6);
    pub const NONE: Self = Self(7);
}
impl From<NotifyDetail> for u8 {
    #[inline]
    fn from(input: NotifyDetail) -> Self {
        input.0
    }
}
impl From<NotifyDetail> for Option<u8> {
    #[inline]
    fn from(input: NotifyDetail) -> Self {
        Some(input.0)
    }
}
impl From<NotifyDetail> for u16 {
    #[inline]
    fn from(input: NotifyDetail) -> Self {
        u16::from(input.0)
    }
}
impl From<NotifyDetail> for Option<u16> {
    #[inline]
    fn from(input: NotifyDetail) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<NotifyDetail> for u32 {
    #[inline]
    fn from(input: NotifyDetail) -> Self {
        u32::from(input.0)
    }
}
impl From<NotifyDetail> for Option<u32> {
    #[inline]
    fn from(input: NotifyDetail) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for NotifyDetail {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for NotifyDetail  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::ANCESTOR.0.into(), "ANCESTOR", "Ancestor"),
            (Self::VIRTUAL.0.into(), "VIRTUAL", "Virtual"),
            (Self::INFERIOR.0.into(), "INFERIOR", "Inferior"),
            (Self::NONLINEAR.0.into(), "NONLINEAR", "Nonlinear"),
            (Self::NONLINEAR_VIRTUAL.0.into(), "NONLINEAR_VIRTUAL", "NonlinearVirtual"),
            (Self::POINTER.0.into(), "POINTER", "Pointer"),
            (Self::POINTER_ROOT.0.into(), "POINTER_ROOT", "PointerRoot"),
            (Self::NONE.0.into(), "NONE", "None"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for NotifyDetail {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::ANCESTOR.0,
            Self::VIRTUAL.0,
            Self::INFERIOR.0,
            Self::NONLINEAR.0,
            Self::NONLINEAR_VIRTUAL.0,
            Self::POINTER.0,
            Self::POINTER_ROOT.0,
            Self::NONE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the Enter event
pub const ENTER_EVENT: u16 = 7;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for EnterEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let (buttons, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len.try_to_usize()?)?;
        let mode = mode.into();
        let detail = detail.into();
        let result = EnterEvent { response_type, extension, sequence, length, event_type, deviceid, time, sourceid, mode, detail, root, event, child, root_x, root_y, event_x, event_y, same_screen, focus, mods, group, buttons };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for EnterEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(72);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        u8::from(self.mode).serialize_into(bytes);
        u8::from(self.detail).serialize_into(bytes);
        self.root.serialize_into(bytes);
        self.event.serialize_into(bytes);
        self.child.serialize_into(bytes);
        self.root_x.serialize_into(bytes);
        self.root_y.serialize_into(bytes);
        self.event_x.serialize_into(bytes);
        self.event_y.serialize_into(bytes);
        self.same_screen.serialize_into(bytes);
        self.focus.serialize_into(bytes);
        let buttons_len = u16::try_from(self.buttons.len()).expect("`buttons` has too many elements");
        buttons_len.serialize_into(bytes);
        self.mods.serialize_into(bytes);
        self.group.serialize_into(bytes);
        self.buttons.serialize_into(bytes);
    }
}
impl EnterEvent {
    /// Get the value of the `buttons_len` field.
    ///
    /// The `buttons_len` field is used as the length field of the `buttons` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn buttons_len(&self) -> u16 {
        self.buttons.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod enter_event {
    use super::EnterEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for EnterEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                mode: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                root: GenRandom::generate(rng),
                event: GenRandom::generate(rng),
                child: GenRandom::generate(rng),
                root_x: GenRandom::generate(rng),
                root_y: GenRandom::generate(rng),
                event_x: GenRandom::generate(rng),
                event_y: GenRandom::generate(rng),
                same_screen: GenRandom::generate(rng),
                focus: GenRandom::generate(rng),
                mods: GenRandom::generate(rng),
                group: GenRandom::generate(rng),
                buttons: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14138723337778789120);
        let value = EnterEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the Leave event
pub const LEAVE_EVENT: u16 = 8;
pub type LeaveEvent = EnterEvent;

/// Opcode for the FocusIn event
pub const FOCUS_IN_EVENT: u16 = 9;
pub type FocusInEvent = EnterEvent;

/// Opcode for the FocusOut event
pub const FOCUS_OUT_EVENT: u16 = 10;
pub type FocusOutEvent = EnterEvent;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HierarchyMask(u8);
impl HierarchyMask {
    pub const MASTER_ADDED: Self = Self(1 << 0);
    pub const MASTER_REMOVED: Self = Self(1 << 1);
    pub const SLAVE_ADDED: Self = Self(1 << 2);
    pub const SLAVE_REMOVED: Self = Self(1 << 3);
    pub const SLAVE_ATTACHED: Self = Self(1 << 4);
    pub const SLAVE_DETACHED: Self = Self(1 << 5);
    pub const DEVICE_ENABLED: Self = Self(1 << 6);
    pub const DEVICE_DISABLED: Self = Self(1 << 7);
}
impl From<HierarchyMask> for u8 {
    #[inline]
    fn from(input: HierarchyMask) -> Self {
        input.0
    }
}
impl From<HierarchyMask> for Option<u8> {
    #[inline]
    fn from(input: HierarchyMask) -> Self {
        Some(input.0)
    }
}
impl From<HierarchyMask> for u16 {
    #[inline]
    fn from(input: HierarchyMask) -> Self {
        u16::from(input.0)
    }
}
impl From<HierarchyMask> for Option<u16> {
    #[inline]
    fn from(input: HierarchyMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<HierarchyMask> for u32 {
    #[inline]
    fn from(input: HierarchyMask) -> Self {
        u32::from(input.0)
    }
}
impl From<HierarchyMask> for Option<u32> {
    #[inline]
    fn from(input: HierarchyMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for HierarchyMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for HierarchyMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::MASTER_ADDED.0.into(), "MASTER_ADDED", "MasterAdded"),
            (Self::MASTER_REMOVED.0.into(), "MASTER_REMOVED", "MasterRemoved"),
            (Self::SLAVE_ADDED.0.into(), "SLAVE_ADDED", "SlaveAdded"),
            (Self::SLAVE_REMOVED.0.into(), "SLAVE_REMOVED", "SlaveRemoved"),
            (Self::SLAVE_ATTACHED.0.into(), "SLAVE_ATTACHED", "SlaveAttached"),
            (Self::SLAVE_DETACHED.0.into(), "SLAVE_DETACHED", "SlaveDetached"),
            (Self::DEVICE_ENABLED.0.into(), "DEVICE_ENABLED", "DeviceEnabled"),
            (Self::DEVICE_DISABLED.0.into(), "DEVICE_DISABLED", "DeviceDisabled"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(HierarchyMask, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for HierarchyMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::MASTER_ADDED.0,
            Self::MASTER_REMOVED.0,
            Self::SLAVE_ADDED.0,
            Self::SLAVE_REMOVED.0,
            Self::SLAVE_ATTACHED.0,
            Self::SLAVE_DETACHED.0,
            Self::DEVICE_ENABLED.0,
            Self::DEVICE_DISABLED.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let type_ = type_.into();
        let result = HierarchyInfo { deviceid, attachment, type_, enabled, flags };
        Ok((result, remaining))
    }
}
impl Serialize for HierarchyInfo {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let deviceid_bytes = self.deviceid.serialize();
        let attachment_bytes = self.attachment.serialize();
        let type_bytes = (u16::from(self.type_) as u8).serialize();
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
        (u16::from(self.type_) as u8).serialize_into(bytes);
        self.enabled.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
    }
}
#[cfg(test)]
mod hierarchy_info {
    use super::HierarchyInfo;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for HierarchyInfo {
        fn generate(rng: &Rng) -> Self {
            Self {
                deviceid: GenRandom::generate(rng),
                attachment: GenRandom::generate(rng),
                type_: GenRandom::generate(rng),
                enabled: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2280645545909863424);
        let value = HierarchyInfo::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the Hierarchy event
pub const HIERARCHY_EVENT: u16 = 11;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for HierarchyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (num_infos, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(10..).ok_or(ParseError::InsufficientData)?;
        let (infos, remaining) = crate::x11_utils::parse_list::<HierarchyInfo>(remaining, num_infos.try_to_usize()?)?;
        let result = HierarchyEvent { response_type, extension, sequence, length, event_type, deviceid, time, flags, infos };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for HierarchyEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.flags.serialize_into(bytes);
        let num_infos = u16::try_from(self.infos.len()).expect("`infos` has too many elements");
        num_infos.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 10]);
        self.infos.serialize_into(bytes);
    }
}
impl HierarchyEvent {
    /// Get the value of the `num_infos` field.
    ///
    /// The `num_infos` field is used as the length field of the `infos` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_infos(&self) -> u16 {
        self.infos.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod hierarchy_event {
    use super::HierarchyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for HierarchyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                infos: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8139899991617548288);
        let value = HierarchyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PropertyFlag(u8);
impl PropertyFlag {
    pub const DELETED: Self = Self(0);
    pub const CREATED: Self = Self(1);
    pub const MODIFIED: Self = Self(2);
}
impl From<PropertyFlag> for u8 {
    #[inline]
    fn from(input: PropertyFlag) -> Self {
        input.0
    }
}
impl From<PropertyFlag> for Option<u8> {
    #[inline]
    fn from(input: PropertyFlag) -> Self {
        Some(input.0)
    }
}
impl From<PropertyFlag> for u16 {
    #[inline]
    fn from(input: PropertyFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<PropertyFlag> for Option<u16> {
    #[inline]
    fn from(input: PropertyFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<PropertyFlag> for u32 {
    #[inline]
    fn from(input: PropertyFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<PropertyFlag> for Option<u32> {
    #[inline]
    fn from(input: PropertyFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for PropertyFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for PropertyFlag  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::DELETED.0.into(), "DELETED", "Deleted"),
            (Self::CREATED.0.into(), "CREATED", "Created"),
            (Self::MODIFIED.0.into(), "MODIFIED", "Modified"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for PropertyFlag {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::DELETED.0,
            Self::CREATED.0,
            Self::MODIFIED.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the Property event
pub const PROPERTY_EVENT: u16 = 12;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for PropertyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (deviceid, remaining) = DeviceId::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let (what, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::InsufficientData)?;
        let what = what.into();
        let result = PropertyEvent { response_type, extension, sequence, length, event_type, deviceid, time, property, what };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for PropertyEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let extension_bytes = self.extension.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let event_type_bytes = self.event_type.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let time_bytes = self.time.serialize();
        let property_bytes = self.property.serialize();
        let what_bytes = u8::from(self.what).serialize();
        [
            response_type_bytes[0],
            extension_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            event_type_bytes[0],
            event_type_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
            what_bytes[0],
            0,
            0,
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
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.property.serialize_into(bytes);
        u8::from(self.what).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 11]);
    }
}
#[cfg(test)]
mod property_event {
    use super::PropertyEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for PropertyEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                property: GenRandom::generate(rng),
                what: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12000850040323375104);
        let value = PropertyEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the RawKeyPress event
pub const RAW_KEY_PRESS_EVENT: u16 = 13;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for RawKeyPressEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len.try_to_usize()?)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = RawKeyPressEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for RawKeyPressEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.detail.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        let valuators_len = u16::try_from(self.valuator_mask.len()).expect("`valuator_mask` has too many elements");
        valuators_len.serialize_into(bytes);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.valuator_mask.serialize_into(bytes);
        assert_eq!(self.axisvalues.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues` has an incorrect length");
        self.axisvalues.serialize_into(bytes);
        assert_eq!(self.axisvalues_raw.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues_raw` has an incorrect length");
        self.axisvalues_raw.serialize_into(bytes);
    }
}
impl RawKeyPressEvent {
    /// Get the value of the `valuators_len` field.
    ///
    /// The `valuators_len` field is used as the length field of the `valuator_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn valuators_len(&self) -> u16 {
        self.valuator_mask.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod raw_key_press_event {
    use super::RawKeyPressEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for RawKeyPressEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                valuator_mask: GenRandom::generate(rng),
                axisvalues: GenRandom::generate(rng),
                axisvalues_raw: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11043864972233171968);
        let value = RawKeyPressEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the RawKeyRelease event
pub const RAW_KEY_RELEASE_EVENT: u16 = 14;
pub type RawKeyReleaseEvent = RawKeyPressEvent;

/// Opcode for the RawButtonPress event
pub const RAW_BUTTON_PRESS_EVENT: u16 = 15;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for RawButtonPressEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len.try_to_usize()?)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = RawButtonPressEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for RawButtonPressEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.detail.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        let valuators_len = u16::try_from(self.valuator_mask.len()).expect("`valuator_mask` has too many elements");
        valuators_len.serialize_into(bytes);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.valuator_mask.serialize_into(bytes);
        assert_eq!(self.axisvalues.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues` has an incorrect length");
        self.axisvalues.serialize_into(bytes);
        assert_eq!(self.axisvalues_raw.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues_raw` has an incorrect length");
        self.axisvalues_raw.serialize_into(bytes);
    }
}
impl RawButtonPressEvent {
    /// Get the value of the `valuators_len` field.
    ///
    /// The `valuators_len` field is used as the length field of the `valuator_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn valuators_len(&self) -> u16 {
        self.valuator_mask.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod raw_button_press_event {
    use super::RawButtonPressEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for RawButtonPressEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                valuator_mask: GenRandom::generate(rng),
                axisvalues: GenRandom::generate(rng),
                axisvalues_raw: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14516884022861430784);
        let value = RawButtonPressEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the RawButtonRelease event
pub const RAW_BUTTON_RELEASE_EVENT: u16 = 16;
pub type RawButtonReleaseEvent = RawButtonPressEvent;

/// Opcode for the RawMotion event
pub const RAW_MOTION_EVENT: u16 = 17;
pub type RawMotionEvent = RawButtonPressEvent;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TouchEventFlags(u32);
impl TouchEventFlags {
    pub const TOUCH_PENDING_END: Self = Self(1 << 16);
    pub const TOUCH_EMULATING_POINTER: Self = Self(1 << 17);
}
impl From<TouchEventFlags> for u32 {
    #[inline]
    fn from(input: TouchEventFlags) -> Self {
        input.0
    }
}
impl From<TouchEventFlags> for Option<u32> {
    #[inline]
    fn from(input: TouchEventFlags) -> Self {
        Some(input.0)
    }
}
impl From<u8> for TouchEventFlags {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for TouchEventFlags {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for TouchEventFlags {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for TouchEventFlags  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::TOUCH_PENDING_END.0, "TOUCH_PENDING_END", "TouchPendingEnd"),
            (Self::TOUCH_EMULATING_POINTER.0, "TOUCH_EMULATING_POINTER", "TouchEmulatingPointer"),
        ];
        pretty_print_bitmask(fmt, self.0, &variants)
    }
}
bitmask_binop!(TouchEventFlags, u32);
#[cfg(test)]
impl crate::x11_utils::GenRandom for TouchEventFlags {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::TOUCH_PENDING_END.0,
            Self::TOUCH_EMULATING_POINTER.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the TouchBegin event
pub const TOUCH_BEGIN_EVENT: u16 = 18;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for TouchBeginEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = ModifierInfo::try_parse(remaining)?;
        let (group, remaining) = GroupInfo::try_parse(remaining)?;
        let (button_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, buttons_len.try_to_usize()?)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len.try_to_usize()?)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = TouchBeginEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, root, event, child, root_x, root_y, event_x, event_y, sourceid, flags, mods, group, button_mask, valuator_mask, axisvalues };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for TouchBeginEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(80);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.detail.serialize_into(bytes);
        self.root.serialize_into(bytes);
        self.event.serialize_into(bytes);
        self.child.serialize_into(bytes);
        self.root_x.serialize_into(bytes);
        self.root_y.serialize_into(bytes);
        self.event_x.serialize_into(bytes);
        self.event_y.serialize_into(bytes);
        let buttons_len = u16::try_from(self.button_mask.len()).expect("`button_mask` has too many elements");
        buttons_len.serialize_into(bytes);
        let valuators_len = u16::try_from(self.valuator_mask.len()).expect("`valuator_mask` has too many elements");
        valuators_len.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.flags.serialize_into(bytes);
        self.mods.serialize_into(bytes);
        self.group.serialize_into(bytes);
        self.button_mask.serialize_into(bytes);
        self.valuator_mask.serialize_into(bytes);
        assert_eq!(self.axisvalues.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues` has an incorrect length");
        self.axisvalues.serialize_into(bytes);
    }
}
impl TouchBeginEvent {
    /// Get the value of the `buttons_len` field.
    ///
    /// The `buttons_len` field is used as the length field of the `button_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn buttons_len(&self) -> u16 {
        self.button_mask.len()
            .try_into().unwrap()
    }
    /// Get the value of the `valuators_len` field.
    ///
    /// The `valuators_len` field is used as the length field of the `valuator_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn valuators_len(&self) -> u16 {
        self.valuator_mask.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod touch_begin_event {
    use super::TouchBeginEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for TouchBeginEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                root: GenRandom::generate(rng),
                event: GenRandom::generate(rng),
                child: GenRandom::generate(rng),
                root_x: GenRandom::generate(rng),
                root_y: GenRandom::generate(rng),
                event_x: GenRandom::generate(rng),
                event_y: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                mods: GenRandom::generate(rng),
                group: GenRandom::generate(rng),
                button_mask: GenRandom::generate(rng),
                valuator_mask: GenRandom::generate(rng),
                axisvalues: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12845195065756411904);
        let value = TouchBeginEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the TouchUpdate event
pub const TOUCH_UPDATE_EVENT: u16 = 19;
pub type TouchUpdateEvent = TouchBeginEvent;

/// Opcode for the TouchEnd event
pub const TOUCH_END_EVENT: u16 = 20;
pub type TouchEndEvent = TouchBeginEvent;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TouchOwnershipFlags(u32);
impl TouchOwnershipFlags {
    pub const NONE: Self = Self(0);
}
impl From<TouchOwnershipFlags> for u32 {
    #[inline]
    fn from(input: TouchOwnershipFlags) -> Self {
        input.0
    }
}
impl From<TouchOwnershipFlags> for Option<u32> {
    #[inline]
    fn from(input: TouchOwnershipFlags) -> Self {
        Some(input.0)
    }
}
impl From<u8> for TouchOwnershipFlags {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for TouchOwnershipFlags {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for TouchOwnershipFlags {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for TouchOwnershipFlags  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::NONE.0, "NONE", "None"),
        ];
        pretty_print_enum(fmt, self.0, &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for TouchOwnershipFlags {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::NONE.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the TouchOwnership event
pub const TOUCH_OWNERSHIP_EVENT: u16 = 21;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for TouchOwnershipEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let flags = flags.into();
        let result = TouchOwnershipEvent { response_type, extension, sequence, length, event_type, deviceid, time, touchid, root, event, child, sourceid, flags };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for TouchOwnershipEvent {
    type Bytes = [u8; 48];
    fn serialize(&self) -> [u8; 48] {
        let response_type_bytes = self.response_type.serialize();
        let extension_bytes = self.extension.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let event_type_bytes = self.event_type.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let time_bytes = self.time.serialize();
        let touchid_bytes = self.touchid.serialize();
        let root_bytes = self.root.serialize();
        let event_bytes = self.event.serialize();
        let child_bytes = self.child.serialize();
        let sourceid_bytes = self.sourceid.serialize();
        let flags_bytes = u32::from(self.flags).serialize();
        [
            response_type_bytes[0],
            extension_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            event_type_bytes[0],
            event_type_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            touchid_bytes[0],
            touchid_bytes[1],
            touchid_bytes[2],
            touchid_bytes[3],
            root_bytes[0],
            root_bytes[1],
            root_bytes[2],
            root_bytes[3],
            event_bytes[0],
            event_bytes[1],
            event_bytes[2],
            event_bytes[3],
            child_bytes[0],
            child_bytes[1],
            child_bytes[2],
            child_bytes[3],
            sourceid_bytes[0],
            sourceid_bytes[1],
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
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(48);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.touchid.serialize_into(bytes);
        self.root.serialize_into(bytes);
        self.event.serialize_into(bytes);
        self.child.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        u32::from(self.flags).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
    }
}
#[cfg(test)]
mod touch_ownership_event {
    use super::TouchOwnershipEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for TouchOwnershipEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                touchid: GenRandom::generate(rng),
                root: GenRandom::generate(rng),
                event: GenRandom::generate(rng),
                child: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17925002372647223296);
        let value = TouchOwnershipEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the RawTouchBegin event
pub const RAW_TOUCH_BEGIN_EVENT: u16 = 22;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for RawTouchBeginEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (valuator_mask, remaining) = crate::x11_utils::parse_list::<u32>(remaining, valuators_len.try_to_usize()?)?;
        let (axisvalues, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let (axisvalues_raw, remaining) = crate::x11_utils::parse_list::<Fp3232>(remaining, valuator_mask.iter().try_fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = RawTouchBeginEvent { response_type, extension, sequence, length, event_type, deviceid, time, detail, sourceid, flags, valuator_mask, axisvalues, axisvalues_raw };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for RawTouchBeginEvent {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.detail.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        let valuators_len = u16::try_from(self.valuator_mask.len()).expect("`valuator_mask` has too many elements");
        valuators_len.serialize_into(bytes);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.valuator_mask.serialize_into(bytes);
        assert_eq!(self.axisvalues.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues` has an incorrect length");
        self.axisvalues.serialize_into(bytes);
        assert_eq!(self.axisvalues_raw.len(), usize::try_from(self.valuator_mask.iter().fold(0u32, |acc, x| acc.checked_add((*x).count_ones()).unwrap())).unwrap(), "`axisvalues_raw` has an incorrect length");
        self.axisvalues_raw.serialize_into(bytes);
    }
}
impl RawTouchBeginEvent {
    /// Get the value of the `valuators_len` field.
    ///
    /// The `valuators_len` field is used as the length field of the `valuator_mask` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn valuators_len(&self) -> u16 {
        self.valuator_mask.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod raw_touch_begin_event {
    use super::RawTouchBeginEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for RawTouchBeginEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                detail: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                valuator_mask: GenRandom::generate(rng),
                axisvalues: GenRandom::generate(rng),
                axisvalues_raw: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6745595966935085056);
        let value = RawTouchBeginEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the RawTouchUpdate event
pub const RAW_TOUCH_UPDATE_EVENT: u16 = 23;
pub type RawTouchUpdateEvent = RawTouchBeginEvent;

/// Opcode for the RawTouchEnd event
pub const RAW_TOUCH_END_EVENT: u16 = 24;
pub type RawTouchEndEvent = RawTouchBeginEvent;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarrierFlags(u8);
impl BarrierFlags {
    pub const POINTER_RELEASED: Self = Self(1 << 0);
    pub const DEVICE_IS_GRABBED: Self = Self(1 << 1);
}
impl From<BarrierFlags> for u8 {
    #[inline]
    fn from(input: BarrierFlags) -> Self {
        input.0
    }
}
impl From<BarrierFlags> for Option<u8> {
    #[inline]
    fn from(input: BarrierFlags) -> Self {
        Some(input.0)
    }
}
impl From<BarrierFlags> for u16 {
    #[inline]
    fn from(input: BarrierFlags) -> Self {
        u16::from(input.0)
    }
}
impl From<BarrierFlags> for Option<u16> {
    #[inline]
    fn from(input: BarrierFlags) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<BarrierFlags> for u32 {
    #[inline]
    fn from(input: BarrierFlags) -> Self {
        u32::from(input.0)
    }
}
impl From<BarrierFlags> for Option<u32> {
    #[inline]
    fn from(input: BarrierFlags) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for BarrierFlags {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for BarrierFlags  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::POINTER_RELEASED.0.into(), "POINTER_RELEASED", "PointerReleased"),
            (Self::DEVICE_IS_GRABBED.0.into(), "DEVICE_IS_GRABBED", "DeviceIsGrabbed"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(BarrierFlags, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for BarrierFlags {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::POINTER_RELEASED.0,
            Self::DEVICE_IS_GRABBED.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the BarrierHit event
pub const BARRIER_HIT_EVENT: u16 = 25;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for BarrierHitEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (root_x, remaining) = Fp1616::try_parse(remaining)?;
        let (root_y, remaining) = Fp1616::try_parse(remaining)?;
        let (dx, remaining) = Fp3232::try_parse(remaining)?;
        let (dy, remaining) = Fp3232::try_parse(remaining)?;
        let result = BarrierHitEvent { response_type, extension, sequence, length, event_type, deviceid, time, eventid, root, event, barrier, dtime, flags, sourceid, root_x, root_y, dx, dy };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for BarrierHitEvent {
    type Bytes = [u8; 68];
    fn serialize(&self) -> [u8; 68] {
        let response_type_bytes = self.response_type.serialize();
        let extension_bytes = self.extension.serialize();
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let event_type_bytes = self.event_type.serialize();
        let deviceid_bytes = self.deviceid.serialize();
        let time_bytes = self.time.serialize();
        let eventid_bytes = self.eventid.serialize();
        let root_bytes = self.root.serialize();
        let event_bytes = self.event.serialize();
        let barrier_bytes = self.barrier.serialize();
        let dtime_bytes = self.dtime.serialize();
        let flags_bytes = self.flags.serialize();
        let sourceid_bytes = self.sourceid.serialize();
        let root_x_bytes = self.root_x.serialize();
        let root_y_bytes = self.root_y.serialize();
        let dx_bytes = self.dx.serialize();
        let dy_bytes = self.dy.serialize();
        [
            response_type_bytes[0],
            extension_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            event_type_bytes[0],
            event_type_bytes[1],
            deviceid_bytes[0],
            deviceid_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            eventid_bytes[0],
            eventid_bytes[1],
            eventid_bytes[2],
            eventid_bytes[3],
            root_bytes[0],
            root_bytes[1],
            root_bytes[2],
            root_bytes[3],
            event_bytes[0],
            event_bytes[1],
            event_bytes[2],
            event_bytes[3],
            barrier_bytes[0],
            barrier_bytes[1],
            barrier_bytes[2],
            barrier_bytes[3],
            dtime_bytes[0],
            dtime_bytes[1],
            dtime_bytes[2],
            dtime_bytes[3],
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            sourceid_bytes[0],
            sourceid_bytes[1],
            0,
            0,
            root_x_bytes[0],
            root_x_bytes[1],
            root_x_bytes[2],
            root_x_bytes[3],
            root_y_bytes[0],
            root_y_bytes[1],
            root_y_bytes[2],
            root_y_bytes[3],
            dx_bytes[0],
            dx_bytes[1],
            dx_bytes[2],
            dx_bytes[3],
            dx_bytes[4],
            dx_bytes[5],
            dx_bytes[6],
            dx_bytes[7],
            dy_bytes[0],
            dy_bytes[1],
            dy_bytes[2],
            dy_bytes[3],
            dy_bytes[4],
            dy_bytes[5],
            dy_bytes[6],
            dy_bytes[7],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(68);
        self.response_type.serialize_into(bytes);
        self.extension.serialize_into(bytes);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.deviceid.serialize_into(bytes);
        self.time.serialize_into(bytes);
        self.eventid.serialize_into(bytes);
        self.root.serialize_into(bytes);
        self.event.serialize_into(bytes);
        self.barrier.serialize_into(bytes);
        self.dtime.serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.sourceid.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.root_x.serialize_into(bytes);
        self.root_y.serialize_into(bytes);
        self.dx.serialize_into(bytes);
        self.dy.serialize_into(bytes);
    }
}
#[cfg(test)]
mod barrier_hit_event {
    use super::BarrierHitEvent;
    #[allow(unused_imports)]
    use crate::x11_utils::{GenRandom, Serialize};
    use fastrand::Rng;
    impl GenRandom for BarrierHitEvent {
        fn generate(rng: &Rng) -> Self {
            Self {
                response_type: GenRandom::generate(rng),
                extension: GenRandom::generate(rng),
                sequence: GenRandom::generate(rng),
                length: GenRandom::generate(rng),
                event_type: GenRandom::generate(rng),
                deviceid: GenRandom::generate(rng),
                time: GenRandom::generate(rng),
                eventid: GenRandom::generate(rng),
                root: GenRandom::generate(rng),
                event: GenRandom::generate(rng),
                barrier: GenRandom::generate(rng),
                dtime: GenRandom::generate(rng),
                flags: GenRandom::generate(rng),
                sourceid: GenRandom::generate(rng),
                root_x: GenRandom::generate(rng),
                root_y: GenRandom::generate(rng),
                dx: GenRandom::generate(rng),
                dy: GenRandom::generate(rng),
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11876715904131145728);
        let value = BarrierHitEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the BarrierLeave event
pub const BARRIER_LEAVE_EVENT: u16 = 26;
pub type BarrierLeaveEvent = BarrierHitEvent;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventForSend([u8; 32]);
impl EventForSend {
    pub fn as_device_valuator_event(&self) -> DeviceValuatorEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceValuatorEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_key_press_event(&self) -> DeviceKeyPressEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceKeyPressEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_key_release_event(&self) -> DeviceKeyReleaseEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceKeyReleaseEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_button_press_event(&self) -> DeviceButtonPressEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceButtonPressEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_button_release_event(&self) -> DeviceButtonReleaseEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceButtonReleaseEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_motion_notify_event(&self) -> DeviceMotionNotifyEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceMotionNotifyEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_focus_in_event(&self) -> DeviceFocusInEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceFocusInEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_focus_out_event(&self) -> DeviceFocusOutEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceFocusOutEvent::try_parse(value).unwrap().0
    }
    pub fn as_proximity_in_event(&self) -> ProximityInEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        ProximityInEvent::try_parse(value).unwrap().0
    }
    pub fn as_proximity_out_event(&self) -> ProximityOutEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        ProximityOutEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_state_notify_event(&self) -> DeviceStateNotifyEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceStateNotifyEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_mapping_notify_event(&self) -> DeviceMappingNotifyEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceMappingNotifyEvent::try_parse(value).unwrap().0
    }
    pub fn as_change_device_notify_event(&self) -> ChangeDeviceNotifyEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        ChangeDeviceNotifyEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_key_state_notify_event(&self) -> DeviceKeyStateNotifyEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceKeyStateNotifyEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_button_state_notify_event(&self) -> DeviceButtonStateNotifyEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DeviceButtonStateNotifyEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_presence_notify_event(&self) -> DevicePresenceNotifyEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DevicePresenceNotifyEvent::try_parse(value).unwrap().0
    }
    pub fn as_device_property_notify_event(&self) -> DevicePropertyNotifyEvent {
        let value: &[u8] = &self.0;
        // FIXME: event parsing can fail
        DevicePropertyNotifyEvent::try_parse(value).unwrap().0
    }
}
impl Serialize for EventForSend {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        self.0
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}
impl TryParse for EventForSend {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner: [u8; 32] = value.get(..32)
            .ok_or(ParseError::InsufficientData)?
            .try_into()
            .unwrap();
        let result = EventForSend(inner);
        Ok((result, &value[32..]))
    }
}
impl From<DeviceValuatorEvent> for EventForSend {
    fn from(value: DeviceValuatorEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DeviceValuatorEvent> for EventForSend {
    fn from(value: &DeviceValuatorEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<DeviceKeyPressEvent> for EventForSend {
    fn from(value: DeviceKeyPressEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DeviceKeyPressEvent> for EventForSend {
    fn from(value: &DeviceKeyPressEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<DeviceFocusInEvent> for EventForSend {
    fn from(value: DeviceFocusInEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DeviceFocusInEvent> for EventForSend {
    fn from(value: &DeviceFocusInEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<DeviceStateNotifyEvent> for EventForSend {
    fn from(value: DeviceStateNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DeviceStateNotifyEvent> for EventForSend {
    fn from(value: &DeviceStateNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<DeviceMappingNotifyEvent> for EventForSend {
    fn from(value: DeviceMappingNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DeviceMappingNotifyEvent> for EventForSend {
    fn from(value: &DeviceMappingNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<ChangeDeviceNotifyEvent> for EventForSend {
    fn from(value: ChangeDeviceNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&ChangeDeviceNotifyEvent> for EventForSend {
    fn from(value: &ChangeDeviceNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<DeviceKeyStateNotifyEvent> for EventForSend {
    fn from(value: DeviceKeyStateNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DeviceKeyStateNotifyEvent> for EventForSend {
    fn from(value: &DeviceKeyStateNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<DeviceButtonStateNotifyEvent> for EventForSend {
    fn from(value: DeviceButtonStateNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DeviceButtonStateNotifyEvent> for EventForSend {
    fn from(value: &DeviceButtonStateNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<DevicePresenceNotifyEvent> for EventForSend {
    fn from(value: DevicePresenceNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DevicePresenceNotifyEvent> for EventForSend {
    fn from(value: &DevicePresenceNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<DevicePropertyNotifyEvent> for EventForSend {
    fn from(value: DevicePropertyNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}
impl From<&DevicePropertyNotifyEvent> for EventForSend {
    fn from(value: &DevicePropertyNotifyEvent) -> Self {
        Self(<[u8; 32]>::from(value))
    }
}

/// Opcode for the SendExtensionEvent request
pub const SEND_EXTENSION_EVENT_REQUEST: u8 = 31;
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SendExtensionEventRequest<'input> {
    pub destination: xproto::Window,
    pub device_id: u8,
    pub propagate: bool,
    pub events: Cow<'input, [EventForSend]>,
    pub classes: Cow<'input, [EventClass]>,
}
impl<'input> SendExtensionEventRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let destination_bytes = self.destination.serialize();
        let device_id_bytes = self.device_id.serialize();
        let propagate_bytes = self.propagate.serialize();
        let num_classes = u16::try_from(self.classes.len()).expect("`classes` has too many elements");
        let num_classes_bytes = num_classes.serialize();
        let num_events = u8::try_from(self.events.len()).expect("`events` has too many elements");
        let num_events_bytes = num_events.serialize();
        let mut request0 = vec![
            major_opcode,
            SEND_EXTENSION_EVENT_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let events_bytes = self.events.serialize();
        let length_so_far = length_so_far + events_bytes.len();
        let classes_bytes = self.classes.serialize();
        let length_so_far = length_so_far + classes_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), events_bytes.into(), classes_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SEND_EXTENSION_EVENT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (destination, remaining) = xproto::Window::try_parse(value)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (propagate, remaining) = bool::try_parse(remaining)?;
        let (num_classes, remaining) = u16::try_parse(remaining)?;
        let (num_events, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (events, remaining) = crate::x11_utils::parse_list::<EventForSend>(remaining, num_events.try_to_usize()?)?;
        let (classes, remaining) = crate::x11_utils::parse_list::<EventClass>(remaining, num_classes.try_to_usize()?)?;
        let _ = remaining;
        Ok(SendExtensionEventRequest {
            destination,
            device_id,
            propagate,
            events: Cow::Owned(events),
            classes: Cow::Owned(classes),
        })
    }
    /// Clone all borrowed data in this SendExtensionEventRequest.
    pub fn into_owned(self) -> SendExtensionEventRequest<'static> {
        SendExtensionEventRequest {
            destination: self.destination,
            device_id: self.device_id,
            propagate: self.propagate,
            events: Cow::Owned(self.events.into_owned()),
            classes: Cow::Owned(self.classes.into_owned()),
        }
    }
}
impl<'input> Request for SendExtensionEventRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SendExtensionEventRequest<'input> {
}

/// Opcode for the Device error
pub const DEVICE_ERROR: u8 = 0;

/// Opcode for the Event error
pub const EVENT_ERROR: u8 = 1;

/// Opcode for the Mode error
pub const MODE_ERROR: u8 = 2;

/// Opcode for the DeviceBusy error
pub const DEVICE_BUSY_ERROR: u8 = 3;

/// Opcode for the Class error
pub const CLASS_ERROR: u8 = 4;

