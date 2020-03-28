// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

use std::convert::TryInto;
use crate::errors::ParseError;
use crate::x11_utils::{Event as _, GenericError, GenericEvent};
use xproto::QueryExtensionReply;
pub mod bigreq;
#[cfg(feature = "composite")]
pub mod composite;
#[cfg(feature = "damage")]
pub mod damage;
#[cfg(feature = "dpms")]
pub mod dpms;
#[cfg(feature = "dri2")]
pub mod dri2;
#[cfg(feature = "dri3")]
pub mod dri3;
pub mod ge;
#[cfg(feature = "glx")]
pub mod glx;
#[cfg(feature = "present")]
pub mod present;
#[cfg(feature = "randr")]
pub mod randr;
#[cfg(feature = "record")]
pub mod record;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "res")]
pub mod res;
#[cfg(feature = "screensaver")]
pub mod screensaver;
#[cfg(feature = "shape")]
pub mod shape;
#[cfg(feature = "shm")]
pub mod shm;
#[cfg(feature = "sync")]
pub mod sync;
pub mod xc_misc;
#[cfg(feature = "xevie")]
pub mod xevie;
#[cfg(feature = "xf86dri")]
pub mod xf86dri;
#[cfg(feature = "xf86vidmode")]
pub mod xf86vidmode;
#[cfg(feature = "xfixes")]
pub mod xfixes;
#[cfg(feature = "xinerama")]
pub mod xinerama;
#[cfg(feature = "xinput")]
pub mod xinput;
#[cfg(feature = "xkb")]
pub mod xkb;
#[cfg(feature = "xprint")]
pub mod xprint;
pub mod xproto;
#[cfg(feature = "xselinux")]
pub mod xselinux;
#[cfg(feature = "xtest")]
pub mod xtest;
#[cfg(feature = "xv")]
pub mod xv;
#[cfg(feature = "xvmc")]
pub mod xvmc;

