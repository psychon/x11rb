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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            134217728 => Ok(Control::GroupsWrap),
            268435456 => Ok(Control::InternalMods),
            536870912 => Ok(Control::IgnoreLockMods),
            1073741824 => Ok(Control::PerKeyRepeat),
            2147483648 => Ok(Control::ControlsEnabled),
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            SymInterpMatch::OpMask => 127,
            SymInterpMatch::LevelOneOnly => 1 << 7,
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let flags_bytes = Into::<u8>::into(self.flags).serialize();
        let which_groups_bytes = Into::<u8>::into(self.which_groups).serialize();
        let groups_bytes = Into::<u8>::into(self.groups).serialize();
        let which_mods_bytes = Into::<u8>::into(self.which_mods).serialize();
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
        Into::<u8>::into(self.flags).serialize_into(bytes);
        Into::<u8>::into(self.which_groups).serialize_into(bytes);
        Into::<u8>::into(self.groups).serialize_into(bytes);
        Into::<u8>::into(self.which_mods).serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
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
        let (name_0, remaining) = u8::try_parse(remaining)?;
        let (name_1, remaining) = u8::try_parse(remaining)?;
        let (name_2, remaining) = u8::try_parse(remaining)?;
        let (name_3, remaining) = u8::try_parse(remaining)?;
        let name = [
            name_0,
            name_1,
            name_2,
            name_3,
        ];
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
    fn serialize(&self) -> Self::Bytes {
        [
            self.name[0],
            self.name[1],
            self.name[2],
            self.name[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.name.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyAlias {
    pub real: [u8; 4],
    pub alias: [u8; 4],
}
impl TryParse for KeyAlias {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (real_0, remaining) = u8::try_parse(remaining)?;
        let (real_1, remaining) = u8::try_parse(remaining)?;
        let (real_2, remaining) = u8::try_parse(remaining)?;
        let (real_3, remaining) = u8::try_parse(remaining)?;
        let real = [
            real_0,
            real_1,
            real_2,
            real_3,
        ];
        let (alias_0, remaining) = u8::try_parse(remaining)?;
        let (alias_1, remaining) = u8::try_parse(remaining)?;
        let (alias_2, remaining) = u8::try_parse(remaining)?;
        let (alias_3, remaining) = u8::try_parse(remaining)?;
        let alias = [
            alias_0,
            alias_1,
            alias_2,
            alias_3,
        ];
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
    fn serialize(&self) -> Self::Bytes {
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
        self.real.serialize_into(bytes);
        self.alias.serialize_into(bytes);
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
        let (string, remaining) = crate::x11_utils::parse_list::<u8>(remaining, length as usize)?;
        let (alignment_pad, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (((length as usize) + (5)) & (!(3))) - ((length as usize) + (2)))?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        let length = self.string.len() as u16;
        length.serialize_into(bytes);
        self.string.serialize_into(bytes);
        self.alignment_pad.serialize_into(bytes);
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
    fn serialize(&self) -> Self::Bytes {
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
        let (map, remaining) = crate::x11_utils::parse_list::<KTMapEntry>(remaining, n_map_entries as usize)?;
        let (preserve, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, (has_preserve as usize) * (n_map_entries as usize))?;
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
    fn serialize(&self) -> Self::Bytes {
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
        let n_map_entries = self.map.len() as u8;
        n_map_entries.serialize_into(bytes);
        self.has_preserve.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.map.serialize_into(bytes);
        self.preserve.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeySymMap {
    pub kt_index: [u8; 4],
    pub group_info: u8,
    pub width: u8,
    pub syms: Vec<Keysym>,
}
impl TryParse for KeySymMap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (kt_index_0, remaining) = u8::try_parse(remaining)?;
        let (kt_index_1, remaining) = u8::try_parse(remaining)?;
        let (kt_index_2, remaining) = u8::try_parse(remaining)?;
        let (kt_index_3, remaining) = u8::try_parse(remaining)?;
        let kt_index = [
            kt_index_0,
            kt_index_1,
            kt_index_2,
            kt_index_3,
        ];
        let (group_info, remaining) = u8::try_parse(remaining)?;
        let (width, remaining) = u8::try_parse(remaining)?;
        let (n_syms, remaining) = u16::try_parse(remaining)?;
        let (syms, remaining) = crate::x11_utils::parse_list::<Keysym>(remaining, n_syms as usize)?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.kt_index.serialize_into(bytes);
        self.group_info.serialize_into(bytes);
        self.width.serialize_into(bytes);
        let n_syms = self.syms.len() as u16;
        n_syms.serialize_into(bytes);
        self.syms.serialize_into(bytes);
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockBehavior {
    pub type_: u8,
}
impl TryParse for LockBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = LockBehavior { type_ };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for LockBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for LockBehavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    pub key: Keycode,
}
impl TryParse for OverlayBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (key, remaining) = Keycode::try_parse(remaining)?;
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
    fn serialize(&self) -> Self::Bytes {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PermamentLockBehavior {
    pub type_: u8,
}
impl TryParse for PermamentLockBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = PermamentLockBehavior { type_ };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PermamentLockBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for PermamentLockBehavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> Self::Bytes {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PermamentRadioGroupBehavior {
    pub type_: u8,
    pub group: u8,
}
impl TryParse for PermamentRadioGroupBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = u8::try_parse(remaining)?;
        let result = PermamentRadioGroupBehavior { type_, group };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PermamentRadioGroupBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for PermamentRadioGroupBehavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> Self::Bytes {
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
pub struct PermamentOverlayBehavior {
    pub type_: u8,
    pub key: Keycode,
}
impl TryParse for PermamentOverlayBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (key, remaining) = Keycode::try_parse(remaining)?;
        let result = PermamentOverlayBehavior { type_, key };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PermamentOverlayBehavior {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for PermamentOverlayBehavior {
    type Bytes = [u8; 2];
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn from(value: CommonBehavior) -> Self {
        Self(value.serialize())
    }
}
impl From<DefaultBehavior> for Behavior {
    fn from(value: DefaultBehavior) -> Self {
        Self(value.serialize())
    }
}
impl From<RadioGroupBehavior> for Behavior {
    fn from(value: RadioGroupBehavior) -> Self {
        Self(value.serialize())
    }
}
impl From<OverlayBehavior> for Behavior {
    fn from(value: OverlayBehavior) -> Self {
        Self(value.serialize())
    }
}
impl From<u8> for Behavior {
    fn from(value: u8) -> Self {
        let field_bytes = value.serialize();
        // Pad with zeros
        let mut union_bytes = [0; 2];
        union_bytes[..field_bytes.len()].copy_from_slice(&field_bytes);
        Self(union_bytes)
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
            _ => Err(ParseError::ParseError)
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
    pub keycode: Keycode,
    pub behavior: Behavior,
}
impl TryParse for SetBehavior {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keycode, remaining) = Keycode::try_parse(remaining)?;
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
    fn serialize(&self) -> Self::Bytes {
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
    pub keycode: Keycode,
    pub explicit: u8,
}
impl TryParse for SetExplicit {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keycode, remaining) = Keycode::try_parse(remaining)?;
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
    fn serialize(&self) -> Self::Bytes {
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
    pub keycode: Keycode,
    pub mods: u8,
}
impl TryParse for KeyModMap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keycode, remaining) = Keycode::try_parse(remaining)?;
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
    fn serialize(&self) -> Self::Bytes {
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
    pub keycode: Keycode,
    pub vmods: u16,
}
impl TryParse for KeyVModMap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (keycode, remaining) = Keycode::try_parse(remaining)?;
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
        let (entries, remaining) = crate::x11_utils::parse_list::<KTSetMapEntry>(remaining, n_map_entries as usize)?;
        let (preserve_entries, remaining) = crate::x11_utils::parse_list::<KTSetMapEntry>(remaining, (preserve as usize) * (n_map_entries as usize))?;
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
    fn serialize(&self) -> Self::Bytes {
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
        let n_map_entries = self.entries.len() as u8;
        n_map_entries.serialize_into(bytes);
        self.preserve.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.entries.serialize_into(bytes);
        self.preserve_entries.serialize_into(bytes);
    }
}

pub type String8 = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outline {
    pub corner_radius: u8,
    pub points: Vec<Point>,
}
impl TryParse for Outline {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (n_points, remaining) = u8::try_parse(remaining)?;
        let (corner_radius, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (points, remaining) = crate::x11_utils::parse_list::<Point>(remaining, n_points as usize)?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        let n_points = self.points.len() as u8;
        n_points.serialize_into(bytes);
        self.corner_radius.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.points.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub name: Atom,
    pub primary_ndx: u8,
    pub approx_ndx: u8,
    pub outlines: Vec<Outline>,
}
impl TryParse for Shape {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name, remaining) = Atom::try_parse(remaining)?;
        let (n_outlines, remaining) = u8::try_parse(remaining)?;
        let (primary_ndx, remaining) = u8::try_parse(remaining)?;
        let (approx_ndx, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (outlines, remaining) = crate::x11_utils::parse_list::<Outline>(remaining, n_outlines as usize)?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.name.serialize_into(bytes);
        let n_outlines = self.outlines.len() as u8;
        n_outlines.serialize_into(bytes);
        self.primary_ndx.serialize_into(bytes);
        self.approx_ndx.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.outlines.serialize_into(bytes);
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
        let (name_0, remaining) = String8::try_parse(remaining)?;
        let (name_1, remaining) = String8::try_parse(remaining)?;
        let (name_2, remaining) = String8::try_parse(remaining)?;
        let (name_3, remaining) = String8::try_parse(remaining)?;
        let name = [
            name_0,
            name_1,
            name_2,
            name_3,
        ];
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
    fn serialize(&self) -> Self::Bytes {
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
        self.name.serialize_into(bytes);
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
        let (over_0, remaining) = String8::try_parse(remaining)?;
        let (over_1, remaining) = String8::try_parse(remaining)?;
        let (over_2, remaining) = String8::try_parse(remaining)?;
        let (over_3, remaining) = String8::try_parse(remaining)?;
        let over = [
            over_0,
            over_1,
            over_2,
            over_3,
        ];
        let (under_0, remaining) = String8::try_parse(remaining)?;
        let (under_1, remaining) = String8::try_parse(remaining)?;
        let (under_2, remaining) = String8::try_parse(remaining)?;
        let (under_3, remaining) = String8::try_parse(remaining)?;
        let under = [
            under_0,
            under_1,
            under_2,
            under_3,
        ];
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
    fn serialize(&self) -> Self::Bytes {
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
        self.over.serialize_into(bytes);
        self.under.serialize_into(bytes);
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
        let (keys, remaining) = crate::x11_utils::parse_list::<OverlayKey>(remaining, n_keys as usize)?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.row_under.serialize_into(bytes);
        let n_keys = self.keys.len() as u8;
        n_keys.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.keys.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overlay {
    pub name: Atom,
    pub rows: Vec<OverlayRow>,
}
impl TryParse for Overlay {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name, remaining) = Atom::try_parse(remaining)?;
        let (n_rows, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (rows, remaining) = crate::x11_utils::parse_list::<OverlayRow>(remaining, n_rows as usize)?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.name.serialize_into(bytes);
        let n_rows = self.rows.len() as u8;
        n_rows.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.rows.serialize_into(bytes);
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
        let (keys, remaining) = crate::x11_utils::parse_list::<Key>(remaining, n_keys as usize)?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.top.serialize_into(bytes);
        self.left.serialize_into(bytes);
        let n_keys = self.keys.len() as u8;
        n_keys.serialize_into(bytes);
        self.vertical.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.keys.serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
        let (string, remaining) = crate::x11_utils::parse_list::<String8>(remaining, length as usize)?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.flags.serialize_into(bytes);
        let length = self.string.len() as u16;
        length.serialize_into(bytes);
        self.string.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1][..(2 - (bytes.len() % 2)) % 2]);
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
    pub names: Vec<Atom>,
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
        let (names, remaining) = crate::x11_utils::parse_list::<Atom>(remaining, TryInto::<usize>::try_into(names_present.count_ones()).unwrap())?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, TryInto::<usize>::try_into(maps_present.count_ones()).unwrap())?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(20);
        Into::<LedClassSpec>::into(self.led_class).serialize_into(bytes);
        self.led_id.serialize_into(bytes);
        self.names_present.serialize_into(bytes);
        self.maps_present.serialize_into(bytes);
        self.phys_indicators.serialize_into(bytes);
        self.state.serialize_into(bytes);
        self.names.serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
impl KeyboardError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl<B: AsRef<[u8]>> From<GenericError<B>> for KeyboardError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for KeyboardError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&KeyboardError> for [u8; 32] {
    fn from(input: &KeyboardError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let value = input.value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], value[0], value[1], value[2], value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.mask.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.vmods_high.serialize_into(bytes);
        self.vmods_low.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SALatchMods {
    pub type_: SAType,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}
impl TryParse for SALatchMods {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (mask, remaining) = u8::try_parse(remaining)?;
        let (real_mods, remaining) = u8::try_parse(remaining)?;
        let (vmods_high, remaining) = u8::try_parse(remaining)?;
        let (vmods_low, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SALatchMods { type_, flags, mask, real_mods, vmods_high, vmods_low };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SALatchMods {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SALatchMods {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.mask.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.vmods_high.serialize_into(bytes);
        self.vmods_low.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SALockMods {
    pub type_: SAType,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}
impl TryParse for SALockMods {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (mask, remaining) = u8::try_parse(remaining)?;
        let (real_mods, remaining) = u8::try_parse(remaining)?;
        let (vmods_high, remaining) = u8::try_parse(remaining)?;
        let (vmods_low, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SALockMods { type_, flags, mask, real_mods, vmods_high, vmods_low };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SALockMods {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SALockMods {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.mask.serialize_into(bytes);
        self.real_mods.serialize_into(bytes);
        self.vmods_high.serialize_into(bytes);
        self.vmods_low.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.group.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 5]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SALatchGroup {
    pub type_: SAType,
    pub flags: u8,
    pub group: i8,
}
impl TryParse for SALatchGroup {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = i8::try_parse(remaining)?;
        let remaining = remaining.get(5..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SALatchGroup { type_, flags, group };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SALatchGroup {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SALatchGroup {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.group.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 5]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SALockGroup {
    pub type_: SAType,
    pub flags: u8,
    pub group: i8,
}
impl TryParse for SALockGroup {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = i8::try_parse(remaining)?;
        let remaining = remaining.get(5..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SALockGroup { type_, flags, group };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SALockGroup {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SALockGroup {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.group.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 5]);
    }
}

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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.bool_ctrls_high.serialize_into(bytes);
        self.bool_ctrls_low.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SALockControls {
    pub type_: SAType,
    pub bool_ctrls_high: u8,
    pub bool_ctrls_low: u8,
}
impl TryParse for SALockControls {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (bool_ctrls_high, remaining) = u8::try_parse(remaining)?;
        let (bool_ctrls_low, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let type_ = type_.try_into()?;
        let result = SALockControls { type_, bool_ctrls_high, bool_ctrls_low };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SALockControls {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SALockControls {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.bool_ctrls_high.serialize_into(bytes);
        self.bool_ctrls_low.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

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
            _ => Err(ParseError::ParseError)
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
        let (message_0, remaining) = u8::try_parse(remaining)?;
        let (message_1, remaining) = u8::try_parse(remaining)?;
        let (message_2, remaining) = u8::try_parse(remaining)?;
        let (message_3, remaining) = u8::try_parse(remaining)?;
        let (message_4, remaining) = u8::try_parse(remaining)?;
        let (message_5, remaining) = u8::try_parse(remaining)?;
        let message = [
            message_0,
            message_1,
            message_2,
            message_3,
            message_4,
            message_5,
        ];
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.flags.serialize_into(bytes);
        self.message.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SARedirectKey {
    pub type_: SAType,
    pub newkey: Keycode,
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
        let (newkey, remaining) = Keycode::try_parse(remaining)?;
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
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
            _ => Err(ParseError::ParseError)
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
        let device_bytes = self.device.serialize();
        let val1what_bytes = Into::<u8>::into(self.val1what).serialize();
        let val1index_bytes = self.val1index.serialize();
        let val1value_bytes = self.val1value.serialize();
        let val2what_bytes = Into::<u8>::into(self.val2what).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.device.serialize_into(bytes);
        Into::<u8>::into(self.val1what).serialize_into(bytes);
        self.val1index.serialize_into(bytes);
        self.val1value.serialize_into(bytes);
        Into::<u8>::into(self.val2what).serialize_into(bytes);
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
        let (data_0, remaining) = u8::try_parse(remaining)?;
        let (data_1, remaining) = u8::try_parse(remaining)?;
        let (data_2, remaining) = u8::try_parse(remaining)?;
        let (data_3, remaining) = u8::try_parse(remaining)?;
        let (data_4, remaining) = u8::try_parse(remaining)?;
        let (data_5, remaining) = u8::try_parse(remaining)?;
        let (data_6, remaining) = u8::try_parse(remaining)?;
        let data = [
            data_0,
            data_1,
            data_2,
            data_3,
            data_4,
            data_5,
            data_6,
        ];
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
    fn serialize(&self) -> Self::Bytes {
        let type_bytes = Into::<u8>::into(self.type_).serialize();
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
        Into::<u8>::into(self.type_).serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymInterpret {
    pub sym: Keysym,
    pub mods: u8,
    pub match_: u8,
    pub virtual_mod: u8,
    pub flags: u8,
    pub action: SIAction,
}
impl TryParse for SymInterpret {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (sym, remaining) = Keysym::try_parse(remaining)?;
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
    fn serialize(&self) -> Self::Bytes {
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
    pub fn as_type(&self) -> u8 {
        fn do_the_parse(remaining: &[u8]) -> Result<u8, ParseError> {
            let (type_, remaining) = u8::try_parse(remaining)?;
            let _ = remaining;
            Ok(type_)
        }
        do_the_parse(&self.0).unwrap()
    }
}
impl Serialize for Action {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
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
    fn from(value: SANoAction) -> Self {
        Self(value.serialize())
    }
}
impl From<SASetMods> for Action {
    fn from(value: SASetMods) -> Self {
        Self(value.serialize())
    }
}
impl From<SASetGroup> for Action {
    fn from(value: SASetGroup) -> Self {
        Self(value.serialize())
    }
}
impl From<SAMovePtr> for Action {
    fn from(value: SAMovePtr) -> Self {
        Self(value.serialize())
    }
}
impl From<SAPtrBtn> for Action {
    fn from(value: SAPtrBtn) -> Self {
        Self(value.serialize())
    }
}
impl From<SALockPtrBtn> for Action {
    fn from(value: SALockPtrBtn) -> Self {
        Self(value.serialize())
    }
}
impl From<SASetPtrDflt> for Action {
    fn from(value: SASetPtrDflt) -> Self {
        Self(value.serialize())
    }
}
impl From<SAIsoLock> for Action {
    fn from(value: SAIsoLock) -> Self {
        Self(value.serialize())
    }
}
impl From<SATerminate> for Action {
    fn from(value: SATerminate) -> Self {
        Self(value.serialize())
    }
}
impl From<SASwitchScreen> for Action {
    fn from(value: SASwitchScreen) -> Self {
        Self(value.serialize())
    }
}
impl From<SASetControls> for Action {
    fn from(value: SASetControls) -> Self {
        Self(value.serialize())
    }
}
impl From<SAActionMessage> for Action {
    fn from(value: SAActionMessage) -> Self {
        Self(value.serialize())
    }
}
impl From<SARedirectKey> for Action {
    fn from(value: SARedirectKey) -> Self {
        Self(value.serialize())
    }
}
impl From<SADeviceBtn> for Action {
    fn from(value: SADeviceBtn) -> Self {
        Self(value.serialize())
    }
}
impl From<SALockDeviceBtn> for Action {
    fn from(value: SALockDeviceBtn) -> Self {
        Self(value.serialize())
    }
}
impl From<SADeviceValuator> for Action {
    fn from(value: SADeviceValuator) -> Self {
        Self(value.serialize())
    }
}
impl From<u8> for Action {
    fn from(value: u8) -> Self {
        let field_bytes = value.serialize();
        // Pad with zeros
        let mut union_bytes = [0; 8];
        union_bytes[..field_bytes.len()].copy_from_slice(&field_bytes);
        Self(union_bytes)
    }
}

/// Opcode for the UseExtension request
pub const USE_EXTENSION_REQUEST: u8 = 0;
pub fn use_extension<Conn>(conn: &Conn, wanted_major: u16, wanted_minor: u16) -> Result<Cookie<'_, Conn, UseExtensionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let wanted_major_bytes = wanted_major.serialize();
    let wanted_minor_bytes = wanted_minor.serialize();
    let request0 = [
        extension_information.major_opcode,
        USE_EXTENSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        wanted_major_bytes[0],
        wanted_major_bytes[1],
        wanted_minor_bytes[0],
        wanted_minor_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl UseExtensionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
/// Auxiliary and optional information for the select_events function.
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
impl SelectEventsAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u16 {
        let mut mask = 0;
        if self.bitcase1.is_some() {
            mask |= Into::<u16>::into(EventType::NewKeyboardNotify);
        }
        if self.bitcase2.is_some() {
            mask |= Into::<u16>::into(EventType::StateNotify);
        }
        if self.bitcase3.is_some() {
            mask |= Into::<u16>::into(EventType::ControlsNotify);
        }
        if self.bitcase4.is_some() {
            mask |= Into::<u16>::into(EventType::IndicatorStateNotify);
        }
        if self.bitcase5.is_some() {
            mask |= Into::<u16>::into(EventType::IndicatorMapNotify);
        }
        if self.bitcase6.is_some() {
            mask |= Into::<u16>::into(EventType::NamesNotify);
        }
        if self.bitcase7.is_some() {
            mask |= Into::<u16>::into(EventType::CompatMapNotify);
        }
        if self.bitcase8.is_some() {
            mask |= Into::<u16>::into(EventType::BellNotify);
        }
        if self.bitcase9.is_some() {
            mask |= Into::<u16>::into(EventType::ActionMessage);
        }
        if self.bitcase10.is_some() {
            mask |= Into::<u16>::into(EventType::AccessXNotify);
        }
        if self.bitcase11.is_some() {
            mask |= Into::<u16>::into(EventType::ExtensionDeviceNotify);
        }
        mask
    }
    /// Set the bitcase1 field of this structure.
    pub fn bitcase1<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase1>> {
        self.bitcase1 = value.into();
        self
    }
    /// Set the bitcase2 field of this structure.
    pub fn bitcase2<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase2>> {
        self.bitcase2 = value.into();
        self
    }
    /// Set the bitcase3 field of this structure.
    pub fn bitcase3<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase3>> {
        self.bitcase3 = value.into();
        self
    }
    /// Set the bitcase4 field of this structure.
    pub fn bitcase4<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase4>> {
        self.bitcase4 = value.into();
        self
    }
    /// Set the bitcase5 field of this structure.
    pub fn bitcase5<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase5>> {
        self.bitcase5 = value.into();
        self
    }
    /// Set the bitcase6 field of this structure.
    pub fn bitcase6<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase6>> {
        self.bitcase6 = value.into();
        self
    }
    /// Set the bitcase7 field of this structure.
    pub fn bitcase7<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase7>> {
        self.bitcase7 = value.into();
        self
    }
    /// Set the bitcase8 field of this structure.
    pub fn bitcase8<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase8>> {
        self.bitcase8 = value.into();
        self
    }
    /// Set the bitcase9 field of this structure.
    pub fn bitcase9<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase9>> {
        self.bitcase9 = value.into();
        self
    }
    /// Set the bitcase10 field of this structure.
    pub fn bitcase10<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase10>> {
        self.bitcase10 = value.into();
        self
    }
    /// Set the bitcase11 field of this structure.
    pub fn bitcase11<I>(mut self, value: I) -> Self where I: Into<Option<SelectEventsAuxBitcase11>> {
        self.bitcase11 = value.into();
        self
    }
}
impl Serialize for SelectEventsAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.bitcase1 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase2 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase3 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase4 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase5 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase6 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase7 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase8 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase9 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase10 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase11 {
            value.serialize_into(bytes);
        }
    }
}
pub fn select_events<'c, Conn>(conn: &'c Conn, device_spec: DeviceSpec, affect_which: u16, clear: u16, select_all: u16, affect_map: u16, map: u16, details: &SelectEventsAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let details_bytes = details.serialize();
    let length: usize = (16 + details_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let affect_which_bytes = affect_which.serialize();
    let clear_bytes = clear.serialize();
    let select_all_bytes = select_all.serialize();
    let affect_map_bytes = affect_map.serialize();
    let map_bytes = map.serialize();
    assert_eq!((affect_which) & ((!(clear)) & (!(select_all))), details.value_mask(),
        "Expression (affect_which) & ((!(clear)) & (!(select_all))) must match present values of 'details' argument (details.value_mask())");
    let request0 = [
        extension_information.major_opcode,
        SELECT_EVENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&details_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&details_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the Bell request
pub const BELL_REQUEST: u8 = 3;
pub fn bell<Conn>(conn: &Conn, device_spec: DeviceSpec, bell_class: BellClassSpec, bell_id: IDSpec, percent: i8, force_sound: bool, event_only: bool, pitch: i16, duration: i16, name: Atom, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let bell_class_bytes = bell_class.serialize();
    let bell_id_bytes = bell_id.serialize();
    let percent_bytes = percent.serialize();
    let force_sound_bytes = (force_sound as u8).serialize();
    let event_only_bytes = (event_only as u8).serialize();
    let pitch_bytes = pitch.serialize();
    let duration_bytes = duration.serialize();
    let name_bytes = name.serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        BELL_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetState request
pub const GET_STATE_REQUEST: u8 = 4;
pub fn get_state<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetStateReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_STATE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl GetStateReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
pub fn latch_lock_state<Conn, A>(conn: &Conn, device_spec: DeviceSpec, affect_mod_locks: u8, mod_locks: u8, lock_group: bool, group_lock: A, affect_mod_latches: u8, latch_group: bool, group_latch: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let affect_mod_locks_bytes = affect_mod_locks.serialize();
    let mod_locks_bytes = mod_locks.serialize();
    let lock_group_bytes = (lock_group as u8).serialize();
    let group_lock = group_lock.into();
    let group_lock_bytes = group_lock.serialize();
    let affect_mod_latches_bytes = affect_mod_latches.serialize();
    let latch_group_bytes = (latch_group as u8).serialize();
    let group_latch_bytes = group_latch.serialize();
    let request0 = [
        extension_information.major_opcode,
        LATCH_LOCK_STATE_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetControls request
pub const GET_CONTROLS_REQUEST: u8 = 6;
pub fn get_controls<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetControlsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CONTROLS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl GetControlsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
        let (per_key_repeat_0, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_1, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_2, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_3, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_4, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_5, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_6, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_7, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_8, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_9, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_10, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_11, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_12, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_13, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_14, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_15, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_16, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_17, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_18, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_19, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_20, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_21, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_22, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_23, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_24, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_25, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_26, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_27, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_28, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_29, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_30, remaining) = u8::try_parse(remaining)?;
        let (per_key_repeat_31, remaining) = u8::try_parse(remaining)?;
        let per_key_repeat = [
            per_key_repeat_0,
            per_key_repeat_1,
            per_key_repeat_2,
            per_key_repeat_3,
            per_key_repeat_4,
            per_key_repeat_5,
            per_key_repeat_6,
            per_key_repeat_7,
            per_key_repeat_8,
            per_key_repeat_9,
            per_key_repeat_10,
            per_key_repeat_11,
            per_key_repeat_12,
            per_key_repeat_13,
            per_key_repeat_14,
            per_key_repeat_15,
            per_key_repeat_16,
            per_key_repeat_17,
            per_key_repeat_18,
            per_key_repeat_19,
            per_key_repeat_20,
            per_key_repeat_21,
            per_key_repeat_22,
            per_key_repeat_23,
            per_key_repeat_24,
            per_key_repeat_25,
            per_key_repeat_26,
            per_key_repeat_27,
            per_key_repeat_28,
            per_key_repeat_29,
            per_key_repeat_30,
            per_key_repeat_31,
        ];
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
pub fn set_controls<'c, Conn>(conn: &'c Conn, device_spec: DeviceSpec, affect_internal_real_mods: u8, internal_real_mods: u8, affect_ignore_lock_real_mods: u8, ignore_lock_real_mods: u8, affect_internal_virtual_mods: u16, internal_virtual_mods: u16, affect_ignore_lock_virtual_mods: u16, ignore_lock_virtual_mods: u16, mouse_keys_dflt_btn: u8, groups_wrap: u8, access_x_options: u16, affect_enabled_controls: u32, enabled_controls: u32, change_controls: u32, repeat_delay: u16, repeat_interval: u16, slow_keys_delay: u16, debounce_delay: u16, mouse_keys_delay: u16, mouse_keys_interval: u16, mouse_keys_time_to_max: u16, mouse_keys_max_speed: u16, mouse_keys_curve: i16, access_x_timeout: u16, access_x_timeout_mask: u32, access_x_timeout_values: u32, access_x_timeout_options_mask: u16, access_x_timeout_options_values: u16, per_key_repeat: &[u8; 32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (100) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
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
    let request0 = [
        extension_information.major_opcode,
        SET_CONTROLS_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (per_key_repeat).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(per_key_repeat)], Vec::new())?)
}

/// Opcode for the GetMap request
pub const GET_MAP_REQUEST: u8 = 8;
pub fn get_map<Conn>(conn: &Conn, device_spec: DeviceSpec, full: u16, partial: u16, first_type: u8, n_types: u8, first_key_sym: Keycode, n_key_syms: u8, first_key_action: Keycode, n_key_actions: u8, first_key_behavior: Keycode, n_key_behaviors: u8, virtual_mods: u16, first_key_explicit: Keycode, n_key_explicit: u8, first_mod_map_key: Keycode, n_mod_map_keys: u8, first_v_mod_map_key: Keycode, n_v_mod_map_keys: u8) -> Result<Cookie<'_, Conn, GetMapReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
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
    let request0 = [
        extension_information.major_opcode,
        GET_MAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone)]
pub struct GetMapMapBitcase3 {
    pub acts_rtrn_count: Vec<u8>,
    pub acts_rtrn_acts: Vec<Action>,
}
impl GetMapMapBitcase3 {
    pub fn try_parse(remaining: &[u8], n_key_actions: u8, total_actions: u16) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (acts_rtrn_count, remaining) = crate::x11_utils::parse_list::<u8>(remaining, n_key_actions as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (acts_rtrn_acts, remaining) = crate::x11_utils::parse_list::<Action>(remaining, total_actions as usize)?;
        let result = GetMapMapBitcase3 { acts_rtrn_count, acts_rtrn_acts };
        Ok((result, remaining))
    }
}
// Skipping TryFrom implementations because of unresolved members
impl Serialize for GetMapMapBitcase3 {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(0);
        self.acts_rtrn_count.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.acts_rtrn_acts.serialize_into(bytes);
    }
}
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
        let mut outer_remaining = value;
        let types_rtrn = if present & Into::<u16>::into(MapPart::KeyTypes) != 0 {
            let remaining = outer_remaining;
            let (types_rtrn, remaining) = crate::x11_utils::parse_list::<KeyType>(remaining, n_types as usize)?;
            outer_remaining = remaining;
            Some(types_rtrn)
        } else {
            None
        };
        let syms_rtrn = if present & Into::<u16>::into(MapPart::KeySyms) != 0 {
            let remaining = outer_remaining;
            let (syms_rtrn, remaining) = crate::x11_utils::parse_list::<KeySymMap>(remaining, n_key_syms as usize)?;
            outer_remaining = remaining;
            Some(syms_rtrn)
        } else {
            None
        };
        let bitcase3 = if present & Into::<u16>::into(MapPart::KeyActions) != 0 {
            let (bitcase3, new_remaining) = GetMapMapBitcase3::try_parse(outer_remaining, n_key_actions, total_actions)?;
            outer_remaining = new_remaining;
            Some(bitcase3)
        } else {
            None
        };
        let behaviors_rtrn = if present & Into::<u16>::into(MapPart::KeyBehaviors) != 0 {
            let remaining = outer_remaining;
            let (behaviors_rtrn, remaining) = crate::x11_utils::parse_list::<SetBehavior>(remaining, total_key_behaviors as usize)?;
            outer_remaining = remaining;
            Some(behaviors_rtrn)
        } else {
            None
        };
        let vmods_rtrn = if present & Into::<u16>::into(MapPart::VirtualMods) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (vmods_rtrn, remaining) = crate::x11_utils::parse_list::<u8>(remaining, TryInto::<usize>::try_into(virtual_mods.count_ones()).unwrap())?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(vmods_rtrn)
        } else {
            None
        };
        let explicit_rtrn = if present & Into::<u16>::into(MapPart::ExplicitComponents) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (explicit_rtrn, remaining) = crate::x11_utils::parse_list::<SetExplicit>(remaining, total_key_explicit as usize)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(explicit_rtrn)
        } else {
            None
        };
        let modmap_rtrn = if present & Into::<u16>::into(MapPart::ModifierMap) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (modmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyModMap>(remaining, total_mod_map_keys as usize)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
            outer_remaining = remaining;
            Some(modmap_rtrn)
        } else {
            None
        };
        let vmodmap_rtrn = if present & Into::<u16>::into(MapPart::VirtualModMap) != 0 {
            let remaining = outer_remaining;
            let (vmodmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyVModMap>(remaining, total_v_mod_map_keys as usize)?;
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
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub present: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: Keycode,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: Keycode,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: Keycode,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: Keycode,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: Keycode,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_v_mod_map_key: Keycode,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub virtual_mods: u16,
    pub map: GetMapMap,
}
impl GetMapReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (min_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (present, remaining) = u16::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (total_types, remaining) = u8::try_parse(remaining)?;
        let (first_key_sym, remaining) = Keycode::try_parse(remaining)?;
        let (total_syms, remaining) = u16::try_parse(remaining)?;
        let (n_key_syms, remaining) = u8::try_parse(remaining)?;
        let (first_key_action, remaining) = Keycode::try_parse(remaining)?;
        let (total_actions, remaining) = u16::try_parse(remaining)?;
        let (n_key_actions, remaining) = u8::try_parse(remaining)?;
        let (first_key_behavior, remaining) = Keycode::try_parse(remaining)?;
        let (n_key_behaviors, remaining) = u8::try_parse(remaining)?;
        let (total_key_behaviors, remaining) = u8::try_parse(remaining)?;
        let (first_key_explicit, remaining) = Keycode::try_parse(remaining)?;
        let (n_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (total_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (first_mod_map_key, remaining) = Keycode::try_parse(remaining)?;
        let (n_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (total_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (first_v_mod_map_key, remaining) = Keycode::try_parse(remaining)?;
        let (n_v_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (total_v_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (map, remaining) = GetMapMap::try_parse(remaining, present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys)?;
        let result = GetMapReply { response_type, device_id, sequence, length, min_key_code, max_key_code, present, first_type, n_types, total_types, first_key_sym, total_syms, n_key_syms, first_key_action, total_actions, n_key_actions, first_key_behavior, n_key_behaviors, total_key_behaviors, first_key_explicit, n_key_explicit, total_key_explicit, first_mod_map_key, n_mod_map_keys, total_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, total_v_mod_map_keys, virtual_mods, map };
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
impl Serialize for SetMapAuxBitcase3 {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(0);
        self.actions_count.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.actions.serialize_into(bytes);
    }
}
/// Auxiliary and optional information for the set_map function.
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
impl SetMapAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u16 {
        let mut mask = 0;
        if self.types.is_some() {
            mask |= Into::<u16>::into(MapPart::KeyTypes);
        }
        if self.syms.is_some() {
            mask |= Into::<u16>::into(MapPart::KeySyms);
        }
        if self.bitcase3.is_some() {
            mask |= Into::<u16>::into(MapPart::KeyActions);
        }
        if self.behaviors.is_some() {
            mask |= Into::<u16>::into(MapPart::KeyBehaviors);
        }
        if self.vmods.is_some() {
            mask |= Into::<u16>::into(MapPart::VirtualMods);
        }
        if self.explicit.is_some() {
            mask |= Into::<u16>::into(MapPart::ExplicitComponents);
        }
        if self.modmap.is_some() {
            mask |= Into::<u16>::into(MapPart::ModifierMap);
        }
        if self.vmodmap.is_some() {
            mask |= Into::<u16>::into(MapPart::VirtualModMap);
        }
        mask
    }
    /// Set the types field of this structure.
    pub fn types<I>(mut self, value: I) -> Self where I: Into<Option<Vec<SetKeyType>>> {
        self.types = value.into();
        self
    }
    /// Set the syms field of this structure.
    pub fn syms<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeySymMap>>> {
        self.syms = value.into();
        self
    }
    /// Set the bitcase3 field of this structure.
    pub fn bitcase3<I>(mut self, value: I) -> Self where I: Into<Option<SetMapAuxBitcase3>> {
        self.bitcase3 = value.into();
        self
    }
    /// Set the behaviors field of this structure.
    pub fn behaviors<I>(mut self, value: I) -> Self where I: Into<Option<Vec<SetBehavior>>> {
        self.behaviors = value.into();
        self
    }
    /// Set the vmods field of this structure.
    pub fn vmods<I>(mut self, value: I) -> Self where I: Into<Option<Vec<u8>>> {
        self.vmods = value.into();
        self
    }
    /// Set the explicit field of this structure.
    pub fn explicit<I>(mut self, value: I) -> Self where I: Into<Option<Vec<SetExplicit>>> {
        self.explicit = value.into();
        self
    }
    /// Set the modmap field of this structure.
    pub fn modmap<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeyModMap>>> {
        self.modmap = value.into();
        self
    }
    /// Set the vmodmap field of this structure.
    pub fn vmodmap<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeyVModMap>>> {
        self.vmodmap = value.into();
        self
    }
}
impl Serialize for SetMapAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.types {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.syms {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase3 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.behaviors {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.vmods {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.explicit {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.modmap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.vmodmap {
            value.serialize_into(bytes);
        }
    }
}
pub fn set_map<'c, Conn>(conn: &'c Conn, device_spec: DeviceSpec, flags: u16, min_key_code: Keycode, max_key_code: Keycode, first_type: u8, n_types: u8, first_key_sym: Keycode, n_key_syms: u8, total_syms: u16, first_key_action: Keycode, n_key_actions: u8, total_actions: u16, first_key_behavior: Keycode, n_key_behaviors: u8, total_key_behaviors: u8, first_key_explicit: Keycode, n_key_explicit: u8, total_key_explicit: u8, first_mod_map_key: Keycode, n_mod_map_keys: u8, total_mod_map_keys: u8, first_v_mod_map_key: Keycode, n_v_mod_map_keys: u8, total_v_mod_map_keys: u8, virtual_mods: u16, values: &SetMapAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let present = values.value_mask();
    let values_bytes = values.serialize();
    let length: usize = (36 + values_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
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
    let request0 = [
        extension_information.major_opcode,
        SET_MAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&values_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&values_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetCompatMap request
pub const GET_COMPAT_MAP_REQUEST: u8 = 10;
pub fn get_compat_map<Conn>(conn: &Conn, device_spec: DeviceSpec, groups: u8, get_all_si: bool, first_si: u16, n_si: u16) -> Result<Cookie<'_, Conn, GetCompatMapReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let groups_bytes = groups.serialize();
    let get_all_si_bytes = (get_all_si as u8).serialize();
    let first_si_bytes = first_si.serialize();
    let n_si_bytes = n_si.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_COMPAT_MAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        groups_bytes[0],
        get_all_si_bytes[0],
        first_si_bytes[0],
        first_si_bytes[1],
        n_si_bytes[0],
        n_si_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl GetCompatMapReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
        let (si_rtrn, remaining) = crate::x11_utils::parse_list::<SymInterpret>(remaining, n_si_rtrn as usize)?;
        let (group_rtrn, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, TryInto::<usize>::try_into(groups_rtrn.count_ones()).unwrap())?;
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

/// Opcode for the SetCompatMap request
pub const SET_COMPAT_MAP_REQUEST: u8 = 11;
pub fn set_compat_map<'c, Conn>(conn: &'c Conn, device_spec: DeviceSpec, recompute_actions: bool, truncate_si: bool, groups: u8, first_si: u16, si: &[SymInterpret], group_maps: &[ModDef]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16 + 16 * si.len() + 4 * group_maps.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let recompute_actions_bytes = (recompute_actions as u8).serialize();
    let truncate_si_bytes = (truncate_si as u8).serialize();
    let groups_bytes = groups.serialize();
    let first_si_bytes = first_si.serialize();
    let n_si: u16 = si.len().try_into()?;
    let n_si_bytes = n_si.serialize();
    let si_bytes = si.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_COMPAT_MAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&si_bytes).len();
    assert_eq!(group_maps.len(), TryInto::<usize>::try_into(groups.count_ones()).unwrap(), "Argument groupMaps has an incorrect length");
    let group_maps_bytes = group_maps.serialize();
    let length_so_far = length_so_far + (&group_maps_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&si_bytes), IoSlice::new(&group_maps_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetIndicatorState request
pub const GET_INDICATOR_STATE_REQUEST: u8 = 12;
pub fn get_indicator_state<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetIndicatorStateReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_INDICATOR_STATE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetIndicatorStateReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}
impl GetIndicatorStateReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let which_bytes = which.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_INDICATOR_MAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
        which_bytes[0],
        which_bytes[1],
        which_bytes[2],
        which_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl GetIndicatorMapReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (real_indicators, remaining) = u32::try_parse(remaining)?;
        let (n_indicators, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::ParseError)?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, TryInto::<usize>::try_into(which.count_ones()).unwrap())?;
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
pub fn set_indicator_map<'c, Conn>(conn: &'c Conn, device_spec: DeviceSpec, which: u32, maps: &[IndicatorMap]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 12 * maps.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let which_bytes = which.serialize();
    assert_eq!(maps.len(), TryInto::<usize>::try_into(which.count_ones()).unwrap(), "Argument maps has an incorrect length");
    let maps_bytes = maps.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_INDICATOR_MAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
        which_bytes[0],
        which_bytes[1],
        which_bytes[2],
        which_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&maps_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&maps_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetNamedIndicator request
pub const GET_NAMED_INDICATOR_REQUEST: u8 = 15;
pub fn get_named_indicator<Conn, A>(conn: &Conn, device_spec: DeviceSpec, led_class: A, led_id: IDSpec, indicator: Atom) -> Result<Cookie<'_, Conn, GetNamedIndicatorReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<LedClassSpec>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let led_class = led_class.into();
    let led_class_bytes = led_class.serialize();
    let led_id_bytes = led_id.serialize();
    let indicator_bytes = indicator.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_NAMED_INDICATOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetNamedIndicatorReply {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub indicator: Atom,
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
impl GetNamedIndicatorReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (indicator, remaining) = Atom::try_parse(remaining)?;
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
pub fn set_named_indicator<Conn, A>(conn: &Conn, device_spec: DeviceSpec, led_class: A, led_id: IDSpec, indicator: Atom, set_state: bool, on: bool, set_map: bool, create_map: bool, map_flags: u8, map_which_groups: u8, map_groups: u8, map_which_mods: u8, map_real_mods: u8, map_vmods: u16, map_ctrls: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<LedClassSpec>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let led_class = led_class.into();
    let led_class_bytes = led_class.serialize();
    let led_id_bytes = led_id.serialize();
    let indicator_bytes = indicator.serialize();
    let set_state_bytes = (set_state as u8).serialize();
    let on_bytes = (on as u8).serialize();
    let set_map_bytes = (set_map as u8).serialize();
    let create_map_bytes = (create_map as u8).serialize();
    let map_flags_bytes = map_flags.serialize();
    let map_which_groups_bytes = map_which_groups.serialize();
    let map_groups_bytes = map_groups.serialize();
    let map_which_mods_bytes = map_which_mods.serialize();
    let map_real_mods_bytes = map_real_mods.serialize();
    let map_vmods_bytes = map_vmods.serialize();
    let map_ctrls_bytes = map_ctrls.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_NAMED_INDICATOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetNames request
pub const GET_NAMES_REQUEST: u8 = 17;
pub fn get_names<Conn>(conn: &Conn, device_spec: DeviceSpec, which: u32) -> Result<Cookie<'_, Conn, GetNamesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let which_bytes = which.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_NAMES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        0,
        0,
        which_bytes[0],
        which_bytes[1],
        which_bytes[2],
        which_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetNamesValueListBitcase8 {
    pub n_levels_per_type: Vec<u8>,
    pub kt_level_names: Vec<Atom>,
}
impl GetNamesValueListBitcase8 {
    pub fn try_parse(remaining: &[u8], n_types: u8) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (n_levels_per_type, remaining) = crate::x11_utils::parse_list::<u8>(remaining, n_types as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (kt_level_names, remaining) = crate::x11_utils::parse_list::<Atom>(remaining, n_levels_per_type.iter().map(|x| TryInto::<usize>::try_into(*x).unwrap()).sum())?;
        let result = GetNamesValueListBitcase8 { n_levels_per_type, kt_level_names };
        Ok((result, remaining))
    }
}
// Skipping TryFrom implementations because of unresolved members
impl Serialize for GetNamesValueListBitcase8 {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(0);
        self.n_levels_per_type.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.kt_level_names.serialize_into(bytes);
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GetNamesValueList {
    pub keycodes_name: Option<Atom>,
    pub geometry_name: Option<Atom>,
    pub symbols_name: Option<Atom>,
    pub phys_symbols_name: Option<Atom>,
    pub types_name: Option<Atom>,
    pub compat_name: Option<Atom>,
    pub type_names: Option<Vec<Atom>>,
    pub bitcase8: Option<GetNamesValueListBitcase8>,
    pub indicator_names: Option<Vec<Atom>>,
    pub virtual_mod_names: Option<Vec<Atom>>,
    pub groups: Option<Vec<Atom>>,
    pub key_names: Option<Vec<KeyName>>,
    pub key_aliases: Option<Vec<KeyAlias>>,
    pub radio_group_names: Option<Vec<Atom>>,
}
impl GetNamesValueList {
    fn try_parse(value: &[u8], which: u32, n_types: u8, indicators: u32, virtual_mods: u16, group_names: u8, n_keys: u8, n_key_aliases: u8, n_radio_groups: u8) -> Result<(Self, &[u8]), ParseError> {
        let mut outer_remaining = value;
        let keycodes_name = if which & Into::<u32>::into(NameDetail::Keycodes) != 0 {
            let remaining = outer_remaining;
            let (keycodes_name, remaining) = Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(keycodes_name)
        } else {
            None
        };
        let geometry_name = if which & Into::<u32>::into(NameDetail::Geometry) != 0 {
            let remaining = outer_remaining;
            let (geometry_name, remaining) = Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(geometry_name)
        } else {
            None
        };
        let symbols_name = if which & Into::<u32>::into(NameDetail::Symbols) != 0 {
            let remaining = outer_remaining;
            let (symbols_name, remaining) = Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(symbols_name)
        } else {
            None
        };
        let phys_symbols_name = if which & Into::<u32>::into(NameDetail::PhysSymbols) != 0 {
            let remaining = outer_remaining;
            let (phys_symbols_name, remaining) = Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(phys_symbols_name)
        } else {
            None
        };
        let types_name = if which & Into::<u32>::into(NameDetail::Types) != 0 {
            let remaining = outer_remaining;
            let (types_name, remaining) = Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(types_name)
        } else {
            None
        };
        let compat_name = if which & Into::<u32>::into(NameDetail::Compat) != 0 {
            let remaining = outer_remaining;
            let (compat_name, remaining) = Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(compat_name)
        } else {
            None
        };
        let type_names = if which & Into::<u32>::into(NameDetail::KeyTypeNames) != 0 {
            let remaining = outer_remaining;
            let (type_names, remaining) = crate::x11_utils::parse_list::<Atom>(remaining, n_types as usize)?;
            outer_remaining = remaining;
            Some(type_names)
        } else {
            None
        };
        let bitcase8 = if which & Into::<u32>::into(NameDetail::KTLevelNames) != 0 {
            let (bitcase8, new_remaining) = GetNamesValueListBitcase8::try_parse(outer_remaining, n_types)?;
            outer_remaining = new_remaining;
            Some(bitcase8)
        } else {
            None
        };
        let indicator_names = if which & Into::<u32>::into(NameDetail::IndicatorNames) != 0 {
            let remaining = outer_remaining;
            let (indicator_names, remaining) = crate::x11_utils::parse_list::<Atom>(remaining, TryInto::<usize>::try_into(indicators.count_ones()).unwrap())?;
            outer_remaining = remaining;
            Some(indicator_names)
        } else {
            None
        };
        let virtual_mod_names = if which & Into::<u32>::into(NameDetail::VirtualModNames) != 0 {
            let remaining = outer_remaining;
            let (virtual_mod_names, remaining) = crate::x11_utils::parse_list::<Atom>(remaining, TryInto::<usize>::try_into(virtual_mods.count_ones()).unwrap())?;
            outer_remaining = remaining;
            Some(virtual_mod_names)
        } else {
            None
        };
        let groups = if which & Into::<u32>::into(NameDetail::GroupNames) != 0 {
            let remaining = outer_remaining;
            let (groups, remaining) = crate::x11_utils::parse_list::<Atom>(remaining, TryInto::<usize>::try_into(group_names.count_ones()).unwrap())?;
            outer_remaining = remaining;
            Some(groups)
        } else {
            None
        };
        let key_names = if which & Into::<u32>::into(NameDetail::KeyNames) != 0 {
            let remaining = outer_remaining;
            let (key_names, remaining) = crate::x11_utils::parse_list::<KeyName>(remaining, n_keys as usize)?;
            outer_remaining = remaining;
            Some(key_names)
        } else {
            None
        };
        let key_aliases = if which & Into::<u32>::into(NameDetail::KeyAliases) != 0 {
            let remaining = outer_remaining;
            let (key_aliases, remaining) = crate::x11_utils::parse_list::<KeyAlias>(remaining, n_key_aliases as usize)?;
            outer_remaining = remaining;
            Some(key_aliases)
        } else {
            None
        };
        let radio_group_names = if which & Into::<u32>::into(NameDetail::RGNames) != 0 {
            let remaining = outer_remaining;
            let (radio_group_names, remaining) = crate::x11_utils::parse_list::<Atom>(remaining, n_radio_groups as usize)?;
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
    pub which: u32,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub n_types: u8,
    pub group_names: u8,
    pub virtual_mods: u16,
    pub first_key: Keycode,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_kt_levels: u16,
    pub value_list: GetNamesValueList,
}
impl GetNamesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (min_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (group_names, remaining) = u8::try_parse(remaining)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (first_key, remaining) = Keycode::try_parse(remaining)?;
        let (n_keys, remaining) = u8::try_parse(remaining)?;
        let (indicators, remaining) = u32::try_parse(remaining)?;
        let (n_radio_groups, remaining) = u8::try_parse(remaining)?;
        let (n_key_aliases, remaining) = u8::try_parse(remaining)?;
        let (n_kt_levels, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (value_list, remaining) = GetNamesValueList::try_parse(remaining, which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups)?;
        let result = GetNamesReply { response_type, device_id, sequence, length, which, min_key_code, max_key_code, n_types, group_names, virtual_mods, first_key, n_keys, indicators, n_radio_groups, n_key_aliases, n_kt_levels, value_list };
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
    pub kt_level_names: Vec<Atom>,
}
impl Serialize for SetNamesAuxBitcase8 {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(0);
        self.n_levels_per_type.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.kt_level_names.serialize_into(bytes);
    }
}
/// Auxiliary and optional information for the set_names function.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SetNamesAux {
    pub keycodes_name: Option<Atom>,
    pub geometry_name: Option<Atom>,
    pub symbols_name: Option<Atom>,
    pub phys_symbols_name: Option<Atom>,
    pub types_name: Option<Atom>,
    pub compat_name: Option<Atom>,
    pub type_names: Option<Vec<Atom>>,
    pub bitcase8: Option<SetNamesAuxBitcase8>,
    pub indicator_names: Option<Vec<Atom>>,
    pub virtual_mod_names: Option<Vec<Atom>>,
    pub groups: Option<Vec<Atom>>,
    pub key_names: Option<Vec<KeyName>>,
    pub key_aliases: Option<Vec<KeyAlias>>,
    pub radio_group_names: Option<Vec<Atom>>,
}
impl SetNamesAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.keycodes_name.is_some() {
            mask |= Into::<u32>::into(NameDetail::Keycodes);
        }
        if self.geometry_name.is_some() {
            mask |= Into::<u32>::into(NameDetail::Geometry);
        }
        if self.symbols_name.is_some() {
            mask |= Into::<u32>::into(NameDetail::Symbols);
        }
        if self.phys_symbols_name.is_some() {
            mask |= Into::<u32>::into(NameDetail::PhysSymbols);
        }
        if self.types_name.is_some() {
            mask |= Into::<u32>::into(NameDetail::Types);
        }
        if self.compat_name.is_some() {
            mask |= Into::<u32>::into(NameDetail::Compat);
        }
        if self.type_names.is_some() {
            mask |= Into::<u32>::into(NameDetail::KeyTypeNames);
        }
        if self.bitcase8.is_some() {
            mask |= Into::<u32>::into(NameDetail::KTLevelNames);
        }
        if self.indicator_names.is_some() {
            mask |= Into::<u32>::into(NameDetail::IndicatorNames);
        }
        if self.virtual_mod_names.is_some() {
            mask |= Into::<u32>::into(NameDetail::VirtualModNames);
        }
        if self.groups.is_some() {
            mask |= Into::<u32>::into(NameDetail::GroupNames);
        }
        if self.key_names.is_some() {
            mask |= Into::<u32>::into(NameDetail::KeyNames);
        }
        if self.key_aliases.is_some() {
            mask |= Into::<u32>::into(NameDetail::KeyAliases);
        }
        if self.radio_group_names.is_some() {
            mask |= Into::<u32>::into(NameDetail::RGNames);
        }
        mask
    }
    /// Set the keycodesName field of this structure.
    pub fn keycodes_name<I>(mut self, value: I) -> Self where I: Into<Option<Atom>> {
        self.keycodes_name = value.into();
        self
    }
    /// Set the geometryName field of this structure.
    pub fn geometry_name<I>(mut self, value: I) -> Self where I: Into<Option<Atom>> {
        self.geometry_name = value.into();
        self
    }
    /// Set the symbolsName field of this structure.
    pub fn symbols_name<I>(mut self, value: I) -> Self where I: Into<Option<Atom>> {
        self.symbols_name = value.into();
        self
    }
    /// Set the physSymbolsName field of this structure.
    pub fn phys_symbols_name<I>(mut self, value: I) -> Self where I: Into<Option<Atom>> {
        self.phys_symbols_name = value.into();
        self
    }
    /// Set the typesName field of this structure.
    pub fn types_name<I>(mut self, value: I) -> Self where I: Into<Option<Atom>> {
        self.types_name = value.into();
        self
    }
    /// Set the compatName field of this structure.
    pub fn compat_name<I>(mut self, value: I) -> Self where I: Into<Option<Atom>> {
        self.compat_name = value.into();
        self
    }
    /// Set the typeNames field of this structure.
    pub fn type_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<Atom>>> {
        self.type_names = value.into();
        self
    }
    /// Set the bitcase8 field of this structure.
    pub fn bitcase8<I>(mut self, value: I) -> Self where I: Into<Option<SetNamesAuxBitcase8>> {
        self.bitcase8 = value.into();
        self
    }
    /// Set the indicatorNames field of this structure.
    pub fn indicator_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<Atom>>> {
        self.indicator_names = value.into();
        self
    }
    /// Set the virtualModNames field of this structure.
    pub fn virtual_mod_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<Atom>>> {
        self.virtual_mod_names = value.into();
        self
    }
    /// Set the groups field of this structure.
    pub fn groups<I>(mut self, value: I) -> Self where I: Into<Option<Vec<Atom>>> {
        self.groups = value.into();
        self
    }
    /// Set the keyNames field of this structure.
    pub fn key_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeyName>>> {
        self.key_names = value.into();
        self
    }
    /// Set the keyAliases field of this structure.
    pub fn key_aliases<I>(mut self, value: I) -> Self where I: Into<Option<Vec<KeyAlias>>> {
        self.key_aliases = value.into();
        self
    }
    /// Set the radioGroupNames field of this structure.
    pub fn radio_group_names<I>(mut self, value: I) -> Self where I: Into<Option<Vec<Atom>>> {
        self.radio_group_names = value.into();
        self
    }
}
impl Serialize for SetNamesAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.keycodes_name {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.geometry_name {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.symbols_name {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.phys_symbols_name {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.types_name {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.compat_name {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.type_names {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bitcase8 {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.indicator_names {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.virtual_mod_names {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.groups {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.key_names {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.key_aliases {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.radio_group_names {
            value.serialize_into(bytes);
        }
    }
}
pub fn set_names<'c, Conn>(conn: &'c Conn, device_spec: DeviceSpec, virtual_mods: u16, first_type: u8, n_types: u8, first_kt_levelt: u8, n_kt_levels: u8, indicators: u32, group_names: u8, n_radio_groups: u8, first_key: Keycode, n_keys: u8, n_key_aliases: u8, total_kt_level_names: u16, values: &SetNamesAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let which = values.value_mask();
    let values_bytes = values.serialize();
    let length: usize = (28 + values_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let virtual_mods_bytes = virtual_mods.serialize();
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
    let request0 = [
        extension_information.major_opcode,
        SET_NAMES_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&values_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&values_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the PerClientFlags request
pub const PER_CLIENT_FLAGS_REQUEST: u8 = 21;
pub fn per_client_flags<Conn>(conn: &Conn, device_spec: DeviceSpec, change: u32, value: u32, ctrls_to_change: u32, auto_ctrls: u32, auto_ctrls_values: u32) -> Result<Cookie<'_, Conn, PerClientFlagsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let change_bytes = change.serialize();
    let value_bytes = value.serialize();
    let ctrls_to_change_bytes = ctrls_to_change.serialize();
    let auto_ctrls_bytes = auto_ctrls.serialize();
    let auto_ctrls_values_bytes = auto_ctrls_values.serialize();
    let request0 = [
        extension_information.major_opcode,
        PER_CLIENT_FLAGS_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl PerClientFlagsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let max_names_bytes = max_names.serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_COMPONENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        max_names_bytes[0],
        max_names_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl ListComponentsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
        let (keymaps, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_keymaps as usize)?;
        let (keycodes, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_keycodes as usize)?;
        let (types, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_types as usize)?;
        let (compat_maps, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_compat_maps as usize)?;
        let (symbols, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_symbols as usize)?;
        let (geometries, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_geometries as usize)?;
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

/// Opcode for the GetKbdByName request
pub const GET_KBD_BY_NAME_REQUEST: u8 = 23;
pub fn get_kbd_by_name<Conn>(_conn: &Conn) where Conn: RequestConnection + ?Sized {
unimplemented!("Not yet supported by the code generator") }
/// Opcode for the GetDeviceInfo request
pub const GET_DEVICE_INFO_REQUEST: u8 = 24;
pub fn get_device_info<Conn, A>(conn: &Conn, device_spec: DeviceSpec, wanted: u16, all_buttons: bool, first_button: u8, n_buttons: u8, led_class: A, led_id: IDSpec) -> Result<Cookie<'_, Conn, GetDeviceInfoReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<LedClassSpec>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let wanted_bytes = wanted.serialize();
    let all_buttons_bytes = (all_buttons as u8).serialize();
    let first_button_bytes = first_button.serialize();
    let n_buttons_bytes = n_buttons.serialize();
    let led_class = led_class.into();
    let led_class_bytes = led_class.serialize();
    let led_id_bytes = led_id.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DEVICE_INFO_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
    pub dev_type: Atom,
    pub name: Vec<String8>,
    pub btn_actions: Vec<Action>,
    pub leds: Vec<DeviceLedInfo>,
}
impl GetDeviceInfoReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
        let (dev_type, remaining) = Atom::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_list::<String8>(remaining, name_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (btn_actions, remaining) = crate::x11_utils::parse_list::<Action>(remaining, n_btns_rtrn as usize)?;
        let (leds, remaining) = crate::x11_utils::parse_list::<DeviceLedInfo>(remaining, n_device_led_f_bs as usize)?;
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

/// Opcode for the SetDeviceInfo request
pub const SET_DEVICE_INFO_REQUEST: u8 = 25;
pub fn set_device_info<'c, Conn>(conn: &'c Conn, device_spec: DeviceSpec, first_btn: u8, change: u16, btn_actions: &[Action], leds: &[DeviceLedInfo]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let leds_bytes = leds.serialize();
    let length: usize = (12 + 8 * btn_actions.len() + leds_bytes.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let device_spec_bytes = device_spec.serialize();
    let first_btn_bytes = first_btn.serialize();
    let n_btns: u8 = btn_actions.len().try_into()?;
    let n_btns_bytes = n_btns.serialize();
    let change_bytes = change.serialize();
    let n_device_led_f_bs: u16 = leds.len().try_into()?;
    let n_device_led_f_bs_bytes = n_device_led_f_bs.serialize();
    let btn_actions_bytes = btn_actions.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEVICE_INFO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        device_spec_bytes[0],
        device_spec_bytes[1],
        first_btn_bytes[0],
        n_btns_bytes[0],
        change_bytes[0],
        change_bytes[1],
        n_device_led_f_bs_bytes[0],
        n_device_led_f_bs_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&btn_actions_bytes).len();
    let length_so_far = length_so_far + (&leds_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&btn_actions_bytes), IoSlice::new(&leds_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the SetDebuggingFlags request
pub const SET_DEBUGGING_FLAGS_REQUEST: u8 = 101;
pub fn set_debugging_flags<'c, Conn>(conn: &'c Conn, affect_flags: u32, flags: u32, affect_ctrls: u32, ctrls: u32, message: &[String8]) -> Result<Cookie<'c, Conn, SetDebuggingFlagsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24 + 1 * message.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let msg_length: u16 = message.len().try_into()?;
    let msg_length_bytes = msg_length.serialize();
    let affect_flags_bytes = affect_flags.serialize();
    let flags_bytes = flags.serialize();
    let affect_ctrls_bytes = affect_ctrls.serialize();
    let ctrls_bytes = ctrls.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_DEBUGGING_FLAGS_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (message).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(message), IoSlice::new(&padding1)], Vec::new())?)
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
impl SetDebuggingFlagsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
    pub time: Timestamp,
    pub device_id: u8,
    pub old_device_id: u8,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub old_min_key_code: Keycode,
    pub old_max_key_code: Keycode,
    pub request_major: u8,
    pub request_minor: u8,
    pub changed: u16,
}
impl NewKeyboardNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (old_device_id, remaining) = u8::try_parse(remaining)?;
        let (min_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (old_min_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (old_max_key_code, remaining) = Keycode::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for NewKeyboardNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for NewKeyboardNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&NewKeyboardNotifyEvent> for [u8; 32] {
    fn from(input: &NewKeyboardNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let old_device_id = input.old_device_id.serialize();
        let min_key_code = input.min_key_code.serialize();
        let max_key_code = input.max_key_code.serialize();
        let old_min_key_code = input.old_min_key_code.serialize();
        let old_max_key_code = input.old_max_key_code.serialize();
        let request_major = input.request_major.serialize();
        let request_minor = input.request_minor.serialize();
        let changed = input.changed.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], old_device_id[0], min_key_code[0], max_key_code[0], old_min_key_code[0], old_max_key_code[0], request_major[0], request_minor[0],
            changed[0], changed[1], 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
    pub time: Timestamp,
    pub device_id: u8,
    pub ptr_btn_actions: u8,
    pub changed: u16,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: Keycode,
    pub n_key_syms: u8,
    pub first_key_act: Keycode,
    pub n_key_acts: u8,
    pub first_key_behavior: Keycode,
    pub n_key_behavior: u8,
    pub first_key_explicit: Keycode,
    pub n_key_explicit: u8,
    pub first_mod_map_key: Keycode,
    pub n_mod_map_keys: u8,
    pub first_v_mod_map_key: Keycode,
    pub n_v_mod_map_keys: u8,
    pub virtual_mods: u16,
}
impl MapNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (ptr_btn_actions, remaining) = u8::try_parse(remaining)?;
        let (changed, remaining) = u16::try_parse(remaining)?;
        let (min_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = Keycode::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (first_key_sym, remaining) = Keycode::try_parse(remaining)?;
        let (n_key_syms, remaining) = u8::try_parse(remaining)?;
        let (first_key_act, remaining) = Keycode::try_parse(remaining)?;
        let (n_key_acts, remaining) = u8::try_parse(remaining)?;
        let (first_key_behavior, remaining) = Keycode::try_parse(remaining)?;
        let (n_key_behavior, remaining) = u8::try_parse(remaining)?;
        let (first_key_explicit, remaining) = Keycode::try_parse(remaining)?;
        let (n_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (first_mod_map_key, remaining) = Keycode::try_parse(remaining)?;
        let (n_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (first_v_mod_map_key, remaining) = Keycode::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for MapNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for MapNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&MapNotifyEvent> for [u8; 32] {
    fn from(input: &MapNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let ptr_btn_actions = input.ptr_btn_actions.serialize();
        let changed = input.changed.serialize();
        let min_key_code = input.min_key_code.serialize();
        let max_key_code = input.max_key_code.serialize();
        let first_type = input.first_type.serialize();
        let n_types = input.n_types.serialize();
        let first_key_sym = input.first_key_sym.serialize();
        let n_key_syms = input.n_key_syms.serialize();
        let first_key_act = input.first_key_act.serialize();
        let n_key_acts = input.n_key_acts.serialize();
        let first_key_behavior = input.first_key_behavior.serialize();
        let n_key_behavior = input.n_key_behavior.serialize();
        let first_key_explicit = input.first_key_explicit.serialize();
        let n_key_explicit = input.n_key_explicit.serialize();
        let first_mod_map_key = input.first_mod_map_key.serialize();
        let n_mod_map_keys = input.n_mod_map_keys.serialize();
        let first_v_mod_map_key = input.first_v_mod_map_key.serialize();
        let n_v_mod_map_keys = input.n_v_mod_map_keys.serialize();
        let virtual_mods = input.virtual_mods.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], ptr_btn_actions[0], changed[0], changed[1], min_key_code[0], max_key_code[0], first_type[0], n_types[0],
            first_key_sym[0], n_key_syms[0], first_key_act[0], n_key_acts[0], first_key_behavior[0], n_key_behavior[0], first_key_explicit[0], n_key_explicit[0],
            first_mod_map_key[0], n_mod_map_keys[0], first_v_mod_map_key[0], n_v_mod_map_keys[0], virtual_mods[0], virtual_mods[1], 0, 0
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
    pub time: Timestamp,
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
    pub keycode: Keycode,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
}
impl StateNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
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
        let (keycode, remaining) = Keycode::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for StateNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for StateNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&StateNotifyEvent> for [u8; 32] {
    fn from(input: &StateNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let mods = input.mods.serialize();
        let base_mods = input.base_mods.serialize();
        let latched_mods = input.latched_mods.serialize();
        let locked_mods = input.locked_mods.serialize();
        let group = Into::<u8>::into(input.group).serialize();
        let base_group = input.base_group.serialize();
        let latched_group = input.latched_group.serialize();
        let locked_group = Into::<u8>::into(input.locked_group).serialize();
        let compat_state = input.compat_state.serialize();
        let grab_mods = input.grab_mods.serialize();
        let compat_grab_mods = input.compat_grab_mods.serialize();
        let lookup_mods = input.lookup_mods.serialize();
        let compat_loockup_mods = input.compat_loockup_mods.serialize();
        let ptr_btn_state = input.ptr_btn_state.serialize();
        let changed = input.changed.serialize();
        let keycode = input.keycode.serialize();
        let event_type = input.event_type.serialize();
        let request_major = input.request_major.serialize();
        let request_minor = input.request_minor.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], mods[0], base_mods[0], latched_mods[0], locked_mods[0], group[0], base_group[0], base_group[1],
            latched_group[0], latched_group[1], locked_group[0], compat_state[0], grab_mods[0], compat_grab_mods[0], lookup_mods[0], compat_loockup_mods[0],
            ptr_btn_state[0], ptr_btn_state[1], changed[0], changed[1], keycode[0], event_type[0], request_major[0], request_minor[0]
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
    pub time: Timestamp,
    pub device_id: u8,
    pub num_groups: u8,
    pub changed_controls: u32,
    pub enabled_controls: u32,
    pub enabled_control_changes: u32,
    pub keycode: Keycode,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
}
impl ControlsNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (num_groups, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (changed_controls, remaining) = u32::try_parse(remaining)?;
        let (enabled_controls, remaining) = u32::try_parse(remaining)?;
        let (enabled_control_changes, remaining) = u32::try_parse(remaining)?;
        let (keycode, remaining) = Keycode::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for ControlsNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for ControlsNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ControlsNotifyEvent> for [u8; 32] {
    fn from(input: &ControlsNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let num_groups = input.num_groups.serialize();
        let changed_controls = input.changed_controls.serialize();
        let enabled_controls = input.enabled_controls.serialize();
        let enabled_control_changes = input.enabled_control_changes.serialize();
        let keycode = input.keycode.serialize();
        let event_type = input.event_type.serialize();
        let request_major = input.request_major.serialize();
        let request_minor = input.request_minor.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], num_groups[0], 0, 0, changed_controls[0], changed_controls[1], changed_controls[2], changed_controls[3],
            enabled_controls[0], enabled_controls[1], enabled_controls[2], enabled_controls[3], enabled_control_changes[0], enabled_control_changes[1], enabled_control_changes[2], enabled_control_changes[3],
            keycode[0], event_type[0], request_major[0], request_minor[0], 0, 0, 0, 0
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
    pub time: Timestamp,
    pub device_id: u8,
    pub state: u32,
    pub state_changed: u32,
}
impl IndicatorStateNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for IndicatorStateNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for IndicatorStateNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&IndicatorStateNotifyEvent> for [u8; 32] {
    fn from(input: &IndicatorStateNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let state = input.state.serialize();
        let state_changed = input.state_changed.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], 0, 0, 0, state[0], state[1], state[2], state[3],
            state_changed[0], state_changed[1], state_changed[2], state_changed[3], 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
    pub time: Timestamp,
    pub device_id: u8,
    pub state: u32,
    pub map_changed: u32,
}
impl IndicatorMapNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for IndicatorMapNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for IndicatorMapNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&IndicatorMapNotifyEvent> for [u8; 32] {
    fn from(input: &IndicatorMapNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let state = input.state.serialize();
        let map_changed = input.map_changed.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], 0, 0, 0, state[0], state[1], state[2], state[3],
            map_changed[0], map_changed[1], map_changed[2], map_changed[3], 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
    pub time: Timestamp,
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
    pub first_key: Keycode,
    pub n_keys: u8,
    pub changed_indicators: u32,
}
impl NamesNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
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
        let (first_key, remaining) = Keycode::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for NamesNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for NamesNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&NamesNotifyEvent> for [u8; 32] {
    fn from(input: &NamesNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let changed = input.changed.serialize();
        let first_type = input.first_type.serialize();
        let n_types = input.n_types.serialize();
        let first_level_name = input.first_level_name.serialize();
        let n_level_names = input.n_level_names.serialize();
        let n_radio_groups = input.n_radio_groups.serialize();
        let n_key_aliases = input.n_key_aliases.serialize();
        let changed_group_names = input.changed_group_names.serialize();
        let changed_virtual_mods = input.changed_virtual_mods.serialize();
        let first_key = input.first_key.serialize();
        let n_keys = input.n_keys.serialize();
        let changed_indicators = input.changed_indicators.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], 0, changed[0], changed[1], first_type[0], n_types[0], first_level_name[0], n_level_names[0],
            0, n_radio_groups[0], n_key_aliases[0], changed_group_names[0], changed_virtual_mods[0], changed_virtual_mods[1], first_key[0], n_keys[0],
            changed_indicators[0], changed_indicators[1], changed_indicators[2], changed_indicators[3], 0, 0, 0, 0
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
    pub time: Timestamp,
    pub device_id: u8,
    pub changed_groups: u8,
    pub first_si: u16,
    pub n_si: u16,
    pub n_total_si: u16,
}
impl CompatMapNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for CompatMapNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for CompatMapNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&CompatMapNotifyEvent> for [u8; 32] {
    fn from(input: &CompatMapNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let changed_groups = input.changed_groups.serialize();
        let first_si = input.first_si.serialize();
        let n_si = input.n_si.serialize();
        let n_total_si = input.n_total_si.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], changed_groups[0], first_si[0], first_si[1], n_si[0], n_si[1], n_total_si[0], n_total_si[1],
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
    pub time: Timestamp,
    pub device_id: u8,
    pub bell_class: BellClassResult,
    pub bell_id: u8,
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
    pub name: Atom,
    pub window: Window,
    pub event_only: bool,
}
impl BellNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (bell_class, remaining) = u8::try_parse(remaining)?;
        let (bell_id, remaining) = u8::try_parse(remaining)?;
        let (percent, remaining) = u8::try_parse(remaining)?;
        let (pitch, remaining) = u16::try_parse(remaining)?;
        let (duration, remaining) = u16::try_parse(remaining)?;
        let (name, remaining) = Atom::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for BellNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for BellNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BellNotifyEvent> for [u8; 32] {
    fn from(input: &BellNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let bell_class = Into::<u8>::into(input.bell_class).serialize();
        let bell_id = input.bell_id.serialize();
        let percent = input.percent.serialize();
        let pitch = input.pitch.serialize();
        let duration = input.duration.serialize();
        let name = input.name.serialize();
        let window = input.window.serialize();
        let event_only = input.event_only.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], bell_class[0], bell_id[0], percent[0], pitch[0], pitch[1], duration[0], duration[1],
            name[0], name[1], name[2], name[3], window[0], window[1], window[2], window[3],
            event_only[0], 0, 0, 0, 0, 0, 0, 0
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
    pub time: Timestamp,
    pub device_id: u8,
    pub keycode: Keycode,
    pub press: bool,
    pub key_event_follows: bool,
    pub mods: u8,
    pub group: Group,
    pub message: [String8; 8],
}
impl ActionMessageEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (keycode, remaining) = Keycode::try_parse(remaining)?;
        let (press, remaining) = bool::try_parse(remaining)?;
        let (key_event_follows, remaining) = bool::try_parse(remaining)?;
        let (mods, remaining) = u8::try_parse(remaining)?;
        let (group, remaining) = u8::try_parse(remaining)?;
        let (message_0, remaining) = String8::try_parse(remaining)?;
        let (message_1, remaining) = String8::try_parse(remaining)?;
        let (message_2, remaining) = String8::try_parse(remaining)?;
        let (message_3, remaining) = String8::try_parse(remaining)?;
        let (message_4, remaining) = String8::try_parse(remaining)?;
        let (message_5, remaining) = String8::try_parse(remaining)?;
        let (message_6, remaining) = String8::try_parse(remaining)?;
        let (message_7, remaining) = String8::try_parse(remaining)?;
        let message = [
            message_0,
            message_1,
            message_2,
            message_3,
            message_4,
            message_5,
            message_6,
            message_7,
        ];
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for ActionMessageEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for ActionMessageEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ActionMessageEvent> for [u8; 32] {
    fn from(input: &ActionMessageEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let keycode = input.keycode.serialize();
        let press = input.press.serialize();
        let key_event_follows = input.key_event_follows.serialize();
        let mods = input.mods.serialize();
        let group = Into::<u8>::into(input.group).serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], keycode[0], press[0], key_event_follows[0], mods[0], group[0], input.message[0], input.message[1],
            input.message[2], input.message[3], input.message[4], input.message[5], input.message[6], input.message[7], 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
    pub time: Timestamp,
    pub device_id: u8,
    pub keycode: Keycode,
    pub detailt: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
}
impl AccessXNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (keycode, remaining) = Keycode::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for AccessXNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for AccessXNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&AccessXNotifyEvent> for [u8; 32] {
    fn from(input: &AccessXNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let keycode = input.keycode.serialize();
        let detailt = input.detailt.serialize();
        let slow_keys_delay = input.slow_keys_delay.serialize();
        let debounce_delay = input.debounce_delay.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], keycode[0], detailt[0], detailt[1], slow_keys_delay[0], slow_keys_delay[1], debounce_delay[0], debounce_delay[1],
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
    pub time: Timestamp,
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
impl ExtensionDeviceNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
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
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for ExtensionDeviceNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for ExtensionDeviceNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ExtensionDeviceNotifyEvent> for [u8; 32] {
    fn from(input: &ExtensionDeviceNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let xkb_type = input.xkb_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let device_id = input.device_id.serialize();
        let reason = input.reason.serialize();
        let led_class = Into::<u16>::into(input.led_class).serialize();
        let led_id = input.led_id.serialize();
        let leds_defined = input.leds_defined.serialize();
        let led_state = input.led_state.serialize();
        let first_button = input.first_button.serialize();
        let n_buttons = input.n_buttons.serialize();
        let supported = input.supported.serialize();
        let unsupported = input.unsupported.serialize();
        [
            response_type[0], xkb_type[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            device_id[0], 0, reason[0], reason[1], led_class[0], led_class[1], led_id[0], led_id[1],
            leds_defined[0], leds_defined[1], leds_defined[2], leds_defined[3], led_state[0], led_state[1], led_state[2], led_state[3],
            first_button[0], n_buttons[0], supported[0], supported[1], unsupported[0], unsupported[1], 0, 0
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

    fn xkb_select_events<'c>(&'c self, device_spec: DeviceSpec, affect_which: u16, clear: u16, select_all: u16, affect_map: u16, map: u16, details: &SelectEventsAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        select_events(self, device_spec, affect_which, clear, select_all, affect_map, map, details)
    }

    fn xkb_bell(&self, device_spec: DeviceSpec, bell_class: BellClassSpec, bell_id: IDSpec, percent: i8, force_sound: bool, event_only: bool, pitch: i16, duration: i16, name: Atom, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        bell(self, device_spec, bell_class, bell_id, percent, force_sound, event_only, pitch, duration, name, window)
    }

    fn xkb_get_state(&self, device_spec: DeviceSpec) -> Result<Cookie<'_, Self, GetStateReply>, ConnectionError>
    {
        get_state(self, device_spec)
    }

    fn xkb_latch_lock_state<A>(&self, device_spec: DeviceSpec, affect_mod_locks: u8, mod_locks: u8, lock_group: bool, group_lock: A, affect_mod_latches: u8, latch_group: bool, group_latch: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        latch_lock_state(self, device_spec, affect_mod_locks, mod_locks, lock_group, group_lock, affect_mod_latches, latch_group, group_latch)
    }

    fn xkb_get_controls(&self, device_spec: DeviceSpec) -> Result<Cookie<'_, Self, GetControlsReply>, ConnectionError>
    {
        get_controls(self, device_spec)
    }

    fn xkb_set_controls<'c>(&'c self, device_spec: DeviceSpec, affect_internal_real_mods: u8, internal_real_mods: u8, affect_ignore_lock_real_mods: u8, ignore_lock_real_mods: u8, affect_internal_virtual_mods: u16, internal_virtual_mods: u16, affect_ignore_lock_virtual_mods: u16, ignore_lock_virtual_mods: u16, mouse_keys_dflt_btn: u8, groups_wrap: u8, access_x_options: u16, affect_enabled_controls: u32, enabled_controls: u32, change_controls: u32, repeat_delay: u16, repeat_interval: u16, slow_keys_delay: u16, debounce_delay: u16, mouse_keys_delay: u16, mouse_keys_interval: u16, mouse_keys_time_to_max: u16, mouse_keys_max_speed: u16, mouse_keys_curve: i16, access_x_timeout: u16, access_x_timeout_mask: u32, access_x_timeout_values: u32, access_x_timeout_options_mask: u16, access_x_timeout_options_values: u16, per_key_repeat: &[u8; 32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_controls(self, device_spec, affect_internal_real_mods, internal_real_mods, affect_ignore_lock_real_mods, ignore_lock_real_mods, affect_internal_virtual_mods, internal_virtual_mods, affect_ignore_lock_virtual_mods, ignore_lock_virtual_mods, mouse_keys_dflt_btn, groups_wrap, access_x_options, affect_enabled_controls, enabled_controls, change_controls, repeat_delay, repeat_interval, slow_keys_delay, debounce_delay, mouse_keys_delay, mouse_keys_interval, mouse_keys_time_to_max, mouse_keys_max_speed, mouse_keys_curve, access_x_timeout, access_x_timeout_mask, access_x_timeout_values, access_x_timeout_options_mask, access_x_timeout_options_values, per_key_repeat)
    }

    fn xkb_get_map(&self, device_spec: DeviceSpec, full: u16, partial: u16, first_type: u8, n_types: u8, first_key_sym: Keycode, n_key_syms: u8, first_key_action: Keycode, n_key_actions: u8, first_key_behavior: Keycode, n_key_behaviors: u8, virtual_mods: u16, first_key_explicit: Keycode, n_key_explicit: u8, first_mod_map_key: Keycode, n_mod_map_keys: u8, first_v_mod_map_key: Keycode, n_v_mod_map_keys: u8) -> Result<Cookie<'_, Self, GetMapReply>, ConnectionError>
    {
        get_map(self, device_spec, full, partial, first_type, n_types, first_key_sym, n_key_syms, first_key_action, n_key_actions, first_key_behavior, n_key_behaviors, virtual_mods, first_key_explicit, n_key_explicit, first_mod_map_key, n_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys)
    }

    fn xkb_set_map<'c>(&'c self, device_spec: DeviceSpec, flags: u16, min_key_code: Keycode, max_key_code: Keycode, first_type: u8, n_types: u8, first_key_sym: Keycode, n_key_syms: u8, total_syms: u16, first_key_action: Keycode, n_key_actions: u8, total_actions: u16, first_key_behavior: Keycode, n_key_behaviors: u8, total_key_behaviors: u8, first_key_explicit: Keycode, n_key_explicit: u8, total_key_explicit: u8, first_mod_map_key: Keycode, n_mod_map_keys: u8, total_mod_map_keys: u8, first_v_mod_map_key: Keycode, n_v_mod_map_keys: u8, total_v_mod_map_keys: u8, virtual_mods: u16, values: &SetMapAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_map(self, device_spec, flags, min_key_code, max_key_code, first_type, n_types, first_key_sym, n_key_syms, total_syms, first_key_action, n_key_actions, total_actions, first_key_behavior, n_key_behaviors, total_key_behaviors, first_key_explicit, n_key_explicit, total_key_explicit, first_mod_map_key, n_mod_map_keys, total_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, total_v_mod_map_keys, virtual_mods, values)
    }

    fn xkb_get_compat_map(&self, device_spec: DeviceSpec, groups: u8, get_all_si: bool, first_si: u16, n_si: u16) -> Result<Cookie<'_, Self, GetCompatMapReply>, ConnectionError>
    {
        get_compat_map(self, device_spec, groups, get_all_si, first_si, n_si)
    }

    fn xkb_set_compat_map<'c>(&'c self, device_spec: DeviceSpec, recompute_actions: bool, truncate_si: bool, groups: u8, first_si: u16, si: &[SymInterpret], group_maps: &[ModDef]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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

    fn xkb_set_indicator_map<'c>(&'c self, device_spec: DeviceSpec, which: u32, maps: &[IndicatorMap]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_indicator_map(self, device_spec, which, maps)
    }

    fn xkb_get_named_indicator<A>(&self, device_spec: DeviceSpec, led_class: A, led_id: IDSpec, indicator: Atom) -> Result<Cookie<'_, Self, GetNamedIndicatorReply>, ConnectionError>
    where A: Into<LedClassSpec>
    {
        get_named_indicator(self, device_spec, led_class, led_id, indicator)
    }

    fn xkb_set_named_indicator<A>(&self, device_spec: DeviceSpec, led_class: A, led_id: IDSpec, indicator: Atom, set_state: bool, on: bool, set_map: bool, create_map: bool, map_flags: u8, map_which_groups: u8, map_groups: u8, map_which_mods: u8, map_real_mods: u8, map_vmods: u16, map_ctrls: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<LedClassSpec>
    {
        set_named_indicator(self, device_spec, led_class, led_id, indicator, set_state, on, set_map, create_map, map_flags, map_which_groups, map_groups, map_which_mods, map_real_mods, map_vmods, map_ctrls)
    }

    fn xkb_get_names(&self, device_spec: DeviceSpec, which: u32) -> Result<Cookie<'_, Self, GetNamesReply>, ConnectionError>
    {
        get_names(self, device_spec, which)
    }

    fn xkb_set_names<'c>(&'c self, device_spec: DeviceSpec, virtual_mods: u16, first_type: u8, n_types: u8, first_kt_levelt: u8, n_kt_levels: u8, indicators: u32, group_names: u8, n_radio_groups: u8, first_key: Keycode, n_keys: u8, n_key_aliases: u8, total_kt_level_names: u16, values: &SetNamesAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_names(self, device_spec, virtual_mods, first_type, n_types, first_kt_levelt, n_kt_levels, indicators, group_names, n_radio_groups, first_key, n_keys, n_key_aliases, total_kt_level_names, values)
    }

    fn xkb_per_client_flags(&self, device_spec: DeviceSpec, change: u32, value: u32, ctrls_to_change: u32, auto_ctrls: u32, auto_ctrls_values: u32) -> Result<Cookie<'_, Self, PerClientFlagsReply>, ConnectionError>
    {
        per_client_flags(self, device_spec, change, value, ctrls_to_change, auto_ctrls, auto_ctrls_values)
    }

    fn xkb_list_components(&self, device_spec: DeviceSpec, max_names: u16) -> Result<Cookie<'_, Self, ListComponentsReply>, ConnectionError>
    {
        list_components(self, device_spec, max_names)
    }

    fn get_kbd_by_name(&self) { unimplemented!("Not yet supported by the code generator") }
    fn xkb_get_device_info<A>(&self, device_spec: DeviceSpec, wanted: u16, all_buttons: bool, first_button: u8, n_buttons: u8, led_class: A, led_id: IDSpec) -> Result<Cookie<'_, Self, GetDeviceInfoReply>, ConnectionError>
    where A: Into<LedClassSpec>
    {
        get_device_info(self, device_spec, wanted, all_buttons, first_button, n_buttons, led_class, led_id)
    }

    fn xkb_set_device_info<'c>(&'c self, device_spec: DeviceSpec, first_btn: u8, change: u16, btn_actions: &[Action], leds: &[DeviceLedInfo]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_info(self, device_spec, first_btn, change, btn_actions, leds)
    }

    fn xkb_set_debugging_flags<'c>(&'c self, affect_flags: u32, flags: u32, affect_ctrls: u32, ctrls: u32, message: &[String8]) -> Result<Cookie<'c, Self, SetDebuggingFlagsReply>, ConnectionError>
    {
        set_debugging_flags(self, affect_flags, flags, affect_ctrls, ctrls, message)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
