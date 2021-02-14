// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `xkb` X11 extension.

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Const(u8);
impl Const {
    pub const MAX_LEGAL_KEY_CODE: Self = Self(255);
    pub const PER_KEY_BIT_ARRAY_SIZE: Self = Self(32);
    pub const KEY_NAME_LENGTH: Self = Self(4);
}
impl From<Const> for u8 {
    #[inline]
    fn from(input: Const) -> Self {
        input.0
    }
}
impl From<Const> for Option<u8> {
    #[inline]
    fn from(input: Const) -> Self {
        Some(input.0)
    }
}
impl From<Const> for u16 {
    #[inline]
    fn from(input: Const) -> Self {
        u16::from(input.0)
    }
}
impl From<Const> for Option<u16> {
    #[inline]
    fn from(input: Const) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Const> for u32 {
    #[inline]
    fn from(input: Const) -> Self {
        u32::from(input.0)
    }
}
impl From<Const> for Option<u32> {
    #[inline]
    fn from(input: Const) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Const {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Const  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::MAX_LEGAL_KEY_CODE.0.into(), "MAX_LEGAL_KEY_CODE", "MaxLegalKeyCode"),
            (Self::PER_KEY_BIT_ARRAY_SIZE.0.into(), "PER_KEY_BIT_ARRAY_SIZE", "PerKeyBitArraySize"),
            (Self::KEY_NAME_LENGTH.0.into(), "KEY_NAME_LENGTH", "KeyNameLength"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EventType(u16);
impl EventType {
    pub const NEW_KEYBOARD_NOTIFY: Self = Self(1 << 0);
    pub const MAP_NOTIFY: Self = Self(1 << 1);
    pub const STATE_NOTIFY: Self = Self(1 << 2);
    pub const CONTROLS_NOTIFY: Self = Self(1 << 3);
    pub const INDICATOR_STATE_NOTIFY: Self = Self(1 << 4);
    pub const INDICATOR_MAP_NOTIFY: Self = Self(1 << 5);
    pub const NAMES_NOTIFY: Self = Self(1 << 6);
    pub const COMPAT_MAP_NOTIFY: Self = Self(1 << 7);
    pub const BELL_NOTIFY: Self = Self(1 << 8);
    pub const ACTION_MESSAGE: Self = Self(1 << 9);
    pub const ACCESS_X_NOTIFY: Self = Self(1 << 10);
    pub const EXTENSION_DEVICE_NOTIFY: Self = Self(1 << 11);
}
impl From<EventType> for u16 {
    #[inline]
    fn from(input: EventType) -> Self {
        input.0
    }
}
impl From<EventType> for Option<u16> {
    #[inline]
    fn from(input: EventType) -> Self {
        Some(input.0)
    }
}
impl From<EventType> for u32 {
    #[inline]
    fn from(input: EventType) -> Self {
        u32::from(input.0)
    }
}
impl From<EventType> for Option<u32> {
    #[inline]
    fn from(input: EventType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for EventType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for EventType {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for EventType  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::NEW_KEYBOARD_NOTIFY.0.into(), "NEW_KEYBOARD_NOTIFY", "NewKeyboardNotify"),
            (Self::MAP_NOTIFY.0.into(), "MAP_NOTIFY", "MapNotify"),
            (Self::STATE_NOTIFY.0.into(), "STATE_NOTIFY", "StateNotify"),
            (Self::CONTROLS_NOTIFY.0.into(), "CONTROLS_NOTIFY", "ControlsNotify"),
            (Self::INDICATOR_STATE_NOTIFY.0.into(), "INDICATOR_STATE_NOTIFY", "IndicatorStateNotify"),
            (Self::INDICATOR_MAP_NOTIFY.0.into(), "INDICATOR_MAP_NOTIFY", "IndicatorMapNotify"),
            (Self::NAMES_NOTIFY.0.into(), "NAMES_NOTIFY", "NamesNotify"),
            (Self::COMPAT_MAP_NOTIFY.0.into(), "COMPAT_MAP_NOTIFY", "CompatMapNotify"),
            (Self::BELL_NOTIFY.0.into(), "BELL_NOTIFY", "BellNotify"),
            (Self::ACTION_MESSAGE.0.into(), "ACTION_MESSAGE", "ActionMessage"),
            (Self::ACCESS_X_NOTIFY.0.into(), "ACCESS_X_NOTIFY", "AccessXNotify"),
            (Self::EXTENSION_DEVICE_NOTIFY.0.into(), "EXTENSION_DEVICE_NOTIFY", "ExtensionDeviceNotify"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(EventType, u16);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NKNDetail(u8);
impl NKNDetail {
    pub const KEYCODES: Self = Self(1 << 0);
    pub const GEOMETRY: Self = Self(1 << 1);
    pub const DEVICE_ID: Self = Self(1 << 2);
}
impl From<NKNDetail> for u8 {
    #[inline]
    fn from(input: NKNDetail) -> Self {
        input.0
    }
}
impl From<NKNDetail> for Option<u8> {
    #[inline]
    fn from(input: NKNDetail) -> Self {
        Some(input.0)
    }
}
impl From<NKNDetail> for u16 {
    #[inline]
    fn from(input: NKNDetail) -> Self {
        u16::from(input.0)
    }
}
impl From<NKNDetail> for Option<u16> {
    #[inline]
    fn from(input: NKNDetail) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<NKNDetail> for u32 {
    #[inline]
    fn from(input: NKNDetail) -> Self {
        u32::from(input.0)
    }
}
impl From<NKNDetail> for Option<u32> {
    #[inline]
    fn from(input: NKNDetail) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for NKNDetail {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for NKNDetail  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::KEYCODES.0.into(), "KEYCODES", "Keycodes"),
            (Self::GEOMETRY.0.into(), "GEOMETRY", "Geometry"),
            (Self::DEVICE_ID.0.into(), "DEVICE_ID", "DeviceID"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(NKNDetail, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AXNDetail(u8);
impl AXNDetail {
    pub const SK_PRESS: Self = Self(1 << 0);
    pub const SK_ACCEPT: Self = Self(1 << 1);
    pub const SK_REJECT: Self = Self(1 << 2);
    pub const SK_RELEASE: Self = Self(1 << 3);
    pub const BK_ACCEPT: Self = Self(1 << 4);
    pub const BK_REJECT: Self = Self(1 << 5);
    pub const AXK_WARNING: Self = Self(1 << 6);
}
impl From<AXNDetail> for u8 {
    #[inline]
    fn from(input: AXNDetail) -> Self {
        input.0
    }
}
impl From<AXNDetail> for Option<u8> {
    #[inline]
    fn from(input: AXNDetail) -> Self {
        Some(input.0)
    }
}
impl From<AXNDetail> for u16 {
    #[inline]
    fn from(input: AXNDetail) -> Self {
        u16::from(input.0)
    }
}
impl From<AXNDetail> for Option<u16> {
    #[inline]
    fn from(input: AXNDetail) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<AXNDetail> for u32 {
    #[inline]
    fn from(input: AXNDetail) -> Self {
        u32::from(input.0)
    }
}
impl From<AXNDetail> for Option<u32> {
    #[inline]
    fn from(input: AXNDetail) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for AXNDetail {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for AXNDetail  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::SK_PRESS.0.into(), "SK_PRESS", "SKPress"),
            (Self::SK_ACCEPT.0.into(), "SK_ACCEPT", "SKAccept"),
            (Self::SK_REJECT.0.into(), "SK_REJECT", "SKReject"),
            (Self::SK_RELEASE.0.into(), "SK_RELEASE", "SKRelease"),
            (Self::BK_ACCEPT.0.into(), "BK_ACCEPT", "BKAccept"),
            (Self::BK_REJECT.0.into(), "BK_REJECT", "BKReject"),
            (Self::AXK_WARNING.0.into(), "AXK_WARNING", "AXKWarning"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(AXNDetail, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MapPart(u8);
impl MapPart {
    pub const KEY_TYPES: Self = Self(1 << 0);
    pub const KEY_SYMS: Self = Self(1 << 1);
    pub const MODIFIER_MAP: Self = Self(1 << 2);
    pub const EXPLICIT_COMPONENTS: Self = Self(1 << 3);
    pub const KEY_ACTIONS: Self = Self(1 << 4);
    pub const KEY_BEHAVIORS: Self = Self(1 << 5);
    pub const VIRTUAL_MODS: Self = Self(1 << 6);
    pub const VIRTUAL_MOD_MAP: Self = Self(1 << 7);
}
impl From<MapPart> for u8 {
    #[inline]
    fn from(input: MapPart) -> Self {
        input.0
    }
}
impl From<MapPart> for Option<u8> {
    #[inline]
    fn from(input: MapPart) -> Self {
        Some(input.0)
    }
}
impl From<MapPart> for u16 {
    #[inline]
    fn from(input: MapPart) -> Self {
        u16::from(input.0)
    }
}
impl From<MapPart> for Option<u16> {
    #[inline]
    fn from(input: MapPart) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<MapPart> for u32 {
    #[inline]
    fn from(input: MapPart) -> Self {
        u32::from(input.0)
    }
}
impl From<MapPart> for Option<u32> {
    #[inline]
    fn from(input: MapPart) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for MapPart {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for MapPart  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::KEY_TYPES.0.into(), "KEY_TYPES", "KeyTypes"),
            (Self::KEY_SYMS.0.into(), "KEY_SYMS", "KeySyms"),
            (Self::MODIFIER_MAP.0.into(), "MODIFIER_MAP", "ModifierMap"),
            (Self::EXPLICIT_COMPONENTS.0.into(), "EXPLICIT_COMPONENTS", "ExplicitComponents"),
            (Self::KEY_ACTIONS.0.into(), "KEY_ACTIONS", "KeyActions"),
            (Self::KEY_BEHAVIORS.0.into(), "KEY_BEHAVIORS", "KeyBehaviors"),
            (Self::VIRTUAL_MODS.0.into(), "VIRTUAL_MODS", "VirtualMods"),
            (Self::VIRTUAL_MOD_MAP.0.into(), "VIRTUAL_MOD_MAP", "VirtualModMap"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(MapPart, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SetMapFlags(u8);
impl SetMapFlags {
    pub const RESIZE_TYPES: Self = Self(1 << 0);
    pub const RECOMPUTE_ACTIONS: Self = Self(1 << 1);
}
impl From<SetMapFlags> for u8 {
    #[inline]
    fn from(input: SetMapFlags) -> Self {
        input.0
    }
}
impl From<SetMapFlags> for Option<u8> {
    #[inline]
    fn from(input: SetMapFlags) -> Self {
        Some(input.0)
    }
}
impl From<SetMapFlags> for u16 {
    #[inline]
    fn from(input: SetMapFlags) -> Self {
        u16::from(input.0)
    }
}
impl From<SetMapFlags> for Option<u16> {
    #[inline]
    fn from(input: SetMapFlags) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SetMapFlags> for u32 {
    #[inline]
    fn from(input: SetMapFlags) -> Self {
        u32::from(input.0)
    }
}
impl From<SetMapFlags> for Option<u32> {
    #[inline]
    fn from(input: SetMapFlags) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SetMapFlags {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SetMapFlags  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::RESIZE_TYPES.0.into(), "RESIZE_TYPES", "ResizeTypes"),
            (Self::RECOMPUTE_ACTIONS.0.into(), "RECOMPUTE_ACTIONS", "RecomputeActions"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(SetMapFlags, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StatePart(u16);
impl StatePart {
    pub const MODIFIER_STATE: Self = Self(1 << 0);
    pub const MODIFIER_BASE: Self = Self(1 << 1);
    pub const MODIFIER_LATCH: Self = Self(1 << 2);
    pub const MODIFIER_LOCK: Self = Self(1 << 3);
    pub const GROUP_STATE: Self = Self(1 << 4);
    pub const GROUP_BASE: Self = Self(1 << 5);
    pub const GROUP_LATCH: Self = Self(1 << 6);
    pub const GROUP_LOCK: Self = Self(1 << 7);
    pub const COMPAT_STATE: Self = Self(1 << 8);
    pub const GRAB_MODS: Self = Self(1 << 9);
    pub const COMPAT_GRAB_MODS: Self = Self(1 << 10);
    pub const LOOKUP_MODS: Self = Self(1 << 11);
    pub const COMPAT_LOOKUP_MODS: Self = Self(1 << 12);
    pub const POINTER_BUTTONS: Self = Self(1 << 13);
}
impl From<StatePart> for u16 {
    #[inline]
    fn from(input: StatePart) -> Self {
        input.0
    }
}
impl From<StatePart> for Option<u16> {
    #[inline]
    fn from(input: StatePart) -> Self {
        Some(input.0)
    }
}
impl From<StatePart> for u32 {
    #[inline]
    fn from(input: StatePart) -> Self {
        u32::from(input.0)
    }
}
impl From<StatePart> for Option<u32> {
    #[inline]
    fn from(input: StatePart) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for StatePart {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for StatePart {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for StatePart  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::MODIFIER_STATE.0.into(), "MODIFIER_STATE", "ModifierState"),
            (Self::MODIFIER_BASE.0.into(), "MODIFIER_BASE", "ModifierBase"),
            (Self::MODIFIER_LATCH.0.into(), "MODIFIER_LATCH", "ModifierLatch"),
            (Self::MODIFIER_LOCK.0.into(), "MODIFIER_LOCK", "ModifierLock"),
            (Self::GROUP_STATE.0.into(), "GROUP_STATE", "GroupState"),
            (Self::GROUP_BASE.0.into(), "GROUP_BASE", "GroupBase"),
            (Self::GROUP_LATCH.0.into(), "GROUP_LATCH", "GroupLatch"),
            (Self::GROUP_LOCK.0.into(), "GROUP_LOCK", "GroupLock"),
            (Self::COMPAT_STATE.0.into(), "COMPAT_STATE", "CompatState"),
            (Self::GRAB_MODS.0.into(), "GRAB_MODS", "GrabMods"),
            (Self::COMPAT_GRAB_MODS.0.into(), "COMPAT_GRAB_MODS", "CompatGrabMods"),
            (Self::LOOKUP_MODS.0.into(), "LOOKUP_MODS", "LookupMods"),
            (Self::COMPAT_LOOKUP_MODS.0.into(), "COMPAT_LOOKUP_MODS", "CompatLookupMods"),
            (Self::POINTER_BUTTONS.0.into(), "POINTER_BUTTONS", "PointerButtons"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(StatePart, u16);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoolCtrl(u16);
impl BoolCtrl {
    pub const REPEAT_KEYS: Self = Self(1 << 0);
    pub const SLOW_KEYS: Self = Self(1 << 1);
    pub const BOUNCE_KEYS: Self = Self(1 << 2);
    pub const STICKY_KEYS: Self = Self(1 << 3);
    pub const MOUSE_KEYS: Self = Self(1 << 4);
    pub const MOUSE_KEYS_ACCEL: Self = Self(1 << 5);
    pub const ACCESS_X_KEYS: Self = Self(1 << 6);
    pub const ACCESS_X_TIMEOUT_MASK: Self = Self(1 << 7);
    pub const ACCESS_X_FEEDBACK_MASK: Self = Self(1 << 8);
    pub const AUDIBLE_BELL_MASK: Self = Self(1 << 9);
    pub const OVERLAY1_MASK: Self = Self(1 << 10);
    pub const OVERLAY2_MASK: Self = Self(1 << 11);
    pub const IGNORE_GROUP_LOCK_MASK: Self = Self(1 << 12);
}
impl From<BoolCtrl> for u16 {
    #[inline]
    fn from(input: BoolCtrl) -> Self {
        input.0
    }
}
impl From<BoolCtrl> for Option<u16> {
    #[inline]
    fn from(input: BoolCtrl) -> Self {
        Some(input.0)
    }
}
impl From<BoolCtrl> for u32 {
    #[inline]
    fn from(input: BoolCtrl) -> Self {
        u32::from(input.0)
    }
}
impl From<BoolCtrl> for Option<u32> {
    #[inline]
    fn from(input: BoolCtrl) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for BoolCtrl {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for BoolCtrl {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for BoolCtrl  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::REPEAT_KEYS.0.into(), "REPEAT_KEYS", "RepeatKeys"),
            (Self::SLOW_KEYS.0.into(), "SLOW_KEYS", "SlowKeys"),
            (Self::BOUNCE_KEYS.0.into(), "BOUNCE_KEYS", "BounceKeys"),
            (Self::STICKY_KEYS.0.into(), "STICKY_KEYS", "StickyKeys"),
            (Self::MOUSE_KEYS.0.into(), "MOUSE_KEYS", "MouseKeys"),
            (Self::MOUSE_KEYS_ACCEL.0.into(), "MOUSE_KEYS_ACCEL", "MouseKeysAccel"),
            (Self::ACCESS_X_KEYS.0.into(), "ACCESS_X_KEYS", "AccessXKeys"),
            (Self::ACCESS_X_TIMEOUT_MASK.0.into(), "ACCESS_X_TIMEOUT_MASK", "AccessXTimeoutMask"),
            (Self::ACCESS_X_FEEDBACK_MASK.0.into(), "ACCESS_X_FEEDBACK_MASK", "AccessXFeedbackMask"),
            (Self::AUDIBLE_BELL_MASK.0.into(), "AUDIBLE_BELL_MASK", "AudibleBellMask"),
            (Self::OVERLAY1_MASK.0.into(), "OVERLAY1_MASK", "Overlay1Mask"),
            (Self::OVERLAY2_MASK.0.into(), "OVERLAY2_MASK", "Overlay2Mask"),
            (Self::IGNORE_GROUP_LOCK_MASK.0.into(), "IGNORE_GROUP_LOCK_MASK", "IgnoreGroupLockMask"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(BoolCtrl, u16);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Control(u32);
impl Control {
    pub const GROUPS_WRAP: Self = Self(1 << 27);
    pub const INTERNAL_MODS: Self = Self(1 << 28);
    pub const IGNORE_LOCK_MODS: Self = Self(1 << 29);
    pub const PER_KEY_REPEAT: Self = Self(1 << 30);
    pub const CONTROLS_ENABLED: Self = Self(1 << 31);
}
impl From<Control> for u32 {
    #[inline]
    fn from(input: Control) -> Self {
        input.0
    }
}
impl From<Control> for Option<u32> {
    #[inline]
    fn from(input: Control) -> Self {
        Some(input.0)
    }
}
impl From<u8> for Control {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for Control {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for Control {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Control  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::GROUPS_WRAP.0, "GROUPS_WRAP", "GroupsWrap"),
            (Self::INTERNAL_MODS.0, "INTERNAL_MODS", "InternalMods"),
            (Self::IGNORE_LOCK_MODS.0, "IGNORE_LOCK_MODS", "IgnoreLockMods"),
            (Self::PER_KEY_REPEAT.0, "PER_KEY_REPEAT", "PerKeyRepeat"),
            (Self::CONTROLS_ENABLED.0, "CONTROLS_ENABLED", "ControlsEnabled"),
        ];
        pretty_print_bitmask(fmt, self.0, &variants)
    }
}
bitmask_binop!(Control, u32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AXOption(u16);
impl AXOption {
    pub const SK_PRESS_FB: Self = Self(1 << 0);
    pub const SK_ACCEPT_FB: Self = Self(1 << 1);
    pub const FEATURE_FB: Self = Self(1 << 2);
    pub const SLOW_WARN_FB: Self = Self(1 << 3);
    pub const INDICATOR_FB: Self = Self(1 << 4);
    pub const STICKY_KEYS_FB: Self = Self(1 << 5);
    pub const TWO_KEYS: Self = Self(1 << 6);
    pub const LATCH_TO_LOCK: Self = Self(1 << 7);
    pub const SK_RELEASE_FB: Self = Self(1 << 8);
    pub const SK_REJECT_FB: Self = Self(1 << 9);
    pub const BK_REJECT_FB: Self = Self(1 << 10);
    pub const DUMB_BELL: Self = Self(1 << 11);
}
impl From<AXOption> for u16 {
    #[inline]
    fn from(input: AXOption) -> Self {
        input.0
    }
}
impl From<AXOption> for Option<u16> {
    #[inline]
    fn from(input: AXOption) -> Self {
        Some(input.0)
    }
}
impl From<AXOption> for u32 {
    #[inline]
    fn from(input: AXOption) -> Self {
        u32::from(input.0)
    }
}
impl From<AXOption> for Option<u32> {
    #[inline]
    fn from(input: AXOption) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for AXOption {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for AXOption {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for AXOption  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::SK_PRESS_FB.0.into(), "SK_PRESS_FB", "SKPressFB"),
            (Self::SK_ACCEPT_FB.0.into(), "SK_ACCEPT_FB", "SKAcceptFB"),
            (Self::FEATURE_FB.0.into(), "FEATURE_FB", "FeatureFB"),
            (Self::SLOW_WARN_FB.0.into(), "SLOW_WARN_FB", "SlowWarnFB"),
            (Self::INDICATOR_FB.0.into(), "INDICATOR_FB", "IndicatorFB"),
            (Self::STICKY_KEYS_FB.0.into(), "STICKY_KEYS_FB", "StickyKeysFB"),
            (Self::TWO_KEYS.0.into(), "TWO_KEYS", "TwoKeys"),
            (Self::LATCH_TO_LOCK.0.into(), "LATCH_TO_LOCK", "LatchToLock"),
            (Self::SK_RELEASE_FB.0.into(), "SK_RELEASE_FB", "SKReleaseFB"),
            (Self::SK_REJECT_FB.0.into(), "SK_REJECT_FB", "SKRejectFB"),
            (Self::BK_REJECT_FB.0.into(), "BK_REJECT_FB", "BKRejectFB"),
            (Self::DUMB_BELL.0.into(), "DUMB_BELL", "DumbBell"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(AXOption, u16);

pub type DeviceSpec = u16;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LedClassResult(u16);
impl LedClassResult {
    pub const KBD_FEEDBACK_CLASS: Self = Self(0);
    pub const LED_FEEDBACK_CLASS: Self = Self(4);
}
impl From<LedClassResult> for u16 {
    #[inline]
    fn from(input: LedClassResult) -> Self {
        input.0
    }
}
impl From<LedClassResult> for Option<u16> {
    #[inline]
    fn from(input: LedClassResult) -> Self {
        Some(input.0)
    }
}
impl From<LedClassResult> for u32 {
    #[inline]
    fn from(input: LedClassResult) -> Self {
        u32::from(input.0)
    }
}
impl From<LedClassResult> for Option<u32> {
    #[inline]
    fn from(input: LedClassResult) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for LedClassResult {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for LedClassResult {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for LedClassResult  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::KBD_FEEDBACK_CLASS.0.into(), "KBD_FEEDBACK_CLASS", "KbdFeedbackClass"),
            (Self::LED_FEEDBACK_CLASS.0.into(), "LED_FEEDBACK_CLASS", "LedFeedbackClass"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LedClass(u16);
impl LedClass {
    pub const KBD_FEEDBACK_CLASS: Self = Self(0);
    pub const LED_FEEDBACK_CLASS: Self = Self(4);
    pub const DFLT_XI_CLASS: Self = Self(768);
    pub const ALL_XI_CLASSES: Self = Self(1280);
}
impl From<LedClass> for u16 {
    #[inline]
    fn from(input: LedClass) -> Self {
        input.0
    }
}
impl From<LedClass> for Option<u16> {
    #[inline]
    fn from(input: LedClass) -> Self {
        Some(input.0)
    }
}
impl From<LedClass> for u32 {
    #[inline]
    fn from(input: LedClass) -> Self {
        u32::from(input.0)
    }
}
impl From<LedClass> for Option<u32> {
    #[inline]
    fn from(input: LedClass) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for LedClass {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for LedClass {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for LedClass  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::KBD_FEEDBACK_CLASS.0.into(), "KBD_FEEDBACK_CLASS", "KbdFeedbackClass"),
            (Self::LED_FEEDBACK_CLASS.0.into(), "LED_FEEDBACK_CLASS", "LedFeedbackClass"),
            (Self::DFLT_XI_CLASS.0.into(), "DFLT_XI_CLASS", "DfltXIClass"),
            (Self::ALL_XI_CLASSES.0.into(), "ALL_XI_CLASSES", "AllXIClasses"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

pub type LedClassSpec = u16;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BellClassResult(u8);
impl BellClassResult {
    pub const KBD_FEEDBACK_CLASS: Self = Self(0);
    pub const BELL_FEEDBACK_CLASS: Self = Self(5);
}
impl From<BellClassResult> for u8 {
    #[inline]
    fn from(input: BellClassResult) -> Self {
        input.0
    }
}
impl From<BellClassResult> for Option<u8> {
    #[inline]
    fn from(input: BellClassResult) -> Self {
        Some(input.0)
    }
}
impl From<BellClassResult> for u16 {
    #[inline]
    fn from(input: BellClassResult) -> Self {
        u16::from(input.0)
    }
}
impl From<BellClassResult> for Option<u16> {
    #[inline]
    fn from(input: BellClassResult) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<BellClassResult> for u32 {
    #[inline]
    fn from(input: BellClassResult) -> Self {
        u32::from(input.0)
    }
}
impl From<BellClassResult> for Option<u32> {
    #[inline]
    fn from(input: BellClassResult) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for BellClassResult {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for BellClassResult  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::KBD_FEEDBACK_CLASS.0.into(), "KBD_FEEDBACK_CLASS", "KbdFeedbackClass"),
            (Self::BELL_FEEDBACK_CLASS.0.into(), "BELL_FEEDBACK_CLASS", "BellFeedbackClass"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BellClass(u16);
impl BellClass {
    pub const KBD_FEEDBACK_CLASS: Self = Self(0);
    pub const BELL_FEEDBACK_CLASS: Self = Self(5);
    pub const DFLT_XI_CLASS: Self = Self(768);
}
impl From<BellClass> for u16 {
    #[inline]
    fn from(input: BellClass) -> Self {
        input.0
    }
}
impl From<BellClass> for Option<u16> {
    #[inline]
    fn from(input: BellClass) -> Self {
        Some(input.0)
    }
}
impl From<BellClass> for u32 {
    #[inline]
    fn from(input: BellClass) -> Self {
        u32::from(input.0)
    }
}
impl From<BellClass> for Option<u32> {
    #[inline]
    fn from(input: BellClass) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for BellClass {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for BellClass {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for BellClass  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::KBD_FEEDBACK_CLASS.0.into(), "KBD_FEEDBACK_CLASS", "KbdFeedbackClass"),
            (Self::BELL_FEEDBACK_CLASS.0.into(), "BELL_FEEDBACK_CLASS", "BellFeedbackClass"),
            (Self::DFLT_XI_CLASS.0.into(), "DFLT_XI_CLASS", "DfltXIClass"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

pub type BellClassSpec = u16;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ID(u16);
impl ID {
    pub const USE_CORE_KBD: Self = Self(256);
    pub const USE_CORE_PTR: Self = Self(512);
    pub const DFLT_XI_CLASS: Self = Self(768);
    pub const DFLT_XI_ID: Self = Self(1024);
    pub const ALL_XI_CLASS: Self = Self(1280);
    pub const ALL_XI_ID: Self = Self(1536);
    pub const XI_NONE: Self = Self(65280);
}
impl From<ID> for u16 {
    #[inline]
    fn from(input: ID) -> Self {
        input.0
    }
}
impl From<ID> for Option<u16> {
    #[inline]
    fn from(input: ID) -> Self {
        Some(input.0)
    }
}
impl From<ID> for u32 {
    #[inline]
    fn from(input: ID) -> Self {
        u32::from(input.0)
    }
}
impl From<ID> for Option<u32> {
    #[inline]
    fn from(input: ID) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ID {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for ID {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ID  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::USE_CORE_KBD.0.into(), "USE_CORE_KBD", "UseCoreKbd"),
            (Self::USE_CORE_PTR.0.into(), "USE_CORE_PTR", "UseCorePtr"),
            (Self::DFLT_XI_CLASS.0.into(), "DFLT_XI_CLASS", "DfltXIClass"),
            (Self::DFLT_XI_ID.0.into(), "DFLT_XI_ID", "DfltXIId"),
            (Self::ALL_XI_CLASS.0.into(), "ALL_XI_CLASS", "AllXIClass"),
            (Self::ALL_XI_ID.0.into(), "ALL_XI_ID", "AllXIId"),
            (Self::XI_NONE.0.into(), "XI_NONE", "XINone"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

pub type IDSpec = u16;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Group(u8);
impl Group {
    pub const M1: Self = Self(0);
    pub const M2: Self = Self(1);
    pub const M3: Self = Self(2);
    pub const M4: Self = Self(3);
}
impl From<Group> for u8 {
    #[inline]
    fn from(input: Group) -> Self {
        input.0
    }
}
impl From<Group> for Option<u8> {
    #[inline]
    fn from(input: Group) -> Self {
        Some(input.0)
    }
}
impl From<Group> for u16 {
    #[inline]
    fn from(input: Group) -> Self {
        u16::from(input.0)
    }
}
impl From<Group> for Option<u16> {
    #[inline]
    fn from(input: Group) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Group> for u32 {
    #[inline]
    fn from(input: Group) -> Self {
        u32::from(input.0)
    }
}
impl From<Group> for Option<u32> {
    #[inline]
    fn from(input: Group) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Group {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Group  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::M1.0.into(), "M1", "M1"),
            (Self::M2.0.into(), "M2", "M2"),
            (Self::M3.0.into(), "M3", "M3"),
            (Self::M4.0.into(), "M4", "M4"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Groups(u8);
impl Groups {
    pub const ANY: Self = Self(254);
    pub const ALL: Self = Self(255);
}
impl From<Groups> for u8 {
    #[inline]
    fn from(input: Groups) -> Self {
        input.0
    }
}
impl From<Groups> for Option<u8> {
    #[inline]
    fn from(input: Groups) -> Self {
        Some(input.0)
    }
}
impl From<Groups> for u16 {
    #[inline]
    fn from(input: Groups) -> Self {
        u16::from(input.0)
    }
}
impl From<Groups> for Option<u16> {
    #[inline]
    fn from(input: Groups) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Groups> for u32 {
    #[inline]
    fn from(input: Groups) -> Self {
        u32::from(input.0)
    }
}
impl From<Groups> for Option<u32> {
    #[inline]
    fn from(input: Groups) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Groups {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Groups  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::ANY.0.into(), "ANY", "Any"),
            (Self::ALL.0.into(), "ALL", "All"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SetOfGroup(u8);
impl SetOfGroup {
    pub const GROUP1: Self = Self(1 << 0);
    pub const GROUP2: Self = Self(1 << 1);
    pub const GROUP3: Self = Self(1 << 2);
    pub const GROUP4: Self = Self(1 << 3);
}
impl From<SetOfGroup> for u8 {
    #[inline]
    fn from(input: SetOfGroup) -> Self {
        input.0
    }
}
impl From<SetOfGroup> for Option<u8> {
    #[inline]
    fn from(input: SetOfGroup) -> Self {
        Some(input.0)
    }
}
impl From<SetOfGroup> for u16 {
    #[inline]
    fn from(input: SetOfGroup) -> Self {
        u16::from(input.0)
    }
}
impl From<SetOfGroup> for Option<u16> {
    #[inline]
    fn from(input: SetOfGroup) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SetOfGroup> for u32 {
    #[inline]
    fn from(input: SetOfGroup) -> Self {
        u32::from(input.0)
    }
}
impl From<SetOfGroup> for Option<u32> {
    #[inline]
    fn from(input: SetOfGroup) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SetOfGroup {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SetOfGroup  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::GROUP1.0.into(), "GROUP1", "Group1"),
            (Self::GROUP2.0.into(), "GROUP2", "Group2"),
            (Self::GROUP3.0.into(), "GROUP3", "Group3"),
            (Self::GROUP4.0.into(), "GROUP4", "Group4"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(SetOfGroup, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SetOfGroups(u8);
impl SetOfGroups {
    pub const ANY: Self = Self(1 << 7);
}
impl From<SetOfGroups> for u8 {
    #[inline]
    fn from(input: SetOfGroups) -> Self {
        input.0
    }
}
impl From<SetOfGroups> for Option<u8> {
    #[inline]
    fn from(input: SetOfGroups) -> Self {
        Some(input.0)
    }
}
impl From<SetOfGroups> for u16 {
    #[inline]
    fn from(input: SetOfGroups) -> Self {
        u16::from(input.0)
    }
}
impl From<SetOfGroups> for Option<u16> {
    #[inline]
    fn from(input: SetOfGroups) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SetOfGroups> for u32 {
    #[inline]
    fn from(input: SetOfGroups) -> Self {
        u32::from(input.0)
    }
}
impl From<SetOfGroups> for Option<u32> {
    #[inline]
    fn from(input: SetOfGroups) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SetOfGroups {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SetOfGroups  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::ANY.0.into(), "ANY", "Any"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(SetOfGroups, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GroupsWrap(u8);
impl GroupsWrap {
    pub const WRAP_INTO_RANGE: Self = Self(0);
    pub const CLAMP_INTO_RANGE: Self = Self(1 << 6);
    pub const REDIRECT_INTO_RANGE: Self = Self(1 << 7);
}
impl From<GroupsWrap> for u8 {
    #[inline]
    fn from(input: GroupsWrap) -> Self {
        input.0
    }
}
impl From<GroupsWrap> for Option<u8> {
    #[inline]
    fn from(input: GroupsWrap) -> Self {
        Some(input.0)
    }
}
impl From<GroupsWrap> for u16 {
    #[inline]
    fn from(input: GroupsWrap) -> Self {
        u16::from(input.0)
    }
}
impl From<GroupsWrap> for Option<u16> {
    #[inline]
    fn from(input: GroupsWrap) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<GroupsWrap> for u32 {
    #[inline]
    fn from(input: GroupsWrap) -> Self {
        u32::from(input.0)
    }
}
impl From<GroupsWrap> for Option<u32> {
    #[inline]
    fn from(input: GroupsWrap) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for GroupsWrap {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for GroupsWrap  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::WRAP_INTO_RANGE.0.into(), "WRAP_INTO_RANGE", "WrapIntoRange"),
            (Self::CLAMP_INTO_RANGE.0.into(), "CLAMP_INTO_RANGE", "ClampIntoRange"),
            (Self::REDIRECT_INTO_RANGE.0.into(), "REDIRECT_INTO_RANGE", "RedirectIntoRange"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(GroupsWrap, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VModsHigh(u8);
impl VModsHigh {
    pub const M15: Self = Self(1 << 7);
    pub const M14: Self = Self(1 << 6);
    pub const M13: Self = Self(1 << 5);
    pub const M12: Self = Self(1 << 4);
    pub const M11: Self = Self(1 << 3);
    pub const M10: Self = Self(1 << 2);
    pub const M9: Self = Self(1 << 1);
    pub const M8: Self = Self(1 << 0);
}
impl From<VModsHigh> for u8 {
    #[inline]
    fn from(input: VModsHigh) -> Self {
        input.0
    }
}
impl From<VModsHigh> for Option<u8> {
    #[inline]
    fn from(input: VModsHigh) -> Self {
        Some(input.0)
    }
}
impl From<VModsHigh> for u16 {
    #[inline]
    fn from(input: VModsHigh) -> Self {
        u16::from(input.0)
    }
}
impl From<VModsHigh> for Option<u16> {
    #[inline]
    fn from(input: VModsHigh) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<VModsHigh> for u32 {
    #[inline]
    fn from(input: VModsHigh) -> Self {
        u32::from(input.0)
    }
}
impl From<VModsHigh> for Option<u32> {
    #[inline]
    fn from(input: VModsHigh) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for VModsHigh {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for VModsHigh  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::M15.0.into(), "M15", "M15"),
            (Self::M14.0.into(), "M14", "M14"),
            (Self::M13.0.into(), "M13", "M13"),
            (Self::M12.0.into(), "M12", "M12"),
            (Self::M11.0.into(), "M11", "M11"),
            (Self::M10.0.into(), "M10", "M10"),
            (Self::M9.0.into(), "M9", "M9"),
            (Self::M8.0.into(), "M8", "M8"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(VModsHigh, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VModsLow(u8);
impl VModsLow {
    pub const M7: Self = Self(1 << 7);
    pub const M6: Self = Self(1 << 6);
    pub const M5: Self = Self(1 << 5);
    pub const M4: Self = Self(1 << 4);
    pub const M3: Self = Self(1 << 3);
    pub const M2: Self = Self(1 << 2);
    pub const M1: Self = Self(1 << 1);
    pub const M0: Self = Self(1 << 0);
}
impl From<VModsLow> for u8 {
    #[inline]
    fn from(input: VModsLow) -> Self {
        input.0
    }
}
impl From<VModsLow> for Option<u8> {
    #[inline]
    fn from(input: VModsLow) -> Self {
        Some(input.0)
    }
}
impl From<VModsLow> for u16 {
    #[inline]
    fn from(input: VModsLow) -> Self {
        u16::from(input.0)
    }
}
impl From<VModsLow> for Option<u16> {
    #[inline]
    fn from(input: VModsLow) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<VModsLow> for u32 {
    #[inline]
    fn from(input: VModsLow) -> Self {
        u32::from(input.0)
    }
}
impl From<VModsLow> for Option<u32> {
    #[inline]
    fn from(input: VModsLow) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for VModsLow {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for VModsLow  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::M7.0.into(), "M7", "M7"),
            (Self::M6.0.into(), "M6", "M6"),
            (Self::M5.0.into(), "M5", "M5"),
            (Self::M4.0.into(), "M4", "M4"),
            (Self::M3.0.into(), "M3", "M3"),
            (Self::M2.0.into(), "M2", "M2"),
            (Self::M1.0.into(), "M1", "M1"),
            (Self::M0.0.into(), "M0", "M0"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(VModsLow, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VMod(u16);
impl VMod {
    pub const M15: Self = Self(1 << 15);
    pub const M14: Self = Self(1 << 14);
    pub const M13: Self = Self(1 << 13);
    pub const M12: Self = Self(1 << 12);
    pub const M11: Self = Self(1 << 11);
    pub const M10: Self = Self(1 << 10);
    pub const M9: Self = Self(1 << 9);
    pub const M8: Self = Self(1 << 8);
    pub const M7: Self = Self(1 << 7);
    pub const M6: Self = Self(1 << 6);
    pub const M5: Self = Self(1 << 5);
    pub const M4: Self = Self(1 << 4);
    pub const M3: Self = Self(1 << 3);
    pub const M2: Self = Self(1 << 2);
    pub const M1: Self = Self(1 << 1);
    pub const M0: Self = Self(1 << 0);
}
impl From<VMod> for u16 {
    #[inline]
    fn from(input: VMod) -> Self {
        input.0
    }
}
impl From<VMod> for Option<u16> {
    #[inline]
    fn from(input: VMod) -> Self {
        Some(input.0)
    }
}
impl From<VMod> for u32 {
    #[inline]
    fn from(input: VMod) -> Self {
        u32::from(input.0)
    }
}
impl From<VMod> for Option<u32> {
    #[inline]
    fn from(input: VMod) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for VMod {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for VMod {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for VMod  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::M15.0.into(), "M15", "M15"),
            (Self::M14.0.into(), "M14", "M14"),
            (Self::M13.0.into(), "M13", "M13"),
            (Self::M12.0.into(), "M12", "M12"),
            (Self::M11.0.into(), "M11", "M11"),
            (Self::M10.0.into(), "M10", "M10"),
            (Self::M9.0.into(), "M9", "M9"),
            (Self::M8.0.into(), "M8", "M8"),
            (Self::M7.0.into(), "M7", "M7"),
            (Self::M6.0.into(), "M6", "M6"),
            (Self::M5.0.into(), "M5", "M5"),
            (Self::M4.0.into(), "M4", "M4"),
            (Self::M3.0.into(), "M3", "M3"),
            (Self::M2.0.into(), "M2", "M2"),
            (Self::M1.0.into(), "M1", "M1"),
            (Self::M0.0.into(), "M0", "M0"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(VMod, u16);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Explicit(u8);
impl Explicit {
    pub const V_MOD_MAP: Self = Self(1 << 7);
    pub const BEHAVIOR: Self = Self(1 << 6);
    pub const AUTO_REPEAT: Self = Self(1 << 5);
    pub const INTERPRET: Self = Self(1 << 4);
    pub const KEY_TYPE4: Self = Self(1 << 3);
    pub const KEY_TYPE3: Self = Self(1 << 2);
    pub const KEY_TYPE2: Self = Self(1 << 1);
    pub const KEY_TYPE1: Self = Self(1 << 0);
}
impl From<Explicit> for u8 {
    #[inline]
    fn from(input: Explicit) -> Self {
        input.0
    }
}
impl From<Explicit> for Option<u8> {
    #[inline]
    fn from(input: Explicit) -> Self {
        Some(input.0)
    }
}
impl From<Explicit> for u16 {
    #[inline]
    fn from(input: Explicit) -> Self {
        u16::from(input.0)
    }
}
impl From<Explicit> for Option<u16> {
    #[inline]
    fn from(input: Explicit) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Explicit> for u32 {
    #[inline]
    fn from(input: Explicit) -> Self {
        u32::from(input.0)
    }
}
impl From<Explicit> for Option<u32> {
    #[inline]
    fn from(input: Explicit) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Explicit {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Explicit  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::V_MOD_MAP.0.into(), "V_MOD_MAP", "VModMap"),
            (Self::BEHAVIOR.0.into(), "BEHAVIOR", "Behavior"),
            (Self::AUTO_REPEAT.0.into(), "AUTO_REPEAT", "AutoRepeat"),
            (Self::INTERPRET.0.into(), "INTERPRET", "Interpret"),
            (Self::KEY_TYPE4.0.into(), "KEY_TYPE4", "KeyType4"),
            (Self::KEY_TYPE3.0.into(), "KEY_TYPE3", "KeyType3"),
            (Self::KEY_TYPE2.0.into(), "KEY_TYPE2", "KeyType2"),
            (Self::KEY_TYPE1.0.into(), "KEY_TYPE1", "KeyType1"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(Explicit, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SymInterpretMatch(u8);
impl SymInterpretMatch {
    pub const NONE_OF: Self = Self(0);
    pub const ANY_OF_OR_NONE: Self = Self(1);
    pub const ANY_OF: Self = Self(2);
    pub const ALL_OF: Self = Self(3);
    pub const EXACTLY: Self = Self(4);
}
impl From<SymInterpretMatch> for u8 {
    #[inline]
    fn from(input: SymInterpretMatch) -> Self {
        input.0
    }
}
impl From<SymInterpretMatch> for Option<u8> {
    #[inline]
    fn from(input: SymInterpretMatch) -> Self {
        Some(input.0)
    }
}
impl From<SymInterpretMatch> for u16 {
    #[inline]
    fn from(input: SymInterpretMatch) -> Self {
        u16::from(input.0)
    }
}
impl From<SymInterpretMatch> for Option<u16> {
    #[inline]
    fn from(input: SymInterpretMatch) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SymInterpretMatch> for u32 {
    #[inline]
    fn from(input: SymInterpretMatch) -> Self {
        u32::from(input.0)
    }
}
impl From<SymInterpretMatch> for Option<u32> {
    #[inline]
    fn from(input: SymInterpretMatch) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SymInterpretMatch {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SymInterpretMatch  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::NONE_OF.0.into(), "NONE_OF", "NoneOf"),
            (Self::ANY_OF_OR_NONE.0.into(), "ANY_OF_OR_NONE", "AnyOfOrNone"),
            (Self::ANY_OF.0.into(), "ANY_OF", "AnyOf"),
            (Self::ALL_OF.0.into(), "ALL_OF", "AllOf"),
            (Self::EXACTLY.0.into(), "EXACTLY", "Exactly"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SymInterpMatch(u8);
impl SymInterpMatch {
    pub const LEVEL_ONE_ONLY: Self = Self(1 << 7);
    pub const OP_MASK: Self = Self(127);
}
impl From<SymInterpMatch> for u8 {
    #[inline]
    fn from(input: SymInterpMatch) -> Self {
        input.0
    }
}
impl From<SymInterpMatch> for Option<u8> {
    #[inline]
    fn from(input: SymInterpMatch) -> Self {
        Some(input.0)
    }
}
impl From<SymInterpMatch> for u16 {
    #[inline]
    fn from(input: SymInterpMatch) -> Self {
        u16::from(input.0)
    }
}
impl From<SymInterpMatch> for Option<u16> {
    #[inline]
    fn from(input: SymInterpMatch) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SymInterpMatch> for u32 {
    #[inline]
    fn from(input: SymInterpMatch) -> Self {
        u32::from(input.0)
    }
}
impl From<SymInterpMatch> for Option<u32> {
    #[inline]
    fn from(input: SymInterpMatch) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SymInterpMatch {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SymInterpMatch  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::LEVEL_ONE_ONLY.0.into(), "LEVEL_ONE_ONLY", "LevelOneOnly"),
            (Self::OP_MASK.0.into(), "OP_MASK", "OpMask"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IMFlag(u8);
impl IMFlag {
    pub const NO_EXPLICIT: Self = Self(1 << 7);
    pub const NO_AUTOMATIC: Self = Self(1 << 6);
    pub const LED_DRIVES_KB: Self = Self(1 << 5);
}
impl From<IMFlag> for u8 {
    #[inline]
    fn from(input: IMFlag) -> Self {
        input.0
    }
}
impl From<IMFlag> for Option<u8> {
    #[inline]
    fn from(input: IMFlag) -> Self {
        Some(input.0)
    }
}
impl From<IMFlag> for u16 {
    #[inline]
    fn from(input: IMFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<IMFlag> for Option<u16> {
    #[inline]
    fn from(input: IMFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<IMFlag> for u32 {
    #[inline]
    fn from(input: IMFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<IMFlag> for Option<u32> {
    #[inline]
    fn from(input: IMFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for IMFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for IMFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::NO_EXPLICIT.0.into(), "NO_EXPLICIT", "NoExplicit"),
            (Self::NO_AUTOMATIC.0.into(), "NO_AUTOMATIC", "NoAutomatic"),
            (Self::LED_DRIVES_KB.0.into(), "LED_DRIVES_KB", "LEDDrivesKB"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(IMFlag, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IMModsWhich(u8);
impl IMModsWhich {
    pub const USE_COMPAT: Self = Self(1 << 4);
    pub const USE_EFFECTIVE: Self = Self(1 << 3);
    pub const USE_LOCKED: Self = Self(1 << 2);
    pub const USE_LATCHED: Self = Self(1 << 1);
    pub const USE_BASE: Self = Self(1 << 0);
}
impl From<IMModsWhich> for u8 {
    #[inline]
    fn from(input: IMModsWhich) -> Self {
        input.0
    }
}
impl From<IMModsWhich> for Option<u8> {
    #[inline]
    fn from(input: IMModsWhich) -> Self {
        Some(input.0)
    }
}
impl From<IMModsWhich> for u16 {
    #[inline]
    fn from(input: IMModsWhich) -> Self {
        u16::from(input.0)
    }
}
impl From<IMModsWhich> for Option<u16> {
    #[inline]
    fn from(input: IMModsWhich) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<IMModsWhich> for u32 {
    #[inline]
    fn from(input: IMModsWhich) -> Self {
        u32::from(input.0)
    }
}
impl From<IMModsWhich> for Option<u32> {
    #[inline]
    fn from(input: IMModsWhich) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for IMModsWhich {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for IMModsWhich  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::USE_COMPAT.0.into(), "USE_COMPAT", "UseCompat"),
            (Self::USE_EFFECTIVE.0.into(), "USE_EFFECTIVE", "UseEffective"),
            (Self::USE_LOCKED.0.into(), "USE_LOCKED", "UseLocked"),
            (Self::USE_LATCHED.0.into(), "USE_LATCHED", "UseLatched"),
            (Self::USE_BASE.0.into(), "USE_BASE", "UseBase"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(IMModsWhich, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IMGroupsWhich(u8);
impl IMGroupsWhich {
    pub const USE_COMPAT: Self = Self(1 << 4);
    pub const USE_EFFECTIVE: Self = Self(1 << 3);
    pub const USE_LOCKED: Self = Self(1 << 2);
    pub const USE_LATCHED: Self = Self(1 << 1);
    pub const USE_BASE: Self = Self(1 << 0);
}
impl From<IMGroupsWhich> for u8 {
    #[inline]
    fn from(input: IMGroupsWhich) -> Self {
        input.0
    }
}
impl From<IMGroupsWhich> for Option<u8> {
    #[inline]
    fn from(input: IMGroupsWhich) -> Self {
        Some(input.0)
    }
}
impl From<IMGroupsWhich> for u16 {
    #[inline]
    fn from(input: IMGroupsWhich) -> Self {
        u16::from(input.0)
    }
}
impl From<IMGroupsWhich> for Option<u16> {
    #[inline]
    fn from(input: IMGroupsWhich) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<IMGroupsWhich> for u32 {
    #[inline]
    fn from(input: IMGroupsWhich) -> Self {
        u32::from(input.0)
    }
}
impl From<IMGroupsWhich> for Option<u32> {
    #[inline]
    fn from(input: IMGroupsWhich) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for IMGroupsWhich {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for IMGroupsWhich  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::USE_COMPAT.0.into(), "USE_COMPAT", "UseCompat"),
            (Self::USE_EFFECTIVE.0.into(), "USE_EFFECTIVE", "UseEffective"),
            (Self::USE_LOCKED.0.into(), "USE_LOCKED", "UseLocked"),
            (Self::USE_LATCHED.0.into(), "USE_LATCHED", "UseLatched"),
            (Self::USE_BASE.0.into(), "USE_BASE", "UseBase"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let flags = flags.into();
        let which_groups = which_groups.into();
        let groups = groups.into();
        let which_mods = which_mods.into();
        let result = IndicatorMap { flags, which_groups, groups, which_mods, mods, real_mods, vmods, ctrls };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CMDetail(u8);
impl CMDetail {
    pub const SYM_INTERP: Self = Self(1 << 0);
    pub const GROUP_COMPAT: Self = Self(1 << 1);
}
impl From<CMDetail> for u8 {
    #[inline]
    fn from(input: CMDetail) -> Self {
        input.0
    }
}
impl From<CMDetail> for Option<u8> {
    #[inline]
    fn from(input: CMDetail) -> Self {
        Some(input.0)
    }
}
impl From<CMDetail> for u16 {
    #[inline]
    fn from(input: CMDetail) -> Self {
        u16::from(input.0)
    }
}
impl From<CMDetail> for Option<u16> {
    #[inline]
    fn from(input: CMDetail) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<CMDetail> for u32 {
    #[inline]
    fn from(input: CMDetail) -> Self {
        u32::from(input.0)
    }
}
impl From<CMDetail> for Option<u32> {
    #[inline]
    fn from(input: CMDetail) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for CMDetail {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for CMDetail  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::SYM_INTERP.0.into(), "SYM_INTERP", "SymInterp"),
            (Self::GROUP_COMPAT.0.into(), "GROUP_COMPAT", "GroupCompat"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(CMDetail, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NameDetail(u16);
impl NameDetail {
    pub const KEYCODES: Self = Self(1 << 0);
    pub const GEOMETRY: Self = Self(1 << 1);
    pub const SYMBOLS: Self = Self(1 << 2);
    pub const PHYS_SYMBOLS: Self = Self(1 << 3);
    pub const TYPES: Self = Self(1 << 4);
    pub const COMPAT: Self = Self(1 << 5);
    pub const KEY_TYPE_NAMES: Self = Self(1 << 6);
    pub const KT_LEVEL_NAMES: Self = Self(1 << 7);
    pub const INDICATOR_NAMES: Self = Self(1 << 8);
    pub const KEY_NAMES: Self = Self(1 << 9);
    pub const KEY_ALIASES: Self = Self(1 << 10);
    pub const VIRTUAL_MOD_NAMES: Self = Self(1 << 11);
    pub const GROUP_NAMES: Self = Self(1 << 12);
    pub const RG_NAMES: Self = Self(1 << 13);
}
impl From<NameDetail> for u16 {
    #[inline]
    fn from(input: NameDetail) -> Self {
        input.0
    }
}
impl From<NameDetail> for Option<u16> {
    #[inline]
    fn from(input: NameDetail) -> Self {
        Some(input.0)
    }
}
impl From<NameDetail> for u32 {
    #[inline]
    fn from(input: NameDetail) -> Self {
        u32::from(input.0)
    }
}
impl From<NameDetail> for Option<u32> {
    #[inline]
    fn from(input: NameDetail) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for NameDetail {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for NameDetail {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for NameDetail  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::KEYCODES.0.into(), "KEYCODES", "Keycodes"),
            (Self::GEOMETRY.0.into(), "GEOMETRY", "Geometry"),
            (Self::SYMBOLS.0.into(), "SYMBOLS", "Symbols"),
            (Self::PHYS_SYMBOLS.0.into(), "PHYS_SYMBOLS", "PhysSymbols"),
            (Self::TYPES.0.into(), "TYPES", "Types"),
            (Self::COMPAT.0.into(), "COMPAT", "Compat"),
            (Self::KEY_TYPE_NAMES.0.into(), "KEY_TYPE_NAMES", "KeyTypeNames"),
            (Self::KT_LEVEL_NAMES.0.into(), "KT_LEVEL_NAMES", "KTLevelNames"),
            (Self::INDICATOR_NAMES.0.into(), "INDICATOR_NAMES", "IndicatorNames"),
            (Self::KEY_NAMES.0.into(), "KEY_NAMES", "KeyNames"),
            (Self::KEY_ALIASES.0.into(), "KEY_ALIASES", "KeyAliases"),
            (Self::VIRTUAL_MOD_NAMES.0.into(), "VIRTUAL_MOD_NAMES", "VirtualModNames"),
            (Self::GROUP_NAMES.0.into(), "GROUP_NAMES", "GroupNames"),
            (Self::RG_NAMES.0.into(), "RG_NAMES", "RGNames"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(NameDetail, u16);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GBNDetail(u8);
impl GBNDetail {
    pub const TYPES: Self = Self(1 << 0);
    pub const COMPAT_MAP: Self = Self(1 << 1);
    pub const CLIENT_SYMBOLS: Self = Self(1 << 2);
    pub const SERVER_SYMBOLS: Self = Self(1 << 3);
    pub const INDICATOR_MAPS: Self = Self(1 << 4);
    pub const KEY_NAMES: Self = Self(1 << 5);
    pub const GEOMETRY: Self = Self(1 << 6);
    pub const OTHER_NAMES: Self = Self(1 << 7);
}
impl From<GBNDetail> for u8 {
    #[inline]
    fn from(input: GBNDetail) -> Self {
        input.0
    }
}
impl From<GBNDetail> for Option<u8> {
    #[inline]
    fn from(input: GBNDetail) -> Self {
        Some(input.0)
    }
}
impl From<GBNDetail> for u16 {
    #[inline]
    fn from(input: GBNDetail) -> Self {
        u16::from(input.0)
    }
}
impl From<GBNDetail> for Option<u16> {
    #[inline]
    fn from(input: GBNDetail) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<GBNDetail> for u32 {
    #[inline]
    fn from(input: GBNDetail) -> Self {
        u32::from(input.0)
    }
}
impl From<GBNDetail> for Option<u32> {
    #[inline]
    fn from(input: GBNDetail) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for GBNDetail {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for GBNDetail  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::TYPES.0.into(), "TYPES", "Types"),
            (Self::COMPAT_MAP.0.into(), "COMPAT_MAP", "CompatMap"),
            (Self::CLIENT_SYMBOLS.0.into(), "CLIENT_SYMBOLS", "ClientSymbols"),
            (Self::SERVER_SYMBOLS.0.into(), "SERVER_SYMBOLS", "ServerSymbols"),
            (Self::INDICATOR_MAPS.0.into(), "INDICATOR_MAPS", "IndicatorMaps"),
            (Self::KEY_NAMES.0.into(), "KEY_NAMES", "KeyNames"),
            (Self::GEOMETRY.0.into(), "GEOMETRY", "Geometry"),
            (Self::OTHER_NAMES.0.into(), "OTHER_NAMES", "OtherNames"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(GBNDetail, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct XIFeature(u8);
impl XIFeature {
    pub const KEYBOARDS: Self = Self(1 << 0);
    pub const BUTTON_ACTIONS: Self = Self(1 << 1);
    pub const INDICATOR_NAMES: Self = Self(1 << 2);
    pub const INDICATOR_MAPS: Self = Self(1 << 3);
    pub const INDICATOR_STATE: Self = Self(1 << 4);
}
impl From<XIFeature> for u8 {
    #[inline]
    fn from(input: XIFeature) -> Self {
        input.0
    }
}
impl From<XIFeature> for Option<u8> {
    #[inline]
    fn from(input: XIFeature) -> Self {
        Some(input.0)
    }
}
impl From<XIFeature> for u16 {
    #[inline]
    fn from(input: XIFeature) -> Self {
        u16::from(input.0)
    }
}
impl From<XIFeature> for Option<u16> {
    #[inline]
    fn from(input: XIFeature) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<XIFeature> for u32 {
    #[inline]
    fn from(input: XIFeature) -> Self {
        u32::from(input.0)
    }
}
impl From<XIFeature> for Option<u32> {
    #[inline]
    fn from(input: XIFeature) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for XIFeature {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for XIFeature  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::KEYBOARDS.0.into(), "KEYBOARDS", "Keyboards"),
            (Self::BUTTON_ACTIONS.0.into(), "BUTTON_ACTIONS", "ButtonActions"),
            (Self::INDICATOR_NAMES.0.into(), "INDICATOR_NAMES", "IndicatorNames"),
            (Self::INDICATOR_MAPS.0.into(), "INDICATOR_MAPS", "IndicatorMaps"),
            (Self::INDICATOR_STATE.0.into(), "INDICATOR_STATE", "IndicatorState"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(XIFeature, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PerClientFlag(u8);
impl PerClientFlag {
    pub const DETECTABLE_AUTO_REPEAT: Self = Self(1 << 0);
    pub const GRABS_USE_XKB_STATE: Self = Self(1 << 1);
    pub const AUTO_RESET_CONTROLS: Self = Self(1 << 2);
    pub const LOOKUP_STATE_WHEN_GRABBED: Self = Self(1 << 3);
    pub const SEND_EVENT_USES_XKB_STATE: Self = Self(1 << 4);
}
impl From<PerClientFlag> for u8 {
    #[inline]
    fn from(input: PerClientFlag) -> Self {
        input.0
    }
}
impl From<PerClientFlag> for Option<u8> {
    #[inline]
    fn from(input: PerClientFlag) -> Self {
        Some(input.0)
    }
}
impl From<PerClientFlag> for u16 {
    #[inline]
    fn from(input: PerClientFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<PerClientFlag> for Option<u16> {
    #[inline]
    fn from(input: PerClientFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<PerClientFlag> for u32 {
    #[inline]
    fn from(input: PerClientFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<PerClientFlag> for Option<u32> {
    #[inline]
    fn from(input: PerClientFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for PerClientFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for PerClientFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::DETECTABLE_AUTO_REPEAT.0.into(), "DETECTABLE_AUTO_REPEAT", "DetectableAutoRepeat"),
            (Self::GRABS_USE_XKB_STATE.0.into(), "GRABS_USE_XKB_STATE", "GrabsUseXKBState"),
            (Self::AUTO_RESET_CONTROLS.0.into(), "AUTO_RESET_CONTROLS", "AutoResetControls"),
            (Self::LOOKUP_STATE_WHEN_GRABBED.0.into(), "LOOKUP_STATE_WHEN_GRABBED", "LookupStateWhenGrabbed"),
            (Self::SEND_EVENT_USES_XKB_STATE.0.into(), "SEND_EVENT_USES_XKB_STATE", "SendEventUsesXKBState"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, length.try_to_usize()?)?;
        let string = string.to_vec();
        let (alignment_pad, remaining) = crate::x11_utils::parse_u8_list(remaining, (u32::from(length).checked_add(5u32).ok_or(ParseError::InvalidExpression)? & (!3u32)).checked_sub(u32::from(length).checked_add(2u32).ok_or(ParseError::InvalidExpression)?).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let alignment_pad = alignment_pad.to_vec();
        let result = CountedString16 { string, alignment_pad };
        Ok((result, remaining))
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let result = KTMapEntry { active, mods_mask, level, mods_mods, mods_vmods };
        Ok((result, remaining))
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (map, remaining) = crate::x11_utils::parse_list::<KTMapEntry>(remaining, n_map_entries.try_to_usize()?)?;
        let (preserve, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, u32::from(has_preserve).checked_mul(u32::from(n_map_entries)).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let result = KeyType { mods_mask, mods_mods, mods_vmods, num_levels, has_preserve, map, preserve };
        Ok((result, remaining))
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
        let (syms, remaining) = crate::x11_utils::parse_list::<xproto::Keysym>(remaining, n_syms.try_to_usize()?)?;
        let result = KeySymMap { kt_index, group_info, width, syms };
        Ok((result, remaining))
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let result = DefaultBehavior { type_ };
        Ok((result, remaining))
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
            .ok_or(ParseError::InsufficientData)?
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BehaviorType(u8);
impl BehaviorType {
    pub const DEFAULT: Self = Self(0);
    pub const LOCK: Self = Self(1);
    pub const RADIO_GROUP: Self = Self(2);
    pub const OVERLAY1: Self = Self(3);
    pub const OVERLAY2: Self = Self(4);
    pub const PERMAMENT_LOCK: Self = Self(129);
    pub const PERMAMENT_RADIO_GROUP: Self = Self(130);
    pub const PERMAMENT_OVERLAY1: Self = Self(131);
    pub const PERMAMENT_OVERLAY2: Self = Self(132);
}
impl From<BehaviorType> for u8 {
    #[inline]
    fn from(input: BehaviorType) -> Self {
        input.0
    }
}
impl From<BehaviorType> for Option<u8> {
    #[inline]
    fn from(input: BehaviorType) -> Self {
        Some(input.0)
    }
}
impl From<BehaviorType> for u16 {
    #[inline]
    fn from(input: BehaviorType) -> Self {
        u16::from(input.0)
    }
}
impl From<BehaviorType> for Option<u16> {
    #[inline]
    fn from(input: BehaviorType) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<BehaviorType> for u32 {
    #[inline]
    fn from(input: BehaviorType) -> Self {
        u32::from(input.0)
    }
}
impl From<BehaviorType> for Option<u32> {
    #[inline]
    fn from(input: BehaviorType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for BehaviorType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for BehaviorType  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::DEFAULT.0.into(), "DEFAULT", "Default"),
            (Self::LOCK.0.into(), "LOCK", "Lock"),
            (Self::RADIO_GROUP.0.into(), "RADIO_GROUP", "RadioGroup"),
            (Self::OVERLAY1.0.into(), "OVERLAY1", "Overlay1"),
            (Self::OVERLAY2.0.into(), "OVERLAY2", "Overlay2"),
            (Self::PERMAMENT_LOCK.0.into(), "PERMAMENT_LOCK", "PermamentLock"),
            (Self::PERMAMENT_RADIO_GROUP.0.into(), "PERMAMENT_RADIO_GROUP", "PermamentRadioGroup"),
            (Self::PERMAMENT_OVERLAY1.0.into(), "PERMAMENT_OVERLAY1", "PermamentOverlay1"),
            (Self::PERMAMENT_OVERLAY2.0.into(), "PERMAMENT_OVERLAY2", "PermamentOverlay2"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let result = SetBehavior { keycode, behavior };
        Ok((result, remaining))
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (vmods, remaining) = u16::try_parse(remaining)?;
        let result = KeyVModMap { keycode, vmods };
        Ok((result, remaining))
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (entries, remaining) = crate::x11_utils::parse_list::<KTSetMapEntry>(remaining, n_map_entries.try_to_usize()?)?;
        let (preserve_entries, remaining) = crate::x11_utils::parse_list::<KTSetMapEntry>(remaining, u32::from(preserve).checked_mul(u32::from(n_map_entries)).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let result = SetKeyType { mask, real_mods, virtual_mods, num_levels, preserve, entries, preserve_entries };
        Ok((result, remaining))
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (points, remaining) = crate::x11_utils::parse_list::<xproto::Point>(remaining, n_points.try_to_usize()?)?;
        let result = Outline { corner_radius, points };
        Ok((result, remaining))
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (outlines, remaining) = crate::x11_utils::parse_list::<Outline>(remaining, n_outlines.try_to_usize()?)?;
        let result = Shape { name, primary_ndx, approx_ndx, outlines };
        Ok((result, remaining))
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (keys, remaining) = crate::x11_utils::parse_list::<OverlayKey>(remaining, n_keys.try_to_usize()?)?;
        let result = OverlayRow { row_under, keys };
        Ok((result, remaining))
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (rows, remaining) = crate::x11_utils::parse_list::<OverlayRow>(remaining, n_rows.try_to_usize()?)?;
        let result = Overlay { name, rows };
        Ok((result, remaining))
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (keys, remaining) = crate::x11_utils::parse_list::<Key>(remaining, n_keys.try_to_usize()?)?;
        let result = Row { top, left, vertical, keys };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DoodadType(u8);
impl DoodadType {
    pub const OUTLINE: Self = Self(1);
    pub const SOLID: Self = Self(2);
    pub const TEXT: Self = Self(3);
    pub const INDICATOR: Self = Self(4);
    pub const LOGO: Self = Self(5);
}
impl From<DoodadType> for u8 {
    #[inline]
    fn from(input: DoodadType) -> Self {
        input.0
    }
}
impl From<DoodadType> for Option<u8> {
    #[inline]
    fn from(input: DoodadType) -> Self {
        Some(input.0)
    }
}
impl From<DoodadType> for u16 {
    #[inline]
    fn from(input: DoodadType) -> Self {
        u16::from(input.0)
    }
}
impl From<DoodadType> for Option<u16> {
    #[inline]
    fn from(input: DoodadType) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<DoodadType> for u32 {
    #[inline]
    fn from(input: DoodadType) -> Self {
        u32::from(input.0)
    }
}
impl From<DoodadType> for Option<u32> {
    #[inline]
    fn from(input: DoodadType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for DoodadType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for DoodadType  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::OUTLINE.0.into(), "OUTLINE", "Outline"),
            (Self::SOLID.0.into(), "SOLID", "Solid"),
            (Self::TEXT.0.into(), "TEXT", "Text"),
            (Self::INDICATOR.0.into(), "INDICATOR", "Indicator"),
            (Self::LOGO.0.into(), "LOGO", "Logo"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
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
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, length.try_to_usize()?)?;
        let string = string.to_vec();
        // Align offset to multiple of 2
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (2 - (offset % 2)) % 2;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let result = Listing { flags, string };
        Ok((result, remaining))
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
        let (names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, names_present.count_ones().try_to_usize()?)?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, maps_present.count_ones().try_to_usize()?)?;
        let led_class = led_class.into();
        let result = DeviceLedInfo { led_class, led_id, names_present, maps_present, phys_indicators, state, names, maps };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error(u8);
impl Error {
    pub const BAD_DEVICE: Self = Self(255);
    pub const BAD_CLASS: Self = Self(254);
    pub const BAD_ID: Self = Self(253);
}
impl From<Error> for u8 {
    #[inline]
    fn from(input: Error) -> Self {
        input.0
    }
}
impl From<Error> for Option<u8> {
    #[inline]
    fn from(input: Error) -> Self {
        Some(input.0)
    }
}
impl From<Error> for u16 {
    #[inline]
    fn from(input: Error) -> Self {
        u16::from(input.0)
    }
}
impl From<Error> for Option<u16> {
    #[inline]
    fn from(input: Error) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Error> for u32 {
    #[inline]
    fn from(input: Error) -> Self {
        u32::from(input.0)
    }
}
impl From<Error> for Option<u32> {
    #[inline]
    fn from(input: Error) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Error {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Error  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::BAD_DEVICE.0.into(), "BAD_DEVICE", "BadDevice"),
            (Self::BAD_CLASS.0.into(), "BAD_CLASS", "BadClass"),
            (Self::BAD_ID.0.into(), "BAD_ID", "BadId"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

/// Opcode for the Keyboard error
pub const KEYBOARD_ERROR: u8 = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SA(u8);
impl SA {
    pub const CLEAR_LOCKS: Self = Self(1 << 0);
    pub const LATCH_TO_LOCK: Self = Self(1 << 1);
    pub const USE_MOD_MAP_MODS: Self = Self(1 << 2);
    pub const GROUP_ABSOLUTE: Self = Self(1 << 2);
}
impl From<SA> for u8 {
    #[inline]
    fn from(input: SA) -> Self {
        input.0
    }
}
impl From<SA> for Option<u8> {
    #[inline]
    fn from(input: SA) -> Self {
        Some(input.0)
    }
}
impl From<SA> for u16 {
    #[inline]
    fn from(input: SA) -> Self {
        u16::from(input.0)
    }
}
impl From<SA> for Option<u16> {
    #[inline]
    fn from(input: SA) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SA> for u32 {
    #[inline]
    fn from(input: SA) -> Self {
        u32::from(input.0)
    }
}
impl From<SA> for Option<u32> {
    #[inline]
    fn from(input: SA) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SA {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SA  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::CLEAR_LOCKS.0.into(), "CLEAR_LOCKS", "ClearLocks"),
            (Self::LATCH_TO_LOCK.0.into(), "LATCH_TO_LOCK", "LatchToLock"),
            (Self::USE_MOD_MAP_MODS.0.into(), "USE_MOD_MAP_MODS", "UseModMapMods"),
            (Self::GROUP_ABSOLUTE.0.into(), "GROUP_ABSOLUTE", "GroupAbsolute"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(SA, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SAType(u8);
impl SAType {
    pub const NO_ACTION: Self = Self(0);
    pub const SET_MODS: Self = Self(1);
    pub const LATCH_MODS: Self = Self(2);
    pub const LOCK_MODS: Self = Self(3);
    pub const SET_GROUP: Self = Self(4);
    pub const LATCH_GROUP: Self = Self(5);
    pub const LOCK_GROUP: Self = Self(6);
    pub const MOVE_PTR: Self = Self(7);
    pub const PTR_BTN: Self = Self(8);
    pub const LOCK_PTR_BTN: Self = Self(9);
    pub const SET_PTR_DFLT: Self = Self(10);
    pub const ISO_LOCK: Self = Self(11);
    pub const TERMINATE: Self = Self(12);
    pub const SWITCH_SCREEN: Self = Self(13);
    pub const SET_CONTROLS: Self = Self(14);
    pub const LOCK_CONTROLS: Self = Self(15);
    pub const ACTION_MESSAGE: Self = Self(16);
    pub const REDIRECT_KEY: Self = Self(17);
    pub const DEVICE_BTN: Self = Self(18);
    pub const LOCK_DEVICE_BTN: Self = Self(19);
    pub const DEVICE_VALUATOR: Self = Self(20);
}
impl From<SAType> for u8 {
    #[inline]
    fn from(input: SAType) -> Self {
        input.0
    }
}
impl From<SAType> for Option<u8> {
    #[inline]
    fn from(input: SAType) -> Self {
        Some(input.0)
    }
}
impl From<SAType> for u16 {
    #[inline]
    fn from(input: SAType) -> Self {
        u16::from(input.0)
    }
}
impl From<SAType> for Option<u16> {
    #[inline]
    fn from(input: SAType) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SAType> for u32 {
    #[inline]
    fn from(input: SAType) -> Self {
        u32::from(input.0)
    }
}
impl From<SAType> for Option<u32> {
    #[inline]
    fn from(input: SAType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SAType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SAType  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::NO_ACTION.0.into(), "NO_ACTION", "NoAction"),
            (Self::SET_MODS.0.into(), "SET_MODS", "SetMods"),
            (Self::LATCH_MODS.0.into(), "LATCH_MODS", "LatchMods"),
            (Self::LOCK_MODS.0.into(), "LOCK_MODS", "LockMods"),
            (Self::SET_GROUP.0.into(), "SET_GROUP", "SetGroup"),
            (Self::LATCH_GROUP.0.into(), "LATCH_GROUP", "LatchGroup"),
            (Self::LOCK_GROUP.0.into(), "LOCK_GROUP", "LockGroup"),
            (Self::MOVE_PTR.0.into(), "MOVE_PTR", "MovePtr"),
            (Self::PTR_BTN.0.into(), "PTR_BTN", "PtrBtn"),
            (Self::LOCK_PTR_BTN.0.into(), "LOCK_PTR_BTN", "LockPtrBtn"),
            (Self::SET_PTR_DFLT.0.into(), "SET_PTR_DFLT", "SetPtrDflt"),
            (Self::ISO_LOCK.0.into(), "ISO_LOCK", "ISOLock"),
            (Self::TERMINATE.0.into(), "TERMINATE", "Terminate"),
            (Self::SWITCH_SCREEN.0.into(), "SWITCH_SCREEN", "SwitchScreen"),
            (Self::SET_CONTROLS.0.into(), "SET_CONTROLS", "SetControls"),
            (Self::LOCK_CONTROLS.0.into(), "LOCK_CONTROLS", "LockControls"),
            (Self::ACTION_MESSAGE.0.into(), "ACTION_MESSAGE", "ActionMessage"),
            (Self::REDIRECT_KEY.0.into(), "REDIRECT_KEY", "RedirectKey"),
            (Self::DEVICE_BTN.0.into(), "DEVICE_BTN", "DeviceBtn"),
            (Self::LOCK_DEVICE_BTN.0.into(), "LOCK_DEVICE_BTN", "LockDeviceBtn"),
            (Self::DEVICE_VALUATOR.0.into(), "DEVICE_VALUATOR", "DeviceValuator"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SANoAction {
    pub type_: SAType,
}
impl TryParse for SANoAction {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(7..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SANoAction { type_ };
        Ok((result, remaining))
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SASetMods { type_, flags, mask, real_mods, vmods_high, vmods_low };
        Ok((result, remaining))
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
        let remaining = remaining.get(5..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SASetGroup { type_, flags, group };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SAMovePtrFlag(u8);
impl SAMovePtrFlag {
    pub const NO_ACCELERATION: Self = Self(1 << 0);
    pub const MOVE_ABSOLUTE_X: Self = Self(1 << 1);
    pub const MOVE_ABSOLUTE_Y: Self = Self(1 << 2);
}
impl From<SAMovePtrFlag> for u8 {
    #[inline]
    fn from(input: SAMovePtrFlag) -> Self {
        input.0
    }
}
impl From<SAMovePtrFlag> for Option<u8> {
    #[inline]
    fn from(input: SAMovePtrFlag) -> Self {
        Some(input.0)
    }
}
impl From<SAMovePtrFlag> for u16 {
    #[inline]
    fn from(input: SAMovePtrFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<SAMovePtrFlag> for Option<u16> {
    #[inline]
    fn from(input: SAMovePtrFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SAMovePtrFlag> for u32 {
    #[inline]
    fn from(input: SAMovePtrFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<SAMovePtrFlag> for Option<u32> {
    #[inline]
    fn from(input: SAMovePtrFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SAMovePtrFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SAMovePtrFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::NO_ACCELERATION.0.into(), "NO_ACCELERATION", "NoAcceleration"),
            (Self::MOVE_ABSOLUTE_X.0.into(), "MOVE_ABSOLUTE_X", "MoveAbsoluteX"),
            (Self::MOVE_ABSOLUTE_Y.0.into(), "MOVE_ABSOLUTE_Y", "MoveAbsoluteY"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SAMovePtr { type_, flags, x_high, x_low, y_high, y_low };
        Ok((result, remaining))
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
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SAPtrBtn { type_, flags, count, button };
        Ok((result, remaining))
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (button, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SALockPtrBtn { type_, flags, button };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SASetPtrDfltFlag(u8);
impl SASetPtrDfltFlag {
    pub const DFLT_BTN_ABSOLUTE: Self = Self(1 << 2);
    pub const AFFECT_DFLT_BUTTON: Self = Self(1 << 0);
}
impl From<SASetPtrDfltFlag> for u8 {
    #[inline]
    fn from(input: SASetPtrDfltFlag) -> Self {
        input.0
    }
}
impl From<SASetPtrDfltFlag> for Option<u8> {
    #[inline]
    fn from(input: SASetPtrDfltFlag) -> Self {
        Some(input.0)
    }
}
impl From<SASetPtrDfltFlag> for u16 {
    #[inline]
    fn from(input: SASetPtrDfltFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<SASetPtrDfltFlag> for Option<u16> {
    #[inline]
    fn from(input: SASetPtrDfltFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SASetPtrDfltFlag> for u32 {
    #[inline]
    fn from(input: SASetPtrDfltFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<SASetPtrDfltFlag> for Option<u32> {
    #[inline]
    fn from(input: SASetPtrDfltFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SASetPtrDfltFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SASetPtrDfltFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::DFLT_BTN_ABSOLUTE.0.into(), "DFLT_BTN_ABSOLUTE", "DfltBtnAbsolute"),
            (Self::AFFECT_DFLT_BUTTON.0.into(), "AFFECT_DFLT_BUTTON", "AffectDfltButton"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SASetPtrDflt { type_, flags, affect, value };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SAIsoLockFlag(u8);
impl SAIsoLockFlag {
    pub const NO_LOCK: Self = Self(1 << 0);
    pub const NO_UNLOCK: Self = Self(1 << 1);
    pub const USE_MOD_MAP_MODS: Self = Self(1 << 2);
    pub const GROUP_ABSOLUTE: Self = Self(1 << 2);
    pub const ISO_DFLT_IS_GROUP: Self = Self(1 << 3);
}
impl From<SAIsoLockFlag> for u8 {
    #[inline]
    fn from(input: SAIsoLockFlag) -> Self {
        input.0
    }
}
impl From<SAIsoLockFlag> for Option<u8> {
    #[inline]
    fn from(input: SAIsoLockFlag) -> Self {
        Some(input.0)
    }
}
impl From<SAIsoLockFlag> for u16 {
    #[inline]
    fn from(input: SAIsoLockFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<SAIsoLockFlag> for Option<u16> {
    #[inline]
    fn from(input: SAIsoLockFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SAIsoLockFlag> for u32 {
    #[inline]
    fn from(input: SAIsoLockFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<SAIsoLockFlag> for Option<u32> {
    #[inline]
    fn from(input: SAIsoLockFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SAIsoLockFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SAIsoLockFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::NO_LOCK.0.into(), "NO_LOCK", "NoLock"),
            (Self::NO_UNLOCK.0.into(), "NO_UNLOCK", "NoUnlock"),
            (Self::USE_MOD_MAP_MODS.0.into(), "USE_MOD_MAP_MODS", "UseModMapMods"),
            (Self::GROUP_ABSOLUTE.0.into(), "GROUP_ABSOLUTE", "GroupAbsolute"),
            (Self::ISO_DFLT_IS_GROUP.0.into(), "ISO_DFLT_IS_GROUP", "ISODfltIsGroup"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(SAIsoLockFlag, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SAIsoLockNoAffect(u8);
impl SAIsoLockNoAffect {
    pub const CTRLS: Self = Self(1 << 3);
    pub const PTR: Self = Self(1 << 4);
    pub const GROUP: Self = Self(1 << 5);
    pub const MODS: Self = Self(1 << 6);
}
impl From<SAIsoLockNoAffect> for u8 {
    #[inline]
    fn from(input: SAIsoLockNoAffect) -> Self {
        input.0
    }
}
impl From<SAIsoLockNoAffect> for Option<u8> {
    #[inline]
    fn from(input: SAIsoLockNoAffect) -> Self {
        Some(input.0)
    }
}
impl From<SAIsoLockNoAffect> for u16 {
    #[inline]
    fn from(input: SAIsoLockNoAffect) -> Self {
        u16::from(input.0)
    }
}
impl From<SAIsoLockNoAffect> for Option<u16> {
    #[inline]
    fn from(input: SAIsoLockNoAffect) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SAIsoLockNoAffect> for u32 {
    #[inline]
    fn from(input: SAIsoLockNoAffect) -> Self {
        u32::from(input.0)
    }
}
impl From<SAIsoLockNoAffect> for Option<u32> {
    #[inline]
    fn from(input: SAIsoLockNoAffect) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SAIsoLockNoAffect {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SAIsoLockNoAffect  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::CTRLS.0.into(), "CTRLS", "Ctrls"),
            (Self::PTR.0.into(), "PTR", "Ptr"),
            (Self::GROUP.0.into(), "GROUP", "Group"),
            (Self::MODS.0.into(), "MODS", "Mods"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let type_ = type_.into();
        let result = SAIsoLock { type_, flags, mask, real_mods, group, affect, vmods_high, vmods_low };
        Ok((result, remaining))
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
        let remaining = remaining.get(7..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SATerminate { type_ };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SwitchScreenFlag(u8);
impl SwitchScreenFlag {
    pub const APPLICATION: Self = Self(1 << 0);
    pub const ABSOLUTE: Self = Self(1 << 2);
}
impl From<SwitchScreenFlag> for u8 {
    #[inline]
    fn from(input: SwitchScreenFlag) -> Self {
        input.0
    }
}
impl From<SwitchScreenFlag> for Option<u8> {
    #[inline]
    fn from(input: SwitchScreenFlag) -> Self {
        Some(input.0)
    }
}
impl From<SwitchScreenFlag> for u16 {
    #[inline]
    fn from(input: SwitchScreenFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<SwitchScreenFlag> for Option<u16> {
    #[inline]
    fn from(input: SwitchScreenFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SwitchScreenFlag> for u32 {
    #[inline]
    fn from(input: SwitchScreenFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<SwitchScreenFlag> for Option<u32> {
    #[inline]
    fn from(input: SwitchScreenFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SwitchScreenFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SwitchScreenFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::APPLICATION.0.into(), "APPLICATION", "Application"),
            (Self::ABSOLUTE.0.into(), "ABSOLUTE", "Absolute"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let remaining = remaining.get(5..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SASwitchScreen { type_, flags, new_screen };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoolCtrlsHigh(u8);
impl BoolCtrlsHigh {
    pub const ACCESS_X_FEEDBACK: Self = Self(1 << 0);
    pub const AUDIBLE_BELL: Self = Self(1 << 1);
    pub const OVERLAY1: Self = Self(1 << 2);
    pub const OVERLAY2: Self = Self(1 << 3);
    pub const IGNORE_GROUP_LOCK: Self = Self(1 << 4);
}
impl From<BoolCtrlsHigh> for u8 {
    #[inline]
    fn from(input: BoolCtrlsHigh) -> Self {
        input.0
    }
}
impl From<BoolCtrlsHigh> for Option<u8> {
    #[inline]
    fn from(input: BoolCtrlsHigh) -> Self {
        Some(input.0)
    }
}
impl From<BoolCtrlsHigh> for u16 {
    #[inline]
    fn from(input: BoolCtrlsHigh) -> Self {
        u16::from(input.0)
    }
}
impl From<BoolCtrlsHigh> for Option<u16> {
    #[inline]
    fn from(input: BoolCtrlsHigh) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<BoolCtrlsHigh> for u32 {
    #[inline]
    fn from(input: BoolCtrlsHigh) -> Self {
        u32::from(input.0)
    }
}
impl From<BoolCtrlsHigh> for Option<u32> {
    #[inline]
    fn from(input: BoolCtrlsHigh) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for BoolCtrlsHigh {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for BoolCtrlsHigh  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::ACCESS_X_FEEDBACK.0.into(), "ACCESS_X_FEEDBACK", "AccessXFeedback"),
            (Self::AUDIBLE_BELL.0.into(), "AUDIBLE_BELL", "AudibleBell"),
            (Self::OVERLAY1.0.into(), "OVERLAY1", "Overlay1"),
            (Self::OVERLAY2.0.into(), "OVERLAY2", "Overlay2"),
            (Self::IGNORE_GROUP_LOCK.0.into(), "IGNORE_GROUP_LOCK", "IgnoreGroupLock"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(BoolCtrlsHigh, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoolCtrlsLow(u8);
impl BoolCtrlsLow {
    pub const REPEAT_KEYS: Self = Self(1 << 0);
    pub const SLOW_KEYS: Self = Self(1 << 1);
    pub const BOUNCE_KEYS: Self = Self(1 << 2);
    pub const STICKY_KEYS: Self = Self(1 << 3);
    pub const MOUSE_KEYS: Self = Self(1 << 4);
    pub const MOUSE_KEYS_ACCEL: Self = Self(1 << 5);
    pub const ACCESS_X_KEYS: Self = Self(1 << 6);
    pub const ACCESS_X_TIMEOUT: Self = Self(1 << 7);
}
impl From<BoolCtrlsLow> for u8 {
    #[inline]
    fn from(input: BoolCtrlsLow) -> Self {
        input.0
    }
}
impl From<BoolCtrlsLow> for Option<u8> {
    #[inline]
    fn from(input: BoolCtrlsLow) -> Self {
        Some(input.0)
    }
}
impl From<BoolCtrlsLow> for u16 {
    #[inline]
    fn from(input: BoolCtrlsLow) -> Self {
        u16::from(input.0)
    }
}
impl From<BoolCtrlsLow> for Option<u16> {
    #[inline]
    fn from(input: BoolCtrlsLow) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<BoolCtrlsLow> for u32 {
    #[inline]
    fn from(input: BoolCtrlsLow) -> Self {
        u32::from(input.0)
    }
}
impl From<BoolCtrlsLow> for Option<u32> {
    #[inline]
    fn from(input: BoolCtrlsLow) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for BoolCtrlsLow {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for BoolCtrlsLow  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::REPEAT_KEYS.0.into(), "REPEAT_KEYS", "RepeatKeys"),
            (Self::SLOW_KEYS.0.into(), "SLOW_KEYS", "SlowKeys"),
            (Self::BOUNCE_KEYS.0.into(), "BOUNCE_KEYS", "BounceKeys"),
            (Self::STICKY_KEYS.0.into(), "STICKY_KEYS", "StickyKeys"),
            (Self::MOUSE_KEYS.0.into(), "MOUSE_KEYS", "MouseKeys"),
            (Self::MOUSE_KEYS_ACCEL.0.into(), "MOUSE_KEYS_ACCEL", "MouseKeysAccel"),
            (Self::ACCESS_X_KEYS.0.into(), "ACCESS_X_KEYS", "AccessXKeys"),
            (Self::ACCESS_X_TIMEOUT.0.into(), "ACCESS_X_TIMEOUT", "AccessXTimeout"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (bool_ctrls_high, remaining) = u8::try_parse(remaining)?;
        let (bool_ctrls_low, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SASetControls { type_, bool_ctrls_high, bool_ctrls_low };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ActionMessageFlag(u8);
impl ActionMessageFlag {
    pub const ON_PRESS: Self = Self(1 << 0);
    pub const ON_RELEASE: Self = Self(1 << 1);
    pub const GEN_KEY_EVENT: Self = Self(1 << 2);
}
impl From<ActionMessageFlag> for u8 {
    #[inline]
    fn from(input: ActionMessageFlag) -> Self {
        input.0
    }
}
impl From<ActionMessageFlag> for Option<u8> {
    #[inline]
    fn from(input: ActionMessageFlag) -> Self {
        Some(input.0)
    }
}
impl From<ActionMessageFlag> for u16 {
    #[inline]
    fn from(input: ActionMessageFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<ActionMessageFlag> for Option<u16> {
    #[inline]
    fn from(input: ActionMessageFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ActionMessageFlag> for u32 {
    #[inline]
    fn from(input: ActionMessageFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<ActionMessageFlag> for Option<u32> {
    #[inline]
    fn from(input: ActionMessageFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ActionMessageFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ActionMessageFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::ON_PRESS.0.into(), "ON_PRESS", "OnPress"),
            (Self::ON_RELEASE.0.into(), "ON_RELEASE", "OnRelease"),
            (Self::GEN_KEY_EVENT.0.into(), "GEN_KEY_EVENT", "GenKeyEvent"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let type_ = type_.into();
        let result = SAActionMessage { type_, flags, message };
        Ok((result, remaining))
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
        let type_ = type_.into();
        let result = SARedirectKey { type_, newkey, mask, real_modifiers, vmods_mask_high, vmods_mask_low, vmods_high, vmods_low };
        Ok((result, remaining))
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SADeviceBtn { type_, flags, count, button, device };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LockDeviceFlags(u8);
impl LockDeviceFlags {
    pub const NO_LOCK: Self = Self(1 << 0);
    pub const NO_UNLOCK: Self = Self(1 << 1);
}
impl From<LockDeviceFlags> for u8 {
    #[inline]
    fn from(input: LockDeviceFlags) -> Self {
        input.0
    }
}
impl From<LockDeviceFlags> for Option<u8> {
    #[inline]
    fn from(input: LockDeviceFlags) -> Self {
        Some(input.0)
    }
}
impl From<LockDeviceFlags> for u16 {
    #[inline]
    fn from(input: LockDeviceFlags) -> Self {
        u16::from(input.0)
    }
}
impl From<LockDeviceFlags> for Option<u16> {
    #[inline]
    fn from(input: LockDeviceFlags) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<LockDeviceFlags> for u32 {
    #[inline]
    fn from(input: LockDeviceFlags) -> Self {
        u32::from(input.0)
    }
}
impl From<LockDeviceFlags> for Option<u32> {
    #[inline]
    fn from(input: LockDeviceFlags) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for LockDeviceFlags {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for LockDeviceFlags  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::NO_LOCK.0.into(), "NO_LOCK", "NoLock"),
            (Self::NO_UNLOCK.0.into(), "NO_UNLOCK", "NoUnlock"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (button, remaining) = u8::try_parse(remaining)?;
        let (device, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let result = SALockDeviceBtn { type_, flags, button, device };
        Ok((result, remaining))
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SAValWhat(u8);
impl SAValWhat {
    pub const IGNORE_VAL: Self = Self(0);
    pub const SET_VAL_MIN: Self = Self(1);
    pub const SET_VAL_CENTER: Self = Self(2);
    pub const SET_VAL_MAX: Self = Self(3);
    pub const SET_VAL_RELATIVE: Self = Self(4);
    pub const SET_VAL_ABSOLUTE: Self = Self(5);
}
impl From<SAValWhat> for u8 {
    #[inline]
    fn from(input: SAValWhat) -> Self {
        input.0
    }
}
impl From<SAValWhat> for Option<u8> {
    #[inline]
    fn from(input: SAValWhat) -> Self {
        Some(input.0)
    }
}
impl From<SAValWhat> for u16 {
    #[inline]
    fn from(input: SAValWhat) -> Self {
        u16::from(input.0)
    }
}
impl From<SAValWhat> for Option<u16> {
    #[inline]
    fn from(input: SAValWhat) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<SAValWhat> for u32 {
    #[inline]
    fn from(input: SAValWhat) -> Self {
        u32::from(input.0)
    }
}
impl From<SAValWhat> for Option<u32> {
    #[inline]
    fn from(input: SAValWhat) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for SAValWhat {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SAValWhat  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::IGNORE_VAL.0.into(), "IGNORE_VAL", "IgnoreVal"),
            (Self::SET_VAL_MIN.0.into(), "SET_VAL_MIN", "SetValMin"),
            (Self::SET_VAL_CENTER.0.into(), "SET_VAL_CENTER", "SetValCenter"),
            (Self::SET_VAL_MAX.0.into(), "SET_VAL_MAX", "SetValMax"),
            (Self::SET_VAL_RELATIVE.0.into(), "SET_VAL_RELATIVE", "SetValRelative"),
            (Self::SET_VAL_ABSOLUTE.0.into(), "SET_VAL_ABSOLUTE", "SetValAbsolute"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
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
        let type_ = type_.into();
        let val1what = val1what.into();
        let val2what = val2what.into();
        let result = SADeviceValuator { type_, device, val1what, val1index, val1value, val2what, val2index, val2value };
        Ok((result, remaining))
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
        let type_ = type_.into();
        let result = SIAction { type_, data };
        Ok((result, remaining))
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
            let type_ = type_.into();
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
            .ok_or(ParseError::InsufficientData)?
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseExtensionRequest {
    pub wanted_major: u16,
    pub wanted_minor: u16,
}
impl UseExtensionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let wanted_major_bytes = self.wanted_major.serialize();
        let wanted_minor_bytes = self.wanted_minor.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, UseExtensionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != USE_EXTENSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (wanted_major, remaining) = u16::try_parse(value)?;
        let (wanted_minor, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(UseExtensionRequest {
            wanted_major,
            wanted_minor,
        })
    }
}
impl Request for UseExtensionRequest {
    type Reply = UseExtensionReply;
}
pub fn use_extension<Conn>(conn: &Conn, wanted_major: u16, wanted_minor: u16) -> Result<Cookie<'_, Conn, UseExtensionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UseExtensionRequest {
        wanted_major,
        wanted_minor,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseExtensionReply {
    pub supported: bool,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}
impl TryParse for UseExtensionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (supported, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = UseExtensionReply { supported, sequence, length, server_major, server_minor };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectEventsAuxBitcase1 {
    pub affect_new_keyboard: u16,
    pub new_keyboard_details: u16,
}
impl TryParse for SelectEventsAuxBitcase1 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_new_keyboard, remaining) = u16::try_parse(remaining)?;
        let (new_keyboard_details, remaining) = u16::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase1 { affect_new_keyboard, new_keyboard_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase2 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_state, remaining) = u16::try_parse(remaining)?;
        let (state_details, remaining) = u16::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase2 { affect_state, state_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase3 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_ctrls, remaining) = u32::try_parse(remaining)?;
        let (ctrl_details, remaining) = u32::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase3 { affect_ctrls, ctrl_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase4 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_indicator_state, remaining) = u32::try_parse(remaining)?;
        let (indicator_state_details, remaining) = u32::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase4 { affect_indicator_state, indicator_state_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase5 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_indicator_map, remaining) = u32::try_parse(remaining)?;
        let (indicator_map_details, remaining) = u32::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase5 { affect_indicator_map, indicator_map_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase6 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_names, remaining) = u16::try_parse(remaining)?;
        let (names_details, remaining) = u16::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase6 { affect_names, names_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase7 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_compat, remaining) = u8::try_parse(remaining)?;
        let (compat_details, remaining) = u8::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase7 { affect_compat, compat_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase8 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_bell, remaining) = u8::try_parse(remaining)?;
        let (bell_details, remaining) = u8::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase8 { affect_bell, bell_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase9 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_msg_details, remaining) = u8::try_parse(remaining)?;
        let (msg_details, remaining) = u8::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase9 { affect_msg_details, msg_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase10 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_access_x, remaining) = u16::try_parse(remaining)?;
        let (access_x_details, remaining) = u16::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase10 { affect_access_x, access_x_details };
        Ok((result, remaining))
    }
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
impl TryParse for SelectEventsAuxBitcase11 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (affect_ext_dev, remaining) = u16::try_parse(remaining)?;
        let (extdev_details, remaining) = u16::try_parse(remaining)?;
        let result = SelectEventsAuxBitcase11 { affect_ext_dev, extdev_details };
        Ok((result, remaining))
    }
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
impl SelectEventsAux {
    fn try_parse(value: &[u8], affect_which: u16, clear: u16, select_all: u16) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = u32::from(affect_which) & ((!u32::from(clear)) & (!u32::from(select_all)));
        let mut outer_remaining = value;
        let bitcase1 = if switch_expr & u32::from(EventType::NEW_KEYBOARD_NOTIFY) != 0 {
            let (bitcase1, new_remaining) = SelectEventsAuxBitcase1::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase1)
        } else {
            None
        };
        let bitcase2 = if switch_expr & u32::from(EventType::STATE_NOTIFY) != 0 {
            let (bitcase2, new_remaining) = SelectEventsAuxBitcase2::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase2)
        } else {
            None
        };
        let bitcase3 = if switch_expr & u32::from(EventType::CONTROLS_NOTIFY) != 0 {
            let (bitcase3, new_remaining) = SelectEventsAuxBitcase3::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase3)
        } else {
            None
        };
        let bitcase4 = if switch_expr & u32::from(EventType::INDICATOR_STATE_NOTIFY) != 0 {
            let (bitcase4, new_remaining) = SelectEventsAuxBitcase4::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase4)
        } else {
            None
        };
        let bitcase5 = if switch_expr & u32::from(EventType::INDICATOR_MAP_NOTIFY) != 0 {
            let (bitcase5, new_remaining) = SelectEventsAuxBitcase5::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase5)
        } else {
            None
        };
        let bitcase6 = if switch_expr & u32::from(EventType::NAMES_NOTIFY) != 0 {
            let (bitcase6, new_remaining) = SelectEventsAuxBitcase6::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase6)
        } else {
            None
        };
        let bitcase7 = if switch_expr & u32::from(EventType::COMPAT_MAP_NOTIFY) != 0 {
            let (bitcase7, new_remaining) = SelectEventsAuxBitcase7::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase7)
        } else {
            None
        };
        let bitcase8 = if switch_expr & u32::from(EventType::BELL_NOTIFY) != 0 {
            let (bitcase8, new_remaining) = SelectEventsAuxBitcase8::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase8)
        } else {
            None
        };
        let bitcase9 = if switch_expr & u32::from(EventType::ACTION_MESSAGE) != 0 {
            let (bitcase9, new_remaining) = SelectEventsAuxBitcase9::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase9)
        } else {
            None
        };
        let bitcase10 = if switch_expr & u32::from(EventType::ACCESS_X_NOTIFY) != 0 {
            let (bitcase10, new_remaining) = SelectEventsAuxBitcase10::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase10)
        } else {
            None
        };
        let bitcase11 = if switch_expr & u32::from(EventType::EXTENSION_DEVICE_NOTIFY) != 0 {
            let (bitcase11, new_remaining) = SelectEventsAuxBitcase11::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(bitcase11)
        } else {
            None
        };
        let result = SelectEventsAux { bitcase1, bitcase2, bitcase3, bitcase4, bitcase5, bitcase6, bitcase7, bitcase8, bitcase9, bitcase10, bitcase11 };
        Ok((result, outer_remaining))
    }
}
impl SelectEventsAux {
    #[allow(dead_code)]
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
            expr_value |= u32::from(EventType::NEW_KEYBOARD_NOTIFY);
        }
        if self.bitcase2.is_some() {
            expr_value |= u32::from(EventType::STATE_NOTIFY);
        }
        if self.bitcase3.is_some() {
            expr_value |= u32::from(EventType::CONTROLS_NOTIFY);
        }
        if self.bitcase4.is_some() {
            expr_value |= u32::from(EventType::INDICATOR_STATE_NOTIFY);
        }
        if self.bitcase5.is_some() {
            expr_value |= u32::from(EventType::INDICATOR_MAP_NOTIFY);
        }
        if self.bitcase6.is_some() {
            expr_value |= u32::from(EventType::NAMES_NOTIFY);
        }
        if self.bitcase7.is_some() {
            expr_value |= u32::from(EventType::COMPAT_MAP_NOTIFY);
        }
        if self.bitcase8.is_some() {
            expr_value |= u32::from(EventType::BELL_NOTIFY);
        }
        if self.bitcase9.is_some() {
            expr_value |= u32::from(EventType::ACTION_MESSAGE);
        }
        if self.bitcase10.is_some() {
            expr_value |= u32::from(EventType::ACCESS_X_NOTIFY);
        }
        if self.bitcase11.is_some() {
            expr_value |= u32::from(EventType::EXTENSION_DEVICE_NOTIFY);
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

/// Opcode for the SelectEvents request
pub const SELECT_EVENTS_REQUEST: u8 = 1;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectEventsRequest<'input> {
    pub device_spec: DeviceSpec,
    pub clear: u16,
    pub select_all: u16,
    pub affect_map: u16,
    pub map: u16,
    pub details: Cow<'input, SelectEventsAux>,
}
impl<'input> SelectEventsRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let affect_which = u16::try_from(self.details.switch_expr() | (u32::from(self.clear) | u32::from(self.select_all))).unwrap();
        let affect_which_bytes = affect_which.serialize();
        let clear_bytes = self.clear.serialize();
        let select_all_bytes = self.select_all.serialize();
        let affect_map_bytes = self.affect_map.serialize();
        let map_bytes = self.map.serialize();
        let mut request0 = vec![
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
        let details_bytes = self.details.serialize(affect_which, self.clear, self.select_all);
        let length_so_far = length_so_far + details_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), details_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SELECT_EVENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (affect_which, remaining) = u16::try_parse(remaining)?;
        let (clear, remaining) = u16::try_parse(remaining)?;
        let (select_all, remaining) = u16::try_parse(remaining)?;
        let (affect_map, remaining) = u16::try_parse(remaining)?;
        let (map, remaining) = u16::try_parse(remaining)?;
        let (details, remaining) = SelectEventsAux::try_parse(remaining, affect_which, clear, select_all)?;
        let _ = remaining;
        Ok(SelectEventsRequest {
            device_spec,
            clear,
            select_all,
            affect_map,
            map,
            details: Cow::Owned(details),
        })
    }
    /// Clone all borrowed data in this SelectEventsRequest.
    pub fn into_owned(self) -> SelectEventsRequest<'static> {
        SelectEventsRequest {
            device_spec: self.device_spec,
            clear: self.clear,
            select_all: self.select_all,
            affect_map: self.affect_map,
            map: self.map,
            details: Cow::Owned(self.details.into_owned()),
        }
    }
}
impl<'input> Request for SelectEventsRequest<'input> {
    type Reply = ();
}
pub fn select_events<'c, 'input, Conn, A, B, C, D>(conn: &'c Conn, device_spec: DeviceSpec, clear: A, select_all: B, affect_map: C, map: D, details: &'input SelectEventsAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u16>,
    C: Into<u16>,
    D: Into<u16>,
{
    let clear: u16 = clear.into();
    let select_all: u16 = select_all.into();
    let affect_map: u16 = affect_map.into();
    let map: u16 = map.into();
    let request0 = SelectEventsRequest {
        device_spec,
        clear,
        select_all,
        affect_map,
        map,
        details: Cow::Borrowed(details),
    };
    request0.send(conn)
}

/// Opcode for the Bell request
pub const BELL_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BellRequest {
    pub device_spec: DeviceSpec,
    pub bell_class: BellClassSpec,
    pub bell_id: IDSpec,
    pub percent: i8,
    pub force_sound: bool,
    pub event_only: bool,
    pub pitch: i16,
    pub duration: i16,
    pub name: xproto::Atom,
    pub window: xproto::Window,
}
impl BellRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let bell_class_bytes = self.bell_class.serialize();
        let bell_id_bytes = self.bell_id.serialize();
        let percent_bytes = self.percent.serialize();
        let force_sound_bytes = self.force_sound.serialize();
        let event_only_bytes = self.event_only.serialize();
        let pitch_bytes = self.pitch.serialize();
        let duration_bytes = self.duration.serialize();
        let name_bytes = self.name.serialize();
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != BELL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (bell_class, remaining) = BellClassSpec::try_parse(remaining)?;
        let (bell_id, remaining) = IDSpec::try_parse(remaining)?;
        let (percent, remaining) = i8::try_parse(remaining)?;
        let (force_sound, remaining) = bool::try_parse(remaining)?;
        let (event_only, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (pitch, remaining) = i16::try_parse(remaining)?;
        let (duration, remaining) = i16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let _ = remaining;
        Ok(BellRequest {
            device_spec,
            bell_class,
            bell_id,
            percent,
            force_sound,
            event_only,
            pitch,
            duration,
            name,
            window,
        })
    }
}
impl Request for BellRequest {
    type Reply = ();
}
pub fn bell<Conn>(conn: &Conn, device_spec: DeviceSpec, bell_class: BellClassSpec, bell_id: IDSpec, percent: i8, force_sound: bool, event_only: bool, pitch: i16, duration: i16, name: xproto::Atom, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = BellRequest {
        device_spec,
        bell_class,
        bell_id,
        percent,
        force_sound,
        event_only,
        pitch,
        duration,
        name,
        window,
    };
    request0.send(conn)
}

/// Opcode for the GetState request
pub const GET_STATE_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStateRequest {
    pub device_spec: DeviceSpec,
}
impl GetStateRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetStateReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_STATE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetStateRequest {
            device_spec,
        })
    }
}
impl Request for GetStateRequest {
    type Reply = GetStateReply;
}
pub fn get_state<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetStateRequest {
        device_spec,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStateReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (ptr_btn_state, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(6..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let group = group.into();
        let locked_group = locked_group.into();
        let result = GetStateReply { device_id, sequence, length, mods, base_mods, latched_mods, locked_mods, group, locked_group, base_group, latched_group, compat_state, grab_mods, compat_grab_mods, lookup_mods, compat_lookup_mods, ptr_btn_state };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the LatchLockState request
pub const LATCH_LOCK_STATE_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LatchLockStateRequest {
    pub device_spec: DeviceSpec,
    pub affect_mod_locks: u8,
    pub mod_locks: u8,
    pub lock_group: bool,
    pub group_lock: Group,
    pub affect_mod_latches: u8,
    pub latch_group: bool,
    pub group_latch: u16,
}
impl LatchLockStateRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let affect_mod_locks_bytes = self.affect_mod_locks.serialize();
        let mod_locks_bytes = self.mod_locks.serialize();
        let lock_group_bytes = self.lock_group.serialize();
        let group_lock_bytes = u8::from(self.group_lock).serialize();
        let affect_mod_latches_bytes = self.affect_mod_latches.serialize();
        let latch_group_bytes = self.latch_group.serialize();
        let group_latch_bytes = self.group_latch.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LATCH_LOCK_STATE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (affect_mod_locks, remaining) = u8::try_parse(remaining)?;
        let (mod_locks, remaining) = u8::try_parse(remaining)?;
        let (lock_group, remaining) = bool::try_parse(remaining)?;
        let (group_lock, remaining) = u8::try_parse(remaining)?;
        let group_lock = group_lock.into();
        let (affect_mod_latches, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (latch_group, remaining) = bool::try_parse(remaining)?;
        let (group_latch, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(LatchLockStateRequest {
            device_spec,
            affect_mod_locks,
            mod_locks,
            lock_group,
            group_lock,
            affect_mod_latches,
            latch_group,
            group_latch,
        })
    }
}
impl Request for LatchLockStateRequest {
    type Reply = ();
}
pub fn latch_lock_state<Conn, A, B, C>(conn: &Conn, device_spec: DeviceSpec, affect_mod_locks: A, mod_locks: B, lock_group: bool, group_lock: Group, affect_mod_latches: C, latch_group: bool, group_latch: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
    B: Into<u8>,
    C: Into<u8>,
{
    let affect_mod_locks: u8 = affect_mod_locks.into();
    let mod_locks: u8 = mod_locks.into();
    let affect_mod_latches: u8 = affect_mod_latches.into();
    let request0 = LatchLockStateRequest {
        device_spec,
        affect_mod_locks,
        mod_locks,
        lock_group,
        group_lock,
        affect_mod_latches,
        latch_group,
        group_latch,
    };
    request0.send(conn)
}

/// Opcode for the GetControls request
pub const GET_CONTROLS_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetControlsRequest {
    pub device_spec: DeviceSpec,
}
impl GetControlsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetControlsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CONTROLS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetControlsRequest {
            device_spec,
        })
    }
}
impl Request for GetControlsRequest {
    type Reply = GetControlsReply;
}
pub fn get_controls<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetControlsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetControlsRequest {
        device_spec,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetControlsReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (access_x_timeout_mask, remaining) = u32::try_parse(remaining)?;
        let (access_x_timeout_values, remaining) = u32::try_parse(remaining)?;
        let (enabled_controls, remaining) = u32::try_parse(remaining)?;
        let (per_key_repeat, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let per_key_repeat = <[u8; 32]>::try_from(per_key_repeat).unwrap();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetControlsReply { device_id, sequence, length, mouse_keys_dflt_btn, num_groups, groups_wrap, internal_mods_mask, ignore_lock_mods_mask, internal_mods_real_mods, ignore_lock_mods_real_mods, internal_mods_vmods, ignore_lock_mods_vmods, repeat_delay, repeat_interval, slow_keys_delay, debounce_delay, mouse_keys_delay, mouse_keys_interval, mouse_keys_time_to_max, mouse_keys_max_speed, mouse_keys_curve, access_x_option, access_x_timeout, access_x_timeout_options_mask, access_x_timeout_options_values, access_x_timeout_mask, access_x_timeout_values, enabled_controls, per_key_repeat };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetControls request
pub const SET_CONTROLS_REQUEST: u8 = 7;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetControlsRequest<'input> {
    pub device_spec: DeviceSpec,
    pub affect_internal_real_mods: u8,
    pub internal_real_mods: u8,
    pub affect_ignore_lock_real_mods: u8,
    pub ignore_lock_real_mods: u8,
    pub affect_internal_virtual_mods: u16,
    pub internal_virtual_mods: u16,
    pub affect_ignore_lock_virtual_mods: u16,
    pub ignore_lock_virtual_mods: u16,
    pub mouse_keys_dflt_btn: u8,
    pub groups_wrap: u8,
    pub access_x_options: u16,
    pub affect_enabled_controls: u32,
    pub enabled_controls: u32,
    pub change_controls: u32,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys_delay: u16,
    pub mouse_keys_interval: u16,
    pub mouse_keys_time_to_max: u16,
    pub mouse_keys_max_speed: u16,
    pub mouse_keys_curve: i16,
    pub access_x_timeout: u16,
    pub access_x_timeout_mask: u32,
    pub access_x_timeout_values: u32,
    pub access_x_timeout_options_mask: u16,
    pub access_x_timeout_options_values: u16,
    pub per_key_repeat: Cow<'input, [u8; 32]>,
}
impl<'input> SetControlsRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let affect_internal_real_mods_bytes = self.affect_internal_real_mods.serialize();
        let internal_real_mods_bytes = self.internal_real_mods.serialize();
        let affect_ignore_lock_real_mods_bytes = self.affect_ignore_lock_real_mods.serialize();
        let ignore_lock_real_mods_bytes = self.ignore_lock_real_mods.serialize();
        let affect_internal_virtual_mods_bytes = self.affect_internal_virtual_mods.serialize();
        let internal_virtual_mods_bytes = self.internal_virtual_mods.serialize();
        let affect_ignore_lock_virtual_mods_bytes = self.affect_ignore_lock_virtual_mods.serialize();
        let ignore_lock_virtual_mods_bytes = self.ignore_lock_virtual_mods.serialize();
        let mouse_keys_dflt_btn_bytes = self.mouse_keys_dflt_btn.serialize();
        let groups_wrap_bytes = self.groups_wrap.serialize();
        let access_x_options_bytes = self.access_x_options.serialize();
        let affect_enabled_controls_bytes = self.affect_enabled_controls.serialize();
        let enabled_controls_bytes = self.enabled_controls.serialize();
        let change_controls_bytes = self.change_controls.serialize();
        let repeat_delay_bytes = self.repeat_delay.serialize();
        let repeat_interval_bytes = self.repeat_interval.serialize();
        let slow_keys_delay_bytes = self.slow_keys_delay.serialize();
        let debounce_delay_bytes = self.debounce_delay.serialize();
        let mouse_keys_delay_bytes = self.mouse_keys_delay.serialize();
        let mouse_keys_interval_bytes = self.mouse_keys_interval.serialize();
        let mouse_keys_time_to_max_bytes = self.mouse_keys_time_to_max.serialize();
        let mouse_keys_max_speed_bytes = self.mouse_keys_max_speed.serialize();
        let mouse_keys_curve_bytes = self.mouse_keys_curve.serialize();
        let access_x_timeout_bytes = self.access_x_timeout.serialize();
        let access_x_timeout_mask_bytes = self.access_x_timeout_mask.serialize();
        let access_x_timeout_values_bytes = self.access_x_timeout_values.serialize();
        let access_x_timeout_options_mask_bytes = self.access_x_timeout_options_mask.serialize();
        let access_x_timeout_options_values_bytes = self.access_x_timeout_options_values.serialize();
        let mut request0 = vec![
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
        let length_so_far = length_so_far + self.per_key_repeat.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), Cow::Owned(self.per_key_repeat.to_vec())], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_CONTROLS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (affect_internal_real_mods, remaining) = u8::try_parse(remaining)?;
        let (internal_real_mods, remaining) = u8::try_parse(remaining)?;
        let (affect_ignore_lock_real_mods, remaining) = u8::try_parse(remaining)?;
        let (ignore_lock_real_mods, remaining) = u8::try_parse(remaining)?;
        let (affect_internal_virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (internal_virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (affect_ignore_lock_virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (ignore_lock_virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_dflt_btn, remaining) = u8::try_parse(remaining)?;
        let (groups_wrap, remaining) = u8::try_parse(remaining)?;
        let (access_x_options, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (affect_enabled_controls, remaining) = u32::try_parse(remaining)?;
        let (enabled_controls, remaining) = u32::try_parse(remaining)?;
        let (change_controls, remaining) = u32::try_parse(remaining)?;
        let (repeat_delay, remaining) = u16::try_parse(remaining)?;
        let (repeat_interval, remaining) = u16::try_parse(remaining)?;
        let (slow_keys_delay, remaining) = u16::try_parse(remaining)?;
        let (debounce_delay, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_delay, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_interval, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_time_to_max, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_max_speed, remaining) = u16::try_parse(remaining)?;
        let (mouse_keys_curve, remaining) = i16::try_parse(remaining)?;
        let (access_x_timeout, remaining) = u16::try_parse(remaining)?;
        let (access_x_timeout_mask, remaining) = u32::try_parse(remaining)?;
        let (access_x_timeout_values, remaining) = u32::try_parse(remaining)?;
        let (access_x_timeout_options_mask, remaining) = u16::try_parse(remaining)?;
        let (access_x_timeout_options_values, remaining) = u16::try_parse(remaining)?;
        let (per_key_repeat, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let per_key_repeat = <&[u8; 32]>::try_from(per_key_repeat).unwrap();
        let _ = remaining;
        Ok(SetControlsRequest {
            device_spec,
            affect_internal_real_mods,
            internal_real_mods,
            affect_ignore_lock_real_mods,
            ignore_lock_real_mods,
            affect_internal_virtual_mods,
            internal_virtual_mods,
            affect_ignore_lock_virtual_mods,
            ignore_lock_virtual_mods,
            mouse_keys_dflt_btn,
            groups_wrap,
            access_x_options,
            affect_enabled_controls,
            enabled_controls,
            change_controls,
            repeat_delay,
            repeat_interval,
            slow_keys_delay,
            debounce_delay,
            mouse_keys_delay,
            mouse_keys_interval,
            mouse_keys_time_to_max,
            mouse_keys_max_speed,
            mouse_keys_curve,
            access_x_timeout,
            access_x_timeout_mask,
            access_x_timeout_values,
            access_x_timeout_options_mask,
            access_x_timeout_options_values,
            per_key_repeat: Cow::Borrowed(per_key_repeat),
        })
    }
    /// Clone all borrowed data in this SetControlsRequest.
    pub fn into_owned(self) -> SetControlsRequest<'static> {
        SetControlsRequest {
            device_spec: self.device_spec,
            affect_internal_real_mods: self.affect_internal_real_mods,
            internal_real_mods: self.internal_real_mods,
            affect_ignore_lock_real_mods: self.affect_ignore_lock_real_mods,
            ignore_lock_real_mods: self.ignore_lock_real_mods,
            affect_internal_virtual_mods: self.affect_internal_virtual_mods,
            internal_virtual_mods: self.internal_virtual_mods,
            affect_ignore_lock_virtual_mods: self.affect_ignore_lock_virtual_mods,
            ignore_lock_virtual_mods: self.ignore_lock_virtual_mods,
            mouse_keys_dflt_btn: self.mouse_keys_dflt_btn,
            groups_wrap: self.groups_wrap,
            access_x_options: self.access_x_options,
            affect_enabled_controls: self.affect_enabled_controls,
            enabled_controls: self.enabled_controls,
            change_controls: self.change_controls,
            repeat_delay: self.repeat_delay,
            repeat_interval: self.repeat_interval,
            slow_keys_delay: self.slow_keys_delay,
            debounce_delay: self.debounce_delay,
            mouse_keys_delay: self.mouse_keys_delay,
            mouse_keys_interval: self.mouse_keys_interval,
            mouse_keys_time_to_max: self.mouse_keys_time_to_max,
            mouse_keys_max_speed: self.mouse_keys_max_speed,
            mouse_keys_curve: self.mouse_keys_curve,
            access_x_timeout: self.access_x_timeout,
            access_x_timeout_mask: self.access_x_timeout_mask,
            access_x_timeout_values: self.access_x_timeout_values,
            access_x_timeout_options_mask: self.access_x_timeout_options_mask,
            access_x_timeout_options_values: self.access_x_timeout_options_values,
            per_key_repeat: Cow::Owned(self.per_key_repeat.into_owned()),
        }
    }
}
impl<'input> Request for SetControlsRequest<'input> {
    type Reply = ();
}
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
    let request0 = SetControlsRequest {
        device_spec,
        affect_internal_real_mods,
        internal_real_mods,
        affect_ignore_lock_real_mods,
        ignore_lock_real_mods,
        affect_internal_virtual_mods,
        internal_virtual_mods,
        affect_ignore_lock_virtual_mods,
        ignore_lock_virtual_mods,
        mouse_keys_dflt_btn,
        groups_wrap,
        access_x_options,
        affect_enabled_controls,
        enabled_controls,
        change_controls,
        repeat_delay,
        repeat_interval,
        slow_keys_delay,
        debounce_delay,
        mouse_keys_delay,
        mouse_keys_interval,
        mouse_keys_time_to_max,
        mouse_keys_max_speed,
        mouse_keys_curve,
        access_x_timeout,
        access_x_timeout_mask,
        access_x_timeout_values,
        access_x_timeout_options_mask,
        access_x_timeout_options_values,
        per_key_repeat: Cow::Borrowed(per_key_repeat),
    };
    request0.send(conn)
}

/// Opcode for the GetMap request
pub const GET_MAP_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMapRequest {
    pub device_spec: DeviceSpec,
    pub full: u16,
    pub partial: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xproto::Keycode,
    pub n_key_syms: u8,
    pub first_key_action: xproto::Keycode,
    pub n_key_actions: u8,
    pub first_key_behavior: xproto::Keycode,
    pub n_key_behaviors: u8,
    pub virtual_mods: u16,
    pub first_key_explicit: xproto::Keycode,
    pub n_key_explicit: u8,
    pub first_mod_map_key: xproto::Keycode,
    pub n_mod_map_keys: u8,
    pub first_v_mod_map_key: xproto::Keycode,
    pub n_v_mod_map_keys: u8,
}
impl GetMapRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let full_bytes = self.full.serialize();
        let partial_bytes = self.partial.serialize();
        let first_type_bytes = self.first_type.serialize();
        let n_types_bytes = self.n_types.serialize();
        let first_key_sym_bytes = self.first_key_sym.serialize();
        let n_key_syms_bytes = self.n_key_syms.serialize();
        let first_key_action_bytes = self.first_key_action.serialize();
        let n_key_actions_bytes = self.n_key_actions.serialize();
        let first_key_behavior_bytes = self.first_key_behavior.serialize();
        let n_key_behaviors_bytes = self.n_key_behaviors.serialize();
        let virtual_mods_bytes = self.virtual_mods.serialize();
        let first_key_explicit_bytes = self.first_key_explicit.serialize();
        let n_key_explicit_bytes = self.n_key_explicit.serialize();
        let first_mod_map_key_bytes = self.first_mod_map_key.serialize();
        let n_mod_map_keys_bytes = self.n_mod_map_keys.serialize();
        let first_v_mod_map_key_bytes = self.first_v_mod_map_key.serialize();
        let n_v_mod_map_keys_bytes = self.n_v_mod_map_keys.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetMapReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (full, remaining) = u16::try_parse(remaining)?;
        let (partial, remaining) = u16::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (first_key_sym, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_syms, remaining) = u8::try_parse(remaining)?;
        let (first_key_action, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_actions, remaining) = u8::try_parse(remaining)?;
        let (first_key_behavior, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_behaviors, remaining) = u8::try_parse(remaining)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (first_key_explicit, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_explicit, remaining) = u8::try_parse(remaining)?;
        let (first_mod_map_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let (first_v_mod_map_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_v_mod_map_keys, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetMapRequest {
            device_spec,
            full,
            partial,
            first_type,
            n_types,
            first_key_sym,
            n_key_syms,
            first_key_action,
            n_key_actions,
            first_key_behavior,
            n_key_behaviors,
            virtual_mods,
            first_key_explicit,
            n_key_explicit,
            first_mod_map_key,
            n_mod_map_keys,
            first_v_mod_map_key,
            n_v_mod_map_keys,
        })
    }
}
impl Request for GetMapRequest {
    type Reply = GetMapReply;
}
pub fn get_map<Conn, A, B, C>(conn: &Conn, device_spec: DeviceSpec, full: A, partial: B, first_type: u8, n_types: u8, first_key_sym: xproto::Keycode, n_key_syms: u8, first_key_action: xproto::Keycode, n_key_actions: u8, first_key_behavior: xproto::Keycode, n_key_behaviors: u8, virtual_mods: C, first_key_explicit: xproto::Keycode, n_key_explicit: u8, first_mod_map_key: xproto::Keycode, n_mod_map_keys: u8, first_v_mod_map_key: xproto::Keycode, n_v_mod_map_keys: u8) -> Result<Cookie<'_, Conn, GetMapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u16>,
    C: Into<u16>,
{
    let full: u16 = full.into();
    let partial: u16 = partial.into();
    let virtual_mods: u16 = virtual_mods.into();
    let request0 = GetMapRequest {
        device_spec,
        full,
        partial,
        first_type,
        n_types,
        first_key_sym,
        n_key_syms,
        first_key_action,
        n_key_actions,
        first_key_behavior,
        n_key_behaviors,
        virtual_mods,
        first_key_explicit,
        n_key_explicit,
        first_mod_map_key,
        n_mod_map_keys,
        first_v_mod_map_key,
        n_v_mod_map_keys,
    };
    request0.send(conn)
}

#[derive(Debug, Clone)]
pub struct GetMapMapBitcase3 {
    pub acts_rtrn_count: Vec<u8>,
    pub acts_rtrn_acts: Vec<Action>,
}
impl GetMapMapBitcase3 {
    pub fn try_parse(remaining: &[u8], n_key_actions: u8, total_actions: u16) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (acts_rtrn_count, remaining) = crate::x11_utils::parse_u8_list(remaining, n_key_actions.try_to_usize()?)?;
        let acts_rtrn_count = acts_rtrn_count.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (acts_rtrn_acts, remaining) = crate::x11_utils::parse_list::<Action>(remaining, total_actions.try_to_usize()?)?;
        let result = GetMapMapBitcase3 { acts_rtrn_count, acts_rtrn_acts };
        Ok((result, remaining))
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
        let switch_expr = u32::from(present);
        let mut outer_remaining = value;
        let types_rtrn = if switch_expr & u32::from(MapPart::KEY_TYPES) != 0 {
            let remaining = outer_remaining;
            let (types_rtrn, remaining) = crate::x11_utils::parse_list::<KeyType>(remaining, n_types.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(types_rtrn)
        } else {
            None
        };
        let syms_rtrn = if switch_expr & u32::from(MapPart::KEY_SYMS) != 0 {
            let remaining = outer_remaining;
            let (syms_rtrn, remaining) = crate::x11_utils::parse_list::<KeySymMap>(remaining, n_key_syms.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(syms_rtrn)
        } else {
            None
        };
        let bitcase3 = if switch_expr & u32::from(MapPart::KEY_ACTIONS) != 0 {
            let (bitcase3, new_remaining) = GetMapMapBitcase3::try_parse(outer_remaining, n_key_actions, total_actions)?;
            outer_remaining = new_remaining;
            Some(bitcase3)
        } else {
            None
        };
        let behaviors_rtrn = if switch_expr & u32::from(MapPart::KEY_BEHAVIORS) != 0 {
            let remaining = outer_remaining;
            let (behaviors_rtrn, remaining) = crate::x11_utils::parse_list::<SetBehavior>(remaining, total_key_behaviors.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(behaviors_rtrn)
        } else {
            None
        };
        let vmods_rtrn = if switch_expr & u32::from(MapPart::VIRTUAL_MODS) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (vmods_rtrn, remaining) = crate::x11_utils::parse_u8_list(remaining, virtual_mods.count_ones().try_to_usize()?)?;
            let vmods_rtrn = vmods_rtrn.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            Some(vmods_rtrn)
        } else {
            None
        };
        let explicit_rtrn = if switch_expr & u32::from(MapPart::EXPLICIT_COMPONENTS) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (explicit_rtrn, remaining) = crate::x11_utils::parse_list::<SetExplicit>(remaining, total_key_explicit.try_to_usize()?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            Some(explicit_rtrn)
        } else {
            None
        };
        let modmap_rtrn = if switch_expr & u32::from(MapPart::MODIFIER_MAP) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (modmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyModMap>(remaining, total_mod_map_keys.try_to_usize()?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            Some(modmap_rtrn)
        } else {
            None
        };
        let vmodmap_rtrn = if switch_expr & u32::from(MapPart::VIRTUAL_MOD_MAP) != 0 {
            let remaining = outer_remaining;
            let (vmodmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyVModMap>(remaining, total_v_mod_map_keys.try_to_usize()?)?;
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (map, remaining) = GetMapMap::try_parse(remaining, present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMapReply { device_id, sequence, length, min_key_code, max_key_code, first_type, n_types, total_types, first_key_sym, total_syms, n_key_syms, first_key_action, total_actions, n_key_actions, first_key_behavior, n_key_behaviors, total_key_behaviors, first_key_explicit, n_key_explicit, total_key_explicit, first_mod_map_key, n_mod_map_keys, total_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, total_v_mod_map_keys, virtual_mods, map };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

#[derive(Debug, Clone)]
pub struct SetMapAuxBitcase3 {
    pub actions_count: Vec<u8>,
    pub actions: Vec<Action>,
}
impl SetMapAuxBitcase3 {
    pub fn try_parse(remaining: &[u8], n_key_actions: u8, total_actions: u16) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (actions_count, remaining) = crate::x11_utils::parse_u8_list(remaining, n_key_actions.try_to_usize()?)?;
        let actions_count = actions_count.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (actions, remaining) = crate::x11_utils::parse_list::<Action>(remaining, total_actions.try_to_usize()?)?;
        let result = SetMapAuxBitcase3 { actions_count, actions };
        Ok((result, remaining))
    }
}
impl SetMapAuxBitcase3 {
    #[allow(dead_code)]
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
impl SetMapAux {
    fn try_parse(value: &[u8], present: u16, n_types: u8, n_key_syms: u8, n_key_actions: u8, total_actions: u16, total_key_behaviors: u8, virtual_mods: u16, total_key_explicit: u8, total_mod_map_keys: u8, total_v_mod_map_keys: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = u32::from(present);
        let mut outer_remaining = value;
        let types = if switch_expr & u32::from(MapPart::KEY_TYPES) != 0 {
            let remaining = outer_remaining;
            let (types, remaining) = crate::x11_utils::parse_list::<SetKeyType>(remaining, n_types.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(types)
        } else {
            None
        };
        let syms = if switch_expr & u32::from(MapPart::KEY_SYMS) != 0 {
            let remaining = outer_remaining;
            let (syms, remaining) = crate::x11_utils::parse_list::<KeySymMap>(remaining, n_key_syms.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(syms)
        } else {
            None
        };
        let bitcase3 = if switch_expr & u32::from(MapPart::KEY_ACTIONS) != 0 {
            let (bitcase3, new_remaining) = SetMapAuxBitcase3::try_parse(outer_remaining, n_key_actions, total_actions)?;
            outer_remaining = new_remaining;
            Some(bitcase3)
        } else {
            None
        };
        let behaviors = if switch_expr & u32::from(MapPart::KEY_BEHAVIORS) != 0 {
            let remaining = outer_remaining;
            let (behaviors, remaining) = crate::x11_utils::parse_list::<SetBehavior>(remaining, total_key_behaviors.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(behaviors)
        } else {
            None
        };
        let vmods = if switch_expr & u32::from(MapPart::VIRTUAL_MODS) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (vmods, remaining) = crate::x11_utils::parse_u8_list(remaining, virtual_mods.count_ones().try_to_usize()?)?;
            let vmods = vmods.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            Some(vmods)
        } else {
            None
        };
        let explicit = if switch_expr & u32::from(MapPart::EXPLICIT_COMPONENTS) != 0 {
            let remaining = outer_remaining;
            let (explicit, remaining) = crate::x11_utils::parse_list::<SetExplicit>(remaining, total_key_explicit.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(explicit)
        } else {
            None
        };
        let modmap = if switch_expr & u32::from(MapPart::MODIFIER_MAP) != 0 {
            let remaining = outer_remaining;
            let (modmap, remaining) = crate::x11_utils::parse_list::<KeyModMap>(remaining, total_mod_map_keys.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(modmap)
        } else {
            None
        };
        let vmodmap = if switch_expr & u32::from(MapPart::VIRTUAL_MOD_MAP) != 0 {
            let remaining = outer_remaining;
            let (vmodmap, remaining) = crate::x11_utils::parse_list::<KeyVModMap>(remaining, total_v_mod_map_keys.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(vmodmap)
        } else {
            None
        };
        let result = SetMapAux { types, syms, bitcase3, behaviors, vmods, explicit, modmap, vmodmap };
        Ok((result, outer_remaining))
    }
}
impl SetMapAux {
    #[allow(dead_code)]
    fn serialize(&self, present: u16, n_types: u8, n_key_syms: u8, n_key_actions: u8, total_actions: u16, total_key_behaviors: u8, virtual_mods: u16, total_key_explicit: u8, total_mod_map_keys: u8, total_v_mod_map_keys: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, present: u16, n_types: u8, n_key_syms: u8, n_key_actions: u8, total_actions: u16, total_key_behaviors: u8, virtual_mods: u16, total_key_explicit: u8, total_mod_map_keys: u8, total_v_mod_map_keys: u8) {
        assert_eq!(self.switch_expr(), u32::from(present), "switch `values` has an inconsistent discriminant");
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
            expr_value |= u32::from(MapPart::KEY_TYPES);
        }
        if self.syms.is_some() {
            expr_value |= u32::from(MapPart::KEY_SYMS);
        }
        if self.bitcase3.is_some() {
            expr_value |= u32::from(MapPart::KEY_ACTIONS);
        }
        if self.behaviors.is_some() {
            expr_value |= u32::from(MapPart::KEY_BEHAVIORS);
        }
        if self.vmods.is_some() {
            expr_value |= u32::from(MapPart::VIRTUAL_MODS);
        }
        if self.explicit.is_some() {
            expr_value |= u32::from(MapPart::EXPLICIT_COMPONENTS);
        }
        if self.modmap.is_some() {
            expr_value |= u32::from(MapPart::MODIFIER_MAP);
        }
        if self.vmodmap.is_some() {
            expr_value |= u32::from(MapPart::VIRTUAL_MOD_MAP);
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

/// Opcode for the SetMap request
pub const SET_MAP_REQUEST: u8 = 9;
#[derive(Debug, Clone)]
pub struct SetMapRequest<'input> {
    pub device_spec: DeviceSpec,
    pub flags: u16,
    pub min_key_code: xproto::Keycode,
    pub max_key_code: xproto::Keycode,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xproto::Keycode,
    pub n_key_syms: u8,
    pub total_syms: u16,
    pub first_key_action: xproto::Keycode,
    pub n_key_actions: u8,
    pub total_actions: u16,
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
    pub values: Cow<'input, SetMapAux>,
}
impl<'input> SetMapRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let present = u16::try_from(self.values.switch_expr()).unwrap();
        let present_bytes = present.serialize();
        let flags_bytes = self.flags.serialize();
        let min_key_code_bytes = self.min_key_code.serialize();
        let max_key_code_bytes = self.max_key_code.serialize();
        let first_type_bytes = self.first_type.serialize();
        let n_types_bytes = self.n_types.serialize();
        let first_key_sym_bytes = self.first_key_sym.serialize();
        let n_key_syms_bytes = self.n_key_syms.serialize();
        let total_syms_bytes = self.total_syms.serialize();
        let first_key_action_bytes = self.first_key_action.serialize();
        let n_key_actions_bytes = self.n_key_actions.serialize();
        let total_actions_bytes = self.total_actions.serialize();
        let first_key_behavior_bytes = self.first_key_behavior.serialize();
        let n_key_behaviors_bytes = self.n_key_behaviors.serialize();
        let total_key_behaviors_bytes = self.total_key_behaviors.serialize();
        let first_key_explicit_bytes = self.first_key_explicit.serialize();
        let n_key_explicit_bytes = self.n_key_explicit.serialize();
        let total_key_explicit_bytes = self.total_key_explicit.serialize();
        let first_mod_map_key_bytes = self.first_mod_map_key.serialize();
        let n_mod_map_keys_bytes = self.n_mod_map_keys.serialize();
        let total_mod_map_keys_bytes = self.total_mod_map_keys.serialize();
        let first_v_mod_map_key_bytes = self.first_v_mod_map_key.serialize();
        let n_v_mod_map_keys_bytes = self.n_v_mod_map_keys.serialize();
        let total_v_mod_map_keys_bytes = self.total_v_mod_map_keys.serialize();
        let virtual_mods_bytes = self.virtual_mods.serialize();
        let mut request0 = vec![
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
        let values_bytes = self.values.serialize(present, self.n_types, self.n_key_syms, self.n_key_actions, self.total_actions, self.total_key_behaviors, self.virtual_mods, self.total_key_explicit, self.total_mod_map_keys, self.total_v_mod_map_keys);
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_MAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (present, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u16::try_parse(remaining)?;
        let (min_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (max_key_code, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (first_key_sym, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_syms, remaining) = u8::try_parse(remaining)?;
        let (total_syms, remaining) = u16::try_parse(remaining)?;
        let (first_key_action, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_key_actions, remaining) = u8::try_parse(remaining)?;
        let (total_actions, remaining) = u16::try_parse(remaining)?;
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
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (values, remaining) = SetMapAux::try_parse(remaining, present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys)?;
        let _ = remaining;
        Ok(SetMapRequest {
            device_spec,
            flags,
            min_key_code,
            max_key_code,
            first_type,
            n_types,
            first_key_sym,
            n_key_syms,
            total_syms,
            first_key_action,
            n_key_actions,
            total_actions,
            first_key_behavior,
            n_key_behaviors,
            total_key_behaviors,
            first_key_explicit,
            n_key_explicit,
            total_key_explicit,
            first_mod_map_key,
            n_mod_map_keys,
            total_mod_map_keys,
            first_v_mod_map_key,
            n_v_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
            values: Cow::Owned(values),
        })
    }
    /// Clone all borrowed data in this SetMapRequest.
    pub fn into_owned(self) -> SetMapRequest<'static> {
        SetMapRequest {
            device_spec: self.device_spec,
            flags: self.flags,
            min_key_code: self.min_key_code,
            max_key_code: self.max_key_code,
            first_type: self.first_type,
            n_types: self.n_types,
            first_key_sym: self.first_key_sym,
            n_key_syms: self.n_key_syms,
            total_syms: self.total_syms,
            first_key_action: self.first_key_action,
            n_key_actions: self.n_key_actions,
            total_actions: self.total_actions,
            first_key_behavior: self.first_key_behavior,
            n_key_behaviors: self.n_key_behaviors,
            total_key_behaviors: self.total_key_behaviors,
            first_key_explicit: self.first_key_explicit,
            n_key_explicit: self.n_key_explicit,
            total_key_explicit: self.total_key_explicit,
            first_mod_map_key: self.first_mod_map_key,
            n_mod_map_keys: self.n_mod_map_keys,
            total_mod_map_keys: self.total_mod_map_keys,
            first_v_mod_map_key: self.first_v_mod_map_key,
            n_v_mod_map_keys: self.n_v_mod_map_keys,
            total_v_mod_map_keys: self.total_v_mod_map_keys,
            virtual_mods: self.virtual_mods,
            values: Cow::Owned(self.values.into_owned()),
        }
    }
}
impl<'input> Request for SetMapRequest<'input> {
    type Reply = ();
}
pub fn set_map<'c, 'input, Conn, A, B>(conn: &'c Conn, device_spec: DeviceSpec, flags: A, min_key_code: xproto::Keycode, max_key_code: xproto::Keycode, first_type: u8, n_types: u8, first_key_sym: xproto::Keycode, n_key_syms: u8, total_syms: u16, first_key_action: xproto::Keycode, n_key_actions: u8, total_actions: u16, first_key_behavior: xproto::Keycode, n_key_behaviors: u8, total_key_behaviors: u8, first_key_explicit: xproto::Keycode, n_key_explicit: u8, total_key_explicit: u8, first_mod_map_key: xproto::Keycode, n_mod_map_keys: u8, total_mod_map_keys: u8, first_v_mod_map_key: xproto::Keycode, n_v_mod_map_keys: u8, total_v_mod_map_keys: u8, virtual_mods: B, values: &'input SetMapAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u16>,
{
    let flags: u16 = flags.into();
    let virtual_mods: u16 = virtual_mods.into();
    let request0 = SetMapRequest {
        device_spec,
        flags,
        min_key_code,
        max_key_code,
        first_type,
        n_types,
        first_key_sym,
        n_key_syms,
        total_syms,
        first_key_action,
        n_key_actions,
        total_actions,
        first_key_behavior,
        n_key_behaviors,
        total_key_behaviors,
        first_key_explicit,
        n_key_explicit,
        total_key_explicit,
        first_mod_map_key,
        n_mod_map_keys,
        total_mod_map_keys,
        first_v_mod_map_key,
        n_v_mod_map_keys,
        total_v_mod_map_keys,
        virtual_mods,
        values: Cow::Borrowed(values),
    };
    request0.send(conn)
}

/// Opcode for the GetCompatMap request
pub const GET_COMPAT_MAP_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCompatMapRequest {
    pub device_spec: DeviceSpec,
    pub groups: u8,
    pub get_all_si: bool,
    pub first_si: u16,
    pub n_si: u16,
}
impl GetCompatMapRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let groups_bytes = self.groups.serialize();
        let get_all_si_bytes = self.get_all_si.serialize();
        let first_si_bytes = self.first_si.serialize();
        let n_si_bytes = self.n_si.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetCompatMapReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_COMPAT_MAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (groups, remaining) = u8::try_parse(remaining)?;
        let (get_all_si, remaining) = bool::try_parse(remaining)?;
        let (first_si, remaining) = u16::try_parse(remaining)?;
        let (n_si, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetCompatMapRequest {
            device_spec,
            groups,
            get_all_si,
            first_si,
            n_si,
        })
    }
}
impl Request for GetCompatMapRequest {
    type Reply = GetCompatMapReply;
}
pub fn get_compat_map<Conn, A>(conn: &Conn, device_spec: DeviceSpec, groups: A, get_all_si: bool, first_si: u16, n_si: u16) -> Result<Cookie<'_, Conn, GetCompatMapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let groups: u8 = groups.into();
    let request0 = GetCompatMapRequest {
        device_spec,
        groups,
        get_all_si,
        first_si,
        n_si,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCompatMapReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (groups_rtrn, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (first_si_rtrn, remaining) = u16::try_parse(remaining)?;
        let (n_si_rtrn, remaining) = u16::try_parse(remaining)?;
        let (n_total_si, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (si_rtrn, remaining) = crate::x11_utils::parse_list::<SymInterpret>(remaining, n_si_rtrn.try_to_usize()?)?;
        let (group_rtrn, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, groups_rtrn.count_ones().try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetCompatMapReply { device_id, sequence, length, groups_rtrn, first_si_rtrn, n_total_si, si_rtrn, group_rtrn };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCompatMapRequest<'input> {
    pub device_spec: DeviceSpec,
    pub recompute_actions: bool,
    pub truncate_si: bool,
    pub groups: u8,
    pub first_si: u16,
    pub si: Cow<'input, [SymInterpret]>,
    pub group_maps: Cow<'input, [ModDef]>,
}
impl<'input> SetCompatMapRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let recompute_actions_bytes = self.recompute_actions.serialize();
        let truncate_si_bytes = self.truncate_si.serialize();
        let groups_bytes = self.groups.serialize();
        let first_si_bytes = self.first_si.serialize();
        let n_si = u16::try_from(self.si.len()).expect("`si` has too many elements");
        let n_si_bytes = n_si.serialize();
        let mut request0 = vec![
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
        let si_bytes = self.si.serialize();
        let length_so_far = length_so_far + si_bytes.len();
        assert_eq!(self.group_maps.len(), usize::try_from(self.groups.count_ones()).unwrap(), "`group_maps` has an incorrect length");
        let group_maps_bytes = self.group_maps.serialize();
        let length_so_far = length_so_far + group_maps_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), si_bytes.into(), group_maps_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_COMPAT_MAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (recompute_actions, remaining) = bool::try_parse(remaining)?;
        let (truncate_si, remaining) = bool::try_parse(remaining)?;
        let (groups, remaining) = u8::try_parse(remaining)?;
        let (first_si, remaining) = u16::try_parse(remaining)?;
        let (n_si, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (si, remaining) = crate::x11_utils::parse_list::<SymInterpret>(remaining, n_si.try_to_usize()?)?;
        let (group_maps, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, groups.count_ones().try_to_usize()?)?;
        let _ = remaining;
        Ok(SetCompatMapRequest {
            device_spec,
            recompute_actions,
            truncate_si,
            groups,
            first_si,
            si: Cow::Owned(si),
            group_maps: Cow::Owned(group_maps),
        })
    }
    /// Clone all borrowed data in this SetCompatMapRequest.
    pub fn into_owned(self) -> SetCompatMapRequest<'static> {
        SetCompatMapRequest {
            device_spec: self.device_spec,
            recompute_actions: self.recompute_actions,
            truncate_si: self.truncate_si,
            groups: self.groups,
            first_si: self.first_si,
            si: Cow::Owned(self.si.into_owned()),
            group_maps: Cow::Owned(self.group_maps.into_owned()),
        }
    }
}
impl<'input> Request for SetCompatMapRequest<'input> {
    type Reply = ();
}
pub fn set_compat_map<'c, 'input, Conn, A>(conn: &'c Conn, device_spec: DeviceSpec, recompute_actions: bool, truncate_si: bool, groups: A, first_si: u16, si: &'input [SymInterpret], group_maps: &'input [ModDef]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let groups: u8 = groups.into();
    let request0 = SetCompatMapRequest {
        device_spec,
        recompute_actions,
        truncate_si,
        groups,
        first_si,
        si: Cow::Borrowed(si),
        group_maps: Cow::Borrowed(group_maps),
    };
    request0.send(conn)
}

/// Opcode for the GetIndicatorState request
pub const GET_INDICATOR_STATE_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetIndicatorStateRequest {
    pub device_spec: DeviceSpec,
}
impl GetIndicatorStateRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetIndicatorStateReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_INDICATOR_STATE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetIndicatorStateRequest {
            device_spec,
        })
    }
}
impl Request for GetIndicatorStateRequest {
    type Reply = GetIndicatorStateReply;
}
pub fn get_indicator_state<Conn>(conn: &Conn, device_spec: DeviceSpec) -> Result<Cookie<'_, Conn, GetIndicatorStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetIndicatorStateRequest {
        device_spec,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetIndicatorStateReply {
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}
impl TryParse for GetIndicatorStateReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetIndicatorStateReply { device_id, sequence, length, state };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the GetIndicatorMap request
pub const GET_INDICATOR_MAP_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetIndicatorMapRequest {
    pub device_spec: DeviceSpec,
    pub which: u32,
}
impl GetIndicatorMapRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let which_bytes = self.which.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetIndicatorMapReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_INDICATOR_MAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetIndicatorMapRequest {
            device_spec,
            which,
        })
    }
}
impl Request for GetIndicatorMapRequest {
    type Reply = GetIndicatorMapReply;
}
pub fn get_indicator_map<Conn>(conn: &Conn, device_spec: DeviceSpec, which: u32) -> Result<Cookie<'_, Conn, GetIndicatorMapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetIndicatorMapRequest {
        device_spec,
        which,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetIndicatorMapReply {
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub which: u32,
    pub real_indicators: u32,
    pub n_indicators: u8,
    pub maps: Vec<IndicatorMap>,
}
impl TryParse for GetIndicatorMapReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (real_indicators, remaining) = u32::try_parse(remaining)?;
        let (n_indicators, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::InsufficientData)?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, which.count_ones().try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetIndicatorMapReply { device_id, sequence, length, which, real_indicators, n_indicators, maps };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetIndicatorMap request
pub const SET_INDICATOR_MAP_REQUEST: u8 = 14;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetIndicatorMapRequest<'input> {
    pub device_spec: DeviceSpec,
    pub which: u32,
    pub maps: Cow<'input, [IndicatorMap]>,
}
impl<'input> SetIndicatorMapRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let which_bytes = self.which.serialize();
        let mut request0 = vec![
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
        assert_eq!(self.maps.len(), usize::try_from(self.which.count_ones()).unwrap(), "`maps` has an incorrect length");
        let maps_bytes = self.maps.serialize();
        let length_so_far = length_so_far + maps_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), maps_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_INDICATOR_MAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, which.count_ones().try_to_usize()?)?;
        let _ = remaining;
        Ok(SetIndicatorMapRequest {
            device_spec,
            which,
            maps: Cow::Owned(maps),
        })
    }
    /// Clone all borrowed data in this SetIndicatorMapRequest.
    pub fn into_owned(self) -> SetIndicatorMapRequest<'static> {
        SetIndicatorMapRequest {
            device_spec: self.device_spec,
            which: self.which,
            maps: Cow::Owned(self.maps.into_owned()),
        }
    }
}
impl<'input> Request for SetIndicatorMapRequest<'input> {
    type Reply = ();
}
pub fn set_indicator_map<'c, 'input, Conn>(conn: &'c Conn, device_spec: DeviceSpec, which: u32, maps: &'input [IndicatorMap]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetIndicatorMapRequest {
        device_spec,
        which,
        maps: Cow::Borrowed(maps),
    };
    request0.send(conn)
}

/// Opcode for the GetNamedIndicator request
pub const GET_NAMED_INDICATOR_REQUEST: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetNamedIndicatorRequest {
    pub device_spec: DeviceSpec,
    pub led_class: LedClass,
    pub led_id: IDSpec,
    pub indicator: xproto::Atom,
}
impl GetNamedIndicatorRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let led_class_bytes = LedClassSpec::from(self.led_class).serialize();
        let led_id_bytes = self.led_id.serialize();
        let indicator_bytes = self.indicator.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetNamedIndicatorReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_NAMED_INDICATOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (led_class, remaining) = LedClassSpec::try_parse(remaining)?;
        let led_class = led_class.into();
        let (led_id, remaining) = IDSpec::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (indicator, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetNamedIndicatorRequest {
            device_spec,
            led_class,
            led_id,
            indicator,
        })
    }
}
impl Request for GetNamedIndicatorRequest {
    type Reply = GetNamedIndicatorReply;
}
pub fn get_named_indicator<Conn, A>(conn: &Conn, device_spec: DeviceSpec, led_class: LedClass, led_id: A, indicator: xproto::Atom) -> Result<Cookie<'_, Conn, GetNamedIndicatorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<IDSpec>,
{
    let led_id: IDSpec = led_id.into();
    let request0 = GetNamedIndicatorRequest {
        device_spec,
        led_class,
        led_id,
        indicator,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetNamedIndicatorReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetNamedIndicatorReply { device_id, sequence, length, indicator, found, on, real_indicator, ndx, map_flags, map_which_groups, map_groups, map_which_mods, map_mods, map_real_mods, map_vmod, map_ctrls, supported };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetNamedIndicator request
pub const SET_NAMED_INDICATOR_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetNamedIndicatorRequest {
    pub device_spec: DeviceSpec,
    pub led_class: LedClass,
    pub led_id: IDSpec,
    pub indicator: xproto::Atom,
    pub set_state: bool,
    pub on: bool,
    pub set_map: bool,
    pub create_map: bool,
    pub map_flags: u8,
    pub map_which_groups: u8,
    pub map_groups: u8,
    pub map_which_mods: u8,
    pub map_real_mods: u8,
    pub map_vmods: u16,
    pub map_ctrls: u32,
}
impl SetNamedIndicatorRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let led_class_bytes = LedClassSpec::from(self.led_class).serialize();
        let led_id_bytes = self.led_id.serialize();
        let indicator_bytes = self.indicator.serialize();
        let set_state_bytes = self.set_state.serialize();
        let on_bytes = self.on.serialize();
        let set_map_bytes = self.set_map.serialize();
        let create_map_bytes = self.create_map.serialize();
        let map_flags_bytes = self.map_flags.serialize();
        let map_which_groups_bytes = self.map_which_groups.serialize();
        let map_groups_bytes = self.map_groups.serialize();
        let map_which_mods_bytes = self.map_which_mods.serialize();
        let map_real_mods_bytes = self.map_real_mods.serialize();
        let map_vmods_bytes = self.map_vmods.serialize();
        let map_ctrls_bytes = self.map_ctrls.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_NAMED_INDICATOR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (led_class, remaining) = LedClassSpec::try_parse(remaining)?;
        let led_class = led_class.into();
        let (led_id, remaining) = IDSpec::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (indicator, remaining) = xproto::Atom::try_parse(remaining)?;
        let (set_state, remaining) = bool::try_parse(remaining)?;
        let (on, remaining) = bool::try_parse(remaining)?;
        let (set_map, remaining) = bool::try_parse(remaining)?;
        let (create_map, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (map_flags, remaining) = u8::try_parse(remaining)?;
        let (map_which_groups, remaining) = u8::try_parse(remaining)?;
        let (map_groups, remaining) = u8::try_parse(remaining)?;
        let (map_which_mods, remaining) = u8::try_parse(remaining)?;
        let (map_real_mods, remaining) = u8::try_parse(remaining)?;
        let (map_vmods, remaining) = u16::try_parse(remaining)?;
        let (map_ctrls, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetNamedIndicatorRequest {
            device_spec,
            led_class,
            led_id,
            indicator,
            set_state,
            on,
            set_map,
            create_map,
            map_flags,
            map_which_groups,
            map_groups,
            map_which_mods,
            map_real_mods,
            map_vmods,
            map_ctrls,
        })
    }
}
impl Request for SetNamedIndicatorRequest {
    type Reply = ();
}
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
    let led_id: IDSpec = led_id.into();
    let map_flags: u8 = map_flags.into();
    let map_which_groups: u8 = map_which_groups.into();
    let map_groups: u8 = map_groups.into();
    let map_which_mods: u8 = map_which_mods.into();
    let map_real_mods: u8 = map_real_mods.into();
    let map_vmods: u16 = map_vmods.into();
    let map_ctrls: u32 = map_ctrls.into();
    let request0 = SetNamedIndicatorRequest {
        device_spec,
        led_class,
        led_id,
        indicator,
        set_state,
        on,
        set_map,
        create_map,
        map_flags,
        map_which_groups,
        map_groups,
        map_which_mods,
        map_real_mods,
        map_vmods,
        map_ctrls,
    };
    request0.send(conn)
}

/// Opcode for the GetNames request
pub const GET_NAMES_REQUEST: u8 = 17;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetNamesRequest {
    pub device_spec: DeviceSpec,
    pub which: u32,
}
impl GetNamesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let which_bytes = self.which.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetNamesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_NAMES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetNamesRequest {
            device_spec,
            which,
        })
    }
}
impl Request for GetNamesRequest {
    type Reply = GetNamesReply;
}
pub fn get_names<Conn, A>(conn: &Conn, device_spec: DeviceSpec, which: A) -> Result<Cookie<'_, Conn, GetNamesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let which: u32 = which.into();
    let request0 = GetNamesRequest {
        device_spec,
        which,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetNamesValueListBitcase8 {
    pub n_levels_per_type: Vec<u8>,
    pub kt_level_names: Vec<xproto::Atom>,
}
impl GetNamesValueListBitcase8 {
    pub fn try_parse(remaining: &[u8], n_types: u8) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (n_levels_per_type, remaining) = crate::x11_utils::parse_u8_list(remaining, n_types.try_to_usize()?)?;
        let n_levels_per_type = n_levels_per_type.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (kt_level_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_levels_per_type.iter().try_fold(0u32, |acc, x| acc.checked_add(u32::from(*x)).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = GetNamesValueListBitcase8 { n_levels_per_type, kt_level_names };
        Ok((result, remaining))
    }
}
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
        let keycodes_name = if switch_expr & u32::from(NameDetail::KEYCODES) != 0 {
            let remaining = outer_remaining;
            let (keycodes_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(keycodes_name)
        } else {
            None
        };
        let geometry_name = if switch_expr & u32::from(NameDetail::GEOMETRY) != 0 {
            let remaining = outer_remaining;
            let (geometry_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(geometry_name)
        } else {
            None
        };
        let symbols_name = if switch_expr & u32::from(NameDetail::SYMBOLS) != 0 {
            let remaining = outer_remaining;
            let (symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(symbols_name)
        } else {
            None
        };
        let phys_symbols_name = if switch_expr & u32::from(NameDetail::PHYS_SYMBOLS) != 0 {
            let remaining = outer_remaining;
            let (phys_symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(phys_symbols_name)
        } else {
            None
        };
        let types_name = if switch_expr & u32::from(NameDetail::TYPES) != 0 {
            let remaining = outer_remaining;
            let (types_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(types_name)
        } else {
            None
        };
        let compat_name = if switch_expr & u32::from(NameDetail::COMPAT) != 0 {
            let remaining = outer_remaining;
            let (compat_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(compat_name)
        } else {
            None
        };
        let type_names = if switch_expr & u32::from(NameDetail::KEY_TYPE_NAMES) != 0 {
            let remaining = outer_remaining;
            let (type_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_types.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(type_names)
        } else {
            None
        };
        let bitcase8 = if switch_expr & u32::from(NameDetail::KT_LEVEL_NAMES) != 0 {
            let (bitcase8, new_remaining) = GetNamesValueListBitcase8::try_parse(outer_remaining, n_types)?;
            outer_remaining = new_remaining;
            Some(bitcase8)
        } else {
            None
        };
        let indicator_names = if switch_expr & u32::from(NameDetail::INDICATOR_NAMES) != 0 {
            let remaining = outer_remaining;
            let (indicator_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, indicators.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(indicator_names)
        } else {
            None
        };
        let virtual_mod_names = if switch_expr & u32::from(NameDetail::VIRTUAL_MOD_NAMES) != 0 {
            let remaining = outer_remaining;
            let (virtual_mod_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, virtual_mods.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(virtual_mod_names)
        } else {
            None
        };
        let groups = if switch_expr & u32::from(NameDetail::GROUP_NAMES) != 0 {
            let remaining = outer_remaining;
            let (groups, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, group_names.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(groups)
        } else {
            None
        };
        let key_names = if switch_expr & u32::from(NameDetail::KEY_NAMES) != 0 {
            let remaining = outer_remaining;
            let (key_names, remaining) = crate::x11_utils::parse_list::<KeyName>(remaining, n_keys.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(key_names)
        } else {
            None
        };
        let key_aliases = if switch_expr & u32::from(NameDetail::KEY_ALIASES) != 0 {
            let remaining = outer_remaining;
            let (key_aliases, remaining) = crate::x11_utils::parse_list::<KeyAlias>(remaining, n_key_aliases.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(key_aliases)
        } else {
            None
        };
        let radio_group_names = if switch_expr & u32::from(NameDetail::RG_NAMES) != 0 {
            let remaining = outer_remaining;
            let (radio_group_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_radio_groups.try_to_usize()?)?;
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (value_list, remaining) = GetNamesValueList::try_parse(remaining, which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetNamesReply { device_id, sequence, length, min_key_code, max_key_code, n_types, group_names, virtual_mods, first_key, n_keys, indicators, n_radio_groups, n_key_aliases, n_kt_levels, value_list };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetNamesAuxBitcase8 {
    pub n_levels_per_type: Vec<u8>,
    pub kt_level_names: Vec<xproto::Atom>,
}
impl SetNamesAuxBitcase8 {
    pub fn try_parse(remaining: &[u8], n_types: u8) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (n_levels_per_type, remaining) = crate::x11_utils::parse_u8_list(remaining, n_types.try_to_usize()?)?;
        let n_levels_per_type = n_levels_per_type.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (kt_level_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_levels_per_type.iter().try_fold(0u32, |acc, x| acc.checked_add(u32::from(*x)).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = SetNamesAuxBitcase8 { n_levels_per_type, kt_level_names };
        Ok((result, remaining))
    }
}
impl SetNamesAuxBitcase8 {
    #[allow(dead_code)]
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
impl SetNamesAux {
    fn try_parse(value: &[u8], which: u32, n_types: u8, indicators: u32, virtual_mods: u16, group_names: u8, n_keys: u8, n_key_aliases: u8, n_radio_groups: u8) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = which;
        let mut outer_remaining = value;
        let keycodes_name = if switch_expr & u32::from(NameDetail::KEYCODES) != 0 {
            let remaining = outer_remaining;
            let (keycodes_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(keycodes_name)
        } else {
            None
        };
        let geometry_name = if switch_expr & u32::from(NameDetail::GEOMETRY) != 0 {
            let remaining = outer_remaining;
            let (geometry_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(geometry_name)
        } else {
            None
        };
        let symbols_name = if switch_expr & u32::from(NameDetail::SYMBOLS) != 0 {
            let remaining = outer_remaining;
            let (symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(symbols_name)
        } else {
            None
        };
        let phys_symbols_name = if switch_expr & u32::from(NameDetail::PHYS_SYMBOLS) != 0 {
            let remaining = outer_remaining;
            let (phys_symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(phys_symbols_name)
        } else {
            None
        };
        let types_name = if switch_expr & u32::from(NameDetail::TYPES) != 0 {
            let remaining = outer_remaining;
            let (types_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(types_name)
        } else {
            None
        };
        let compat_name = if switch_expr & u32::from(NameDetail::COMPAT) != 0 {
            let remaining = outer_remaining;
            let (compat_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(compat_name)
        } else {
            None
        };
        let type_names = if switch_expr & u32::from(NameDetail::KEY_TYPE_NAMES) != 0 {
            let remaining = outer_remaining;
            let (type_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_types.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(type_names)
        } else {
            None
        };
        let bitcase8 = if switch_expr & u32::from(NameDetail::KT_LEVEL_NAMES) != 0 {
            let (bitcase8, new_remaining) = SetNamesAuxBitcase8::try_parse(outer_remaining, n_types)?;
            outer_remaining = new_remaining;
            Some(bitcase8)
        } else {
            None
        };
        let indicator_names = if switch_expr & u32::from(NameDetail::INDICATOR_NAMES) != 0 {
            let remaining = outer_remaining;
            let (indicator_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, indicators.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(indicator_names)
        } else {
            None
        };
        let virtual_mod_names = if switch_expr & u32::from(NameDetail::VIRTUAL_MOD_NAMES) != 0 {
            let remaining = outer_remaining;
            let (virtual_mod_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, virtual_mods.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(virtual_mod_names)
        } else {
            None
        };
        let groups = if switch_expr & u32::from(NameDetail::GROUP_NAMES) != 0 {
            let remaining = outer_remaining;
            let (groups, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, group_names.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(groups)
        } else {
            None
        };
        let key_names = if switch_expr & u32::from(NameDetail::KEY_NAMES) != 0 {
            let remaining = outer_remaining;
            let (key_names, remaining) = crate::x11_utils::parse_list::<KeyName>(remaining, n_keys.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(key_names)
        } else {
            None
        };
        let key_aliases = if switch_expr & u32::from(NameDetail::KEY_ALIASES) != 0 {
            let remaining = outer_remaining;
            let (key_aliases, remaining) = crate::x11_utils::parse_list::<KeyAlias>(remaining, n_key_aliases.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(key_aliases)
        } else {
            None
        };
        let radio_group_names = if switch_expr & u32::from(NameDetail::RG_NAMES) != 0 {
            let remaining = outer_remaining;
            let (radio_group_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_radio_groups.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(radio_group_names)
        } else {
            None
        };
        let result = SetNamesAux { keycodes_name, geometry_name, symbols_name, phys_symbols_name, types_name, compat_name, type_names, bitcase8, indicator_names, virtual_mod_names, groups, key_names, key_aliases, radio_group_names };
        Ok((result, outer_remaining))
    }
}
impl SetNamesAux {
    #[allow(dead_code)]
    fn serialize(&self, which: u32, n_types: u8, indicators: u32, virtual_mods: u16, group_names: u8, n_keys: u8, n_key_aliases: u8, n_radio_groups: u8) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, which: u32, n_types: u8, indicators: u32, virtual_mods: u16, group_names: u8, n_keys: u8, n_key_aliases: u8, n_radio_groups: u8) {
        assert_eq!(self.switch_expr(), which, "switch `values` has an inconsistent discriminant");
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
            expr_value |= u32::from(NameDetail::KEYCODES);
        }
        if self.geometry_name.is_some() {
            expr_value |= u32::from(NameDetail::GEOMETRY);
        }
        if self.symbols_name.is_some() {
            expr_value |= u32::from(NameDetail::SYMBOLS);
        }
        if self.phys_symbols_name.is_some() {
            expr_value |= u32::from(NameDetail::PHYS_SYMBOLS);
        }
        if self.types_name.is_some() {
            expr_value |= u32::from(NameDetail::TYPES);
        }
        if self.compat_name.is_some() {
            expr_value |= u32::from(NameDetail::COMPAT);
        }
        if self.type_names.is_some() {
            expr_value |= u32::from(NameDetail::KEY_TYPE_NAMES);
        }
        if self.bitcase8.is_some() {
            expr_value |= u32::from(NameDetail::KT_LEVEL_NAMES);
        }
        if self.indicator_names.is_some() {
            expr_value |= u32::from(NameDetail::INDICATOR_NAMES);
        }
        if self.virtual_mod_names.is_some() {
            expr_value |= u32::from(NameDetail::VIRTUAL_MOD_NAMES);
        }
        if self.groups.is_some() {
            expr_value |= u32::from(NameDetail::GROUP_NAMES);
        }
        if self.key_names.is_some() {
            expr_value |= u32::from(NameDetail::KEY_NAMES);
        }
        if self.key_aliases.is_some() {
            expr_value |= u32::from(NameDetail::KEY_ALIASES);
        }
        if self.radio_group_names.is_some() {
            expr_value |= u32::from(NameDetail::RG_NAMES);
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

/// Opcode for the SetNames request
pub const SET_NAMES_REQUEST: u8 = 18;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetNamesRequest<'input> {
    pub device_spec: DeviceSpec,
    pub virtual_mods: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub first_kt_levelt: u8,
    pub n_kt_levels: u8,
    pub indicators: u32,
    pub group_names: u8,
    pub n_radio_groups: u8,
    pub first_key: xproto::Keycode,
    pub n_keys: u8,
    pub n_key_aliases: u8,
    pub total_kt_level_names: u16,
    pub values: Cow<'input, SetNamesAux>,
}
impl<'input> SetNamesRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let virtual_mods_bytes = self.virtual_mods.serialize();
        let which = self.values.switch_expr();
        let which_bytes = which.serialize();
        let first_type_bytes = self.first_type.serialize();
        let n_types_bytes = self.n_types.serialize();
        let first_kt_levelt_bytes = self.first_kt_levelt.serialize();
        let n_kt_levels_bytes = self.n_kt_levels.serialize();
        let indicators_bytes = self.indicators.serialize();
        let group_names_bytes = self.group_names.serialize();
        let n_radio_groups_bytes = self.n_radio_groups.serialize();
        let first_key_bytes = self.first_key.serialize();
        let n_keys_bytes = self.n_keys.serialize();
        let n_key_aliases_bytes = self.n_key_aliases.serialize();
        let total_kt_level_names_bytes = self.total_kt_level_names.serialize();
        let mut request0 = vec![
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
        let values_bytes = self.values.serialize(which, self.n_types, self.indicators, self.virtual_mods, self.group_names, self.n_keys, self.n_key_aliases, self.n_radio_groups);
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_NAMES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (which, remaining) = u32::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (first_kt_levelt, remaining) = u8::try_parse(remaining)?;
        let (n_kt_levels, remaining) = u8::try_parse(remaining)?;
        let (indicators, remaining) = u32::try_parse(remaining)?;
        let (group_names, remaining) = u8::try_parse(remaining)?;
        let (n_radio_groups, remaining) = u8::try_parse(remaining)?;
        let (first_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_keys, remaining) = u8::try_parse(remaining)?;
        let (n_key_aliases, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (total_kt_level_names, remaining) = u16::try_parse(remaining)?;
        let (values, remaining) = SetNamesAux::try_parse(remaining, which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups)?;
        let _ = remaining;
        Ok(SetNamesRequest {
            device_spec,
            virtual_mods,
            first_type,
            n_types,
            first_kt_levelt,
            n_kt_levels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_kt_level_names,
            values: Cow::Owned(values),
        })
    }
    /// Clone all borrowed data in this SetNamesRequest.
    pub fn into_owned(self) -> SetNamesRequest<'static> {
        SetNamesRequest {
            device_spec: self.device_spec,
            virtual_mods: self.virtual_mods,
            first_type: self.first_type,
            n_types: self.n_types,
            first_kt_levelt: self.first_kt_levelt,
            n_kt_levels: self.n_kt_levels,
            indicators: self.indicators,
            group_names: self.group_names,
            n_radio_groups: self.n_radio_groups,
            first_key: self.first_key,
            n_keys: self.n_keys,
            n_key_aliases: self.n_key_aliases,
            total_kt_level_names: self.total_kt_level_names,
            values: Cow::Owned(self.values.into_owned()),
        }
    }
}
impl<'input> Request for SetNamesRequest<'input> {
    type Reply = ();
}
pub fn set_names<'c, 'input, Conn, A, B>(conn: &'c Conn, device_spec: DeviceSpec, virtual_mods: A, first_type: u8, n_types: u8, first_kt_levelt: u8, n_kt_levels: u8, indicators: u32, group_names: B, n_radio_groups: u8, first_key: xproto::Keycode, n_keys: u8, n_key_aliases: u8, total_kt_level_names: u16, values: &'input SetNamesAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u8>,
{
    let virtual_mods: u16 = virtual_mods.into();
    let group_names: u8 = group_names.into();
    let request0 = SetNamesRequest {
        device_spec,
        virtual_mods,
        first_type,
        n_types,
        first_kt_levelt,
        n_kt_levels,
        indicators,
        group_names,
        n_radio_groups,
        first_key,
        n_keys,
        n_key_aliases,
        total_kt_level_names,
        values: Cow::Borrowed(values),
    };
    request0.send(conn)
}

/// Opcode for the PerClientFlags request
pub const PER_CLIENT_FLAGS_REQUEST: u8 = 21;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerClientFlagsRequest {
    pub device_spec: DeviceSpec,
    pub change: u32,
    pub value: u32,
    pub ctrls_to_change: u32,
    pub auto_ctrls: u32,
    pub auto_ctrls_values: u32,
}
impl PerClientFlagsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let change_bytes = self.change.serialize();
        let value_bytes = self.value.serialize();
        let ctrls_to_change_bytes = self.ctrls_to_change.serialize();
        let auto_ctrls_bytes = self.auto_ctrls.serialize();
        let auto_ctrls_values_bytes = self.auto_ctrls_values.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, PerClientFlagsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PER_CLIENT_FLAGS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (change, remaining) = u32::try_parse(remaining)?;
        let (value, remaining) = u32::try_parse(remaining)?;
        let (ctrls_to_change, remaining) = u32::try_parse(remaining)?;
        let (auto_ctrls, remaining) = u32::try_parse(remaining)?;
        let (auto_ctrls_values, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(PerClientFlagsRequest {
            device_spec,
            change,
            value,
            ctrls_to_change,
            auto_ctrls,
            auto_ctrls_values,
        })
    }
}
impl Request for PerClientFlagsRequest {
    type Reply = PerClientFlagsReply;
}
pub fn per_client_flags<Conn, A, B, C, D, E>(conn: &Conn, device_spec: DeviceSpec, change: A, value: B, ctrls_to_change: C, auto_ctrls: D, auto_ctrls_values: E) -> Result<Cookie<'_, Conn, PerClientFlagsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
    B: Into<u32>,
    C: Into<u32>,
    D: Into<u32>,
    E: Into<u32>,
{
    let change: u32 = change.into();
    let value: u32 = value.into();
    let ctrls_to_change: u32 = ctrls_to_change.into();
    let auto_ctrls: u32 = auto_ctrls.into();
    let auto_ctrls_values: u32 = auto_ctrls_values.into();
    let request0 = PerClientFlagsRequest {
        device_spec,
        change,
        value,
        ctrls_to_change,
        auto_ctrls,
        auto_ctrls_values,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerClientFlagsReply {
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub supported: u32,
    pub value: u32,
    pub auto_ctrls: u32,
    pub auto_ctrls_values: u32,
}
impl TryParse for PerClientFlagsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (supported, remaining) = u32::try_parse(remaining)?;
        let (value, remaining) = u32::try_parse(remaining)?;
        let (auto_ctrls, remaining) = u32::try_parse(remaining)?;
        let (auto_ctrls_values, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PerClientFlagsReply { device_id, sequence, length, supported, value, auto_ctrls, auto_ctrls_values };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the ListComponents request
pub const LIST_COMPONENTS_REQUEST: u8 = 22;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListComponentsRequest {
    pub device_spec: DeviceSpec,
    pub max_names: u16,
}
impl ListComponentsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let max_names_bytes = self.max_names.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListComponentsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_COMPONENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (max_names, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(ListComponentsRequest {
            device_spec,
            max_names,
        })
    }
}
impl Request for ListComponentsRequest {
    type Reply = ListComponentsReply;
}
pub fn list_components<Conn>(conn: &Conn, device_spec: DeviceSpec, max_names: u16) -> Result<Cookie<'_, Conn, ListComponentsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListComponentsRequest {
        device_spec,
        max_names,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListComponentsReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(10..).ok_or(ParseError::InsufficientData)?;
        let (keymaps, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_keymaps.try_to_usize()?)?;
        let (keycodes, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_keycodes.try_to_usize()?)?;
        let (types, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_types.try_to_usize()?)?;
        let (compat_maps, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_compat_maps.try_to_usize()?)?;
        let (symbols, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_symbols.try_to_usize()?)?;
        let (geometries, remaining) = crate::x11_utils::parse_list::<Listing>(remaining, n_geometries.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListComponentsReply { device_id, sequence, length, extra, keymaps, keycodes, types, compat_maps, symbols, geometries };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetKbdByNameRequest {
    pub device_spec: DeviceSpec,
    pub need: u16,
    pub want: u16,
    pub load: bool,
}
impl GetKbdByNameRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let need_bytes = self.need.serialize();
        let want_bytes = self.want.serialize();
        let load_bytes = self.load.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetKbdByNameReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_KBD_BY_NAME_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (need, remaining) = u16::try_parse(remaining)?;
        let (want, remaining) = u16::try_parse(remaining)?;
        let (load, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetKbdByNameRequest {
            device_spec,
            need,
            want,
            load,
        })
    }
}
impl Request for GetKbdByNameRequest {
    type Reply = GetKbdByNameReply;
}
pub fn get_kbd_by_name<Conn, A, B>(conn: &Conn, device_spec: DeviceSpec, need: A, want: B, load: bool) -> Result<Cookie<'_, Conn, GetKbdByNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u16>,
{
    let need: u16 = need.into();
    let want: u16 = want.into();
    let request0 = GetKbdByNameRequest {
        device_spec,
        need,
        want,
        load,
    };
    request0.send(conn)
}

#[derive(Debug, Clone)]
pub struct GetKbdByNameRepliesTypesMapBitcase3 {
    pub acts_rtrn_count: Vec<u8>,
    pub acts_rtrn_acts: Vec<Action>,
}
impl GetKbdByNameRepliesTypesMapBitcase3 {
    pub fn try_parse(remaining: &[u8], n_key_actions: u8, total_actions: u16) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (acts_rtrn_count, remaining) = crate::x11_utils::parse_u8_list(remaining, n_key_actions.try_to_usize()?)?;
        let acts_rtrn_count = acts_rtrn_count.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (acts_rtrn_acts, remaining) = crate::x11_utils::parse_list::<Action>(remaining, total_actions.try_to_usize()?)?;
        let result = GetKbdByNameRepliesTypesMapBitcase3 { acts_rtrn_count, acts_rtrn_acts };
        Ok((result, remaining))
    }
}
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
        let types_rtrn = if switch_expr & u32::from(MapPart::KEY_TYPES) != 0 {
            let remaining = outer_remaining;
            let (types_rtrn, remaining) = crate::x11_utils::parse_list::<KeyType>(remaining, n_types.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(types_rtrn)
        } else {
            None
        };
        let syms_rtrn = if switch_expr & u32::from(MapPart::KEY_SYMS) != 0 {
            let remaining = outer_remaining;
            let (syms_rtrn, remaining) = crate::x11_utils::parse_list::<KeySymMap>(remaining, n_key_syms.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(syms_rtrn)
        } else {
            None
        };
        let bitcase3 = if switch_expr & u32::from(MapPart::KEY_ACTIONS) != 0 {
            let (bitcase3, new_remaining) = GetKbdByNameRepliesTypesMapBitcase3::try_parse(outer_remaining, n_key_actions, total_actions)?;
            outer_remaining = new_remaining;
            Some(bitcase3)
        } else {
            None
        };
        let behaviors_rtrn = if switch_expr & u32::from(MapPart::KEY_BEHAVIORS) != 0 {
            let remaining = outer_remaining;
            let (behaviors_rtrn, remaining) = crate::x11_utils::parse_list::<SetBehavior>(remaining, total_key_behaviors.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(behaviors_rtrn)
        } else {
            None
        };
        let vmods_rtrn = if switch_expr & u32::from(MapPart::VIRTUAL_MODS) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (vmods_rtrn, remaining) = crate::x11_utils::parse_u8_list(remaining, virtual_mods.count_ones().try_to_usize()?)?;
            let vmods_rtrn = vmods_rtrn.to_vec();
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            Some(vmods_rtrn)
        } else {
            None
        };
        let explicit_rtrn = if switch_expr & u32::from(MapPart::EXPLICIT_COMPONENTS) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (explicit_rtrn, remaining) = crate::x11_utils::parse_list::<SetExplicit>(remaining, total_key_explicit.try_to_usize()?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            Some(explicit_rtrn)
        } else {
            None
        };
        let modmap_rtrn = if switch_expr & u32::from(MapPart::MODIFIER_MAP) != 0 {
            let remaining = outer_remaining;
            let value = remaining;
            let (modmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyModMap>(remaining, total_mod_map_keys.try_to_usize()?)?;
            // Align offset to multiple of 4
            let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
            let misalignment = (4 - (offset % 4)) % 4;
            let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
            outer_remaining = remaining;
            Some(modmap_rtrn)
        } else {
            None
        };
        let vmodmap_rtrn = if switch_expr & u32::from(MapPart::VIRTUAL_MOD_MAP) != 0 {
            let remaining = outer_remaining;
            let (vmodmap_rtrn, remaining) = crate::x11_utils::parse_list::<KeyVModMap>(remaining, total_v_mod_map_keys.try_to_usize()?)?;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (map, remaining) = GetKbdByNameRepliesTypesMap::try_parse(remaining, present, n_types, n_key_syms, n_key_actions, total_actions, total_key_behaviors, virtual_mods, total_key_explicit, total_mod_map_keys, total_v_mod_map_keys)?;
        let result = GetKbdByNameRepliesTypes { getmap_type, type_device_id, getmap_sequence, getmap_length, type_min_key_code, type_max_key_code, first_type, n_types, total_types, first_key_sym, total_syms, n_key_syms, first_key_action, total_actions, n_key_actions, first_key_behavior, n_key_behaviors, total_key_behaviors, first_key_explicit, n_key_explicit, total_key_explicit, first_mod_map_key, n_mod_map_keys, total_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, total_v_mod_map_keys, virtual_mods, map };
        Ok((result, remaining))
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (first_si_rtrn, remaining) = u16::try_parse(remaining)?;
        let (n_si_rtrn, remaining) = u16::try_parse(remaining)?;
        let (n_total_si, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (si_rtrn, remaining) = crate::x11_utils::parse_list::<SymInterpret>(remaining, n_si_rtrn.try_to_usize()?)?;
        let (group_rtrn, remaining) = crate::x11_utils::parse_list::<ModDef>(remaining, groups_rtrn.count_ones().try_to_usize()?)?;
        let result = GetKbdByNameRepliesCompatMap { compatmap_type, compat_device_id, compatmap_sequence, compatmap_length, groups_rtrn, first_si_rtrn, n_total_si, si_rtrn, group_rtrn };
        Ok((result, remaining))
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
        let remaining = remaining.get(15..).ok_or(ParseError::InsufficientData)?;
        let (maps, remaining) = crate::x11_utils::parse_list::<IndicatorMap>(remaining, n_indicators.try_to_usize()?)?;
        let result = GetKbdByNameRepliesIndicatorMaps { indicatormap_type, indicator_device_id, indicatormap_sequence, indicatormap_length, which, real_indicators, maps };
        Ok((result, remaining))
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
        let (n_levels_per_type, remaining) = crate::x11_utils::parse_u8_list(remaining, n_types.try_to_usize()?)?;
        let n_levels_per_type = n_levels_per_type.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (kt_level_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_levels_per_type.iter().try_fold(0u32, |acc, x| acc.checked_add(u32::from(*x)).ok_or(ParseError::InvalidExpression))?.try_to_usize()?)?;
        let result = GetKbdByNameRepliesKeyNamesValueListBitcase8 { n_levels_per_type, kt_level_names };
        Ok((result, remaining))
    }
}
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
        let keycodes_name = if switch_expr & u32::from(NameDetail::KEYCODES) != 0 {
            let remaining = outer_remaining;
            let (keycodes_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(keycodes_name)
        } else {
            None
        };
        let geometry_name = if switch_expr & u32::from(NameDetail::GEOMETRY) != 0 {
            let remaining = outer_remaining;
            let (geometry_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(geometry_name)
        } else {
            None
        };
        let symbols_name = if switch_expr & u32::from(NameDetail::SYMBOLS) != 0 {
            let remaining = outer_remaining;
            let (symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(symbols_name)
        } else {
            None
        };
        let phys_symbols_name = if switch_expr & u32::from(NameDetail::PHYS_SYMBOLS) != 0 {
            let remaining = outer_remaining;
            let (phys_symbols_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(phys_symbols_name)
        } else {
            None
        };
        let types_name = if switch_expr & u32::from(NameDetail::TYPES) != 0 {
            let remaining = outer_remaining;
            let (types_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(types_name)
        } else {
            None
        };
        let compat_name = if switch_expr & u32::from(NameDetail::COMPAT) != 0 {
            let remaining = outer_remaining;
            let (compat_name, remaining) = xproto::Atom::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(compat_name)
        } else {
            None
        };
        let type_names = if switch_expr & u32::from(NameDetail::KEY_TYPE_NAMES) != 0 {
            let remaining = outer_remaining;
            let (type_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_types.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(type_names)
        } else {
            None
        };
        let bitcase8 = if switch_expr & u32::from(NameDetail::KT_LEVEL_NAMES) != 0 {
            let (bitcase8, new_remaining) = GetKbdByNameRepliesKeyNamesValueListBitcase8::try_parse(outer_remaining, n_types)?;
            outer_remaining = new_remaining;
            Some(bitcase8)
        } else {
            None
        };
        let indicator_names = if switch_expr & u32::from(NameDetail::INDICATOR_NAMES) != 0 {
            let remaining = outer_remaining;
            let (indicator_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, indicators.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(indicator_names)
        } else {
            None
        };
        let virtual_mod_names = if switch_expr & u32::from(NameDetail::VIRTUAL_MOD_NAMES) != 0 {
            let remaining = outer_remaining;
            let (virtual_mod_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, virtual_mods.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(virtual_mod_names)
        } else {
            None
        };
        let groups = if switch_expr & u32::from(NameDetail::GROUP_NAMES) != 0 {
            let remaining = outer_remaining;
            let (groups, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, group_names.count_ones().try_to_usize()?)?;
            outer_remaining = remaining;
            Some(groups)
        } else {
            None
        };
        let key_names = if switch_expr & u32::from(NameDetail::KEY_NAMES) != 0 {
            let remaining = outer_remaining;
            let (key_names, remaining) = crate::x11_utils::parse_list::<KeyName>(remaining, n_keys.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(key_names)
        } else {
            None
        };
        let key_aliases = if switch_expr & u32::from(NameDetail::KEY_ALIASES) != 0 {
            let remaining = outer_remaining;
            let (key_aliases, remaining) = crate::x11_utils::parse_list::<KeyAlias>(remaining, n_key_aliases.try_to_usize()?)?;
            outer_remaining = remaining;
            Some(key_aliases)
        } else {
            None
        };
        let radio_group_names = if switch_expr & u32::from(NameDetail::RG_NAMES) != 0 {
            let remaining = outer_remaining;
            let (radio_group_names, remaining) = crate::x11_utils::parse_list::<xproto::Atom>(remaining, n_radio_groups.try_to_usize()?)?;
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
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (value_list, remaining) = GetKbdByNameRepliesKeyNamesValueList::try_parse(remaining, which, n_types, indicators, virtual_mods, group_names, n_keys, n_key_aliases, n_radio_groups)?;
        let result = GetKbdByNameRepliesKeyNames { keyname_type, key_device_id, keyname_sequence, keyname_length, key_min_key_code, key_max_key_code, n_types, group_names, virtual_mods, first_key, n_keys, indicators, n_radio_groups, n_key_aliases, n_kt_levels, value_list };
        Ok((result, remaining))
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
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
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
        let types = if switch_expr & u32::from(GBNDetail::TYPES) != 0 || switch_expr & u32::from(GBNDetail::CLIENT_SYMBOLS) != 0 || switch_expr & u32::from(GBNDetail::SERVER_SYMBOLS) != 0 {
            let (types, new_remaining) = GetKbdByNameRepliesTypes::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(types)
        } else {
            None
        };
        let compat_map = if switch_expr & u32::from(GBNDetail::COMPAT_MAP) != 0 {
            let (compat_map, new_remaining) = GetKbdByNameRepliesCompatMap::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(compat_map)
        } else {
            None
        };
        let indicator_maps = if switch_expr & u32::from(GBNDetail::INDICATOR_MAPS) != 0 {
            let (indicator_maps, new_remaining) = GetKbdByNameRepliesIndicatorMaps::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(indicator_maps)
        } else {
            None
        };
        let key_names = if switch_expr & u32::from(GBNDetail::KEY_NAMES) != 0 || switch_expr & u32::from(GBNDetail::OTHER_NAMES) != 0 {
            let (key_names, new_remaining) = GetKbdByNameRepliesKeyNames::try_parse(outer_remaining)?;
            outer_remaining = new_remaining;
            Some(key_names)
        } else {
            None
        };
        let geometry = if switch_expr & u32::from(GBNDetail::GEOMETRY) != 0 {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (replies, remaining) = GetKbdByNameReplies::try_parse(remaining, reported)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetKbdByNameReply { device_id, sequence, length, min_key_code, max_key_code, loaded, new_keyboard, found, reported, replies };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the GetDeviceInfo request
pub const GET_DEVICE_INFO_REQUEST: u8 = 24;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDeviceInfoRequest {
    pub device_spec: DeviceSpec,
    pub wanted: u16,
    pub all_buttons: bool,
    pub first_button: u8,
    pub n_buttons: u8,
    pub led_class: LedClass,
    pub led_id: IDSpec,
}
impl GetDeviceInfoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let wanted_bytes = self.wanted.serialize();
        let all_buttons_bytes = self.all_buttons.serialize();
        let first_button_bytes = self.first_button.serialize();
        let n_buttons_bytes = self.n_buttons.serialize();
        let led_class_bytes = LedClassSpec::from(self.led_class).serialize();
        let led_id_bytes = self.led_id.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetDeviceInfoReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_INFO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (wanted, remaining) = u16::try_parse(remaining)?;
        let (all_buttons, remaining) = bool::try_parse(remaining)?;
        let (first_button, remaining) = u8::try_parse(remaining)?;
        let (n_buttons, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (led_class, remaining) = LedClassSpec::try_parse(remaining)?;
        let led_class = led_class.into();
        let (led_id, remaining) = IDSpec::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetDeviceInfoRequest {
            device_spec,
            wanted,
            all_buttons,
            first_button,
            n_buttons,
            led_class,
            led_id,
        })
    }
}
impl Request for GetDeviceInfoRequest {
    type Reply = GetDeviceInfoReply;
}
pub fn get_device_info<Conn, A, B>(conn: &Conn, device_spec: DeviceSpec, wanted: A, all_buttons: bool, first_button: u8, n_buttons: u8, led_class: LedClass, led_id: B) -> Result<Cookie<'_, Conn, GetDeviceInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<IDSpec>,
{
    let wanted: u16 = wanted.into();
    let led_id: IDSpec = led_id.into();
    let request0 = GetDeviceInfoRequest {
        device_spec,
        wanted,
        all_buttons,
        first_button,
        n_buttons,
        led_class,
        led_id,
    };
    request0.send(conn)
}

#[derive(Debug, Clone)]
pub struct GetDeviceInfoReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (dev_type, remaining) = xproto::Atom::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (btn_actions, remaining) = crate::x11_utils::parse_list::<Action>(remaining, n_btns_rtrn.try_to_usize()?)?;
        let (leds, remaining) = crate::x11_utils::parse_list::<DeviceLedInfo>(remaining, n_device_led_f_bs.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceInfoReply { device_id, sequence, length, present, supported, unsupported, first_btn_wanted, n_btns_wanted, first_btn_rtrn, total_btns, has_own_state, dflt_kbd_fb, dflt_led_fb, dev_type, name, btn_actions, leds };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
#[derive(Debug, Clone)]
pub struct SetDeviceInfoRequest<'input> {
    pub device_spec: DeviceSpec,
    pub first_btn: u8,
    pub change: u16,
    pub btn_actions: Cow<'input, [Action]>,
    pub leds: Cow<'input, [DeviceLedInfo]>,
}
impl<'input> SetDeviceInfoRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_spec_bytes = self.device_spec.serialize();
        let first_btn_bytes = self.first_btn.serialize();
        let n_btns = u8::try_from(self.btn_actions.len()).expect("`btn_actions` has too many elements");
        let n_btns_bytes = n_btns.serialize();
        let change_bytes = self.change.serialize();
        let n_device_led_f_bs = u16::try_from(self.leds.len()).expect("`leds` has too many elements");
        let n_device_led_f_bs_bytes = n_device_led_f_bs.serialize();
        let mut request0 = vec![
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
        let btn_actions_bytes = self.btn_actions.serialize();
        let length_so_far = length_so_far + btn_actions_bytes.len();
        let leds_bytes = self.leds.serialize();
        let length_so_far = length_so_far + leds_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), btn_actions_bytes.into(), leds_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_INFO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device_spec, remaining) = DeviceSpec::try_parse(value)?;
        let (first_btn, remaining) = u8::try_parse(remaining)?;
        let (n_btns, remaining) = u8::try_parse(remaining)?;
        let (change, remaining) = u16::try_parse(remaining)?;
        let (n_device_led_f_bs, remaining) = u16::try_parse(remaining)?;
        let (btn_actions, remaining) = crate::x11_utils::parse_list::<Action>(remaining, n_btns.try_to_usize()?)?;
        let (leds, remaining) = crate::x11_utils::parse_list::<DeviceLedInfo>(remaining, n_device_led_f_bs.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetDeviceInfoRequest {
            device_spec,
            first_btn,
            change,
            btn_actions: Cow::Owned(btn_actions),
            leds: Cow::Owned(leds),
        })
    }
    /// Clone all borrowed data in this SetDeviceInfoRequest.
    pub fn into_owned(self) -> SetDeviceInfoRequest<'static> {
        SetDeviceInfoRequest {
            device_spec: self.device_spec,
            first_btn: self.first_btn,
            change: self.change,
            btn_actions: Cow::Owned(self.btn_actions.into_owned()),
            leds: Cow::Owned(self.leds.into_owned()),
        }
    }
}
impl<'input> Request for SetDeviceInfoRequest<'input> {
    type Reply = ();
}
pub fn set_device_info<'c, 'input, Conn, A>(conn: &'c Conn, device_spec: DeviceSpec, first_btn: u8, change: A, btn_actions: &'input [Action], leds: &'input [DeviceLedInfo]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
{
    let change: u16 = change.into();
    let request0 = SetDeviceInfoRequest {
        device_spec,
        first_btn,
        change,
        btn_actions: Cow::Borrowed(btn_actions),
        leds: Cow::Borrowed(leds),
    };
    request0.send(conn)
}

/// Opcode for the SetDebuggingFlags request
pub const SET_DEBUGGING_FLAGS_REQUEST: u8 = 101;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetDebuggingFlagsRequest<'input> {
    pub affect_flags: u32,
    pub flags: u32,
    pub affect_ctrls: u32,
    pub ctrls: u32,
    pub message: Cow<'input, [String8]>,
}
impl<'input> SetDebuggingFlagsRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let msg_length = u16::try_from(self.message.len()).expect("`message` has too many elements");
        let msg_length_bytes = msg_length.serialize();
        let affect_flags_bytes = self.affect_flags.serialize();
        let flags_bytes = self.flags.serialize();
        let affect_ctrls_bytes = self.affect_ctrls.serialize();
        let ctrls_bytes = self.ctrls.serialize();
        let mut request0 = vec![
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
        let length_so_far = length_so_far + self.message.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.message, padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, SetDebuggingFlagsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEBUGGING_FLAGS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (msg_length, remaining) = u16::try_parse(value)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (affect_flags, remaining) = u32::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (affect_ctrls, remaining) = u32::try_parse(remaining)?;
        let (ctrls, remaining) = u32::try_parse(remaining)?;
        let (message, remaining) = crate::x11_utils::parse_u8_list(remaining, msg_length.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetDebuggingFlagsRequest {
            affect_flags,
            flags,
            affect_ctrls,
            ctrls,
            message: Cow::Borrowed(message),
        })
    }
    /// Clone all borrowed data in this SetDebuggingFlagsRequest.
    pub fn into_owned(self) -> SetDebuggingFlagsRequest<'static> {
        SetDebuggingFlagsRequest {
            affect_flags: self.affect_flags,
            flags: self.flags,
            affect_ctrls: self.affect_ctrls,
            ctrls: self.ctrls,
            message: Cow::Owned(self.message.into_owned()),
        }
    }
}
impl<'input> Request for SetDebuggingFlagsRequest<'input> {
    type Reply = SetDebuggingFlagsReply;
}
pub fn set_debugging_flags<'c, 'input, Conn>(conn: &'c Conn, affect_flags: u32, flags: u32, affect_ctrls: u32, ctrls: u32, message: &'input [String8]) -> Result<Cookie<'c, Conn, SetDebuggingFlagsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDebuggingFlagsRequest {
        affect_flags,
        flags,
        affect_ctrls,
        ctrls,
        message: Cow::Borrowed(message),
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetDebuggingFlagsReply {
    pub sequence: u16,
    pub length: u32,
    pub current_flags: u32,
    pub current_ctrls: u32,
    pub supported_flags: u32,
    pub supported_ctrls: u32,
}
impl TryParse for SetDebuggingFlagsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (current_flags, remaining) = u32::try_parse(remaining)?;
        let (current_ctrls, remaining) = u32::try_parse(remaining)?;
        let (supported_flags, remaining) = u32::try_parse(remaining)?;
        let (supported_ctrls, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = SetDebuggingFlagsReply { sequence, length, current_flags, current_ctrls, supported_flags, supported_ctrls };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(14..).ok_or(ParseError::InsufficientData)?;
        let result = NewKeyboardNotifyEvent { response_type, xkb_type, sequence, time, device_id, old_device_id, min_key_code, max_key_code, old_min_key_code, old_max_key_code, request_major, request_minor, changed };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let result = MapNotifyEvent { response_type, xkb_type, sequence, time, device_id, ptr_btn_actions, changed, min_key_code, max_key_code, first_type, n_types, first_key_sym, n_key_syms, first_key_act, n_key_acts, first_key_behavior, n_key_behavior, first_key_explicit, n_key_explicit, first_mod_map_key, n_mod_map_keys, first_v_mod_map_key, n_v_mod_map_keys, virtual_mods };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let group = group.into();
        let locked_group = locked_group.into();
        let result = StateNotifyEvent { response_type, xkb_type, sequence, time, device_id, mods, base_mods, latched_mods, locked_mods, group, base_group, latched_group, locked_group, compat_state, grab_mods, compat_grab_mods, lookup_mods, compat_loockup_mods, ptr_btn_state, changed, keycode, event_type, request_major, request_minor };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (num_groups, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (changed_controls, remaining) = u32::try_parse(remaining)?;
        let (enabled_controls, remaining) = u32::try_parse(remaining)?;
        let (enabled_control_changes, remaining) = u32::try_parse(remaining)?;
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (event_type, remaining) = u8::try_parse(remaining)?;
        let (request_major, remaining) = u8::try_parse(remaining)?;
        let (request_minor, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let result = ControlsNotifyEvent { response_type, xkb_type, sequence, time, device_id, num_groups, changed_controls, enabled_controls, enabled_control_changes, keycode, event_type, request_major, request_minor };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        let (state_changed, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let result = IndicatorStateNotifyEvent { response_type, xkb_type, sequence, time, device_id, state, state_changed };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (state, remaining) = u32::try_parse(remaining)?;
        let (map_changed, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let result = IndicatorMapNotifyEvent { response_type, xkb_type, sequence, time, device_id, state, map_changed };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (changed, remaining) = u16::try_parse(remaining)?;
        let (first_type, remaining) = u8::try_parse(remaining)?;
        let (n_types, remaining) = u8::try_parse(remaining)?;
        let (first_level_name, remaining) = u8::try_parse(remaining)?;
        let (n_level_names, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (n_radio_groups, remaining) = u8::try_parse(remaining)?;
        let (n_key_aliases, remaining) = u8::try_parse(remaining)?;
        let (changed_group_names, remaining) = u8::try_parse(remaining)?;
        let (changed_virtual_mods, remaining) = u16::try_parse(remaining)?;
        let (first_key, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (n_keys, remaining) = u8::try_parse(remaining)?;
        let (changed_indicators, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let result = NamesNotifyEvent { response_type, xkb_type, sequence, time, device_id, changed, first_type, n_types, first_level_name, n_level_names, n_radio_groups, n_key_aliases, changed_group_names, changed_virtual_mods, first_key, n_keys, changed_indicators };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (changed_groups, remaining) = u8::try_parse(remaining)?;
        let (first_si, remaining) = u16::try_parse(remaining)?;
        let (n_si, remaining) = u16::try_parse(remaining)?;
        let (n_total_si, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let result = CompatMapNotifyEvent { response_type, xkb_type, sequence, time, device_id, changed_groups, first_si, n_si, n_total_si };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(7..).ok_or(ParseError::InsufficientData)?;
        let bell_class = bell_class.into();
        let result = BellNotifyEvent { response_type, xkb_type, sequence, time, device_id, bell_class, bell_id, percent, pitch, duration, name, window, event_only };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
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
        let remaining = remaining.get(10..).ok_or(ParseError::InsufficientData)?;
        let group = group.into();
        let result = ActionMessageEvent { response_type, xkb_type, sequence, time, device_id, keycode, press, key_event_follows, mods, group, message };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let (keycode, remaining) = xproto::Keycode::try_parse(remaining)?;
        let (detailt, remaining) = u16::try_parse(remaining)?;
        let (slow_keys_delay, remaining) = u16::try_parse(remaining)?;
        let (debounce_delay, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let result = AccessXNotifyEvent { response_type, xkb_type, sequence, time, device_id, keycode, detailt, slow_keys_delay, debounce_delay };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (xkb_type, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (device_id, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (reason, remaining) = u16::try_parse(remaining)?;
        let (led_class, remaining) = u16::try_parse(remaining)?;
        let (led_id, remaining) = u16::try_parse(remaining)?;
        let (leds_defined, remaining) = u32::try_parse(remaining)?;
        let (led_state, remaining) = u32::try_parse(remaining)?;
        let (first_button, remaining) = u8::try_parse(remaining)?;
        let (n_buttons, remaining) = u8::try_parse(remaining)?;
        let (supported, remaining) = u16::try_parse(remaining)?;
        let (unsupported, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let led_class = led_class.into();
        let result = ExtensionDeviceNotifyEvent { response_type, xkb_type, sequence, time, device_id, reason, led_class, led_id, leds_defined, led_state, first_button, n_buttons, supported, unsupported };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
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
    fn xkb_select_events<'c, 'input, A, B, C, D>(&'c self, device_spec: DeviceSpec, clear: A, select_all: B, affect_map: C, map: D, details: &'input SelectEventsAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u16>,
        C: Into<u16>,
        D: Into<u16>,
    {
        select_events(self, device_spec, clear, select_all, affect_map, map, details)
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