/// Enumeration of all possible X11 errors.
#[derive(Debug, Clone)]
pub enum Error<B: std::fmt::Debug + AsRef<[u8]>> {
    Unknown(GenericError<B>),
    #[cfg(feature = "damage")]
    DamageBadDamageError(damage::BadDamageError),
    #[cfg(feature = "glx")]
    GlxBadContextError(glx::BadContextError),
    #[cfg(feature = "glx")]
    GlxBadContextStateError(glx::BadContextStateError),
    #[cfg(feature = "glx")]
    GlxBadContextTagError(glx::BadContextTagError),
    #[cfg(feature = "glx")]
    GlxBadCurrentDrawableError(glx::BadCurrentDrawableError),
    #[cfg(feature = "glx")]
    GlxBadCurrentWindowError(glx::BadCurrentWindowError),
    #[cfg(feature = "glx")]
    GlxBadDrawableError(glx::BadDrawableError),
    #[cfg(feature = "glx")]
    GlxBadFBConfigError(glx::BadFBConfigError),
    #[cfg(feature = "glx")]
    GlxBadLargeRequestError(glx::BadLargeRequestError),
    #[cfg(feature = "glx")]
    GlxBadPbufferError(glx::BadPbufferError),
    #[cfg(feature = "glx")]
    GlxBadPixmapError(glx::BadPixmapError),
    #[cfg(feature = "glx")]
    GlxBadRenderRequestError(glx::BadRenderRequestError),
    #[cfg(feature = "glx")]
    GlxBadWindowError(glx::BadWindowError),
    #[cfg(feature = "glx")]
    GlxGLXBadProfileARBError(glx::GLXBadProfileARBError),
    #[cfg(feature = "glx")]
    GlxUnsupportedPrivateRequestError(glx::UnsupportedPrivateRequestError),
    #[cfg(feature = "randr")]
    RandrBadCrtcError(randr::BadCrtcError),
    #[cfg(feature = "randr")]
    RandrBadModeError(randr::BadModeError),
    #[cfg(feature = "randr")]
    RandrBadOutputError(randr::BadOutputError),
    #[cfg(feature = "randr")]
    RandrBadProviderError(randr::BadProviderError),
    #[cfg(feature = "record")]
    RecordBadContextError(record::BadContextError),
    #[cfg(feature = "render")]
    RenderGlyphError(render::GlyphError),
    #[cfg(feature = "render")]
    RenderGlyphSetError(render::GlyphSetError),
    #[cfg(feature = "render")]
    RenderPictFormatError(render::PictFormatError),
    #[cfg(feature = "render")]
    RenderPictOpError(render::PictOpError),
    #[cfg(feature = "render")]
    RenderPictureError(render::PictureError),
    #[cfg(feature = "shm")]
    ShmBadSegError(shm::BadSegError),
    #[cfg(feature = "sync")]
    SyncAlarmError(sync::AlarmError),
    #[cfg(feature = "sync")]
    SyncCounterError(sync::CounterError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadClockError(xf86vidmode::BadClockError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadHTimingsError(xf86vidmode::BadHTimingsError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadVTimingsError(xf86vidmode::BadVTimingsError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeClientNotLocalError(xf86vidmode::ClientNotLocalError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeExtensionDisabledError(xf86vidmode::ExtensionDisabledError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeModeUnsuitableError(xf86vidmode::ModeUnsuitableError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeZoomLockedError(xf86vidmode::ZoomLockedError),
    #[cfg(feature = "xfixes")]
    XfixesBadRegionError(xfixes::BadRegionError),
    #[cfg(feature = "xinput")]
    XinputClassError(xinput::ClassError),
    #[cfg(feature = "xinput")]
    XinputDeviceError(xinput::DeviceError),
    #[cfg(feature = "xinput")]
    XinputDeviceBusyError(xinput::DeviceBusyError),
    #[cfg(feature = "xinput")]
    XinputEventError(xinput::EventError),
    #[cfg(feature = "xinput")]
    XinputModeError(xinput::ModeError),
    #[cfg(feature = "xkb")]
    XkbKeyboardError(xkb::KeyboardError),
    #[cfg(feature = "xprint")]
    XprintBadContextError(xprint::BadContextError),
    #[cfg(feature = "xprint")]
    XprintBadSequenceError(xprint::BadSequenceError),
    XprotoAccessError(xproto::AccessError),
    XprotoAllocError(xproto::AllocError),
    XprotoAtomError(xproto::AtomError),
    XprotoColormapError(xproto::ColormapError),
    XprotoCursorError(xproto::CursorError),
    XprotoDrawableError(xproto::DrawableError),
    XprotoFontError(xproto::FontError),
    XprotoGContextError(xproto::GContextError),
    XprotoIDChoiceError(xproto::IDChoiceError),
    XprotoImplementationError(xproto::ImplementationError),
    XprotoLengthError(xproto::LengthError),
    XprotoMatchError(xproto::MatchError),
    XprotoNameError(xproto::NameError),
    XprotoPixmapError(xproto::PixmapError),
    XprotoRequestError(xproto::RequestError),
    XprotoValueError(xproto::ValueError),
    XprotoWindowError(xproto::WindowError),
    #[cfg(feature = "xv")]
    XvBadControlError(xv::BadControlError),
    #[cfg(feature = "xv")]
    XvBadEncodingError(xv::BadEncodingError),
    #[cfg(feature = "xv")]
    XvBadPortError(xv::BadPortError),
}
impl<B: std::fmt::Debug + AsRef<[u8]>> Error<B> {
    /// Parse a generic X11 error into a concrete error type.
    pub fn parse(
        error: GenericError<B>,
        iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,
    ) -> Result<Self, ParseError> {
        let error_code = error.error_code();
        let bytes = error.raw_bytes();
        // Check if this is a core protocol error
        match error_code {
            10 => return Ok(Self::XprotoAccessError(xproto::AccessError::try_parse(bytes)?.0)),
            11 => return Ok(Self::XprotoAllocError(xproto::AllocError::try_parse(bytes)?.0)),
            5 => return Ok(Self::XprotoAtomError(xproto::AtomError::try_parse(bytes)?.0)),
            12 => return Ok(Self::XprotoColormapError(xproto::ColormapError::try_parse(bytes)?.0)),
            6 => return Ok(Self::XprotoCursorError(xproto::CursorError::try_parse(bytes)?.0)),
            9 => return Ok(Self::XprotoDrawableError(xproto::DrawableError::try_parse(bytes)?.0)),
            7 => return Ok(Self::XprotoFontError(xproto::FontError::try_parse(bytes)?.0)),
            13 => return Ok(Self::XprotoGContextError(xproto::GContextError::try_parse(bytes)?.0)),
            14 => return Ok(Self::XprotoIDChoiceError(xproto::IDChoiceError::try_parse(bytes)?.0)),
            17 => return Ok(Self::XprotoImplementationError(xproto::ImplementationError::try_parse(bytes)?.0)),
            16 => return Ok(Self::XprotoLengthError(xproto::LengthError::try_parse(bytes)?.0)),
            8 => return Ok(Self::XprotoMatchError(xproto::MatchError::try_parse(bytes)?.0)),
            15 => return Ok(Self::XprotoNameError(xproto::NameError::try_parse(bytes)?.0)),
            4 => return Ok(Self::XprotoPixmapError(xproto::PixmapError::try_parse(bytes)?.0)),
            1 => return Ok(Self::XprotoRequestError(xproto::RequestError::try_parse(bytes)?.0)),
            2 => return Ok(Self::XprotoValueError(xproto::ValueError::try_parse(bytes)?.0)),
            3 => return Ok(Self::XprotoWindowError(xproto::WindowError::try_parse(bytes)?.0)),
            _ => {}
        }
        // Find the extension that this error could belong to
        let ext_info = iter
            .map(|(name, reply)| (name, reply.first_error))
            .filter(|&(_, first_error)| first_error <= error_code)
            .max_by_key(|&(_, first_error)| first_error);
        match ext_info {
            #[cfg(feature = "damage")]
            Some(("DAMAGE", first_error)) => {
                match error_code - first_error {
                    0 => Ok(Self::DamageBadDamageError(damage::BadDamageError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "glx")]
            Some(("GLX", first_error)) => {
                match error_code - first_error {
                    0 => Ok(Self::GlxBadContextError(glx::BadContextError::try_parse(bytes)?.0)),
                    1 => Ok(Self::GlxBadContextStateError(glx::BadContextStateError::try_parse(bytes)?.0)),
                    4 => Ok(Self::GlxBadContextTagError(glx::BadContextTagError::try_parse(bytes)?.0)),
                    11 => Ok(Self::GlxBadCurrentDrawableError(glx::BadCurrentDrawableError::try_parse(bytes)?.0)),
                    5 => Ok(Self::GlxBadCurrentWindowError(glx::BadCurrentWindowError::try_parse(bytes)?.0)),
                    2 => Ok(Self::GlxBadDrawableError(glx::BadDrawableError::try_parse(bytes)?.0)),
                    9 => Ok(Self::GlxBadFBConfigError(glx::BadFBConfigError::try_parse(bytes)?.0)),
                    7 => Ok(Self::GlxBadLargeRequestError(glx::BadLargeRequestError::try_parse(bytes)?.0)),
                    10 => Ok(Self::GlxBadPbufferError(glx::BadPbufferError::try_parse(bytes)?.0)),
                    3 => Ok(Self::GlxBadPixmapError(glx::BadPixmapError::try_parse(bytes)?.0)),
                    6 => Ok(Self::GlxBadRenderRequestError(glx::BadRenderRequestError::try_parse(bytes)?.0)),
                    12 => Ok(Self::GlxBadWindowError(glx::BadWindowError::try_parse(bytes)?.0)),
                    13 => Ok(Self::GlxGLXBadProfileARBError(glx::GLXBadProfileARBError::try_parse(bytes)?.0)),
                    8 => Ok(Self::GlxUnsupportedPrivateRequestError(glx::UnsupportedPrivateRequestError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "randr")]
            Some(("RANDR", first_error)) => {
                match error_code - first_error {
                    1 => Ok(Self::RandrBadCrtcError(randr::BadCrtcError::try_parse(bytes)?.0)),
                    2 => Ok(Self::RandrBadModeError(randr::BadModeError::try_parse(bytes)?.0)),
                    0 => Ok(Self::RandrBadOutputError(randr::BadOutputError::try_parse(bytes)?.0)),
                    3 => Ok(Self::RandrBadProviderError(randr::BadProviderError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "record")]
            Some(("RECORD", first_error)) => {
                match error_code - first_error {
                    0 => Ok(Self::RecordBadContextError(record::BadContextError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "render")]
            Some(("RENDER", first_error)) => {
                match error_code - first_error {
                    4 => Ok(Self::RenderGlyphError(render::GlyphError::try_parse(bytes)?.0)),
                    3 => Ok(Self::RenderGlyphSetError(render::GlyphSetError::try_parse(bytes)?.0)),
                    0 => Ok(Self::RenderPictFormatError(render::PictFormatError::try_parse(bytes)?.0)),
                    2 => Ok(Self::RenderPictOpError(render::PictOpError::try_parse(bytes)?.0)),
                    1 => Ok(Self::RenderPictureError(render::PictureError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "shm")]
            Some(("MIT-SHM", first_error)) => {
                match error_code - first_error {
                    0 => Ok(Self::ShmBadSegError(shm::BadSegError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "sync")]
            Some(("SYNC", first_error)) => {
                match error_code - first_error {
                    1 => Ok(Self::SyncAlarmError(sync::AlarmError::try_parse(bytes)?.0)),
                    0 => Ok(Self::SyncCounterError(sync::CounterError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xf86vidmode")]
            Some(("XFree86-VidModeExtension", first_error)) => {
                match error_code - first_error {
                    0 => Ok(Self::Xf86vidmodeBadClockError(xf86vidmode::BadClockError::try_parse(bytes)?.0)),
                    1 => Ok(Self::Xf86vidmodeBadHTimingsError(xf86vidmode::BadHTimingsError::try_parse(bytes)?.0)),
                    2 => Ok(Self::Xf86vidmodeBadVTimingsError(xf86vidmode::BadVTimingsError::try_parse(bytes)?.0)),
                    5 => Ok(Self::Xf86vidmodeClientNotLocalError(xf86vidmode::ClientNotLocalError::try_parse(bytes)?.0)),
                    4 => Ok(Self::Xf86vidmodeExtensionDisabledError(xf86vidmode::ExtensionDisabledError::try_parse(bytes)?.0)),
                    3 => Ok(Self::Xf86vidmodeModeUnsuitableError(xf86vidmode::ModeUnsuitableError::try_parse(bytes)?.0)),
                    6 => Ok(Self::Xf86vidmodeZoomLockedError(xf86vidmode::ZoomLockedError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xfixes")]
            Some(("XFIXES", first_error)) => {
                match error_code - first_error {
                    0 => Ok(Self::XfixesBadRegionError(xfixes::BadRegionError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xinput")]
            Some(("XInputExtension", first_error)) => {
                match error_code - first_error {
                    4 => Ok(Self::XinputClassError(xinput::ClassError::try_parse(bytes)?.0)),
                    0 => Ok(Self::XinputDeviceError(xinput::DeviceError::try_parse(bytes)?.0)),
                    3 => Ok(Self::XinputDeviceBusyError(xinput::DeviceBusyError::try_parse(bytes)?.0)),
                    1 => Ok(Self::XinputEventError(xinput::EventError::try_parse(bytes)?.0)),
                    2 => Ok(Self::XinputModeError(xinput::ModeError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xkb")]
            Some(("XKEYBOARD", first_error)) => {
                match error_code - first_error {
                    0 => Ok(Self::XkbKeyboardError(xkb::KeyboardError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xprint")]
            Some(("XpExtension", first_error)) => {
                match error_code - first_error {
                    0 => Ok(Self::XprintBadContextError(xprint::BadContextError::try_parse(bytes)?.0)),
                    1 => Ok(Self::XprintBadSequenceError(xprint::BadSequenceError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xv")]
            Some(("XVideo", first_error)) => {
                match error_code - first_error {
                    2 => Ok(Self::XvBadControlError(xv::BadControlError::try_parse(bytes)?.0)),
                    1 => Ok(Self::XvBadEncodingError(xv::BadEncodingError::try_parse(bytes)?.0)),
                    0 => Ok(Self::XvBadPortError(xv::BadPortError::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(error))
                }
            }
            _ => Ok(Self::Unknown(error))
        }
    }
}
/// Enumeration of all possible X11 events.
#[derive(Debug, Clone)]
pub enum Event<B: std::fmt::Debug + AsRef<[u8]>> {
    Unknown(GenericEvent<B>),
    Error(Error<B>),
    #[cfg(feature = "damage")]
    DamageNotifyEvent(damage::NotifyEvent),
    #[cfg(feature = "dri2")]
    Dri2BufferSwapCompleteEvent(dri2::BufferSwapCompleteEvent),
    #[cfg(feature = "dri2")]
    Dri2InvalidateBuffersEvent(dri2::InvalidateBuffersEvent),
    #[cfg(feature = "glx")]
    GlxBufferSwapCompleteEvent(glx::BufferSwapCompleteEvent),
    #[cfg(feature = "glx")]
    GlxPbufferClobberEvent(glx::PbufferClobberEvent),
    #[cfg(feature = "present")]
    PresentCompleteNotifyEvent(present::CompleteNotifyEvent),
    #[cfg(feature = "present")]
    PresentConfigureNotifyEvent(present::ConfigureNotifyEvent),
    #[cfg(feature = "present")]
    PresentGenericEvent(present::GenericEvent),
    #[cfg(feature = "present")]
    PresentIdleNotifyEvent(present::IdleNotifyEvent),
    #[cfg(feature = "present")]
    PresentRedirectNotifyEvent(present::RedirectNotifyEvent),
    #[cfg(feature = "randr")]
    RandrNotifyEvent(randr::NotifyEvent),
    #[cfg(feature = "randr")]
    RandrScreenChangeNotifyEvent(randr::ScreenChangeNotifyEvent),
    #[cfg(feature = "screensaver")]
    ScreensaverNotifyEvent(screensaver::NotifyEvent),
    #[cfg(feature = "shape")]
    ShapeNotifyEvent(shape::NotifyEvent),
    #[cfg(feature = "shm")]
    ShmCompletionEvent(shm::CompletionEvent),
    #[cfg(feature = "sync")]
    SyncAlarmNotifyEvent(sync::AlarmNotifyEvent),
    #[cfg(feature = "sync")]
    SyncCounterNotifyEvent(sync::CounterNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesCursorNotifyEvent(xfixes::CursorNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesSelectionNotifyEvent(xfixes::SelectionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputBarrierHitEvent(xinput::BarrierHitEvent),
    #[cfg(feature = "xinput")]
    XinputBarrierLeaveEvent(xinput::BarrierLeaveEvent),
    #[cfg(feature = "xinput")]
    XinputButtonPressEvent(xinput::ButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputButtonReleaseEvent(xinput::ButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceNotifyEvent(xinput::ChangeDeviceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonPressEvent(xinput::DeviceButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonReleaseEvent(xinput::DeviceButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonStateNotifyEvent(xinput::DeviceButtonStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceChangedEvent(xinput::DeviceChangedEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusInEvent(xinput::DeviceFocusInEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusOutEvent(xinput::DeviceFocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyPressEvent(xinput::DeviceKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyReleaseEvent(xinput::DeviceKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyStateNotifyEvent(xinput::DeviceKeyStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMappingNotifyEvent(xinput::DeviceMappingNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMotionNotifyEvent(xinput::DeviceMotionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePresenceNotifyEvent(xinput::DevicePresenceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePropertyNotifyEvent(xinput::DevicePropertyNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceStateNotifyEvent(xinput::DeviceStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceValuatorEvent(xinput::DeviceValuatorEvent),
    #[cfg(feature = "xinput")]
    XinputEnterEvent(xinput::EnterEvent),
    #[cfg(feature = "xinput")]
    XinputFocusInEvent(xinput::FocusInEvent),
    #[cfg(feature = "xinput")]
    XinputFocusOutEvent(xinput::FocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputHierarchyEvent(xinput::HierarchyEvent),
    #[cfg(feature = "xinput")]
    XinputKeyPressEvent(xinput::KeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputKeyReleaseEvent(xinput::KeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputLeaveEvent(xinput::LeaveEvent),
    #[cfg(feature = "xinput")]
    XinputMotionEvent(xinput::MotionEvent),
    #[cfg(feature = "xinput")]
    XinputPropertyEvent(xinput::PropertyEvent),
    #[cfg(feature = "xinput")]
    XinputProximityInEvent(xinput::ProximityInEvent),
    #[cfg(feature = "xinput")]
    XinputProximityOutEvent(xinput::ProximityOutEvent),
    #[cfg(feature = "xinput")]
    XinputRawButtonPressEvent(xinput::RawButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputRawButtonReleaseEvent(xinput::RawButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputRawKeyPressEvent(xinput::RawKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputRawKeyReleaseEvent(xinput::RawKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputRawMotionEvent(xinput::RawMotionEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchBeginEvent(xinput::RawTouchBeginEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchEndEvent(xinput::RawTouchEndEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchUpdateEvent(xinput::RawTouchUpdateEvent),
    #[cfg(feature = "xinput")]
    XinputTouchBeginEvent(xinput::TouchBeginEvent),
    #[cfg(feature = "xinput")]
    XinputTouchEndEvent(xinput::TouchEndEvent),
    #[cfg(feature = "xinput")]
    XinputTouchOwnershipEvent(xinput::TouchOwnershipEvent),
    #[cfg(feature = "xinput")]
    XinputTouchUpdateEvent(xinput::TouchUpdateEvent),
    #[cfg(feature = "xkb")]
    XkbAccessXNotifyEvent(xkb::AccessXNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbActionMessageEvent(xkb::ActionMessageEvent),
    #[cfg(feature = "xkb")]
    XkbBellNotifyEvent(xkb::BellNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbCompatMapNotifyEvent(xkb::CompatMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbControlsNotifyEvent(xkb::ControlsNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbExtensionDeviceNotifyEvent(xkb::ExtensionDeviceNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorMapNotifyEvent(xkb::IndicatorMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorStateNotifyEvent(xkb::IndicatorStateNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbMapNotifyEvent(xkb::MapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNamesNotifyEvent(xkb::NamesNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNewKeyboardNotifyEvent(xkb::NewKeyboardNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbStateNotifyEvent(xkb::StateNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintAttributNotifyEvent(xprint::AttributNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintNotifyEvent(xprint::NotifyEvent),
    XprotoButtonPressEvent(xproto::ButtonPressEvent),
    XprotoButtonReleaseEvent(xproto::ButtonReleaseEvent),
    XprotoCirculateNotifyEvent(xproto::CirculateNotifyEvent),
    XprotoCirculateRequestEvent(xproto::CirculateRequestEvent),
    XprotoClientMessageEvent(xproto::ClientMessageEvent),
    XprotoColormapNotifyEvent(xproto::ColormapNotifyEvent),
    XprotoConfigureNotifyEvent(xproto::ConfigureNotifyEvent),
    XprotoConfigureRequestEvent(xproto::ConfigureRequestEvent),
    XprotoCreateNotifyEvent(xproto::CreateNotifyEvent),
    XprotoDestroyNotifyEvent(xproto::DestroyNotifyEvent),
    XprotoEnterNotifyEvent(xproto::EnterNotifyEvent),
    XprotoExposeEvent(xproto::ExposeEvent),
    XprotoFocusInEvent(xproto::FocusInEvent),
    XprotoFocusOutEvent(xproto::FocusOutEvent),
    XprotoGeGenericEvent(xproto::GeGenericEvent),
    XprotoGraphicsExposureEvent(xproto::GraphicsExposureEvent),
    XprotoGravityNotifyEvent(xproto::GravityNotifyEvent),
    XprotoKeyPressEvent(xproto::KeyPressEvent),
    XprotoKeyReleaseEvent(xproto::KeyReleaseEvent),
    XprotoKeymapNotifyEvent(xproto::KeymapNotifyEvent),
    XprotoLeaveNotifyEvent(xproto::LeaveNotifyEvent),
    XprotoMapNotifyEvent(xproto::MapNotifyEvent),
    XprotoMapRequestEvent(xproto::MapRequestEvent),
    XprotoMappingNotifyEvent(xproto::MappingNotifyEvent),
    XprotoMotionNotifyEvent(xproto::MotionNotifyEvent),
    XprotoNoExposureEvent(xproto::NoExposureEvent),
    XprotoPropertyNotifyEvent(xproto::PropertyNotifyEvent),
    XprotoReparentNotifyEvent(xproto::ReparentNotifyEvent),
    XprotoResizeRequestEvent(xproto::ResizeRequestEvent),
    XprotoSelectionClearEvent(xproto::SelectionClearEvent),
    XprotoSelectionNotifyEvent(xproto::SelectionNotifyEvent),
    XprotoSelectionRequestEvent(xproto::SelectionRequestEvent),
    XprotoUnmapNotifyEvent(xproto::UnmapNotifyEvent),
    XprotoVisibilityNotifyEvent(xproto::VisibilityNotifyEvent),
    #[cfg(feature = "xv")]
    XvPortNotifyEvent(xv::PortNotifyEvent),
    #[cfg(feature = "xv")]
    XvVideoNotifyEvent(xv::VideoNotifyEvent),
}
impl<B: std::fmt::Debug + AsRef<[u8]>> Event<B> {
    /// Parse a generic X11 event into a concrete event type.
    pub fn parse(
        event: GenericEvent<B>,
        iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,
    ) -> Result<Self, ParseError> {
        let event_type = event.response_type();
        let bytes = event.raw_bytes();
        // Check if this is a core protocol error or from the generic event extension
        match event_type {
            0 => return Ok(Self::Error(Error::parse(event.try_into()?, iter)?)),
            4 => return Ok(Self::XprotoButtonPressEvent(xproto::ButtonPressEvent::try_parse(bytes)?.0)),
            5 => return Ok(Self::XprotoButtonReleaseEvent(xproto::ButtonReleaseEvent::try_parse(bytes)?.0)),
            26 => return Ok(Self::XprotoCirculateNotifyEvent(xproto::CirculateNotifyEvent::try_parse(bytes)?.0)),
            27 => return Ok(Self::XprotoCirculateRequestEvent(xproto::CirculateRequestEvent::try_parse(bytes)?.0)),
            33 => return Ok(Self::XprotoClientMessageEvent(xproto::ClientMessageEvent::try_parse(bytes)?.0)),
            32 => return Ok(Self::XprotoColormapNotifyEvent(xproto::ColormapNotifyEvent::try_parse(bytes)?.0)),
            22 => return Ok(Self::XprotoConfigureNotifyEvent(xproto::ConfigureNotifyEvent::try_parse(bytes)?.0)),
            23 => return Ok(Self::XprotoConfigureRequestEvent(xproto::ConfigureRequestEvent::try_parse(bytes)?.0)),
            16 => return Ok(Self::XprotoCreateNotifyEvent(xproto::CreateNotifyEvent::try_parse(bytes)?.0)),
            17 => return Ok(Self::XprotoDestroyNotifyEvent(xproto::DestroyNotifyEvent::try_parse(bytes)?.0)),
            7 => return Ok(Self::XprotoEnterNotifyEvent(xproto::EnterNotifyEvent::try_parse(bytes)?.0)),
            12 => return Ok(Self::XprotoExposeEvent(xproto::ExposeEvent::try_parse(bytes)?.0)),
            9 => return Ok(Self::XprotoFocusInEvent(xproto::FocusInEvent::try_parse(bytes)?.0)),
            10 => return Ok(Self::XprotoFocusOutEvent(xproto::FocusOutEvent::try_parse(bytes)?.0)),
            13 => return Ok(Self::XprotoGraphicsExposureEvent(xproto::GraphicsExposureEvent::try_parse(bytes)?.0)),
            24 => return Ok(Self::XprotoGravityNotifyEvent(xproto::GravityNotifyEvent::try_parse(bytes)?.0)),
            2 => return Ok(Self::XprotoKeyPressEvent(xproto::KeyPressEvent::try_parse(bytes)?.0)),
            3 => return Ok(Self::XprotoKeyReleaseEvent(xproto::KeyReleaseEvent::try_parse(bytes)?.0)),
            11 => return Ok(Self::XprotoKeymapNotifyEvent(xproto::KeymapNotifyEvent::try_parse(bytes)?.0)),
            8 => return Ok(Self::XprotoLeaveNotifyEvent(xproto::LeaveNotifyEvent::try_parse(bytes)?.0)),
            19 => return Ok(Self::XprotoMapNotifyEvent(xproto::MapNotifyEvent::try_parse(bytes)?.0)),
            20 => return Ok(Self::XprotoMapRequestEvent(xproto::MapRequestEvent::try_parse(bytes)?.0)),
            34 => return Ok(Self::XprotoMappingNotifyEvent(xproto::MappingNotifyEvent::try_parse(bytes)?.0)),
            6 => return Ok(Self::XprotoMotionNotifyEvent(xproto::MotionNotifyEvent::try_parse(bytes)?.0)),
            14 => return Ok(Self::XprotoNoExposureEvent(xproto::NoExposureEvent::try_parse(bytes)?.0)),
            28 => return Ok(Self::XprotoPropertyNotifyEvent(xproto::PropertyNotifyEvent::try_parse(bytes)?.0)),
            21 => return Ok(Self::XprotoReparentNotifyEvent(xproto::ReparentNotifyEvent::try_parse(bytes)?.0)),
            25 => return Ok(Self::XprotoResizeRequestEvent(xproto::ResizeRequestEvent::try_parse(bytes)?.0)),
            29 => return Ok(Self::XprotoSelectionClearEvent(xproto::SelectionClearEvent::try_parse(bytes)?.0)),
            31 => return Ok(Self::XprotoSelectionNotifyEvent(xproto::SelectionNotifyEvent::try_parse(bytes)?.0)),
            30 => return Ok(Self::XprotoSelectionRequestEvent(xproto::SelectionRequestEvent::try_parse(bytes)?.0)),
            18 => return Ok(Self::XprotoUnmapNotifyEvent(xproto::UnmapNotifyEvent::try_parse(bytes)?.0)),
            15 => return Ok(Self::XprotoVisibilityNotifyEvent(xproto::VisibilityNotifyEvent::try_parse(bytes)?.0)),
            xproto::GE_GENERIC_EVENT => return Self::from_generic_event(event, iter),
            _ => {}
        }
        // Find the extension that this event could belong to
        let ext_info = iter
            .map(|(name, reply)| (name, reply.first_event))
            .filter(|&(_, first_event)| first_event <= event_type)
            .max_by_key(|&(_, first_event)| first_event);
        match ext_info {
            #[cfg(feature = "damage")]
            Some(("DAMAGE", first_event)) => {
                match event_type - first_event {
                    0 => Ok(Self::DamageNotifyEvent(damage::NotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "dri2")]
            Some(("DRI2", first_event)) => {
                match event_type - first_event {
                    0 => Ok(Self::Dri2BufferSwapCompleteEvent(dri2::BufferSwapCompleteEvent::try_parse(bytes)?.0)),
                    1 => Ok(Self::Dri2InvalidateBuffersEvent(dri2::InvalidateBuffersEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "glx")]
            Some(("GLX", first_event)) => {
                match event_type - first_event {
                    1 => Ok(Self::GlxBufferSwapCompleteEvent(glx::BufferSwapCompleteEvent::try_parse(bytes)?.0)),
                    0 => Ok(Self::GlxPbufferClobberEvent(glx::PbufferClobberEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "present")]
            Some(("Present", first_event)) => {
                match event_type - first_event {
                    0 => Ok(Self::PresentGenericEvent(present::GenericEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "randr")]
            Some(("RANDR", first_event)) => {
                match event_type - first_event {
                    1 => Ok(Self::RandrNotifyEvent(randr::NotifyEvent::try_parse(bytes)?.0)),
                    0 => Ok(Self::RandrScreenChangeNotifyEvent(randr::ScreenChangeNotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "screensaver")]
            Some(("MIT-SCREEN-SAVER", first_event)) => {
                match event_type - first_event {
                    0 => Ok(Self::ScreensaverNotifyEvent(screensaver::NotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "shape")]
            Some(("SHAPE", first_event)) => {
                match event_type - first_event {
                    0 => Ok(Self::ShapeNotifyEvent(shape::NotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "shm")]
            Some(("MIT-SHM", first_event)) => {
                match event_type - first_event {
                    0 => Ok(Self::ShmCompletionEvent(shm::CompletionEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "sync")]
            Some(("SYNC", first_event)) => {
                match event_type - first_event {
                    1 => Ok(Self::SyncAlarmNotifyEvent(sync::AlarmNotifyEvent::try_parse(bytes)?.0)),
                    0 => Ok(Self::SyncCounterNotifyEvent(sync::CounterNotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xfixes")]
            Some(("XFIXES", first_event)) => {
                match event_type - first_event {
                    1 => Ok(Self::XfixesCursorNotifyEvent(xfixes::CursorNotifyEvent::try_parse(bytes)?.0)),
                    0 => Ok(Self::XfixesSelectionNotifyEvent(xfixes::SelectionNotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xinput")]
            Some(("XInputExtension", first_event)) => {
                match event_type - first_event {
                    12 => Ok(Self::XinputChangeDeviceNotifyEvent(xinput::ChangeDeviceNotifyEvent::try_parse(bytes)?.0)),
                    3 => Ok(Self::XinputDeviceButtonPressEvent(xinput::DeviceButtonPressEvent::try_parse(bytes)?.0)),
                    4 => Ok(Self::XinputDeviceButtonReleaseEvent(xinput::DeviceButtonReleaseEvent::try_parse(bytes)?.0)),
                    14 => Ok(Self::XinputDeviceButtonStateNotifyEvent(xinput::DeviceButtonStateNotifyEvent::try_parse(bytes)?.0)),
                    6 => Ok(Self::XinputDeviceFocusInEvent(xinput::DeviceFocusInEvent::try_parse(bytes)?.0)),
                    7 => Ok(Self::XinputDeviceFocusOutEvent(xinput::DeviceFocusOutEvent::try_parse(bytes)?.0)),
                    1 => Ok(Self::XinputDeviceKeyPressEvent(xinput::DeviceKeyPressEvent::try_parse(bytes)?.0)),
                    2 => Ok(Self::XinputDeviceKeyReleaseEvent(xinput::DeviceKeyReleaseEvent::try_parse(bytes)?.0)),
                    13 => Ok(Self::XinputDeviceKeyStateNotifyEvent(xinput::DeviceKeyStateNotifyEvent::try_parse(bytes)?.0)),
                    11 => Ok(Self::XinputDeviceMappingNotifyEvent(xinput::DeviceMappingNotifyEvent::try_parse(bytes)?.0)),
                    5 => Ok(Self::XinputDeviceMotionNotifyEvent(xinput::DeviceMotionNotifyEvent::try_parse(bytes)?.0)),
                    15 => Ok(Self::XinputDevicePresenceNotifyEvent(xinput::DevicePresenceNotifyEvent::try_parse(bytes)?.0)),
                    16 => Ok(Self::XinputDevicePropertyNotifyEvent(xinput::DevicePropertyNotifyEvent::try_parse(bytes)?.0)),
                    10 => Ok(Self::XinputDeviceStateNotifyEvent(xinput::DeviceStateNotifyEvent::try_parse(bytes)?.0)),
                    0 => Ok(Self::XinputDeviceValuatorEvent(xinput::DeviceValuatorEvent::try_parse(bytes)?.0)),
                    8 => Ok(Self::XinputProximityInEvent(xinput::ProximityInEvent::try_parse(bytes)?.0)),
                    9 => Ok(Self::XinputProximityOutEvent(xinput::ProximityOutEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xkb")]
            Some(("XKEYBOARD", first_event)) => {
                match event_type - first_event {
                    10 => Ok(Self::XkbAccessXNotifyEvent(xkb::AccessXNotifyEvent::try_parse(bytes)?.0)),
                    9 => Ok(Self::XkbActionMessageEvent(xkb::ActionMessageEvent::try_parse(bytes)?.0)),
                    8 => Ok(Self::XkbBellNotifyEvent(xkb::BellNotifyEvent::try_parse(bytes)?.0)),
                    7 => Ok(Self::XkbCompatMapNotifyEvent(xkb::CompatMapNotifyEvent::try_parse(bytes)?.0)),
                    3 => Ok(Self::XkbControlsNotifyEvent(xkb::ControlsNotifyEvent::try_parse(bytes)?.0)),
                    11 => Ok(Self::XkbExtensionDeviceNotifyEvent(xkb::ExtensionDeviceNotifyEvent::try_parse(bytes)?.0)),
                    5 => Ok(Self::XkbIndicatorMapNotifyEvent(xkb::IndicatorMapNotifyEvent::try_parse(bytes)?.0)),
                    4 => Ok(Self::XkbIndicatorStateNotifyEvent(xkb::IndicatorStateNotifyEvent::try_parse(bytes)?.0)),
                    1 => Ok(Self::XkbMapNotifyEvent(xkb::MapNotifyEvent::try_parse(bytes)?.0)),
                    6 => Ok(Self::XkbNamesNotifyEvent(xkb::NamesNotifyEvent::try_parse(bytes)?.0)),
                    0 => Ok(Self::XkbNewKeyboardNotifyEvent(xkb::NewKeyboardNotifyEvent::try_parse(bytes)?.0)),
                    2 => Ok(Self::XkbStateNotifyEvent(xkb::StateNotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xprint")]
            Some(("XpExtension", first_event)) => {
                match event_type - first_event {
                    1 => Ok(Self::XprintAttributNotifyEvent(xprint::AttributNotifyEvent::try_parse(bytes)?.0)),
                    0 => Ok(Self::XprintNotifyEvent(xprint::NotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xv")]
            Some(("XVideo", first_event)) => {
                match event_type - first_event {
                    1 => Ok(Self::XvPortNotifyEvent(xv::PortNotifyEvent::try_parse(bytes)?.0)),
                    0 => Ok(Self::XvVideoNotifyEvent(xv::VideoNotifyEvent::try_parse(bytes)?.0)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            _ => Ok(Self::Unknown(event))
        }
    }

    fn from_generic_event(
        event: GenericEvent<B>,
        iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,
    ) -> Result<Self, ParseError> {
        todo!()
    }
}
