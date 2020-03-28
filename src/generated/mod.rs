// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

use std::convert::{TryFrom, TryInto};
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
        // Check if this is a core protocol error
        match error_code {
            xproto::ACCESS_ERROR => return Ok(Self::XprotoAccessError(error.into())),
            xproto::ALLOC_ERROR => return Ok(Self::XprotoAllocError(error.into())),
            xproto::ATOM_ERROR => return Ok(Self::XprotoAtomError(error.into())),
            xproto::COLORMAP_ERROR => return Ok(Self::XprotoColormapError(error.into())),
            xproto::CURSOR_ERROR => return Ok(Self::XprotoCursorError(error.into())),
            xproto::DRAWABLE_ERROR => return Ok(Self::XprotoDrawableError(error.into())),
            xproto::FONT_ERROR => return Ok(Self::XprotoFontError(error.into())),
            xproto::G_CONTEXT_ERROR => return Ok(Self::XprotoGContextError(error.into())),
            xproto::ID_CHOICE_ERROR => return Ok(Self::XprotoIDChoiceError(error.into())),
            xproto::IMPLEMENTATION_ERROR => return Ok(Self::XprotoImplementationError(error.into())),
            xproto::LENGTH_ERROR => return Ok(Self::XprotoLengthError(error.into())),
            xproto::MATCH_ERROR => return Ok(Self::XprotoMatchError(error.into())),
            xproto::NAME_ERROR => return Ok(Self::XprotoNameError(error.into())),
            xproto::PIXMAP_ERROR => return Ok(Self::XprotoPixmapError(error.into())),
            xproto::REQUEST_ERROR => return Ok(Self::XprotoRequestError(error.into())),
            xproto::VALUE_ERROR => return Ok(Self::XprotoValueError(error.into())),
            xproto::WINDOW_ERROR => return Ok(Self::XprotoWindowError(error.into())),
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
                    damage::BAD_DAMAGE_ERROR => Ok(Self::DamageBadDamageError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "glx")]
            Some(("GLX", first_error)) => {
                match error_code - first_error {
                    glx::BAD_CONTEXT_ERROR => Ok(Self::GlxBadContextError(error.into())),
                    glx::BAD_CONTEXT_STATE_ERROR => Ok(Self::GlxBadContextStateError(error.into())),
                    glx::BAD_CONTEXT_TAG_ERROR => Ok(Self::GlxBadContextTagError(error.into())),
                    glx::BAD_CURRENT_DRAWABLE_ERROR => Ok(Self::GlxBadCurrentDrawableError(error.into())),
                    glx::BAD_CURRENT_WINDOW_ERROR => Ok(Self::GlxBadCurrentWindowError(error.into())),
                    glx::BAD_DRAWABLE_ERROR => Ok(Self::GlxBadDrawableError(error.into())),
                    glx::BAD_FB_CONFIG_ERROR => Ok(Self::GlxBadFBConfigError(error.into())),
                    glx::BAD_LARGE_REQUEST_ERROR => Ok(Self::GlxBadLargeRequestError(error.into())),
                    glx::BAD_PBUFFER_ERROR => Ok(Self::GlxBadPbufferError(error.into())),
                    glx::BAD_PIXMAP_ERROR => Ok(Self::GlxBadPixmapError(error.into())),
                    glx::BAD_RENDER_REQUEST_ERROR => Ok(Self::GlxBadRenderRequestError(error.into())),
                    glx::BAD_WINDOW_ERROR => Ok(Self::GlxBadWindowError(error.into())),
                    glx::GLX_BAD_PROFILE_ARB_ERROR => Ok(Self::GlxGLXBadProfileARBError(error.into())),
                    glx::UNSUPPORTED_PRIVATE_REQUEST_ERROR => Ok(Self::GlxUnsupportedPrivateRequestError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "randr")]
            Some(("RANDR", first_error)) => {
                match error_code - first_error {
                    randr::BAD_CRTC_ERROR => Ok(Self::RandrBadCrtcError(error.into())),
                    randr::BAD_MODE_ERROR => Ok(Self::RandrBadModeError(error.into())),
                    randr::BAD_OUTPUT_ERROR => Ok(Self::RandrBadOutputError(error.into())),
                    randr::BAD_PROVIDER_ERROR => Ok(Self::RandrBadProviderError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "record")]
            Some(("RECORD", first_error)) => {
                match error_code - first_error {
                    record::BAD_CONTEXT_ERROR => Ok(Self::RecordBadContextError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "render")]
            Some(("RENDER", first_error)) => {
                match error_code - first_error {
                    render::GLYPH_ERROR => Ok(Self::RenderGlyphError(error.into())),
                    render::GLYPH_SET_ERROR => Ok(Self::RenderGlyphSetError(error.into())),
                    render::PICT_FORMAT_ERROR => Ok(Self::RenderPictFormatError(error.into())),
                    render::PICT_OP_ERROR => Ok(Self::RenderPictOpError(error.into())),
                    render::PICTURE_ERROR => Ok(Self::RenderPictureError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "shm")]
            Some(("MIT-SHM", first_error)) => {
                match error_code - first_error {
                    shm::BAD_SEG_ERROR => Ok(Self::ShmBadSegError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "sync")]
            Some(("SYNC", first_error)) => {
                match error_code - first_error {
                    sync::ALARM_ERROR => Ok(Self::SyncAlarmError(error.into())),
                    sync::COUNTER_ERROR => Ok(Self::SyncCounterError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xf86vidmode")]
            Some(("XFree86-VidModeExtension", first_error)) => {
                match error_code - first_error {
                    xf86vidmode::BAD_CLOCK_ERROR => Ok(Self::Xf86vidmodeBadClockError(error.into())),
                    xf86vidmode::BAD_H_TIMINGS_ERROR => Ok(Self::Xf86vidmodeBadHTimingsError(error.into())),
                    xf86vidmode::BAD_V_TIMINGS_ERROR => Ok(Self::Xf86vidmodeBadVTimingsError(error.into())),
                    xf86vidmode::CLIENT_NOT_LOCAL_ERROR => Ok(Self::Xf86vidmodeClientNotLocalError(error.into())),
                    xf86vidmode::EXTENSION_DISABLED_ERROR => Ok(Self::Xf86vidmodeExtensionDisabledError(error.into())),
                    xf86vidmode::MODE_UNSUITABLE_ERROR => Ok(Self::Xf86vidmodeModeUnsuitableError(error.into())),
                    xf86vidmode::ZOOM_LOCKED_ERROR => Ok(Self::Xf86vidmodeZoomLockedError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xfixes")]
            Some(("XFIXES", first_error)) => {
                match error_code - first_error {
                    xfixes::BAD_REGION_ERROR => Ok(Self::XfixesBadRegionError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xinput")]
            Some(("XInputExtension", first_error)) => {
                match error_code - first_error {
                    xinput::CLASS_ERROR => Ok(Self::XinputClassError(error.into())),
                    xinput::DEVICE_ERROR => Ok(Self::XinputDeviceError(error.into())),
                    xinput::DEVICE_BUSY_ERROR => Ok(Self::XinputDeviceBusyError(error.into())),
                    xinput::EVENT_ERROR => Ok(Self::XinputEventError(error.into())),
                    xinput::MODE_ERROR => Ok(Self::XinputModeError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xkb")]
            Some(("XKEYBOARD", first_error)) => {
                match error_code - first_error {
                    xkb::KEYBOARD_ERROR => Ok(Self::XkbKeyboardError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xprint")]
            Some(("XpExtension", first_error)) => {
                match error_code - first_error {
                    xprint::BAD_CONTEXT_ERROR => Ok(Self::XprintBadContextError(error.into())),
                    xprint::BAD_SEQUENCE_ERROR => Ok(Self::XprintBadSequenceError(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xv")]
            Some(("XVideo", first_error)) => {
                match error_code - first_error {
                    xv::BAD_CONTROL_ERROR => Ok(Self::XvBadControlError(error.into())),
                    xv::BAD_ENCODING_ERROR => Ok(Self::XvBadEncodingError(error.into())),
                    xv::BAD_PORT_ERROR => Ok(Self::XvBadPortError(error.into())),
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
        // Check if this is a core protocol error or from the generic event extension
        match event_type {
            0 => return Ok(Self::Error(Error::parse(event.try_into()?, iter)?)),
            xproto::BUTTON_PRESS_EVENT => return Ok(Self::XprotoButtonPressEvent(event.into())),
            xproto::BUTTON_RELEASE_EVENT => return Ok(Self::XprotoButtonReleaseEvent(event.into())),
            xproto::CIRCULATE_NOTIFY_EVENT => return Ok(Self::XprotoCirculateNotifyEvent(event.into())),
            xproto::CIRCULATE_REQUEST_EVENT => return Ok(Self::XprotoCirculateRequestEvent(event.into())),
            xproto::CLIENT_MESSAGE_EVENT => return Ok(Self::XprotoClientMessageEvent(event.into())),
            xproto::COLORMAP_NOTIFY_EVENT => return Ok(Self::XprotoColormapNotifyEvent(event.into())),
            xproto::CONFIGURE_NOTIFY_EVENT => return Ok(Self::XprotoConfigureNotifyEvent(event.into())),
            xproto::CONFIGURE_REQUEST_EVENT => return Ok(Self::XprotoConfigureRequestEvent(event.into())),
            xproto::CREATE_NOTIFY_EVENT => return Ok(Self::XprotoCreateNotifyEvent(event.into())),
            xproto::DESTROY_NOTIFY_EVENT => return Ok(Self::XprotoDestroyNotifyEvent(event.into())),
            xproto::ENTER_NOTIFY_EVENT => return Ok(Self::XprotoEnterNotifyEvent(event.into())),
            xproto::EXPOSE_EVENT => return Ok(Self::XprotoExposeEvent(event.into())),
            xproto::FOCUS_IN_EVENT => return Ok(Self::XprotoFocusInEvent(event.into())),
            xproto::FOCUS_OUT_EVENT => return Ok(Self::XprotoFocusOutEvent(event.into())),
            xproto::GRAPHICS_EXPOSURE_EVENT => return Ok(Self::XprotoGraphicsExposureEvent(event.into())),
            xproto::GRAVITY_NOTIFY_EVENT => return Ok(Self::XprotoGravityNotifyEvent(event.into())),
            xproto::KEY_PRESS_EVENT => return Ok(Self::XprotoKeyPressEvent(event.into())),
            xproto::KEY_RELEASE_EVENT => return Ok(Self::XprotoKeyReleaseEvent(event.into())),
            xproto::KEYMAP_NOTIFY_EVENT => return Ok(Self::XprotoKeymapNotifyEvent(event.into())),
            xproto::LEAVE_NOTIFY_EVENT => return Ok(Self::XprotoLeaveNotifyEvent(event.into())),
            xproto::MAP_NOTIFY_EVENT => return Ok(Self::XprotoMapNotifyEvent(event.into())),
            xproto::MAP_REQUEST_EVENT => return Ok(Self::XprotoMapRequestEvent(event.into())),
            xproto::MAPPING_NOTIFY_EVENT => return Ok(Self::XprotoMappingNotifyEvent(event.into())),
            xproto::MOTION_NOTIFY_EVENT => return Ok(Self::XprotoMotionNotifyEvent(event.into())),
            xproto::NO_EXPOSURE_EVENT => return Ok(Self::XprotoNoExposureEvent(event.into())),
            xproto::PROPERTY_NOTIFY_EVENT => return Ok(Self::XprotoPropertyNotifyEvent(event.into())),
            xproto::REPARENT_NOTIFY_EVENT => return Ok(Self::XprotoReparentNotifyEvent(event.into())),
            xproto::RESIZE_REQUEST_EVENT => return Ok(Self::XprotoResizeRequestEvent(event.into())),
            xproto::SELECTION_CLEAR_EVENT => return Ok(Self::XprotoSelectionClearEvent(event.into())),
            xproto::SELECTION_NOTIFY_EVENT => return Ok(Self::XprotoSelectionNotifyEvent(event.into())),
            xproto::SELECTION_REQUEST_EVENT => return Ok(Self::XprotoSelectionRequestEvent(event.into())),
            xproto::UNMAP_NOTIFY_EVENT => return Ok(Self::XprotoUnmapNotifyEvent(event.into())),
            xproto::VISIBILITY_NOTIFY_EVENT => return Ok(Self::XprotoVisibilityNotifyEvent(event.into())),
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
                    damage::NOTIFY_EVENT => Ok(Self::DamageNotifyEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "dri2")]
            Some(("DRI2", first_event)) => {
                match event_type - first_event {
                    dri2::BUFFER_SWAP_COMPLETE_EVENT => Ok(Self::Dri2BufferSwapCompleteEvent(event.into())),
                    dri2::INVALIDATE_BUFFERS_EVENT => Ok(Self::Dri2InvalidateBuffersEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "glx")]
            Some(("GLX", first_event)) => {
                match event_type - first_event {
                    glx::BUFFER_SWAP_COMPLETE_EVENT => Ok(Self::GlxBufferSwapCompleteEvent(event.into())),
                    glx::PBUFFER_CLOBBER_EVENT => Ok(Self::GlxPbufferClobberEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "present")]
            Some(("Present", first_event)) => {
                match event_type - first_event {
                    present::GENERIC_EVENT => Ok(Self::PresentGenericEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "randr")]
            Some(("RANDR", first_event)) => {
                match event_type - first_event {
                    randr::NOTIFY_EVENT => Ok(Self::RandrNotifyEvent(event.into())),
                    randr::SCREEN_CHANGE_NOTIFY_EVENT => Ok(Self::RandrScreenChangeNotifyEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "screensaver")]
            Some(("MIT-SCREEN-SAVER", first_event)) => {
                match event_type - first_event {
                    screensaver::NOTIFY_EVENT => Ok(Self::ScreensaverNotifyEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "shape")]
            Some(("SHAPE", first_event)) => {
                match event_type - first_event {
                    shape::NOTIFY_EVENT => Ok(Self::ShapeNotifyEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "shm")]
            Some(("MIT-SHM", first_event)) => {
                match event_type - first_event {
                    shm::COMPLETION_EVENT => Ok(Self::ShmCompletionEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "sync")]
            Some(("SYNC", first_event)) => {
                match event_type - first_event {
                    sync::ALARM_NOTIFY_EVENT => Ok(Self::SyncAlarmNotifyEvent(event.into())),
                    sync::COUNTER_NOTIFY_EVENT => Ok(Self::SyncCounterNotifyEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xfixes")]
            Some(("XFIXES", first_event)) => {
                match event_type - first_event {
                    xfixes::CURSOR_NOTIFY_EVENT => Ok(Self::XfixesCursorNotifyEvent(event.into())),
                    xfixes::SELECTION_NOTIFY_EVENT => Ok(Self::XfixesSelectionNotifyEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xinput")]
            Some(("XInputExtension", first_event)) => {
                match event_type - first_event {
                    xinput::CHANGE_DEVICE_NOTIFY_EVENT => Ok(Self::XinputChangeDeviceNotifyEvent(event.into())),
                    xinput::DEVICE_BUTTON_PRESS_EVENT => Ok(Self::XinputDeviceButtonPressEvent(event.into())),
                    xinput::DEVICE_BUTTON_RELEASE_EVENT => Ok(Self::XinputDeviceButtonReleaseEvent(event.into())),
                    xinput::DEVICE_BUTTON_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceButtonStateNotifyEvent(event.into())),
                    xinput::DEVICE_FOCUS_IN_EVENT => Ok(Self::XinputDeviceFocusInEvent(event.into())),
                    xinput::DEVICE_FOCUS_OUT_EVENT => Ok(Self::XinputDeviceFocusOutEvent(event.into())),
                    xinput::DEVICE_KEY_PRESS_EVENT => Ok(Self::XinputDeviceKeyPressEvent(event.into())),
                    xinput::DEVICE_KEY_RELEASE_EVENT => Ok(Self::XinputDeviceKeyReleaseEvent(event.into())),
                    xinput::DEVICE_KEY_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceKeyStateNotifyEvent(event.into())),
                    xinput::DEVICE_MAPPING_NOTIFY_EVENT => Ok(Self::XinputDeviceMappingNotifyEvent(event.into())),
                    xinput::DEVICE_MOTION_NOTIFY_EVENT => Ok(Self::XinputDeviceMotionNotifyEvent(event.into())),
                    xinput::DEVICE_PRESENCE_NOTIFY_EVENT => Ok(Self::XinputDevicePresenceNotifyEvent(event.into())),
                    xinput::DEVICE_PROPERTY_NOTIFY_EVENT => Ok(Self::XinputDevicePropertyNotifyEvent(event.into())),
                    xinput::DEVICE_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceStateNotifyEvent(event.into())),
                    xinput::DEVICE_VALUATOR_EVENT => Ok(Self::XinputDeviceValuatorEvent(event.into())),
                    xinput::PROXIMITY_IN_EVENT => Ok(Self::XinputProximityInEvent(event.into())),
                    xinput::PROXIMITY_OUT_EVENT => Ok(Self::XinputProximityOutEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xkb")]
            Some(("XKEYBOARD", first_event)) => {
                if event_type != first_event {
                    return Ok(Self::Unknown(event));
                }
                match event.raw_bytes()[1] {
                    xkb::ACCESS_X_NOTIFY_EVENT => Ok(Self::XkbAccessXNotifyEvent(event.into())),
                    xkb::ACTION_MESSAGE_EVENT => Ok(Self::XkbActionMessageEvent(event.into())),
                    xkb::BELL_NOTIFY_EVENT => Ok(Self::XkbBellNotifyEvent(event.into())),
                    xkb::COMPAT_MAP_NOTIFY_EVENT => Ok(Self::XkbCompatMapNotifyEvent(event.into())),
                    xkb::CONTROLS_NOTIFY_EVENT => Ok(Self::XkbControlsNotifyEvent(event.into())),
                    xkb::EXTENSION_DEVICE_NOTIFY_EVENT => Ok(Self::XkbExtensionDeviceNotifyEvent(event.into())),
                    xkb::INDICATOR_MAP_NOTIFY_EVENT => Ok(Self::XkbIndicatorMapNotifyEvent(event.into())),
                    xkb::INDICATOR_STATE_NOTIFY_EVENT => Ok(Self::XkbIndicatorStateNotifyEvent(event.into())),
                    xkb::MAP_NOTIFY_EVENT => Ok(Self::XkbMapNotifyEvent(event.into())),
                    xkb::NAMES_NOTIFY_EVENT => Ok(Self::XkbNamesNotifyEvent(event.into())),
                    xkb::NEW_KEYBOARD_NOTIFY_EVENT => Ok(Self::XkbNewKeyboardNotifyEvent(event.into())),
                    xkb::STATE_NOTIFY_EVENT => Ok(Self::XkbStateNotifyEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xprint")]
            Some(("XpExtension", first_event)) => {
                match event_type - first_event {
                    xprint::ATTRIBUT_NOTIFY_EVENT => Ok(Self::XprintAttributNotifyEvent(event.into())),
                    xprint::NOTIFY_EVENT => Ok(Self::XprintNotifyEvent(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xv")]
            Some(("XVideo", first_event)) => {
                match event_type - first_event {
                    xv::PORT_NOTIFY_EVENT => Ok(Self::XvPortNotifyEvent(event.into())),
                    xv::VIDEO_NOTIFY_EVENT => Ok(Self::XvVideoNotifyEvent(event.into())),
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
        let bytes = event.raw_bytes();
        let ge_event = xproto::GeGenericEvent::try_from(bytes)?;
        let (extension, event_type) = (ge_event.extension, ge_event.event_type);
        let ext_name = iter
            .map(|(name, reply)| (name, reply.major_opcode))
            .find(|&(_, opcode)| extension == opcode)
            .map(|(name, _)| name);
        match ext_name {
            #[cfg(feature = "present")]
            Some("Present") => {
                match event_type {
                    present::COMPLETE_NOTIFY_EVENT => Ok(Self::PresentCompleteNotifyEvent(event.try_into()?)),
                    present::CONFIGURE_NOTIFY_EVENT => Ok(Self::PresentConfigureNotifyEvent(event.try_into()?)),
                    present::IDLE_NOTIFY_EVENT => Ok(Self::PresentIdleNotifyEvent(event.try_into()?)),
                    present::REDIRECT_NOTIFY_EVENT => Ok(Self::PresentRedirectNotifyEvent(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xinput")]
            Some("XInputExtension") => {
                match event_type {
                    xinput::BARRIER_HIT_EVENT => Ok(Self::XinputBarrierHitEvent(event.try_into()?)),
                    xinput::BARRIER_LEAVE_EVENT => Ok(Self::XinputBarrierLeaveEvent(event.try_into()?)),
                    xinput::BUTTON_PRESS_EVENT => Ok(Self::XinputButtonPressEvent(event.try_into()?)),
                    xinput::BUTTON_RELEASE_EVENT => Ok(Self::XinputButtonReleaseEvent(event.try_into()?)),
                    xinput::DEVICE_CHANGED_EVENT => Ok(Self::XinputDeviceChangedEvent(event.try_into()?)),
                    xinput::ENTER_EVENT => Ok(Self::XinputEnterEvent(event.try_into()?)),
                    xinput::FOCUS_IN_EVENT => Ok(Self::XinputFocusInEvent(event.try_into()?)),
                    xinput::FOCUS_OUT_EVENT => Ok(Self::XinputFocusOutEvent(event.try_into()?)),
                    xinput::HIERARCHY_EVENT => Ok(Self::XinputHierarchyEvent(event.try_into()?)),
                    xinput::KEY_PRESS_EVENT => Ok(Self::XinputKeyPressEvent(event.try_into()?)),
                    xinput::KEY_RELEASE_EVENT => Ok(Self::XinputKeyReleaseEvent(event.try_into()?)),
                    xinput::LEAVE_EVENT => Ok(Self::XinputLeaveEvent(event.try_into()?)),
                    xinput::MOTION_EVENT => Ok(Self::XinputMotionEvent(event.try_into()?)),
                    xinput::PROPERTY_EVENT => Ok(Self::XinputPropertyEvent(event.try_into()?)),
                    xinput::RAW_BUTTON_PRESS_EVENT => Ok(Self::XinputRawButtonPressEvent(event.try_into()?)),
                    xinput::RAW_BUTTON_RELEASE_EVENT => Ok(Self::XinputRawButtonReleaseEvent(event.try_into()?)),
                    xinput::RAW_KEY_PRESS_EVENT => Ok(Self::XinputRawKeyPressEvent(event.try_into()?)),
                    xinput::RAW_KEY_RELEASE_EVENT => Ok(Self::XinputRawKeyReleaseEvent(event.try_into()?)),
                    xinput::RAW_MOTION_EVENT => Ok(Self::XinputRawMotionEvent(event.try_into()?)),
                    xinput::RAW_TOUCH_BEGIN_EVENT => Ok(Self::XinputRawTouchBeginEvent(event.try_into()?)),
                    xinput::RAW_TOUCH_END_EVENT => Ok(Self::XinputRawTouchEndEvent(event.try_into()?)),
                    xinput::RAW_TOUCH_UPDATE_EVENT => Ok(Self::XinputRawTouchUpdateEvent(event.try_into()?)),
                    xinput::TOUCH_BEGIN_EVENT => Ok(Self::XinputTouchBeginEvent(event.try_into()?)),
                    xinput::TOUCH_END_EVENT => Ok(Self::XinputTouchEndEvent(event.try_into()?)),
                    xinput::TOUCH_OWNERSHIP_EVENT => Ok(Self::XinputTouchOwnershipEvent(event.try_into()?)),
                    xinput::TOUCH_UPDATE_EVENT => Ok(Self::XinputTouchUpdateEvent(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            _ => Ok(Self::Unknown(event))
        }
    }
}
