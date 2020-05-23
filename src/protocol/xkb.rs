// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `xkb` X11 extension.

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
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XKEYBOARD";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Const {
    MaxLegalKeyCode = 255,
    PerKeyBitArraySize = 32,
    KeyNameLength = 4,
}
impl From<Const> for u8 {
    fn from(input: Const) -> Self {
        match input {
            Const::MaxLegalKeyCode => 255,
            Const::PerKeyBitArraySize => 32,
            Const::KeyNameLength => 4,
        }
    }
}
impl From<Const> for Option<u8> {
    fn from(input: Const) -> Self {
        Some(u8::from(input))
    }
}
impl From<Const> for u16 {
    fn from(input: Const) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Const> for Option<u16> {
    fn from(input: Const) -> Self {
        Some(u16::from(input))
    }
}
impl From<Const> for u32 {
    fn from(input: Const) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Const> for Option<u32> {
    fn from(input: Const) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Const {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            255 => Ok(Const::MaxLegalKeyCode),
            32 => Ok(Const::PerKeyBitArraySize),
            4 => Ok(Const::KeyNameLength),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Const {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Const {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EventType {
    NewKeyboardNotify = 1 << 0,
    MapNotify = 1 << 1,
    StateNotify = 1 << 2,
    ControlsNotify = 1 << 3,
    IndicatorStateNotify = 1 << 4,
    IndicatorMapNotify = 1 << 5,
    NamesNotify = 1 << 6,
    CompatMapNotify = 1 << 7,
    BellNotify = 1 << 8,
    ActionMessage = 1 << 9,
    AccessXNotify = 1 << 10,
    ExtensionDeviceNotify = 1 << 11,
}
impl From<EventType> for u16 {
    fn from(input: EventType) -> Self {
        match input {
            EventType::NewKeyboardNotify => 1 << 0,
            EventType::MapNotify => 1 << 1,
            EventType::StateNotify => 1 << 2,
            EventType::ControlsNotify => 1 << 3,
            EventType::IndicatorStateNotify => 1 << 4,
            EventType::IndicatorMapNotify => 1 << 5,
            EventType::NamesNotify => 1 << 6,
            EventType::CompatMapNotify => 1 << 7,
            EventType::BellNotify => 1 << 8,
            EventType::ActionMessage => 1 << 9,
            EventType::AccessXNotify => 1 << 10,
            EventType::ExtensionDeviceNotify => 1 << 11,
        }
    }
}
impl From<EventType> for Option<u16> {
    fn from(input: EventType) -> Self {
        Some(u16::from(input))
    }
}
impl From<EventType> for u32 {
    fn from(input: EventType) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<EventType> for Option<u32> {
    fn from(input: EventType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for EventType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EventType::NewKeyboardNotify),
            2 => Ok(EventType::MapNotify),
            4 => Ok(EventType::StateNotify),
            8 => Ok(EventType::ControlsNotify),
            16 => Ok(EventType::IndicatorStateNotify),
            32 => Ok(EventType::IndicatorMapNotify),
            64 => Ok(EventType::NamesNotify),
            128 => Ok(EventType::CompatMapNotify),
            256 => Ok(EventType::BellNotify),
            512 => Ok(EventType::ActionMessage),
            1024 => Ok(EventType::AccessXNotify),
            2048 => Ok(EventType::ExtensionDeviceNotify),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for EventType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(EventType, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NKNDetail {
    Keycodes = 1 << 0,
    Geometry = 1 << 1,
    DeviceID = 1 << 2,
}
impl From<NKNDetail> for u8 {
    fn from(input: NKNDetail) -> Self {
        match input {
            NKNDetail::Keycodes => 1 << 0,
            NKNDetail::Geometry => 1 << 1,
            NKNDetail::DeviceID => 1 << 2,
        }
    }
}
impl From<NKNDetail> for Option<u8> {
    fn from(input: NKNDetail) -> Self {
        Some(u8::from(input))
    }
}
impl From<NKNDetail> for u16 {
    fn from(input: NKNDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NKNDetail> for Option<u16> {
    fn from(input: NKNDetail) -> Self {
        Some(u16::from(input))
    }
}
impl From<NKNDetail> for u32 {
    fn from(input: NKNDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NKNDetail> for Option<u32> {
    fn from(input: NKNDetail) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for NKNDetail {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NKNDetail::Keycodes),
            2 => Ok(NKNDetail::Geometry),
            4 => Ok(NKNDetail::DeviceID),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for NKNDetail {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for NKNDetail {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(NKNDetail, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AXNDetail {
    SKPress = 1 << 0,
    SKAccept = 1 << 1,
    SKReject = 1 << 2,
    SKRelease = 1 << 3,
    BKAccept = 1 << 4,
    BKReject = 1 << 5,
    AXKWarning = 1 << 6,
}
impl From<AXNDetail> for u8 {
    fn from(input: AXNDetail) -> Self {
        match input {
            AXNDetail::SKPress => 1 << 0,
            AXNDetail::SKAccept => 1 << 1,
            AXNDetail::SKReject => 1 << 2,
            AXNDetail::SKRelease => 1 << 3,
            AXNDetail::BKAccept => 1 << 4,
            AXNDetail::BKReject => 1 << 5,
            AXNDetail::AXKWarning => 1 << 6,
        }
    }
}
impl From<AXNDetail> for Option<u8> {
    fn from(input: AXNDetail) -> Self {
        Some(u8::from(input))
    }
}
impl From<AXNDetail> for u16 {
    fn from(input: AXNDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AXNDetail> for Option<u16> {
    fn from(input: AXNDetail) -> Self {
        Some(u16::from(input))
    }
}
impl From<AXNDetail> for u32 {
    fn from(input: AXNDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AXNDetail> for Option<u32> {
    fn from(input: AXNDetail) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for AXNDetail {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AXNDetail::SKPress),
            2 => Ok(AXNDetail::SKAccept),
            4 => Ok(AXNDetail::SKReject),
            8 => Ok(AXNDetail::SKRelease),
            16 => Ok(AXNDetail::BKAccept),
            32 => Ok(AXNDetail::BKReject),
            64 => Ok(AXNDetail::AXKWarning),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for AXNDetail {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for AXNDetail {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(AXNDetail, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MapPart {
    KeyTypes = 1 << 0,
    KeySyms = 1 << 1,
    ModifierMap = 1 << 2,
    ExplicitComponents = 1 << 3,
    KeyActions = 1 << 4,
    KeyBehaviors = 1 << 5,
    VirtualMods = 1 << 6,
    VirtualModMap = 1 << 7,
}
impl From<MapPart> for u8 {
    fn from(input: MapPart) -> Self {
        match input {
            MapPart::KeyTypes => 1 << 0,
            MapPart::KeySyms => 1 << 1,
            MapPart::ModifierMap => 1 << 2,
            MapPart::ExplicitComponents => 1 << 3,
            MapPart::KeyActions => 1 << 4,
            MapPart::KeyBehaviors => 1 << 5,
            MapPart::VirtualMods => 1 << 6,
            MapPart::VirtualModMap => 1 << 7,
        }
    }
}
impl From<MapPart> for Option<u8> {
    fn from(input: MapPart) -> Self {
        Some(u8::from(input))
    }
}
impl From<MapPart> for u16 {
    fn from(input: MapPart) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MapPart> for Option<u16> {
    fn from(input: MapPart) -> Self {
        Some(u16::from(input))
    }
}
impl From<MapPart> for u32 {
    fn from(input: MapPart) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MapPart> for Option<u32> {
    fn from(input: MapPart) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for MapPart {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MapPart::KeyTypes),
            2 => Ok(MapPart::KeySyms),
            4 => Ok(MapPart::ModifierMap),
            8 => Ok(MapPart::ExplicitComponents),
            16 => Ok(MapPart::KeyActions),
            32 => Ok(MapPart::KeyBehaviors),
            64 => Ok(MapPart::VirtualMods),
            128 => Ok(MapPart::VirtualModMap),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for MapPart {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for MapPart {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(MapPart, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SetMapFlags {
    ResizeTypes = 1 << 0,
    RecomputeActions = 1 << 1,
}
impl From<SetMapFlags> for u8 {
    fn from(input: SetMapFlags) -> Self {
        match input {
            SetMapFlags::ResizeTypes => 1 << 0,
            SetMapFlags::RecomputeActions => 1 << 1,
        }
    }
}
impl From<SetMapFlags> for Option<u8> {
    fn from(input: SetMapFlags) -> Self {
        Some(u8::from(input))
    }
}
impl From<SetMapFlags> for u16 {
    fn from(input: SetMapFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetMapFlags> for Option<u16> {
    fn from(input: SetMapFlags) -> Self {
        Some(u16::from(input))
    }
}
impl From<SetMapFlags> for u32 {
    fn from(input: SetMapFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetMapFlags> for Option<u32> {
    fn from(input: SetMapFlags) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SetMapFlags {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SetMapFlags::ResizeTypes),
            2 => Ok(SetMapFlags::RecomputeActions),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SetMapFlags {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SetMapFlags {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(SetMapFlags, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum StatePart {
    ModifierState = 1 << 0,
    ModifierBase = 1 << 1,
    ModifierLatch = 1 << 2,
    ModifierLock = 1 << 3,
    GroupState = 1 << 4,
    GroupBase = 1 << 5,
    GroupLatch = 1 << 6,
    GroupLock = 1 << 7,
    CompatState = 1 << 8,
    GrabMods = 1 << 9,
    CompatGrabMods = 1 << 10,
    LookupMods = 1 << 11,
    CompatLookupMods = 1 << 12,
    PointerButtons = 1 << 13,
}
impl From<StatePart> for u16 {
    fn from(input: StatePart) -> Self {
        match input {
            StatePart::ModifierState => 1 << 0,
            StatePart::ModifierBase => 1 << 1,
            StatePart::ModifierLatch => 1 << 2,
            StatePart::ModifierLock => 1 << 3,
            StatePart::GroupState => 1 << 4,
            StatePart::GroupBase => 1 << 5,
            StatePart::GroupLatch => 1 << 6,
            StatePart::GroupLock => 1 << 7,
            StatePart::CompatState => 1 << 8,
            StatePart::GrabMods => 1 << 9,
            StatePart::CompatGrabMods => 1 << 10,
            StatePart::LookupMods => 1 << 11,
            StatePart::CompatLookupMods => 1 << 12,
            StatePart::PointerButtons => 1 << 13,
        }
    }
}
impl From<StatePart> for Option<u16> {
    fn from(input: StatePart) -> Self {
        Some(u16::from(input))
    }
}
impl From<StatePart> for u32 {
    fn from(input: StatePart) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<StatePart> for Option<u32> {
    fn from(input: StatePart) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for StatePart {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(StatePart::ModifierState),
            2 => Ok(StatePart::ModifierBase),
            4 => Ok(StatePart::ModifierLatch),
            8 => Ok(StatePart::ModifierLock),
            16 => Ok(StatePart::GroupState),
            32 => Ok(StatePart::GroupBase),
            64 => Ok(StatePart::GroupLatch),
            128 => Ok(StatePart::GroupLock),
            256 => Ok(StatePart::CompatState),
            512 => Ok(StatePart::GrabMods),
            1024 => Ok(StatePart::CompatGrabMods),
            2048 => Ok(StatePart::LookupMods),
            4096 => Ok(StatePart::CompatLookupMods),
            8192 => Ok(StatePart::PointerButtons),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for StatePart {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(StatePart, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum BoolCtrl {
    RepeatKeys = 1 << 0,
    SlowKeys = 1 << 1,
    BounceKeys = 1 << 2,
    StickyKeys = 1 << 3,
    MouseKeys = 1 << 4,
    MouseKeysAccel = 1 << 5,
    AccessXKeys = 1 << 6,
    AccessXTimeoutMask = 1 << 7,
    AccessXFeedbackMask = 1 << 8,
    AudibleBellMask = 1 << 9,
    Overlay1Mask = 1 << 10,
    Overlay2Mask = 1 << 11,
    IgnoreGroupLockMask = 1 << 12,
}
impl From<BoolCtrl> for u16 {
    fn from(input: BoolCtrl) -> Self {
        match input {
            BoolCtrl::RepeatKeys => 1 << 0,
            BoolCtrl::SlowKeys => 1 << 1,
            BoolCtrl::BounceKeys => 1 << 2,
            BoolCtrl::StickyKeys => 1 << 3,
            BoolCtrl::MouseKeys => 1 << 4,
            BoolCtrl::MouseKeysAccel => 1 << 5,
            BoolCtrl::AccessXKeys => 1 << 6,
            BoolCtrl::AccessXTimeoutMask => 1 << 7,
            BoolCtrl::AccessXFeedbackMask => 1 << 8,
            BoolCtrl::AudibleBellMask => 1 << 9,
            BoolCtrl::Overlay1Mask => 1 << 10,
            BoolCtrl::Overlay2Mask => 1 << 11,
            BoolCtrl::IgnoreGroupLockMask => 1 << 12,
        }
    }
}
impl From<BoolCtrl> for Option<u16> {
    fn from(input: BoolCtrl) -> Self {
        Some(u16::from(input))
    }
}
impl From<BoolCtrl> for u32 {
    fn from(input: BoolCtrl) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<BoolCtrl> for Option<u32> {
    fn from(input: BoolCtrl) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for BoolCtrl {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BoolCtrl::RepeatKeys),
            2 => Ok(BoolCtrl::SlowKeys),
            4 => Ok(BoolCtrl::BounceKeys),
            8 => Ok(BoolCtrl::StickyKeys),
            16 => Ok(BoolCtrl::MouseKeys),
            32 => Ok(BoolCtrl::MouseKeysAccel),
            64 => Ok(BoolCtrl::AccessXKeys),
            128 => Ok(BoolCtrl::AccessXTimeoutMask),
            256 => Ok(BoolCtrl::AccessXFeedbackMask),
            512 => Ok(BoolCtrl::AudibleBellMask),
            1024 => Ok(BoolCtrl::Overlay1Mask),
            2048 => Ok(BoolCtrl::Overlay2Mask),
            4096 => Ok(BoolCtrl::IgnoreGroupLockMask),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for BoolCtrl {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(BoolCtrl, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Control {
    GroupsWrap = 1 << 27,
    InternalMods = 1 << 28,
    IgnoreLockMods = 1 << 29,
    PerKeyRepeat = 1 << 30,
    ControlsEnabled = 1 << 31,
}
impl From<Control> for u32 {
    fn from(input: Control) -> Self {
        match input {
            Control::GroupsWrap => 1 << 27,
            Control::InternalMods => 1 << 28,
            Control::IgnoreLockMods => 1 << 29,
            Control::PerKeyRepeat => 1 << 30,
            Control::ControlsEnabled => 1 << 31,
        }
    }
}
impl From<Control> for Option<u32> {
    fn from(input: Control) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for Control {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            134_217_728 => Ok(Control::GroupsWrap),
            268_435_456 => Ok(Control::InternalMods),
            536_870_912 => Ok(Control::IgnoreLockMods),
            1_073_741_824 => Ok(Control::PerKeyRepeat),
            2_147_483_648 => Ok(Control::ControlsEnabled),
            _ => Err(ParseError::ParseError),
        }
    }
}
bitmask_binop!(Control, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum AXOption {
    SKPressFB = 1 << 0,
    SKAcceptFB = 1 << 1,
    FeatureFB = 1 << 2,
    SlowWarnFB = 1 << 3,
    IndicatorFB = 1 << 4,
    StickyKeysFB = 1 << 5,
    TwoKeys = 1 << 6,
    LatchToLock = 1 << 7,
    SKReleaseFB = 1 << 8,
    SKRejectFB = 1 << 9,
    BKRejectFB = 1 << 10,
    DumbBell = 1 << 11,
}
impl From<AXOption> for u16 {
    fn from(input: AXOption) -> Self {
        match input {
            AXOption::SKPressFB => 1 << 0,
            AXOption::SKAcceptFB => 1 << 1,
            AXOption::FeatureFB => 1 << 2,
            AXOption::SlowWarnFB => 1 << 3,
            AXOption::IndicatorFB => 1 << 4,
            AXOption::StickyKeysFB => 1 << 5,
            AXOption::TwoKeys => 1 << 6,
            AXOption::LatchToLock => 1 << 7,
            AXOption::SKReleaseFB => 1 << 8,
            AXOption::SKRejectFB => 1 << 9,
            AXOption::BKRejectFB => 1 << 10,
            AXOption::DumbBell => 1 << 11,
        }
    }
}
impl From<AXOption> for Option<u16> {
    fn from(input: AXOption) -> Self {
        Some(u16::from(input))
    }
}
impl From<AXOption> for u32 {
    fn from(input: AXOption) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<AXOption> for Option<u32> {
    fn from(input: AXOption) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for AXOption {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AXOption::SKPressFB),
            2 => Ok(AXOption::SKAcceptFB),
            4 => Ok(AXOption::FeatureFB),
            8 => Ok(AXOption::SlowWarnFB),
            16 => Ok(AXOption::IndicatorFB),
            32 => Ok(AXOption::StickyKeysFB),
            64 => Ok(AXOption::TwoKeys),
            128 => Ok(AXOption::LatchToLock),
            256 => Ok(AXOption::SKReleaseFB),
            512 => Ok(AXOption::SKRejectFB),
            1024 => Ok(AXOption::BKRejectFB),
            2048 => Ok(AXOption::DumbBell),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for AXOption {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(AXOption, u16);

pub type DeviceSpec = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LedClassResult {
    KbdFeedbackClass = 0,
    LedFeedbackClass = 4,
}
impl From<LedClassResult> for u8 {
    fn from(input: LedClassResult) -> Self {
        match input {
            LedClassResult::KbdFeedbackClass => 0,
            LedClassResult::LedFeedbackClass => 4,
        }
    }
}
impl From<LedClassResult> for Option<u8> {
    fn from(input: LedClassResult) -> Self {
        Some(u8::from(input))
    }
}
impl From<LedClassResult> for u16 {
    fn from(input: LedClassResult) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<LedClassResult> for Option<u16> {
    fn from(input: LedClassResult) -> Self {
        Some(u16::from(input))
    }
}
impl From<LedClassResult> for u32 {
    fn from(input: LedClassResult) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<LedClassResult> for Option<u32> {
    fn from(input: LedClassResult) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for LedClassResult {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LedClassResult::KbdFeedbackClass),
            4 => Ok(LedClassResult::LedFeedbackClass),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for LedClassResult {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for LedClassResult {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum LedClass {
    KbdFeedbackClass = 0,
    LedFeedbackClass = 4,
    DfltXIClass = 768,
    AllXIClasses = 1280,
}
impl From<LedClass> for u16 {
    fn from(input: LedClass) -> Self {
        match input {
            LedClass::KbdFeedbackClass => 0,
            LedClass::LedFeedbackClass => 4,
            LedClass::DfltXIClass => 768,
            LedClass::AllXIClasses => 1280,
        }
    }
}
impl From<LedClass> for Option<u16> {
    fn from(input: LedClass) -> Self {
        Some(u16::from(input))
    }
}
impl From<LedClass> for u32 {
    fn from(input: LedClass) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<LedClass> for Option<u32> {
    fn from(input: LedClass) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for LedClass {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LedClass::KbdFeedbackClass),
            4 => Ok(LedClass::LedFeedbackClass),
            768 => Ok(LedClass::DfltXIClass),
            1280 => Ok(LedClass::AllXIClasses),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for LedClass {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

pub type LedClassSpec = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BellClassResult {
    KbdFeedbackClass = 0,
    BellFeedbackClass = 5,
}
impl From<BellClassResult> for u8 {
    fn from(input: BellClassResult) -> Self {
        match input {
            BellClassResult::KbdFeedbackClass => 0,
            BellClassResult::BellFeedbackClass => 5,
        }
    }
}
impl From<BellClassResult> for Option<u8> {
    fn from(input: BellClassResult) -> Self {
        Some(u8::from(input))
    }
}
impl From<BellClassResult> for u16 {
    fn from(input: BellClassResult) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BellClassResult> for Option<u16> {
    fn from(input: BellClassResult) -> Self {
        Some(u16::from(input))
    }
}
impl From<BellClassResult> for u32 {
    fn from(input: BellClassResult) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BellClassResult> for Option<u32> {
    fn from(input: BellClassResult) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BellClassResult {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BellClassResult::KbdFeedbackClass),
            5 => Ok(BellClassResult::BellFeedbackClass),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for BellClassResult {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for BellClassResult {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum BellClass {
    KbdFeedbackClass = 0,
    BellFeedbackClass = 5,
    DfltXIClass = 768,
}
impl From<BellClass> for u16 {
    fn from(input: BellClass) -> Self {
        match input {
            BellClass::KbdFeedbackClass => 0,
            BellClass::BellFeedbackClass => 5,
            BellClass::DfltXIClass => 768,
        }
    }
}
impl From<BellClass> for Option<u16> {
    fn from(input: BellClass) -> Self {
        Some(u16::from(input))
    }
}
impl From<BellClass> for u32 {
    fn from(input: BellClass) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<BellClass> for Option<u32> {
    fn from(input: BellClass) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for BellClass {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BellClass::KbdFeedbackClass),
            5 => Ok(BellClass::BellFeedbackClass),
            768 => Ok(BellClass::DfltXIClass),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for BellClass {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

pub type BellClassSpec = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ID {
    UseCoreKbd = 256,
    UseCorePtr = 512,
    DfltXIClass = 768,
    DfltXIId = 1024,
    AllXIClass = 1280,
    AllXIId = 1536,
    XINone = 65280,
}
impl From<ID> for u16 {
    fn from(input: ID) -> Self {
        match input {
            ID::UseCoreKbd => 256,
            ID::UseCorePtr => 512,
            ID::DfltXIClass => 768,
            ID::DfltXIId => 1024,
            ID::AllXIClass => 1280,
            ID::AllXIId => 1536,
            ID::XINone => 65280,
        }
    }
}
impl From<ID> for Option<u16> {
    fn from(input: ID) -> Self {
        Some(u16::from(input))
    }
}
impl From<ID> for u32 {
    fn from(input: ID) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<ID> for Option<u32> {
    fn from(input: ID) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for ID {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            256 => Ok(ID::UseCoreKbd),
            512 => Ok(ID::UseCorePtr),
            768 => Ok(ID::DfltXIClass),
            1024 => Ok(ID::DfltXIId),
            1280 => Ok(ID::AllXIClass),
            1536 => Ok(ID::AllXIId),
            65280 => Ok(ID::XINone),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for ID {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

pub type IDSpec = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Group {
    M1 = 0,
    M2 = 1,
    M3 = 2,
    M4 = 3,
}
impl From<Group> for u8 {
    fn from(input: Group) -> Self {
        match input {
            Group::M1 => 0,
            Group::M2 => 1,
            Group::M3 => 2,
            Group::M4 => 3,
        }
    }
}
impl From<Group> for Option<u8> {
    fn from(input: Group) -> Self {
        Some(u8::from(input))
    }
}
impl From<Group> for u16 {
    fn from(input: Group) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Group> for Option<u16> {
    fn from(input: Group) -> Self {
        Some(u16::from(input))
    }
}
impl From<Group> for u32 {
    fn from(input: Group) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Group> for Option<u32> {
    fn from(input: Group) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Group {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Group::M1),
            1 => Ok(Group::M2),
            2 => Ok(Group::M3),
            3 => Ok(Group::M4),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Group {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Group {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Groups {
    Any = 254,
    All = 255,
}
impl From<Groups> for u8 {
    fn from(input: Groups) -> Self {
        match input {
            Groups::Any => 254,
            Groups::All => 255,
        }
    }
}
impl From<Groups> for Option<u8> {
    fn from(input: Groups) -> Self {
        Some(u8::from(input))
    }
}
impl From<Groups> for u16 {
    fn from(input: Groups) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Groups> for Option<u16> {
    fn from(input: Groups) -> Self {
        Some(u16::from(input))
    }
}
impl From<Groups> for u32 {
    fn from(input: Groups) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Groups> for Option<u32> {
    fn from(input: Groups) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Groups {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            254 => Ok(Groups::Any),
            255 => Ok(Groups::All),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Groups {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Groups {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SetOfGroup {
    Group1 = 1 << 0,
    Group2 = 1 << 1,
    Group3 = 1 << 2,
    Group4 = 1 << 3,
}
impl From<SetOfGroup> for u8 {
    fn from(input: SetOfGroup) -> Self {
        match input {
            SetOfGroup::Group1 => 1 << 0,
            SetOfGroup::Group2 => 1 << 1,
            SetOfGroup::Group3 => 1 << 2,
            SetOfGroup::Group4 => 1 << 3,
        }
    }
}
impl From<SetOfGroup> for Option<u8> {
    fn from(input: SetOfGroup) -> Self {
        Some(u8::from(input))
    }
}
impl From<SetOfGroup> for u16 {
    fn from(input: SetOfGroup) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetOfGroup> for Option<u16> {
    fn from(input: SetOfGroup) -> Self {
        Some(u16::from(input))
    }
}
impl From<SetOfGroup> for u32 {
    fn from(input: SetOfGroup) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetOfGroup> for Option<u32> {
    fn from(input: SetOfGroup) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SetOfGroup {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SetOfGroup::Group1),
            2 => Ok(SetOfGroup::Group2),
            4 => Ok(SetOfGroup::Group3),
            8 => Ok(SetOfGroup::Group4),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SetOfGroup {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SetOfGroup {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(SetOfGroup, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SetOfGroups {
    Any = 1 << 7,
}
impl From<SetOfGroups> for u8 {
    fn from(input: SetOfGroups) -> Self {
        match input {
            SetOfGroups::Any => 1 << 7,
        }
    }
}
impl From<SetOfGroups> for Option<u8> {
    fn from(input: SetOfGroups) -> Self {
        Some(u8::from(input))
    }
}
impl From<SetOfGroups> for u16 {
    fn from(input: SetOfGroups) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetOfGroups> for Option<u16> {
    fn from(input: SetOfGroups) -> Self {
        Some(u16::from(input))
    }
}
impl From<SetOfGroups> for u32 {
    fn from(input: SetOfGroups) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetOfGroups> for Option<u32> {
    fn from(input: SetOfGroups) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SetOfGroups {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(SetOfGroups::Any),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SetOfGroups {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SetOfGroups {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(SetOfGroups, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GroupsWrap {
    WrapIntoRange = 0,
    ClampIntoRange = 1 << 6,
    RedirectIntoRange = 1 << 7,
}
impl From<GroupsWrap> for u8 {
    fn from(input: GroupsWrap) -> Self {
        match input {
            GroupsWrap::WrapIntoRange => 0,
            GroupsWrap::ClampIntoRange => 1 << 6,
            GroupsWrap::RedirectIntoRange => 1 << 7,
        }
    }
}
impl From<GroupsWrap> for Option<u8> {
    fn from(input: GroupsWrap) -> Self {
        Some(u8::from(input))
    }
}
impl From<GroupsWrap> for u16 {
    fn from(input: GroupsWrap) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GroupsWrap> for Option<u16> {
    fn from(input: GroupsWrap) -> Self {
        Some(u16::from(input))
    }
}
impl From<GroupsWrap> for u32 {
    fn from(input: GroupsWrap) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GroupsWrap> for Option<u32> {
    fn from(input: GroupsWrap) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GroupsWrap {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GroupsWrap::WrapIntoRange),
            64 => Ok(GroupsWrap::ClampIntoRange),
            128 => Ok(GroupsWrap::RedirectIntoRange),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for GroupsWrap {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GroupsWrap {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(GroupsWrap, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VModsHigh {
    M15 = 1 << 7,
    M14 = 1 << 6,
    M13 = 1 << 5,
    M12 = 1 << 4,
    M11 = 1 << 3,
    M10 = 1 << 2,
    M9 = 1 << 1,
    M8 = 1 << 0,
}
impl From<VModsHigh> for u8 {
    fn from(input: VModsHigh) -> Self {
        match input {
            VModsHigh::M15 => 1 << 7,
            VModsHigh::M14 => 1 << 6,
            VModsHigh::M13 => 1 << 5,
            VModsHigh::M12 => 1 << 4,
            VModsHigh::M11 => 1 << 3,
            VModsHigh::M10 => 1 << 2,
            VModsHigh::M9 => 1 << 1,
            VModsHigh::M8 => 1 << 0,
        }
    }
}
impl From<VModsHigh> for Option<u8> {
    fn from(input: VModsHigh) -> Self {
        Some(u8::from(input))
    }
}
impl From<VModsHigh> for u16 {
    fn from(input: VModsHigh) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VModsHigh> for Option<u16> {
    fn from(input: VModsHigh) -> Self {
        Some(u16::from(input))
    }
}
impl From<VModsHigh> for u32 {
    fn from(input: VModsHigh) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VModsHigh> for Option<u32> {
    fn from(input: VModsHigh) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for VModsHigh {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(VModsHigh::M15),
            64 => Ok(VModsHigh::M14),
            32 => Ok(VModsHigh::M13),
            16 => Ok(VModsHigh::M12),
            8 => Ok(VModsHigh::M11),
            4 => Ok(VModsHigh::M10),
            2 => Ok(VModsHigh::M9),
            1 => Ok(VModsHigh::M8),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for VModsHigh {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for VModsHigh {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(VModsHigh, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VModsLow {
    M7 = 1 << 7,
    M6 = 1 << 6,
    M5 = 1 << 5,
    M4 = 1 << 4,
    M3 = 1 << 3,
    M2 = 1 << 2,
    M1 = 1 << 1,
    M0 = 1 << 0,
}
impl From<VModsLow> for u8 {
    fn from(input: VModsLow) -> Self {
        match input {
            VModsLow::M7 => 1 << 7,
            VModsLow::M6 => 1 << 6,
            VModsLow::M5 => 1 << 5,
            VModsLow::M4 => 1 << 4,
            VModsLow::M3 => 1 << 3,
            VModsLow::M2 => 1 << 2,
            VModsLow::M1 => 1 << 1,
            VModsLow::M0 => 1 << 0,
        }
    }
}
impl From<VModsLow> for Option<u8> {
    fn from(input: VModsLow) -> Self {
        Some(u8::from(input))
    }
}
impl From<VModsLow> for u16 {
    fn from(input: VModsLow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VModsLow> for Option<u16> {
    fn from(input: VModsLow) -> Self {
        Some(u16::from(input))
    }
}
impl From<VModsLow> for u32 {
    fn from(input: VModsLow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VModsLow> for Option<u32> {
    fn from(input: VModsLow) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for VModsLow {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(VModsLow::M7),
            64 => Ok(VModsLow::M6),
            32 => Ok(VModsLow::M5),
            16 => Ok(VModsLow::M4),
            8 => Ok(VModsLow::M3),
            4 => Ok(VModsLow::M2),
            2 => Ok(VModsLow::M1),
            1 => Ok(VModsLow::M0),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for VModsLow {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for VModsLow {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(VModsLow, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum VMod {
    M15 = 1 << 15,
    M14 = 1 << 14,
    M13 = 1 << 13,
    M12 = 1 << 12,
    M11 = 1 << 11,
    M10 = 1 << 10,
    M9 = 1 << 9,
    M8 = 1 << 8,
    M7 = 1 << 7,
    M6 = 1 << 6,
    M5 = 1 << 5,
    M4 = 1 << 4,
    M3 = 1 << 3,
    M2 = 1 << 2,
    M1 = 1 << 1,
    M0 = 1 << 0,
}
impl From<VMod> for u16 {
    fn from(input: VMod) -> Self {
        match input {
            VMod::M15 => 1 << 15,
            VMod::M14 => 1 << 14,
            VMod::M13 => 1 << 13,
            VMod::M12 => 1 << 12,
            VMod::M11 => 1 << 11,
            VMod::M10 => 1 << 10,
            VMod::M9 => 1 << 9,
            VMod::M8 => 1 << 8,
            VMod::M7 => 1 << 7,
            VMod::M6 => 1 << 6,
            VMod::M5 => 1 << 5,
            VMod::M4 => 1 << 4,
            VMod::M3 => 1 << 3,
            VMod::M2 => 1 << 2,
            VMod::M1 => 1 << 1,
            VMod::M0 => 1 << 0,
        }
    }
}
impl From<VMod> for Option<u16> {
    fn from(input: VMod) -> Self {
        Some(u16::from(input))
    }
}
impl From<VMod> for u32 {
    fn from(input: VMod) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<VMod> for Option<u32> {
    fn from(input: VMod) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for VMod {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            32768 => Ok(VMod::M15),
            16384 => Ok(VMod::M14),
            8192 => Ok(VMod::M13),
            4096 => Ok(VMod::M12),
            2048 => Ok(VMod::M11),
            1024 => Ok(VMod::M10),
            512 => Ok(VMod::M9),
            256 => Ok(VMod::M8),
            128 => Ok(VMod::M7),
            64 => Ok(VMod::M6),
            32 => Ok(VMod::M5),
            16 => Ok(VMod::M4),
            8 => Ok(VMod::M3),
            4 => Ok(VMod::M2),
            2 => Ok(VMod::M1),
            1 => Ok(VMod::M0),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for VMod {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(VMod, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Explicit {
    VModMap = 1 << 7,
    Behavior = 1 << 6,
    AutoRepeat = 1 << 5,
    Interpret = 1 << 4,
    KeyType4 = 1 << 3,
    KeyType3 = 1 << 2,
    KeyType2 = 1 << 1,
    KeyType1 = 1 << 0,
}
impl From<Explicit> for u8 {
    fn from(input: Explicit) -> Self {
        match input {
            Explicit::VModMap => 1 << 7,
            Explicit::Behavior => 1 << 6,
            Explicit::AutoRepeat => 1 << 5,
            Explicit::Interpret => 1 << 4,
            Explicit::KeyType4 => 1 << 3,
            Explicit::KeyType3 => 1 << 2,
            Explicit::KeyType2 => 1 << 1,
            Explicit::KeyType1 => 1 << 0,
        }
    }
}
impl From<Explicit> for Option<u8> {
    fn from(input: Explicit) -> Self {
        Some(u8::from(input))
    }
}
impl From<Explicit> for u16 {
    fn from(input: Explicit) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Explicit> for Option<u16> {
    fn from(input: Explicit) -> Self {
        Some(u16::from(input))
    }
}
impl From<Explicit> for u32 {
    fn from(input: Explicit) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Explicit> for Option<u32> {
    fn from(input: Explicit) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Explicit {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(Explicit::VModMap),
            64 => Ok(Explicit::Behavior),
            32 => Ok(Explicit::AutoRepeat),
            16 => Ok(Explicit::Interpret),
            8 => Ok(Explicit::KeyType4),
            4 => Ok(Explicit::KeyType3),
            2 => Ok(Explicit::KeyType2),
            1 => Ok(Explicit::KeyType1),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Explicit {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Explicit {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(Explicit, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SymInterpretMatch {
    NoneOf = 0,
    AnyOfOrNone = 1,
    AnyOf = 2,
    AllOf = 3,
    Exactly = 4,
}
impl From<SymInterpretMatch> for u8 {
    fn from(input: SymInterpretMatch) -> Self {
        match input {
            SymInterpretMatch::NoneOf => 0,
            SymInterpretMatch::AnyOfOrNone => 1,
            SymInterpretMatch::AnyOf => 2,
            SymInterpretMatch::AllOf => 3,
            SymInterpretMatch::Exactly => 4,
        }
    }
}
impl From<SymInterpretMatch> for Option<u8> {
    fn from(input: SymInterpretMatch) -> Self {
        Some(u8::from(input))
    }
}
impl From<SymInterpretMatch> for u16 {
    fn from(input: SymInterpretMatch) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SymInterpretMatch> for Option<u16> {
    fn from(input: SymInterpretMatch) -> Self {
        Some(u16::from(input))
    }
}
impl From<SymInterpretMatch> for u32 {
    fn from(input: SymInterpretMatch) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SymInterpretMatch> for Option<u32> {
    fn from(input: SymInterpretMatch) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SymInterpretMatch {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SymInterpretMatch::NoneOf),
            1 => Ok(SymInterpretMatch::AnyOfOrNone),
            2 => Ok(SymInterpretMatch::AnyOf),
            3 => Ok(SymInterpretMatch::AllOf),
            4 => Ok(SymInterpretMatch::Exactly),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SymInterpretMatch {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SymInterpretMatch {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SymInterpMatch {
    LevelOneOnly = 1 << 7,
    OpMask = 127,
}
impl From<SymInterpMatch> for u8 {
    fn from(input: SymInterpMatch) -> Self {
        match input {
            SymInterpMatch::LevelOneOnly => 1 << 7,
            SymInterpMatch::OpMask => 127,
        }
    }
}
impl From<SymInterpMatch> for Option<u8> {
    fn from(input: SymInterpMatch) -> Self {
        Some(u8::from(input))
    }
}
impl From<SymInterpMatch> for u16 {
    fn from(input: SymInterpMatch) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SymInterpMatch> for Option<u16> {
    fn from(input: SymInterpMatch) -> Self {
        Some(u16::from(input))
    }
}
impl From<SymInterpMatch> for u32 {
    fn from(input: SymInterpMatch) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SymInterpMatch> for Option<u32> {
    fn from(input: SymInterpMatch) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SymInterpMatch {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(SymInterpMatch::LevelOneOnly),
            127 => Ok(SymInterpMatch::OpMask),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SymInterpMatch {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SymInterpMatch {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IMFlag {
    NoExplicit = 1 << 7,
    NoAutomatic = 1 << 6,
    LEDDrivesKB = 1 << 5,
}
impl From<IMFlag> for u8 {
    fn from(input: IMFlag) -> Self {
        match input {
            IMFlag::NoExplicit => 1 << 7,
            IMFlag::NoAutomatic => 1 << 6,
            IMFlag::LEDDrivesKB => 1 << 5,
        }
    }
}
impl From<IMFlag> for Option<u8> {
    fn from(input: IMFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<IMFlag> for u16 {
    fn from(input: IMFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<IMFlag> for Option<u16> {
    fn from(input: IMFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<IMFlag> for u32 {
    fn from(input: IMFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<IMFlag> for Option<u32> {
    fn from(input: IMFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for IMFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(IMFlag::NoExplicit),
            64 => Ok(IMFlag::NoAutomatic),
            32 => Ok(IMFlag::LEDDrivesKB),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for IMFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for IMFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(IMFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IMModsWhich {
    UseCompat = 1 << 4,
    UseEffective = 1 << 3,
    UseLocked = 1 << 2,
    UseLatched = 1 << 1,
    UseBase = 1 << 0,
}
impl From<IMModsWhich> for u8 {
    fn from(input: IMModsWhich) -> Self {
        match input {
            IMModsWhich::UseCompat => 1 << 4,
            IMModsWhich::UseEffective => 1 << 3,
            IMModsWhich::UseLocked => 1 << 2,
            IMModsWhich::UseLatched => 1 << 1,
            IMModsWhich::UseBase => 1 << 0,
        }
    }
}
impl From<IMModsWhich> for Option<u8> {
    fn from(input: IMModsWhich) -> Self {
        Some(u8::from(input))
    }
}
impl From<IMModsWhich> for u16 {
    fn from(input: IMModsWhich) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<IMModsWhich> for Option<u16> {
    fn from(input: IMModsWhich) -> Self {
        Some(u16::from(input))
    }
}
impl From<IMModsWhich> for u32 {
    fn from(input: IMModsWhich) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<IMModsWhich> for Option<u32> {
    fn from(input: IMModsWhich) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for IMModsWhich {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            16 => Ok(IMModsWhich::UseCompat),
            8 => Ok(IMModsWhich::UseEffective),
            4 => Ok(IMModsWhich::UseLocked),
            2 => Ok(IMModsWhich::UseLatched),
            1 => Ok(IMModsWhich::UseBase),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for IMModsWhich {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for IMModsWhich {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(IMModsWhich, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IMGroupsWhich {
    UseCompat = 1 << 4,
    UseEffective = 1 << 3,
    UseLocked = 1 << 2,
    UseLatched = 1 << 1,
    UseBase = 1 << 0,
}
impl From<IMGroupsWhich> for u8 {
    fn from(input: IMGroupsWhich) -> Self {
        match input {
            IMGroupsWhich::UseCompat => 1 << 4,
            IMGroupsWhich::UseEffective => 1 << 3,
            IMGroupsWhich::UseLocked => 1 << 2,
            IMGroupsWhich::UseLatched => 1 << 1,
            IMGroupsWhich::UseBase => 1 << 0,
        }
    }
}
impl From<IMGroupsWhich> for Option<u8> {
    fn from(input: IMGroupsWhich) -> Self {
        Some(u8::from(input))
    }
}
impl From<IMGroupsWhich> for u16 {
    fn from(input: IMGroupsWhich) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<IMGroupsWhich> for Option<u16> {
    fn from(input: IMGroupsWhich) -> Self {
        Some(u16::from(input))
    }
}
impl From<IMGroupsWhich> for u32 {
    fn from(input: IMGroupsWhich) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<IMGroupsWhich> for Option<u32> {
    fn from(input: IMGroupsWhich) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for IMGroupsWhich {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            16 => Ok(IMGroupsWhich::UseCompat),
            8 => Ok(IMGroupsWhich::UseEffective),
            4 => Ok(IMGroupsWhich::UseLocked),
            2 => Ok(IMGroupsWhich::UseLatched),
            1 => Ok(IMGroupsWhich::UseBase),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for IMGroupsWhich {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for IMGroupsWhich {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(IMGroupsWhich, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicatorMap {
    pub flags: IMFlag,
    pub which_groups: IMGroupsWhich,
    pub groups: SetOfGroup,
    pub which_mods: IMModsWhich,
    pub mods: u8,
    pub real_mods: u8,
    pub vmods: u16,
    pub ctrls: u32,
}
impl TryParse for IndicatorMap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (which_groups, remaining) = u8::try_parse(remaining)?;
        let (groups, remaining) = u8::try_parse(remaining)?;
        let (which_mods, remaining) = u8::try_parse(remaining)?;
        let (mods, remaining) = u8::try_parse(remaining)?;
        let (real_mods, remaining) = u8::try_parse(remaining)?;
        let (vmods, remaining) = u16::try_parse(remaining)?;
        let (ctrls, remaining) = u32::try_parse(remaining)?;
        let flags = flags.try_into()?;
        let which_groups = which_groups.try_into()?;
        let groups = groups.try_into()?;
        let which_mods = which_mods.try_into()?;
        let result = IndicatorMap { flags, which_groups, groups, which_mods, mods, real_mods, vmods, ctrls };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IndicatorMap {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for IndicatorMap {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let flags_bytes = u8::from(self.flags).serialize();
        let which_groups_bytes = u8::from(self.which_groups).serialize();
        let groups_bytes = u8::from(self.groups).serialize();
        let which_mods_bytes = u8::from(self.which_mods).serialize();
        let mods_bytes = self.mods.serialize();
        let real_mods_bytes = self.real_mods.serialize();
        let vmods_bytes = self.vmods.serialize();
        let ctrls_bytes = self.ctrls.serialize();
        [
            flags_bytes[0],
            which_groups_bytes[0],
            groups_bytes[0],
            which_mods_bytes[0],
            mods_bytes[0],
            real_mods_bytes[0],
            vmods_bytes[0],
            vmods_bytes[1],
            ctrls_bytes[0],
            ctrls_bytes[1],
            ctrls_bytes[2],
            ctrls_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        u8::from(self.flags).serialize_into(bytes);
        u8::from(self.which_groups).serialize_into(bytes);
        u8::from(self.groups).serialize_into(bytes);
        u8::from(self.which_mods).serialize_into(bytes);
        self.mods.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.vmods.serialize_into(bytes);
        self.ctrls.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDetail {
    SymInterp = 1 << 0,
    GroupCompat = 1 << 1,
}
impl From<CMDetail> for u8 {
    fn from(input: CMDetail) -> Self {
        match input {
            CMDetail::SymInterp => 1 << 0,
            CMDetail::GroupCompat => 1 << 1,
        }
    }
}
impl From<CMDetail> for Option<u8> {
    fn from(input: CMDetail) -> Self {
        Some(u8::from(input))
    }
}
impl From<CMDetail> for u16 {
    fn from(input: CMDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CMDetail> for Option<u16> {
    fn from(input: CMDetail) -> Self {
        Some(u16::from(input))
    }
}
impl From<CMDetail> for u32 {
    fn from(input: CMDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CMDetail> for Option<u32> {
    fn from(input: CMDetail) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CMDetail {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CMDetail::SymInterp),
            2 => Ok(CMDetail::GroupCompat),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CMDetail {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CMDetail {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(CMDetail, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum NameDetail {
    Keycodes = 1 << 0,
    Geometry = 1 << 1,
    Symbols = 1 << 2,
    PhysSymbols = 1 << 3,
    Types = 1 << 4,
    Compat = 1 << 5,
    KeyTypeNames = 1 << 6,
    KTLevelNames = 1 << 7,
    IndicatorNames = 1 << 8,
    KeyNames = 1 << 9,
    KeyAliases = 1 << 10,
    VirtualModNames = 1 << 11,
    GroupNames = 1 << 12,
    RGNames = 1 << 13,
}
impl From<NameDetail> for u16 {
    fn from(input: NameDetail) -> Self {
        match input {
            NameDetail::Keycodes => 1 << 0,
            NameDetail::Geometry => 1 << 1,
            NameDetail::Symbols => 1 << 2,
            NameDetail::PhysSymbols => 1 << 3,
            NameDetail::Types => 1 << 4,
            NameDetail::Compat => 1 << 5,
            NameDetail::KeyTypeNames => 1 << 6,
            NameDetail::KTLevelNames => 1 << 7,
            NameDetail::IndicatorNames => 1 << 8,
            NameDetail::KeyNames => 1 << 9,
            NameDetail::KeyAliases => 1 << 10,
            NameDetail::VirtualModNames => 1 << 11,
            NameDetail::GroupNames => 1 << 12,
            NameDetail::RGNames => 1 << 13,
        }
    }
}
impl From<NameDetail> for Option<u16> {
    fn from(input: NameDetail) -> Self {
        Some(u16::from(input))
    }
}
impl From<NameDetail> for u32 {
    fn from(input: NameDetail) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<NameDetail> for Option<u32> {
    fn from(input: NameDetail) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for NameDetail {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NameDetail::Keycodes),
            2 => Ok(NameDetail::Geometry),
            4 => Ok(NameDetail::Symbols),
            8 => Ok(NameDetail::PhysSymbols),
            16 => Ok(NameDetail::Types),
            32 => Ok(NameDetail::Compat),
            64 => Ok(NameDetail::KeyTypeNames),
            128 => Ok(NameDetail::KTLevelNames),
            256 => Ok(NameDetail::IndicatorNames),
            512 => Ok(NameDetail::KeyNames),
            1024 => Ok(NameDetail::KeyAliases),
            2048 => Ok(NameDetail::VirtualModNames),
            4096 => Ok(NameDetail::GroupNames),
            8192 => Ok(NameDetail::RGNames),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for NameDetail {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(NameDetail, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GBNDetail {
    Types = 1 << 0,
    CompatMap = 1 << 1,
    ClientSymbols = 1 << 2,
    ServerSymbols = 1 << 3,
    IndicatorMaps = 1 << 4,
    KeyNames = 1 << 5,
    Geometry = 1 << 6,
    OtherNames = 1 << 7,
}
impl From<GBNDetail> for u8 {
    fn from(input: GBNDetail) -> Self {
        match input {
            GBNDetail::Types => 1 << 0,
            GBNDetail::CompatMap => 1 << 1,
            GBNDetail::ClientSymbols => 1 << 2,
            GBNDetail::ServerSymbols => 1 << 3,
            GBNDetail::IndicatorMaps => 1 << 4,
            GBNDetail::KeyNames => 1 << 5,
            GBNDetail::Geometry => 1 << 6,
            GBNDetail::OtherNames => 1 << 7,
        }
    }
}
impl From<GBNDetail> for Option<u8> {
    fn from(input: GBNDetail) -> Self {
        Some(u8::from(input))
    }
}
impl From<GBNDetail> for u16 {
    fn from(input: GBNDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GBNDetail> for Option<u16> {
    fn from(input: GBNDetail) -> Self {
        Some(u16::from(input))
    }
}
impl From<GBNDetail> for u32 {
    fn from(input: GBNDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GBNDetail> for Option<u32> {
    fn from(input: GBNDetail) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GBNDetail {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(GBNDetail::Types),
            2 => Ok(GBNDetail::CompatMap),
            4 => Ok(GBNDetail::ClientSymbols),
            8 => Ok(GBNDetail::ServerSymbols),
            16 => Ok(GBNDetail::IndicatorMaps),
            32 => Ok(GBNDetail::KeyNames),
            64 => Ok(GBNDetail::Geometry),
            128 => Ok(GBNDetail::OtherNames),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for GBNDetail {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GBNDetail {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(GBNDetail, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum XIFeature {
    Keyboards = 1 << 0,
    ButtonActions = 1 << 1,
    IndicatorNames = 1 << 2,
    IndicatorMaps = 1 << 3,
    IndicatorState = 1 << 4,
}
impl From<XIFeature> for u8 {
    fn from(input: XIFeature) -> Self {
        match input {
            XIFeature::Keyboards => 1 << 0,
            XIFeature::ButtonActions => 1 << 1,
            XIFeature::IndicatorNames => 1 << 2,
            XIFeature::IndicatorMaps => 1 << 3,
            XIFeature::IndicatorState => 1 << 4,
        }
    }
}
impl From<XIFeature> for Option<u8> {
    fn from(input: XIFeature) -> Self {
        Some(u8::from(input))
    }
}
impl From<XIFeature> for u16 {
    fn from(input: XIFeature) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<XIFeature> for Option<u16> {
    fn from(input: XIFeature) -> Self {
        Some(u16::from(input))
    }
}
impl From<XIFeature> for u32 {
    fn from(input: XIFeature) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<XIFeature> for Option<u32> {
    fn from(input: XIFeature) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for XIFeature {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(XIFeature::Keyboards),
            2 => Ok(XIFeature::ButtonActions),
            4 => Ok(XIFeature::IndicatorNames),
            8 => Ok(XIFeature::IndicatorMaps),
            16 => Ok(XIFeature::IndicatorState),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for XIFeature {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for XIFeature {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(XIFeature, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PerClientFlag {
    DetectableAutoRepeat = 1 << 0,
    GrabsUseXKBState = 1 << 1,
    AutoResetControls = 1 << 2,
    LookupStateWhenGrabbed = 1 << 3,
    SendEventUsesXKBState = 1 << 4,
}
impl From<PerClientFlag> for u8 {
    fn from(input: PerClientFlag) -> Self {
        match input {
            PerClientFlag::DetectableAutoRepeat => 1 << 0,
            PerClientFlag::GrabsUseXKBState => 1 << 1,
            PerClientFlag::AutoResetControls => 1 << 2,
            PerClientFlag::LookupStateWhenGrabbed => 1 << 3,
            PerClientFlag::SendEventUsesXKBState => 1 << 4,
        }
    }
}
impl From<PerClientFlag> for Option<u8> {
    fn from(input: PerClientFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<PerClientFlag> for u16 {
    fn from(input: PerClientFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PerClientFlag> for Option<u16> {
    fn from(input: PerClientFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<PerClientFlag> for u32 {
    fn from(input: PerClientFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PerClientFlag> for Option<u32> {
    fn from(input: PerClientFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PerClientFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PerClientFlag::DetectableAutoRepeat),
            2 => Ok(PerClientFlag::GrabsUseXKBState),
            4 => Ok(PerClientFlag::AutoResetControls),
            8 => Ok(PerClientFlag::LookupStateWhenGrabbed),
            16 => Ok(PerClientFlag::SendEventUsesXKBState),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PerClientFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PerClientFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(PerClientFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModDef {
    pub mask: u8,
    pub real_mods: u8,
    pub vmods: u16,
}
impl TryParse for ModDef {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (mask, remaining) = u8::try_parse(remaining)?;
        let (real_mods, remaining) = u8::try_parse(remaining)?;
        let (vmods, remaining) = u16::try_parse(remaining)?;
        let result = ModDef { mask, real_mods, vmods };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ModDef {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ModDef {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let mask_bytes = self.mask.serialize();
        let real_mods_bytes = self.real_mods.serialize();
        let vmods_bytes = self.vmods.serialize();
        [
            mask_bytes[0],
            real_mods_bytes[0],
            vmods_bytes[0],
            vmods_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.mask.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.vmods.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyName {
    pub name: [u8; 4],
}
impl TryParse for KeyName {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let name = <[u8; 4]>::try_from(name).unwrap();
        let result = KeyName { name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyName {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeyName {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        [
            self.name[0],
            self.name[1],
            self.name[2],
            self.name[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        bytes.extend_from_slice(&self.name);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyAlias {
    pub real: [u8; 4],
    pub alias: [u8; 4],
}
impl TryParse for KeyAlias {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (real, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let real = <[u8; 4]>::try_from(real).unwrap();
        let (alias, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let alias = <[u8; 4]>::try_from(alias).unwrap();
        let result = KeyAlias { real, alias };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyAlias {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeyAlias {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        [
            self.real[0],
            self.real[1],
            self.real[2],
            self.real[3],
            self.alias[0],
            self.alias[1],
            self.alias[2],
            self.alias[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        bytes.extend_from_slice(&self.real);
        bytes.extend_from_slice(&self.alias);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountedString16 {
    pub string: Vec<u8>,
    pub alignment_pad: Vec<u8>,
}
impl TryParse for CountedString16 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (length, remaining) = u16::try_parse(remaining)?;
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, length.try_into().or(Err(ParseError::ParseError))?)?;
        let string = string.to_vec();
        let (alignment_pad, remaining) = crate::x11_utils::parse_u8_list(remaining, (u32::from(length).checked_add(5u32).ok_or(ParseError::ParseError)? & (!3u32)).checked_sub(u32::from(length).checked_add(2u32).ok_or(ParseError::ParseError)?).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let alignment_pad = alignment_pad.to_vec();
        let result = CountedString16 { string, alignment_pad };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CountedString16 {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for CountedString16 {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        let length = u16::try_from(self.string.len()).expect("`string` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&self.string);
        assert_eq!(self.alignment_pad.len(), usize::try_from((u32::from(length).checked_add(5u32).unwrap() & (!3u32)).checked_sub(u32::from(length).checked_add(2u32).unwrap()).unwrap()).unwrap(), "`alignment_pad` has an incorrect length");
        bytes.extend_from_slice(&self.alignment_pad);
    }
}
impl CountedString16 {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `string` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u16 {
        self.string.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KTMapEntry {
    pub active: bool,
    pub mods_mask: u8,
    pub level: u8,
    pub mods_mods: u8,
    pub mods_vmods: u16,
}
impl TryParse for KTMapEntry {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (active, remaining) = bool::try_parse(remaining)?;
        let (mods_mask, remaining) = u8::try_parse(remaining)?;
        let (level, remaining) = u8::try_parse(remaining)?;
        let (mods_mods, remaining) = u8::try_parse(remaining)?;
        let (mods_vmods, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let result = KTMapEntry { active, mods_mask, level, mods_mods, mods_vmods };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KTMapEntry {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KTMapEntry {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let active_bytes = self.active.serialize();
        let mods_mask_bytes = self.mods_mask.serialize();
        let level_bytes = self.level.serialize();
        let mods_mods_bytes = self.mods_mods.serialize();
        let mods_vmods_bytes = self.mods_vmods.serialize();
        [
            active_bytes[0],
            mods_mask_bytes[0],
            level_bytes[0],
            mods_mods_bytes[0],
            mods_vmods_bytes[0],
            mods_vmods_bytes[1],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.active.serialize_into(bytes);
        self.mods_mask.serialize_into(bytes);
        self.level.serialize_into(bytes);
        self.mods_mods.serialize_into(bytes);
        self.mods_vmods.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyType {
    pub mods_mask: u8,
    pub mods_mods: u8,
    pub mods_vmods: u16,
    pub num_levels: u8,
    pub has_preserve: bool,
    pub map: Vec<KTMapEntry>,
    pub preserve: Vec<ModDef>,
}
impl TryParse for KeyType {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (mods_mask, remaining) = u8::try_parse(remaining)?;
        let (mods_mods, remaining) = u8::try_parse(remaining)?;
        let (mods_vmods, remaining) = u16::try_parse(remaining)?;
        let (num_levels, remaining) = u8::try_parse(remaining)?;
        let (n_map_entries, remaining) = u8::try_parse(remaining)?;
        let (has_preserve, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (map, remaining) = crate::x11_utils::parse_list::<KTMapEntry>(remaining, n_map_entries.try_into().or(Err(ParseError::ParseError))?)?;
        let (preserve, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, u32::from(has_preserve).checked_mul(u32::from(n_map_entries)).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let result = KeyType { mods_mask, mods_mods, mods_vmods, num_levels, has_preserve, map, preserve };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyType {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeyType {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.mods_mask.serialize_into(bytes);
        self.mods_mods.serialize_into(bytes);
        self.mods_vmods.serialize_into(bytes);
        self.num_levels.serialize_into(bytes);
        let n_map_entries = u8::try_from(self.map.len()).expect("`map` has too many elements");
        n_map_entries.serialize_into(bytes);
        self.has_preserve.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.map.serialize_into(bytes);
        assert_eq!(self.preserve.len(), usize::try_from(u32::from(self.has_preserve).checked_mul(u32::from(n_map_entries)).unwrap()).unwrap(), "`preserve` has an incorrect length");
        self.preserve.serialize_into(bytes);
    }
}
impl KeyType {
    /// Get the value of the `nMapEntries` field.
    ///
    /// The `nMapEntries` field is used as the length field of the `map` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_map_entries(&self) -> u8 {
        self.map.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeySymMap {
    pub kt_index: [u8; 4],
    pub group_info: u8,
    pub width: u8,
    pub syms: Vec<xproto::Keysym>,
}
impl TryParse for KeySymMap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (kt_index, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let kt_index = <[u8; 4]>::try_from(kt_index).unwrap();
        let (group_info, remaining) = u8::try_parse(remaining)?;
        let (width, remaining) = u8::try_parse(remaining)?;
        let (n_syms, remaining) = u16::try_parse(remaining)?;
        let (syms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, n_syms.try_into().or(Err(ParseError::ParseError))?)?;
        let result = KeySymMap { kt_index, group_info, width, syms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeySymMap {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeySymMap {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        bytes.extend_from_slice(&self.kt_index);
        self.group_info.serialize_into(bytes);
        self.width.serialize_into(bytes);
        let n_syms = u16::try_from(self.syms.len()).expect("`syms` has too many elements");
        n_syms.serialize_into(bytes);
        self.syms.serialize_into(bytes);
    }
}
impl KeySymMap {
    /// Get the value of the `nSyms` field.
    ///
    /// The `nSyms` field is used as the length field of the `syms` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_syms(&self) -> u16 {
        self.syms.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommonBehavior {
    pub type_: u8,
    pub data: u8,
}
impl TryParse for CommonBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (data, remaining) = u8::try_parse(remaining)?;
        let result = CommonBehavior { type_, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CommonBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for CommonBehavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let type_bytes = self.type_.serialize();
        let data_bytes = self.data.serialize();
        [
            type_bytes[0],
            data_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.type_.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultBehavior {
    pub type_: u8,
}
impl TryParse for DefaultBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = DefaultBehavior { type_ };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DefaultBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DefaultBehavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let type_bytes = self.type_.serialize();
        [
            type_bytes[0],
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.type_.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
    }
}

pub type LockBehavior = DefaultBehavior;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadioGroupBehavior {
    pub type_: u8,
    pub group: u8,
}
impl TryParse for RadioGroupBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = u8::try_parse(remaining)?;
        let result = RadioGroupBehavior { type_, group };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RadioGroupBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for RadioGroupBehavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let type_bytes = self.type_.serialize();
        let group_bytes = self.group.serialize();
        [
            type_bytes[0],
            group_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.type_.serialize_into(bytes);
        self.group.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayBehavior {
    pub type_: u8,
    pub key: xproto::Keycode,
}
impl TryParse for OverlayBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let result = OverlayBehavior { type_, key };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for OverlayBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for OverlayBehavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let type_bytes = self.type_.serialize();
        let key_bytes = self.key.serialize();
        [
            type_bytes[0],
            key_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.type_.serialize_into(bytes);
        self.key.serialize_into(bytes);
    }
}

pub type PermamentLockBehavior = LockBehavior;

pub type PermamentRadioGroupBehavior = RadioGroupBehavior;

pub type PermamentOverlayBehavior = OverlayBehavior;

#[derive(Debug, Copy, Clone)]
pub struct Behavior([u8; 2]);
impl Behavior {
    pub fn as_common(&self) -> CommonBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<CommonBehavior, ParseError> {
            let (common, remaining) = CommonBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(common)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_default(&self) -> DefaultBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<DefaultBehavior, ParseError> {
            let (default, remaining) = DefaultBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(default)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_lock(&self) -> LockBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<LockBehavior, ParseError> {
            let (lock, remaining) = LockBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(lock)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_radio_group(&self) -> RadioGroupBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<RadioGroupBehavior, ParseError> {
            let (radio_group, remaining) = RadioGroupBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(radio_group)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_overlay1(&self) -> OverlayBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<OverlayBehavior, ParseError> {
            let (overlay1, remaining) = OverlayBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(overlay1)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_overlay2(&self) -> OverlayBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<OverlayBehavior, ParseError> {
            let (overlay2, remaining) = OverlayBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(overlay2)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_permament_lock(&self) -> PermamentLockBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<PermamentLockBehavior, ParseError> {
            let (permament_lock, remaining) = PermamentLockBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(permament_lock)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_permament_radio_group(&self) -> PermamentRadioGroupBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<PermamentRadioGroupBehavior, ParseError> {
            let (permament_radio_group, remaining) = PermamentRadioGroupBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(permament_radio_group)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_permament_overlay1(&self) -> PermamentOverlayBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<PermamentOverlayBehavior, ParseError> {
            let (permament_overlay1, remaining) = PermamentOverlayBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(permament_overlay1)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_permament_overlay2(&self) -> PermamentOverlayBehavior {
        fn do_the_parse(remaining: &[u8]) -> Result<PermamentOverlayBehavior, ParseError> {
            let (permament_overlay2, remaining) = PermamentOverlayBehavior::try_parse(remaining)?;
            let _ = remaining;
            Ok(permament_overlay2)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_type(&self) -> u8 {
        fn do_the_parse(remaining: &[u8]) -> Result<u8, ParseError> {
            let (type_, remaining) = u8::try_parse(remaining)?;
            let _ = remaining;
            Ok(type_)
        }
        do_the_parse(&self.0).unwrap()
    }
}
impl Serialize for Behavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        self.0
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}
impl TryParse for Behavior {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner: [u8; 2] = value.get(..2)
            .ok_or(ParseError::ParseError)?
            .try_into()
            .unwrap();
        let result = Behavior(inner);
        Ok((result, &value[2..]))
    }
}
impl From<CommonBehavior> for Behavior {
    fn from(common: CommonBehavior) -> Self {
        let common_bytes = common.serialize();
        Self(common_bytes)
    }
}
impl From<DefaultBehavior> for Behavior {
    fn from(default: DefaultBehavior) -> Self {
        let default_bytes = default.serialize();
        Self(default_bytes)
    }
}
impl From<RadioGroupBehavior> for Behavior {
    fn from(radio_group: RadioGroupBehavior) -> Self {
        let radio_group_bytes = radio_group.serialize();
        Self(radio_group_bytes)
    }
}
impl From<OverlayBehavior> for Behavior {
    fn from(overlay1: OverlayBehavior) -> Self {
        let overlay1_bytes = overlay1.serialize();
        Self(overlay1_bytes)
    }
}
impl From<u8> for Behavior {
    fn from(type_: u8) -> Self {
        let type_bytes = type_.serialize();
        let value = [
            type_bytes[0],
            0,
        ];
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BehaviorType {
    Default = 0,
    Lock = 1,
    RadioGroup = 2,
    Overlay1 = 3,
    Overlay2 = 4,
    PermamentLock = 129,
    PermamentRadioGroup = 130,
    PermamentOverlay1 = 131,
    PermamentOverlay2 = 132,
}
impl From<BehaviorType> for u8 {
    fn from(input: BehaviorType) -> Self {
        match input {
            BehaviorType::Default => 0,
            BehaviorType::Lock => 1,
            BehaviorType::RadioGroup => 2,
            BehaviorType::Overlay1 => 3,
            BehaviorType::Overlay2 => 4,
            BehaviorType::PermamentLock => 129,
            BehaviorType::PermamentRadioGroup => 130,
            BehaviorType::PermamentOverlay1 => 131,
            BehaviorType::PermamentOverlay2 => 132,
        }
    }
}
impl From<BehaviorType> for Option<u8> {
    fn from(input: BehaviorType) -> Self {
        Some(u8::from(input))
    }
}
impl From<BehaviorType> for u16 {
    fn from(input: BehaviorType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BehaviorType> for Option<u16> {
    fn from(input: BehaviorType) -> Self {
        Some(u16::from(input))
    }
}
impl From<BehaviorType> for u32 {
    fn from(input: BehaviorType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BehaviorType> for Option<u32> {
    fn from(input: BehaviorType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BehaviorType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BehaviorType::Default),
            1 => Ok(BehaviorType::Lock),
            2 => Ok(BehaviorType::RadioGroup),
            3 => Ok(BehaviorType::Overlay1),
            4 => Ok(BehaviorType::Overlay2),
            129 => Ok(BehaviorType::PermamentLock),
            130 => Ok(BehaviorType::PermamentRadioGroup),
            131 => Ok(BehaviorType::PermamentOverlay1),
            132 => Ok(BehaviorType::PermamentOverlay2),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for BehaviorType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for BehaviorType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetBehavior {
    pub keycode: xproto::Keycode,
    pub behavior: Behavior,
}
impl TryParse for SetBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (behavior, remaining) = Behavior::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = SetBehavior { keycode, behavior };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SetBehavior {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let keycode_bytes = self.keycode.serialize();
        let behavior_bytes = self.behavior.serialize();
        [
            keycode_bytes[0],
            behavior_bytes[0],
            behavior_bytes[1],
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.keycode.serialize_into(bytes);
        self.behavior.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetExplicit {
    pub keycode: xproto::Keycode,
    pub explicit: u8,
}
impl TryParse for SetExplicit {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (explicit, remaining) = u8::try_parse(remaining)?;
        let result = SetExplicit { keycode, explicit };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetExplicit {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SetExplicit {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let keycode_bytes = self.keycode.serialize();
        let explicit_bytes = self.explicit.serialize();
        [
            keycode_bytes[0],
            explicit_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.keycode.serialize_into(bytes);
        self.explicit.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModMap {
    pub keycode: xproto::Keycode,
    pub mods: u8,
}
impl TryParse for KeyModMap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (mods, remaining) = u8::try_parse(remaining)?;
        let result = KeyModMap { keycode, mods };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyModMap {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeyModMap {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let keycode_bytes = self.keycode.serialize();
        let mods_bytes = self.mods.serialize();
        [
            keycode_bytes[0],
            mods_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.keycode.serialize_into(bytes);
        self.mods.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyVModMap {
    pub keycode: xproto::Keycode,
    pub vmods: u16,
}
impl TryParse for KeyVModMap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (vmods, remaining) = u16::try_parse(remaining)?;
        let result = KeyVModMap { keycode, vmods };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyVModMap {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KeyVModMap {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let keycode_bytes = self.keycode.serialize();
        let vmods_bytes = self.vmods.serialize();
        [
            keycode_bytes[0],
            0,
            vmods_bytes[0],
            vmods_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.keycode.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.vmods.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KTSetMapEntry {
    pub level: u8,
    pub real_mods: u8,
    pub virtual_mods: u16,
}
impl TryParse for KTSetMapEntry {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (level, remaining) = u8::try_parse(remaining)?;
        let (real_mods, remaining) = u8::try_parse(remaining)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let result = KTSetMapEntry { level, real_mods, virtual_mods };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KTSetMapEntry {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for KTSetMapEntry {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let level_bytes = self.level.serialize();
        let real_mods_bytes = self.real_mods.serialize();
        let virtual_mods_bytes = self.virtual_mods.serialize();
        [
            level_bytes[0],
            real_mods_bytes[0],
            virtual_mods_bytes[0],
            virtual_mods_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.level.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.virtual_mods.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetKeyType {
    pub mask: u8,
    pub real_mods: u8,
    pub virtual_mods: u16,
    pub num_levels: u8,
    pub preserve: bool,
    pub entries: Vec<KTSetMapEntry>,
    pub preserve_entries: Vec<KTSetMapEntry>,
}
impl TryParse for SetKeyType {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (mask, remaining) = u8::try_parse(remaining)?;
        let (real_mods, remaining) = u8::try_parse(remaining)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (num_levels, remaining) = u8::try_parse(remaining)?;
        let (n_map_entries, remaining) = u8::try_parse(remaining)?;
        let (preserve, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (entries, remaining) = crate::x11_utils::parse_list::<KTSetMapEntry>(remaining, n_map_entries.try_into().or(Err(ParseError::ParseError))?)?;
        let (preserve_entries, remaining) = crate::x11_utils::parse_list::<KTSetMapEntry>(remaining, u32::from(preserve).checked_mul(u32::from(n_map_entries)).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let result = SetKeyType { mask, real_mods, virtual_mods, num_levels, preserve, entries, preserve_entries };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetKeyType {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SetKeyType {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.mask.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.virtual_mods.serialize_into(bytes);
        self.num_levels.serialize_into(bytes);
        let n_map_entries = u8::try_from(self.entries.len()).expect("`entries` has too many elements");
        n_map_entries.serialize_into(bytes);
        self.preserve.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.entries.serialize_into(bytes);
        assert_eq!(self.preserve_entries.len(), usize::try_from(u32::from(self.preserve).checked_mul(u32::from(n_map_entries)).unwrap()).unwrap(), "`preserve_entries` has an incorrect length");
        self.preserve_entries.serialize_into(bytes);
    }
}
impl SetKeyType {
    /// Get the value of the `nMapEntries` field.
    ///
    /// The `nMapEntries` field is used as the length field of the `entries` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_map_entries(&self) -> u8 {
        self.entries.len()
            .try_into().unwrap()
    }
}

pub type String8 = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outline {
    pub corner_radius: u8,
    pub points: Vec<xproto::Point>,
}
impl TryParse for Outline {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (n_points, remaining) = u8::try_parse(remaining)?;
        let (corner_radius, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (points, remaining) = crate::x11_utils::parse_list::<xproto::Point>(remaining, n_points.try_into().or(Err(ParseError::ParseError))?)?;
        let result = Outline { corner_radius, points };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Outline {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Outline {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let n_points = u8::try_from(self.points.len()).expect("`points` has too many elements");
        n_points.serialize_into(bytes);
        self.corner_radius.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.points.serialize_into(bytes);
    }
}
impl Outline {
    /// Get the value of the `nPoints` field.
    ///
    /// The `nPoints` field is used as the length field of the `points` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_points(&self) -> u8 {
        self.points.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub name: xproto::Atom,
    pub primary_ndx: u8,
    pub approx_ndx: u8,
    pub outlines: Vec<Outline>,
}
impl TryParse for Shape {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (n_outlines, remaining) = u8::try_parse(remaining)?;
        let (primary_ndx, remaining) = u8::try_parse(remaining)?;
        let (approx_ndx, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (outlines, remaining) = crate::x11_utils::parse_list::<Outline>(remaining, n_outlines.try_into().or(Err(ParseError::ParseError))?)?;
        let result = Shape { name, primary_ndx, approx_ndx, outlines };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Shape {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Shape {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.name.serialize_into(bytes);
        let n_outlines = u8::try_from(self.outlines.len()).expect("`outlines` has too many elements");
        n_outlines.serialize_into(bytes);
        self.primary_ndx.serialize_into(bytes);
        self.approx_ndx.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.outlines.serialize_into(bytes);
    }
}
impl Shape {
    /// Get the value of the `nOutlines` field.
    ///
    /// The `nOutlines` field is used as the length field of the `outlines` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_outlines(&self) -> u8 {
        self.outlines.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    pub name: [String8; 4],
    pub gap: i16,
    pub shape_ndx: u8,
    pub color_ndx: u8,
}
impl TryParse for Key {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let name = <[u8; 4]>::try_from(name).unwrap();
        let (gap, remaining) = i16::try_parse(remaining)?;
        let (shape_ndx, remaining) = u8::try_parse(remaining)?;
        let (color_ndx, remaining) = u8::try_parse(remaining)?;
        let result = Key { name, gap, shape_ndx, color_ndx };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Key {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Key {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let gap_bytes = self.gap.serialize();
        let shape_ndx_bytes = self.shape_ndx.serialize();
        let color_ndx_bytes = self.color_ndx.serialize();
        [
            self.name[0],
            self.name[1],
            self.name[2],
            self.name[3],
            gap_bytes[0],
            gap_bytes[1],
            shape_ndx_bytes[0],
            color_ndx_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        bytes.extend_from_slice(&self.name);
        self.gap.serialize_into(bytes);
        self.shape_ndx.serialize_into(bytes);
        self.color_ndx.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayKey {
    pub over: [String8; 4],
    pub under: [String8; 4],
}
impl TryParse for OverlayKey {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (over, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let over = <[u8; 4]>::try_from(over).unwrap();
        let (under, remaining) = crate::x11_utils::parse_u8_list(remaining, 4)?;
        let under = <[u8; 4]>::try_from(under).unwrap();
        let result = OverlayKey { over, under };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for OverlayKey {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for OverlayKey {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        [
            self.over[0],
            self.over[1],
            self.over[2],
            self.over[3],
            self.under[0],
            self.under[1],
            self.under[2],
            self.under[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        bytes.extend_from_slice(&self.over);
        bytes.extend_from_slice(&self.under);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayRow {
    pub row_under: u8,
    pub keys: Vec<OverlayKey>,
}
impl TryParse for OverlayRow {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (row_under, remaining) = u8::try_parse(remaining)?;
        let (n_keys, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (keys, remaining) = crate::x11_utils::parse_list::<OverlayKey>(remaining, n_keys.try_into().or(Err(ParseError::ParseError))?)?;
        let result = OverlayRow { row_under, keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for OverlayRow {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for OverlayRow {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.row_under.serialize_into(bytes);
        let n_keys = u8::try_from(self.keys.len()).expect("`keys` has too many elements");
        n_keys.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.keys.serialize_into(bytes);
    }
}
impl OverlayRow {
    /// Get the value of the `nKeys` field.
    ///
    /// The `nKeys` field is used as the length field of the `keys` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_keys(&self) -> u8 {
        self.keys.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overlay {
    pub name: xproto::Atom,
    pub rows: Vec<OverlayRow>,
}
impl TryParse for Overlay {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (n_rows, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (rows, remaining) = crate::x11_utils::parse_list::<OverlayRow>(remaining, n_rows.try_into().or(Err(ParseError::ParseError))?)?;
        let result = Overlay { name, rows };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Overlay {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Overlay {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.name.serialize_into(bytes);
        let n_rows = u8::try_from(self.rows.len()).expect("`rows` has too many elements");
        n_rows.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.rows.serialize_into(bytes);
    }
}
impl Overlay {
    /// Get the value of the `nRows` field.
    ///
    /// The `nRows` field is used as the length field of the `rows` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_rows(&self) -> u8 {
        self.rows.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub top: i16,
    pub left: i16,
    pub vertical: bool,
    pub keys: Vec<Key>,
}
impl TryParse for Row {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (top, remaining) = i16::try_parse(remaining)?;
        let (left, remaining) = i16::try_parse(remaining)?;
        let (n_keys, remaining) = u8::try_parse(remaining)?;
        let (vertical, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (keys, remaining) = crate::x11_utils::parse_list::<Key>(remaining, n_keys.try_into().or(Err(ParseError::ParseError))?)?;
        let result = Row { top, left, vertical, keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Row {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Row {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.top.serialize_into(bytes);
        self.left.serialize_into(bytes);
        let n_keys = u8::try_from(self.keys.len()).expect("`keys` has too many elements");
        n_keys.serialize_into(bytes);
        self.vertical.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.keys.serialize_into(bytes);
    }
}
impl Row {
    /// Get the value of the `nKeys` field.
    ///
    /// The `nKeys` field is used as the length field of the `keys` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_keys(&self) -> u8 {
        self.keys.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DoodadType {
    Outline = 1,
    Solid = 2,
    Text = 3,
    Indicator = 4,
    Logo = 5,
}
impl From<DoodadType> for u8 {
    fn from(input: DoodadType) -> Self {
        match input {
            DoodadType::Outline => 1,
            DoodadType::Solid => 2,
            DoodadType::Text => 3,
            DoodadType::Indicator => 4,
            DoodadType::Logo => 5,
        }
    }
}
impl From<DoodadType> for Option<u8> {
    fn from(input: DoodadType) -> Self {
        Some(u8::from(input))
    }
}
impl From<DoodadType> for u16 {
    fn from(input: DoodadType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DoodadType> for Option<u16> {
    fn from(input: DoodadType) -> Self {
        Some(u16::from(input))
    }
}
impl From<DoodadType> for u32 {
    fn from(input: DoodadType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<DoodadType> for Option<u32> {
    fn from(input: DoodadType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for DoodadType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DoodadType::Outline),
            2 => Ok(DoodadType::Solid),
            3 => Ok(DoodadType::Text),
            4 => Ok(DoodadType::Indicator),
            5 => Ok(DoodadType::Logo),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for DoodadType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for DoodadType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listing {
    pub flags: u16,
    pub string: Vec<String8>,
}
impl TryParse for Listing {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (flags, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u16::try_parse(remaining)?;
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, length.try_into().or(Err(ParseError::ParseError))?)?;
        let string = string.to_vec();
        // Align offset to multiple of 2
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (2 - (offset % 2)) % 2;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = Listing { flags, string };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Listing {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Listing {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.flags.serialize_into(bytes);
        let length = u16::try_from(self.string.len()).expect("`string` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&self.string);
        bytes.extend_from_slice(&[0; 1][..(2 - (bytes.len() % 2)) % 2]);
    }
}
impl Listing {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `string` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u16 {
        self.string.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceLedInfo {
    pub led_class: LedClass,
    pub led_id: IDSpec,
    pub names_present: u32,
    pub maps_present: u32,
    pub phys_indicators: u32,
    pub state: u32,
    pub names: Vec<xproto::Atom>,
    pub maps: Vec<IndicatorMap>,
}
impl TryParse for DeviceLedInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (led_class, remaining) = LedClassSpec::try_parse(remaining)?;
        let (led_id, remaining) = IDSpec::try_parse(remaining)?;
        let (names_present, remaining) = u32::try_parse(remaining)?;
        let (maps_present, remaining) = u32::try_parse(remaining)?;
        let (phys_indicators, remaining) = u32::try_parse(remaining)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        let (names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, names_present.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, maps_present.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
        let led_class = led_class.try_into()?;
        let result = DeviceLedInfo { led_class, led_id, names_present, maps_present, phys_indicators, state, names, maps };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DeviceLedInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for DeviceLedInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(20);
        LedClassSpec::from(self.led_class).serialize_into(bytes);
        self.led_id.serialize_into(bytes);
        self.names_present.serialize_into(bytes);
        self.maps_present.serialize_into(bytes);
        self.phys_indicators.serialize_into(bytes);
        self.state.serialize_into(bytes);
        assert_eq!(self.names.len(), usize::try_from(self.names_present.count_ones()).unwrap(), "`names` has an incorrect length");
        self.names.serialize_into(bytes);
        assert_eq!(self.maps.len(), usize::try_from(self.maps_present.count_ones()).unwrap(), "`maps` has an incorrect length");
        self.maps.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Error {
    BadDevice = 255,
    BadClass = 254,
    BadId = 253,
}
impl From<Error> for u8 {
    fn from(input: Error) -> Self {
        match input {
            Error::BadDevice => 255,
            Error::BadClass => 254,
            Error::BadId => 253,
        }
    }
}
impl From<Error> for Option<u8> {
    fn from(input: Error) -> Self {
        Some(u8::from(input))
    }
}
impl From<Error> for u16 {
    fn from(input: Error) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Error> for Option<u16> {
    fn from(input: Error) -> Self {
        Some(u16::from(input))
    }
}
impl From<Error> for u32 {
    fn from(input: Error) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Error> for Option<u32> {
    fn from(input: Error) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Error {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            255 => Ok(Error::BadDevice),
            254 => Ok(Error::BadClass),
            253 => Ok(Error::BadId),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Error {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Error {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the Keyboard error
pub const KEYBOARD_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl TryParse for KeyboardError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = KeyboardError { response_type, error_code, sequence, value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyboardError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&KeyboardError> for [u8; 32] {
    fn from(input: &KeyboardError) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let error_code_bytes = input.error_code.serialize();
        let sequence_bytes = input.sequence.serialize();
        let value_bytes = input.value.serialize();
        let minor_opcode_bytes = input.minor_opcode.serialize();
        let major_opcode_bytes = input.major_opcode.serialize();
        [
            response_type_bytes[0],
            error_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
            minor_opcode_bytes[0],
            minor_opcode_bytes[1],
            major_opcode_bytes[0],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
impl From<KeyboardError> for [u8; 32] {
    fn from(input: KeyboardError) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SA {
    ClearLocks,
    LatchToLock,
    UseModMapMods,
    GroupAbsolute,
}
impl From<SA> for u8 {
    fn from(input: SA) -> Self {
        match input {
            SA::ClearLocks => 1 << 0,
            SA::LatchToLock => 1 << 1,
            SA::UseModMapMods => 1 << 2,
            SA::GroupAbsolute => 1 << 2,
        }
    }
}
impl From<SA> for Option<u8> {
    fn from(input: SA) -> Self {
        Some(u8::from(input))
    }
}
impl From<SA> for u16 {
    fn from(input: SA) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SA> for Option<u16> {
    fn from(input: SA) -> Self {
        Some(u16::from(input))
    }
}
impl From<SA> for u32 {
    fn from(input: SA) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SA> for Option<u32> {
    fn from(input: SA) -> Self {
        Some(u32::from(input))
    }
}
bitmask_binop!(SA, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SAType {
    NoAction = 0,
    SetMods = 1,
    LatchMods = 2,
    LockMods = 3,
    SetGroup = 4,
    LatchGroup = 5,
    LockGroup = 6,
    MovePtr = 7,
    PtrBtn = 8,
    LockPtrBtn = 9,
    SetPtrDflt = 10,
    ISOLock = 11,
    Terminate = 12,
    SwitchScreen = 13,
    SetControls = 14,
    LockControls = 15,
    ActionMessage = 16,
    RedirectKey = 17,
    DeviceBtn = 18,
    LockDeviceBtn = 19,
    DeviceValuator = 20,
}
impl From<SAType> for u8 {
    fn from(input: SAType) -> Self {
        match input {
            SAType::NoAction => 0,
            SAType::SetMods => 1,
            SAType::LatchMods => 2,
            SAType::LockMods => 3,
            SAType::SetGroup => 4,
            SAType::LatchGroup => 5,
            SAType::LockGroup => 6,
            SAType::MovePtr => 7,
            SAType::PtrBtn => 8,
            SAType::LockPtrBtn => 9,
            SAType::SetPtrDflt => 10,
            SAType::ISOLock => 11,
            SAType::Terminate => 12,
            SAType::SwitchScreen => 13,
            SAType::SetControls => 14,
            SAType::LockControls => 15,
            SAType::ActionMessage => 16,
            SAType::RedirectKey => 17,
            SAType::DeviceBtn => 18,
            SAType::LockDeviceBtn => 19,
            SAType::DeviceValuator => 20,
        }
    }
}
impl From<SAType> for Option<u8> {
    fn from(input: SAType) -> Self {
        Some(u8::from(input))
    }
}
impl From<SAType> for u16 {
    fn from(input: SAType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAType> for Option<u16> {
    fn from(input: SAType) -> Self {
        Some(u16::from(input))
    }
}
impl From<SAType> for u32 {
    fn from(input: SAType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAType> for Option<u32> {
    fn from(input: SAType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SAType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SAType::NoAction),
            1 => Ok(SAType::SetMods),
            2 => Ok(SAType::LatchMods),
            3 => Ok(SAType::LockMods),
            4 => Ok(SAType::SetGroup),
            5 => Ok(SAType::LatchGroup),
            6 => Ok(SAType::LockGroup),
            7 => Ok(SAType::MovePtr),
            8 => Ok(SAType::PtrBtn),
            9 => Ok(SAType::LockPtrBtn),
            10 => Ok(SAType::SetPtrDflt),
            11 => Ok(SAType::ISOLock),
            12 => Ok(SAType::Terminate),
            13 => Ok(SAType::SwitchScreen),
            14 => Ok(SAType::SetControls),
            15 => Ok(SAType::LockControls),
            16 => Ok(SAType::ActionMessage),
            17 => Ok(SAType::RedirectKey),
            18 => Ok(SAType::DeviceBtn),
            19 => Ok(SAType::LockDeviceBtn),
            20 => Ok(SAType::DeviceValuator),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SAType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SAType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SANoAction {
    pub type_: SAType,
}
impl TryParse for SANoAction {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(7..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SANoAction { type_ };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SANoAction {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SANoAction {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        [
            type_bytes[0],
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
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 7]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SASetMods {
    pub type_: SAType,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}
impl TryParse for SASetMods {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (mask, remaining) = u8::try_parse(remaining)?;
        let (real_mods, remaining) = u8::try_parse(remaining)?;
        let (vmods_high, remaining) = u8::try_parse(remaining)?;
        let (vmods_low, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SASetMods { type_, flags, mask, real_mods, vmods_high, vmods_low };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SASetMods {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SASetMods {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let mask_bytes = self.mask.serialize();
        let real_mods_bytes = self.real_mods.serialize();
        let vmods_high_bytes = self.vmods_high.serialize();
        let vmods_low_bytes = self.vmods_low.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            mask_bytes[0],
            real_mods_bytes[0],
            vmods_high_bytes[0],
            vmods_low_bytes[0],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.mask.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.vmods_high.serialize_into(bytes);
        self.vmods_low.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

pub type SALatchMods = SASetMods;

pub type SALockMods = SASetMods;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SASetGroup {
    pub type_: SAType,
    pub flags: u8,
    pub group: i8,
}
impl TryParse for SASetGroup {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = i8::try_parse(remaining)?;
        let remaining = remaining.get(5..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SASetGroup { type_, flags, group };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SASetGroup {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SASetGroup {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let group_bytes = self.group.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            group_bytes[0],
            0,
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.group.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 5]);
    }
}

pub type SALatchGroup = SASetGroup;

pub type SALockGroup = SASetGroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMovePtrFlag {
    NoAcceleration = 1 << 0,
    MoveAbsoluteX = 1 << 1,
    MoveAbsoluteY = 1 << 2,
}
impl From<SAMovePtrFlag> for u8 {
    fn from(input: SAMovePtrFlag) -> Self {
        match input {
            SAMovePtrFlag::NoAcceleration => 1 << 0,
            SAMovePtrFlag::MoveAbsoluteX => 1 << 1,
            SAMovePtrFlag::MoveAbsoluteY => 1 << 2,
        }
    }
}
impl From<SAMovePtrFlag> for Option<u8> {
    fn from(input: SAMovePtrFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<SAMovePtrFlag> for u16 {
    fn from(input: SAMovePtrFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAMovePtrFlag> for Option<u16> {
    fn from(input: SAMovePtrFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<SAMovePtrFlag> for u32 {
    fn from(input: SAMovePtrFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAMovePtrFlag> for Option<u32> {
    fn from(input: SAMovePtrFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SAMovePtrFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SAMovePtrFlag::NoAcceleration),
            2 => Ok(SAMovePtrFlag::MoveAbsoluteX),
            4 => Ok(SAMovePtrFlag::MoveAbsoluteY),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SAMovePtrFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SAMovePtrFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(SAMovePtrFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SAMovePtr {
    pub type_: SAType,
    pub flags: u8,
    pub x_high: i8,
    pub x_low: u8,
    pub y_high: i8,
    pub y_low: u8,
}
impl TryParse for SAMovePtr {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (x_high, remaining) = i8::try_parse(remaining)?;
        let (x_low, remaining) = u8::try_parse(remaining)?;
        let (y_high, remaining) = i8::try_parse(remaining)?;
        let (y_low, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SAMovePtr { type_, flags, x_high, x_low, y_high, y_low };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SAMovePtr {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SAMovePtr {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let x_high_bytes = self.x_high.serialize();
        let x_low_bytes = self.x_low.serialize();
        let y_high_bytes = self.y_high.serialize();
        let y_low_bytes = self.y_low.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            x_high_bytes[0],
            x_low_bytes[0],
            y_high_bytes[0],
            y_low_bytes[0],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.x_high.serialize_into(bytes);
        self.x_low.serialize_into(bytes);
        self.y_high.serialize_into(bytes);
        self.y_low.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SAPtrBtn {
    pub type_: SAType,
    pub flags: u8,
    pub count: u8,
    pub button: u8,
}
impl TryParse for SAPtrBtn {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (count, remaining) = u8::try_parse(remaining)?;
        let (button, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SAPtrBtn { type_, flags, count, button };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SAPtrBtn {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SAPtrBtn {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let count_bytes = self.count.serialize();
        let button_bytes = self.button.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            count_bytes[0],
            button_bytes[0],
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.count.serialize_into(bytes);
        self.button.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SALockPtrBtn {
    pub type_: SAType,
    pub flags: u8,
    pub button: u8,
}
impl TryParse for SALockPtrBtn {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (button, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SALockPtrBtn { type_, flags, button };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SALockPtrBtn {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SALockPtrBtn {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let button_bytes = self.button.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            0,
            button_bytes[0],
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.button.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SASetPtrDfltFlag {
    DfltBtnAbsolute = 1 << 2,
    AffectDfltButton = 1 << 0,
}
impl From<SASetPtrDfltFlag> for u8 {
    fn from(input: SASetPtrDfltFlag) -> Self {
        match input {
            SASetPtrDfltFlag::DfltBtnAbsolute => 1 << 2,
            SASetPtrDfltFlag::AffectDfltButton => 1 << 0,
        }
    }
}
impl From<SASetPtrDfltFlag> for Option<u8> {
    fn from(input: SASetPtrDfltFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<SASetPtrDfltFlag> for u16 {
    fn from(input: SASetPtrDfltFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SASetPtrDfltFlag> for Option<u16> {
    fn from(input: SASetPtrDfltFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<SASetPtrDfltFlag> for u32 {
    fn from(input: SASetPtrDfltFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SASetPtrDfltFlag> for Option<u32> {
    fn from(input: SASetPtrDfltFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SASetPtrDfltFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(SASetPtrDfltFlag::DfltBtnAbsolute),
            1 => Ok(SASetPtrDfltFlag::AffectDfltButton),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SASetPtrDfltFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SASetPtrDfltFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(SASetPtrDfltFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SASetPtrDflt {
    pub type_: SAType,
    pub flags: u8,
    pub affect: u8,
    pub value: i8,
}
impl TryParse for SASetPtrDflt {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (affect, remaining) = u8::try_parse(remaining)?;
        let (value, remaining) = i8::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SASetPtrDflt { type_, flags, affect, value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SASetPtrDflt {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SASetPtrDflt {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let affect_bytes = self.affect.serialize();
        let value_bytes = self.value.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            affect_bytes[0],
            value_bytes[0],
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.affect.serialize_into(bytes);
        self.value.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SAIsoLockFlag {
    NoLock,
    NoUnlock,
    UseModMapMods,
    GroupAbsolute,
    ISODfltIsGroup,
}
impl From<SAIsoLockFlag> for u8 {
    fn from(input: SAIsoLockFlag) -> Self {
        match input {
            SAIsoLockFlag::NoLock => 1 << 0,
            SAIsoLockFlag::NoUnlock => 1 << 1,
            SAIsoLockFlag::UseModMapMods => 1 << 2,
            SAIsoLockFlag::GroupAbsolute => 1 << 2,
            SAIsoLockFlag::ISODfltIsGroup => 1 << 3,
        }
    }
}
impl From<SAIsoLockFlag> for Option<u8> {
    fn from(input: SAIsoLockFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<SAIsoLockFlag> for u16 {
    fn from(input: SAIsoLockFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAIsoLockFlag> for Option<u16> {
    fn from(input: SAIsoLockFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<SAIsoLockFlag> for u32 {
    fn from(input: SAIsoLockFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAIsoLockFlag> for Option<u32> {
    fn from(input: SAIsoLockFlag) -> Self {
        Some(u32::from(input))
    }
}
bitmask_binop!(SAIsoLockFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SAIsoLockNoAffect {
    Ctrls = 1 << 3,
    Ptr = 1 << 4,
    Group = 1 << 5,
    Mods = 1 << 6,
}
impl From<SAIsoLockNoAffect> for u8 {
    fn from(input: SAIsoLockNoAffect) -> Self {
        match input {
            SAIsoLockNoAffect::Ctrls => 1 << 3,
            SAIsoLockNoAffect::Ptr => 1 << 4,
            SAIsoLockNoAffect::Group => 1 << 5,
            SAIsoLockNoAffect::Mods => 1 << 6,
        }
    }
}
impl From<SAIsoLockNoAffect> for Option<u8> {
    fn from(input: SAIsoLockNoAffect) -> Self {
        Some(u8::from(input))
    }
}
impl From<SAIsoLockNoAffect> for u16 {
    fn from(input: SAIsoLockNoAffect) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAIsoLockNoAffect> for Option<u16> {
    fn from(input: SAIsoLockNoAffect) -> Self {
        Some(u16::from(input))
    }
}
impl From<SAIsoLockNoAffect> for u32 {
    fn from(input: SAIsoLockNoAffect) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAIsoLockNoAffect> for Option<u32> {
    fn from(input: SAIsoLockNoAffect) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SAIsoLockNoAffect {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            8 => Ok(SAIsoLockNoAffect::Ctrls),
            16 => Ok(SAIsoLockNoAffect::Ptr),
            32 => Ok(SAIsoLockNoAffect::Group),
            64 => Ok(SAIsoLockNoAffect::Mods),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SAIsoLockNoAffect {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SAIsoLockNoAffect {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(SAIsoLockNoAffect, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SAIsoLock {
    pub type_: SAType,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub group: i8,
    pub affect: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}
impl TryParse for SAIsoLock {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (mask, remaining) = u8::try_parse(remaining)?;
        let (real_mods, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = i8::try_parse(remaining)?;
        let (affect, remaining) = u8::try_parse(remaining)?;
        let (vmods_high, remaining) = u8::try_parse(remaining)?;
        let (vmods_low, remaining) = u8::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let result = SAIsoLock { type_, flags, mask, real_mods, group, affect, vmods_high, vmods_low };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SAIsoLock {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SAIsoLock {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let mask_bytes = self.mask.serialize();
        let real_mods_bytes = self.real_mods.serialize();
        let group_bytes = self.group.serialize();
        let affect_bytes = self.affect.serialize();
        let vmods_high_bytes = self.vmods_high.serialize();
        let vmods_low_bytes = self.vmods_low.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            mask_bytes[0],
            real_mods_bytes[0],
            group_bytes[0],
            affect_bytes[0],
            vmods_high_bytes[0],
            vmods_low_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.mask.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.group.serialize_into(bytes);
        self.affect.serialize_into(bytes);
        self.vmods_high.serialize_into(bytes);
        self.vmods_low.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SATerminate {
    pub type_: SAType,
}
impl TryParse for SATerminate {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(7..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SATerminate { type_ };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SATerminate {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SATerminate {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        [
            type_bytes[0],
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
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 7]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SwitchScreenFlag {
    Application = 1 << 0,
    Absolute = 1 << 2,
}
impl From<SwitchScreenFlag> for u8 {
    fn from(input: SwitchScreenFlag) -> Self {
        match input {
            SwitchScreenFlag::Application => 1 << 0,
            SwitchScreenFlag::Absolute => 1 << 2,
        }
    }
}
impl From<SwitchScreenFlag> for Option<u8> {
    fn from(input: SwitchScreenFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<SwitchScreenFlag> for u16 {
    fn from(input: SwitchScreenFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SwitchScreenFlag> for Option<u16> {
    fn from(input: SwitchScreenFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<SwitchScreenFlag> for u32 {
    fn from(input: SwitchScreenFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SwitchScreenFlag> for Option<u32> {
    fn from(input: SwitchScreenFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SwitchScreenFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SwitchScreenFlag::Application),
            4 => Ok(SwitchScreenFlag::Absolute),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SwitchScreenFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SwitchScreenFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(SwitchScreenFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SASwitchScreen {
    pub type_: SAType,
    pub flags: u8,
    pub new_screen: i8,
}
impl TryParse for SASwitchScreen {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (new_screen, remaining) = i8::try_parse(remaining)?;
        let remaining = remaining.get(5..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SASwitchScreen { type_, flags, new_screen };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SASwitchScreen {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SASwitchScreen {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let new_screen_bytes = self.new_screen.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            new_screen_bytes[0],
            0,
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.new_screen.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 5]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BoolCtrlsHigh {
    AccessXFeedback = 1 << 0,
    AudibleBell = 1 << 1,
    Overlay1 = 1 << 2,
    Overlay2 = 1 << 3,
    IgnoreGroupLock = 1 << 4,
}
impl From<BoolCtrlsHigh> for u8 {
    fn from(input: BoolCtrlsHigh) -> Self {
        match input {
            BoolCtrlsHigh::AccessXFeedback => 1 << 0,
            BoolCtrlsHigh::AudibleBell => 1 << 1,
            BoolCtrlsHigh::Overlay1 => 1 << 2,
            BoolCtrlsHigh::Overlay2 => 1 << 3,
            BoolCtrlsHigh::IgnoreGroupLock => 1 << 4,
        }
    }
}
impl From<BoolCtrlsHigh> for Option<u8> {
    fn from(input: BoolCtrlsHigh) -> Self {
        Some(u8::from(input))
    }
}
impl From<BoolCtrlsHigh> for u16 {
    fn from(input: BoolCtrlsHigh) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BoolCtrlsHigh> for Option<u16> {
    fn from(input: BoolCtrlsHigh) -> Self {
        Some(u16::from(input))
    }
}
impl From<BoolCtrlsHigh> for u32 {
    fn from(input: BoolCtrlsHigh) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BoolCtrlsHigh> for Option<u32> {
    fn from(input: BoolCtrlsHigh) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BoolCtrlsHigh {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BoolCtrlsHigh::AccessXFeedback),
            2 => Ok(BoolCtrlsHigh::AudibleBell),
            4 => Ok(BoolCtrlsHigh::Overlay1),
            8 => Ok(BoolCtrlsHigh::Overlay2),
            16 => Ok(BoolCtrlsHigh::IgnoreGroupLock),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for BoolCtrlsHigh {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for BoolCtrlsHigh {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(BoolCtrlsHigh, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BoolCtrlsLow {
    RepeatKeys = 1 << 0,
    SlowKeys = 1 << 1,
    BounceKeys = 1 << 2,
    StickyKeys = 1 << 3,
    MouseKeys = 1 << 4,
    MouseKeysAccel = 1 << 5,
    AccessXKeys = 1 << 6,
    AccessXTimeout = 1 << 7,
}
impl From<BoolCtrlsLow> for u8 {
    fn from(input: BoolCtrlsLow) -> Self {
        match input {
            BoolCtrlsLow::RepeatKeys => 1 << 0,
            BoolCtrlsLow::SlowKeys => 1 << 1,
            BoolCtrlsLow::BounceKeys => 1 << 2,
            BoolCtrlsLow::StickyKeys => 1 << 3,
            BoolCtrlsLow::MouseKeys => 1 << 4,
            BoolCtrlsLow::MouseKeysAccel => 1 << 5,
            BoolCtrlsLow::AccessXKeys => 1 << 6,
            BoolCtrlsLow::AccessXTimeout => 1 << 7,
        }
    }
}
impl From<BoolCtrlsLow> for Option<u8> {
    fn from(input: BoolCtrlsLow) -> Self {
        Some(u8::from(input))
    }
}
impl From<BoolCtrlsLow> for u16 {
    fn from(input: BoolCtrlsLow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BoolCtrlsLow> for Option<u16> {
    fn from(input: BoolCtrlsLow) -> Self {
        Some(u16::from(input))
    }
}
impl From<BoolCtrlsLow> for u32 {
    fn from(input: BoolCtrlsLow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BoolCtrlsLow> for Option<u32> {
    fn from(input: BoolCtrlsLow) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BoolCtrlsLow {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BoolCtrlsLow::RepeatKeys),
            2 => Ok(BoolCtrlsLow::SlowKeys),
            4 => Ok(BoolCtrlsLow::BounceKeys),
            8 => Ok(BoolCtrlsLow::StickyKeys),
            16 => Ok(BoolCtrlsLow::MouseKeys),
            32 => Ok(BoolCtrlsLow::MouseKeysAccel),
            64 => Ok(BoolCtrlsLow::AccessXKeys),
            128 => Ok(BoolCtrlsLow::AccessXTimeout),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for BoolCtrlsLow {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for BoolCtrlsLow {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(BoolCtrlsLow, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SASetControls {
    pub type_: SAType,
    pub bool_ctrls_high: u8,
    pub bool_ctrls_low: u8,
}
impl TryParse for SASetControls {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (bool_ctrls_high, remaining) = u8::try_parse(remaining)?;
        let (bool_ctrls_low, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SASetControls { type_, bool_ctrls_high, bool_ctrls_low };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SASetControls {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SASetControls {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let bool_ctrls_high_bytes = self.bool_ctrls_high.serialize();
        let bool_ctrls_low_bytes = self.bool_ctrls_low.serialize();
        [
            type_bytes[0],
            0,
            0,
            0,
            bool_ctrls_high_bytes[0],
            bool_ctrls_low_bytes[0],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.bool_ctrls_high.serialize_into(bytes);
        self.bool_ctrls_low.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

pub type SALockControls = SASetControls;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ActionMessageFlag {
    OnPress = 1 << 0,
    OnRelease = 1 << 1,
    GenKeyEvent = 1 << 2,
}
impl From<ActionMessageFlag> for u8 {
    fn from(input: ActionMessageFlag) -> Self {
        match input {
            ActionMessageFlag::OnPress => 1 << 0,
            ActionMessageFlag::OnRelease => 1 << 1,
            ActionMessageFlag::GenKeyEvent => 1 << 2,
        }
    }
}
impl From<ActionMessageFlag> for Option<u8> {
    fn from(input: ActionMessageFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<ActionMessageFlag> for u16 {
    fn from(input: ActionMessageFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ActionMessageFlag> for Option<u16> {
    fn from(input: ActionMessageFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<ActionMessageFlag> for u32 {
    fn from(input: ActionMessageFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ActionMessageFlag> for Option<u32> {
    fn from(input: ActionMessageFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ActionMessageFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ActionMessageFlag::OnPress),
            2 => Ok(ActionMessageFlag::OnRelease),
            4 => Ok(ActionMessageFlag::GenKeyEvent),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ActionMessageFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ActionMessageFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ActionMessageFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SAActionMessage {
    pub type_: SAType,
    pub flags: u8,
    pub message: [u8; 6],
}
impl TryParse for SAActionMessage {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (message, remaining) = crate::x11_utils::parse_u8_list(remaining, 6)?;
        let message = <[u8; 6]>::try_from(message).unwrap();
        let type_ = type_.try_into()?;
        let result = SAActionMessage { type_, flags, message };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SAActionMessage {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SAActionMessage {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            self.message[0],
            self.message[1],
            self.message[2],
            self.message[3],
            self.message[4],
            self.message[5],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&self.message);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SARedirectKey {
    pub type_: SAType,
    pub newkey: xproto::Keycode,
    pub mask: u8,
    pub real_modifiers: u8,
    pub vmods_mask_high: u8,
    pub vmods_mask_low: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}
impl TryParse for SARedirectKey {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (newkey, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (mask, remaining) = u8::try_parse(remaining)?;
        let (real_modifiers, remaining) = u8::try_parse(remaining)?;
        let (vmods_mask_high, remaining) = u8::try_parse(remaining)?;
        let (vmods_mask_low, remaining) = u8::try_parse(remaining)?;
        let (vmods_high, remaining) = u8::try_parse(remaining)?;
        let (vmods_low, remaining) = u8::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let result = SARedirectKey { type_, newkey, mask, real_modifiers, vmods_mask_high, vmods_mask_low, vmods_high, vmods_low };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SARedirectKey {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SARedirectKey {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let newkey_bytes = self.newkey.serialize();
        let mask_bytes = self.mask.serialize();
        let real_modifiers_bytes = self.real_modifiers.serialize();
        let vmods_mask_high_bytes = self.vmods_mask_high.serialize();
        let vmods_mask_low_bytes = self.vmods_mask_low.serialize();
        let vmods_high_bytes = self.vmods_high.serialize();
        let vmods_low_bytes = self.vmods_low.serialize();
        [
            type_bytes[0],
            newkey_bytes[0],
            mask_bytes[0],
            real_modifiers_bytes[0],
            vmods_mask_high_bytes[0],
            vmods_mask_low_bytes[0],
            vmods_high_bytes[0],
            vmods_low_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.newkey.serialize_into(bytes);
        self.mask.serialize_into(bytes);
        self.real_modifiers.serialize_into(bytes);
        self.vmods_mask_high.serialize_into(bytes);
        self.vmods_mask_low.serialize_into(bytes);
        self.vmods_high.serialize_into(bytes);
        self.vmods_low.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SADeviceBtn {
    pub type_: SAType,
    pub flags: u8,
    pub count: u8,
    pub button: u8,
    pub device: u8,
}
impl TryParse for SADeviceBtn {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (count, remaining) = u8::try_parse(remaining)?;
        let (button, remaining) = u8::try_parse(remaining)?;
        let (device, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SADeviceBtn { type_, flags, count, button, device };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SADeviceBtn {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SADeviceBtn {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let count_bytes = self.count.serialize();
        let button_bytes = self.button.serialize();
        let device_bytes = self.device.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            count_bytes[0],
            button_bytes[0],
            device_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.count.serialize_into(bytes);
        self.button.serialize_into(bytes);
        self.device.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LockDeviceFlags {
    NoLock = 1 << 0,
    NoUnlock = 1 << 1,
}
impl From<LockDeviceFlags> for u8 {
    fn from(input: LockDeviceFlags) -> Self {
        match input {
            LockDeviceFlags::NoLock => 1 << 0,
            LockDeviceFlags::NoUnlock => 1 << 1,
        }
    }
}
impl From<LockDeviceFlags> for Option<u8> {
    fn from(input: LockDeviceFlags) -> Self {
        Some(u8::from(input))
    }
}
impl From<LockDeviceFlags> for u16 {
    fn from(input: LockDeviceFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<LockDeviceFlags> for Option<u16> {
    fn from(input: LockDeviceFlags) -> Self {
        Some(u16::from(input))
    }
}
impl From<LockDeviceFlags> for u32 {
    fn from(input: LockDeviceFlags) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<LockDeviceFlags> for Option<u32> {
    fn from(input: LockDeviceFlags) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for LockDeviceFlags {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(LockDeviceFlags::NoLock),
            2 => Ok(LockDeviceFlags::NoUnlock),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for LockDeviceFlags {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for LockDeviceFlags {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(LockDeviceFlags, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SALockDeviceBtn {
    pub type_: SAType,
    pub flags: u8,
    pub button: u8,
    pub device: u8,
}
impl TryParse for SALockDeviceBtn {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (button, remaining) = u8::try_parse(remaining)?;
        let (device, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SALockDeviceBtn { type_, flags, button, device };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SALockDeviceBtn {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SALockDeviceBtn {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let flags_bytes = self.flags.serialize();
        let button_bytes = self.button.serialize();
        let device_bytes = self.device.serialize();
        [
            type_bytes[0],
            flags_bytes[0],
            0,
            button_bytes[0],
            device_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.button.serialize_into(bytes);
        self.device.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SAValWhat {
    IgnoreVal = 0,
    SetValMin = 1,
    SetValCenter = 2,
    SetValMax = 3,
    SetValRelative = 4,
    SetValAbsolute = 5,
}
impl From<SAValWhat> for u8 {
    fn from(input: SAValWhat) -> Self {
        match input {
            SAValWhat::IgnoreVal => 0,
            SAValWhat::SetValMin => 1,
            SAValWhat::SetValCenter => 2,
            SAValWhat::SetValMax => 3,
            SAValWhat::SetValRelative => 4,
            SAValWhat::SetValAbsolute => 5,
        }
    }
}
impl From<SAValWhat> for Option<u8> {
    fn from(input: SAValWhat) -> Self {
        Some(u8::from(input))
    }
}
impl From<SAValWhat> for u16 {
    fn from(input: SAValWhat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAValWhat> for Option<u16> {
    fn from(input: SAValWhat) -> Self {
        Some(u16::from(input))
    }
}
impl From<SAValWhat> for u32 {
    fn from(input: SAValWhat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SAValWhat> for Option<u32> {
    fn from(input: SAValWhat) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SAValWhat {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SAValWhat::IgnoreVal),
            1 => Ok(SAValWhat::SetValMin),
            2 => Ok(SAValWhat::SetValCenter),
            3 => Ok(SAValWhat::SetValMax),
            4 => Ok(SAValWhat::SetValRelative),
            5 => Ok(SAValWhat::SetValAbsolute),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SAValWhat {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SAValWhat {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SADeviceValuator {
    pub type_: SAType,
    pub device: u8,
    pub val1what: SAValWhat,
    pub val1index: u8,
    pub val1value: u8,
    pub val2what: SAValWhat,
    pub val2index: u8,
    pub val2value: u8,
}
impl TryParse for SADeviceValuator {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (device, remaining) = u8::try_parse(remaining)?;
        let (val1what, remaining) = u8::try_parse(remaining)?;
        let (val1index, remaining) = u8::try_parse(remaining)?;
        let (val1value, remaining) = u8::try_parse(remaining)?;
        let (val2what, remaining) = u8::try_parse(remaining)?;
        let (val2index, remaining) = u8::try_parse(remaining)?;
        let (val2value, remaining) = u8::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let val1what = val1what.try_into()?;
        let val2what = val2what.try_into()?;
        let result = SADeviceValuator { type_, device, val1what, val1index, val1value, val2what, val2index, val2value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SADeviceValuator {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SADeviceValuator {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        let device_bytes = self.device.serialize();
        let val1what_bytes = u8::from(self.val1what).serialize();
        let val1index_bytes = self.val1index.serialize();
        let val1value_bytes = self.val1value.serialize();
        let val2what_bytes = u8::from(self.val2what).serialize();
        let val2index_bytes = self.val2index.serialize();
        let val2value_bytes = self.val2value.serialize();
        [
            type_bytes[0],
            device_bytes[0],
            val1what_bytes[0],
            val1index_bytes[0],
            val1value_bytes[0],
            val2what_bytes[0],
            val2index_bytes[0],
            val2value_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        self.device.serialize_into(bytes);
        u8::from(self.val1what).serialize_into(bytes);
        self.val1index.serialize_into(bytes);
        self.val1value.serialize_into(bytes);
        u8::from(self.val2what).serialize_into(bytes);
        self.val2index.serialize_into(bytes);
        self.val2value.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SIAction {
    pub type_: SAType,
    pub data: [u8; 7],
}
impl TryParse for SIAction {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, 7)?;
        let data = <[u8; 7]>::try_from(data).unwrap();
        let type_ = type_.try_into()?;
        let result = SIAction { type_, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SIAction {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SIAction {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let type_bytes = u8::from(self.type_).serialize();
        [
            type_bytes[0],
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5],
            self.data[6],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        u8::from(self.type_).serialize_into(bytes);
        bytes.extend_from_slice(&self.data);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymInterpret {
    pub sym: xproto::Keysym,
    pub mods: u8,
    pub match_: u8,
    pub virtual_mod: u8,
    pub flags: u8,
    pub action: SIAction,
}
impl TryParse for SymInterpret {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (sym, remaining) = xproto::Keysym::try_parse(remaining)?;
        let (mods, remaining) = u8::try_parse(remaining)?;
        let (match_, remaining) = u8::try_parse(remaining)?;
        let (virtual_mod, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (action, remaining) = SIAction::try_parse(remaining)?;
        let result = SymInterpret { sym, mods, match_, virtual_mod, flags, action };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SymInterpret {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SymInterpret {
    type Bytes = [u8; 16];
    fn serialize(&self) -> [u8; 16] {
        let sym_bytes = self.sym.serialize();
        let mods_bytes = self.mods.serialize();
        let match_bytes = self.match_.serialize();
        let virtual_mod_bytes = self.virtual_mod.serialize();
        let flags_bytes = self.flags.serialize();
        let action_bytes = self.action.serialize();
        [
            sym_bytes[0],
            sym_bytes[1],
            sym_bytes[2],
            sym_bytes[3],
            mods_bytes[0],
            match_bytes[0],
            virtual_mod_bytes[0],
            flags_bytes[0],
            action_bytes[0],
            action_bytes[1],
            action_bytes[2],
            action_bytes[3],
            action_bytes[4],
            action_bytes[5],
            action_bytes[6],
            action_bytes[7],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.sym.serialize_into(bytes);
        self.mods.serialize_into(bytes);
        self.match_.serialize_into(bytes);
        self.virtual_mod.serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.action.serialize_into(bytes);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Action([u8; 8]);
impl Action {
    pub fn as_noaction(&self) -> SANoAction {
        fn do_the_parse(remaining: &[u8]) -> Result<SANoAction, ParseError> {
            let (noaction, remaining) = SANoAction::try_parse(remaining)?;
            let _ = remaining;
            Ok(noaction)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_setmods(&self) -> SASetMods {
        fn do_the_parse(remaining: &[u8]) -> Result<SASetMods, ParseError> {
            let (setmods, remaining) = SASetMods::try_parse(remaining)?;
            let _ = remaining;
            Ok(setmods)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_latchmods(&self) -> SALatchMods {
        fn do_the_parse(remaining: &[u8]) -> Result<SALatchMods, ParseError> {
            let (latchmods, remaining) = SALatchMods::try_parse(remaining)?;
            let _ = remaining;
            Ok(latchmods)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_lockmods(&self) -> SALockMods {
        fn do_the_parse(remaining: &[u8]) -> Result<SALockMods, ParseError> {
            let (lockmods, remaining) = SALockMods::try_parse(remaining)?;
            let _ = remaining;
            Ok(lockmods)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_setgroup(&self) -> SASetGroup {
        fn do_the_parse(remaining: &[u8]) -> Result<SASetGroup, ParseError> {
            let (setgroup, remaining) = SASetGroup::try_parse(remaining)?;
            let _ = remaining;
            Ok(setgroup)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_latchgroup(&self) -> SALatchGroup {
        fn do_the_parse(remaining: &[u8]) -> Result<SALatchGroup, ParseError> {
            let (latchgroup, remaining) = SALatchGroup::try_parse(remaining)?;
            let _ = remaining;
            Ok(latchgroup)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_lockgroup(&self) -> SALockGroup {
        fn do_the_parse(remaining: &[u8]) -> Result<SALockGroup, ParseError> {
            let (lockgroup, remaining) = SALockGroup::try_parse(remaining)?;
            let _ = remaining;
            Ok(lockgroup)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_moveptr(&self) -> SAMovePtr {
        fn do_the_parse(remaining: &[u8]) -> Result<SAMovePtr, ParseError> {
            let (moveptr, remaining) = SAMovePtr::try_parse(remaining)?;
            let _ = remaining;
            Ok(moveptr)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_ptrbtn(&self) -> SAPtrBtn {
        fn do_the_parse(remaining: &[u8]) -> Result<SAPtrBtn, ParseError> {
            let (ptrbtn, remaining) = SAPtrBtn::try_parse(remaining)?;
            let _ = remaining;
            Ok(ptrbtn)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_lockptrbtn(&self) -> SALockPtrBtn {
        fn do_the_parse(remaining: &[u8]) -> Result<SALockPtrBtn, ParseError> {
            let (lockptrbtn, remaining) = SALockPtrBtn::try_parse(remaining)?;
            let _ = remaining;
            Ok(lockptrbtn)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_setptrdflt(&self) -> SASetPtrDflt {
        fn do_the_parse(remaining: &[u8]) -> Result<SASetPtrDflt, ParseError> {
            let (setptrdflt, remaining) = SASetPtrDflt::try_parse(remaining)?;
            let _ = remaining;
            Ok(setptrdflt)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_isolock(&self) -> SAIsoLock {
        fn do_the_parse(remaining: &[u8]) -> Result<SAIsoLock, ParseError> {
            let (isolock, remaining) = SAIsoLock::try_parse(remaining)?;
            let _ = remaining;
            Ok(isolock)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_terminate(&self) -> SATerminate {
        fn do_the_parse(remaining: &[u8]) -> Result<SATerminate, ParseError> {
            let (terminate, remaining) = SATerminate::try_parse(remaining)?;
            let _ = remaining;
            Ok(terminate)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_switchscreen(&self) -> SASwitchScreen {
        fn do_the_parse(remaining: &[u8]) -> Result<SASwitchScreen, ParseError> {
            let (switchscreen, remaining) = SASwitchScreen::try_parse(remaining)?;
            let _ = remaining;
            Ok(switchscreen)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_setcontrols(&self) -> SASetControls {
        fn do_the_parse(remaining: &[u8]) -> Result<SASetControls, ParseError> {
            let (setcontrols, remaining) = SASetControls::try_parse(remaining)?;
            let _ = remaining;
            Ok(setcontrols)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_lockcontrols(&self) -> SALockControls {
        fn do_the_parse(remaining: &[u8]) -> Result<SALockControls, ParseError> {
            let (lockcontrols, remaining) = SALockControls::try_parse(remaining)?;
            let _ = remaining;
            Ok(lockcontrols)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_message(&self) -> SAActionMessage {
        fn do_the_parse(remaining: &[u8]) -> Result<SAActionMessage, ParseError> {
            let (message, remaining) = SAActionMessage::try_parse(remaining)?;
            let _ = remaining;
            Ok(message)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_redirect(&self) -> SARedirectKey {
        fn do_the_parse(remaining: &[u8]) -> Result<SARedirectKey, ParseError> {
            let (redirect, remaining) = SARedirectKey::try_parse(remaining)?;
            let _ = remaining;
            Ok(redirect)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_devbtn(&self) -> SADeviceBtn {
        fn do_the_parse(remaining: &[u8]) -> Result<SADeviceBtn, ParseError> {
            let (devbtn, remaining) = SADeviceBtn::try_parse(remaining)?;
            let _ = remaining;
            Ok(devbtn)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_lockdevbtn(&self) -> SALockDeviceBtn {
        fn do_the_parse(remaining: &[u8]) -> Result<SALockDeviceBtn, ParseError> {
            let (lockdevbtn, remaining) = SALockDeviceBtn::try_parse(remaining)?;
            let _ = remaining;
            Ok(lockdevbtn)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_devval(&self) -> SADeviceValuator {
        fn do_the_parse(remaining: &[u8]) -> Result<SADeviceValuator, ParseError> {
            let (devval, remaining) = SADeviceValuator::try_parse(remaining)?;
            let _ = remaining;
            Ok(devval)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_type(&self) -> SAType {
        fn do_the_parse(remaining: &[u8]) -> Result<SAType, ParseError> {
            let (type_, remaining) = u8::try_parse(remaining)?;
            let type_ = type_.try_into()?;
            let _ = remaining;
            Ok(type_)
        }
        do_the_parse(&self.0).unwrap()
    }
}
impl Serialize for Action {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        self.0
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}
impl TryParse for Action {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner: [u8; 8] = value.get(..8)
            .ok_or(ParseError::ParseError)?
            .try_into()
            .unwrap();
        let result = Action(inner);
        Ok((result, &value[8..]))
    }
}
impl From<SANoAction> for Action {
    fn from(noaction: SANoAction) -> Self {
        let noaction_bytes = noaction.serialize();
        Self(noaction_bytes)
    }
}
impl From<SASetMods> for Action {
    fn from(setmods: SASetMods) -> Self {
        let setmods_bytes = setmods.serialize();
        Self(setmods_bytes)
    }
}
impl From<SASetGroup> for Action {
    fn from(setgroup: SASetGroup) -> Self {
        let setgroup_bytes = setgroup.serialize();
        Self(setgroup_bytes)
    }
}
impl From<SAMovePtr> for Action {
    fn from(moveptr: SAMovePtr) -> Self {
        let moveptr_bytes = moveptr.serialize();
        Self(moveptr_bytes)
    }
}
impl From<SAPtrBtn> for Action {
    fn from(ptrbtn: SAPtrBtn) -> Self {
        let ptrbtn_bytes = ptrbtn.serialize();
        Self(ptrbtn_bytes)
    }
}
impl From<SALockPtrBtn> for Action {
    fn from(lockptrbtn: SALockPtrBtn) -> Self {
        let lockptrbtn_bytes = lockptrbtn.serialize();
        Self(lockptrbtn_bytes)
    }
}
impl From<SASetPtrDflt> for Action {
    fn from(setptrdflt: SASetPtrDflt) -> Self {
        let setptrdflt_bytes = setptrdflt.serialize();
        Self(setptrdflt_bytes)
    }
}
impl From<SAIsoLock> for Action {
    fn from(isolock: SAIsoLock) -> Self {
        let isolock_bytes = isolock.serialize();
        Self(isolock_bytes)
    }
}
impl From<SATerminate> for Action {
    fn from(terminate: SATerminate) -> Self {
        let terminate_bytes = terminate.serialize();
        Self(terminate_bytes)
    }
}
impl From<SASwitchScreen> for Action {
    fn from(switchscreen: SASwitchScreen) -> Self {
        let switchscreen_bytes = switchscreen.serialize();
        Self(switchscreen_bytes)
    }
}
impl From<SASetControls> for Action {
    fn from(setcontrols: SASetControls) -> Self {
        let setcontrols_bytes = setcontrols.serialize();
        Self(setcontrols_bytes)
    }
}
impl From<SAActionMessage> for Action {
    fn from(message: SAActionMessage) -> Self {
        let message_bytes = message.serialize();
        Self(message_bytes)
    }
}
impl From<SARedirectKey> for Action {
    fn from(redirect: SARedirectKey) -> Self {
        let redirect_bytes = redirect.serialize();
        Self(redirect_bytes)
    }
}
impl From<SADeviceBtn> for Action {
    fn from(devbtn: SADeviceBtn) -> Self {
        let devbtn_bytes = devbtn.serialize();
        Self(devbtn_bytes)
    }
}
impl From<SALockDeviceBtn> for Action {
    fn from(lockdevbtn: SALockDeviceBtn) -> Self {
        let lockdevbtn_bytes = lockdevbtn.serialize();
        Self(lockdevbtn_bytes)
    }
}
impl From<SADeviceValuator> for Action {
    fn from(devval: SADeviceValuator) -> Self {
        let devval_bytes = devval.serialize();
        Self(devval_bytes)
    }
}
impl From<SAType> for Action {
    fn from(type_: SAType) -> Self {
        let type_bytes = u8::from(type_).serialize();
        let value = [
            type_bytes[0],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        Self(value)
    }
}

/// Opcode for the UseExtension request
pub const USE_EXTENSION_REQUEST: u8 = 0;
pub fn use_extension<Conn>(conn: &Conn, wanted_major: u16, wanted_minor: u16) -> Result<Cookie<'_, Conn, UseExtensionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let wanted_major_bytes = wanted_major.serialize();
    let wanted_minor_bytes = wanted_minor.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        USE_EXTENSION_REQUEST,
        0,
        0,
        wanted_major_bytes[0],
        wanted_major_bytes[1],
        wanted_minor_bytes[0],
        wanted_minor_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseExtensionReply {
    pub response_type: u8,
    pub supported: bool,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}
impl TryParse for UseExtensionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (supported, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = UseExtensionReply { response_type, supported, sequence, length, server_major, server_minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for UseExtensionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SelectEvents request
pub const SELECT_EVENTS_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase1 {
    pub affect_new_keyboard: u16,
    pub new_keyboard_details: u16,
}
impl Serialize for SelectEventsAuxBitcase1 {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let affect_new_keyboard_bytes = self.affect_new_keyboard.serialize();
        let new_keyboard_details_bytes = self.new_keyboard_details.serialize();
        [
            affect_new_keyboard_bytes[0],
            affect_new_keyboard_bytes[1],
            new_keyboard_details_bytes[0],
            new_keyboard_details_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.affect_new_keyboard.serialize_into(bytes);
        self.new_keyboard_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase2 {
    pub affect_state: u16,
    pub state_details: u16,
}
impl Serialize for SelectEventsAuxBitcase2 {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let affect_state_bytes = self.affect_state.serialize();
        let state_details_bytes = self.state_details.serialize();
        [
            affect_state_bytes[0],
            affect_state_bytes[1],
            state_details_bytes[0],
            state_details_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.affect_state.serialize_into(bytes);
        self.state_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase3 {
    pub affect_ctrls: u32,
    pub ctrl_details: u32,
}
impl Serialize for SelectEventsAuxBitcase3 {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let affect_ctrls_bytes = self.affect_ctrls.serialize();
        let ctrl_details_bytes = self.ctrl_details.serialize();
        [
            affect_ctrls_bytes[0],
            affect_ctrls_bytes[1],
            affect_ctrls_bytes[2],
            affect_ctrls_bytes[3],
            ctrl_details_bytes[0],
            ctrl_details_bytes[1],
            ctrl_details_bytes[2],
            ctrl_details_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.affect_ctrls.serialize_into(bytes);
        self.ctrl_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase4 {
    pub affect_indicator_state: u32,
    pub indicator_state_details: u32,
}
impl Serialize for SelectEventsAuxBitcase4 {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let affect_indicator_state_bytes = self.affect_indicator_state.serialize();
        let indicator_state_details_bytes = self.indicator_state_details.serialize();
        [
            affect_indicator_state_bytes[0],
            affect_indicator_state_bytes[1],
            affect_indicator_state_bytes[2],
            affect_indicator_state_bytes[3],
            indicator_state_details_bytes[0],
            indicator_state_details_bytes[1],
            indicator_state_details_bytes[2],
            indicator_state_details_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.affect_indicator_state.serialize_into(bytes);
        self.indicator_state_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase5 {
    pub affect_indicator_map: u32,
    pub indicator_map_details: u32,
}
impl Serialize for SelectEventsAuxBitcase5 {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let affect_indicator_map_bytes = self.affect_indicator_map.serialize();
        let indicator_map_details_bytes = self.indicator_map_details.serialize();
        [
            affect_indicator_map_bytes[0],
            affect_indicator_map_bytes[1],
            affect_indicator_map_bytes[2],
            affect_indicator_map_bytes[3],
            indicator_map_details_bytes[0],
            indicator_map_details_bytes[1],
            indicator_map_details_bytes[2],
            indicator_map_details_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.affect_indicator_map.serialize_into(bytes);
        self.indicator_map_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase6 {
    pub affect_names: u16,
    pub names_details: u16,
}
impl Serialize for SelectEventsAuxBitcase6 {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let affect_names_bytes = self.affect_names.serialize();
        let names_details_bytes = self.names_details.serialize();
        [
            affect_names_bytes[0],
            affect_names_bytes[1],
            names_details_bytes[0],
            names_details_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.affect_names.serialize_into(bytes);
        self.names_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase7 {
    pub affect_compat: u8,
    pub compat_details: u8,
}
impl Serialize for SelectEventsAuxBitcase7 {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let affect_compat_bytes = self.affect_compat.serialize();
        let compat_details_bytes = self.compat_details.serialize();
        [
            affect_compat_bytes[0],
            compat_details_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.affect_compat.serialize_into(bytes);
        self.compat_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase8 {
    pub affect_bell: u8,
    pub bell_details: u8,
}
impl Serialize for SelectEventsAuxBitcase8 {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let affect_bell_bytes = self.affect_bell.serialize();
        let bell_details_bytes = self.bell_details.serialize();
        [
            affect_bell_bytes[0],
            bell_details_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.affect_bell.serialize_into(bytes);
        self.bell_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase9 {
    pub affect_msg_details: u8,
    pub msg_details: u8,
}
impl Serialize for SelectEventsAuxBitcase9 {
    type Bytes = [u8; 2];
    fn serialize(&self) -> [u8; 2] {
        let affect_msg_details_bytes = self.affect_msg_details.serialize();
        let msg_details_bytes = self.msg_details.serialize();
        [
            affect_msg_details_bytes[0],
            msg_details_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.affect_msg_details.serialize_into(bytes);
        self.msg_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase10 {
    pub affect_access_x: u16,
    pub access_x_details: u16,
}
impl Serialize for SelectEventsAuxBitcase10 {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let affect_access_x_bytes = self.affect_access_x.serialize();
        let access_x_details_bytes = self.access_x_details.serialize();
        [
            affect_access_x_bytes[0],
            affect_access_x_bytes[1],
            access_x_details_bytes[0],
            access_x_details_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.affect_access_x.serialize_into(bytes);
        self.access_x_details.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase11 {
    pub affect_ext_dev: u16,
    pub extdev_details: u16,
}
impl Serialize for SelectEventsAuxBitcase11 {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let affect_ext_dev_bytes = self.affect_ext_dev.serialize();
        let extdev_details_bytes = self.extdev_details.serialize();
        [
            affect_ext_dev_bytes[0],
            affect_ext_dev_bytes[1],
            extdev_details_bytes[0],
            extdev_details_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.affect_ext_dev.serialize_into(bytes);
        self.extdev_details.serialize_into(bytes);
    }
}
/// Auxiliary and optional information for the `select_events` function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectEventsAux {
    pub bitcase1: Option<SelectEventsAuxBitcase1>,
    pub bitcase2: Option<SelectEventsAuxBitcase2>,
    pub bitcase3: Option<SelectEventsAuxBitcase3>,
    pub bitcase4: Option<SelectEventsAuxBitcase4>,
    pub bitcase5: Option<SelectEventsAuxBitcase5>,
    pub bitcase6: Option<SelectEventsAuxBitcase6>,
    pub bitcase7: Option<SelectEventsAuxBitcase7>,
    pub bitcase8: Option<SelectEventsAuxBitcase8>,
    pub bitcase9: Option<SelectEventsAuxBitcase9>,
    pub bitcase10: Option<SelectEventsAuxBitcase10>,
    pub bitcase11: Option<SelectEventsAuxBitcase11>,
}
#[allow(dead_code, unused_variables)]
impl SelectEventsAux {
    fn serialize(&self, affect_which: u16, clear: u16, select_all: u16) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, affect_which, clear, select_all);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, affect_which: u16, clear: u16, select_all: u16) {
        assert_eq!(self.switch_expr(), u32::from(affect_which) & ((!u32::from(clear)) & (!u32::from(select_all))), "switch `details` has an inconsistent discriminant");
        if let Some(ref bitcase1) = self.bitcase1 {
            bitcase1.serialize_into(bytes);
        }
        if let Some(ref bitcase2) = self.bitcase2 {
            bitcase2.serialize_into(bytes);
        }
        if let Some(ref bitcase3) = self.bitcase3 {
            bitcase3.serialize_into(bytes);
        }
        if let Some(ref bitcase4) = self.bitcase4 {
            bitcase4.serialize_into(bytes);
        }
        if let Some(ref bitcase5) = self.bitcase5 {
            bitcase5.serialize_into(bytes);
        }
        if let Some(ref bitcase6) = self.bitcase6 {
            bitcase6.serialize_into(bytes);
        }
        if let Some(ref bitcase7) = self.bitcase7 {
            bitcase7.serialize_into(bytes);
        }
        if let Some(ref bitcase8) = self.bitcase8 {
            bitcase8.serialize_into(bytes);
        }
        if let Some(ref bitcase9) = self.bitcase9 {
            bitcase9.serialize_into(bytes);
        }
        if let Some(ref bitcase10) = self.bitcase10 {
            bitcase10.serialize_into(bytes);
        }
        if let Some(ref bitcase11) = self.bitcase11 {
            bitcase11.serialize_into(bytes);
        }
    }
}
impl SelectEventsAux {
    fn switch_expr(&self) -> u32 {
        let mut expr_value = 0;
        if self.bitcase1.is_some() {
            expr_value |= u32::from(EventType::NewKeyboardNotify);
        }
        if self.bitcase2.is_some() {
            expr_value |= u32::from(EventType::StateNotify);
        }
        if self.bitcase3.is_some() {
            expr_value |= u32::from(EventType::ControlsNotify);
        }
        if self.bitcase4.is_some() {
            expr_value |= u32::from(EventType::IndicatorStateNotify);
        }
        if self.bitcase5.is_some() {
            expr_value |= u32::from(EventType::IndicatorMapNotify);
        }
        if self.bitcase6.is_some() {
            expr_value |= u32::from(EventType::NamesNotify);
        }
        if self.bitcase7.is_some() {
            expr_value |= u32::from(EventType::CompatMapNotify);
        }
        if self.bitcase8.is_some() {
            expr_value |= u32::from(EventType::BellNotify);
        }
        if self.bitcase9.is_some() {
            expr_value |= u32::from(EventType::ActionMessage);
        }
        if self.bitcase10.is_some() {
            expr_value |= u32::from(EventType::AccessXNotify);
        }
        if self.bitcase11.is_some() {
            expr_value |= u32::from(EventType::ExtensionDeviceNotify);
        }
        expr_value
    }
}
impl SelectEventsAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the `bitcase1` field of this structure.
    pub fn bitcase1<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase1>> {
        self.bitcase1 = value.into();
        self
    }
    /// Set the `bitcase2` field of this structure.
    pub fn bitcase2<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase2>> {
        self.bitcase2 = value.into();
        self
    }
    /// Set the `bitcase3` field of this structure.
    pub fn bitcase3<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase3>> {
        self.bitcase3 = value.into();
        self
    }
    /// Set the `bitcase4` field of this structure.
    pub fn bitcase4<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase4>> {
        self.bitcase4 = value.into();
        self
    }
    /// Set the `bitcase5` field of this structure.
    pub fn bitcase5<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase5>> {
        self.bitcase5 = value.into();
        self
    }
    /// Set the `bitcase6` field of this structure.
    pub fn bitcase6<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase6>> {
        self.bitcase6 = value.into();
        self
    }
    /// Set the `bitcase7` field of this structure.
    pub fn bitcase7<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase7>> {
        self.bitcase7 = value.into();
        self
    }
    /// Set the `bitcase8` field of this structure.
    pub fn bitcase8<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase8>> {
        self.bitcase8 = value.into();
        self
    }
    /// Set the `bitcase9` field of this structure.
    pub fn bitcase9<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase9>> {
        self.bitcase9 = value.into();
        self
    }
    /// Set the `bitcase10` field of this structure.
    pub fn bitcase10<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase10>> {
        self.bitcase10 = value.into();
        self
    }
    /// Set the `bitcase11` field of this structure.
    pub fn bitcase11<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase11>> {
        self.bitcase11 = value.into();
        self
    }
}

pub fn select_events<'c, 'input, Conn, A, B, C, D, E>(conn: &'c Conn, device_spec: DeviceSpec, affect_which: A, clear: B, select_all: C, affect_map: D, map: E, details: &'input SelectEventsAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u16>,
    C: Into<u16>,
    D: Into<u16>,
    E: Into<u16>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let affect_which: u16 = affect_which.into();
    let clear: u16 = clear.into();
    let select_all: u16 = select_all.into();
    let affect_map: u16 = affect_map.into();
    let map: u16 = map.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let affect_which_bytes = affect_which.serialize();
    let clear_bytes = clear.serialize();
    let select_all_bytes = select_all.serialize();
    let affect_map_bytes = affect_map.serialize();
    let map_bytes = map.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SELECT_EVENTS_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        affect_which_bytes[0],
        affect_which_bytes[1],
        clear_bytes[0],
        clear_bytes[1],
        select_all_bytes[0],
        select_all_bytes[1],
        affect_map_bytes[0],
        affect_map_bytes[1],
        map_bytes[0],
        map_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    let details_bytes = details.serialize(affect_which, clear, select_all);
    let length_so_far = length_so_far + details_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&details_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the Bell request
pub const BELL_REQUEST: u8 = 3;
pub fn bell<Conn>(conn: &Conn, device_spec: DeviceSpec, bell_class: BellClassSpec, bell_id: IDSpec, percent: i8, force_sound: bool, event_only: bool, pitch: i16, duration: i16, name: xproto::Atom, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let bell_class_bytes = bell_class.serialize();
    let bell_id_bytes = bell_id.serialize();
    let percent_bytes = percent.serialize();
    let force_sound_bytes = force_sound.serialize();
    let event_only_bytes = event_only.serialize();
    let pitch_bytes = pitch.serialize();
    let duration_bytes = duration.serialize();
    let name_bytes = name.serialize();
    let window_bytes = window.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        BELL_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        bell_class_bytes[0],
        bell_class_bytes[1],
        bell_id_bytes[0],
        bell_id_bytes[1],
        percent_bytes[0],
        force_sound_bytes[0],
        event_only_bytes[0],
        0,
        pitch_bytes[0],
        pitch_bytes[1],
        duration_bytes[0],
        duration_bytes[1],
        0,
        0,
        name_bytes[0],
        name_bytes[1],
        name_bytes[2],
        name_bytes[3],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the GetState request
pub const GET_STATE_REQUEST: u8 = 4;
pub fn get_state<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_STATE_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStateReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub mods: u8,
    pub base_mods: u8,
    pub latched_mods: u8,
    pub locked_mods: u8,
    pub group: Group,
    pub locked_group: Group,
    pub base_group: i16,
    pub latched_group: i16,
    pub compat_state: u8,
    pub grab_mods: u8,
    pub compat_grab_mods: u8,
    pub lookup_mods: u8,
    pub compat_lookup_mods: u8,
    pub ptr_btn_state: u16,
}
impl TryParse for GetStateReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (mods, remaining) = u8::try_parse(remaining)?;
        let (base_mods, remaining) = u8::try_parse(remaining)?;
        let (latched_mods, remaining) = u8::try_parse(remaining)?;
        let (locked_mods, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = u8::try_parse(remaining)?;
        let (locked_group, remaining) = u8::try_parse(remaining)?;
        let (base_group, remaining) = i16::try_parse(remaining)?;
        let (latched_group, remaining) = i16::try_parse(remaining)?;
        let (compat_state, remaining) = u8::try_parse(remaining)?;
        let (grab_mods, remaining) = u8::try_parse(remaining)?;
        let (compat_grab_mods, remaining) = u8::try_parse(remaining)?;
        let (lookup_mods, remaining) = u8::try_parse(remaining)?;
        let (compat_lookup_mods, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (ptr_btn_state, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(6..).ok_or(ParseError::ParseError)?;
        let group = group.try_into()?;
        let locked_group = locked_group.try_into()?;
        let result = GetStateReply { response_type, device_id, sequence, length, mods, base_mods, latched_mods, locked_mods, group, locked_group, base_group, latched_group, compat_state, grab_mods, compat_grab_mods, lookup_mods, compat_lookup_mods, ptr_btn_state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetStateReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the LatchLockState request
pub const LATCH_LOCK_STATE_REQUEST: u8 = 5;
pub fn latch_lock_state<Conn, A, B, C>(conn: &Conn, device_spec: DeviceSpec, affect_mod_locks: A, mod_locks: B, lock_group: bool, group_lock: Group, affect_mod_latches: C, latch_group: bool, group_latch: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
    B: Into<u8>,
    C: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let affect_mod_locks: u8 = affect_mod_locks.into();
    let mod_locks: u8 = mod_locks.into();
    let affect_mod_latches: u8 = affect_mod_latches.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let affect_mod_locks_bytes = affect_mod_locks.serialize();
    let mod_locks_bytes = mod_locks.serialize();
    let lock_group_bytes = lock_group.serialize();
    let group_lock_bytes = u8::from(group_lock).serialize();
    let affect_mod_latches_bytes = affect_mod_latches.serialize();
    let latch_group_bytes = latch_group.serialize();
    let group_latch_bytes = group_latch.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        LATCH_LOCK_STATE_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        affect_mod_locks_bytes[0],
        mod_locks_bytes[0],
        lock_group_bytes[0],
        group_lock_bytes[0],
        affect_mod_latches_bytes[0],
        0,
        0,
        latch_group_bytes[0],
        group_latch_bytes[0],
        group_latch_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the GetControls request
pub const GET_CONTROLS_REQUEST: u8 = 6;
pub fn get_controls<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetControlsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_CONTROLS_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetControlsReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub mouse_keys_dflt_btn: u8,
    pub num_groups: u8,
    pub groups_wrap: u8,
    pub internal_mods_mask: u8,
    pub ignore_lock_mods_mask: u8,
    pub internal_mods_real_mods: u8,
    pub ignore_lock_mods_real_mods: u8,
    pub internal_mods_vmods: u16,
    pub ignore_lock_mods_vmods: u16,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys_delay: u16,
    pub mouse_keys_interval: u16,
    pub mouse_keys_time_to_max: u16,
    pub mouse_keys_max_speed: u16,
    pub mouse_keys_curve: i16,
    pub access_x_option: u16,
    pub access_x_timeout: u16,
    pub access_x_timeout_options_mask: u16,
    pub access_x_timeout_options_values: u16,
    pub access_x_timeout_mask: u32,
    pub access_x_timeout_values: u32,
    pub enabled_controls: u32,
    pub per_key_repeat: [u8; 32],
}
impl TryParse for GetControlsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (mouse_keys_dflt_btn, remaining) = u8::try_parse(remaining)?;
        let (num_groups, remaining) = u8::try_parse(remaining)?;
        let (groups_wrap, remaining) = u8::try_parse(remaining)?;
        let (internal_mods_mask, remaining) = u8::try_parse(remaining)?;
        let (ignore_lock_mods_mask, remaining) = u8::try_parse(remaining)?;
        let (internal_mods_real_mods, remaining) = u8::try_parse(remaining)?;
        let (ignore_lock_mods_real_mods, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (internal_mods_vmods, remaining) = u16::try_parse(remaining)?;
        let (ignore_lock_mods_vmods, remaining) = u16::try_parse(remaining)?;
        let (repeat_delay, remaining) = u16::try_parse(remaining)?;
        let (repeat_interval, remaining) = u16::try_parse(remaining)?;
        let (slow_keys_delay, remaining) = u16::try_parse(remaining)?;
        let (debounce_delay, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_delay, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_interval, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_time_to_max, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_max_speed, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_curve, remaining) = i16::try_parse(remaining)?;
        let (access_x_option, remaining) = u16::try_parse(remaining)?;
        let (access_x_timeout, remaining) = u16::try_parse(remaining)?;
        let (access_x_timeout_options_mask, remaining) = u16::try_parse(remaining)?;
        let (access_x_timeout_options_values, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (access_x_timeout_mask, remaining) = u32::try_parse(remaining)?;
        let (access_x_timeout_values, remaining) = u32::try_parse(remaining)?;
        let (enabled_controls, remaining) = u32::try_parse(remaining)?;
        let (per_key_repeat, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let per_key_repeat = <[u8; 32]>::try_from(per_key_repeat).unwrap();
        let result = GetControlsReply { response_type, device_id, sequence, length, mouse_keys_dflt_btn, num_groups, groups_wrap, internal_mods_mask, ignore_lock_mods_mask, internal_mods_real_mods, ignore_lock_mods_real_mods, internal_mods_vmods, ignore_lock_mods_vmods, repeat_delay, repeat_interval, slow_keys_delay, debounce_delay, mouse_keys_delay, mouse_keys_interval, mouse_keys_time_to_max, mouse_keys_max_speed, mouse_keys_curve, access_x_option, access_x_timeout, access_x_timeout_options_mask, access_x_timeout_options_values, access_x_timeout_mask, access_x_timeout_values, enabled_controls, per_key_repeat };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetControlsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetControls request
pub const SET_CONTROLS_REQUEST: u8 = 7;
pub fn set_controls<'c, 'input, Conn, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P>(conn: &'c Conn, device_spec: DeviceSpec, affect_internal_real_mods: A, internal_real_mods: B, affect_ignore_lock_real_mods: C, ignore_lock_real_mods: D, affect_internal_virtual_mods: E, internal_virtual_mods: F, affect_ignore_lock_virtual_mods: G, ignore_lock_virtual_mods: H, mouse_keys_dflt_btn: u8, groups_wrap: u8, access_x_options: I, affect_enabled_controls: J, enabled_controls: K, change_controls: L, repeat_delay: u16, repeat_interval: u16, slow_keys_delay: u16, debounce_delay: u16, mouse_keys_delay: u16, mouse_keys_interval: u16, mouse_keys_time_to_max: u16, mouse_keys_max_speed: u16, mouse_keys_curve: i16, access_x_timeout: u16, access_x_timeout_mask: M, access_x_timeout_values: N, access_x_timeout_options_mask: O, access_x_timeout_options_values: P, per_key_repeat: &'input [u8; 32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
    B: Into<u8>,
    C: Into<u8>,
    D: Into<u8>,
    E: Into<u16>,
    F: Into<u16>,
    G: Into<u16>,
    H: Into<u16>,
    I: Into<u16>,
    J: Into<u32>,
    K: Into<u32>,
    L: Into<u32>,
    M: Into<u32>,
    N: Into<u32>,
    O: Into<u16>,
    P: Into<u16>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let affect_internal_real_mods: u8 = affect_internal_real_mods.into();
    let internal_real_mods: u8 = internal_real_mods.into();
    let affect_ignore_lock_real_mods: u8 = affect_ignore_lock_real_mods.into();
    let ignore_lock_real_mods: u8 = ignore_lock_real_mods.into();
    let affect_internal_virtual_mods: u16 = affect_internal_virtual_mods.into();
    let internal_virtual_mods: u16 = internal_virtual_mods.into();
    let affect_ignore_lock_virtual_mods: u16 = affect_ignore_lock_virtual_mods.into();
    let ignore_lock_virtual_mods: u16 = ignore_lock_virtual_mods.into();
    let access_x_options: u16 = access_x_options.into();
    let affect_enabled_controls: u32 = affect_enabled_controls.into();
    let enabled_controls: u32 = enabled_controls.into();
    let change_controls: u32 = change_controls.into();
    let access_x_timeout_mask: u32 = access_x_timeout_mask.into();
    let access_x_timeout_values: u32 = access_x_timeout_values.into();
    let access_x_timeout_options_mask: u16 = access_x_timeout_options_mask.into();
    let access_x_timeout_options_values: u16 = access_x_timeout_options_values.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let affect_internal_real_mods_bytes = affect_internal_real_mods.serialize();
    let internal_real_mods_bytes = internal_real_mods.serialize();
    let affect_ignore_lock_real_mods_bytes = affect_ignore_lock_real_mods.serialize();
    let ignore_lock_real_mods_bytes = ignore_lock_real_mods.serialize();
    let affect_internal_virtual_mods_bytes = affect_internal_virtual_mods.serialize();
    let internal_virtual_mods_bytes = internal_virtual_mods.serialize();
    let affect_ignore_lock_virtual_mods_bytes = affect_ignore_lock_virtual_mods.serialize();
    let ignore_lock_virtual_mods_bytes = ignore_lock_virtual_mods.serialize();
    let mouse_keys_dflt_btn_bytes = mouse_keys_dflt_btn.serialize();
    let groups_wrap_bytes = groups_wrap.serialize();
    let access_x_options_bytes = access_x_options.serialize();
    let affect_enabled_controls_bytes = affect_enabled_controls.serialize();
    let enabled_controls_bytes = enabled_controls.serialize();
    let change_controls_bytes = change_controls.serialize();
    let repeat_delay_bytes = repeat_delay.serialize();
    let repeat_interval_bytes = repeat_interval.serialize();
    let slow_keys_delay_bytes = slow_keys_delay.serialize();
    let debounce_delay_bytes = debounce_delay.serialize();
    let mouse_keys_delay_bytes = mouse_keys_delay.serialize();
    let mouse_keys_interval_bytes = mouse_keys_interval.serialize();
    let mouse_keys_time_to_max_bytes = mouse_keys_time_to_max.serialize();
    let mouse_keys_max_speed_bytes = mouse_keys_max_speed.serialize();
    let mouse_keys_curve_bytes = mouse_keys_curve.serialize();
    let access_x_timeout_bytes = access_x_timeout.serialize();
    let access_x_timeout_mask_bytes = access_x_timeout_mask.serialize();
    let access_x_timeout_values_bytes = access_x_timeout_values.serialize();
    let access_x_timeout_options_mask_bytes = access_x_timeout_options_mask.serialize();
    let access_x_timeout_options_values_bytes = access_x_timeout_options_values.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_CONTROLS_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        affect_internal_real_mods_bytes[0],
        internal_real_mods_bytes[0],
        affect_ignore_lock_real_mods_bytes[0],
        ignore_lock_real_mods_bytes[0],
        affect_internal_virtual_mods_bytes[0],
        affect_internal_virtual_mods_bytes[1],
        internal_virtual_mods_bytes[0],
        internal_virtual_mods_bytes[1],
        affect_ignore_lock_virtual_mods_bytes[0],
        affect_ignore_lock_virtual_mods_bytes[1],
        ignore_lock_virtual_mods_bytes[0],
        ignore_lock_virtual_mods_bytes[1],
        mouse_keys_dflt_btn_bytes[0],
        groups_wrap_bytes[0],
        access_x_options_bytes[0],
        access_x_options_bytes[1],
        0,
        0,
        affect_enabled_controls_bytes[0],
        affect_enabled_controls_bytes[1],
        affect_enabled_controls_bytes[2],
        affect_enabled_controls_bytes[3],
        enabled_controls_bytes[0],
        enabled_controls_bytes[1],
        enabled_controls_bytes[2],
        enabled_controls_bytes[3],
        change_controls_bytes[0],
        change_controls_bytes[1],
        change_controls_bytes[2],
        change_controls_bytes[3],
        repeat_delay_bytes[0],
        repeat_delay_bytes[1],
        repeat_interval_bytes[0],
        repeat_interval_bytes[1],
        slow_keys_delay_bytes[0],
        slow_keys_delay_bytes[1],
        debounce_delay_bytes[0],
        debounce_delay_bytes[1],
        mouse_keys_delay_bytes[0],
        mouse_keys_delay_bytes[1],
        mouse_keys_interval_bytes[0],
        mouse_keys_interval_bytes[1],
        mouse_keys_time_to_max_bytes[0],
        mouse_keys_time_to_max_bytes[1],
        mouse_keys_max_speed_bytes[0],
        mouse_keys_max_speed_bytes[1],
        mouse_keys_curve_bytes[0],
        mouse_keys_curve_bytes[1],
        access_x_timeout_bytes[0],
        access_x_timeout_bytes[1],
        access_x_timeout_mask_bytes[0],
        access_x_timeout_mask_bytes[1],
        access_x_timeout_mask_bytes[2],
        access_x_timeout_mask_bytes[3],
        access_x_timeout_values_bytes[0],
        access_x_timeout_values_bytes[1],
        access_x_timeout_values_bytes[2],
        access_x_timeout_values_bytes[3],
        access_x_timeout_options_mask_bytes[0],
        access_x_timeout_options_mask_bytes[1],
        access_x_timeout_options_values_bytes[0],
        access_x_timeout_options_values_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    let length_so_far = length_so_far + per_key_repeat.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(per_key_repeat)], vec![])?)
}

/// Opcode for the GetMap request
pub const GET_MAP_REQUEST: u8 = 8;
pub fn get_map<Conn, A, B, C>(conn: &Conn, device_spec: DeviceSpec, full: A, partial: B, first_type: u8, n_types: u8, first_key_sym: xproto::Keycode, n_key_syms: u8, first_key_action: xproto::Keycode, n_key_actions: u8, first_key_behavior: xproto::Keycode, n_key_behaviors: u8, virtual_mods: C, first_key_explicit: xproto::Keycode, n_key_explicit: u8, first_mod_map_key: xproto::Keycode, n_mod_map_keys: u8, first_v_mod_map_key: xproto::Keycode, n_v_mod_map_keys: u8) -> Result<Cookie<'_, Conn, GetMapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u16>,
    C: Into<u16>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let full: u16 = full.into();
    let partial: u16 = partial.into();
    let virtual_mods: u16 = virtual_mods.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let full_bytes = full.serialize();
    let partial_bytes = partial.serialize();
    let first_type_bytes = first_type.serialize();
    let n_types_bytes = n_types.serialize();
    let first_key_sym_bytes = first_key_sym.serialize();
    let n_key_syms_bytes = n_key_syms.serialize();
    let first_key_action_bytes = first_key_action.serialize();
    let n_key_actions_bytes = n_key_actions.serialize();
    let first_key_behavior_bytes = first_key_behavior.serialize();
    let n_key_behaviors_bytes = n_key_behaviors.serialize();
    let virtual_mods_bytes = virtual_mods.serialize();
    let first_key_explicit_bytes = first_key_explicit.serialize();
    let n_key_explicit_bytes = n_key_explicit.serialize();
    let first_mod_map_key_bytes = first_mod_map_key.serialize();
    let n_mod_map_keys_bytes = n_mod_map_keys.serialize();
    let first_v_mod_map_key_bytes = first_v_mod_map_key.serialize();
    let n_v_mod_map_keys_bytes = n_v_mod_map_keys.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_MAP_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        full_bytes[0],
        full_bytes[1],
        partial_bytes[0],
        partial_bytes[1],
        first_type_bytes[0],
        n_types_bytes[0],
        first_key_sym_bytes[0],
        n_key_syms_bytes[0],
        first_key_action_bytes[0],
        n_key_actions_bytes[0],
        first_key_behavior_bytes[0],
        n_key_behaviors_bytes[0],
        virtual_mods_bytes[0],
        virtual_mods_bytes[1],
        first_key_explicit_bytes[0],
        n_key_explicit_bytes[0],
        first_mod_map_key_bytes[0],
        n_mod_map_keys_bytes[0],
        first_v_mod_map_key_bytes[0],
        n_v_mod_map_keys_bytes[0],
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone)]
pub struct GetMapMapBitcase3 {
    pub acts_rtrn_count: Vec<u8>,
    pub acts_rtrn_acts: Vec<Action>,
}
impl GetMapMapBitcase3 {
    pub fn try_parse(remaining: &[u8], n_key_actions: u8, total_actions: u16) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (acts_rtrn_count, remaining) = crate::x11_utils::parse_u8_list(remaining, n_key_actions.try_into().or(Err(ParseError::ParseError))?)?;
        let acts_rtrn_count = acts_rtrn_count.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (acts_rtrn_acts, remaining) = crate::x11_utils::parse_list::<Action>(remaining, total_actions.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetMapMapBitcase3 { acts_rtrn_count, acts_rtrn_acts };
        Ok((result, remaining))
    }
}
// Skipping TryFrom implementations because of unresolved members
#[derive(Debug, Clone, Default)]
pub struct GetMapMap {
    pub types_rtrn: Option<Vec<KeyType>>,
    pub syms_rtrn: Option<Vec<KeySymMap>>,
    pub bitcase3: Option<GetMapMapBitcase3>,
    pub behaviors_rtrn: Option<Vec<SetBehavior>>,
    pub vmods_rtrn: Option<Vec<u8>>,
    pub explicit_rtrn: Option<Vec<SetExplicit>>,
    pub modmap_rtrn: Option<Vec<KeyModMap>>,
    pub vmodmap_rtrn: Option<Vec<KeyVModMap>>,
}
impl GetMapMap {
    fn try_parse(value: &[u8], present: u16, n_types: u8, n_key_syms: u8, n_key_actions: u8, total_actions: u16, total_key_behaviors: u8, virtual_mods: u16, total_key_explicit: u8, total_mod_map_keys: u8, total_v_mod_map_keys: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = u32::from(present);
        let mut outer_remaining = value;
        let types_rtrn = if switch_expr & u32::from(MapPart::KeyTypes) != 0 {
            let remaining = outer_remaining;
            let (types_rtrn, remaining) = crate::x11_utils::parse_list::<KeyType>(remaining, n_types.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(types_rtrn)
        } else {
            None
        };
        let syms_rtrn = if switch_expr & u32::from(MapPart::KeySyms) != 0 {
            let remaining = outer_remaining;
            let (syms_rtrn, remaining) = crate::x11_utils::parse_list::<KeySymMap>(remaining, n_key_syms.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(syms_rtrn)
        } else {
            None
        };
        let bitcase3 = if switch_expr & u32::from(MapPart::KeyActions) != 0 {
            let (bitcase3, new_remaining) = GetMapMapBitcase3::try_parse(outer_remaining, n_key_actions, total_actions)?;
            outer_remaining = new_remaining;
            Some(bitcase3)
        } else {
            None
        };
        let behaviors_rtrn = if switch_expr & u32::from(MapPart::KeyBehaviors) != 0 {
            let remaining = outer_remaining;
            let (behaviors_rtrn, remaining) = crate::x11_utils::parse_list::<SetBehavior>(remaining, total_key_behaviors.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(behaviors_rtrn)
        } else {
            None
        };
        let vmods_rtrn = if switch_expr & u32::from(MapPart::VirtualMods) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (vmods_rtrn, remaining) = crate::x11_utils::parse_u8_list(remaining, virtual_mods.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
            let vmods_rtrn = vmods_rtrn.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(vmods_rtrn)
        } else {
            None
        };
        let explicit_rtrn = if switch_expr & u32::from(MapPart::ExplicitComponents) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (explicit_rtrn, remaining) = crate::x11_utils::parse_list::<SetExplicit>(remaining, total_key_explicit.try_into().or(Err(ParseError::ParseError))?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(explicit_rtrn)
        } else {
            None
        };
        let modmap_rtrn = if switch_expr & u32::from(MapPart::ModifierMap) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (modmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyModMap>(remaining, total_mod_map_keys.try_into().or(Err(ParseError::ParseError))?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(modmap_rtrn)
        } else {
            None
        };
        let vmodmap_rtrn = if switch_expr & u32::from(MapPart::VirtualModMap) != 0 {
            let remaining = outer_remaining;
            let (vmodmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyVModMap>(remaining, total_v_mod_map_keys.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(vmodmap_rtrn)
        } else {
            None
        };
        let result = GetMapMap { types_rtrn, syms_rtrn, bitcase3, behaviors_rtrn, vmods_rtrn, explicit_rtrn, modmap_rtrn, vmodmap_rtrn };
        Ok((result, outer_remaining))
    }
}

#[derive(Debug, Clone)]
pub struct GetMapReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: xproto::Keycode,
    pub max_key_code: xproto::Keycode,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: xproto::Keycode,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: xproto::Keycode,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: xproto::Keycode,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xproto::Keycode,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: xproto::Keycode,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub virtual_mods: u16,
    pub map: GetMapMap,
}
impl TryParse for GetMapReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (present, remaining) = u16::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (total_types, remaining) = u8::try_parse(remaining)?;
        let (first_key_sym, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (total_syms, remaining) = u16::try_parse(remaining)?;
        let (n_key_syms, remaining) = u8::try_parse(remaining)?;
        let (first_key_action, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (total_actions, remaining) = u16::try_parse(remaining)?;
        let (n_key_actions, remaining) = u8::try_parse(remaining)?;
        let (first_key_behavior, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_behaviors, remaining) = u8::try_parse(remaining)?;
        let (total_key_behaviors, remaining) = u8::try_parse(remaining)?;
        let (first_key_explicit, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (total_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (first_mod_map_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (total_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (first_v_mod_map_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_v_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (total_v_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (map, remaining) = GetMapMap::try_parse(remaining, present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys)?;
        let result = GetMapReply { response_type, device_id, sequence, length, min_key_code, max_key_code, first_type, n_types, total_types, first_key_sym, total_syms, n_key_syms, first_key_action, total_actions, n_key_actions, first_key_behavior, n_key_behaviors, total_key_behaviors, first_key_explicit, n_key_explicit, total_key_explicit, first_mod_map_key, n_mod_map_keys, total_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, total_v_mod_map_keys, virtual_mods, map };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMapReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetMap request
pub const SET_MAP_REQUEST: u8 = 9;
#[derive(Debug, Clone)]
pub struct SetMapAuxBitcase3 {
    pub actions_count: Vec<u8>,
    pub actions: Vec<Action>,
}
#[allow(dead_code, unused_variables)]
impl SetMapAuxBitcase3 {
    fn serialize(&self, n_key_actions: u8, total_actions: u16) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, n_key_actions, total_actions);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, n_key_actions: u8, total_actions: u16) {
        assert_eq!(self.actions_count.len(), usize::try_from(n_key_actions).unwrap(), "`actions_count` has an incorrect length");
        bytes.extend_from_slice(&self.actions_count);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        assert_eq!(self.actions.len(), usize::try_from(total_actions).unwrap(), "`actions` has an incorrect length");
        self.actions.serialize_into(bytes);
    }
}
/// Auxiliary and optional information for the `set_map` function
#[derive(Debug, Clone, Default)]
pub struct SetMapAux {
    pub types: Option<Vec<SetKeyType>>,
    pub syms: Option<Vec<KeySymMap>>,
    pub bitcase3: Option<SetMapAuxBitcase3>,
    pub behaviors: Option<Vec<SetBehavior>>,
    pub vmods: Option<Vec<u8>>,
    pub explicit: Option<Vec<SetExplicit>>,
    pub modmap: Option<Vec<KeyModMap>>,
    pub vmodmap: Option<Vec<KeyVModMap>>,
}
#[allow(dead_code, unused_variables)]
impl SetMapAux {
    fn serialize(&self, present: u16, n_types: u8, n_key_syms: u8, n_key_actions: u8, total_actions: u16, total_key_behaviors: u8, virtual_mods: u16, total_key_explicit: u8, total_mod_map_keys: u8, total_v_mod_map_keys: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, present: u16, n_types: u8, n_key_syms: u8, n_key_actions: u8, total_actions: u16, total_key_behaviors: u8, virtual_mods: u16, total_key_explicit: u8, total_mod_map_keys: u8, total_v_mod_map_keys: u8) {
        if let Some(ref types) = self.types {
            assert_eq!(types.len(), usize::try_from(n_types).unwrap(), "`types` has an incorrect length");
            types.serialize_into(bytes);
        }
        if let Some(ref syms) = self.syms {
            assert_eq!(syms.len(), usize::try_from(n_key_syms).unwrap(), "`syms` has an incorrect length");
            syms.serialize_into(bytes);
        }
        if let Some(ref bitcase3) = self.bitcase3 {
            bitcase3.serialize_into(bytes, n_key_actions, total_actions);
        }
        if let Some(ref behaviors) = self.behaviors {
            assert_eq!(behaviors.len(), usize::try_from(total_key_behaviors).unwrap(), "`behaviors` has an incorrect length");
            behaviors.serialize_into(bytes);
        }
        if let Some(ref vmods) = self.vmods {
            assert_eq!(vmods.len(), usize::try_from(virtual_mods.count_ones()).unwrap(), "`vmods` has an incorrect length");
            bytes.extend_from_slice(&vmods);
            bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        }
        if let Some(ref explicit) = self.explicit {
            assert_eq!(explicit.len(), usize::try_from(total_key_explicit).unwrap(), "`explicit` has an incorrect length");
            explicit.serialize_into(bytes);
        }
        if let Some(ref modmap) = self.modmap {
            assert_eq!(modmap.len(), usize::try_from(total_mod_map_keys).unwrap(), "`modmap` has an incorrect length");
            modmap.serialize_into(bytes);
        }
        if let Some(ref vmodmap) = self.vmodmap {
            assert_eq!(vmodmap.len(), usize::try_from(total_v_mod_map_keys).unwrap(), "`vmodmap` has an incorrect length");
            vmodmap.serialize_into(bytes);
        }
    }
}
impl SetMapAux {
    fn switch_expr(&self) -> u32 {
        let mut expr_value = 0;
        if self.types.is_some() {
            expr_value |= u32::from(MapPart::KeyTypes);
        }
        if self.syms.is_some() {
            expr_value |= u32::from(MapPart::KeySyms);
        }
        if self.bitcase3.is_some() {
            expr_value |= u32::from(MapPart::KeyActions);
        }
        if self.behaviors.is_some() {
            expr_value |= u32::from(MapPart::KeyBehaviors);
        }
        if self.vmods.is_some() {
            expr_value |= u32::from(MapPart::VirtualMods);
        }
        if self.explicit.is_some() {
            expr_value |= u32::from(MapPart::ExplicitComponents);
        }
        if self.modmap.is_some() {
            expr_value |= u32::from(MapPart::ModifierMap);
        }
        if self.vmodmap.is_some() {
            expr_value |= u32::from(MapPart::VirtualModMap);
        }
        expr_value
    }
}
impl SetMapAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the `types` field of this structure.
    pub fn types<I>(mut self, value: I) -> Self where I: Into<Option<Vec<SetKeyType>>> {
        self.types = value.into();
        self
    }
    /// Set the `syms` field of this structure.
    pub fn syms<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeySymMap>>> {
        self.syms = value.into();
        self
    }
    /// Set the `bitcase3` field of this structure.
    pub fn bitcase3<I>(mut self, value: I) -> Self where I: Into<Option<SetMapAuxBitcase3>> {
        self.bitcase3 = value.into();
        self
    }
    /// Set the `behaviors` field of this structure.
    pub fn behaviors<I>(mut self, value: I) -> Self where I: Into<Option<Vec<SetBehavior>>> {
        self.behaviors = value.into();
        self
    }
    /// Set the `vmods` field of this structure.
    pub fn vmods<I>(mut self, value: I) -> Self where I: Into<Option<Vec<u8>>> {
        self.vmods = value.into();
        self
    }
    /// Set the `explicit` field of this structure.
    pub fn explicit<I>(mut self, value: I) -> Self where I: Into<Option<Vec<SetExplicit>>> {
        self.explicit = value.into();
        self
    }
    /// Set the `modmap` field of this structure.
    pub fn modmap<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeyModMap>>> {
        self.modmap = value.into();
        self
    }
    /// Set the `vmodmap` field of this structure.
    pub fn vmodmap<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeyVModMap>>> {
        self.vmodmap = value.into();
        self
    }
}

pub fn set_map<'c, 'input, Conn, A, B>(conn: &'c Conn, device_spec: DeviceSpec, flags: A, min_key_code: xproto::Keycode, max_key_code: xproto::Keycode, first_type: u8, n_types: u8, first_key_sym: xproto::Keycode, n_key_syms: u8, total_syms: u16, first_key_action: xproto::Keycode, n_key_actions: u8, total_actions: u16, first_key_behavior: xproto::Keycode, n_key_behaviors: u8, total_key_behaviors: u8, first_key_explicit: xproto::Keycode, n_key_explicit: u8, total_key_explicit: u8, first_mod_map_key: xproto::Keycode, n_mod_map_keys: u8, total_mod_map_keys: u8, first_v_mod_map_key: xproto::Keycode, n_v_mod_map_keys: u8, total_v_mod_map_keys: u8, virtual_mods: B, values: &'input SetMapAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u16>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let flags: u16 = flags.into();
    let virtual_mods: u16 = virtual_mods.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let present = u16::try_from(values.switch_expr()).unwrap();
    let present_bytes = present.serialize();
    let flags_bytes = flags.serialize();
    let min_key_code_bytes = min_key_code.serialize();
    let max_key_code_bytes = max_key_code.serialize();
    let first_type_bytes = first_type.serialize();
    let n_types_bytes = n_types.serialize();
    let first_key_sym_bytes = first_key_sym.serialize();
    let n_key_syms_bytes = n_key_syms.serialize();
    let total_syms_bytes = total_syms.serialize();
    let first_key_action_bytes = first_key_action.serialize();
    let n_key_actions_bytes = n_key_actions.serialize();
    let total_actions_bytes = total_actions.serialize();
    let first_key_behavior_bytes = first_key_behavior.serialize();
    let n_key_behaviors_bytes = n_key_behaviors.serialize();
    let total_key_behaviors_bytes = total_key_behaviors.serialize();
    let first_key_explicit_bytes = first_key_explicit.serialize();
    let n_key_explicit_bytes = n_key_explicit.serialize();
    let total_key_explicit_bytes = total_key_explicit.serialize();
    let first_mod_map_key_bytes = first_mod_map_key.serialize();
    let n_mod_map_keys_bytes = n_mod_map_keys.serialize();
    let total_mod_map_keys_bytes = total_mod_map_keys.serialize();
    let first_v_mod_map_key_bytes = first_v_mod_map_key.serialize();
    let n_v_mod_map_keys_bytes = n_v_mod_map_keys.serialize();
    let total_v_mod_map_keys_bytes = total_v_mod_map_keys.serialize();
    let virtual_mods_bytes = virtual_mods.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_MAP_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        present_bytes[0],
        present_bytes[1],
        flags_bytes[0],
        flags_bytes[1],
        min_key_code_bytes[0],
        max_key_code_bytes[0],
        first_type_bytes[0],
        n_types_bytes[0],
        first_key_sym_bytes[0],
        n_key_syms_bytes[0],
        total_syms_bytes[0],
        total_syms_bytes[1],
        first_key_action_bytes[0],
        n_key_actions_bytes[0],
        total_actions_bytes[0],
        total_actions_bytes[1],
        first_key_behavior_bytes[0],
        n_key_behaviors_bytes[0],
        total_key_behaviors_bytes[0],
        first_key_explicit_bytes[0],
        n_key_explicit_bytes[0],
        total_key_explicit_bytes[0],
        first_mod_map_key_bytes[0],
        n_mod_map_keys_bytes[0],
        total_mod_map_keys_bytes[0],
        first_v_mod_map_key_bytes[0],
        n_v_mod_map_keys_bytes[0],
        total_v_mod_map_keys_bytes[0],
        virtual_mods_bytes[0],
        virtual_mods_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    let values_bytes = values.serialize(present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys);
    let length_so_far = length_so_far + values_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&values_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the GetCompatMap request
pub const GET_COMPAT_MAP_REQUEST: u8 = 10;
pub fn get_compat_map<Conn, A>(conn: &Conn, device_spec: DeviceSpec, groups: A, get_all_si: bool, first_si: u16, n_si: u16) -> Result<Cookie<'_, Conn, GetCompatMapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let groups: u8 = groups.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let groups_bytes = groups.serialize();
    let get_all_si_bytes = get_all_si.serialize();
    let first_si_bytes = first_si.serialize();
    let n_si_bytes = n_si.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_COMPAT_MAP_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        groups_bytes[0],
        get_all_si_bytes[0],
        first_si_bytes[0],
        first_si_bytes[1],
        n_si_bytes[0],
        n_si_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCompatMapReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub groups_rtrn: u8,
    pub first_si_rtrn: u16,
    pub n_total_si: u16,
    pub si_rtrn: Vec<SymInterpret>,
    pub group_rtrn: Vec<ModDef>,
}
impl TryParse for GetCompatMapReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (groups_rtrn, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (first_si_rtrn, remaining) = u16::try_parse(remaining)?;
        let (n_si_rtrn, remaining) = u16::try_parse(remaining)?;
        let (n_total_si, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (si_rtrn, remaining) = crate::x11_utils::parse_list::<SymInterpret>(remaining, n_si_rtrn.try_into().or(Err(ParseError::ParseError))?)?;
        let (group_rtrn, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, groups_rtrn.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetCompatMapReply { response_type, device_id, sequence, length, groups_rtrn, first_si_rtrn, n_total_si, si_rtrn, group_rtrn };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCompatMapReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetCompatMapReply {
    /// Get the value of the `nSIRtrn` field.
    ///
    /// The `nSIRtrn` field is used as the length field of the `si_rtrn` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_si_rtrn(&self) -> u16 {
        self.si_rtrn.len()
            .try_into().unwrap()
    }
}

/// Opcode for the SetCompatMap request
pub const SET_COMPAT_MAP_REQUEST: u8 = 11;
pub fn set_compat_map<'c, 'input, Conn, A>(conn: &'c Conn, device_spec: DeviceSpec, recompute_actions: bool, truncate_si: bool, groups: A, first_si: u16, si: &'input [SymInterpret], group_maps: &'input [ModDef]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let groups: u8 = groups.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let recompute_actions_bytes = recompute_actions.serialize();
    let truncate_si_bytes = truncate_si.serialize();
    let groups_bytes = groups.serialize();
    let first_si_bytes = first_si.serialize();
    let n_si = u16::try_from(si.len()).expect("`si` has too many elements");
    let n_si_bytes = n_si.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_COMPAT_MAP_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        recompute_actions_bytes[0],
        truncate_si_bytes[0],
        groups_bytes[0],
        first_si_bytes[0],
        first_si_bytes[1],
        n_si_bytes[0],
        n_si_bytes[1],
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    let si_bytes = si.serialize();
    let length_so_far = length_so_far + si_bytes.len();
    assert_eq!(group_maps.len(), usize::try_from(groups.count_ones()).unwrap(), "`group_maps` has an incorrect length");
    let group_maps_bytes = group_maps.serialize();
    let length_so_far = length_so_far + group_maps_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&si_bytes), IoSlice::new(&group_maps_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the GetIndicatorState request
pub const GET_INDICATOR_STATE_REQUEST: u8 = 12;
pub fn get_indicator_state<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetIndicatorStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_INDICATOR_STATE_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetIndicatorStateReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}
impl TryParse for GetIndicatorStateReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = GetIndicatorStateReply { response_type, device_id, sequence, length, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetIndicatorStateReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetIndicatorMap request
pub const GET_INDICATOR_MAP_REQUEST: u8 = 13;
pub fn get_indicator_map<Conn>(conn: &Conn, device_spec: DeviceSpec, which: u32) -> Result<Cookie<'_, Conn, GetIndicatorMapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let which_bytes = which.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_INDICATOR_MAP_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
        which_bytes[0],
        which_bytes[1],
        which_bytes[2],
        which_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetIndicatorMapReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub which: u32,
    pub real_indicators: u32,
    pub n_indicators: u8,
    pub maps: Vec<IndicatorMap>,
}
impl TryParse for GetIndicatorMapReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (real_indicators, remaining) = u32::try_parse(remaining)?;
        let (n_indicators, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::ParseError)?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, which.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetIndicatorMapReply { response_type, device_id, sequence, length, which, real_indicators, n_indicators, maps };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetIndicatorMapReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetIndicatorMap request
pub const SET_INDICATOR_MAP_REQUEST: u8 = 14;
pub fn set_indicator_map<'c, 'input, Conn>(conn: &'c Conn, device_spec: DeviceSpec, which: u32, maps: &'input [IndicatorMap]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let which_bytes = which.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_INDICATOR_MAP_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
        which_bytes[0],
        which_bytes[1],
        which_bytes[2],
        which_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(maps.len(), usize::try_from(which.count_ones()).unwrap(), "`maps` has an incorrect length");
    let maps_bytes = maps.serialize();
    let length_so_far = length_so_far + maps_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&maps_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the GetNamedIndicator request
pub const GET_NAMED_INDICATOR_REQUEST: u8 = 15;
pub fn get_named_indicator<Conn, A>(conn: &Conn, device_spec: DeviceSpec, led_class: LedClass, led_id: A, indicator: xproto::Atom) -> Result<Cookie<'_, Conn, GetNamedIndicatorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<IDSpec>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let led_id: IDSpec = led_id.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let led_class_bytes = LedClassSpec::from(led_class).serialize();
    let led_id_bytes = led_id.serialize();
    let indicator_bytes = indicator.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_NAMED_INDICATOR_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        led_class_bytes[0],
        led_class_bytes[1],
        led_id_bytes[0],
        led_id_bytes[1],
        0,
        0,
        indicator_bytes[0],
        indicator_bytes[1],
        indicator_bytes[2],
        indicator_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetNamedIndicatorReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub indicator: xproto::Atom,
    pub found: bool,
    pub on: bool,
    pub real_indicator: bool,
    pub ndx: u8,
    pub map_flags: u8,
    pub map_which_groups: u8,
    pub map_groups: u8,
    pub map_which_mods: u8,
    pub map_mods: u8,
    pub map_real_mods: u8,
    pub map_vmod: u16,
    pub map_ctrls: u32,
    pub supported: bool,
}
impl TryParse for GetNamedIndicatorReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (indicator, remaining) = xproto::Atom::try_parse(remaining)?;
        let (found, remaining) = bool::try_parse(remaining)?;
        let (on, remaining) = bool::try_parse(remaining)?;
        let (real_indicator, remaining) = bool::try_parse(remaining)?;
        let (ndx, remaining) = u8::try_parse(remaining)?;
        let (map_flags, remaining) = u8::try_parse(remaining)?;
        let (map_which_groups, remaining) = u8::try_parse(remaining)?;
        let (map_groups, remaining) = u8::try_parse(remaining)?;
        let (map_which_mods, remaining) = u8::try_parse(remaining)?;
        let (map_mods, remaining) = u8::try_parse(remaining)?;
        let (map_real_mods, remaining) = u8::try_parse(remaining)?;
        let (map_vmod, remaining) = u16::try_parse(remaining)?;
        let (map_ctrls, remaining) = u32::try_parse(remaining)?;
        let (supported, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let result = GetNamedIndicatorReply { response_type, device_id, sequence, length, indicator, found, on, real_indicator, ndx, map_flags, map_which_groups, map_groups, map_which_mods, map_mods, map_real_mods, map_vmod, map_ctrls, supported };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetNamedIndicatorReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetNamedIndicator request
pub const SET_NAMED_INDICATOR_REQUEST: u8 = 16;
pub fn set_named_indicator<Conn, A, B, C, D, E, F, G, H>(conn: &Conn, device_spec: DeviceSpec, led_class: LedClass, led_id: A, indicator: xproto::Atom, set_state: bool, on: bool, set_map: bool, create_map: bool, map_flags: B, map_which_groups: C, map_groups: D, map_which_mods: E, map_real_mods: F, map_vmods: G, map_ctrls: H) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<IDSpec>,
    B: Into<u8>,
    C: Into<u8>,
    D: Into<u8>,
    E: Into<u8>,
    F: Into<u8>,
    G: Into<u16>,
    H: Into<u32>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let led_id: IDSpec = led_id.into();
    let map_flags: u8 = map_flags.into();
    let map_which_groups: u8 = map_which_groups.into();
    let map_groups: u8 = map_groups.into();
    let map_which_mods: u8 = map_which_mods.into();
    let map_real_mods: u8 = map_real_mods.into();
    let map_vmods: u16 = map_vmods.into();
    let map_ctrls: u32 = map_ctrls.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let led_class_bytes = LedClassSpec::from(led_class).serialize();
    let led_id_bytes = led_id.serialize();
    let indicator_bytes = indicator.serialize();
    let set_state_bytes = set_state.serialize();
    let on_bytes = on.serialize();
    let set_map_bytes = set_map.serialize();
    let create_map_bytes = create_map.serialize();
    let map_flags_bytes = map_flags.serialize();
    let map_which_groups_bytes = map_which_groups.serialize();
    let map_groups_bytes = map_groups.serialize();
    let map_which_mods_bytes = map_which_mods.serialize();
    let map_real_mods_bytes = map_real_mods.serialize();
    let map_vmods_bytes = map_vmods.serialize();
    let map_ctrls_bytes = map_ctrls.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_NAMED_INDICATOR_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        led_class_bytes[0],
        led_class_bytes[1],
        led_id_bytes[0],
        led_id_bytes[1],
        0,
        0,
        indicator_bytes[0],
        indicator_bytes[1],
        indicator_bytes[2],
        indicator_bytes[3],
        set_state_bytes[0],
        on_bytes[0],
        set_map_bytes[0],
        create_map_bytes[0],
        0,
        map_flags_bytes[0],
        map_which_groups_bytes[0],
        map_groups_bytes[0],
        map_which_mods_bytes[0],
        map_real_mods_bytes[0],
        map_vmods_bytes[0],
        map_vmods_bytes[1],
        map_ctrls_bytes[0],
        map_ctrls_bytes[1],
        map_ctrls_bytes[2],
        map_ctrls_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the GetNames request
pub const GET_NAMES_REQUEST: u8 = 17;
pub fn get_names<Conn, A>(conn: &Conn, device_spec: DeviceSpec, which: A) -> Result<Cookie<'_, Conn, GetNamesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let which: u32 = which.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let which_bytes = which.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_NAMES_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
        which_bytes[0],
        which_bytes[1],
        which_bytes[2],
        which_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetNamesValueListBitcase8 {
    pub n_levels_per_type: Vec<u8>,
    pub kt_level_names: Vec<xproto::Atom>,
}
impl GetNamesValueListBitcase8 {
    pub fn try_parse(remaining: &[u8], n_types: u8) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (n_levels_per_type, remaining) = crate::x11_utils::parse_u8_list(remaining, n_types.try_into().or(Err(ParseError::ParseError))?)?;
        let n_levels_per_type = n_levels_per_type.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (kt_level_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_levels_per_type.iter().try_fold(0u32, |acc, x| acc.checked_add(u32::from(*x)).ok_or(ParseError::ParseError))?.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetNamesValueListBitcase8 { n_levels_per_type, kt_level_names };
        Ok((result, remaining))
    }
}
// Skipping TryFrom implementations because of unresolved members
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GetNamesValueList {
    pub keycodes_name: Option<xproto::Atom>,
    pub geometry_name: Option<xproto::Atom>,
    pub symbols_name: Option<xproto::Atom>,
    pub phys_symbols_name: Option<xproto::Atom>,
    pub types_name: Option<xproto::Atom>,
    pub compat_name: Option<xproto::Atom>,
    pub type_names: Option<Vec<xproto::Atom>>,
    pub bitcase8: Option<GetNamesValueListBitcase8>,
    pub indicator_names: Option<Vec<xproto::Atom>>,
    pub virtual_mod_names: Option<Vec<xproto::Atom>>,
    pub groups: Option<Vec<xproto::Atom>>,
    pub key_names: Option<Vec<KeyName>>,
    pub key_aliases: Option<Vec<KeyAlias>>,
    pub radio_group_names: Option<Vec<xproto::Atom>>,
}
impl GetNamesValueList {
    fn try_parse(value: &[u8], which: u32, n_types: u8, indicators: u32, virtual_mods: u16, group_names: u8, n_keys: u8, n_key_aliases: u8, n_radio_groups: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = which;
        let mut outer_remaining = value;
        let keycodes_name = if switch_expr & u32::from(NameDetail::Keycodes) != 0 {
            let remaining = outer_remaining;
            let (keycodes_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(keycodes_name)
        } else {
            None
        };
        let geometry_name = if switch_expr & u32::from(NameDetail::Geometry) != 0 {
            let remaining = outer_remaining;
            let (geometry_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(geometry_name)
        } else {
            None
        };
        let symbols_name = if switch_expr & u32::from(NameDetail::Symbols) != 0 {
            let remaining = outer_remaining;
            let (symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(symbols_name)
        } else {
            None
        };
        let phys_symbols_name = if switch_expr & u32::from(NameDetail::PhysSymbols) != 0 {
            let remaining = outer_remaining;
            let (phys_symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(phys_symbols_name)
        } else {
            None
        };
        let types_name = if switch_expr & u32::from(NameDetail::Types) != 0 {
            let remaining = outer_remaining;
            let (types_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(types_name)
        } else {
            None
        };
        let compat_name = if switch_expr & u32::from(NameDetail::Compat) != 0 {
            let remaining = outer_remaining;
            let (compat_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(compat_name)
        } else {
            None
        };
        let type_names = if switch_expr & u32::from(NameDetail::KeyTypeNames) != 0 {
            let remaining = outer_remaining;
            let (type_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_types.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(type_names)
        } else {
            None
        };
        let bitcase8 = if switch_expr & u32::from(NameDetail::KTLevelNames) != 0 {
            let (bitcase8, new_remaining) = GetNamesValueListBitcase8::try_parse(outer_remaining, n_types)?;
            outer_remaining = new_remaining;
            Some(bitcase8)
        } else {
            None
        };
        let indicator_names = if switch_expr & u32::from(NameDetail::IndicatorNames) != 0 {
            let remaining = outer_remaining;
            let (indicator_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, indicators.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(indicator_names)
        } else {
            None
        };
        let virtual_mod_names = if switch_expr & u32::from(NameDetail::VirtualModNames) != 0 {
            let remaining = outer_remaining;
            let (virtual_mod_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, virtual_mods.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(virtual_mod_names)
        } else {
            None
        };
        let groups = if switch_expr & u32::from(NameDetail::GroupNames) != 0 {
            let remaining = outer_remaining;
            let (groups, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, group_names.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(groups)
        } else {
            None
        };
        let key_names = if switch_expr & u32::from(NameDetail::KeyNames) != 0 {
            let remaining = outer_remaining;
            let (key_names, remaining) = crate::x11_utils::parse_list::<KeyName>(remaining, n_keys.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(key_names)
        } else {
            None
        };
        let key_aliases = if switch_expr & u32::from(NameDetail::KeyAliases) != 0 {
            let remaining = outer_remaining;
            let (key_aliases, remaining) = crate::x11_utils::parse_list::<KeyAlias>(remaining, n_key_aliases.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(key_aliases)
        } else {
            None
        };
        let radio_group_names = if switch_expr & u32::from(NameDetail::RGNames) != 0 {
            let remaining = outer_remaining;
            let (radio_group_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_radio_groups.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(radio_group_names)
        } else {
            None
        };
        let result = GetNamesValueList { keycodes_name, geometry_name, symbols_name, phys_symbols_name, types_name, compat_name, type_names, bitcase8, indicator_names, virtual_mod_names, groups, key_names, key_aliases, radio_group_names };
        Ok((result, outer_remaining))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetNamesReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: xproto::Keycode,
    pub max_key_code: xproto::Keycode,
    pub n_types: u8,
    pub group_names: u8,
    pub virtual_mods: u16,
    pub first_key: xproto::Keycode,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_kt_levels: u16,
    pub value_list: GetNamesValueList,
}
impl TryParse for GetNamesReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (group_names, remaining) = u8::try_parse(remaining)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (first_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_keys, remaining) = u8::try_parse(remaining)?;
        let (indicators, remaining) = u32::try_parse(remaining)?;
        let (n_radio_groups, remaining) = u8::try_parse(remaining)?;
        let (n_key_aliases, remaining) = u8::try_parse(remaining)?;
        let (n_kt_levels, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (value_list, remaining) = GetNamesValueList::try_parse(remaining, which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups)?;
        let result = GetNamesReply { response_type, device_id, sequence, length, min_key_code, max_key_code, n_types, group_names, virtual_mods, first_key, n_keys, indicators, n_radio_groups, n_key_aliases, n_kt_levels, value_list };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetNamesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetNames request
pub const SET_NAMES_REQUEST: u8 = 18;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetNamesAuxBitcase8 {
    pub n_levels_per_type: Vec<u8>,
    pub kt_level_names: Vec<xproto::Atom>,
}
#[allow(dead_code, unused_variables)]
impl SetNamesAuxBitcase8 {
    fn serialize(&self, n_types: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, n_types);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, n_types: u8) {
        assert_eq!(self.n_levels_per_type.len(), usize::try_from(n_types).unwrap(), "`n_levels_per_type` has an incorrect length");
        bytes.extend_from_slice(&self.n_levels_per_type);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        assert_eq!(self.kt_level_names.len(), usize::try_from(self.n_levels_per_type.iter().fold(0u32, |acc, x| acc.checked_add(u32::from(*x)).unwrap())).unwrap(), "`kt_level_names` has an incorrect length");
        self.kt_level_names.serialize_into(bytes);
    }
}
/// Auxiliary and optional information for the `set_names` function
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SetNamesAux {
    pub keycodes_name: Option<xproto::Atom>,
    pub geometry_name: Option<xproto::Atom>,
    pub symbols_name: Option<xproto::Atom>,
    pub phys_symbols_name: Option<xproto::Atom>,
    pub types_name: Option<xproto::Atom>,
    pub compat_name: Option<xproto::Atom>,
    pub type_names: Option<Vec<xproto::Atom>>,
    pub bitcase8: Option<SetNamesAuxBitcase8>,
    pub indicator_names: Option<Vec<xproto::Atom>>,
    pub virtual_mod_names: Option<Vec<xproto::Atom>>,
    pub groups: Option<Vec<xproto::Atom>>,
    pub key_names: Option<Vec<KeyName>>,
    pub key_aliases: Option<Vec<KeyAlias>>,
    pub radio_group_names: Option<Vec<xproto::Atom>>,
}
#[allow(dead_code, unused_variables)]
impl SetNamesAux {
    fn serialize(&self, which: u32, n_types: u8, indicators: u32, virtual_mods: u16, group_names: u8, n_keys: u8, n_key_aliases: u8, n_radio_groups: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, which: u32, n_types: u8, indicators: u32, virtual_mods: u16, group_names: u8, n_keys: u8, n_key_aliases: u8, n_radio_groups: u8) {
        if let Some(keycodes_name) = self.keycodes_name {
            keycodes_name.serialize_into(bytes);
        }
        if let Some(geometry_name) = self.geometry_name {
            geometry_name.serialize_into(bytes);
        }
        if let Some(symbols_name) = self.symbols_name {
            symbols_name.serialize_into(bytes);
        }
        if let Some(phys_symbols_name) = self.phys_symbols_name {
            phys_symbols_name.serialize_into(bytes);
        }
        if let Some(types_name) = self.types_name {
            types_name.serialize_into(bytes);
        }
        if let Some(compat_name) = self.compat_name {
            compat_name.serialize_into(bytes);
        }
        if let Some(ref type_names) = self.type_names {
            assert_eq!(type_names.len(), usize::try_from(n_types).unwrap(), "`type_names` has an incorrect length");
            type_names.serialize_into(bytes);
        }
        if let Some(ref bitcase8) = self.bitcase8 {
            bitcase8.serialize_into(bytes, n_types);
        }
        if let Some(ref indicator_names) = self.indicator_names {
            assert_eq!(indicator_names.len(), usize::try_from(indicators.count_ones()).unwrap(), "`indicator_names` has an incorrect length");
            indicator_names.serialize_into(bytes);
        }
        if let Some(ref virtual_mod_names) = self.virtual_mod_names {
            assert_eq!(virtual_mod_names.len(), usize::try_from(virtual_mods.count_ones()).unwrap(), "`virtual_mod_names` has an incorrect length");
            virtual_mod_names.serialize_into(bytes);
        }
        if let Some(ref groups) = self.groups {
            assert_eq!(groups.len(), usize::try_from(group_names.count_ones()).unwrap(), "`groups` has an incorrect length");
            groups.serialize_into(bytes);
        }
        if let Some(ref key_names) = self.key_names {
            assert_eq!(key_names.len(), usize::try_from(n_keys).unwrap(), "`key_names` has an incorrect length");
            key_names.serialize_into(bytes);
        }
        if let Some(ref key_aliases) = self.key_aliases {
            assert_eq!(key_aliases.len(), usize::try_from(n_key_aliases).unwrap(), "`key_aliases` has an incorrect length");
            key_aliases.serialize_into(bytes);
        }
        if let Some(ref radio_group_names) = self.radio_group_names {
            assert_eq!(radio_group_names.len(), usize::try_from(n_radio_groups).unwrap(), "`radio_group_names` has an incorrect length");
            radio_group_names.serialize_into(bytes);
        }
    }
}
impl SetNamesAux {
    fn switch_expr(&self) -> u32 {
        let mut expr_value = 0;
        if self.keycodes_name.is_some() {
            expr_value |= u32::from(NameDetail::Keycodes);
        }
        if self.geometry_name.is_some() {
            expr_value |= u32::from(NameDetail::Geometry);
        }
        if self.symbols_name.is_some() {
            expr_value |= u32::from(NameDetail::Symbols);
        }
        if self.phys_symbols_name.is_some() {
            expr_value |= u32::from(NameDetail::PhysSymbols);
        }
        if self.types_name.is_some() {
            expr_value |= u32::from(NameDetail::Types);
        }
        if self.compat_name.is_some() {
            expr_value |= u32::from(NameDetail::Compat);
        }
        if self.type_names.is_some() {
            expr_value |= u32::from(NameDetail::KeyTypeNames);
        }
        if self.bitcase8.is_some() {
            expr_value |= u32::from(NameDetail::KTLevelNames);
        }
        if self.indicator_names.is_some() {
            expr_value |= u32::from(NameDetail::IndicatorNames);
        }
        if self.virtual_mod_names.is_some() {
            expr_value |= u32::from(NameDetail::VirtualModNames);
        }
        if self.groups.is_some() {
            expr_value |= u32::from(NameDetail::GroupNames);
        }
        if self.key_names.is_some() {
            expr_value |= u32::from(NameDetail::KeyNames);
        }
        if self.key_aliases.is_some() {
            expr_value |= u32::from(NameDetail::KeyAliases);
        }
        if self.radio_group_names.is_some() {
            expr_value |= u32::from(NameDetail::RGNames);
        }
        expr_value
    }
}
impl SetNamesAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the `keycodes_name` field of this structure.
    pub fn keycodes_name<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Atom>> {
        self.keycodes_name = value.into();
        self
    }
    /// Set the `geometry_name` field of this structure.
    pub fn geometry_name<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Atom>> {
        self.geometry_name = value.into();
        self
    }
    /// Set the `symbols_name` field of this structure.
    pub fn symbols_name<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Atom>> {
        self.symbols_name = value.into();
        self
    }
    /// Set the `phys_symbols_name` field of this structure.
    pub fn phys_symbols_name<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Atom>> {
        self.phys_symbols_name = value.into();
        self
    }
    /// Set the `types_name` field of this structure.
    pub fn types_name<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Atom>> {
        self.types_name = value.into();
        self
    }
    /// Set the `compat_name` field of this structure.
    pub fn compat_name<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Atom>> {
        self.compat_name = value.into();
        self
    }
    /// Set the `type_names` field of this structure.
    pub fn type_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<xproto::Atom>>> {
        self.type_names = value.into();
        self
    }
    /// Set the `bitcase8` field of this structure.
    pub fn bitcase8<I>(mut self, value: I) -> Self where I: Into<Option<SetNamesAuxBitcase8>> {
        self.bitcase8 = value.into();
        self
    }
    /// Set the `indicator_names` field of this structure.
    pub fn indicator_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<xproto::Atom>>> {
        self.indicator_names = value.into();
        self
    }
    /// Set the `virtual_mod_names` field of this structure.
    pub fn virtual_mod_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<xproto::Atom>>> {
        self.virtual_mod_names = value.into();
        self
    }
    /// Set the `groups` field of this structure.
    pub fn groups<I>(mut self, value: I) -> Self where I: Into<Option<Vec<xproto::Atom>>> {
        self.groups = value.into();
        self
    }
    /// Set the `key_names` field of this structure.
    pub fn key_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeyName>>> {
        self.key_names = value.into();
        self
    }
    /// Set the `key_aliases` field of this structure.
    pub fn key_aliases<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeyAlias>>> {
        self.key_aliases = value.into();
        self
    }
    /// Set the `radio_group_names` field of this structure.
    pub fn radio_group_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<xproto::Atom>>> {
        self.radio_group_names = value.into();
        self
    }
}

pub fn set_names<'c, 'input, Conn, A, B>(conn: &'c Conn, device_spec: DeviceSpec, virtual_mods: A, first_type: u8, n_types: u8, first_kt_levelt: u8, n_kt_levels: u8, indicators: u32, group_names: B, n_radio_groups: u8, first_key: xproto::Keycode, n_keys: u8, n_key_aliases: u8, total_kt_level_names: u16, values: &'input SetNamesAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let virtual_mods: u16 = virtual_mods.into();
    let group_names: u8 = group_names.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let virtual_mods_bytes = virtual_mods.serialize();
    let which = u32::try_from(values.switch_expr()).unwrap();
    let which_bytes = which.serialize();
    let first_type_bytes = first_type.serialize();
    let n_types_bytes = n_types.serialize();
    let first_kt_levelt_bytes = first_kt_levelt.serialize();
    let n_kt_levels_bytes = n_kt_levels.serialize();
    let indicators_bytes = indicators.serialize();
    let group_names_bytes = group_names.serialize();
    let n_radio_groups_bytes = n_radio_groups.serialize();
    let first_key_bytes = first_key.serialize();
    let n_keys_bytes = n_keys.serialize();
    let n_key_aliases_bytes = n_key_aliases.serialize();
    let total_kt_level_names_bytes = total_kt_level_names.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_NAMES_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        virtual_mods_bytes[0],
        virtual_mods_bytes[1],
        which_bytes[0],
        which_bytes[1],
        which_bytes[2],
        which_bytes[3],
        first_type_bytes[0],
        n_types_bytes[0],
        first_kt_levelt_bytes[0],
        n_kt_levels_bytes[0],
        indicators_bytes[0],
        indicators_bytes[1],
        indicators_bytes[2],
        indicators_bytes[3],
        group_names_bytes[0],
        n_radio_groups_bytes[0],
        first_key_bytes[0],
        n_keys_bytes[0],
        n_key_aliases_bytes[0],
        0,
        total_kt_level_names_bytes[0],
        total_kt_level_names_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    let values_bytes = values.serialize(which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups);
    let length_so_far = length_so_far + values_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&values_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the PerClientFlags request
pub const PER_CLIENT_FLAGS_REQUEST: u8 = 21;
pub fn per_client_flags<Conn, A, B, C, D, E>(conn: &Conn, device_spec: DeviceSpec, change: A, value: B, ctrls_to_change: C, auto_ctrls: D, auto_ctrls_values: E) -> Result<Cookie<'_, Conn, PerClientFlagsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
    B: Into<u32>,
    C: Into<u32>,
    D: Into<u32>,
    E: Into<u32>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let change: u32 = change.into();
    let value: u32 = value.into();
    let ctrls_to_change: u32 = ctrls_to_change.into();
    let auto_ctrls: u32 = auto_ctrls.into();
    let auto_ctrls_values: u32 = auto_ctrls_values.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let change_bytes = change.serialize();
    let value_bytes = value.serialize();
    let ctrls_to_change_bytes = ctrls_to_change.serialize();
    let auto_ctrls_bytes = auto_ctrls.serialize();
    let auto_ctrls_values_bytes = auto_ctrls_values.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        PER_CLIENT_FLAGS_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
        change_bytes[0],
        change_bytes[1],
        change_bytes[2],
        change_bytes[3],
        value_bytes[0],
        value_bytes[1],
        value_bytes[2],
        value_bytes[3],
        ctrls_to_change_bytes[0],
        ctrls_to_change_bytes[1],
        ctrls_to_change_bytes[2],
        ctrls_to_change_bytes[3],
        auto_ctrls_bytes[0],
        auto_ctrls_bytes[1],
        auto_ctrls_bytes[2],
        auto_ctrls_bytes[3],
        auto_ctrls_values_bytes[0],
        auto_ctrls_values_bytes[1],
        auto_ctrls_values_bytes[2],
        auto_ctrls_values_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerClientFlagsReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub supported: u32,
    pub value: u32,
    pub auto_ctrls: u32,
    pub auto_ctrls_values: u32,
}
impl TryParse for PerClientFlagsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (supported, remaining) = u32::try_parse(remaining)?;
        let (value, remaining) = u32::try_parse(remaining)?;
        let (auto_ctrls, remaining) = u32::try_parse(remaining)?;
        let (auto_ctrls_values, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let result = PerClientFlagsReply { response_type, device_id, sequence, length, supported, value, auto_ctrls, auto_ctrls_values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PerClientFlagsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListComponents request
pub const LIST_COMPONENTS_REQUEST: u8 = 22;
pub fn list_components<Conn>(conn: &Conn, device_spec: DeviceSpec, max_names: u16) -> Result<Cookie<'_, Conn, ListComponentsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let max_names_bytes = max_names.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        LIST_COMPONENTS_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        max_names_bytes[0],
        max_names_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListComponentsReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub extra: u16,
    pub keymaps: Vec<Listing>,
    pub keycodes: Vec<Listing>,
    pub types: Vec<Listing>,
    pub compat_maps: Vec<Listing>,
    pub symbols: Vec<Listing>,
    pub geometries: Vec<Listing>,
}
impl TryParse for ListComponentsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (n_keymaps, remaining) = u16::try_parse(remaining)?;
        let (n_keycodes, remaining) = u16::try_parse(remaining)?;
        let (n_types, remaining) = u16::try_parse(remaining)?;
        let (n_compat_maps, remaining) = u16::try_parse(remaining)?;
        let (n_symbols, remaining) = u16::try_parse(remaining)?;
        let (n_geometries, remaining) = u16::try_parse(remaining)?;
        let (extra, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(10..).ok_or(ParseError::ParseError)?;
        let (keymaps, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_keymaps.try_into().or(Err(ParseError::ParseError))?)?;
        let (keycodes, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_keycodes.try_into().or(Err(ParseError::ParseError))?)?;
        let (types, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_types.try_into().or(Err(ParseError::ParseError))?)?;
        let (compat_maps, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_compat_maps.try_into().or(Err(ParseError::ParseError))?)?;
        let (symbols, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_symbols.try_into().or(Err(ParseError::ParseError))?)?;
        let (geometries, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_geometries.try_into().or(Err(ParseError::ParseError))?)?;
        let result = ListComponentsReply { response_type, device_id, sequence, length, extra, keymaps, keycodes, types, compat_maps, symbols, geometries };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListComponentsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl ListComponentsReply {
    /// Get the value of the `nKeymaps` field.
    ///
    /// The `nKeymaps` field is used as the length field of the `keymaps` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_keymaps(&self) -> u16 {
        self.keymaps.len()
            .try_into().unwrap()
    }
    /// Get the value of the `nKeycodes` field.
    ///
    /// The `nKeycodes` field is used as the length field of the `keycodes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_keycodes(&self) -> u16 {
        self.keycodes.len()
            .try_into().unwrap()
    }
    /// Get the value of the `nTypes` field.
    ///
    /// The `nTypes` field is used as the length field of the `types` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_types(&self) -> u16 {
        self.types.len()
            .try_into().unwrap()
    }
    /// Get the value of the `nCompatMaps` field.
    ///
    /// The `nCompatMaps` field is used as the length field of the `compatMaps` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_compat_maps(&self) -> u16 {
        self.compat_maps.len()
            .try_into().unwrap()
    }
    /// Get the value of the `nSymbols` field.
    ///
    /// The `nSymbols` field is used as the length field of the `symbols` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_symbols(&self) -> u16 {
        self.symbols.len()
            .try_into().unwrap()
    }
    /// Get the value of the `nGeometries` field.
    ///
    /// The `nGeometries` field is used as the length field of the `geometries` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_geometries(&self) -> u16 {
        self.geometries.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GetKbdByName request
pub const GET_KBD_BY_NAME_REQUEST: u8 = 23;
pub fn get_kbd_by_name<Conn, A, B>(conn: &Conn, device_spec: DeviceSpec, need: A, want: B, load: bool) -> Result<Cookie<'_, Conn, GetKbdByNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u16>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let need: u16 = need.into();
    let want: u16 = want.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let need_bytes = need.serialize();
    let want_bytes = want.serialize();
    let load_bytes = load.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_KBD_BY_NAME_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        need_bytes[0],
        need_bytes[1],
        want_bytes[0],
        want_bytes[1],
        load_bytes[0],
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone)]
pub struct GetKbdByNameRepliesTypesMapBitcase3 {
    pub acts_rtrn_count: Vec<u8>,
    pub acts_rtrn_acts: Vec<Action>,
}
impl GetKbdByNameRepliesTypesMapBitcase3 {
    pub fn try_parse(remaining: &[u8], n_key_actions: u8, total_actions: u16) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (acts_rtrn_count, remaining) = crate::x11_utils::parse_u8_list(remaining, n_key_actions.try_into().or(Err(ParseError::ParseError))?)?;
        let acts_rtrn_count = acts_rtrn_count.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (acts_rtrn_acts, remaining) = crate::x11_utils::parse_list::<Action>(remaining, total_actions.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetKbdByNameRepliesTypesMapBitcase3 { acts_rtrn_count, acts_rtrn_acts };
        Ok((result, remaining))
    }
}
// Skipping TryFrom implementations because of unresolved members
#[derive(Debug, Clone, Default)]
pub struct GetKbdByNameRepliesTypesMap {
    pub types_rtrn: Option<Vec<KeyType>>,
    pub syms_rtrn: Option<Vec<KeySymMap>>,
    pub bitcase3: Option<GetKbdByNameRepliesTypesMapBitcase3>,
    pub behaviors_rtrn: Option<Vec<SetBehavior>>,
    pub vmods_rtrn: Option<Vec<u8>>,
    pub explicit_rtrn: Option<Vec<SetExplicit>>,
    pub modmap_rtrn: Option<Vec<KeyModMap>>,
    pub vmodmap_rtrn: Option<Vec<KeyVModMap>>,
}
impl GetKbdByNameRepliesTypesMap {
    fn try_parse(value: &[u8], present: u16, n_types: u8, n_key_syms: u8, n_key_actions: u8, total_actions: u16, total_key_behaviors: u8, virtual_mods: u16, total_key_explicit: u8, total_mod_map_keys: u8, total_v_mod_map_keys: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = u32::from(present);
        let mut outer_remaining = value;
        let types_rtrn = if switch_expr & u32::from(MapPart::KeyTypes) != 0 {
            let remaining = outer_remaining;
            let (types_rtrn, remaining) = crate::x11_utils::parse_list::<KeyType>(remaining, n_types.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(types_rtrn)
        } else {
            None
        };
        let syms_rtrn = if switch_expr & u32::from(MapPart::KeySyms) != 0 {
            let remaining = outer_remaining;
            let (syms_rtrn, remaining) = crate::x11_utils::parse_list::<KeySymMap>(remaining, n_key_syms.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(syms_rtrn)
        } else {
            None
        };
        let bitcase3 = if switch_expr & u32::from(MapPart::KeyActions) != 0 {
            let (bitcase3, new_remaining) = GetKbdByNameRepliesTypesMapBitcase3::try_parse(outer_remaining, n_key_actions, total_actions)?;
            outer_remaining = new_remaining;
            Some(bitcase3)
        } else {
            None
        };
        let behaviors_rtrn = if switch_expr & u32::from(MapPart::KeyBehaviors) != 0 {
            let remaining = outer_remaining;
            let (behaviors_rtrn, remaining) = crate::x11_utils::parse_list::<SetBehavior>(remaining, total_key_behaviors.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(behaviors_rtrn)
        } else {
            None
        };
        let vmods_rtrn = if switch_expr & u32::from(MapPart::VirtualMods) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (vmods_rtrn, remaining) = crate::x11_utils::parse_u8_list(remaining, virtual_mods.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
            let vmods_rtrn = vmods_rtrn.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(vmods_rtrn)
        } else {
            None
        };
        let explicit_rtrn = if switch_expr & u32::from(MapPart::ExplicitComponents) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (explicit_rtrn, remaining) = crate::x11_utils::parse_list::<SetExplicit>(remaining, total_key_explicit.try_into().or(Err(ParseError::ParseError))?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(explicit_rtrn)
        } else {
            None
        };
        let modmap_rtrn = if switch_expr & u32::from(MapPart::ModifierMap) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (modmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyModMap>(remaining, total_mod_map_keys.try_into().or(Err(ParseError::ParseError))?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(modmap_rtrn)
        } else {
            None
        };
        let vmodmap_rtrn = if switch_expr & u32::from(MapPart::VirtualModMap) != 0 {
            let remaining = outer_remaining;
            let (vmodmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyVModMap>(remaining, total_v_mod_map_keys.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(vmodmap_rtrn)
        } else {
            None
        };
        let result = GetKbdByNameRepliesTypesMap { types_rtrn, syms_rtrn, bitcase3, behaviors_rtrn, vmods_rtrn, explicit_rtrn, modmap_rtrn, vmodmap_rtrn };
        Ok((result, outer_remaining))
    }
}

#[derive(Debug, Clone)]
pub struct GetKbdByNameRepliesTypes {
    pub getmap_type: u8,
    pub type_device_id: u8,
    pub getmap_sequence: u16,
    pub getmap_length: u32,
    pub type_min_key_code: xproto::Keycode,
    pub type_max_key_code: xproto::Keycode,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: xproto::Keycode,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: xproto::Keycode,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: xproto::Keycode,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xproto::Keycode,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: xproto::Keycode,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub virtual_mods: u16,
    pub map: GetKbdByNameRepliesTypesMap,
}
impl TryParse for GetKbdByNameRepliesTypes {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (getmap_type, remaining) = u8::try_parse(remaining)?;
        let (type_device_id, remaining) = u8::try_parse(remaining)?;
        let (getmap_sequence, remaining) = u16::try_parse(remaining)?;
        let (getmap_length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (type_min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (type_max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (present, remaining) = u16::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (total_types, remaining) = u8::try_parse(remaining)?;
        let (first_key_sym, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (total_syms, remaining) = u16::try_parse(remaining)?;
        let (n_key_syms, remaining) = u8::try_parse(remaining)?;
        let (first_key_action, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (total_actions, remaining) = u16::try_parse(remaining)?;
        let (n_key_actions, remaining) = u8::try_parse(remaining)?;
        let (first_key_behavior, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_behaviors, remaining) = u8::try_parse(remaining)?;
        let (total_key_behaviors, remaining) = u8::try_parse(remaining)?;
        let (first_key_explicit, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (total_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (first_mod_map_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (total_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (first_v_mod_map_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_v_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (total_v_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (map, remaining) = GetKbdByNameRepliesTypesMap::try_parse(remaining, present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys)?;
        let result = GetKbdByNameRepliesTypes { getmap_type, type_device_id, getmap_sequence, getmap_length, type_min_key_code, type_max_key_code, first_type, n_types, total_types, first_key_sym, total_syms, n_key_syms, first_key_action, total_actions, n_key_actions, first_key_behavior, n_key_behaviors, total_key_behaviors, first_key_explicit, n_key_explicit, total_key_explicit, first_mod_map_key, n_mod_map_keys, total_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, total_v_mod_map_keys, virtual_mods, map };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetKbdByNameRepliesTypes {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetKbdByNameRepliesCompatMap {
    pub compatmap_type: u8,
    pub compat_device_id: u8,
    pub compatmap_sequence: u16,
    pub compatmap_length: u32,
    pub groups_rtrn: u8,
    pub first_si_rtrn: u16,
    pub n_total_si: u16,
    pub si_rtrn: Vec<SymInterpret>,
    pub group_rtrn: Vec<ModDef>,
}
impl TryParse for GetKbdByNameRepliesCompatMap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (compatmap_type, remaining) = u8::try_parse(remaining)?;
        let (compat_device_id, remaining) = u8::try_parse(remaining)?;
        let (compatmap_sequence, remaining) = u16::try_parse(remaining)?;
        let (compatmap_length, remaining) = u32::try_parse(remaining)?;
        let (groups_rtrn, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (first_si_rtrn, remaining) = u16::try_parse(remaining)?;
        let (n_si_rtrn, remaining) = u16::try_parse(remaining)?;
        let (n_total_si, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (si_rtrn, remaining) = crate::x11_utils::parse_list::<SymInterpret>(remaining, n_si_rtrn.try_into().or(Err(ParseError::ParseError))?)?;
        let (group_rtrn, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, groups_rtrn.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetKbdByNameRepliesCompatMap { compatmap_type, compat_device_id, compatmap_sequence, compatmap_length, groups_rtrn, first_si_rtrn, n_total_si, si_rtrn, group_rtrn };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetKbdByNameRepliesCompatMap {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetKbdByNameRepliesCompatMap {
    /// Get the value of the `nSIRtrn` field.
    ///
    /// The `nSIRtrn` field is used as the length field of the `si_rtrn` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_si_rtrn(&self) -> u16 {
        self.si_rtrn.len()
            .try_into().unwrap()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetKbdByNameRepliesIndicatorMaps {
    pub indicatormap_type: u8,
    pub indicator_device_id: u8,
    pub indicatormap_sequence: u16,
    pub indicatormap_length: u32,
    pub which: u32,
    pub real_indicators: u32,
    pub maps: Vec<IndicatorMap>,
}
impl TryParse for GetKbdByNameRepliesIndicatorMaps {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (indicatormap_type, remaining) = u8::try_parse(remaining)?;
        let (indicator_device_id, remaining) = u8::try_parse(remaining)?;
        let (indicatormap_sequence, remaining) = u16::try_parse(remaining)?;
        let (indicatormap_length, remaining) = u32::try_parse(remaining)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (real_indicators, remaining) = u32::try_parse(remaining)?;
        let (n_indicators, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::ParseError)?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, n_indicators.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetKbdByNameRepliesIndicatorMaps { indicatormap_type, indicator_device_id, indicatormap_sequence, indicatormap_length, which, real_indicators, maps };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetKbdByNameRepliesIndicatorMaps {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetKbdByNameRepliesIndicatorMaps {
    /// Get the value of the `nIndicators` field.
    ///
    /// The `nIndicators` field is used as the length field of the `maps` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_indicators(&self) -> u8 {
        self.maps.len()
            .try_into().unwrap()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetKbdByNameRepliesKeyNamesValueListBitcase8 {
    pub n_levels_per_type: Vec<u8>,
    pub kt_level_names: Vec<xproto::Atom>,
}
impl GetKbdByNameRepliesKeyNamesValueListBitcase8 {
    pub fn try_parse(remaining: &[u8], n_types: u8) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (n_levels_per_type, remaining) = crate::x11_utils::parse_u8_list(remaining, n_types.try_into().or(Err(ParseError::ParseError))?)?;
        let n_levels_per_type = n_levels_per_type.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (kt_level_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_levels_per_type.iter().try_fold(0u32, |acc, x| acc.checked_add(u32::from(*x)).ok_or(ParseError::ParseError))?.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetKbdByNameRepliesKeyNamesValueListBitcase8 { n_levels_per_type, kt_level_names };
        Ok((result, remaining))
    }
}
// Skipping TryFrom implementations because of unresolved members
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GetKbdByNameRepliesKeyNamesValueList {
    pub keycodes_name: Option<xproto::Atom>,
    pub geometry_name: Option<xproto::Atom>,
    pub symbols_name: Option<xproto::Atom>,
    pub phys_symbols_name: Option<xproto::Atom>,
    pub types_name: Option<xproto::Atom>,
    pub compat_name: Option<xproto::Atom>,
    pub type_names: Option<Vec<xproto::Atom>>,
    pub bitcase8: Option<GetKbdByNameRepliesKeyNamesValueListBitcase8>,
    pub indicator_names: Option<Vec<xproto::Atom>>,
    pub virtual_mod_names: Option<Vec<xproto::Atom>>,
    pub groups: Option<Vec<xproto::Atom>>,
    pub key_names: Option<Vec<KeyName>>,
    pub key_aliases: Option<Vec<KeyAlias>>,
    pub radio_group_names: Option<Vec<xproto::Atom>>,
}
impl GetKbdByNameRepliesKeyNamesValueList {
    fn try_parse(value: &[u8], which: u32, n_types: u8, indicators: u32, virtual_mods: u16, group_names: u8, n_keys: u8, n_key_aliases: u8, n_radio_groups: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = which;
        let mut outer_remaining = value;
        let keycodes_name = if switch_expr & u32::from(NameDetail::Keycodes) != 0 {
            let remaining = outer_remaining;
            let (keycodes_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(keycodes_name)
        } else {
            None
        };
        let geometry_name = if switch_expr & u32::from(NameDetail::Geometry) != 0 {
            let remaining = outer_remaining;
            let (geometry_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(geometry_name)
        } else {
            None
        };
        let symbols_name = if switch_expr & u32::from(NameDetail::Symbols) != 0 {
            let remaining = outer_remaining;
            let (symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(symbols_name)
        } else {
            None
        };
        let phys_symbols_name = if switch_expr & u32::from(NameDetail::PhysSymbols) != 0 {
            let remaining = outer_remaining;
            let (phys_symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(phys_symbols_name)
        } else {
            None
        };
        let types_name = if switch_expr & u32::from(NameDetail::Types) != 0 {
            let remaining = outer_remaining;
            let (types_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(types_name)
        } else {
            None
        };
        let compat_name = if switch_expr & u32::from(NameDetail::Compat) != 0 {
            let remaining = outer_remaining;
            let (compat_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(compat_name)
        } else {
            None
        };
        let type_names = if switch_expr & u32::from(NameDetail::KeyTypeNames) != 0 {
            let remaining = outer_remaining;
            let (type_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_types.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(type_names)
        } else {
            None
        };
        let bitcase8 = if switch_expr & u32::from(NameDetail::KTLevelNames) != 0 {
            let (bitcase8, new_remaining) = GetKbdByNameRepliesKeyNamesValueListBitcase8::try_parse(outer_remaining, n_types)?;
            outer_remaining = new_remaining;
            Some(bitcase8)
        } else {
            None
        };
        let indicator_names = if switch_expr & u32::from(NameDetail::IndicatorNames) != 0 {
            let remaining = outer_remaining;
            let (indicator_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, indicators.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(indicator_names)
        } else {
            None
        };
        let virtual_mod_names = if switch_expr & u32::from(NameDetail::VirtualModNames) != 0 {
            let remaining = outer_remaining;
            let (virtual_mod_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, virtual_mods.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(virtual_mod_names)
        } else {
            None
        };
        let groups = if switch_expr & u32::from(NameDetail::GroupNames) != 0 {
            let remaining = outer_remaining;
            let (groups, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, group_names.count_ones().try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(groups)
        } else {
            None
        };
        let key_names = if switch_expr & u32::from(NameDetail::KeyNames) != 0 {
            let remaining = outer_remaining;
            let (key_names, remaining) = crate::x11_utils::parse_list::<KeyName>(remaining, n_keys.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(key_names)
        } else {
            None
        };
        let key_aliases = if switch_expr & u32::from(NameDetail::KeyAliases) != 0 {
            let remaining = outer_remaining;
            let (key_aliases, remaining) = crate::x11_utils::parse_list::<KeyAlias>(remaining, n_key_aliases.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(key_aliases)
        } else {
            None
        };
        let radio_group_names = if switch_expr & u32::from(NameDetail::RGNames) != 0 {
            let remaining = outer_remaining;
            let (radio_group_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_radio_groups.try_into().or(Err(ParseError::ParseError))?)?;
            outer_remaining = remaining;
            Some(radio_group_names)
        } else {
            None
        };
        let result = GetKbdByNameRepliesKeyNamesValueList { keycodes_name, geometry_name, symbols_name, phys_symbols_name, types_name, compat_name, type_names, bitcase8, indicator_names, virtual_mod_names, groups, key_names, key_aliases, radio_group_names };
        Ok((result, outer_remaining))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetKbdByNameRepliesKeyNames {
    pub keyname_type: u8,
    pub key_device_id: u8,
    pub keyname_sequence: u16,
    pub keyname_length: u32,
    pub key_min_key_code: xproto::Keycode,
    pub key_max_key_code: xproto::Keycode,
    pub n_types: u8,
    pub group_names: u8,
    pub virtual_mods: u16,
    pub first_key: xproto::Keycode,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_kt_levels: u16,
    pub value_list: GetKbdByNameRepliesKeyNamesValueList,
}
impl TryParse for GetKbdByNameRepliesKeyNames {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keyname_type, remaining) = u8::try_parse(remaining)?;
        let (key_device_id, remaining) = u8::try_parse(remaining)?;
        let (keyname_sequence, remaining) = u16::try_parse(remaining)?;
        let (keyname_length, remaining) = u32::try_parse(remaining)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (key_min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (key_max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (group_names, remaining) = u8::try_parse(remaining)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (first_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_keys, remaining) = u8::try_parse(remaining)?;
        let (indicators, remaining) = u32::try_parse(remaining)?;
        let (n_radio_groups, remaining) = u8::try_parse(remaining)?;
        let (n_key_aliases, remaining) = u8::try_parse(remaining)?;
        let (n_kt_levels, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (value_list, remaining) = GetKbdByNameRepliesKeyNamesValueList::try_parse(remaining, which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups)?;
        let result = GetKbdByNameRepliesKeyNames { keyname_type, key_device_id, keyname_sequence, keyname_length, key_min_key_code, key_max_key_code, n_types, group_names, virtual_mods, first_key, n_keys, indicators, n_radio_groups, n_key_aliases, n_kt_levels, value_list };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetKbdByNameRepliesKeyNames {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetKbdByNameRepliesGeometry {
    pub geometry_type: u8,
    pub geometry_device_id: u8,
    pub geometry_sequence: u16,
    pub geometry_length: u32,
    pub name: xproto::Atom,
    pub geometry_found: bool,
    pub width_mm: u16,
    pub height_mm: u16,
    pub n_properties: u16,
    pub n_colors: u16,
    pub n_shapes: u16,
    pub n_sections: u16,
    pub n_doodads: u16,
    pub n_key_aliases: u16,
    pub base_color_ndx: u8,
    pub label_color_ndx: u8,
    pub label_font: CountedString16,
}
impl TryParse for GetKbdByNameRepliesGeometry {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (geometry_type, remaining) = u8::try_parse(remaining)?;
        let (geometry_device_id, remaining) = u8::try_parse(remaining)?;
        let (geometry_sequence, remaining) = u16::try_parse(remaining)?;
        let (geometry_length, remaining) = u32::try_parse(remaining)?;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (geometry_found, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (width_mm, remaining) = u16::try_parse(remaining)?;
        let (height_mm, remaining) = u16::try_parse(remaining)?;
        let (n_properties, remaining) = u16::try_parse(remaining)?;
        let (n_colors, remaining) = u16::try_parse(remaining)?;
        let (n_shapes, remaining) = u16::try_parse(remaining)?;
        let (n_sections, remaining) = u16::try_parse(remaining)?;
        let (n_doodads, remaining) = u16::try_parse(remaining)?;
        let (n_key_aliases, remaining) = u16::try_parse(remaining)?;
        let (base_color_ndx, remaining) = u8::try_parse(remaining)?;
        let (label_color_ndx, remaining) = u8::try_parse(remaining)?;
        let (label_font, remaining) = CountedString16::try_parse(remaining)?;
        let result = GetKbdByNameRepliesGeometry { geometry_type, geometry_device_id, geometry_sequence, geometry_length, name, geometry_found, width_mm, height_mm, n_properties, n_colors, n_shapes, n_sections, n_doodads, n_key_aliases, base_color_ndx, label_color_ndx, label_font };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetKbdByNameRepliesGeometry {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetKbdByNameReplies {
    pub types: Option<GetKbdByNameRepliesTypes>,
    pub compat_map: Option<GetKbdByNameRepliesCompatMap>,
    pub indicator_maps: Option<GetKbdByNameRepliesIndicatorMaps>,
    pub key_names: Option<GetKbdByNameRepliesKeyNames>,
    pub geometry: Option<GetKbdByNameRepliesGeometry>,
}
impl GetKbdByNameReplies {
    fn try_parse(value: &[u8], reported: u16) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = u32::from(reported);
        let mut outer_remaining = value;
        let types = if switch_expr & u32::from(GBNDetail::Types) != 0 || switch_expr & u32::from(GBNDetail::ClientSymbols) != 0 || switch_expr & u32::from(GBNDetail::ServerSymbols) != 0 {
            let (types, new_remaining) = GetKbdByNameRepliesTypes::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(types)
        } else {
            None
        };
        let compat_map = if switch_expr & u32::from(GBNDetail::CompatMap) != 0 {
            let (compat_map, new_remaining) = GetKbdByNameRepliesCompatMap::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(compat_map)
        } else {
            None
        };
        let indicator_maps = if switch_expr & u32::from(GBNDetail::IndicatorMaps) != 0 {
            let (indicator_maps, new_remaining) = GetKbdByNameRepliesIndicatorMaps::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(indicator_maps)
        } else {
            None
        };
        let key_names = if switch_expr & u32::from(GBNDetail::KeyNames) != 0 || switch_expr & u32::from(GBNDetail::OtherNames) != 0 {
            let (key_names, new_remaining) = GetKbdByNameRepliesKeyNames::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(key_names)
        } else {
            None
        };
        let geometry = if switch_expr & u32::from(GBNDetail::Geometry) != 0 {
            let (geometry, new_remaining) = GetKbdByNameRepliesGeometry::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(geometry)
        } else {
            None
        };
        let result = GetKbdByNameReplies { types, compat_map, indicator_maps, key_names, geometry };
        Ok((result, outer_remaining))
    }
}

#[derive(Debug, Clone)]
pub struct GetKbdByNameReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: xproto::Keycode,
    pub max_key_code: xproto::Keycode,
    pub loaded: bool,
    pub new_keyboard: bool,
    pub found: u16,
    pub reported: u16,
    pub replies: GetKbdByNameReplies,
}
impl TryParse for GetKbdByNameReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (loaded, remaining) = bool::try_parse(remaining)?;
        let (new_keyboard, remaining) = bool::try_parse(remaining)?;
        let (found, remaining) = u16::try_parse(remaining)?;
        let (reported, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (replies, remaining) = GetKbdByNameReplies::try_parse(remaining, reported)?;
        let result = GetKbdByNameReply { response_type, device_id, sequence, length, min_key_code, max_key_code, loaded, new_keyboard, found, reported, replies };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetKbdByNameReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetDeviceInfo request
pub const GET_DEVICE_INFO_REQUEST: u8 = 24;
pub fn get_device_info<Conn, A, B>(conn: &Conn, device_spec: DeviceSpec, wanted: A, all_buttons: bool, first_button: u8, n_buttons: u8, led_class: LedClass, led_id: B) -> Result<Cookie<'_, Conn, GetDeviceInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<IDSpec>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let wanted: u16 = wanted.into();
    let led_id: IDSpec = led_id.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let wanted_bytes = wanted.serialize();
    let all_buttons_bytes = all_buttons.serialize();
    let first_button_bytes = first_button.serialize();
    let n_buttons_bytes = n_buttons.serialize();
    let led_class_bytes = LedClassSpec::from(led_class).serialize();
    let led_id_bytes = led_id.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_DEVICE_INFO_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        wanted_bytes[0],
        wanted_bytes[1],
        all_buttons_bytes[0],
        first_button_bytes[0],
        n_buttons_bytes[0],
        0,
        led_class_bytes[0],
        led_class_bytes[1],
        led_id_bytes[0],
        led_id_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone)]
pub struct GetDeviceInfoReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u16,
    pub supported: u16,
    pub unsupported: u16,
    pub first_btn_wanted: u8,
    pub n_btns_wanted: u8,
    pub first_btn_rtrn: u8,
    pub total_btns: u8,
    pub has_own_state: bool,
    pub dflt_kbd_fb: u16,
    pub dflt_led_fb: u16,
    pub dev_type: xproto::Atom,
    pub name: Vec<String8>,
    pub btn_actions: Vec<Action>,
    pub leds: Vec<DeviceLedInfo>,
}
impl TryParse for GetDeviceInfoReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (present, remaining) = u16::try_parse(remaining)?;
        let (supported, remaining) = u16::try_parse(remaining)?;
        let (unsupported, remaining) = u16::try_parse(remaining)?;
        let (n_device_led_f_bs, remaining) = u16::try_parse(remaining)?;
        let (first_btn_wanted, remaining) = u8::try_parse(remaining)?;
        let (n_btns_wanted, remaining) = u8::try_parse(remaining)?;
        let (first_btn_rtrn, remaining) = u8::try_parse(remaining)?;
        let (n_btns_rtrn, remaining) = u8::try_parse(remaining)?;
        let (total_btns, remaining) = u8::try_parse(remaining)?;
        let (has_own_state, remaining) = bool::try_parse(remaining)?;
        let (dflt_kbd_fb, remaining) = u16::try_parse(remaining)?;
        let (dflt_led_fb, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (dev_type, remaining) = xproto::Atom::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_into().or(Err(ParseError::ParseError))?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (btn_actions, remaining) = crate::x11_utils::parse_list::<Action>(remaining, n_btns_rtrn.try_into().or(Err(ParseError::ParseError))?)?;
        let (leds, remaining) = crate::x11_utils::parse_list::<DeviceLedInfo>(remaining, n_device_led_f_bs.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetDeviceInfoReply { response_type, device_id, sequence, length, present, supported, unsupported, first_btn_wanted, n_btns_wanted, first_btn_rtrn, total_btns, has_own_state, dflt_kbd_fb, dflt_led_fb, dev_type, name, btn_actions, leds };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetDeviceInfoReply {
    /// Get the value of the `nDeviceLedFBs` field.
    ///
    /// The `nDeviceLedFBs` field is used as the length field of the `leds` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_device_led_f_bs(&self) -> u16 {
        self.leds.len()
            .try_into().unwrap()
    }
    /// Get the value of the `nBtnsRtrn` field.
    ///
    /// The `nBtnsRtrn` field is used as the length field of the `btnActions` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n_btns_rtrn(&self) -> u8 {
        self.btn_actions.len()
            .try_into().unwrap()
    }
    /// Get the value of the `nameLen` field.
    ///
    /// The `nameLen` field is used as the length field of the `name` field.
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

/// Opcode for the SetDeviceInfo request
pub const SET_DEVICE_INFO_REQUEST: u8 = 25;
pub fn set_device_info<'c, 'input, Conn, A>(conn: &'c Conn, device_spec: DeviceSpec, first_btn: u8, change: A, btn_actions: &'input [Action], leds: &'input [DeviceLedInfo]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let change: u16 = change.into();
    let length_so_far = 0;
    let device_spec_bytes = device_spec.serialize();
    let first_btn_bytes = first_btn.serialize();
    let n_btns = u8::try_from(btn_actions.len()).expect("`btn_actions` has too many elements");
    let n_btns_bytes = n_btns.serialize();
    let change_bytes = change.serialize();
    let n_device_led_f_bs = u16::try_from(leds.len()).expect("`leds` has too many elements");
    let n_device_led_f_bs_bytes = n_device_led_f_bs.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_DEVICE_INFO_REQUEST,
        0,
        0,
        device_spec_bytes[0],
        device_spec_bytes[1],
        first_btn_bytes[0],
        n_btns_bytes[0],
        change_bytes[0],
        change_bytes[1],
        n_device_led_f_bs_bytes[0],
        n_device_led_f_bs_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    let btn_actions_bytes = btn_actions.serialize();
    let length_so_far = length_so_far + btn_actions_bytes.len();
    let leds_bytes = leds.serialize();
    let length_so_far = length_so_far + leds_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&btn_actions_bytes), IoSlice::new(&leds_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the SetDebuggingFlags request
pub const SET_DEBUGGING_FLAGS_REQUEST: u8 = 101;
pub fn set_debugging_flags<'c, 'input, Conn>(conn: &'c Conn, affect_flags: u32, flags: u32, affect_ctrls: u32, ctrls: u32, message: &'input [String8]) -> Result<Cookie<'c, Conn, SetDebuggingFlagsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let msg_length = u16::try_from(message.len()).expect("`message` has too many elements");
    let msg_length_bytes = msg_length.serialize();
    let affect_flags_bytes = affect_flags.serialize();
    let flags_bytes = flags.serialize();
    let affect_ctrls_bytes = affect_ctrls.serialize();
    let ctrls_bytes = ctrls.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_DEBUGGING_FLAGS_REQUEST,
        0,
        0,
        msg_length_bytes[0],
        msg_length_bytes[1],
        0,
        0,
        affect_flags_bytes[0],
        affect_flags_bytes[1],
        affect_flags_bytes[2],
        affect_flags_bytes[3],
        flags_bytes[0],
        flags_bytes[1],
        flags_bytes[2],
        flags_bytes[3],
        affect_ctrls_bytes[0],
        affect_ctrls_bytes[1],
        affect_ctrls_bytes[2],
        affect_ctrls_bytes[3],
        ctrls_bytes[0],
        ctrls_bytes[1],
        ctrls_bytes[2],
        ctrls_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    let length_so_far = length_so_far + message.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(message), IoSlice::new(&padding0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetDebuggingFlagsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub current_flags: u32,
    pub current_ctrls: u32,
    pub supported_flags: u32,
    pub supported_ctrls: u32,
}
impl TryParse for SetDebuggingFlagsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (current_flags, remaining) = u32::try_parse(remaining)?;
        let (current_ctrls, remaining) = u32::try_parse(remaining)?;
        let (supported_flags, remaining) = u32::try_parse(remaining)?;
        let (supported_ctrls, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let result = SetDebuggingFlagsReply { response_type, sequence, length, current_flags, current_ctrls, supported_flags, supported_ctrls };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetDebuggingFlagsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the NewKeyboardNotify event
pub const NEW_KEYBOARD_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewKeyboardNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub old_device_id: u8,
    pub min_key_code: xproto::Keycode,
    pub max_key_code: xproto::Keycode,
    pub old_min_key_code: xproto::Keycode,
    pub old_max_key_code: xproto::Keycode,
    pub request_major: u8,
    pub request_minor: u8,
    pub changed: u16,
}
impl TryParse for NewKeyboardNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (old_device_id, remaining) = u8::try_parse(remaining)?;
        let (min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (old_min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (old_max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (request_major, remaining) = u8::try_parse(remaining)?;
        let (request_minor, remaining) = u8::try_parse(remaining)?;
        let (changed, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(14..).ok_or(ParseError::ParseError)?;
        let result = NewKeyboardNotifyEvent { response_type, xkb_type, sequence, time, device_id, old_device_id, min_key_code, max_key_code, old_min_key_code, old_max_key_code, request_major, request_minor, changed };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for NewKeyboardNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&NewKeyboardNotifyEvent> for [u8; 32] {
    fn from(input: &NewKeyboardNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let old_device_id_bytes = input.old_device_id.serialize();
        let min_key_code_bytes = input.min_key_code.serialize();
        let max_key_code_bytes = input.max_key_code.serialize();
        let old_min_key_code_bytes = input.old_min_key_code.serialize();
        let old_max_key_code_bytes = input.old_max_key_code.serialize();
        let request_major_bytes = input.request_major.serialize();
        let request_minor_bytes = input.request_minor.serialize();
        let changed_bytes = input.changed.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            old_device_id_bytes[0],
            min_key_code_bytes[0],
            max_key_code_bytes[0],
            old_min_key_code_bytes[0],
            old_max_key_code_bytes[0],
            request_major_bytes[0],
            request_minor_bytes[0],
            changed_bytes[0],
            changed_bytes[1],
            0,
            0,
            0,
            0,
            0,
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
impl From<NewKeyboardNotifyEvent> for [u8; 32] {
    fn from(input: NewKeyboardNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the MapNotify event
pub const MAP_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub ptr_btn_actions: u8,
    pub changed: u16,
    pub min_key_code: xproto::Keycode,
    pub max_key_code: xproto::Keycode,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xproto::Keycode,
    pub n_key_syms: u8,
    pub first_key_act: xproto::Keycode,
    pub n_key_acts: u8,
    pub first_key_behavior: xproto::Keycode,
    pub n_key_behavior: u8,
    pub first_key_explicit: xproto::Keycode,
    pub n_key_explicit: u8,
    pub first_mod_map_key: xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub first_v_mod_map_key: xproto::Keycode,
    pub n_v_mod_map_keys: u8,
    pub virtual_mods: u16,
}
impl TryParse for MapNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (ptr_btn_actions, remaining) = u8::try_parse(remaining)?;
        let (changed, remaining) = u16::try_parse(remaining)?;
        let (min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (first_key_sym, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_syms, remaining) = u8::try_parse(remaining)?;
        let (first_key_act, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_acts, remaining) = u8::try_parse(remaining)?;
        let (first_key_behavior, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_behavior, remaining) = u8::try_parse(remaining)?;
        let (first_key_explicit, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (first_mod_map_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (first_v_mod_map_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_v_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let result = MapNotifyEvent { response_type, xkb_type, sequence, time, device_id, ptr_btn_actions, changed, min_key_code, max_key_code, first_type, n_types, first_key_sym, n_key_syms, first_key_act, n_key_acts, first_key_behavior, n_key_behavior, first_key_explicit, n_key_explicit, first_mod_map_key, n_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, virtual_mods };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&MapNotifyEvent> for [u8; 32] {
    fn from(input: &MapNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let ptr_btn_actions_bytes = input.ptr_btn_actions.serialize();
        let changed_bytes = input.changed.serialize();
        let min_key_code_bytes = input.min_key_code.serialize();
        let max_key_code_bytes = input.max_key_code.serialize();
        let first_type_bytes = input.first_type.serialize();
        let n_types_bytes = input.n_types.serialize();
        let first_key_sym_bytes = input.first_key_sym.serialize();
        let n_key_syms_bytes = input.n_key_syms.serialize();
        let first_key_act_bytes = input.first_key_act.serialize();
        let n_key_acts_bytes = input.n_key_acts.serialize();
        let first_key_behavior_bytes = input.first_key_behavior.serialize();
        let n_key_behavior_bytes = input.n_key_behavior.serialize();
        let first_key_explicit_bytes = input.first_key_explicit.serialize();
        let n_key_explicit_bytes = input.n_key_explicit.serialize();
        let first_mod_map_key_bytes = input.first_mod_map_key.serialize();
        let n_mod_map_keys_bytes = input.n_mod_map_keys.serialize();
        let first_v_mod_map_key_bytes = input.first_v_mod_map_key.serialize();
        let n_v_mod_map_keys_bytes = input.n_v_mod_map_keys.serialize();
        let virtual_mods_bytes = input.virtual_mods.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            ptr_btn_actions_bytes[0],
            changed_bytes[0],
            changed_bytes[1],
            min_key_code_bytes[0],
            max_key_code_bytes[0],
            first_type_bytes[0],
            n_types_bytes[0],
            first_key_sym_bytes[0],
            n_key_syms_bytes[0],
            first_key_act_bytes[0],
            n_key_acts_bytes[0],
            first_key_behavior_bytes[0],
            n_key_behavior_bytes[0],
            first_key_explicit_bytes[0],
            n_key_explicit_bytes[0],
            first_mod_map_key_bytes[0],
            n_mod_map_keys_bytes[0],
            first_v_mod_map_key_bytes[0],
            n_v_mod_map_keys_bytes[0],
            virtual_mods_bytes[0],
            virtual_mods_bytes[1],
            0,
            0,
        ]
    }
}
impl From<MapNotifyEvent> for [u8; 32] {
    fn from(input: MapNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the StateNotify event
pub const STATE_NOTIFY_EVENT: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub mods: u8,
    pub base_mods: u8,
    pub latched_mods: u8,
    pub locked_mods: u8,
    pub group: Group,
    pub base_group: i16,
    pub latched_group: i16,
    pub locked_group: Group,
    pub compat_state: u8,
    pub grab_mods: u8,
    pub compat_grab_mods: u8,
    pub lookup_mods: u8,
    pub compat_loockup_mods: u8,
    pub ptr_btn_state: u16,
    pub changed: u16,
    pub keycode: xproto::Keycode,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
}
impl TryParse for StateNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (mods, remaining) = u8::try_parse(remaining)?;
        let (base_mods, remaining) = u8::try_parse(remaining)?;
        let (latched_mods, remaining) = u8::try_parse(remaining)?;
        let (locked_mods, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = u8::try_parse(remaining)?;
        let (base_group, remaining) = i16::try_parse(remaining)?;
        let (latched_group, remaining) = i16::try_parse(remaining)?;
        let (locked_group, remaining) = u8::try_parse(remaining)?;
        let (compat_state, remaining) = u8::try_parse(remaining)?;
        let (grab_mods, remaining) = u8::try_parse(remaining)?;
        let (compat_grab_mods, remaining) = u8::try_parse(remaining)?;
        let (lookup_mods, remaining) = u8::try_parse(remaining)?;
        let (compat_loockup_mods, remaining) = u8::try_parse(remaining)?;
        let (ptr_btn_state, remaining) = u16::try_parse(remaining)?;
        let (changed, remaining) = u16::try_parse(remaining)?;
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (event_type, remaining) = u8::try_parse(remaining)?;
        let (request_major, remaining) = u8::try_parse(remaining)?;
        let (request_minor, remaining) = u8::try_parse(remaining)?;
        let group = group.try_into()?;
        let locked_group = locked_group.try_into()?;
        let result = StateNotifyEvent { response_type, xkb_type, sequence, time, device_id, mods, base_mods, latched_mods, locked_mods, group, base_group, latched_group, locked_group, compat_state, grab_mods, compat_grab_mods, lookup_mods, compat_loockup_mods, ptr_btn_state, changed, keycode, event_type, request_major, request_minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for StateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&StateNotifyEvent> for [u8; 32] {
    fn from(input: &StateNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let mods_bytes = input.mods.serialize();
        let base_mods_bytes = input.base_mods.serialize();
        let latched_mods_bytes = input.latched_mods.serialize();
        let locked_mods_bytes = input.locked_mods.serialize();
        let group_bytes = u8::from(input.group).serialize();
        let base_group_bytes = input.base_group.serialize();
        let latched_group_bytes = input.latched_group.serialize();
        let locked_group_bytes = u8::from(input.locked_group).serialize();
        let compat_state_bytes = input.compat_state.serialize();
        let grab_mods_bytes = input.grab_mods.serialize();
        let compat_grab_mods_bytes = input.compat_grab_mods.serialize();
        let lookup_mods_bytes = input.lookup_mods.serialize();
        let compat_loockup_mods_bytes = input.compat_loockup_mods.serialize();
        let ptr_btn_state_bytes = input.ptr_btn_state.serialize();
        let changed_bytes = input.changed.serialize();
        let keycode_bytes = input.keycode.serialize();
        let event_type_bytes = input.event_type.serialize();
        let request_major_bytes = input.request_major.serialize();
        let request_minor_bytes = input.request_minor.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            mods_bytes[0],
            base_mods_bytes[0],
            latched_mods_bytes[0],
            locked_mods_bytes[0],
            group_bytes[0],
            base_group_bytes[0],
            base_group_bytes[1],
            latched_group_bytes[0],
            latched_group_bytes[1],
            locked_group_bytes[0],
            compat_state_bytes[0],
            grab_mods_bytes[0],
            compat_grab_mods_bytes[0],
            lookup_mods_bytes[0],
            compat_loockup_mods_bytes[0],
            ptr_btn_state_bytes[0],
            ptr_btn_state_bytes[1],
            changed_bytes[0],
            changed_bytes[1],
            keycode_bytes[0],
            event_type_bytes[0],
            request_major_bytes[0],
            request_minor_bytes[0],
        ]
    }
}
impl From<StateNotifyEvent> for [u8; 32] {
    fn from(input: StateNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ControlsNotify event
pub const CONTROLS_NOTIFY_EVENT: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlsNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub num_groups: u8,
    pub changed_controls: u32,
    pub enabled_controls: u32,
    pub enabled_control_changes: u32,
    pub keycode: xproto::Keycode,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
}
impl TryParse for ControlsNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (num_groups, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (changed_controls, remaining) = u32::try_parse(remaining)?;
        let (enabled_controls, remaining) = u32::try_parse(remaining)?;
        let (enabled_control_changes, remaining) = u32::try_parse(remaining)?;
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (event_type, remaining) = u8::try_parse(remaining)?;
        let (request_major, remaining) = u8::try_parse(remaining)?;
        let (request_minor, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let result = ControlsNotifyEvent { response_type, xkb_type, sequence, time, device_id, num_groups, changed_controls, enabled_controls, enabled_control_changes, keycode, event_type, request_major, request_minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ControlsNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&ControlsNotifyEvent> for [u8; 32] {
    fn from(input: &ControlsNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let num_groups_bytes = input.num_groups.serialize();
        let changed_controls_bytes = input.changed_controls.serialize();
        let enabled_controls_bytes = input.enabled_controls.serialize();
        let enabled_control_changes_bytes = input.enabled_control_changes.serialize();
        let keycode_bytes = input.keycode.serialize();
        let event_type_bytes = input.event_type.serialize();
        let request_major_bytes = input.request_major.serialize();
        let request_minor_bytes = input.request_minor.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            num_groups_bytes[0],
            0,
            0,
            changed_controls_bytes[0],
            changed_controls_bytes[1],
            changed_controls_bytes[2],
            changed_controls_bytes[3],
            enabled_controls_bytes[0],
            enabled_controls_bytes[1],
            enabled_controls_bytes[2],
            enabled_controls_bytes[3],
            enabled_control_changes_bytes[0],
            enabled_control_changes_bytes[1],
            enabled_control_changes_bytes[2],
            enabled_control_changes_bytes[3],
            keycode_bytes[0],
            event_type_bytes[0],
            request_major_bytes[0],
            request_minor_bytes[0],
            0,
            0,
            0,
            0,
        ]
    }
}
impl From<ControlsNotifyEvent> for [u8; 32] {
    fn from(input: ControlsNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the IndicatorStateNotify event
pub const INDICATOR_STATE_NOTIFY_EVENT: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicatorStateNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub state: u32,
    pub state_changed: u32,
}
impl TryParse for IndicatorStateNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        let (state_changed, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let result = IndicatorStateNotifyEvent { response_type, xkb_type, sequence, time, device_id, state, state_changed };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IndicatorStateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&IndicatorStateNotifyEvent> for [u8; 32] {
    fn from(input: &IndicatorStateNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let state_bytes = input.state.serialize();
        let state_changed_bytes = input.state_changed.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            0,
            0,
            0,
            state_bytes[0],
            state_bytes[1],
            state_bytes[2],
            state_bytes[3],
            state_changed_bytes[0],
            state_changed_bytes[1],
            state_changed_bytes[2],
            state_changed_bytes[3],
            0,
            0,
            0,
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
impl From<IndicatorStateNotifyEvent> for [u8; 32] {
    fn from(input: IndicatorStateNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the IndicatorMapNotify event
pub const INDICATOR_MAP_NOTIFY_EVENT: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicatorMapNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub state: u32,
    pub map_changed: u32,
}
impl TryParse for IndicatorMapNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        let (map_changed, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let result = IndicatorMapNotifyEvent { response_type, xkb_type, sequence, time, device_id, state, map_changed };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IndicatorMapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&IndicatorMapNotifyEvent> for [u8; 32] {
    fn from(input: &IndicatorMapNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let state_bytes = input.state.serialize();
        let map_changed_bytes = input.map_changed.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            0,
            0,
            0,
            state_bytes[0],
            state_bytes[1],
            state_bytes[2],
            state_bytes[3],
            map_changed_bytes[0],
            map_changed_bytes[1],
            map_changed_bytes[2],
            map_changed_bytes[3],
            0,
            0,
            0,
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
impl From<IndicatorMapNotifyEvent> for [u8; 32] {
    fn from(input: IndicatorMapNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the NamesNotify event
pub const NAMES_NOTIFY_EVENT: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamesNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub changed: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub first_level_name: u8,
    pub n_level_names: u8,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub changed_group_names: u8,
    pub changed_virtual_mods: u16,
    pub first_key: xproto::Keycode,
    pub n_keys: u8,
    pub changed_indicators: u32,
}
impl TryParse for NamesNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (changed, remaining) = u16::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (first_level_name, remaining) = u8::try_parse(remaining)?;
        let (n_level_names, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (n_radio_groups, remaining) = u8::try_parse(remaining)?;
        let (n_key_aliases, remaining) = u8::try_parse(remaining)?;
        let (changed_group_names, remaining) = u8::try_parse(remaining)?;
        let (changed_virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (first_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_keys, remaining) = u8::try_parse(remaining)?;
        let (changed_indicators, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let result = NamesNotifyEvent { response_type, xkb_type, sequence, time, device_id, changed, first_type, n_types, first_level_name, n_level_names, n_radio_groups, n_key_aliases, changed_group_names, changed_virtual_mods, first_key, n_keys, changed_indicators };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for NamesNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&NamesNotifyEvent> for [u8; 32] {
    fn from(input: &NamesNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let changed_bytes = input.changed.serialize();
        let first_type_bytes = input.first_type.serialize();
        let n_types_bytes = input.n_types.serialize();
        let first_level_name_bytes = input.first_level_name.serialize();
        let n_level_names_bytes = input.n_level_names.serialize();
        let n_radio_groups_bytes = input.n_radio_groups.serialize();
        let n_key_aliases_bytes = input.n_key_aliases.serialize();
        let changed_group_names_bytes = input.changed_group_names.serialize();
        let changed_virtual_mods_bytes = input.changed_virtual_mods.serialize();
        let first_key_bytes = input.first_key.serialize();
        let n_keys_bytes = input.n_keys.serialize();
        let changed_indicators_bytes = input.changed_indicators.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            0,
            changed_bytes[0],
            changed_bytes[1],
            first_type_bytes[0],
            n_types_bytes[0],
            first_level_name_bytes[0],
            n_level_names_bytes[0],
            0,
            n_radio_groups_bytes[0],
            n_key_aliases_bytes[0],
            changed_group_names_bytes[0],
            changed_virtual_mods_bytes[0],
            changed_virtual_mods_bytes[1],
            first_key_bytes[0],
            n_keys_bytes[0],
            changed_indicators_bytes[0],
            changed_indicators_bytes[1],
            changed_indicators_bytes[2],
            changed_indicators_bytes[3],
            0,
            0,
            0,
            0,
        ]
    }
}
impl From<NamesNotifyEvent> for [u8; 32] {
    fn from(input: NamesNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the CompatMapNotify event
pub const COMPAT_MAP_NOTIFY_EVENT: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatMapNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub changed_groups: u8,
    pub first_si: u16,
    pub n_si: u16,
    pub n_total_si: u16,
}
impl TryParse for CompatMapNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (changed_groups, remaining) = u8::try_parse(remaining)?;
        let (first_si, remaining) = u16::try_parse(remaining)?;
        let (n_si, remaining) = u16::try_parse(remaining)?;
        let (n_total_si, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let result = CompatMapNotifyEvent { response_type, xkb_type, sequence, time, device_id, changed_groups, first_si, n_si, n_total_si };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CompatMapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&CompatMapNotifyEvent> for [u8; 32] {
    fn from(input: &CompatMapNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let changed_groups_bytes = input.changed_groups.serialize();
        let first_si_bytes = input.first_si.serialize();
        let n_si_bytes = input.n_si.serialize();
        let n_total_si_bytes = input.n_total_si.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            changed_groups_bytes[0],
            first_si_bytes[0],
            first_si_bytes[1],
            n_si_bytes[0],
            n_si_bytes[1],
            n_total_si_bytes[0],
            n_total_si_bytes[1],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
impl From<CompatMapNotifyEvent> for [u8; 32] {
    fn from(input: CompatMapNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BellNotify event
pub const BELL_NOTIFY_EVENT: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BellNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub bell_class: BellClassResult,
    pub bell_id: u8,
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
    pub name: xproto::Atom,
    pub window: xproto::Window,
    pub event_only: bool,
}
impl TryParse for BellNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (bell_class, remaining) = u8::try_parse(remaining)?;
        let (bell_id, remaining) = u8::try_parse(remaining)?;
        let (percent, remaining) = u8::try_parse(remaining)?;
        let (pitch, remaining) = u16::try_parse(remaining)?;
        let (duration, remaining) = u16::try_parse(remaining)?;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (event_only, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(7..).ok_or(ParseError::ParseError)?;
        let bell_class = bell_class.try_into()?;
        let result = BellNotifyEvent { response_type, xkb_type, sequence, time, device_id, bell_class, bell_id, percent, pitch, duration, name, window, event_only };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BellNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&BellNotifyEvent> for [u8; 32] {
    fn from(input: &BellNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let bell_class_bytes = u8::from(input.bell_class).serialize();
        let bell_id_bytes = input.bell_id.serialize();
        let percent_bytes = input.percent.serialize();
        let pitch_bytes = input.pitch.serialize();
        let duration_bytes = input.duration.serialize();
        let name_bytes = input.name.serialize();
        let window_bytes = input.window.serialize();
        let event_only_bytes = input.event_only.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            bell_class_bytes[0],
            bell_id_bytes[0],
            percent_bytes[0],
            pitch_bytes[0],
            pitch_bytes[1],
            duration_bytes[0],
            duration_bytes[1],
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            event_only_bytes[0],
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
impl From<BellNotifyEvent> for [u8; 32] {
    fn from(input: BellNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ActionMessage event
pub const ACTION_MESSAGE_EVENT: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionMessageEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub keycode: xproto::Keycode,
    pub press: bool,
    pub key_event_follows: bool,
    pub mods: u8,
    pub group: Group,
    pub message: [String8; 8],
}
impl TryParse for ActionMessageEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (press, remaining) = bool::try_parse(remaining)?;
        let (key_event_follows, remaining) = bool::try_parse(remaining)?;
        let (mods, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = u8::try_parse(remaining)?;
        let (message, remaining) = crate::x11_utils::parse_u8_list(remaining, 8)?;
        let message = <[u8; 8]>::try_from(message).unwrap();
        let remaining = remaining.get(10..).ok_or(ParseError::ParseError)?;
        let group = group.try_into()?;
        let result = ActionMessageEvent { response_type, xkb_type, sequence, time, device_id, keycode, press, key_event_follows, mods, group, message };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ActionMessageEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&ActionMessageEvent> for [u8; 32] {
    fn from(input: &ActionMessageEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let keycode_bytes = input.keycode.serialize();
        let press_bytes = input.press.serialize();
        let key_event_follows_bytes = input.key_event_follows.serialize();
        let mods_bytes = input.mods.serialize();
        let group_bytes = u8::from(input.group).serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            keycode_bytes[0],
            press_bytes[0],
            key_event_follows_bytes[0],
            mods_bytes[0],
            group_bytes[0],
            input.message[0],
            input.message[1],
            input.message[2],
            input.message[3],
            input.message[4],
            input.message[5],
            input.message[6],
            input.message[7],
            0,
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
impl From<ActionMessageEvent> for [u8; 32] {
    fn from(input: ActionMessageEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the AccessXNotify event
pub const ACCESS_X_NOTIFY_EVENT: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessXNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub keycode: xproto::Keycode,
    pub detailt: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
}
impl TryParse for AccessXNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (detailt, remaining) = u16::try_parse(remaining)?;
        let (slow_keys_delay, remaining) = u16::try_parse(remaining)?;
        let (debounce_delay, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let result = AccessXNotifyEvent { response_type, xkb_type, sequence, time, device_id, keycode, detailt, slow_keys_delay, debounce_delay };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AccessXNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&AccessXNotifyEvent> for [u8; 32] {
    fn from(input: &AccessXNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let keycode_bytes = input.keycode.serialize();
        let detailt_bytes = input.detailt.serialize();
        let slow_keys_delay_bytes = input.slow_keys_delay.serialize();
        let debounce_delay_bytes = input.debounce_delay.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            keycode_bytes[0],
            detailt_bytes[0],
            detailt_bytes[1],
            slow_keys_delay_bytes[0],
            slow_keys_delay_bytes[1],
            debounce_delay_bytes[0],
            debounce_delay_bytes[1],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
impl From<AccessXNotifyEvent> for [u8; 32] {
    fn from(input: AccessXNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ExtensionDeviceNotify event
pub const EXTENSION_DEVICE_NOTIFY_EVENT: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtensionDeviceNotifyEvent {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub device_id: u8,
    pub reason: u16,
    pub led_class: LedClassResult,
    pub led_id: u16,
    pub leds_defined: u32,
    pub led_state: u32,
    pub first_button: u8,
    pub n_buttons: u8,
    pub supported: u16,
    pub unsupported: u16,
}
impl TryParse for ExtensionDeviceNotifyEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (reason, remaining) = u16::try_parse(remaining)?;
        let (led_class, remaining) = u16::try_parse(remaining)?;
        let (led_id, remaining) = u16::try_parse(remaining)?;
        let (leds_defined, remaining) = u32::try_parse(remaining)?;
        let (led_state, remaining) = u32::try_parse(remaining)?;
        let (first_button, remaining) = u8::try_parse(remaining)?;
        let (n_buttons, remaining) = u8::try_parse(remaining)?;
        let (supported, remaining) = u16::try_parse(remaining)?;
        let (unsupported, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let led_class = led_class.try_into()?;
        let result = ExtensionDeviceNotifyEvent { response_type, xkb_type, sequence, time, device_id, reason, led_class, led_id, leds_defined, led_state, first_button, n_buttons, supported, unsupported };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ExtensionDeviceNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&ExtensionDeviceNotifyEvent> for [u8; 32] {
    fn from(input: &ExtensionDeviceNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let xkb_type_bytes = input.xkb_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let device_id_bytes = input.device_id.serialize();
        let reason_bytes = input.reason.serialize();
        let led_class_bytes = u16::from(input.led_class).serialize();
        let led_id_bytes = input.led_id.serialize();
        let leds_defined_bytes = input.leds_defined.serialize();
        let led_state_bytes = input.led_state.serialize();
        let first_button_bytes = input.first_button.serialize();
        let n_buttons_bytes = input.n_buttons.serialize();
        let supported_bytes = input.supported.serialize();
        let unsupported_bytes = input.unsupported.serialize();
        [
            response_type_bytes[0],
            xkb_type_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            device_id_bytes[0],
            0,
            reason_bytes[0],
            reason_bytes[1],
            led_class_bytes[0],
            led_class_bytes[1],
            led_id_bytes[0],
            led_id_bytes[1],
            leds_defined_bytes[0],
            leds_defined_bytes[1],
            leds_defined_bytes[2],
            leds_defined_bytes[3],
            led_state_bytes[0],
            led_state_bytes[1],
            led_state_bytes[2],
            led_state_bytes[3],
            first_button_bytes[0],
            n_buttons_bytes[0],
            supported_bytes[0],
            supported_bytes[1],
            unsupported_bytes[0],
            unsupported_bytes[1],
            0,
            0,
        ]
    }
}
impl From<ExtensionDeviceNotifyEvent> for [u8; 32] {
    fn from(input: ExtensionDeviceNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xkb_use_extension(&self, wanted_major: u16, wanted_minor: u16) -> Result<Cookie<'_, Self, UseExtensionReply>, ConnectionError>
    {
        use_extension(self, wanted_major, wanted_minor)
    }
    fn xkb_select_events<'c, 'input, A, B, C, D, E>(&'c self, device_spec: DeviceSpec, affect_which: A, clear: B, select_all: C, affect_map: D, map: E, details: &'input SelectEventsAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u16>,
        C: Into<u16>,
        D: Into<u16>,
        E: Into<u16>,
    {
        select_events(self, device_spec, affect_which, clear, select_all, affect_map, map, details)
    }
    fn xkb_bell(&self, device_spec: DeviceSpec, bell_class: BellClassSpec, bell_id: IDSpec, percent: i8, force_sound: bool, event_only: bool, pitch: i16, duration: i16, name: xproto::Atom, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        bell(self, device_spec, bell_class, bell_id, percent, force_sound, event_only, pitch, duration, name, window)
    }
    fn xkb_get_state(&self, device_spec: DeviceSpec) -> Result<Cookie<'_, Self, GetStateReply>, ConnectionError>
    {
        get_state(self, device_spec)
    }
    fn xkb_latch_lock_state<A, B, C>(&self, device_spec: DeviceSpec, affect_mod_locks: A, mod_locks: B, lock_group: bool, group_lock: Group, affect_mod_latches: C, latch_group: bool, group_latch: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u8>,
        B: Into<u8>,
        C: Into<u8>,
    {
        latch_lock_state(self, device_spec, affect_mod_locks, mod_locks, lock_group, group_lock, affect_mod_latches, latch_group, group_latch)
    }
    fn xkb_get_controls(&self, device_spec: DeviceSpec) -> Result<Cookie<'_, Self, GetControlsReply>, ConnectionError>
    {
        get_controls(self, device_spec)
    }
    fn xkb_set_controls<'c, 'input, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P>(&'c self, device_spec: DeviceSpec, affect_internal_real_mods: A, internal_real_mods: B, affect_ignore_lock_real_mods: C, ignore_lock_real_mods: D, affect_internal_virtual_mods: E, internal_virtual_mods: F, affect_ignore_lock_virtual_mods: G, ignore_lock_virtual_mods: H, mouse_keys_dflt_btn: u8, groups_wrap: u8, access_x_options: I, affect_enabled_controls: J, enabled_controls: K, change_controls: L, repeat_delay: u16, repeat_interval: u16, slow_keys_delay: u16, debounce_delay: u16, mouse_keys_delay: u16, mouse_keys_interval: u16, mouse_keys_time_to_max: u16, mouse_keys_max_speed: u16, mouse_keys_curve: i16, access_x_timeout: u16, access_x_timeout_mask: M, access_x_timeout_values: N, access_x_timeout_options_mask: O, access_x_timeout_options_values: P, per_key_repeat: &'input [u8; 32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
        B: Into<u8>,
        C: Into<u8>,
        D: Into<u8>,
        E: Into<u16>,
        F: Into<u16>,
        G: Into<u16>,
        H: Into<u16>,
        I: Into<u16>,
        J: Into<u32>,
        K: Into<u32>,
        L: Into<u32>,
        M: Into<u32>,
        N: Into<u32>,
        O: Into<u16>,
        P: Into<u16>,
    {
        set_controls(self, device_spec, affect_internal_real_mods, internal_real_mods, affect_ignore_lock_real_mods, ignore_lock_real_mods, affect_internal_virtual_mods, internal_virtual_mods, affect_ignore_lock_virtual_mods, ignore_lock_virtual_mods, mouse_keys_dflt_btn, groups_wrap, access_x_options, affect_enabled_controls, enabled_controls, change_controls, repeat_delay, repeat_interval, slow_keys_delay, debounce_delay, mouse_keys_delay, mouse_keys_interval, mouse_keys_time_to_max, mouse_keys_max_speed, mouse_keys_curve, access_x_timeout, access_x_timeout_mask, access_x_timeout_values, access_x_timeout_options_mask, access_x_timeout_options_values, per_key_repeat)
    }
    fn xkb_get_map<A, B, C>(&self, device_spec: DeviceSpec, full: A, partial: B, first_type: u8, n_types: u8, first_key_sym: xproto::Keycode, n_key_syms: u8, first_key_action: xproto::Keycode, n_key_actions: u8, first_key_behavior: xproto::Keycode, n_key_behaviors: u8, virtual_mods: C, first_key_explicit: xproto::Keycode, n_key_explicit: u8, first_mod_map_key: xproto::Keycode, n_mod_map_keys: u8, first_v_mod_map_key: xproto::Keycode, n_v_mod_map_keys: u8) -> Result<Cookie<'_, Self, GetMapReply>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u16>,
        C: Into<u16>,
    {
        get_map(self, device_spec, full, partial, first_type, n_types, first_key_sym, n_key_syms, first_key_action, n_key_actions, first_key_behavior, n_key_behaviors, virtual_mods, first_key_explicit, n_key_explicit, first_mod_map_key, n_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys)
    }
    fn xkb_set_map<'c, 'input, A, B>(&'c self, device_spec: DeviceSpec, flags: A, min_key_code: xproto::Keycode, max_key_code: xproto::Keycode, first_type: u8, n_types: u8, first_key_sym: xproto::Keycode, n_key_syms: u8, total_syms: u16, first_key_action: xproto::Keycode, n_key_actions: u8, total_actions: u16, first_key_behavior: xproto::Keycode, n_key_behaviors: u8, total_key_behaviors: u8, first_key_explicit: xproto::Keycode, n_key_explicit: u8, total_key_explicit: u8, first_mod_map_key: xproto::Keycode, n_mod_map_keys: u8, total_mod_map_keys: u8, first_v_mod_map_key: xproto::Keycode, n_v_mod_map_keys: u8, total_v_mod_map_keys: u8, virtual_mods: B, values: &'input SetMapAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u16>,
    {
        set_map(self, device_spec, flags, min_key_code, max_key_code, first_type, n_types, first_key_sym, n_key_syms, total_syms, first_key_action, n_key_actions, total_actions, first_key_behavior, n_key_behaviors, total_key_behaviors, first_key_explicit, n_key_explicit, total_key_explicit, first_mod_map_key, n_mod_map_keys, total_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, total_v_mod_map_keys, virtual_mods, values)
    }
    fn xkb_get_compat_map<A>(&self, device_spec: DeviceSpec, groups: A, get_all_si: bool, first_si: u16, n_si: u16) -> Result<Cookie<'_, Self, GetCompatMapReply>, ConnectionError>
    where
        A: Into<u8>,
    {
        get_compat_map(self, device_spec, groups, get_all_si, first_si, n_si)
    }
    fn xkb_set_compat_map<'c, 'input, A>(&'c self, device_spec: DeviceSpec, recompute_actions: bool, truncate_si: bool, groups: A, first_si: u16, si: &'input [SymInterpret], group_maps: &'input [ModDef]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        set_compat_map(self, device_spec, recompute_actions, truncate_si, groups, first_si, si, group_maps)
    }
    fn xkb_get_indicator_state(&self, device_spec: DeviceSpec) -> Result<Cookie<'_, Self, GetIndicatorStateReply>, ConnectionError>
    {
        get_indicator_state(self, device_spec)
    }
    fn xkb_get_indicator_map(&self, device_spec: DeviceSpec, which: u32) -> Result<Cookie<'_, Self, GetIndicatorMapReply>, ConnectionError>
    {
        get_indicator_map(self, device_spec, which)
    }
    fn xkb_set_indicator_map<'c, 'input>(&'c self, device_spec: DeviceSpec, which: u32, maps: &'input [IndicatorMap]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_indicator_map(self, device_spec, which, maps)
    }
    fn xkb_get_named_indicator<A>(&self, device_spec: DeviceSpec, led_class: LedClass, led_id: A, indicator: xproto::Atom) -> Result<Cookie<'_, Self, GetNamedIndicatorReply>, ConnectionError>
    where
        A: Into<IDSpec>,
    {
        get_named_indicator(self, device_spec, led_class, led_id, indicator)
    }
    fn xkb_set_named_indicator<A, B, C, D, E, F, G, H>(&self, device_spec: DeviceSpec, led_class: LedClass, led_id: A, indicator: xproto::Atom, set_state: bool, on: bool, set_map: bool, create_map: bool, map_flags: B, map_which_groups: C, map_groups: D, map_which_mods: E, map_real_mods: F, map_vmods: G, map_ctrls: H) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<IDSpec>,
        B: Into<u8>,
        C: Into<u8>,
        D: Into<u8>,
        E: Into<u8>,
        F: Into<u8>,
        G: Into<u16>,
        H: Into<u32>,
    {
        set_named_indicator(self, device_spec, led_class, led_id, indicator, set_state, on, set_map, create_map, map_flags, map_which_groups, map_groups, map_which_mods, map_real_mods, map_vmods, map_ctrls)
    }
    fn xkb_get_names<A>(&self, device_spec: DeviceSpec, which: A) -> Result<Cookie<'_, Self, GetNamesReply>, ConnectionError>
    where
        A: Into<u32>,
    {
        get_names(self, device_spec, which)
    }
    fn xkb_set_names<'c, 'input, A, B>(&'c self, device_spec: DeviceSpec, virtual_mods: A, first_type: u8, n_types: u8, first_kt_levelt: u8, n_kt_levels: u8, indicators: u32, group_names: B, n_radio_groups: u8, first_key: xproto::Keycode, n_keys: u8, n_key_aliases: u8, total_kt_level_names: u16, values: &'input SetNamesAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u8>,
    {
        set_names(self, device_spec, virtual_mods, first_type, n_types, first_kt_levelt, n_kt_levels, indicators, group_names, n_radio_groups, first_key, n_keys, n_key_aliases, total_kt_level_names, values)
    }
    fn xkb_per_client_flags<A, B, C, D, E>(&self, device_spec: DeviceSpec, change: A, value: B, ctrls_to_change: C, auto_ctrls: D, auto_ctrls_values: E) -> Result<Cookie<'_, Self, PerClientFlagsReply>, ConnectionError>
    where
        A: Into<u32>,
        B: Into<u32>,
        C: Into<u32>,
        D: Into<u32>,
        E: Into<u32>,
    {
        per_client_flags(self, device_spec, change, value, ctrls_to_change, auto_ctrls, auto_ctrls_values)
    }
    fn xkb_list_components(&self, device_spec: DeviceSpec, max_names: u16) -> Result<Cookie<'_, Self, ListComponentsReply>, ConnectionError>
    {
        list_components(self, device_spec, max_names)
    }
    fn xkb_get_kbd_by_name<A, B>(&self, device_spec: DeviceSpec, need: A, want: B, load: bool) -> Result<Cookie<'_, Self, GetKbdByNameReply>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u16>,
    {
        get_kbd_by_name(self, device_spec, need, want, load)
    }
    fn xkb_get_device_info<A, B>(&self, device_spec: DeviceSpec, wanted: A, all_buttons: bool, first_button: u8, n_buttons: u8, led_class: LedClass, led_id: B) -> Result<Cookie<'_, Self, GetDeviceInfoReply>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<IDSpec>,
    {
        get_device_info(self, device_spec, wanted, all_buttons, first_button, n_buttons, led_class, led_id)
    }
    fn xkb_set_device_info<'c, 'input, A>(&'c self, device_spec: DeviceSpec, first_btn: u8, change: A, btn_actions: &'input [Action], leds: &'input [DeviceLedInfo]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u16>,
    {
        set_device_info(self, device_spec, first_btn, change, btn_actions, leds)
    }
    fn xkb_set_debugging_flags<'c, 'input>(&'c self, affect_flags: u32, flags: u32, affect_ctrls: u32, ctrls: u32, message: &'input [String8]) -> Result<Cookie<'c, Self, SetDebuggingFlagsReply>, ConnectionError>
    {
        set_debugging_flags(self, affect_flags, flags, affect_ctrls, ctrls, message)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
