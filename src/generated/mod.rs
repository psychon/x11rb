// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

use std::convert::{TryFrom, TryInto};
use crate::errors::ParseError;
use crate::x11_utils::{Event as _, ExtensionInformation, GenericError, GenericEvent};
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
    DamageBadDamage(damage::BadDamageError),
    #[cfg(feature = "glx")]
    GlxBadContext(glx::BadContextError),
    #[cfg(feature = "glx")]
    GlxBadContextState(glx::BadContextStateError),
    #[cfg(feature = "glx")]
    GlxBadContextTag(glx::BadContextTagError),
    #[cfg(feature = "glx")]
    GlxBadCurrentDrawable(glx::BadCurrentDrawableError),
    #[cfg(feature = "glx")]
    GlxBadCurrentWindow(glx::BadCurrentWindowError),
    #[cfg(feature = "glx")]
    GlxBadDrawable(glx::BadDrawableError),
    #[cfg(feature = "glx")]
    GlxBadFBConfig(glx::BadFBConfigError),
    #[cfg(feature = "glx")]
    GlxBadLargeRequest(glx::BadLargeRequestError),
    #[cfg(feature = "glx")]
    GlxBadPbuffer(glx::BadPbufferError),
    #[cfg(feature = "glx")]
    GlxBadPixmap(glx::BadPixmapError),
    #[cfg(feature = "glx")]
    GlxBadRenderRequest(glx::BadRenderRequestError),
    #[cfg(feature = "glx")]
    GlxBadWindow(glx::BadWindowError),
    #[cfg(feature = "glx")]
    GlxGLXBadProfileARB(glx::GLXBadProfileARBError),
    #[cfg(feature = "glx")]
    GlxUnsupportedPrivateRequest(glx::UnsupportedPrivateRequestError),
    #[cfg(feature = "randr")]
    RandrBadCrtc(randr::BadCrtcError),
    #[cfg(feature = "randr")]
    RandrBadMode(randr::BadModeError),
    #[cfg(feature = "randr")]
    RandrBadOutput(randr::BadOutputError),
    #[cfg(feature = "randr")]
    RandrBadProvider(randr::BadProviderError),
    #[cfg(feature = "record")]
    RecordBadContext(record::BadContextError),
    #[cfg(feature = "render")]
    RenderGlyph(render::GlyphError),
    #[cfg(feature = "render")]
    RenderGlyphSet(render::GlyphSetError),
    #[cfg(feature = "render")]
    RenderPictFormat(render::PictFormatError),
    #[cfg(feature = "render")]
    RenderPictOp(render::PictOpError),
    #[cfg(feature = "render")]
    RenderPicture(render::PictureError),
    #[cfg(feature = "shm")]
    ShmBadSeg(shm::BadSegError),
    #[cfg(feature = "sync")]
    SyncAlarm(sync::AlarmError),
    #[cfg(feature = "sync")]
    SyncCounter(sync::CounterError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadClock(xf86vidmode::BadClockError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadHTimings(xf86vidmode::BadHTimingsError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadVTimings(xf86vidmode::BadVTimingsError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeClientNotLocal(xf86vidmode::ClientNotLocalError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeExtensionDisabled(xf86vidmode::ExtensionDisabledError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeModeUnsuitable(xf86vidmode::ModeUnsuitableError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeZoomLocked(xf86vidmode::ZoomLockedError),
    #[cfg(feature = "xfixes")]
    XfixesBadRegion(xfixes::BadRegionError),
    #[cfg(feature = "xinput")]
    XinputClass(xinput::ClassError),
    #[cfg(feature = "xinput")]
    XinputDevice(xinput::DeviceError),
    #[cfg(feature = "xinput")]
    XinputDeviceBusy(xinput::DeviceBusyError),
    #[cfg(feature = "xinput")]
    XinputEvent(xinput::EventError),
    #[cfg(feature = "xinput")]
    XinputMode(xinput::ModeError),
    #[cfg(feature = "xkb")]
    XkbKeyboard(xkb::KeyboardError),
    #[cfg(feature = "xprint")]
    XprintBadContext(xprint::BadContextError),
    #[cfg(feature = "xprint")]
    XprintBadSequence(xprint::BadSequenceError),
    Access(xproto::AccessError),
    Alloc(xproto::AllocError),
    Atom(xproto::AtomError),
    Colormap(xproto::ColormapError),
    Cursor(xproto::CursorError),
    Drawable(xproto::DrawableError),
    Font(xproto::FontError),
    GContext(xproto::GContextError),
    IDChoice(xproto::IDChoiceError),
    Implementation(xproto::ImplementationError),
    Length(xproto::LengthError),
    Match(xproto::MatchError),
    Name(xproto::NameError),
    Pixmap(xproto::PixmapError),
    Request(xproto::RequestError),
    Value(xproto::ValueError),
    Window(xproto::WindowError),
    #[cfg(feature = "xv")]
    XvBadControl(xv::BadControlError),
    #[cfg(feature = "xv")]
    XvBadEncoding(xv::BadEncodingError),
    #[cfg(feature = "xv")]
    XvBadPort(xv::BadPortError),
}
impl<B: std::fmt::Debug + AsRef<[u8]>> Error<B> {
    /// Parse a generic X11 error into a concrete error type.
    pub fn parse(
        error: GenericError<B>,
        iter: impl Iterator<Item=(&'static str, ExtensionInformation)>,
    ) -> Result<Self, ParseError> {
        let error_code = error.error_code();
        // Check if this is a core protocol error
        match error_code {
            xproto::ACCESS_ERROR => return Ok(Self::Access(error.into())),
            xproto::ALLOC_ERROR => return Ok(Self::Alloc(error.into())),
            xproto::ATOM_ERROR => return Ok(Self::Atom(error.into())),
            xproto::COLORMAP_ERROR => return Ok(Self::Colormap(error.into())),
            xproto::CURSOR_ERROR => return Ok(Self::Cursor(error.into())),
            xproto::DRAWABLE_ERROR => return Ok(Self::Drawable(error.into())),
            xproto::FONT_ERROR => return Ok(Self::Font(error.into())),
            xproto::G_CONTEXT_ERROR => return Ok(Self::GContext(error.into())),
            xproto::ID_CHOICE_ERROR => return Ok(Self::IDChoice(error.into())),
            xproto::IMPLEMENTATION_ERROR => return Ok(Self::Implementation(error.into())),
            xproto::LENGTH_ERROR => return Ok(Self::Length(error.into())),
            xproto::MATCH_ERROR => return Ok(Self::Match(error.into())),
            xproto::NAME_ERROR => return Ok(Self::Name(error.into())),
            xproto::PIXMAP_ERROR => return Ok(Self::Pixmap(error.into())),
            xproto::REQUEST_ERROR => return Ok(Self::Request(error.into())),
            xproto::VALUE_ERROR => return Ok(Self::Value(error.into())),
            xproto::WINDOW_ERROR => return Ok(Self::Window(error.into())),
            _ => {}
        }
        // Find the extension that this error could belong to
        let ext_info = iter
            .map(|(name, ext_info)| (name, ext_info.first_error))
            .filter(|&(_, first_error)| first_error <= error_code)
            .max_by_key(|&(_, first_error)| first_error);
        match ext_info {
            #[cfg(feature = "damage")]
            Some(("DAMAGE", first_error)) => {
                match error_code - first_error {
                    damage::BAD_DAMAGE_ERROR => Ok(Self::DamageBadDamage(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "glx")]
            Some(("GLX", first_error)) => {
                match error_code - first_error {
                    glx::BAD_CONTEXT_ERROR => Ok(Self::GlxBadContext(error.into())),
                    glx::BAD_CONTEXT_STATE_ERROR => Ok(Self::GlxBadContextState(error.into())),
                    glx::BAD_CONTEXT_TAG_ERROR => Ok(Self::GlxBadContextTag(error.into())),
                    glx::BAD_CURRENT_DRAWABLE_ERROR => Ok(Self::GlxBadCurrentDrawable(error.into())),
                    glx::BAD_CURRENT_WINDOW_ERROR => Ok(Self::GlxBadCurrentWindow(error.into())),
                    glx::BAD_DRAWABLE_ERROR => Ok(Self::GlxBadDrawable(error.into())),
                    glx::BAD_FB_CONFIG_ERROR => Ok(Self::GlxBadFBConfig(error.into())),
                    glx::BAD_LARGE_REQUEST_ERROR => Ok(Self::GlxBadLargeRequest(error.into())),
                    glx::BAD_PBUFFER_ERROR => Ok(Self::GlxBadPbuffer(error.into())),
                    glx::BAD_PIXMAP_ERROR => Ok(Self::GlxBadPixmap(error.into())),
                    glx::BAD_RENDER_REQUEST_ERROR => Ok(Self::GlxBadRenderRequest(error.into())),
                    glx::BAD_WINDOW_ERROR => Ok(Self::GlxBadWindow(error.into())),
                    glx::GLX_BAD_PROFILE_ARB_ERROR => Ok(Self::GlxGLXBadProfileARB(error.into())),
                    glx::UNSUPPORTED_PRIVATE_REQUEST_ERROR => Ok(Self::GlxUnsupportedPrivateRequest(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "randr")]
            Some(("RANDR", first_error)) => {
                match error_code - first_error {
                    randr::BAD_CRTC_ERROR => Ok(Self::RandrBadCrtc(error.into())),
                    randr::BAD_MODE_ERROR => Ok(Self::RandrBadMode(error.into())),
                    randr::BAD_OUTPUT_ERROR => Ok(Self::RandrBadOutput(error.into())),
                    randr::BAD_PROVIDER_ERROR => Ok(Self::RandrBadProvider(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "record")]
            Some(("RECORD", first_error)) => {
                match error_code - first_error {
                    record::BAD_CONTEXT_ERROR => Ok(Self::RecordBadContext(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "render")]
            Some(("RENDER", first_error)) => {
                match error_code - first_error {
                    render::GLYPH_ERROR => Ok(Self::RenderGlyph(error.into())),
                    render::GLYPH_SET_ERROR => Ok(Self::RenderGlyphSet(error.into())),
                    render::PICT_FORMAT_ERROR => Ok(Self::RenderPictFormat(error.into())),
                    render::PICT_OP_ERROR => Ok(Self::RenderPictOp(error.into())),
                    render::PICTURE_ERROR => Ok(Self::RenderPicture(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "shm")]
            Some(("MIT-SHM", first_error)) => {
                match error_code - first_error {
                    shm::BAD_SEG_ERROR => Ok(Self::ShmBadSeg(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "sync")]
            Some(("SYNC", first_error)) => {
                match error_code - first_error {
                    sync::ALARM_ERROR => Ok(Self::SyncAlarm(error.into())),
                    sync::COUNTER_ERROR => Ok(Self::SyncCounter(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xf86vidmode")]
            Some(("XFree86-VidModeExtension", first_error)) => {
                match error_code - first_error {
                    xf86vidmode::BAD_CLOCK_ERROR => Ok(Self::Xf86vidmodeBadClock(error.into())),
                    xf86vidmode::BAD_H_TIMINGS_ERROR => Ok(Self::Xf86vidmodeBadHTimings(error.into())),
                    xf86vidmode::BAD_V_TIMINGS_ERROR => Ok(Self::Xf86vidmodeBadVTimings(error.into())),
                    xf86vidmode::CLIENT_NOT_LOCAL_ERROR => Ok(Self::Xf86vidmodeClientNotLocal(error.into())),
                    xf86vidmode::EXTENSION_DISABLED_ERROR => Ok(Self::Xf86vidmodeExtensionDisabled(error.into())),
                    xf86vidmode::MODE_UNSUITABLE_ERROR => Ok(Self::Xf86vidmodeModeUnsuitable(error.into())),
                    xf86vidmode::ZOOM_LOCKED_ERROR => Ok(Self::Xf86vidmodeZoomLocked(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xfixes")]
            Some(("XFIXES", first_error)) => {
                match error_code - first_error {
                    xfixes::BAD_REGION_ERROR => Ok(Self::XfixesBadRegion(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xinput")]
            Some(("XInputExtension", first_error)) => {
                match error_code - first_error {
                    xinput::CLASS_ERROR => Ok(Self::XinputClass(error.into())),
                    xinput::DEVICE_ERROR => Ok(Self::XinputDevice(error.into())),
                    xinput::DEVICE_BUSY_ERROR => Ok(Self::XinputDeviceBusy(error.into())),
                    xinput::EVENT_ERROR => Ok(Self::XinputEvent(error.into())),
                    xinput::MODE_ERROR => Ok(Self::XinputMode(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xkb")]
            Some(("XKEYBOARD", first_error)) => {
                match error_code - first_error {
                    xkb::KEYBOARD_ERROR => Ok(Self::XkbKeyboard(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xprint")]
            Some(("XpExtension", first_error)) => {
                match error_code - first_error {
                    xprint::BAD_CONTEXT_ERROR => Ok(Self::XprintBadContext(error.into())),
                    xprint::BAD_SEQUENCE_ERROR => Ok(Self::XprintBadSequence(error.into())),
                    _ => Ok(Self::Unknown(error))
                }
            }
            #[cfg(feature = "xv")]
            Some(("XVideo", first_error)) => {
                match error_code - first_error {
                    xv::BAD_CONTROL_ERROR => Ok(Self::XvBadControl(error.into())),
                    xv::BAD_ENCODING_ERROR => Ok(Self::XvBadEncoding(error.into())),
                    xv::BAD_PORT_ERROR => Ok(Self::XvBadPort(error.into())),
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
    DamageNotify(damage::NotifyEvent),
    #[cfg(feature = "dri2")]
    Dri2BufferSwapComplete(dri2::BufferSwapCompleteEvent),
    #[cfg(feature = "dri2")]
    Dri2InvalidateBuffers(dri2::InvalidateBuffersEvent),
    #[cfg(feature = "glx")]
    GlxBufferSwapComplete(glx::BufferSwapCompleteEvent),
    #[cfg(feature = "glx")]
    GlxPbufferClobber(glx::PbufferClobberEvent),
    #[cfg(feature = "present")]
    PresentCompleteNotify(present::CompleteNotifyEvent),
    #[cfg(feature = "present")]
    PresentConfigureNotify(present::ConfigureNotifyEvent),
    #[cfg(feature = "present")]
    PresentGeneric(present::GenericEvent),
    #[cfg(feature = "present")]
    PresentIdleNotify(present::IdleNotifyEvent),
    #[cfg(feature = "present")]
    PresentRedirectNotify(present::RedirectNotifyEvent),
    #[cfg(feature = "randr")]
    RandrNotify(randr::NotifyEvent),
    #[cfg(feature = "randr")]
    RandrScreenChangeNotify(randr::ScreenChangeNotifyEvent),
    #[cfg(feature = "screensaver")]
    ScreensaverNotify(screensaver::NotifyEvent),
    #[cfg(feature = "shape")]
    ShapeNotify(shape::NotifyEvent),
    #[cfg(feature = "shm")]
    ShmCompletion(shm::CompletionEvent),
    #[cfg(feature = "sync")]
    SyncAlarmNotify(sync::AlarmNotifyEvent),
    #[cfg(feature = "sync")]
    SyncCounterNotify(sync::CounterNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesCursorNotify(xfixes::CursorNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesSelectionNotify(xfixes::SelectionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputBarrierHit(xinput::BarrierHitEvent),
    #[cfg(feature = "xinput")]
    XinputBarrierLeave(xinput::BarrierLeaveEvent),
    #[cfg(feature = "xinput")]
    XinputButtonPress(xinput::ButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputButtonRelease(xinput::ButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceNotify(xinput::ChangeDeviceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonPress(xinput::DeviceButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonRelease(xinput::DeviceButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonStateNotify(xinput::DeviceButtonStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceChanged(xinput::DeviceChangedEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusIn(xinput::DeviceFocusInEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusOut(xinput::DeviceFocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyPress(xinput::DeviceKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyRelease(xinput::DeviceKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyStateNotify(xinput::DeviceKeyStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMappingNotify(xinput::DeviceMappingNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMotionNotify(xinput::DeviceMotionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePresenceNotify(xinput::DevicePresenceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePropertyNotify(xinput::DevicePropertyNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceStateNotify(xinput::DeviceStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceValuator(xinput::DeviceValuatorEvent),
    #[cfg(feature = "xinput")]
    XinputEnter(xinput::EnterEvent),
    #[cfg(feature = "xinput")]
    XinputFocusIn(xinput::FocusInEvent),
    #[cfg(feature = "xinput")]
    XinputFocusOut(xinput::FocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputHierarchy(xinput::HierarchyEvent),
    #[cfg(feature = "xinput")]
    XinputKeyPress(xinput::KeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputKeyRelease(xinput::KeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputLeave(xinput::LeaveEvent),
    #[cfg(feature = "xinput")]
    XinputMotion(xinput::MotionEvent),
    #[cfg(feature = "xinput")]
    XinputProperty(xinput::PropertyEvent),
    #[cfg(feature = "xinput")]
    XinputProximityIn(xinput::ProximityInEvent),
    #[cfg(feature = "xinput")]
    XinputProximityOut(xinput::ProximityOutEvent),
    #[cfg(feature = "xinput")]
    XinputRawButtonPress(xinput::RawButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputRawButtonRelease(xinput::RawButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputRawKeyPress(xinput::RawKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputRawKeyRelease(xinput::RawKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputRawMotion(xinput::RawMotionEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchBegin(xinput::RawTouchBeginEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchEnd(xinput::RawTouchEndEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchUpdate(xinput::RawTouchUpdateEvent),
    #[cfg(feature = "xinput")]
    XinputTouchBegin(xinput::TouchBeginEvent),
    #[cfg(feature = "xinput")]
    XinputTouchEnd(xinput::TouchEndEvent),
    #[cfg(feature = "xinput")]
    XinputTouchOwnership(xinput::TouchOwnershipEvent),
    #[cfg(feature = "xinput")]
    XinputTouchUpdate(xinput::TouchUpdateEvent),
    #[cfg(feature = "xkb")]
    XkbAccessXNotify(xkb::AccessXNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbActionMessage(xkb::ActionMessageEvent),
    #[cfg(feature = "xkb")]
    XkbBellNotify(xkb::BellNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbCompatMapNotify(xkb::CompatMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbControlsNotify(xkb::ControlsNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbExtensionDeviceNotify(xkb::ExtensionDeviceNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorMapNotify(xkb::IndicatorMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorStateNotify(xkb::IndicatorStateNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbMapNotify(xkb::MapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNamesNotify(xkb::NamesNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNewKeyboardNotify(xkb::NewKeyboardNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbStateNotify(xkb::StateNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintAttributNotify(xprint::AttributNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintNotify(xprint::NotifyEvent),
    ButtonPress(xproto::ButtonPressEvent),
    ButtonRelease(xproto::ButtonReleaseEvent),
    CirculateNotify(xproto::CirculateNotifyEvent),
    CirculateRequest(xproto::CirculateRequestEvent),
    ClientMessage(xproto::ClientMessageEvent),
    ColormapNotify(xproto::ColormapNotifyEvent),
    ConfigureNotify(xproto::ConfigureNotifyEvent),
    ConfigureRequest(xproto::ConfigureRequestEvent),
    CreateNotify(xproto::CreateNotifyEvent),
    DestroyNotify(xproto::DestroyNotifyEvent),
    EnterNotify(xproto::EnterNotifyEvent),
    Expose(xproto::ExposeEvent),
    FocusIn(xproto::FocusInEvent),
    FocusOut(xproto::FocusOutEvent),
    GeGeneric(xproto::GeGenericEvent),
    GraphicsExposure(xproto::GraphicsExposureEvent),
    GravityNotify(xproto::GravityNotifyEvent),
    KeyPress(xproto::KeyPressEvent),
    KeyRelease(xproto::KeyReleaseEvent),
    KeymapNotify(xproto::KeymapNotifyEvent),
    LeaveNotify(xproto::LeaveNotifyEvent),
    MapNotify(xproto::MapNotifyEvent),
    MapRequest(xproto::MapRequestEvent),
    MappingNotify(xproto::MappingNotifyEvent),
    MotionNotify(xproto::MotionNotifyEvent),
    NoExposure(xproto::NoExposureEvent),
    PropertyNotify(xproto::PropertyNotifyEvent),
    ReparentNotify(xproto::ReparentNotifyEvent),
    ResizeRequest(xproto::ResizeRequestEvent),
    SelectionClear(xproto::SelectionClearEvent),
    SelectionNotify(xproto::SelectionNotifyEvent),
    SelectionRequest(xproto::SelectionRequestEvent),
    UnmapNotify(xproto::UnmapNotifyEvent),
    VisibilityNotify(xproto::VisibilityNotifyEvent),
    #[cfg(feature = "xv")]
    XvPortNotify(xv::PortNotifyEvent),
    #[cfg(feature = "xv")]
    XvVideoNotify(xv::VideoNotifyEvent),
}
impl<B: std::fmt::Debug + AsRef<[u8]>> Event<B> {
    /// Parse a generic X11 event into a concrete event type.
    #[allow(clippy::cognitive_complexity)]
    pub fn parse(
        event: GenericEvent<B>,
        iter: impl Iterator<Item=(&'static str, ExtensionInformation)>,
    ) -> Result<Self, ParseError> {
        let event_type = event.response_type();
        // Check if this is a core protocol error or from the generic event extension
        match event_type {
            0 => return Ok(Self::Error(Error::parse(event.try_into()?, iter)?)),
            xproto::BUTTON_PRESS_EVENT => return Ok(Self::ButtonPress(event.try_into()?)),
            xproto::BUTTON_RELEASE_EVENT => return Ok(Self::ButtonRelease(event.try_into()?)),
            xproto::CIRCULATE_NOTIFY_EVENT => return Ok(Self::CirculateNotify(event.try_into()?)),
            xproto::CIRCULATE_REQUEST_EVENT => return Ok(Self::CirculateRequest(event.try_into()?)),
            xproto::CLIENT_MESSAGE_EVENT => return Ok(Self::ClientMessage(event.try_into()?)),
            xproto::COLORMAP_NOTIFY_EVENT => return Ok(Self::ColormapNotify(event.try_into()?)),
            xproto::CONFIGURE_NOTIFY_EVENT => return Ok(Self::ConfigureNotify(event.try_into()?)),
            xproto::CONFIGURE_REQUEST_EVENT => return Ok(Self::ConfigureRequest(event.try_into()?)),
            xproto::CREATE_NOTIFY_EVENT => return Ok(Self::CreateNotify(event.try_into()?)),
            xproto::DESTROY_NOTIFY_EVENT => return Ok(Self::DestroyNotify(event.try_into()?)),
            xproto::ENTER_NOTIFY_EVENT => return Ok(Self::EnterNotify(event.try_into()?)),
            xproto::EXPOSE_EVENT => return Ok(Self::Expose(event.try_into()?)),
            xproto::FOCUS_IN_EVENT => return Ok(Self::FocusIn(event.try_into()?)),
            xproto::FOCUS_OUT_EVENT => return Ok(Self::FocusOut(event.try_into()?)),
            xproto::GRAPHICS_EXPOSURE_EVENT => return Ok(Self::GraphicsExposure(event.try_into()?)),
            xproto::GRAVITY_NOTIFY_EVENT => return Ok(Self::GravityNotify(event.try_into()?)),
            xproto::KEY_PRESS_EVENT => return Ok(Self::KeyPress(event.try_into()?)),
            xproto::KEY_RELEASE_EVENT => return Ok(Self::KeyRelease(event.try_into()?)),
            xproto::KEYMAP_NOTIFY_EVENT => return Ok(Self::KeymapNotify(event.try_into()?)),
            xproto::LEAVE_NOTIFY_EVENT => return Ok(Self::LeaveNotify(event.try_into()?)),
            xproto::MAP_NOTIFY_EVENT => return Ok(Self::MapNotify(event.try_into()?)),
            xproto::MAP_REQUEST_EVENT => return Ok(Self::MapRequest(event.try_into()?)),
            xproto::MAPPING_NOTIFY_EVENT => return Ok(Self::MappingNotify(event.try_into()?)),
            xproto::MOTION_NOTIFY_EVENT => return Ok(Self::MotionNotify(event.try_into()?)),
            xproto::NO_EXPOSURE_EVENT => return Ok(Self::NoExposure(event.try_into()?)),
            xproto::PROPERTY_NOTIFY_EVENT => return Ok(Self::PropertyNotify(event.try_into()?)),
            xproto::REPARENT_NOTIFY_EVENT => return Ok(Self::ReparentNotify(event.try_into()?)),
            xproto::RESIZE_REQUEST_EVENT => return Ok(Self::ResizeRequest(event.try_into()?)),
            xproto::SELECTION_CLEAR_EVENT => return Ok(Self::SelectionClear(event.try_into()?)),
            xproto::SELECTION_NOTIFY_EVENT => return Ok(Self::SelectionNotify(event.try_into()?)),
            xproto::SELECTION_REQUEST_EVENT => return Ok(Self::SelectionRequest(event.try_into()?)),
            xproto::UNMAP_NOTIFY_EVENT => return Ok(Self::UnmapNotify(event.try_into()?)),
            xproto::VISIBILITY_NOTIFY_EVENT => return Ok(Self::VisibilityNotify(event.try_into()?)),
            xproto::GE_GENERIC_EVENT => return Self::from_generic_event(event, iter),
            _ => {}
        }
        // Find the extension that this event could belong to
        let ext_info = iter
            .map(|(name, ext_info)| (name, ext_info.first_event))
            .filter(|&(_, first_event)| first_event <= event_type)
            .max_by_key(|&(_, first_event)| first_event);
        match ext_info {
            #[cfg(feature = "damage")]
            Some(("DAMAGE", first_event)) => {
                match event_type - first_event {
                    damage::NOTIFY_EVENT => Ok(Self::DamageNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "dri2")]
            Some(("DRI2", first_event)) => {
                match event_type - first_event {
                    dri2::BUFFER_SWAP_COMPLETE_EVENT => Ok(Self::Dri2BufferSwapComplete(event.try_into()?)),
                    dri2::INVALIDATE_BUFFERS_EVENT => Ok(Self::Dri2InvalidateBuffers(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "glx")]
            Some(("GLX", first_event)) => {
                match event_type - first_event {
                    glx::BUFFER_SWAP_COMPLETE_EVENT => Ok(Self::GlxBufferSwapComplete(event.try_into()?)),
                    glx::PBUFFER_CLOBBER_EVENT => Ok(Self::GlxPbufferClobber(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "present")]
            Some(("Present", first_event)) => {
                match event_type - first_event {
                    present::GENERIC_EVENT => Ok(Self::PresentGeneric(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "randr")]
            Some(("RANDR", first_event)) => {
                match event_type - first_event {
                    randr::NOTIFY_EVENT => Ok(Self::RandrNotify(event.try_into()?)),
                    randr::SCREEN_CHANGE_NOTIFY_EVENT => Ok(Self::RandrScreenChangeNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "screensaver")]
            Some(("MIT-SCREEN-SAVER", first_event)) => {
                match event_type - first_event {
                    screensaver::NOTIFY_EVENT => Ok(Self::ScreensaverNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "shape")]
            Some(("SHAPE", first_event)) => {
                match event_type - first_event {
                    shape::NOTIFY_EVENT => Ok(Self::ShapeNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "shm")]
            Some(("MIT-SHM", first_event)) => {
                match event_type - first_event {
                    shm::COMPLETION_EVENT => Ok(Self::ShmCompletion(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "sync")]
            Some(("SYNC", first_event)) => {
                match event_type - first_event {
                    sync::ALARM_NOTIFY_EVENT => Ok(Self::SyncAlarmNotify(event.try_into()?)),
                    sync::COUNTER_NOTIFY_EVENT => Ok(Self::SyncCounterNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xfixes")]
            Some(("XFIXES", first_event)) => {
                match event_type - first_event {
                    xfixes::CURSOR_NOTIFY_EVENT => Ok(Self::XfixesCursorNotify(event.try_into()?)),
                    xfixes::SELECTION_NOTIFY_EVENT => Ok(Self::XfixesSelectionNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xinput")]
            Some(("XInputExtension", first_event)) => {
                match event_type - first_event {
                    xinput::CHANGE_DEVICE_NOTIFY_EVENT => Ok(Self::XinputChangeDeviceNotify(event.try_into()?)),
                    xinput::DEVICE_BUTTON_PRESS_EVENT => Ok(Self::XinputDeviceButtonPress(event.try_into()?)),
                    xinput::DEVICE_BUTTON_RELEASE_EVENT => Ok(Self::XinputDeviceButtonRelease(event.try_into()?)),
                    xinput::DEVICE_BUTTON_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceButtonStateNotify(event.try_into()?)),
                    xinput::DEVICE_FOCUS_IN_EVENT => Ok(Self::XinputDeviceFocusIn(event.try_into()?)),
                    xinput::DEVICE_FOCUS_OUT_EVENT => Ok(Self::XinputDeviceFocusOut(event.try_into()?)),
                    xinput::DEVICE_KEY_PRESS_EVENT => Ok(Self::XinputDeviceKeyPress(event.try_into()?)),
                    xinput::DEVICE_KEY_RELEASE_EVENT => Ok(Self::XinputDeviceKeyRelease(event.try_into()?)),
                    xinput::DEVICE_KEY_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceKeyStateNotify(event.try_into()?)),
                    xinput::DEVICE_MAPPING_NOTIFY_EVENT => Ok(Self::XinputDeviceMappingNotify(event.try_into()?)),
                    xinput::DEVICE_MOTION_NOTIFY_EVENT => Ok(Self::XinputDeviceMotionNotify(event.try_into()?)),
                    xinput::DEVICE_PRESENCE_NOTIFY_EVENT => Ok(Self::XinputDevicePresenceNotify(event.try_into()?)),
                    xinput::DEVICE_PROPERTY_NOTIFY_EVENT => Ok(Self::XinputDevicePropertyNotify(event.try_into()?)),
                    xinput::DEVICE_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceStateNotify(event.try_into()?)),
                    xinput::DEVICE_VALUATOR_EVENT => Ok(Self::XinputDeviceValuator(event.try_into()?)),
                    xinput::PROXIMITY_IN_EVENT => Ok(Self::XinputProximityIn(event.try_into()?)),
                    xinput::PROXIMITY_OUT_EVENT => Ok(Self::XinputProximityOut(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xkb")]
            Some(("XKEYBOARD", first_event)) => {
                if event_type != first_event {
                    return Ok(Self::Unknown(event));
                }
                match event.raw_bytes()[1] {
                    xkb::ACCESS_X_NOTIFY_EVENT => Ok(Self::XkbAccessXNotify(event.try_into()?)),
                    xkb::ACTION_MESSAGE_EVENT => Ok(Self::XkbActionMessage(event.try_into()?)),
                    xkb::BELL_NOTIFY_EVENT => Ok(Self::XkbBellNotify(event.try_into()?)),
                    xkb::COMPAT_MAP_NOTIFY_EVENT => Ok(Self::XkbCompatMapNotify(event.try_into()?)),
                    xkb::CONTROLS_NOTIFY_EVENT => Ok(Self::XkbControlsNotify(event.try_into()?)),
                    xkb::EXTENSION_DEVICE_NOTIFY_EVENT => Ok(Self::XkbExtensionDeviceNotify(event.try_into()?)),
                    xkb::INDICATOR_MAP_NOTIFY_EVENT => Ok(Self::XkbIndicatorMapNotify(event.try_into()?)),
                    xkb::INDICATOR_STATE_NOTIFY_EVENT => Ok(Self::XkbIndicatorStateNotify(event.try_into()?)),
                    xkb::MAP_NOTIFY_EVENT => Ok(Self::XkbMapNotify(event.try_into()?)),
                    xkb::NAMES_NOTIFY_EVENT => Ok(Self::XkbNamesNotify(event.try_into()?)),
                    xkb::NEW_KEYBOARD_NOTIFY_EVENT => Ok(Self::XkbNewKeyboardNotify(event.try_into()?)),
                    xkb::STATE_NOTIFY_EVENT => Ok(Self::XkbStateNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xprint")]
            Some(("XpExtension", first_event)) => {
                match event_type - first_event {
                    xprint::ATTRIBUT_NOTIFY_EVENT => Ok(Self::XprintAttributNotify(event.try_into()?)),
                    xprint::NOTIFY_EVENT => Ok(Self::XprintNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xv")]
            Some(("XVideo", first_event)) => {
                match event_type - first_event {
                    xv::PORT_NOTIFY_EVENT => Ok(Self::XvPortNotify(event.try_into()?)),
                    xv::VIDEO_NOTIFY_EVENT => Ok(Self::XvVideoNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            _ => Ok(Self::Unknown(event))
        }
    }

    fn from_generic_event(
        event: GenericEvent<B>,
        iter: impl Iterator<Item=(&'static str, ExtensionInformation)>,
    ) -> Result<Self, ParseError> {
        let bytes = event.raw_bytes();
        let ge_event = xproto::GeGenericEvent::try_from(bytes)?;
        #[allow(unused_variables)]
        let (extension, event_type) = (ge_event.extension, ge_event.event_type);
        let ext_name = iter
            .map(|(name, ext_info)| (name, ext_info.major_opcode))
            .find(|&(_, opcode)| extension == opcode)
            .map(|(name, _)| name);
        match ext_name {
            #[cfg(feature = "present")]
            Some("Present") => {
                match event_type {
                    present::COMPLETE_NOTIFY_EVENT => Ok(Self::PresentCompleteNotify(event.try_into()?)),
                    present::CONFIGURE_NOTIFY_EVENT => Ok(Self::PresentConfigureNotify(event.try_into()?)),
                    present::IDLE_NOTIFY_EVENT => Ok(Self::PresentIdleNotify(event.try_into()?)),
                    present::REDIRECT_NOTIFY_EVENT => Ok(Self::PresentRedirectNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xinput")]
            Some("XInputExtension") => {
                match event_type {
                    xinput::BARRIER_HIT_EVENT => Ok(Self::XinputBarrierHit(event.try_into()?)),
                    xinput::BARRIER_LEAVE_EVENT => Ok(Self::XinputBarrierLeave(event.try_into()?)),
                    xinput::BUTTON_PRESS_EVENT => Ok(Self::XinputButtonPress(event.try_into()?)),
                    xinput::BUTTON_RELEASE_EVENT => Ok(Self::XinputButtonRelease(event.try_into()?)),
                    xinput::DEVICE_CHANGED_EVENT => Ok(Self::XinputDeviceChanged(event.try_into()?)),
                    xinput::ENTER_EVENT => Ok(Self::XinputEnter(event.try_into()?)),
                    xinput::FOCUS_IN_EVENT => Ok(Self::XinputFocusIn(event.try_into()?)),
                    xinput::FOCUS_OUT_EVENT => Ok(Self::XinputFocusOut(event.try_into()?)),
                    xinput::HIERARCHY_EVENT => Ok(Self::XinputHierarchy(event.try_into()?)),
                    xinput::KEY_PRESS_EVENT => Ok(Self::XinputKeyPress(event.try_into()?)),
                    xinput::KEY_RELEASE_EVENT => Ok(Self::XinputKeyRelease(event.try_into()?)),
                    xinput::LEAVE_EVENT => Ok(Self::XinputLeave(event.try_into()?)),
                    xinput::MOTION_EVENT => Ok(Self::XinputMotion(event.try_into()?)),
                    xinput::PROPERTY_EVENT => Ok(Self::XinputProperty(event.try_into()?)),
                    xinput::RAW_BUTTON_PRESS_EVENT => Ok(Self::XinputRawButtonPress(event.try_into()?)),
                    xinput::RAW_BUTTON_RELEASE_EVENT => Ok(Self::XinputRawButtonRelease(event.try_into()?)),
                    xinput::RAW_KEY_PRESS_EVENT => Ok(Self::XinputRawKeyPress(event.try_into()?)),
                    xinput::RAW_KEY_RELEASE_EVENT => Ok(Self::XinputRawKeyRelease(event.try_into()?)),
                    xinput::RAW_MOTION_EVENT => Ok(Self::XinputRawMotion(event.try_into()?)),
                    xinput::RAW_TOUCH_BEGIN_EVENT => Ok(Self::XinputRawTouchBegin(event.try_into()?)),
                    xinput::RAW_TOUCH_END_EVENT => Ok(Self::XinputRawTouchEnd(event.try_into()?)),
                    xinput::RAW_TOUCH_UPDATE_EVENT => Ok(Self::XinputRawTouchUpdate(event.try_into()?)),
                    xinput::TOUCH_BEGIN_EVENT => Ok(Self::XinputTouchBegin(event.try_into()?)),
                    xinput::TOUCH_END_EVENT => Ok(Self::XinputTouchEnd(event.try_into()?)),
                    xinput::TOUCH_OWNERSHIP_EVENT => Ok(Self::XinputTouchOwnership(event.try_into()?)),
                    xinput::TOUCH_UPDATE_EVENT => Ok(Self::XinputTouchUpdate(event.try_into()?)),
                    _ => Ok(Self::Unknown(event))
                }
            }
            _ => Ok(Self::Unknown(event))
        }
    }
}
