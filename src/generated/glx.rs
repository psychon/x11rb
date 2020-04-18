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
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "GLX";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 4);

pub type Pixmap = u32;

pub type Context = u32;

pub type Pbuffer = u32;

pub type Window = u32;

pub type Fbconfig = u32;

pub type Drawable = u32;

pub type Float32 = f32;

pub type Float64 = f64;

pub type Bool32 = u32;

pub type ContextTag = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl GenericError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = GenericError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GenericError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for GenericError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for GenericError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&GenericError> for [u8; 32] {
    fn from(input: &GenericError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<GenericError> for [u8; 32] {
    fn from(input: GenericError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadContext error
pub const BAD_CONTEXT_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadContextError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadContextError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadContextError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadContextError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadContextError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadContextError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadContextError> for [u8; 32] {
    fn from(input: &BadContextError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadContextError> for [u8; 32] {
    fn from(input: BadContextError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadContextState error
pub const BAD_CONTEXT_STATE_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadContextStateError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadContextStateError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadContextStateError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadContextStateError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadContextStateError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadContextStateError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadContextStateError> for [u8; 32] {
    fn from(input: &BadContextStateError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadContextStateError> for [u8; 32] {
    fn from(input: BadContextStateError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadDrawable error
pub const BAD_DRAWABLE_ERROR: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadDrawableError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadDrawableError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadDrawableError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadDrawableError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadDrawableError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadDrawableError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadDrawableError> for [u8; 32] {
    fn from(input: &BadDrawableError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadDrawableError> for [u8; 32] {
    fn from(input: BadDrawableError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadPixmap error
pub const BAD_PIXMAP_ERROR: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadPixmapError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadPixmapError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadPixmapError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadPixmapError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadPixmapError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadPixmapError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadPixmapError> for [u8; 32] {
    fn from(input: &BadPixmapError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadPixmapError> for [u8; 32] {
    fn from(input: BadPixmapError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadContextTag error
pub const BAD_CONTEXT_TAG_ERROR: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadContextTagError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadContextTagError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadContextTagError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadContextTagError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadContextTagError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadContextTagError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadContextTagError> for [u8; 32] {
    fn from(input: &BadContextTagError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadContextTagError> for [u8; 32] {
    fn from(input: BadContextTagError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadCurrentWindow error
pub const BAD_CURRENT_WINDOW_ERROR: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadCurrentWindowError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadCurrentWindowError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadCurrentWindowError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadCurrentWindowError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadCurrentWindowError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadCurrentWindowError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadCurrentWindowError> for [u8; 32] {
    fn from(input: &BadCurrentWindowError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadCurrentWindowError> for [u8; 32] {
    fn from(input: BadCurrentWindowError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadRenderRequest error
pub const BAD_RENDER_REQUEST_ERROR: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadRenderRequestError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadRenderRequestError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadRenderRequestError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadRenderRequestError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadRenderRequestError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadRenderRequestError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadRenderRequestError> for [u8; 32] {
    fn from(input: &BadRenderRequestError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadRenderRequestError> for [u8; 32] {
    fn from(input: BadRenderRequestError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadLargeRequest error
pub const BAD_LARGE_REQUEST_ERROR: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadLargeRequestError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadLargeRequestError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadLargeRequestError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadLargeRequestError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadLargeRequestError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadLargeRequestError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadLargeRequestError> for [u8; 32] {
    fn from(input: &BadLargeRequestError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadLargeRequestError> for [u8; 32] {
    fn from(input: BadLargeRequestError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the UnsupportedPrivateRequest error
pub const UNSUPPORTED_PRIVATE_REQUEST_ERROR: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedPrivateRequestError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl UnsupportedPrivateRequestError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = UnsupportedPrivateRequestError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for UnsupportedPrivateRequestError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for UnsupportedPrivateRequestError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for UnsupportedPrivateRequestError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&UnsupportedPrivateRequestError> for [u8; 32] {
    fn from(input: &UnsupportedPrivateRequestError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<UnsupportedPrivateRequestError> for [u8; 32] {
    fn from(input: UnsupportedPrivateRequestError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadFBConfig error
pub const BAD_FB_CONFIG_ERROR: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadFBConfigError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadFBConfigError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadFBConfigError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadFBConfigError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadFBConfigError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadFBConfigError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadFBConfigError> for [u8; 32] {
    fn from(input: &BadFBConfigError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadFBConfigError> for [u8; 32] {
    fn from(input: BadFBConfigError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadPbuffer error
pub const BAD_PBUFFER_ERROR: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadPbufferError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadPbufferError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadPbufferError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadPbufferError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadPbufferError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadPbufferError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadPbufferError> for [u8; 32] {
    fn from(input: &BadPbufferError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadPbufferError> for [u8; 32] {
    fn from(input: BadPbufferError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadCurrentDrawable error
pub const BAD_CURRENT_DRAWABLE_ERROR: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadCurrentDrawableError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadCurrentDrawableError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadCurrentDrawableError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadCurrentDrawableError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadCurrentDrawableError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadCurrentDrawableError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadCurrentDrawableError> for [u8; 32] {
    fn from(input: &BadCurrentDrawableError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadCurrentDrawableError> for [u8; 32] {
    fn from(input: BadCurrentDrawableError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadWindow error
pub const BAD_WINDOW_ERROR: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadWindowError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadWindowError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = BadWindowError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadWindowError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for BadWindowError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for BadWindowError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadWindowError> for [u8; 32] {
    fn from(input: &BadWindowError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadWindowError> for [u8; 32] {
    fn from(input: BadWindowError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the GLXBadProfileARB error
pub const GLX_BAD_PROFILE_ARB_ERROR: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GLXBadProfileARBError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl GLXBadProfileARBError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(21..).ok_or(ParseError::ParseError)?;
        let result = GLXBadProfileARBError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GLXBadProfileARBError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<crate::x11_utils::GenericError<B>> for GLXBadProfileARBError {
    fn from(value: crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&crate::x11_utils::GenericError<B>> for GLXBadProfileARBError {
    fn from(value: &crate::x11_utils::GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&GLXBadProfileARBError> for [u8; 32] {
    fn from(input: &GLXBadProfileARBError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<GLXBadProfileARBError> for [u8; 32] {
    fn from(input: GLXBadProfileARBError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the PbufferClobber event
pub const PBUFFER_CLOBBER_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PbufferClobberEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event_type: u16,
    pub draw_type: u16,
    pub drawable: Drawable,
    pub b_mask: u32,
    pub aux_buffer: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
}
impl PbufferClobberEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let (draw_type, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = Drawable::try_parse(remaining)?;
        let (b_mask, remaining) = u32::try_parse(remaining)?;
        let (aux_buffer, remaining) = u16::try_parse(remaining)?;
        let (x, remaining) = u16::try_parse(remaining)?;
        let (y, remaining) = u16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (count, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let result = PbufferClobberEvent { response_type, sequence, event_type, draw_type, drawable, b_mask, aux_buffer, x, y, width, height, count };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PbufferClobberEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for PbufferClobberEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for PbufferClobberEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&PbufferClobberEvent> for [u8; 32] {
    fn from(input: &PbufferClobberEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event_type = input.event_type.serialize();
        let draw_type = input.draw_type.serialize();
        let drawable = input.drawable.serialize();
        let b_mask = input.b_mask.serialize();
        let aux_buffer = input.aux_buffer.serialize();
        let x = input.x.serialize();
        let y = input.y.serialize();
        let width = input.width.serialize();
        let height = input.height.serialize();
        let count = input.count.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event_type[0], event_type[1], draw_type[0], draw_type[1],
            drawable[0], drawable[1], drawable[2], drawable[3], b_mask[0], b_mask[1], b_mask[2], b_mask[3],
            aux_buffer[0], aux_buffer[1], x[0], x[1], y[0], y[1], width[0], width[1],
            height[0], height[1], count[0], count[1], 0, 0, 0, 0
        ]
    }
}
impl From<PbufferClobberEvent> for [u8; 32] {
    fn from(input: PbufferClobberEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BufferSwapComplete event
pub const BUFFER_SWAP_COMPLETE_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferSwapCompleteEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event_type: u16,
    pub drawable: Drawable,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc: u32,
}
impl BufferSwapCompleteEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (drawable, remaining) = Drawable::try_parse(remaining)?;
        let (ust_hi, remaining) = u32::try_parse(remaining)?;
        let (ust_lo, remaining) = u32::try_parse(remaining)?;
        let (msc_hi, remaining) = u32::try_parse(remaining)?;
        let (msc_lo, remaining) = u32::try_parse(remaining)?;
        let (sbc, remaining) = u32::try_parse(remaining)?;
        let result = BufferSwapCompleteEvent { response_type, sequence, event_type, drawable, ust_hi, ust_lo, msc_hi, msc_lo, sbc };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BufferSwapCompleteEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for BufferSwapCompleteEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for BufferSwapCompleteEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&BufferSwapCompleteEvent> for [u8; 32] {
    fn from(input: &BufferSwapCompleteEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event_type = input.event_type.serialize();
        let drawable = input.drawable.serialize();
        let ust_hi = input.ust_hi.serialize();
        let ust_lo = input.ust_lo.serialize();
        let msc_hi = input.msc_hi.serialize();
        let msc_lo = input.msc_lo.serialize();
        let sbc = input.sbc.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event_type[0], event_type[1], 0, 0,
            drawable[0], drawable[1], drawable[2], drawable[3], ust_hi[0], ust_hi[1], ust_hi[2], ust_hi[3],
            ust_lo[0], ust_lo[1], ust_lo[2], ust_lo[3], msc_hi[0], msc_hi[1], msc_hi[2], msc_hi[3],
            msc_lo[0], msc_lo[1], msc_lo[2], msc_lo[3], sbc[0], sbc[1], sbc[2], sbc[3]
        ]
    }
}
impl From<BufferSwapCompleteEvent> for [u8; 32] {
    fn from(input: BufferSwapCompleteEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum PBCET {
    Damaged = 32791,
    Saved = 32792,
}
impl From<PBCET> for u16 {
    fn from(input: PBCET) -> Self {
        match input {
            PBCET::Damaged => 32791,
            PBCET::Saved => 32792,
        }
    }
}
impl From<PBCET> for Option<u16> {
    fn from(input: PBCET) -> Self {
        Some(u16::from(input))
    }
}
impl From<PBCET> for u32 {
    fn from(input: PBCET) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<PBCET> for Option<u32> {
    fn from(input: PBCET) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for PBCET {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            32791 => Ok(PBCET::Damaged),
            32792 => Ok(PBCET::Saved),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for PBCET {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum PBCDT {
    Window = 32793,
    Pbuffer = 32794,
}
impl From<PBCDT> for u16 {
    fn from(input: PBCDT) -> Self {
        match input {
            PBCDT::Window => 32793,
            PBCDT::Pbuffer => 32794,
        }
    }
}
impl From<PBCDT> for Option<u16> {
    fn from(input: PBCDT) -> Self {
        Some(u16::from(input))
    }
}
impl From<PBCDT> for u32 {
    fn from(input: PBCDT) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<PBCDT> for Option<u32> {
    fn from(input: PBCDT) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for PBCDT {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            32793 => Ok(PBCDT::Window),
            32794 => Ok(PBCDT::Pbuffer),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for PBCDT {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the Render request
pub const RENDER_REQUEST: u8 = 1;
pub fn render<'c, Conn>(conn: &'c Conn, context_tag: ContextTag, data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 1 * data.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        RENDER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the RenderLarge request
pub const RENDER_LARGE_REQUEST: u8 = 2;
pub fn render_large<'c, Conn>(conn: &'c Conn, context_tag: ContextTag, request_num: u16, request_total: u16, data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16 + 1 * data.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let request_num_bytes = request_num.serialize();
    let request_total_bytes = request_total.serialize();
    let data_len: u32 = data.len().try_into()?;
    let data_len_bytes = data_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        RENDER_LARGE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        request_num_bytes[0],
        request_num_bytes[1],
        request_total_bytes[0],
        request_total_bytes[1],
        data_len_bytes[0],
        data_len_bytes[1],
        data_len_bytes[2],
        data_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 3;
pub fn create_context<Conn>(conn: &Conn, context: Context, visual: xproto::Visualid, screen: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_bytes = context.serialize();
    let visual_bytes = visual.serialize();
    let screen_bytes = screen.serialize();
    let share_list_bytes = share_list.serialize();
    let is_direct_bytes = (is_direct as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
        visual_bytes[0],
        visual_bytes[1],
        visual_bytes[2],
        visual_bytes[3],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        share_list_bytes[0],
        share_list_bytes[1],
        share_list_bytes[2],
        share_list_bytes[3],
        is_direct_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the DestroyContext request
pub const DESTROY_CONTEXT_REQUEST: u8 = 4;
pub fn destroy_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_bytes = context.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the MakeCurrent request
pub const MAKE_CURRENT_REQUEST: u8 = 5;
pub fn make_current<Conn>(conn: &Conn, drawable: Drawable, context: Context, old_context_tag: ContextTag) -> Result<Cookie<'_, Conn, MakeCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let context_bytes = context.serialize();
    let old_context_tag_bytes = old_context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        MAKE_CURRENT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
        old_context_tag_bytes[0],
        old_context_tag_bytes[1],
        old_context_tag_bytes[2],
        old_context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeCurrentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl MakeCurrentReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_tag, remaining) = ContextTag::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = MakeCurrentReply { response_type, sequence, length, context_tag };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MakeCurrentReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the IsDirect request
pub const IS_DIRECT_REQUEST: u8 = 6;
pub fn is_direct<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, IsDirectReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_bytes = context.serialize();
    let request0 = [
        extension_information.major_opcode,
        IS_DIRECT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsDirectReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_direct: bool,
}
impl IsDirectReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (is_direct, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let result = IsDirectReply { response_type, sequence, length, is_direct };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IsDirectReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 7;
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let major_version_bytes = major_version.serialize();
    let minor_version_bytes = minor_version.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        major_version_bytes[0],
        major_version_bytes[1],
        major_version_bytes[2],
        major_version_bytes[3],
        minor_version_bytes[0],
        minor_version_bytes[1],
        minor_version_bytes[2],
        minor_version_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u32::try_parse(remaining)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let result = QueryVersionReply { response_type, sequence, length, major_version, minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the WaitGL request
pub const WAIT_GL_REQUEST: u8 = 8;
pub fn wait_gl<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        WAIT_GL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the WaitX request
pub const WAIT_X_REQUEST: u8 = 9;
pub fn wait_x<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        WAIT_X_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CopyContext request
pub const COPY_CONTEXT_REQUEST: u8 = 10;
pub fn copy_context<Conn>(conn: &Conn, src: Context, dest: Context, mask: u32, src_context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let src_bytes = src.serialize();
    let dest_bytes = dest.serialize();
    let mask_bytes = mask.serialize();
    let src_context_tag_bytes = src_context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        COPY_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        dest_bytes[0],
        dest_bytes[1],
        dest_bytes[2],
        dest_bytes[3],
        mask_bytes[0],
        mask_bytes[1],
        mask_bytes[2],
        mask_bytes[3],
        src_context_tag_bytes[0],
        src_context_tag_bytes[1],
        src_context_tag_bytes[2],
        src_context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum GC {
    GL_CURRENT_BIT = 1 << 0,
    GL_POINT_BIT = 1 << 1,
    GL_LINE_BIT = 1 << 2,
    GL_POLYGON_BIT = 1 << 3,
    GL_POLYGON_STIPPLE_BIT = 1 << 4,
    GL_PIXEL_MODE_BIT = 1 << 5,
    GL_LIGHTING_BIT = 1 << 6,
    GL_FOG_BIT = 1 << 7,
    GL_DEPTH_BUFFER_BIT = 1 << 8,
    GL_ACCUM_BUFFER_BIT = 1 << 9,
    GL_STENCIL_BUFFER_BIT = 1 << 10,
    GL_VIEWPORT_BIT = 1 << 11,
    GL_TRANSFORM_BIT = 1 << 12,
    GL_ENABLE_BIT = 1 << 13,
    GL_COLOR_BUFFER_BIT = 1 << 14,
    GL_HINT_BIT = 1 << 15,
    GL_EVAL_BIT = 1 << 16,
    GL_LIST_BIT = 1 << 17,
    GL_TEXTURE_BIT = 1 << 18,
    GL_SCISSOR_BIT = 1 << 19,
    GL_ALL_ATTRIB_BITS = 16_777_215,
}
impl From<GC> for u32 {
    fn from(input: GC) -> Self {
        match input {
            GC::GL_ALL_ATTRIB_BITS => 16_777_215,
            GC::GL_CURRENT_BIT => 1 << 0,
            GC::GL_POINT_BIT => 1 << 1,
            GC::GL_LINE_BIT => 1 << 2,
            GC::GL_POLYGON_BIT => 1 << 3,
            GC::GL_POLYGON_STIPPLE_BIT => 1 << 4,
            GC::GL_PIXEL_MODE_BIT => 1 << 5,
            GC::GL_LIGHTING_BIT => 1 << 6,
            GC::GL_FOG_BIT => 1 << 7,
            GC::GL_DEPTH_BUFFER_BIT => 1 << 8,
            GC::GL_ACCUM_BUFFER_BIT => 1 << 9,
            GC::GL_STENCIL_BUFFER_BIT => 1 << 10,
            GC::GL_VIEWPORT_BIT => 1 << 11,
            GC::GL_TRANSFORM_BIT => 1 << 12,
            GC::GL_ENABLE_BIT => 1 << 13,
            GC::GL_COLOR_BUFFER_BIT => 1 << 14,
            GC::GL_HINT_BIT => 1 << 15,
            GC::GL_EVAL_BIT => 1 << 16,
            GC::GL_LIST_BIT => 1 << 17,
            GC::GL_TEXTURE_BIT => 1 << 18,
            GC::GL_SCISSOR_BIT => 1 << 19,
        }
    }
}
impl From<GC> for Option<u32> {
    fn from(input: GC) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for GC {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(GC::GL_CURRENT_BIT),
            2 => Ok(GC::GL_POINT_BIT),
            4 => Ok(GC::GL_LINE_BIT),
            8 => Ok(GC::GL_POLYGON_BIT),
            16 => Ok(GC::GL_POLYGON_STIPPLE_BIT),
            32 => Ok(GC::GL_PIXEL_MODE_BIT),
            64 => Ok(GC::GL_LIGHTING_BIT),
            128 => Ok(GC::GL_FOG_BIT),
            256 => Ok(GC::GL_DEPTH_BUFFER_BIT),
            512 => Ok(GC::GL_ACCUM_BUFFER_BIT),
            1024 => Ok(GC::GL_STENCIL_BUFFER_BIT),
            2048 => Ok(GC::GL_VIEWPORT_BIT),
            4096 => Ok(GC::GL_TRANSFORM_BIT),
            8192 => Ok(GC::GL_ENABLE_BIT),
            16384 => Ok(GC::GL_COLOR_BUFFER_BIT),
            32768 => Ok(GC::GL_HINT_BIT),
            65536 => Ok(GC::GL_EVAL_BIT),
            131_072 => Ok(GC::GL_LIST_BIT),
            262_144 => Ok(GC::GL_TEXTURE_BIT),
            524_288 => Ok(GC::GL_SCISSOR_BIT),
            16_777_215 => Ok(GC::GL_ALL_ATTRIB_BITS),
            _ => Err(ParseError::ParseError),
        }
    }
}

/// Opcode for the SwapBuffers request
pub const SWAP_BUFFERS_REQUEST: u8 = 11;
pub fn swap_buffers<Conn>(conn: &Conn, context_tag: ContextTag, drawable: Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let drawable_bytes = drawable.serialize();
    let request0 = [
        extension_information.major_opcode,
        SWAP_BUFFERS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the UseXFont request
pub const USE_X_FONT_REQUEST: u8 = 12;
pub fn use_x_font<Conn>(conn: &Conn, context_tag: ContextTag, font: xproto::Font, first: u32, count: u32, list_base: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let font_bytes = font.serialize();
    let first_bytes = first.serialize();
    let count_bytes = count.serialize();
    let list_base_bytes = list_base.serialize();
    let request0 = [
        extension_information.major_opcode,
        USE_X_FONT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        font_bytes[0],
        font_bytes[1],
        font_bytes[2],
        font_bytes[3],
        first_bytes[0],
        first_bytes[1],
        first_bytes[2],
        first_bytes[3],
        count_bytes[0],
        count_bytes[1],
        count_bytes[2],
        count_bytes[3],
        list_base_bytes[0],
        list_base_bytes[1],
        list_base_bytes[2],
        list_base_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CreateGLXPixmap request
pub const CREATE_GLX_PIXMAP_REQUEST: u8 = 13;
pub fn create_glx_pixmap<Conn>(conn: &Conn, screen: u32, visual: xproto::Visualid, pixmap: xproto::Pixmap, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let visual_bytes = visual.serialize();
    let pixmap_bytes = pixmap.serialize();
    let glx_pixmap_bytes = glx_pixmap.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_GLX_PIXMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        visual_bytes[0],
        visual_bytes[1],
        visual_bytes[2],
        visual_bytes[3],
        pixmap_bytes[0],
        pixmap_bytes[1],
        pixmap_bytes[2],
        pixmap_bytes[3],
        glx_pixmap_bytes[0],
        glx_pixmap_bytes[1],
        glx_pixmap_bytes[2],
        glx_pixmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetVisualConfigs request
pub const GET_VISUAL_CONFIGS_REQUEST: u8 = 14;
pub fn get_visual_configs<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetVisualConfigsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_VISUAL_CONFIGS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetVisualConfigsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub num_visuals: u32,
    pub num_properties: u32,
    pub property_list: Vec<u32>,
}
impl GetVisualConfigsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_visuals, remaining) = u32::try_parse(remaining)?;
        let (num_properties, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (property_list, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length as usize)?;
        let result = GetVisualConfigsReply { response_type, sequence, num_visuals, num_properties, property_list };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetVisualConfigsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DestroyGLXPixmap request
pub const DESTROY_GLX_PIXMAP_REQUEST: u8 = 15;
pub fn destroy_glx_pixmap<Conn>(conn: &Conn, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let glx_pixmap_bytes = glx_pixmap.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_GLX_PIXMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        glx_pixmap_bytes[0],
        glx_pixmap_bytes[1],
        glx_pixmap_bytes[2],
        glx_pixmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the VendorPrivate request
pub const VENDOR_PRIVATE_REQUEST: u8 = 16;
pub fn vendor_private<'c, Conn>(conn: &'c Conn, vendor_code: u32, context_tag: ContextTag, data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 1 * data.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let vendor_code_bytes = vendor_code.serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        VENDOR_PRIVATE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        vendor_code_bytes[0],
        vendor_code_bytes[1],
        vendor_code_bytes[2],
        vendor_code_bytes[3],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the VendorPrivateWithReply request
pub const VENDOR_PRIVATE_WITH_REPLY_REQUEST: u8 = 17;
pub fn vendor_private_with_reply<'c, Conn>(conn: &'c Conn, vendor_code: u32, context_tag: ContextTag, data: &[u8]) -> Result<Cookie<'c, Conn, VendorPrivateWithReplyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 1 * data.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let vendor_code_bytes = vendor_code.serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        VENDOR_PRIVATE_WITH_REPLY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        vendor_code_bytes[0],
        vendor_code_bytes[1],
        vendor_code_bytes[2],
        vendor_code_bytes[3],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorPrivateWithReplyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub retval: u32,
    pub data1: [u8; 24],
    pub data2: Vec<u8>,
}
impl VendorPrivateWithReplyReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (retval, remaining) = u32::try_parse(remaining)?;
        let (data1_0, remaining) = u8::try_parse(remaining)?;
        let (data1_1, remaining) = u8::try_parse(remaining)?;
        let (data1_2, remaining) = u8::try_parse(remaining)?;
        let (data1_3, remaining) = u8::try_parse(remaining)?;
        let (data1_4, remaining) = u8::try_parse(remaining)?;
        let (data1_5, remaining) = u8::try_parse(remaining)?;
        let (data1_6, remaining) = u8::try_parse(remaining)?;
        let (data1_7, remaining) = u8::try_parse(remaining)?;
        let (data1_8, remaining) = u8::try_parse(remaining)?;
        let (data1_9, remaining) = u8::try_parse(remaining)?;
        let (data1_10, remaining) = u8::try_parse(remaining)?;
        let (data1_11, remaining) = u8::try_parse(remaining)?;
        let (data1_12, remaining) = u8::try_parse(remaining)?;
        let (data1_13, remaining) = u8::try_parse(remaining)?;
        let (data1_14, remaining) = u8::try_parse(remaining)?;
        let (data1_15, remaining) = u8::try_parse(remaining)?;
        let (data1_16, remaining) = u8::try_parse(remaining)?;
        let (data1_17, remaining) = u8::try_parse(remaining)?;
        let (data1_18, remaining) = u8::try_parse(remaining)?;
        let (data1_19, remaining) = u8::try_parse(remaining)?;
        let (data1_20, remaining) = u8::try_parse(remaining)?;
        let (data1_21, remaining) = u8::try_parse(remaining)?;
        let (data1_22, remaining) = u8::try_parse(remaining)?;
        let (data1_23, remaining) = u8::try_parse(remaining)?;
        let data1 = [
            data1_0,
            data1_1,
            data1_2,
            data1_3,
            data1_4,
            data1_5,
            data1_6,
            data1_7,
            data1_8,
            data1_9,
            data1_10,
            data1_11,
            data1_12,
            data1_13,
            data1_14,
            data1_15,
            data1_16,
            data1_17,
            data1_18,
            data1_19,
            data1_20,
            data1_21,
            data1_22,
            data1_23,
        ];
        let (data2, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = VendorPrivateWithReplyReply { response_type, sequence, retval, data1, data2 };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for VendorPrivateWithReplyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryExtensionsString request
pub const QUERY_EXTENSIONS_STRING_REQUEST: u8 = 18;
pub fn query_extensions_string<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, QueryExtensionsStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_EXTENSIONS_STRING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtensionsStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub n: u32,
}
impl QueryExtensionsStringReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let result = QueryExtensionsStringReply { response_type, sequence, length, n };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryExtensionsStringReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryServerString request
pub const QUERY_SERVER_STRING_REQUEST: u8 = 19;
pub fn query_server_string<Conn>(conn: &Conn, screen: u32, name: u32) -> Result<Cookie<'_, Conn, QueryServerStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let name_bytes = name.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_SERVER_STRING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        name_bytes[0],
        name_bytes[1],
        name_bytes[2],
        name_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryServerStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub string: Vec<u8>,
}
impl QueryServerStringReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (str_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (string, remaining) = crate::x11_utils::parse_list::<u8>(remaining, str_len as usize)?;
        let result = QueryServerStringReply { response_type, sequence, length, string };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryServerStringReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ClientInfo request
pub const CLIENT_INFO_REQUEST: u8 = 20;
pub fn client_info<'c, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, string: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16 + 1 * string.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let major_version_bytes = major_version.serialize();
    let minor_version_bytes = minor_version.serialize();
    let str_len: u32 = string.len().try_into()?;
    let str_len_bytes = str_len.serialize();
    let request0 = [
        extension_information.major_opcode,
        CLIENT_INFO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        major_version_bytes[0],
        major_version_bytes[1],
        major_version_bytes[2],
        major_version_bytes[3],
        minor_version_bytes[0],
        minor_version_bytes[1],
        minor_version_bytes[2],
        minor_version_bytes[3],
        str_len_bytes[0],
        str_len_bytes[1],
        str_len_bytes[2],
        str_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (string).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(string), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetFBConfigs request
pub const GET_FB_CONFIGS_REQUEST: u8 = 21;
pub fn get_fb_configs<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetFBConfigsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_FB_CONFIGS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFBConfigsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub num_fb_configs: u32,
    pub num_properties: u32,
    pub property_list: Vec<u32>,
}
impl GetFBConfigsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_fb_configs, remaining) = u32::try_parse(remaining)?;
        let (num_properties, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (property_list, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length as usize)?;
        let result = GetFBConfigsReply { response_type, sequence, num_fb_configs, num_properties, property_list };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetFBConfigsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreatePixmap request
pub const CREATE_PIXMAP_REQUEST: u8 = 22;
pub fn create_pixmap<'c, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, pixmap: xproto::Pixmap, glx_pixmap: Pixmap, attribs: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24 + 4 * attribs.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let fbconfig_bytes = fbconfig.serialize();
    let pixmap_bytes = pixmap.serialize();
    let glx_pixmap_bytes = glx_pixmap.serialize();
    assert_eq!(0, attribs.len() % 2, "Argument num_attribs has an incorrect length, must be a multiple of 2");
    let num_attribs = u32::try_from(attribs.len() / 2).unwrap();
    let num_attribs_bytes = num_attribs.serialize();
    let attribs_bytes = attribs.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_PIXMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        fbconfig_bytes[0],
        fbconfig_bytes[1],
        fbconfig_bytes[2],
        fbconfig_bytes[3],
        pixmap_bytes[0],
        pixmap_bytes[1],
        pixmap_bytes[2],
        pixmap_bytes[3],
        glx_pixmap_bytes[0],
        glx_pixmap_bytes[1],
        glx_pixmap_bytes[2],
        glx_pixmap_bytes[3],
        num_attribs_bytes[0],
        num_attribs_bytes[1],
        num_attribs_bytes[2],
        num_attribs_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&attribs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&attribs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DestroyPixmap request
pub const DESTROY_PIXMAP_REQUEST: u8 = 23;
pub fn destroy_pixmap<Conn>(conn: &Conn, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let glx_pixmap_bytes = glx_pixmap.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_PIXMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        glx_pixmap_bytes[0],
        glx_pixmap_bytes[1],
        glx_pixmap_bytes[2],
        glx_pixmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CreateNewContext request
pub const CREATE_NEW_CONTEXT_REQUEST: u8 = 24;
pub fn create_new_context<Conn>(conn: &Conn, context: Context, fbconfig: Fbconfig, screen: u32, render_type: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_bytes = context.serialize();
    let fbconfig_bytes = fbconfig.serialize();
    let screen_bytes = screen.serialize();
    let render_type_bytes = render_type.serialize();
    let share_list_bytes = share_list.serialize();
    let is_direct_bytes = (is_direct as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_NEW_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
        fbconfig_bytes[0],
        fbconfig_bytes[1],
        fbconfig_bytes[2],
        fbconfig_bytes[3],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        render_type_bytes[0],
        render_type_bytes[1],
        render_type_bytes[2],
        render_type_bytes[3],
        share_list_bytes[0],
        share_list_bytes[1],
        share_list_bytes[2],
        share_list_bytes[3],
        is_direct_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the QueryContext request
pub const QUERY_CONTEXT_REQUEST: u8 = 25;
pub fn query_context<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, QueryContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_bytes = context.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_CONTEXT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attribs: Vec<u32>,
}
impl QueryContextReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, (num_attribs as usize) * (2))?;
        let result = QueryContextReply { response_type, sequence, length, attribs };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the MakeContextCurrent request
pub const MAKE_CONTEXT_CURRENT_REQUEST: u8 = 26;
pub fn make_context_current<Conn>(conn: &Conn, old_context_tag: ContextTag, drawable: Drawable, read_drawable: Drawable, context: Context) -> Result<Cookie<'_, Conn, MakeContextCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let old_context_tag_bytes = old_context_tag.serialize();
    let drawable_bytes = drawable.serialize();
    let read_drawable_bytes = read_drawable.serialize();
    let context_bytes = context.serialize();
    let request0 = [
        extension_information.major_opcode,
        MAKE_CONTEXT_CURRENT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        old_context_tag_bytes[0],
        old_context_tag_bytes[1],
        old_context_tag_bytes[2],
        old_context_tag_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        read_drawable_bytes[0],
        read_drawable_bytes[1],
        read_drawable_bytes[2],
        read_drawable_bytes[3],
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeContextCurrentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl MakeContextCurrentReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_tag, remaining) = ContextTag::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = MakeContextCurrentReply { response_type, sequence, length, context_tag };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MakeContextCurrentReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreatePbuffer request
pub const CREATE_PBUFFER_REQUEST: u8 = 27;
pub fn create_pbuffer<'c, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, pbuffer: Pbuffer, attribs: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20 + 4 * attribs.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let fbconfig_bytes = fbconfig.serialize();
    let pbuffer_bytes = pbuffer.serialize();
    assert_eq!(0, attribs.len() % 2, "Argument num_attribs has an incorrect length, must be a multiple of 2");
    let num_attribs = u32::try_from(attribs.len() / 2).unwrap();
    let num_attribs_bytes = num_attribs.serialize();
    let attribs_bytes = attribs.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_PBUFFER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        fbconfig_bytes[0],
        fbconfig_bytes[1],
        fbconfig_bytes[2],
        fbconfig_bytes[3],
        pbuffer_bytes[0],
        pbuffer_bytes[1],
        pbuffer_bytes[2],
        pbuffer_bytes[3],
        num_attribs_bytes[0],
        num_attribs_bytes[1],
        num_attribs_bytes[2],
        num_attribs_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&attribs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&attribs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DestroyPbuffer request
pub const DESTROY_PBUFFER_REQUEST: u8 = 28;
pub fn destroy_pbuffer<Conn>(conn: &Conn, pbuffer: Pbuffer) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let pbuffer_bytes = pbuffer.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_PBUFFER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        pbuffer_bytes[0],
        pbuffer_bytes[1],
        pbuffer_bytes[2],
        pbuffer_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetDrawableAttributes request
pub const GET_DRAWABLE_ATTRIBUTES_REQUEST: u8 = 29;
pub fn get_drawable_attributes<Conn>(conn: &Conn, drawable: Drawable) -> Result<Cookie<'_, Conn, GetDrawableAttributesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DRAWABLE_ATTRIBUTES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDrawableAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attribs: Vec<u32>,
}
impl GetDrawableAttributesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, (num_attribs as usize) * (2))?;
        let result = GetDrawableAttributesReply { response_type, sequence, length, attribs };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDrawableAttributesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ChangeDrawableAttributes request
pub const CHANGE_DRAWABLE_ATTRIBUTES_REQUEST: u8 = 30;
pub fn change_drawable_attributes<'c, Conn>(conn: &'c Conn, drawable: Drawable, attribs: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 4 * attribs.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    assert_eq!(0, attribs.len() % 2, "Argument num_attribs has an incorrect length, must be a multiple of 2");
    let num_attribs = u32::try_from(attribs.len() / 2).unwrap();
    let num_attribs_bytes = num_attribs.serialize();
    let attribs_bytes = attribs.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_DRAWABLE_ATTRIBUTES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        num_attribs_bytes[0],
        num_attribs_bytes[1],
        num_attribs_bytes[2],
        num_attribs_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&attribs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&attribs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the CreateWindow request
pub const CREATE_WINDOW_REQUEST: u8 = 31;
pub fn create_window<'c, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, window: xproto::Window, glx_window: Window, attribs: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24 + 4 * attribs.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let fbconfig_bytes = fbconfig.serialize();
    let window_bytes = window.serialize();
    let glx_window_bytes = glx_window.serialize();
    assert_eq!(0, attribs.len() % 2, "Argument num_attribs has an incorrect length, must be a multiple of 2");
    let num_attribs = u32::try_from(attribs.len() / 2).unwrap();
    let num_attribs_bytes = num_attribs.serialize();
    let attribs_bytes = attribs.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_WINDOW_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        fbconfig_bytes[0],
        fbconfig_bytes[1],
        fbconfig_bytes[2],
        fbconfig_bytes[3],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        glx_window_bytes[0],
        glx_window_bytes[1],
        glx_window_bytes[2],
        glx_window_bytes[3],
        num_attribs_bytes[0],
        num_attribs_bytes[1],
        num_attribs_bytes[2],
        num_attribs_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&attribs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&attribs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeleteWindow request
pub const DELETE_WINDOW_REQUEST: u8 = 32;
pub fn delete_window<Conn>(conn: &Conn, glxwindow: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let glxwindow_bytes = glxwindow.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_WINDOW_REQUEST,
        length_bytes[0],
        length_bytes[1],
        glxwindow_bytes[0],
        glxwindow_bytes[1],
        glxwindow_bytes[2],
        glxwindow_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetClientInfoARB request
pub const SET_CLIENT_INFO_ARB_REQUEST: u8 = 33;
pub fn set_client_info_arb<'c, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, gl_versions: &[u32], gl_extension_string: &[u8], glx_extension_string: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24 + 4 * gl_versions.len() + 1 * gl_extension_string.len() + 1 * glx_extension_string.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let major_version_bytes = major_version.serialize();
    let minor_version_bytes = minor_version.serialize();
    assert_eq!(0, gl_versions.len() % 2, "Argument num_versions has an incorrect length, must be a multiple of 2");
    let num_versions = u32::try_from(gl_versions.len() / 2).unwrap();
    let num_versions_bytes = num_versions.serialize();
    let gl_str_len: u32 = gl_extension_string.len().try_into()?;
    let gl_str_len_bytes = gl_str_len.serialize();
    let glx_str_len: u32 = glx_extension_string.len().try_into()?;
    let glx_str_len_bytes = glx_str_len.serialize();
    let gl_versions_bytes = gl_versions.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_CLIENT_INFO_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        major_version_bytes[0],
        major_version_bytes[1],
        major_version_bytes[2],
        major_version_bytes[3],
        minor_version_bytes[0],
        minor_version_bytes[1],
        minor_version_bytes[2],
        minor_version_bytes[3],
        num_versions_bytes[0],
        num_versions_bytes[1],
        num_versions_bytes[2],
        num_versions_bytes[3],
        gl_str_len_bytes[0],
        gl_str_len_bytes[1],
        gl_str_len_bytes[2],
        gl_str_len_bytes[3],
        glx_str_len_bytes[0],
        glx_str_len_bytes[1],
        glx_str_len_bytes[2],
        glx_str_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&gl_versions_bytes).len();
    let length_so_far = length_so_far + (gl_extension_string).len();
    let length_so_far = length_so_far + (glx_extension_string).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&gl_versions_bytes), IoSlice::new(gl_extension_string), IoSlice::new(glx_extension_string), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the CreateContextAttribsARB request
pub const CREATE_CONTEXT_ATTRIBS_ARB_REQUEST: u8 = 34;
pub fn create_context_attribs_arb<'c, Conn>(conn: &'c Conn, context: Context, fbconfig: Fbconfig, screen: u32, share_list: Context, is_direct: bool, attribs: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28 + 4 * attribs.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_bytes = context.serialize();
    let fbconfig_bytes = fbconfig.serialize();
    let screen_bytes = screen.serialize();
    let share_list_bytes = share_list.serialize();
    let is_direct_bytes = (is_direct as u8).serialize();
    assert_eq!(0, attribs.len() % 2, "Argument num_attribs has an incorrect length, must be a multiple of 2");
    let num_attribs = u32::try_from(attribs.len() / 2).unwrap();
    let num_attribs_bytes = num_attribs.serialize();
    let attribs_bytes = attribs.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_CONTEXT_ATTRIBS_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
        fbconfig_bytes[0],
        fbconfig_bytes[1],
        fbconfig_bytes[2],
        fbconfig_bytes[3],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        share_list_bytes[0],
        share_list_bytes[1],
        share_list_bytes[2],
        share_list_bytes[3],
        is_direct_bytes[0],
        0,
        0,
        0,
        num_attribs_bytes[0],
        num_attribs_bytes[1],
        num_attribs_bytes[2],
        num_attribs_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&attribs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&attribs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the SetClientInfo2ARB request
pub const SET_CLIENT_INFO2_ARB_REQUEST: u8 = 35;
pub fn set_client_info2_arb<'c, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, gl_versions: &[u32], gl_extension_string: &[u8], glx_extension_string: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24 + 4 * gl_versions.len() + 1 * gl_extension_string.len() + 1 * glx_extension_string.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let major_version_bytes = major_version.serialize();
    let minor_version_bytes = minor_version.serialize();
    assert_eq!(0, gl_versions.len() % 3, "Argument num_versions has an incorrect length, must be a multiple of 3");
    let num_versions = u32::try_from(gl_versions.len() / 3).unwrap();
    let num_versions_bytes = num_versions.serialize();
    let gl_str_len: u32 = gl_extension_string.len().try_into()?;
    let gl_str_len_bytes = gl_str_len.serialize();
    let glx_str_len: u32 = glx_extension_string.len().try_into()?;
    let glx_str_len_bytes = glx_str_len.serialize();
    let gl_versions_bytes = gl_versions.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_CLIENT_INFO2_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        major_version_bytes[0],
        major_version_bytes[1],
        major_version_bytes[2],
        major_version_bytes[3],
        minor_version_bytes[0],
        minor_version_bytes[1],
        minor_version_bytes[2],
        minor_version_bytes[3],
        num_versions_bytes[0],
        num_versions_bytes[1],
        num_versions_bytes[2],
        num_versions_bytes[3],
        gl_str_len_bytes[0],
        gl_str_len_bytes[1],
        gl_str_len_bytes[2],
        gl_str_len_bytes[3],
        glx_str_len_bytes[0],
        glx_str_len_bytes[1],
        glx_str_len_bytes[2],
        glx_str_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&gl_versions_bytes).len();
    let length_so_far = length_so_far + (gl_extension_string).len();
    let length_so_far = length_so_far + (glx_extension_string).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&gl_versions_bytes), IoSlice::new(gl_extension_string), IoSlice::new(glx_extension_string), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the NewList request
pub const NEW_LIST_REQUEST: u8 = 101;
pub fn new_list<Conn>(conn: &Conn, context_tag: ContextTag, list: u32, mode: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let list_bytes = list.serialize();
    let mode_bytes = mode.serialize();
    let request0 = [
        extension_information.major_opcode,
        NEW_LIST_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        list_bytes[0],
        list_bytes[1],
        list_bytes[2],
        list_bytes[3],
        mode_bytes[0],
        mode_bytes[1],
        mode_bytes[2],
        mode_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the EndList request
pub const END_LIST_REQUEST: u8 = 102;
pub fn end_list<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        END_LIST_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the DeleteLists request
pub const DELETE_LISTS_REQUEST: u8 = 103;
pub fn delete_lists<Conn>(conn: &Conn, context_tag: ContextTag, list: u32, range: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let list_bytes = list.serialize();
    let range_bytes = range.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_LISTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        list_bytes[0],
        list_bytes[1],
        list_bytes[2],
        list_bytes[3],
        range_bytes[0],
        range_bytes[1],
        range_bytes[2],
        range_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GenLists request
pub const GEN_LISTS_REQUEST: u8 = 104;
pub fn gen_lists<Conn>(conn: &Conn, context_tag: ContextTag, range: i32) -> Result<Cookie<'_, Conn, GenListsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let range_bytes = range.serialize();
    let request0 = [
        extension_information.major_opcode,
        GEN_LISTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        range_bytes[0],
        range_bytes[1],
        range_bytes[2],
        range_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenListsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
}
impl GenListsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = u32::try_parse(remaining)?;
        let result = GenListsReply { response_type, sequence, length, ret_val };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GenListsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the FeedbackBuffer request
pub const FEEDBACK_BUFFER_REQUEST: u8 = 105;
pub fn feedback_buffer<Conn>(conn: &Conn, context_tag: ContextTag, size: i32, type_: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let size_bytes = size.serialize();
    let type_bytes = type_.serialize();
    let request0 = [
        extension_information.major_opcode,
        FEEDBACK_BUFFER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        size_bytes[0],
        size_bytes[1],
        size_bytes[2],
        size_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SelectBuffer request
pub const SELECT_BUFFER_REQUEST: u8 = 106;
pub fn select_buffer<Conn>(conn: &Conn, context_tag: ContextTag, size: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let size_bytes = size.serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_BUFFER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        size_bytes[0],
        size_bytes[1],
        size_bytes[2],
        size_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the RenderMode request
pub const RENDER_MODE_REQUEST: u8 = 107;
pub fn render_mode<Conn>(conn: &Conn, context_tag: ContextTag, mode: u32) -> Result<Cookie<'_, Conn, RenderModeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let mode_bytes = mode.serialize();
    let request0 = [
        extension_information.major_opcode,
        RENDER_MODE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        mode_bytes[0],
        mode_bytes[1],
        mode_bytes[2],
        mode_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderModeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
    pub new_mode: u32,
    pub data: Vec<u32>,
}
impl RenderModeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = u32::try_parse(remaining)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (new_mode, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n as usize)?;
        let result = RenderModeReply { response_type, sequence, length, ret_val, new_mode, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RenderModeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum RM {
    GL_RENDER = 7168,
    GL_FEEDBACK = 7169,
    GL_SELECT = 7170,
}
impl From<RM> for u16 {
    fn from(input: RM) -> Self {
        match input {
            RM::GL_RENDER => 7168,
            RM::GL_FEEDBACK => 7169,
            RM::GL_SELECT => 7170,
        }
    }
}
impl From<RM> for Option<u16> {
    fn from(input: RM) -> Self {
        Some(u16::from(input))
    }
}
impl From<RM> for u32 {
    fn from(input: RM) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<RM> for Option<u32> {
    fn from(input: RM) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for RM {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            7168 => Ok(RM::GL_RENDER),
            7169 => Ok(RM::GL_FEEDBACK),
            7170 => Ok(RM::GL_SELECT),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for RM {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the Finish request
pub const FINISH_REQUEST: u8 = 108;
pub fn finish<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<Cookie<'_, Conn, FinishReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        FINISH_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinishReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
}
impl FinishReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let result = FinishReply { response_type, sequence, length };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FinishReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PixelStoref request
pub const PIXEL_STOREF_REQUEST: u8 = 109;
pub fn pixel_storef<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32, datum: Float32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let pname_bytes = pname.serialize();
    let datum_bytes = datum.serialize();
    let request0 = [
        extension_information.major_opcode,
        PIXEL_STOREF_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
        datum_bytes[0],
        datum_bytes[1],
        datum_bytes[2],
        datum_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the PixelStorei request
pub const PIXEL_STOREI_REQUEST: u8 = 110;
pub fn pixel_storei<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32, datum: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let pname_bytes = pname.serialize();
    let datum_bytes = datum.serialize();
    let request0 = [
        extension_information.major_opcode,
        PIXEL_STOREI_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
        datum_bytes[0],
        datum_bytes[1],
        datum_bytes[2],
        datum_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ReadPixels request
pub const READ_PIXELS_REQUEST: u8 = 111;
pub fn read_pixels<Conn>(conn: &Conn, context_tag: ContextTag, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, swap_bytes: bool, lsb_first: bool) -> Result<Cookie<'_, Conn, ReadPixelsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (36) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let format_bytes = format.serialize();
    let type_bytes = type_.serialize();
    let swap_bytes_bytes = (swap_bytes as u8).serialize();
    let lsb_first_bytes = (lsb_first as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        READ_PIXELS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        x_bytes[0],
        x_bytes[1],
        x_bytes[2],
        x_bytes[3],
        y_bytes[0],
        y_bytes[1],
        y_bytes[2],
        y_bytes[3],
        width_bytes[0],
        width_bytes[1],
        width_bytes[2],
        width_bytes[3],
        height_bytes[0],
        height_bytes[1],
        height_bytes[2],
        height_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        swap_bytes_bytes[0],
        lsb_first_bytes[0],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadPixelsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl ReadPixelsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = ReadPixelsReply { response_type, sequence, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ReadPixelsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetBooleanv request
pub const GET_BOOLEANV_REQUEST: u8 = 112;
pub fn get_booleanv<Conn>(conn: &Conn, context_tag: ContextTag, pname: i32) -> Result<Cookie<'_, Conn, GetBooleanvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_BOOLEANV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBooleanvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: bool,
    pub data: Vec<u8>,
}
impl GetBooleanvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, n as usize)?;
        let result = GetBooleanvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetBooleanvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetClipPlane request
pub const GET_CLIP_PLANE_REQUEST: u8 = 113;
pub fn get_clip_plane<Conn>(conn: &Conn, context_tag: ContextTag, plane: i32) -> Result<Cookie<'_, Conn, GetClipPlaneReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let plane_bytes = plane.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CLIP_PLANE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        plane_bytes[0],
        plane_bytes[1],
        plane_bytes[2],
        plane_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetClipPlaneReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<Float64>,
}
impl GetClipPlaneReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, (length as usize) / (2))?;
        let result = GetClipPlaneReply { response_type, sequence, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetClipPlaneReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetDoublev request
pub const GET_DOUBLEV_REQUEST: u8 = 114;
pub fn get_doublev<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetDoublevReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DOUBLEV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetDoublevReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl GetDoublevReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n as usize)?;
        let result = GetDoublevReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDoublevReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetError request
pub const GET_ERROR_REQUEST: u8 = 115;
pub fn get_error<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<Cookie<'_, Conn, GetErrorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_ERROR_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetErrorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub error: i32,
}
impl GetErrorReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (error, remaining) = i32::try_parse(remaining)?;
        let result = GetErrorReply { response_type, sequence, length, error };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetErrorReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetFloatv request
pub const GET_FLOATV_REQUEST: u8 = 116;
pub fn get_floatv<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetFloatvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_FLOATV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetFloatvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetFloatvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetFloatvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetFloatvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetIntegerv request
pub const GET_INTEGERV_REQUEST: u8 = 117;
pub fn get_integerv<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetIntegervReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_INTEGERV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetIntegervReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetIntegervReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetIntegervReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetIntegervReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetLightfv request
pub const GET_LIGHTFV_REQUEST: u8 = 118;
pub fn get_lightfv<Conn>(conn: &Conn, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Conn, GetLightfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let light_bytes = light.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_LIGHTFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        light_bytes[0],
        light_bytes[1],
        light_bytes[2],
        light_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetLightfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetLightfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetLightfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetLightfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetLightiv request
pub const GET_LIGHTIV_REQUEST: u8 = 119;
pub fn get_lightiv<Conn>(conn: &Conn, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Conn, GetLightivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let light_bytes = light.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_LIGHTIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        light_bytes[0],
        light_bytes[1],
        light_bytes[2],
        light_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetLightivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetLightivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetLightivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetLightivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMapdv request
pub const GET_MAPDV_REQUEST: u8 = 120;
pub fn get_mapdv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapdvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let query_bytes = query.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MAPDV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        query_bytes[0],
        query_bytes[1],
        query_bytes[2],
        query_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMapdvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl GetMapdvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n as usize)?;
        let result = GetMapdvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMapdvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMapfv request
pub const GET_MAPFV_REQUEST: u8 = 121;
pub fn get_mapfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let query_bytes = query.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MAPFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        query_bytes[0],
        query_bytes[1],
        query_bytes[2],
        query_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMapfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetMapfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetMapfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMapfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMapiv request
pub const GET_MAPIV_REQUEST: u8 = 122;
pub fn get_mapiv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let query_bytes = query.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MAPIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        query_bytes[0],
        query_bytes[1],
        query_bytes[2],
        query_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMapivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetMapivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetMapivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMapivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMaterialfv request
pub const GET_MATERIALFV_REQUEST: u8 = 123;
pub fn get_materialfv<Conn>(conn: &Conn, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMaterialfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let face_bytes = face.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MATERIALFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        face_bytes[0],
        face_bytes[1],
        face_bytes[2],
        face_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMaterialfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetMaterialfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetMaterialfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMaterialfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMaterialiv request
pub const GET_MATERIALIV_REQUEST: u8 = 124;
pub fn get_materialiv<Conn>(conn: &Conn, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMaterialivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let face_bytes = face.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MATERIALIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        face_bytes[0],
        face_bytes[1],
        face_bytes[2],
        face_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMaterialivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetMaterialivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetMaterialivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMaterialivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPixelMapfv request
pub const GET_PIXEL_MAPFV_REQUEST: u8 = 125;
pub fn get_pixel_mapfv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let map_bytes = map.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PIXEL_MAPFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        map_bytes[0],
        map_bytes[1],
        map_bytes[2],
        map_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPixelMapfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetPixelMapfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetPixelMapfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPixelMapfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPixelMapuiv request
pub const GET_PIXEL_MAPUIV_REQUEST: u8 = 126;
pub fn get_pixel_mapuiv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapuivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let map_bytes = map.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PIXEL_MAPUIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        map_bytes[0],
        map_bytes[1],
        map_bytes[2],
        map_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPixelMapuivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u32,
    pub data: Vec<u32>,
}
impl GetPixelMapuivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n as usize)?;
        let result = GetPixelMapuivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPixelMapuivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPixelMapusv request
pub const GET_PIXEL_MAPUSV_REQUEST: u8 = 127;
pub fn get_pixel_mapusv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapusvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let map_bytes = map.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PIXEL_MAPUSV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        map_bytes[0],
        map_bytes[1],
        map_bytes[2],
        map_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPixelMapusvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u16,
    pub data: Vec<u16>,
}
impl GetPixelMapusvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u16>(remaining, n as usize)?;
        let result = GetPixelMapusvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPixelMapusvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPolygonStipple request
pub const GET_POLYGON_STIPPLE_REQUEST: u8 = 128;
pub fn get_polygon_stipple<Conn>(conn: &Conn, context_tag: ContextTag, lsb_first: bool) -> Result<Cookie<'_, Conn, GetPolygonStippleReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let lsb_first_bytes = (lsb_first as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_POLYGON_STIPPLE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        lsb_first_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPolygonStippleReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl GetPolygonStippleReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetPolygonStippleReply { response_type, sequence, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPolygonStippleReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetString request
pub const GET_STRING_REQUEST: u8 = 129;
pub fn get_string<Conn>(conn: &Conn, context_tag: ContextTag, name: u32) -> Result<Cookie<'_, Conn, GetStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let name_bytes = name.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_STRING_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        name_bytes[0],
        name_bytes[1],
        name_bytes[2],
        name_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub string: Vec<u8>,
}
impl GetStringReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (string, remaining) = crate::x11_utils::parse_list::<u8>(remaining, n as usize)?;
        let result = GetStringReply { response_type, sequence, length, string };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetStringReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexEnvfv request
pub const GET_TEX_ENVFV_REQUEST: u8 = 130;
pub fn get_tex_envfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexEnvfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_ENVFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexEnvfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetTexEnvfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetTexEnvfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexEnvfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexEnviv request
pub const GET_TEX_ENVIV_REQUEST: u8 = 131;
pub fn get_tex_enviv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexEnvivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_ENVIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexEnvivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetTexEnvivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetTexEnvivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexEnvivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexGendv request
pub const GET_TEX_GENDV_REQUEST: u8 = 132;
pub fn get_tex_gendv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGendvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let coord_bytes = coord.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_GENDV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        coord_bytes[0],
        coord_bytes[1],
        coord_bytes[2],
        coord_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexGendvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl GetTexGendvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n as usize)?;
        let result = GetTexGendvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexGendvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexGenfv request
pub const GET_TEX_GENFV_REQUEST: u8 = 133;
pub fn get_tex_genfv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGenfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let coord_bytes = coord.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_GENFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        coord_bytes[0],
        coord_bytes[1],
        coord_bytes[2],
        coord_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexGenfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetTexGenfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetTexGenfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexGenfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexGeniv request
pub const GET_TEX_GENIV_REQUEST: u8 = 134;
pub fn get_tex_geniv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGenivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let coord_bytes = coord.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_GENIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        coord_bytes[0],
        coord_bytes[1],
        coord_bytes[2],
        coord_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexGenivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetTexGenivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetTexGenivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexGenivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexImage request
pub const GET_TEX_IMAGE_REQUEST: u8 = 135;
pub fn get_tex_image<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetTexImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let level_bytes = level.serialize();
    let format_bytes = format.serialize();
    let type_bytes = type_.serialize();
    let swap_bytes_bytes = (swap_bytes as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_IMAGE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        level_bytes[0],
        level_bytes[1],
        level_bytes[2],
        level_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        swap_bytes_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexImageReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub data: Vec<u8>,
}
impl GetTexImageReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let (depth, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetTexImageReply { response_type, sequence, width, height, depth, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexImageReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexParameterfv request
pub const GET_TEX_PARAMETERFV_REQUEST: u8 = 136;
pub fn get_tex_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_PARAMETERFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetTexParameterfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetTexParameterfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexParameterfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexParameteriv request
pub const GET_TEX_PARAMETERIV_REQUEST: u8 = 137;
pub fn get_tex_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_PARAMETERIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetTexParameterivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetTexParameterivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexParameterivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexLevelParameterfv request
pub const GET_TEX_LEVEL_PARAMETERFV_REQUEST: u8 = 138;
pub fn get_tex_level_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Conn, GetTexLevelParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let level_bytes = level.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_LEVEL_PARAMETERFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        level_bytes[0],
        level_bytes[1],
        level_bytes[2],
        level_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTexLevelParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetTexLevelParameterfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetTexLevelParameterfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexLevelParameterfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetTexLevelParameteriv request
pub const GET_TEX_LEVEL_PARAMETERIV_REQUEST: u8 = 139;
pub fn get_tex_level_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Conn, GetTexLevelParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let level_bytes = level.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_TEX_LEVEL_PARAMETERIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        level_bytes[0],
        level_bytes[1],
        level_bytes[2],
        level_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexLevelParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetTexLevelParameterivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetTexLevelParameterivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetTexLevelParameterivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the IsEnabled request
pub const IS_ENABLED_REQUEST: u8 = 140;
pub fn is_enabled<Conn>(conn: &Conn, context_tag: ContextTag, capability: u32) -> Result<Cookie<'_, Conn, IsEnabledReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let capability_bytes = capability.serialize();
    let request0 = [
        extension_information.major_opcode,
        IS_ENABLED_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        capability_bytes[0],
        capability_bytes[1],
        capability_bytes[2],
        capability_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsEnabledReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl IsEnabledReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        let result = IsEnabledReply { response_type, sequence, length, ret_val };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IsEnabledReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the IsList request
pub const IS_LIST_REQUEST: u8 = 141;
pub fn is_list<Conn>(conn: &Conn, context_tag: ContextTag, list: u32) -> Result<Cookie<'_, Conn, IsListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let list_bytes = list.serialize();
    let request0 = [
        extension_information.major_opcode,
        IS_LIST_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        list_bytes[0],
        list_bytes[1],
        list_bytes[2],
        list_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsListReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl IsListReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        let result = IsListReply { response_type, sequence, length, ret_val };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IsListReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the Flush request
pub const FLUSH_REQUEST: u8 = 142;
pub fn flush<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let request0 = [
        extension_information.major_opcode,
        FLUSH_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the AreTexturesResident request
pub const ARE_TEXTURES_RESIDENT_REQUEST: u8 = 143;
pub fn are_textures_resident<'c, Conn>(conn: &'c Conn, context_tag: ContextTag, textures: &[u32]) -> Result<Cookie<'c, Conn, AreTexturesResidentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 4 * textures.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let n: i32 = textures.len().try_into()?;
    let n_bytes = n.serialize();
    let textures_bytes = textures.serialize();
    let request0 = [
        extension_information.major_opcode,
        ARE_TEXTURES_RESIDENT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        n_bytes[0],
        n_bytes[1],
        n_bytes[2],
        n_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&textures_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&textures_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreTexturesResidentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub ret_val: Bool32,
    pub data: Vec<u8>,
}
impl AreTexturesResidentReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = AreTexturesResidentReply { response_type, sequence, ret_val, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AreTexturesResidentReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DeleteTextures request
pub const DELETE_TEXTURES_REQUEST: u8 = 144;
pub fn delete_textures<'c, Conn>(conn: &'c Conn, context_tag: ContextTag, textures: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 4 * textures.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let n: i32 = textures.len().try_into()?;
    let n_bytes = n.serialize();
    let textures_bytes = textures.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_TEXTURES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        n_bytes[0],
        n_bytes[1],
        n_bytes[2],
        n_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&textures_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&textures_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GenTextures request
pub const GEN_TEXTURES_REQUEST: u8 = 145;
pub fn gen_textures<Conn>(conn: &Conn, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Conn, GenTexturesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let n_bytes = n.serialize();
    let request0 = [
        extension_information.major_opcode,
        GEN_TEXTURES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        n_bytes[0],
        n_bytes[1],
        n_bytes[2],
        n_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenTexturesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u32>,
}
impl GenTexturesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length as usize)?;
        let result = GenTexturesReply { response_type, sequence, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GenTexturesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the IsTexture request
pub const IS_TEXTURE_REQUEST: u8 = 146;
pub fn is_texture<Conn>(conn: &Conn, context_tag: ContextTag, texture: u32) -> Result<Cookie<'_, Conn, IsTextureReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let texture_bytes = texture.serialize();
    let request0 = [
        extension_information.major_opcode,
        IS_TEXTURE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        texture_bytes[0],
        texture_bytes[1],
        texture_bytes[2],
        texture_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsTextureReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl IsTextureReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        let result = IsTextureReply { response_type, sequence, length, ret_val };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IsTextureReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetColorTable request
pub const GET_COLOR_TABLE_REQUEST: u8 = 147;
pub fn get_color_table<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetColorTableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let format_bytes = format.serialize();
    let type_bytes = type_.serialize();
    let swap_bytes_bytes = (swap_bytes as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_COLOR_TABLE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        swap_bytes_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetColorTableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width: i32,
    pub data: Vec<u8>,
}
impl GetColorTableReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetColorTableReply { response_type, sequence, width, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetColorTableReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetColorTableParameterfv request
pub const GET_COLOR_TABLE_PARAMETERFV_REQUEST: u8 = 148;
pub fn get_color_table_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetColorTableParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_COLOR_TABLE_PARAMETERFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetColorTableParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetColorTableParameterfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetColorTableParameterfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetColorTableParameterfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetColorTableParameteriv request
pub const GET_COLOR_TABLE_PARAMETERIV_REQUEST: u8 = 149;
pub fn get_color_table_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetColorTableParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_COLOR_TABLE_PARAMETERIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetColorTableParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetColorTableParameterivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetColorTableParameterivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetColorTableParameterivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetConvolutionFilter request
pub const GET_CONVOLUTION_FILTER_REQUEST: u8 = 150;
pub fn get_convolution_filter<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetConvolutionFilterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let format_bytes = format.serialize();
    let type_bytes = type_.serialize();
    let swap_bytes_bytes = (swap_bytes as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CONVOLUTION_FILTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        swap_bytes_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetConvolutionFilterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}
impl GetConvolutionFilterReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetConvolutionFilterReply { response_type, sequence, width, height, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetConvolutionFilterReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetConvolutionParameterfv request
pub const GET_CONVOLUTION_PARAMETERFV_REQUEST: u8 = 151;
pub fn get_convolution_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetConvolutionParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CONVOLUTION_PARAMETERFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetConvolutionParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetConvolutionParameterfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetConvolutionParameterfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetConvolutionParameterfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetConvolutionParameteriv request
pub const GET_CONVOLUTION_PARAMETERIV_REQUEST: u8 = 152;
pub fn get_convolution_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetConvolutionParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_CONVOLUTION_PARAMETERIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetConvolutionParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetConvolutionParameterivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetConvolutionParameterivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetConvolutionParameterivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetSeparableFilter request
pub const GET_SEPARABLE_FILTER_REQUEST: u8 = 153;
pub fn get_separable_filter<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetSeparableFilterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let format_bytes = format.serialize();
    let type_bytes = type_.serialize();
    let swap_bytes_bytes = (swap_bytes as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SEPARABLE_FILTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        swap_bytes_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSeparableFilterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub row_w: i32,
    pub col_h: i32,
    pub rows_and_cols: Vec<u8>,
}
impl GetSeparableFilterReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (row_w, remaining) = i32::try_parse(remaining)?;
        let (col_h, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (rows_and_cols, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetSeparableFilterReply { response_type, sequence, row_w, col_h, rows_and_cols };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSeparableFilterReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetHistogram request
pub const GET_HISTOGRAM_REQUEST: u8 = 154;
pub fn get_histogram<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Conn, GetHistogramReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let format_bytes = format.serialize();
    let type_bytes = type_.serialize();
    let swap_bytes_bytes = (swap_bytes as u8).serialize();
    let reset_bytes = (reset as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_HISTOGRAM_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        swap_bytes_bytes[0],
        reset_bytes[0],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetHistogramReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width: i32,
    pub data: Vec<u8>,
}
impl GetHistogramReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetHistogramReply { response_type, sequence, width, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetHistogramReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetHistogramParameterfv request
pub const GET_HISTOGRAM_PARAMETERFV_REQUEST: u8 = 155;
pub fn get_histogram_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetHistogramParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_HISTOGRAM_PARAMETERFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetHistogramParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetHistogramParameterfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetHistogramParameterfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetHistogramParameterfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetHistogramParameteriv request
pub const GET_HISTOGRAM_PARAMETERIV_REQUEST: u8 = 156;
pub fn get_histogram_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetHistogramParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_HISTOGRAM_PARAMETERIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetHistogramParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetHistogramParameterivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetHistogramParameterivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetHistogramParameterivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMinmax request
pub const GET_MINMAX_REQUEST: u8 = 157;
pub fn get_minmax<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Conn, GetMinmaxReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let format_bytes = format.serialize();
    let type_bytes = type_.serialize();
    let swap_bytes_bytes = (swap_bytes as u8).serialize();
    let reset_bytes = (reset as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MINMAX_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        swap_bytes_bytes[0],
        reset_bytes[0],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMinmaxReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl GetMinmaxReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetMinmaxReply { response_type, sequence, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMinmaxReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMinmaxParameterfv request
pub const GET_MINMAX_PARAMETERFV_REQUEST: u8 = 158;
pub fn get_minmax_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMinmaxParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MINMAX_PARAMETERFV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetMinmaxParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetMinmaxParameterfvReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n as usize)?;
        let result = GetMinmaxParameterfvReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMinmaxParameterfvReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetMinmaxParameteriv request
pub const GET_MINMAX_PARAMETERIV_REQUEST: u8 = 159;
pub fn get_minmax_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMinmaxParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MINMAX_PARAMETERIV_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMinmaxParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetMinmaxParameterivReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetMinmaxParameterivReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMinmaxParameterivReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetCompressedTexImageARB request
pub const GET_COMPRESSED_TEX_IMAGE_ARB_REQUEST: u8 = 160;
pub fn get_compressed_tex_image_arb<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32) -> Result<Cookie<'_, Conn, GetCompressedTexImageARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let level_bytes = level.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_COMPRESSED_TEX_IMAGE_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        level_bytes[0],
        level_bytes[1],
        level_bytes[2],
        level_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCompressedTexImageARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub size: i32,
    pub data: Vec<u8>,
}
impl GetCompressedTexImageARBReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (size, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetCompressedTexImageARBReply { response_type, sequence, size, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetCompressedTexImageARBReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DeleteQueriesARB request
pub const DELETE_QUERIES_ARB_REQUEST: u8 = 161;
pub fn delete_queries_arb<'c, Conn>(conn: &'c Conn, context_tag: ContextTag, ids: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 4 * ids.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let n: i32 = ids.len().try_into()?;
    let n_bytes = n.serialize();
    let ids_bytes = ids.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_QUERIES_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        n_bytes[0],
        n_bytes[1],
        n_bytes[2],
        n_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&ids_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&ids_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GenQueriesARB request
pub const GEN_QUERIES_ARB_REQUEST: u8 = 162;
pub fn gen_queries_arb<Conn>(conn: &Conn, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Conn, GenQueriesARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let n_bytes = n.serialize();
    let request0 = [
        extension_information.major_opcode,
        GEN_QUERIES_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        n_bytes[0],
        n_bytes[1],
        n_bytes[2],
        n_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenQueriesARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u32>,
}
impl GenQueriesARBReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length as usize)?;
        let result = GenQueriesARBReply { response_type, sequence, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GenQueriesARBReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the IsQueryARB request
pub const IS_QUERY_ARB_REQUEST: u8 = 163;
pub fn is_query_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32) -> Result<Cookie<'_, Conn, IsQueryARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let id_bytes = id.serialize();
    let request0 = [
        extension_information.major_opcode,
        IS_QUERY_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsQueryARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl IsQueryARBReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        let result = IsQueryARBReply { response_type, sequence, length, ret_val };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IsQueryARBReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetQueryivARB request
pub const GET_QUERYIV_ARB_REQUEST: u8 = 164;
pub fn get_queryiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let target_bytes = target.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_QUERYIV_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetQueryivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetQueryivARBReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetQueryivARBReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetQueryivARBReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetQueryObjectivARB request
pub const GET_QUERY_OBJECTIV_ARB_REQUEST: u8 = 165;
pub fn get_query_objectiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryObjectivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let id_bytes = id.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_QUERY_OBJECTIV_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetQueryObjectivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl GetQueryObjectivARBReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n as usize)?;
        let result = GetQueryObjectivARBReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetQueryObjectivARBReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetQueryObjectuivARB request
pub const GET_QUERY_OBJECTUIV_ARB_REQUEST: u8 = 166;
pub fn get_query_objectuiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryObjectuivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let context_tag_bytes = context_tag.serialize();
    let id_bytes = id.serialize();
    let pname_bytes = pname.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_QUERY_OBJECTUIV_ARB_REQUEST,
        length_bytes[0],
        length_bytes[1],
        context_tag_bytes[0],
        context_tag_bytes[1],
        context_tag_bytes[2],
        context_tag_bytes[3],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        pname_bytes[0],
        pname_bytes[1],
        pname_bytes[2],
        pname_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetQueryObjectuivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u32,
    pub data: Vec<u32>,
}
impl GetQueryObjectuivARBReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n as usize)?;
        let result = GetQueryObjectuivARBReply { response_type, sequence, length, datum, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetQueryObjectuivARBReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn glx_render<'c>(&'c self, context_tag: ContextTag, data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        render(self, context_tag, data)
    }

    fn glx_render_large<'c>(&'c self, context_tag: ContextTag, request_num: u16, request_total: u16, data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        render_large(self, context_tag, request_num, request_total, data)
    }

    fn glx_create_context(&self, context: Context, visual: xproto::Visualid, screen: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_context(self, context, visual, screen, share_list, is_direct)
    }

    fn glx_destroy_context(&self, context: Context) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_context(self, context)
    }

    fn glx_make_current(&self, drawable: Drawable, context: Context, old_context_tag: ContextTag) -> Result<Cookie<'_, Self, MakeCurrentReply>, ConnectionError>
    {
        make_current(self, drawable, context, old_context_tag)
    }

    fn glx_is_direct(&self, context: Context) -> Result<Cookie<'_, Self, IsDirectReply>, ConnectionError>
    {
        is_direct(self, context)
    }

    fn glx_query_version(&self, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }

    fn glx_wait_gl(&self, context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        wait_gl(self, context_tag)
    }

    fn glx_wait_x(&self, context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        wait_x(self, context_tag)
    }

    fn glx_copy_context(&self, src: Context, dest: Context, mask: u32, src_context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        copy_context(self, src, dest, mask, src_context_tag)
    }

    fn glx_swap_buffers(&self, context_tag: ContextTag, drawable: Drawable) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        swap_buffers(self, context_tag, drawable)
    }

    fn glx_use_x_font(&self, context_tag: ContextTag, font: xproto::Font, first: u32, count: u32, list_base: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        use_x_font(self, context_tag, font, first, count, list_base)
    }

    fn glx_create_glx_pixmap(&self, screen: u32, visual: xproto::Visualid, pixmap: xproto::Pixmap, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_glx_pixmap(self, screen, visual, pixmap, glx_pixmap)
    }

    fn glx_get_visual_configs(&self, screen: u32) -> Result<Cookie<'_, Self, GetVisualConfigsReply>, ConnectionError>
    {
        get_visual_configs(self, screen)
    }

    fn glx_destroy_glx_pixmap(&self, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_glx_pixmap(self, glx_pixmap)
    }

    fn glx_vendor_private<'c>(&'c self, vendor_code: u32, context_tag: ContextTag, data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        vendor_private(self, vendor_code, context_tag, data)
    }

    fn glx_vendor_private_with_reply<'c>(&'c self, vendor_code: u32, context_tag: ContextTag, data: &[u8]) -> Result<Cookie<'c, Self, VendorPrivateWithReplyReply>, ConnectionError>
    {
        vendor_private_with_reply(self, vendor_code, context_tag, data)
    }

    fn glx_query_extensions_string(&self, screen: u32) -> Result<Cookie<'_, Self, QueryExtensionsStringReply>, ConnectionError>
    {
        query_extensions_string(self, screen)
    }

    fn glx_query_server_string(&self, screen: u32, name: u32) -> Result<Cookie<'_, Self, QueryServerStringReply>, ConnectionError>
    {
        query_server_string(self, screen, name)
    }

    fn glx_client_info<'c>(&'c self, major_version: u32, minor_version: u32, string: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        client_info(self, major_version, minor_version, string)
    }

    fn glx_get_fb_configs(&self, screen: u32) -> Result<Cookie<'_, Self, GetFBConfigsReply>, ConnectionError>
    {
        get_fb_configs(self, screen)
    }

    fn glx_create_pixmap<'c>(&'c self, screen: u32, fbconfig: Fbconfig, pixmap: xproto::Pixmap, glx_pixmap: Pixmap, attribs: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_pixmap(self, screen, fbconfig, pixmap, glx_pixmap, attribs)
    }

    fn glx_destroy_pixmap(&self, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_pixmap(self, glx_pixmap)
    }

    fn glx_create_new_context(&self, context: Context, fbconfig: Fbconfig, screen: u32, render_type: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_new_context(self, context, fbconfig, screen, render_type, share_list, is_direct)
    }

    fn glx_query_context(&self, context: Context) -> Result<Cookie<'_, Self, QueryContextReply>, ConnectionError>
    {
        query_context(self, context)
    }

    fn glx_make_context_current(&self, old_context_tag: ContextTag, drawable: Drawable, read_drawable: Drawable, context: Context) -> Result<Cookie<'_, Self, MakeContextCurrentReply>, ConnectionError>
    {
        make_context_current(self, old_context_tag, drawable, read_drawable, context)
    }

    fn glx_create_pbuffer<'c>(&'c self, screen: u32, fbconfig: Fbconfig, pbuffer: Pbuffer, attribs: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_pbuffer(self, screen, fbconfig, pbuffer, attribs)
    }

    fn glx_destroy_pbuffer(&self, pbuffer: Pbuffer) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_pbuffer(self, pbuffer)
    }

    fn glx_get_drawable_attributes(&self, drawable: Drawable) -> Result<Cookie<'_, Self, GetDrawableAttributesReply>, ConnectionError>
    {
        get_drawable_attributes(self, drawable)
    }

    fn glx_change_drawable_attributes<'c>(&'c self, drawable: Drawable, attribs: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_drawable_attributes(self, drawable, attribs)
    }

    fn glx_create_window<'c>(&'c self, screen: u32, fbconfig: Fbconfig, window: xproto::Window, glx_window: Window, attribs: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_window(self, screen, fbconfig, window, glx_window, attribs)
    }

    fn glx_delete_window(&self, glxwindow: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_window(self, glxwindow)
    }

    fn glx_set_client_info_arb<'c>(&'c self, major_version: u32, minor_version: u32, gl_versions: &[u32], gl_extension_string: &[u8], glx_extension_string: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_client_info_arb(self, major_version, minor_version, gl_versions, gl_extension_string, glx_extension_string)
    }

    fn glx_create_context_attribs_arb<'c>(&'c self, context: Context, fbconfig: Fbconfig, screen: u32, share_list: Context, is_direct: bool, attribs: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_context_attribs_arb(self, context, fbconfig, screen, share_list, is_direct, attribs)
    }

    fn glx_set_client_info2_arb<'c>(&'c self, major_version: u32, minor_version: u32, gl_versions: &[u32], gl_extension_string: &[u8], glx_extension_string: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_client_info2_arb(self, major_version, minor_version, gl_versions, gl_extension_string, glx_extension_string)
    }

    fn glx_new_list(&self, context_tag: ContextTag, list: u32, mode: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        new_list(self, context_tag, list, mode)
    }

    fn glx_end_list(&self, context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        end_list(self, context_tag)
    }

    fn glx_delete_lists(&self, context_tag: ContextTag, list: u32, range: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_lists(self, context_tag, list, range)
    }

    fn glx_gen_lists(&self, context_tag: ContextTag, range: i32) -> Result<Cookie<'_, Self, GenListsReply>, ConnectionError>
    {
        gen_lists(self, context_tag, range)
    }

    fn glx_feedback_buffer(&self, context_tag: ContextTag, size: i32, type_: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        feedback_buffer(self, context_tag, size, type_)
    }

    fn glx_select_buffer(&self, context_tag: ContextTag, size: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_buffer(self, context_tag, size)
    }

    fn glx_render_mode(&self, context_tag: ContextTag, mode: u32) -> Result<Cookie<'_, Self, RenderModeReply>, ConnectionError>
    {
        render_mode(self, context_tag, mode)
    }

    fn glx_finish(&self, context_tag: ContextTag) -> Result<Cookie<'_, Self, FinishReply>, ConnectionError>
    {
        finish(self, context_tag)
    }

    fn glx_pixel_storef(&self, context_tag: ContextTag, pname: u32, datum: Float32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        pixel_storef(self, context_tag, pname, datum)
    }

    fn glx_pixel_storei(&self, context_tag: ContextTag, pname: u32, datum: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        pixel_storei(self, context_tag, pname, datum)
    }

    fn glx_read_pixels(&self, context_tag: ContextTag, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, swap_bytes: bool, lsb_first: bool) -> Result<Cookie<'_, Self, ReadPixelsReply>, ConnectionError>
    {
        read_pixels(self, context_tag, x, y, width, height, format, type_, swap_bytes, lsb_first)
    }

    fn glx_get_booleanv(&self, context_tag: ContextTag, pname: i32) -> Result<Cookie<'_, Self, GetBooleanvReply>, ConnectionError>
    {
        get_booleanv(self, context_tag, pname)
    }

    fn glx_get_clip_plane(&self, context_tag: ContextTag, plane: i32) -> Result<Cookie<'_, Self, GetClipPlaneReply>, ConnectionError>
    {
        get_clip_plane(self, context_tag, plane)
    }

    fn glx_get_doublev(&self, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Self, GetDoublevReply>, ConnectionError>
    {
        get_doublev(self, context_tag, pname)
    }

    fn glx_get_error(&self, context_tag: ContextTag) -> Result<Cookie<'_, Self, GetErrorReply>, ConnectionError>
    {
        get_error(self, context_tag)
    }

    fn glx_get_floatv(&self, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Self, GetFloatvReply>, ConnectionError>
    {
        get_floatv(self, context_tag, pname)
    }

    fn glx_get_integerv(&self, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Self, GetIntegervReply>, ConnectionError>
    {
        get_integerv(self, context_tag, pname)
    }

    fn glx_get_lightfv(&self, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Self, GetLightfvReply>, ConnectionError>
    {
        get_lightfv(self, context_tag, light, pname)
    }

    fn glx_get_lightiv(&self, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Self, GetLightivReply>, ConnectionError>
    {
        get_lightiv(self, context_tag, light, pname)
    }

    fn glx_get_mapdv(&self, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Self, GetMapdvReply>, ConnectionError>
    {
        get_mapdv(self, context_tag, target, query)
    }

    fn glx_get_mapfv(&self, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Self, GetMapfvReply>, ConnectionError>
    {
        get_mapfv(self, context_tag, target, query)
    }

    fn glx_get_mapiv(&self, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Self, GetMapivReply>, ConnectionError>
    {
        get_mapiv(self, context_tag, target, query)
    }

    fn glx_get_materialfv(&self, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Self, GetMaterialfvReply>, ConnectionError>
    {
        get_materialfv(self, context_tag, face, pname)
    }

    fn glx_get_materialiv(&self, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Self, GetMaterialivReply>, ConnectionError>
    {
        get_materialiv(self, context_tag, face, pname)
    }

    fn glx_get_pixel_mapfv(&self, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Self, GetPixelMapfvReply>, ConnectionError>
    {
        get_pixel_mapfv(self, context_tag, map)
    }

    fn glx_get_pixel_mapuiv(&self, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Self, GetPixelMapuivReply>, ConnectionError>
    {
        get_pixel_mapuiv(self, context_tag, map)
    }

    fn glx_get_pixel_mapusv(&self, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Self, GetPixelMapusvReply>, ConnectionError>
    {
        get_pixel_mapusv(self, context_tag, map)
    }

    fn glx_get_polygon_stipple(&self, context_tag: ContextTag, lsb_first: bool) -> Result<Cookie<'_, Self, GetPolygonStippleReply>, ConnectionError>
    {
        get_polygon_stipple(self, context_tag, lsb_first)
    }

    fn glx_get_string(&self, context_tag: ContextTag, name: u32) -> Result<Cookie<'_, Self, GetStringReply>, ConnectionError>
    {
        get_string(self, context_tag, name)
    }

    fn glx_get_tex_envfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexEnvfvReply>, ConnectionError>
    {
        get_tex_envfv(self, context_tag, target, pname)
    }

    fn glx_get_tex_enviv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexEnvivReply>, ConnectionError>
    {
        get_tex_enviv(self, context_tag, target, pname)
    }

    fn glx_get_tex_gendv(&self, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexGendvReply>, ConnectionError>
    {
        get_tex_gendv(self, context_tag, coord, pname)
    }

    fn glx_get_tex_genfv(&self, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexGenfvReply>, ConnectionError>
    {
        get_tex_genfv(self, context_tag, coord, pname)
    }

    fn glx_get_tex_geniv(&self, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexGenivReply>, ConnectionError>
    {
        get_tex_geniv(self, context_tag, coord, pname)
    }

    fn glx_get_tex_image(&self, context_tag: ContextTag, target: u32, level: i32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Self, GetTexImageReply>, ConnectionError>
    {
        get_tex_image(self, context_tag, target, level, format, type_, swap_bytes)
    }

    fn glx_get_tex_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexParameterfvReply>, ConnectionError>
    {
        get_tex_parameterfv(self, context_tag, target, pname)
    }

    fn glx_get_tex_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexParameterivReply>, ConnectionError>
    {
        get_tex_parameteriv(self, context_tag, target, pname)
    }

    fn glx_get_tex_level_parameterfv(&self, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Self, GetTexLevelParameterfvReply>, ConnectionError>
    {
        get_tex_level_parameterfv(self, context_tag, target, level, pname)
    }

    fn glx_get_tex_level_parameteriv(&self, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Self, GetTexLevelParameterivReply>, ConnectionError>
    {
        get_tex_level_parameteriv(self, context_tag, target, level, pname)
    }

    fn glx_is_enabled(&self, context_tag: ContextTag, capability: u32) -> Result<Cookie<'_, Self, IsEnabledReply>, ConnectionError>
    {
        is_enabled(self, context_tag, capability)
    }

    fn glx_is_list(&self, context_tag: ContextTag, list: u32) -> Result<Cookie<'_, Self, IsListReply>, ConnectionError>
    {
        is_list(self, context_tag, list)
    }

    fn glx_flush(&self, context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        flush(self, context_tag)
    }

    fn glx_are_textures_resident<'c>(&'c self, context_tag: ContextTag, textures: &[u32]) -> Result<Cookie<'c, Self, AreTexturesResidentReply>, ConnectionError>
    {
        are_textures_resident(self, context_tag, textures)
    }

    fn glx_delete_textures<'c>(&'c self, context_tag: ContextTag, textures: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        delete_textures(self, context_tag, textures)
    }

    fn glx_gen_textures(&self, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Self, GenTexturesReply>, ConnectionError>
    {
        gen_textures(self, context_tag, n)
    }

    fn glx_is_texture(&self, context_tag: ContextTag, texture: u32) -> Result<Cookie<'_, Self, IsTextureReply>, ConnectionError>
    {
        is_texture(self, context_tag, texture)
    }

    fn glx_get_color_table(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Self, GetColorTableReply>, ConnectionError>
    {
        get_color_table(self, context_tag, target, format, type_, swap_bytes)
    }

    fn glx_get_color_table_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetColorTableParameterfvReply>, ConnectionError>
    {
        get_color_table_parameterfv(self, context_tag, target, pname)
    }

    fn glx_get_color_table_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetColorTableParameterivReply>, ConnectionError>
    {
        get_color_table_parameteriv(self, context_tag, target, pname)
    }

    fn glx_get_convolution_filter(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Self, GetConvolutionFilterReply>, ConnectionError>
    {
        get_convolution_filter(self, context_tag, target, format, type_, swap_bytes)
    }

    fn glx_get_convolution_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetConvolutionParameterfvReply>, ConnectionError>
    {
        get_convolution_parameterfv(self, context_tag, target, pname)
    }

    fn glx_get_convolution_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetConvolutionParameterivReply>, ConnectionError>
    {
        get_convolution_parameteriv(self, context_tag, target, pname)
    }

    fn glx_get_separable_filter(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Self, GetSeparableFilterReply>, ConnectionError>
    {
        get_separable_filter(self, context_tag, target, format, type_, swap_bytes)
    }

    fn glx_get_histogram(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Self, GetHistogramReply>, ConnectionError>
    {
        get_histogram(self, context_tag, target, format, type_, swap_bytes, reset)
    }

    fn glx_get_histogram_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetHistogramParameterfvReply>, ConnectionError>
    {
        get_histogram_parameterfv(self, context_tag, target, pname)
    }

    fn glx_get_histogram_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetHistogramParameterivReply>, ConnectionError>
    {
        get_histogram_parameteriv(self, context_tag, target, pname)
    }

    fn glx_get_minmax(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Self, GetMinmaxReply>, ConnectionError>
    {
        get_minmax(self, context_tag, target, format, type_, swap_bytes, reset)
    }

    fn glx_get_minmax_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetMinmaxParameterfvReply>, ConnectionError>
    {
        get_minmax_parameterfv(self, context_tag, target, pname)
    }

    fn glx_get_minmax_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetMinmaxParameterivReply>, ConnectionError>
    {
        get_minmax_parameteriv(self, context_tag, target, pname)
    }

    fn glx_get_compressed_tex_image_arb(&self, context_tag: ContextTag, target: u32, level: i32) -> Result<Cookie<'_, Self, GetCompressedTexImageARBReply>, ConnectionError>
    {
        get_compressed_tex_image_arb(self, context_tag, target, level)
    }

    fn glx_delete_queries_arb<'c>(&'c self, context_tag: ContextTag, ids: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        delete_queries_arb(self, context_tag, ids)
    }

    fn glx_gen_queries_arb(&self, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Self, GenQueriesARBReply>, ConnectionError>
    {
        gen_queries_arb(self, context_tag, n)
    }

    fn glx_is_query_arb(&self, context_tag: ContextTag, id: u32) -> Result<Cookie<'_, Self, IsQueryARBReply>, ConnectionError>
    {
        is_query_arb(self, context_tag, id)
    }

    fn glx_get_queryiv_arb(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetQueryivARBReply>, ConnectionError>
    {
        get_queryiv_arb(self, context_tag, target, pname)
    }

    fn glx_get_query_objectiv_arb(&self, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Self, GetQueryObjectivARBReply>, ConnectionError>
    {
        get_query_objectiv_arb(self, context_tag, id, pname)
    }

    fn glx_get_query_objectuiv_arb(&self, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Self, GetQueryObjectuivARBReply>, ConnectionError>
    {
        get_query_objectuiv_arb(self, context_tag, id, pname)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
