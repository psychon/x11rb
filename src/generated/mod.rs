// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

use crate::errors::ParseError;
use crate::x11_utils::{Event, GenericError};
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
