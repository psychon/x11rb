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
        iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,
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
            .map(|(name, reply)| (name, reply.first_error))
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
    pub fn parse(
        event: GenericEvent<B>,
        iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,
    ) -> Result<Self, ParseError> {
        let event_type = event.response_type();
        // Check if this is a core protocol error or from the generic event extension
        match event_type {
            0 => return Ok(Self::Error(Error::parse(event.try_into()?, iter)?)),
            xproto::BUTTON_PRESS_EVENT => return Ok(Self::ButtonPress(event.into())),
            xproto::BUTTON_RELEASE_EVENT => return Ok(Self::ButtonRelease(event.into())),
            xproto::CIRCULATE_NOTIFY_EVENT => return Ok(Self::CirculateNotify(event.into())),
            xproto::CIRCULATE_REQUEST_EVENT => return Ok(Self::CirculateRequest(event.into())),
            xproto::CLIENT_MESSAGE_EVENT => return Ok(Self::ClientMessage(event.into())),
            xproto::COLORMAP_NOTIFY_EVENT => return Ok(Self::ColormapNotify(event.into())),
            xproto::CONFIGURE_NOTIFY_EVENT => return Ok(Self::ConfigureNotify(event.into())),
            xproto::CONFIGURE_REQUEST_EVENT => return Ok(Self::ConfigureRequest(event.into())),
            xproto::CREATE_NOTIFY_EVENT => return Ok(Self::CreateNotify(event.into())),
            xproto::DESTROY_NOTIFY_EVENT => return Ok(Self::DestroyNotify(event.into())),
            xproto::ENTER_NOTIFY_EVENT => return Ok(Self::EnterNotify(event.into())),
            xproto::EXPOSE_EVENT => return Ok(Self::Expose(event.into())),
            xproto::FOCUS_IN_EVENT => return Ok(Self::FocusIn(event.into())),
            xproto::FOCUS_OUT_EVENT => return Ok(Self::FocusOut(event.into())),
            xproto::GRAPHICS_EXPOSURE_EVENT => return Ok(Self::GraphicsExposure(event.into())),
            xproto::GRAVITY_NOTIFY_EVENT => return Ok(Self::GravityNotify(event.into())),
            xproto::KEY_PRESS_EVENT => return Ok(Self::KeyPress(event.into())),
            xproto::KEY_RELEASE_EVENT => return Ok(Self::KeyRelease(event.into())),
            xproto::KEYMAP_NOTIFY_EVENT => return Ok(Self::KeymapNotify(event.into())),
            xproto::LEAVE_NOTIFY_EVENT => return Ok(Self::LeaveNotify(event.into())),
            xproto::MAP_NOTIFY_EVENT => return Ok(Self::MapNotify(event.into())),
            xproto::MAP_REQUEST_EVENT => return Ok(Self::MapRequest(event.into())),
            xproto::MAPPING_NOTIFY_EVENT => return Ok(Self::MappingNotify(event.into())),
            xproto::MOTION_NOTIFY_EVENT => return Ok(Self::MotionNotify(event.into())),
            xproto::NO_EXPOSURE_EVENT => return Ok(Self::NoExposure(event.into())),
            xproto::PROPERTY_NOTIFY_EVENT => return Ok(Self::PropertyNotify(event.into())),
            xproto::REPARENT_NOTIFY_EVENT => return Ok(Self::ReparentNotify(event.into())),
            xproto::RESIZE_REQUEST_EVENT => return Ok(Self::ResizeRequest(event.into())),
            xproto::SELECTION_CLEAR_EVENT => return Ok(Self::SelectionClear(event.into())),
            xproto::SELECTION_NOTIFY_EVENT => return Ok(Self::SelectionNotify(event.into())),
            xproto::SELECTION_REQUEST_EVENT => return Ok(Self::SelectionRequest(event.into())),
            xproto::UNMAP_NOTIFY_EVENT => return Ok(Self::UnmapNotify(event.into())),
            xproto::VISIBILITY_NOTIFY_EVENT => return Ok(Self::VisibilityNotify(event.into())),
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
                    damage::NOTIFY_EVENT => Ok(Self::DamageNotify(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "dri2")]
            Some(("DRI2", first_event)) => {
                match event_type - first_event {
                    dri2::BUFFER_SWAP_COMPLETE_EVENT => Ok(Self::Dri2BufferSwapComplete(event.into())),
                    dri2::INVALIDATE_BUFFERS_EVENT => Ok(Self::Dri2InvalidateBuffers(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "glx")]
            Some(("GLX", first_event)) => {
                match event_type - first_event {
                    glx::BUFFER_SWAP_COMPLETE_EVENT => Ok(Self::GlxBufferSwapComplete(event.into())),
                    glx::PBUFFER_CLOBBER_EVENT => Ok(Self::GlxPbufferClobber(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "present")]
            Some(("Present", first_event)) => {
                match event_type - first_event {
                    present::GENERIC_EVENT => Ok(Self::PresentGeneric(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "randr")]
            Some(("RANDR", first_event)) => {
                match event_type - first_event {
                    randr::NOTIFY_EVENT => Ok(Self::RandrNotify(event.into())),
                    randr::SCREEN_CHANGE_NOTIFY_EVENT => Ok(Self::RandrScreenChangeNotify(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "screensaver")]
            Some(("MIT-SCREEN-SAVER", first_event)) => {
                match event_type - first_event {
                    screensaver::NOTIFY_EVENT => Ok(Self::ScreensaverNotify(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "shape")]
            Some(("SHAPE", first_event)) => {
                match event_type - first_event {
                    shape::NOTIFY_EVENT => Ok(Self::ShapeNotify(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "shm")]
            Some(("MIT-SHM", first_event)) => {
                match event_type - first_event {
                    shm::COMPLETION_EVENT => Ok(Self::ShmCompletion(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "sync")]
            Some(("SYNC", first_event)) => {
                match event_type - first_event {
                    sync::ALARM_NOTIFY_EVENT => Ok(Self::SyncAlarmNotify(event.into())),
                    sync::COUNTER_NOTIFY_EVENT => Ok(Self::SyncCounterNotify(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xfixes")]
            Some(("XFIXES", first_event)) => {
                match event_type - first_event {
                    xfixes::CURSOR_NOTIFY_EVENT => Ok(Self::XfixesCursorNotify(event.into())),
                    xfixes::SELECTION_NOTIFY_EVENT => Ok(Self::XfixesSelectionNotify(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xinput")]
            Some(("XInputExtension", first_event)) => {
                match event_type - first_event {
                    xinput::CHANGE_DEVICE_NOTIFY_EVENT => Ok(Self::XinputChangeDeviceNotify(event.into())),
                    xinput::DEVICE_BUTTON_PRESS_EVENT => Ok(Self::XinputDeviceButtonPress(event.into())),
                    xinput::DEVICE_BUTTON_RELEASE_EVENT => Ok(Self::XinputDeviceButtonRelease(event.into())),
                    xinput::DEVICE_BUTTON_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceButtonStateNotify(event.into())),
                    xinput::DEVICE_FOCUS_IN_EVENT => Ok(Self::XinputDeviceFocusIn(event.into())),
                    xinput::DEVICE_FOCUS_OUT_EVENT => Ok(Self::XinputDeviceFocusOut(event.into())),
                    xinput::DEVICE_KEY_PRESS_EVENT => Ok(Self::XinputDeviceKeyPress(event.into())),
                    xinput::DEVICE_KEY_RELEASE_EVENT => Ok(Self::XinputDeviceKeyRelease(event.into())),
                    xinput::DEVICE_KEY_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceKeyStateNotify(event.into())),
                    xinput::DEVICE_MAPPING_NOTIFY_EVENT => Ok(Self::XinputDeviceMappingNotify(event.into())),
                    xinput::DEVICE_MOTION_NOTIFY_EVENT => Ok(Self::XinputDeviceMotionNotify(event.into())),
                    xinput::DEVICE_PRESENCE_NOTIFY_EVENT => Ok(Self::XinputDevicePresenceNotify(event.into())),
                    xinput::DEVICE_PROPERTY_NOTIFY_EVENT => Ok(Self::XinputDevicePropertyNotify(event.into())),
                    xinput::DEVICE_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceStateNotify(event.into())),
                    xinput::DEVICE_VALUATOR_EVENT => Ok(Self::XinputDeviceValuator(event.into())),
                    xinput::PROXIMITY_IN_EVENT => Ok(Self::XinputProximityIn(event.into())),
                    xinput::PROXIMITY_OUT_EVENT => Ok(Self::XinputProximityOut(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xkb")]
            Some(("XKEYBOARD", first_event)) => {
                if event_type != first_event {
                    return Ok(Self::Unknown(event));
                }
                match event.raw_bytes()[1] {
                    xkb::ACCESS_X_NOTIFY_EVENT => Ok(Self::XkbAccessXNotify(event.into())),
                    xkb::ACTION_MESSAGE_EVENT => Ok(Self::XkbActionMessage(event.into())),
                    xkb::BELL_NOTIFY_EVENT => Ok(Self::XkbBellNotify(event.into())),
                    xkb::COMPAT_MAP_NOTIFY_EVENT => Ok(Self::XkbCompatMapNotify(event.into())),
                    xkb::CONTROLS_NOTIFY_EVENT => Ok(Self::XkbControlsNotify(event.into())),
                    xkb::EXTENSION_DEVICE_NOTIFY_EVENT => Ok(Self::XkbExtensionDeviceNotify(event.into())),
                    xkb::INDICATOR_MAP_NOTIFY_EVENT => Ok(Self::XkbIndicatorMapNotify(event.into())),
                    xkb::INDICATOR_STATE_NOTIFY_EVENT => Ok(Self::XkbIndicatorStateNotify(event.into())),
                    xkb::MAP_NOTIFY_EVENT => Ok(Self::XkbMapNotify(event.into())),
                    xkb::NAMES_NOTIFY_EVENT => Ok(Self::XkbNamesNotify(event.into())),
                    xkb::NEW_KEYBOARD_NOTIFY_EVENT => Ok(Self::XkbNewKeyboardNotify(event.into())),
                    xkb::STATE_NOTIFY_EVENT => Ok(Self::XkbStateNotify(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xprint")]
            Some(("XpExtension", first_event)) => {
                match event_type - first_event {
                    xprint::ATTRIBUT_NOTIFY_EVENT => Ok(Self::XprintAttributNotify(event.into())),
                    xprint::NOTIFY_EVENT => Ok(Self::XprintNotify(event.into())),
                    _ => Ok(Self::Unknown(event))
                }
            }
            #[cfg(feature = "xv")]
            Some(("XVideo", first_event)) => {
                match event_type - first_event {
                    xv::PORT_NOTIFY_EVENT => Ok(Self::XvPortNotify(event.into())),
                    xv::VIDEO_NOTIFY_EVENT => Ok(Self::XvVideoNotify(event.into())),
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
        #[allow(unused_variables)]
        let (extension, event_type) = (ge_event.extension, ge_event.event_type);
        let ext_name = iter
            .map(|(name, reply)| (name, reply.major_opcode))
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

/// Get the name of a request based on its opcodes.
pub fn request_name(
    major_opcode: u8,
    minor_opcode: u8,
    mut iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,
) -> &'static str {
    // Core protocol
    match major_opcode {
        84 => return "AllocColor",
        86 => return "AllocColorCells",
        87 => return "AllocColorPlanes",
        85 => return "AllocNamedColor",
        35 => return "AllowEvents",
        104 => return "Bell",
        30 => return "ChangeActivePointerGrab",
        56 => return "ChangeGC",
        109 => return "ChangeHosts",
        102 => return "ChangeKeyboardControl",
        100 => return "ChangeKeyboardMapping",
        105 => return "ChangePointerControl",
        18 => return "ChangeProperty",
        6 => return "ChangeSaveSet",
        2 => return "ChangeWindowAttributes",
        13 => return "CirculateWindow",
        61 => return "ClearArea",
        46 => return "CloseFont",
        12 => return "ConfigureWindow",
        24 => return "ConvertSelection",
        62 => return "CopyArea",
        80 => return "CopyColormapAndFree",
        57 => return "CopyGC",
        63 => return "CopyPlane",
        78 => return "CreateColormap",
        93 => return "CreateCursor",
        55 => return "CreateGC",
        94 => return "CreateGlyphCursor",
        53 => return "CreatePixmap",
        1 => return "CreateWindow",
        19 => return "DeleteProperty",
        5 => return "DestroySubwindows",
        4 => return "DestroyWindow",
        69 => return "FillPoly",
        115 => return "ForceScreenSaver",
        79 => return "FreeColormap",
        88 => return "FreeColors",
        95 => return "FreeCursor",
        60 => return "FreeGC",
        54 => return "FreePixmap",
        17 => return "GetAtomName",
        52 => return "GetFontPath",
        14 => return "GetGeometry",
        73 => return "GetImage",
        43 => return "GetInputFocus",
        103 => return "GetKeyboardControl",
        101 => return "GetKeyboardMapping",
        119 => return "GetModifierMapping",
        39 => return "GetMotionEvents",
        106 => return "GetPointerControl",
        117 => return "GetPointerMapping",
        20 => return "GetProperty",
        108 => return "GetScreenSaver",
        23 => return "GetSelectionOwner",
        3 => return "GetWindowAttributes",
        28 => return "GrabButton",
        33 => return "GrabKey",
        31 => return "GrabKeyboard",
        26 => return "GrabPointer",
        36 => return "GrabServer",
        77 => return "ImageText16",
        76 => return "ImageText8",
        81 => return "InstallColormap",
        16 => return "InternAtom",
        113 => return "KillClient",
        99 => return "ListExtensions",
        49 => return "ListFonts",
        50 => return "ListFontsWithInfo",
        110 => return "ListHosts",
        83 => return "ListInstalledColormaps",
        21 => return "ListProperties",
        92 => return "LookupColor",
        9 => return "MapSubwindows",
        8 => return "MapWindow",
        127 => return "NoOperation",
        45 => return "OpenFont",
        68 => return "PolyArc",
        71 => return "PolyFillArc",
        70 => return "PolyFillRectangle",
        65 => return "PolyLine",
        64 => return "PolyPoint",
        67 => return "PolyRectangle",
        66 => return "PolySegment",
        75 => return "PolyText16",
        74 => return "PolyText8",
        72 => return "PutImage",
        97 => return "QueryBestSize",
        91 => return "QueryColors",
        98 => return "QueryExtension",
        47 => return "QueryFont",
        44 => return "QueryKeymap",
        38 => return "QueryPointer",
        48 => return "QueryTextExtents",
        15 => return "QueryTree",
        96 => return "RecolorCursor",
        7 => return "ReparentWindow",
        114 => return "RotateProperties",
        25 => return "SendEvent",
        111 => return "SetAccessControl",
        59 => return "SetClipRectangles",
        112 => return "SetCloseDownMode",
        58 => return "SetDashes",
        51 => return "SetFontPath",
        42 => return "SetInputFocus",
        118 => return "SetModifierMapping",
        116 => return "SetPointerMapping",
        107 => return "SetScreenSaver",
        22 => return "SetSelectionOwner",
        89 => return "StoreColors",
        90 => return "StoreNamedColor",
        40 => return "TranslateCoordinates",
        29 => return "UngrabButton",
        34 => return "UngrabKey",
        32 => return "UngrabKeyboard",
        27 => return "UngrabPointer",
        37 => return "UngrabServer",
        82 => return "UninstallColormap",
        11 => return "UnmapSubwindows",
        10 => return "UnmapWindow",
        41 => return "WarpPointer",
        _ => {}
    }
    // Find the extension by its opcode
    let ext_name = iter
        .find(|&(_, reply)| reply.major_opcode == major_opcode)
        .map(|(name, _)| name);
    match ext_name {
        Some("BIG-REQUESTS") => match minor_opcode {
            0 => "BigRequests::Enable",
            _ => "BigRequests::unknown request"
        }
        #[cfg(feature = "composite")]
        Some("Composite") => match minor_opcode {
            5 => "Composite::CreateRegionFromBorderClip",
            7 => "Composite::GetOverlayWindow",
            6 => "Composite::NameWindowPixmap",
            0 => "Composite::QueryVersion",
            2 => "Composite::RedirectSubwindows",
            1 => "Composite::RedirectWindow",
            8 => "Composite::ReleaseOverlayWindow",
            4 => "Composite::UnredirectSubwindows",
            3 => "Composite::UnredirectWindow",
            _ => "Composite::unknown request"
        }
        #[cfg(feature = "damage")]
        Some("DAMAGE") => match minor_opcode {
            4 => "Damage::Add",
            1 => "Damage::Create",
            2 => "Damage::Destroy",
            0 => "Damage::QueryVersion",
            3 => "Damage::Subtract",
            _ => "Damage::unknown request"
        }
        #[cfg(feature = "dpms")]
        Some("DPMS") => match minor_opcode {
            1 => "DPMS::Capable",
            5 => "DPMS::Disable",
            4 => "DPMS::Enable",
            6 => "DPMS::ForceLevel",
            2 => "DPMS::GetTimeouts",
            0 => "DPMS::GetVersion",
            7 => "DPMS::Info",
            3 => "DPMS::SetTimeouts",
            _ => "DPMS::unknown request"
        }
        #[cfg(feature = "dri2")]
        Some("DRI2") => match minor_opcode {
            2 => "DRI2::Authenticate",
            1 => "DRI2::Connect",
            6 => "DRI2::CopyRegion",
            3 => "DRI2::CreateDrawable",
            4 => "DRI2::DestroyDrawable",
            5 => "DRI2::GetBuffers",
            7 => "DRI2::GetBuffersWithFormat",
            9 => "DRI2::GetMSC",
            13 => "DRI2::GetParam",
            0 => "DRI2::QueryVersion",
            8 => "DRI2::SwapBuffers",
            12 => "DRI2::SwapInterval",
            10 => "DRI2::WaitMSC",
            11 => "DRI2::WaitSBC",
            _ => "DRI2::unknown request"
        }
        #[cfg(feature = "dri3")]
        Some("DRI3") => match minor_opcode {
            3 => "DRI3::BufferFromPixmap",
            8 => "DRI3::BuffersFromPixmap",
            5 => "DRI3::FDFromFence",
            4 => "DRI3::FenceFromFD",
            6 => "DRI3::GetSupportedModifiers",
            1 => "DRI3::Open",
            2 => "DRI3::PixmapFromBuffer",
            7 => "DRI3::PixmapFromBuffers",
            0 => "DRI3::QueryVersion",
            _ => "DRI3::unknown request"
        }
        Some("Generic Event Extension") => match minor_opcode {
            0 => "GenericEvent::QueryVersion",
            _ => "GenericEvent::unknown request"
        }
        #[cfg(feature = "glx")]
        Some("GLX") => match minor_opcode {
            143 => "Glx::AreTexturesResident",
            30 => "Glx::ChangeDrawableAttributes",
            20 => "Glx::ClientInfo",
            10 => "Glx::CopyContext",
            3 => "Glx::CreateContext",
            34 => "Glx::CreateContextAttribsARB",
            13 => "Glx::CreateGLXPixmap",
            24 => "Glx::CreateNewContext",
            27 => "Glx::CreatePbuffer",
            22 => "Glx::CreatePixmap",
            31 => "Glx::CreateWindow",
            103 => "Glx::DeleteLists",
            161 => "Glx::DeleteQueriesARB",
            144 => "Glx::DeleteTextures",
            32 => "Glx::DeleteWindow",
            4 => "Glx::DestroyContext",
            15 => "Glx::DestroyGLXPixmap",
            28 => "Glx::DestroyPbuffer",
            23 => "Glx::DestroyPixmap",
            102 => "Glx::EndList",
            105 => "Glx::FeedbackBuffer",
            108 => "Glx::Finish",
            142 => "Glx::Flush",
            104 => "Glx::GenLists",
            162 => "Glx::GenQueriesARB",
            145 => "Glx::GenTextures",
            112 => "Glx::GetBooleanv",
            113 => "Glx::GetClipPlane",
            147 => "Glx::GetColorTable",
            148 => "Glx::GetColorTableParameterfv",
            149 => "Glx::GetColorTableParameteriv",
            160 => "Glx::GetCompressedTexImageARB",
            150 => "Glx::GetConvolutionFilter",
            151 => "Glx::GetConvolutionParameterfv",
            152 => "Glx::GetConvolutionParameteriv",
            114 => "Glx::GetDoublev",
            29 => "Glx::GetDrawableAttributes",
            115 => "Glx::GetError",
            21 => "Glx::GetFBConfigs",
            116 => "Glx::GetFloatv",
            154 => "Glx::GetHistogram",
            155 => "Glx::GetHistogramParameterfv",
            156 => "Glx::GetHistogramParameteriv",
            117 => "Glx::GetIntegerv",
            118 => "Glx::GetLightfv",
            119 => "Glx::GetLightiv",
            120 => "Glx::GetMapdv",
            121 => "Glx::GetMapfv",
            122 => "Glx::GetMapiv",
            123 => "Glx::GetMaterialfv",
            124 => "Glx::GetMaterialiv",
            157 => "Glx::GetMinmax",
            158 => "Glx::GetMinmaxParameterfv",
            159 => "Glx::GetMinmaxParameteriv",
            125 => "Glx::GetPixelMapfv",
            126 => "Glx::GetPixelMapuiv",
            127 => "Glx::GetPixelMapusv",
            128 => "Glx::GetPolygonStipple",
            165 => "Glx::GetQueryObjectivARB",
            166 => "Glx::GetQueryObjectuivARB",
            164 => "Glx::GetQueryivARB",
            153 => "Glx::GetSeparableFilter",
            129 => "Glx::GetString",
            130 => "Glx::GetTexEnvfv",
            131 => "Glx::GetTexEnviv",
            132 => "Glx::GetTexGendv",
            133 => "Glx::GetTexGenfv",
            134 => "Glx::GetTexGeniv",
            135 => "Glx::GetTexImage",
            138 => "Glx::GetTexLevelParameterfv",
            139 => "Glx::GetTexLevelParameteriv",
            136 => "Glx::GetTexParameterfv",
            137 => "Glx::GetTexParameteriv",
            14 => "Glx::GetVisualConfigs",
            6 => "Glx::IsDirect",
            140 => "Glx::IsEnabled",
            141 => "Glx::IsList",
            163 => "Glx::IsQueryARB",
            146 => "Glx::IsTexture",
            26 => "Glx::MakeContextCurrent",
            5 => "Glx::MakeCurrent",
            101 => "Glx::NewList",
            109 => "Glx::PixelStoref",
            110 => "Glx::PixelStorei",
            25 => "Glx::QueryContext",
            18 => "Glx::QueryExtensionsString",
            19 => "Glx::QueryServerString",
            7 => "Glx::QueryVersion",
            111 => "Glx::ReadPixels",
            1 => "Glx::Render",
            2 => "Glx::RenderLarge",
            107 => "Glx::RenderMode",
            106 => "Glx::SelectBuffer",
            35 => "Glx::SetClientInfo2ARB",
            33 => "Glx::SetClientInfoARB",
            11 => "Glx::SwapBuffers",
            12 => "Glx::UseXFont",
            16 => "Glx::VendorPrivate",
            17 => "Glx::VendorPrivateWithReply",
            8 => "Glx::WaitGL",
            9 => "Glx::WaitX",
            _ => "Glx::unknown request"
        }
        #[cfg(feature = "present")]
        Some("Present") => match minor_opcode {
            2 => "Present::NotifyMSC",
            1 => "Present::Pixmap",
            4 => "Present::QueryCapabilities",
            0 => "Present::QueryVersion",
            3 => "Present::SelectInput",
            _ => "Present::unknown request"
        }
        #[cfg(feature = "randr")]
        Some("RANDR") => match minor_opcode {
            18 => "RandR::AddOutputMode",
            13 => "RandR::ChangeOutputProperty",
            39 => "RandR::ChangeProviderProperty",
            12 => "RandR::ConfigureOutputProperty",
            38 => "RandR::ConfigureProviderProperty",
            45 => "RandR::CreateLease",
            16 => "RandR::CreateMode",
            44 => "RandR::DeleteMonitor",
            19 => "RandR::DeleteOutputMode",
            14 => "RandR::DeleteOutputProperty",
            40 => "RandR::DeleteProviderProperty",
            17 => "RandR::DestroyMode",
            46 => "RandR::FreeLease",
            23 => "RandR::GetCrtcGamma",
            22 => "RandR::GetCrtcGammaSize",
            20 => "RandR::GetCrtcInfo",
            27 => "RandR::GetCrtcTransform",
            42 => "RandR::GetMonitors",
            9 => "RandR::GetOutputInfo",
            31 => "RandR::GetOutputPrimary",
            15 => "RandR::GetOutputProperty",
            28 => "RandR::GetPanning",
            33 => "RandR::GetProviderInfo",
            41 => "RandR::GetProviderProperty",
            32 => "RandR::GetProviders",
            5 => "RandR::GetScreenInfo",
            8 => "RandR::GetScreenResources",
            25 => "RandR::GetScreenResourcesCurrent",
            6 => "RandR::GetScreenSizeRange",
            10 => "RandR::ListOutputProperties",
            36 => "RandR::ListProviderProperties",
            11 => "RandR::QueryOutputProperty",
            37 => "RandR::QueryProviderProperty",
            0 => "RandR::QueryVersion",
            4 => "RandR::SelectInput",
            21 => "RandR::SetCrtcConfig",
            24 => "RandR::SetCrtcGamma",
            26 => "RandR::SetCrtcTransform",
            43 => "RandR::SetMonitor",
            30 => "RandR::SetOutputPrimary",
            29 => "RandR::SetPanning",
            34 => "RandR::SetProviderOffloadSink",
            35 => "RandR::SetProviderOutputSource",
            2 => "RandR::SetScreenConfig",
            7 => "RandR::SetScreenSize",
            _ => "RandR::unknown request"
        }
        #[cfg(feature = "record")]
        Some("RECORD") => match minor_opcode {
            1 => "Record::CreateContext",
            6 => "Record::DisableContext",
            5 => "Record::EnableContext",
            7 => "Record::FreeContext",
            4 => "Record::GetContext",
            0 => "Record::QueryVersion",
            2 => "Record::RegisterClients",
            3 => "Record::UnregisterClients",
            _ => "Record::unknown request"
        }
        #[cfg(feature = "render")]
        Some("RENDER") => match minor_opcode {
            20 => "Render::AddGlyphs",
            32 => "Render::AddTraps",
            5 => "Render::ChangePicture",
            8 => "Render::Composite",
            24 => "Render::CompositeGlyphs16",
            25 => "Render::CompositeGlyphs32",
            23 => "Render::CompositeGlyphs8",
            31 => "Render::CreateAnimCursor",
            36 => "Render::CreateConicalGradient",
            27 => "Render::CreateCursor",
            17 => "Render::CreateGlyphSet",
            34 => "Render::CreateLinearGradient",
            4 => "Render::CreatePicture",
            35 => "Render::CreateRadialGradient",
            33 => "Render::CreateSolidFill",
            26 => "Render::FillRectangles",
            19 => "Render::FreeGlyphSet",
            22 => "Render::FreeGlyphs",
            7 => "Render::FreePicture",
            29 => "Render::QueryFilters",
            1 => "Render::QueryPictFormats",
            2 => "Render::QueryPictIndexValues",
            0 => "Render::QueryVersion",
            18 => "Render::ReferenceGlyphSet",
            6 => "Render::SetPictureClipRectangles",
            30 => "Render::SetPictureFilter",
            28 => "Render::SetPictureTransform",
            10 => "Render::Trapezoids",
            13 => "Render::TriFan",
            12 => "Render::TriStrip",
            11 => "Render::Triangles",
            _ => "Render::unknown request"
        }
        #[cfg(feature = "res")]
        Some("X-Resource") => match minor_opcode {
            4 => "Res::QueryClientIds",
            3 => "Res::QueryClientPixmapBytes",
            2 => "Res::QueryClientResources",
            1 => "Res::QueryClients",
            5 => "Res::QueryResourceBytes",
            0 => "Res::QueryVersion",
            _ => "Res::unknown request"
        }
        #[cfg(feature = "screensaver")]
        Some("MIT-SCREEN-SAVER") => match minor_opcode {
            1 => "ScreenSaver::QueryInfo",
            0 => "ScreenSaver::QueryVersion",
            2 => "ScreenSaver::SelectInput",
            3 => "ScreenSaver::SetAttributes",
            5 => "ScreenSaver::Suspend",
            4 => "ScreenSaver::UnsetAttributes",
            _ => "ScreenSaver::unknown request"
        }
        #[cfg(feature = "shape")]
        Some("SHAPE") => match minor_opcode {
            3 => "Shape::Combine",
            8 => "Shape::GetRectangles",
            7 => "Shape::InputSelected",
            2 => "Shape::Mask",
            4 => "Shape::Offset",
            5 => "Shape::QueryExtents",
            0 => "Shape::QueryVersion",
            1 => "Shape::Rectangles",
            6 => "Shape::SelectInput",
            _ => "Shape::unknown request"
        }
        #[cfg(feature = "shm")]
        Some("MIT-SHM") => match minor_opcode {
            1 => "Shm::Attach",
            6 => "Shm::AttachFd",
            5 => "Shm::CreatePixmap",
            7 => "Shm::CreateSegment",
            2 => "Shm::Detach",
            4 => "Shm::GetImage",
            3 => "Shm::PutImage",
            0 => "Shm::QueryVersion",
            _ => "Shm::unknown request"
        }
        #[cfg(feature = "sync")]
        Some("SYNC") => match minor_opcode {
            7 => "Sync::Await",
            19 => "Sync::AwaitFence",
            9 => "Sync::ChangeAlarm",
            4 => "Sync::ChangeCounter",
            8 => "Sync::CreateAlarm",
            2 => "Sync::CreateCounter",
            14 => "Sync::CreateFence",
            11 => "Sync::DestroyAlarm",
            6 => "Sync::DestroyCounter",
            17 => "Sync::DestroyFence",
            13 => "Sync::GetPriority",
            0 => "Sync::Initialize",
            1 => "Sync::ListSystemCounters",
            10 => "Sync::QueryAlarm",
            5 => "Sync::QueryCounter",
            18 => "Sync::QueryFence",
            16 => "Sync::ResetFence",
            3 => "Sync::SetCounter",
            12 => "Sync::SetPriority",
            15 => "Sync::TriggerFence",
            _ => "Sync::unknown request"
        }
        Some("XC-MISC") => match minor_opcode {
            0 => "XCMisc::GetVersion",
            2 => "XCMisc::GetXIDList",
            1 => "XCMisc::GetXIDRange",
            _ => "XCMisc::unknown request"
        }
        #[cfg(feature = "xevie")]
        Some("XEVIE") => match minor_opcode {
            2 => "Xevie::End",
            0 => "Xevie::QueryVersion",
            4 => "Xevie::SelectInput",
            3 => "Xevie::Send",
            1 => "Xevie::Start",
            _ => "Xevie::unknown request"
        }
        #[cfg(feature = "xf86dri")]
        Some("XFree86-DRI") => match minor_opcode {
            11 => "XF86Dri::AuthConnection",
            3 => "XF86Dri::CloseConnection",
            5 => "XF86Dri::CreateContext",
            7 => "XF86Dri::CreateDrawable",
            6 => "XF86Dri::DestroyContext",
            8 => "XF86Dri::DestroyDrawable",
            4 => "XF86Dri::GetClientDriverName",
            10 => "XF86Dri::GetDeviceInfo",
            9 => "XF86Dri::GetDrawableInfo",
            2 => "XF86Dri::OpenConnection",
            1 => "XF86Dri::QueryDirectRenderingCapable",
            0 => "XF86Dri::QueryVersion",
            _ => "XF86Dri::unknown request"
        }
        #[cfg(feature = "xf86vidmode")]
        Some("XFree86-VidModeExtension") => match minor_opcode {
            7 => "XF86VidMode::AddModeLine",
            8 => "XF86VidMode::DeleteModeLine",
            6 => "XF86VidMode::GetAllModeLines",
            13 => "XF86VidMode::GetDotClocks",
            16 => "XF86VidMode::GetGamma",
            17 => "XF86VidMode::GetGammaRamp",
            19 => "XF86VidMode::GetGammaRampSize",
            1 => "XF86VidMode::GetModeLine",
            4 => "XF86VidMode::GetMonitor",
            20 => "XF86VidMode::GetPermissions",
            11 => "XF86VidMode::GetViewPort",
            5 => "XF86VidMode::LockModeSwitch",
            2 => "XF86VidMode::ModModeLine",
            0 => "XF86VidMode::QueryVersion",
            14 => "XF86VidMode::SetClientVersion",
            15 => "XF86VidMode::SetGamma",
            18 => "XF86VidMode::SetGammaRamp",
            12 => "XF86VidMode::SetViewPort",
            3 => "XF86VidMode::SwitchMode",
            10 => "XF86VidMode::SwitchToMode",
            9 => "XF86VidMode::ValidateModeLine",
            _ => "XF86VidMode::unknown request"
        }
        #[cfg(feature = "xfixes")]
        Some("XFIXES") => match minor_opcode {
            26 => "XFixes::ChangeCursor",
            27 => "XFixes::ChangeCursorByName",
            1 => "XFixes::ChangeSaveSet",
            12 => "XFixes::CopyRegion",
            31 => "XFixes::CreatePointerBarrier",
            5 => "XFixes::CreateRegion",
            6 => "XFixes::CreateRegionFromBitmap",
            8 => "XFixes::CreateRegionFromGC",
            9 => "XFixes::CreateRegionFromPicture",
            7 => "XFixes::CreateRegionFromWindow",
            32 => "XFixes::DeletePointerBarrier",
            10 => "XFixes::DestroyRegion",
            28 => "XFixes::ExpandRegion",
            19 => "XFixes::FetchRegion",
            4 => "XFixes::GetCursorImage",
            25 => "XFixes::GetCursorImageAndName",
            24 => "XFixes::GetCursorName",
            29 => "XFixes::HideCursor",
            14 => "XFixes::IntersectRegion",
            16 => "XFixes::InvertRegion",
            0 => "XFixes::QueryVersion",
            18 => "XFixes::RegionExtents",
            3 => "XFixes::SelectCursorInput",
            2 => "XFixes::SelectSelectionInput",
            23 => "XFixes::SetCursorName",
            20 => "XFixes::SetGCClipRegion",
            22 => "XFixes::SetPictureClipRegion",
            11 => "XFixes::SetRegion",
            21 => "XFixes::SetWindowShapeRegion",
            30 => "XFixes::ShowCursor",
            15 => "XFixes::SubtractRegion",
            17 => "XFixes::TranslateRegion",
            13 => "XFixes::UnionRegion",
            _ => "XFixes::unknown request"
        }
        #[cfg(feature = "xinerama")]
        Some("XINERAMA") => match minor_opcode {
            2 => "Xinerama::GetScreenCount",
            3 => "Xinerama::GetScreenSize",
            1 => "Xinerama::GetState",
            4 => "Xinerama::IsActive",
            5 => "Xinerama::QueryScreens",
            0 => "Xinerama::QueryVersion",
            _ => "Xinerama::unknown request"
        }
        #[cfg(feature = "xinput")]
        Some("XInputExtension") => match minor_opcode {
            19 => "Input::AllowDeviceEvents",
            35 => "Input::ChangeDeviceControl",
            8 => "Input::ChangeDeviceDontPropagateList",
            25 => "Input::ChangeDeviceKeyMapping",
            37 => "Input::ChangeDeviceProperty",
            23 => "Input::ChangeFeedbackControl",
            11 => "Input::ChangeKeyboardDevice",
            12 => "Input::ChangePointerDevice",
            4 => "Input::CloseDevice",
            38 => "Input::DeleteDeviceProperty",
            32 => "Input::DeviceBell",
            28 => "Input::GetDeviceButtonMapping",
            34 => "Input::GetDeviceControl",
            9 => "Input::GetDeviceDontPropagateList",
            20 => "Input::GetDeviceFocus",
            24 => "Input::GetDeviceKeyMapping",
            26 => "Input::GetDeviceModifierMapping",
            10 => "Input::GetDeviceMotionEvents",
            39 => "Input::GetDeviceProperty",
            1 => "Input::GetExtensionVersion",
            22 => "Input::GetFeedbackControl",
            7 => "Input::GetSelectedExtensionEvents",
            13 => "Input::GrabDevice",
            17 => "Input::GrabDeviceButton",
            15 => "Input::GrabDeviceKey",
            36 => "Input::ListDeviceProperties",
            2 => "Input::ListInputDevices",
            3 => "Input::OpenDevice",
            30 => "Input::QueryDeviceState",
            6 => "Input::SelectExtensionEvent",
            31 => "Input::SendExtensionEvent",
            29 => "Input::SetDeviceButtonMapping",
            21 => "Input::SetDeviceFocus",
            5 => "Input::SetDeviceMode",
            27 => "Input::SetDeviceModifierMapping",
            33 => "Input::SetDeviceValuators",
            14 => "Input::UngrabDevice",
            18 => "Input::UngrabDeviceButton",
            16 => "Input::UngrabDeviceKey",
            53 => "Input::XIAllowEvents",
            61 => "Input::XIBarrierReleasePointer",
            42 => "Input::XIChangeCursor",
            43 => "Input::XIChangeHierarchy",
            57 => "Input::XIChangeProperty",
            58 => "Input::XIDeleteProperty",
            45 => "Input::XIGetClientPointer",
            50 => "Input::XIGetFocus",
            59 => "Input::XIGetProperty",
            60 => "Input::XIGetSelectedEvents",
            51 => "Input::XIGrabDevice",
            56 => "Input::XIListProperties",
            54 => "Input::XIPassiveGrabDevice",
            55 => "Input::XIPassiveUngrabDevice",
            48 => "Input::XIQueryDevice",
            40 => "Input::XIQueryPointer",
            47 => "Input::XIQueryVersion",
            46 => "Input::XISelectEvents",
            44 => "Input::XISetClientPointer",
            49 => "Input::XISetFocus",
            52 => "Input::XIUngrabDevice",
            41 => "Input::XIWarpPointer",
            _ => "Input::unknown request"
        }
        #[cfg(feature = "xkb")]
        Some("XKEYBOARD") => match minor_opcode {
            3 => "xkb::Bell",
            10 => "xkb::GetCompatMap",
            6 => "xkb::GetControls",
            24 => "xkb::GetDeviceInfo",
            13 => "xkb::GetIndicatorMap",
            12 => "xkb::GetIndicatorState",
            23 => "xkb::GetKbdByName",
            8 => "xkb::GetMap",
            15 => "xkb::GetNamedIndicator",
            17 => "xkb::GetNames",
            4 => "xkb::GetState",
            5 => "xkb::LatchLockState",
            22 => "xkb::ListComponents",
            21 => "xkb::PerClientFlags",
            1 => "xkb::SelectEvents",
            11 => "xkb::SetCompatMap",
            7 => "xkb::SetControls",
            101 => "xkb::SetDebuggingFlags",
            25 => "xkb::SetDeviceInfo",
            14 => "xkb::SetIndicatorMap",
            9 => "xkb::SetMap",
            16 => "xkb::SetNamedIndicator",
            18 => "xkb::SetNames",
            0 => "xkb::UseExtension",
            _ => "xkb::unknown request"
        }
        #[cfg(feature = "xprint")]
        Some("XpExtension") => match minor_opcode {
            2 => "XPrint::CreateContext",
            5 => "XPrint::PrintDestroyContext",
            10 => "XPrint::PrintEndDoc",
            8 => "XPrint::PrintEndJob",
            14 => "XPrint::PrintEndPage",
            17 => "XPrint::PrintGetAttributes",
            4 => "XPrint::PrintGetContext",
            12 => "XPrint::PrintGetDocumentData",
            24 => "XPrint::PrintGetImageResolution",
            19 => "XPrint::PrintGetOneAttributes",
            21 => "XPrint::PrintGetPageDimensions",
            1 => "XPrint::PrintGetPrinterList",
            6 => "XPrint::PrintGetScreenOfContext",
            16 => "XPrint::PrintInputSelected",
            11 => "XPrint::PrintPutDocumentData",
            22 => "XPrint::PrintQueryScreens",
            0 => "XPrint::PrintQueryVersion",
            20 => "XPrint::PrintRehashPrinterList",
            15 => "XPrint::PrintSelectInput",
            18 => "XPrint::PrintSetAttributes",
            3 => "XPrint::PrintSetContext",
            23 => "XPrint::PrintSetImageResolution",
            9 => "XPrint::PrintStartDoc",
            7 => "XPrint::PrintStartJob",
            13 => "XPrint::PrintStartPage",
            _ => "XPrint::unknown request"
        }
        #[cfg(feature = "xselinux")]
        Some("SELinux") => match minor_opcode {
            22 => "SELinux::GetClientContext",
            4 => "SELinux::GetDeviceContext",
            2 => "SELinux::GetDeviceCreateContext",
            12 => "SELinux::GetPropertyContext",
            9 => "SELinux::GetPropertyCreateContext",
            13 => "SELinux::GetPropertyDataContext",
            11 => "SELinux::GetPropertyUseContext",
            19 => "SELinux::GetSelectionContext",
            16 => "SELinux::GetSelectionCreateContext",
            20 => "SELinux::GetSelectionDataContext",
            18 => "SELinux::GetSelectionUseContext",
            7 => "SELinux::GetWindowContext",
            6 => "SELinux::GetWindowCreateContext",
            14 => "SELinux::ListProperties",
            21 => "SELinux::ListSelections",
            0 => "SELinux::QueryVersion",
            3 => "SELinux::SetDeviceContext",
            1 => "SELinux::SetDeviceCreateContext",
            8 => "SELinux::SetPropertyCreateContext",
            10 => "SELinux::SetPropertyUseContext",
            15 => "SELinux::SetSelectionCreateContext",
            17 => "SELinux::SetSelectionUseContext",
            5 => "SELinux::SetWindowCreateContext",
            _ => "SELinux::unknown request"
        }
        #[cfg(feature = "xtest")]
        Some("XTEST") => match minor_opcode {
            1 => "Test::CompareCursor",
            2 => "Test::FakeInput",
            0 => "Test::GetVersion",
            3 => "Test::GrabControl",
            _ => "Test::unknown request"
        }
        #[cfg(feature = "xv")]
        Some("XVideo") => match minor_opcode {
            14 => "Xv::GetPortAttribute",
            8 => "Xv::GetStill",
            7 => "Xv::GetVideo",
            3 => "Xv::GrabPort",
            16 => "Xv::ListImageFormats",
            18 => "Xv::PutImage",
            6 => "Xv::PutStill",
            5 => "Xv::PutVideo",
            1 => "Xv::QueryAdaptors",
            12 => "Xv::QueryBestSize",
            2 => "Xv::QueryEncodings",
            0 => "Xv::QueryExtension",
            17 => "Xv::QueryImageAttributes",
            15 => "Xv::QueryPortAttributes",
            11 => "Xv::SelectPortNotify",
            10 => "Xv::SelectVideoNotify",
            13 => "Xv::SetPortAttribute",
            19 => "Xv::ShmPutImage",
            9 => "Xv::StopVideo",
            4 => "Xv::UngrabPort",
            _ => "Xv::unknown request"
        }
        #[cfg(feature = "xvmc")]
        Some("XVideo-MotionCompensation") => match minor_opcode {
            2 => "XvMC::CreateContext",
            6 => "XvMC::CreateSubpicture",
            4 => "XvMC::CreateSurface",
            3 => "XvMC::DestroyContext",
            7 => "XvMC::DestroySubpicture",
            5 => "XvMC::DestroySurface",
            8 => "XvMC::ListSubpictureTypes",
            1 => "XvMC::ListSurfaceTypes",
            0 => "XvMC::QueryVersion",
            _ => "XvMC::unknown request"
        }
        _ => "unknown request"
    }
}
