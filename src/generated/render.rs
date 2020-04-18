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
#[allow(unused_imports)]
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;
#[allow(unused_imports)]
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "RENDER";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (0, 11);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PictType {
    Indexed = 0,
    Direct = 1,
}
impl From<PictType> for u8 {
    fn from(input: PictType) -> Self {
        match input {
            PictType::Indexed => 0,
            PictType::Direct => 1,
        }
    }
}
impl From<PictType> for Option<u8> {
    fn from(input: PictType) -> Self {
        Some(u8::from(input))
    }
}
impl From<PictType> for u16 {
    fn from(input: PictType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PictType> for Option<u16> {
    fn from(input: PictType) -> Self {
        Some(u16::from(input))
    }
}
impl From<PictType> for u32 {
    fn from(input: PictType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PictType> for Option<u32> {
    fn from(input: PictType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PictType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PictType::Indexed),
            1 => Ok(PictType::Direct),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PictType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PictType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PictureEnum {
    None = 0,
}
impl From<PictureEnum> for u8 {
    fn from(input: PictureEnum) -> Self {
        match input {
            PictureEnum::None => 0,
        }
    }
}
impl From<PictureEnum> for Option<u8> {
    fn from(input: PictureEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<PictureEnum> for u16 {
    fn from(input: PictureEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PictureEnum> for Option<u16> {
    fn from(input: PictureEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<PictureEnum> for u32 {
    fn from(input: PictureEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PictureEnum> for Option<u32> {
    fn from(input: PictureEnum) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PictureEnum {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PictureEnum::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PictureEnum {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PictureEnum {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PictOp {
    Clear = 0,
    Src = 1,
    Dst = 2,
    Over = 3,
    OverReverse = 4,
    In = 5,
    InReverse = 6,
    Out = 7,
    OutReverse = 8,
    Atop = 9,
    AtopReverse = 10,
    Xor = 11,
    Add = 12,
    Saturate = 13,
    DisjointClear = 16,
    DisjointSrc = 17,
    DisjointDst = 18,
    DisjointOver = 19,
    DisjointOverReverse = 20,
    DisjointIn = 21,
    DisjointInReverse = 22,
    DisjointOut = 23,
    DisjointOutReverse = 24,
    DisjointAtop = 25,
    DisjointAtopReverse = 26,
    DisjointXor = 27,
    ConjointClear = 32,
    ConjointSrc = 33,
    ConjointDst = 34,
    ConjointOver = 35,
    ConjointOverReverse = 36,
    ConjointIn = 37,
    ConjointInReverse = 38,
    ConjointOut = 39,
    ConjointOutReverse = 40,
    ConjointAtop = 41,
    ConjointAtopReverse = 42,
    ConjointXor = 43,
    Multiply = 48,
    Screen = 49,
    Overlay = 50,
    Darken = 51,
    Lighten = 52,
    ColorDodge = 53,
    ColorBurn = 54,
    HardLight = 55,
    SoftLight = 56,
    Difference = 57,
    Exclusion = 58,
    HSLHue = 59,
    HSLSaturation = 60,
    HSLColor = 61,
    HSLLuminosity = 62,
}
impl From<PictOp> for u8 {
    fn from(input: PictOp) -> Self {
        match input {
            PictOp::Clear => 0,
            PictOp::Src => 1,
            PictOp::Dst => 2,
            PictOp::Over => 3,
            PictOp::OverReverse => 4,
            PictOp::In => 5,
            PictOp::InReverse => 6,
            PictOp::Out => 7,
            PictOp::OutReverse => 8,
            PictOp::Atop => 9,
            PictOp::AtopReverse => 10,
            PictOp::Xor => 11,
            PictOp::Add => 12,
            PictOp::Saturate => 13,
            PictOp::DisjointClear => 16,
            PictOp::DisjointSrc => 17,
            PictOp::DisjointDst => 18,
            PictOp::DisjointOver => 19,
            PictOp::DisjointOverReverse => 20,
            PictOp::DisjointIn => 21,
            PictOp::DisjointInReverse => 22,
            PictOp::DisjointOut => 23,
            PictOp::DisjointOutReverse => 24,
            PictOp::DisjointAtop => 25,
            PictOp::DisjointAtopReverse => 26,
            PictOp::DisjointXor => 27,
            PictOp::ConjointClear => 32,
            PictOp::ConjointSrc => 33,
            PictOp::ConjointDst => 34,
            PictOp::ConjointOver => 35,
            PictOp::ConjointOverReverse => 36,
            PictOp::ConjointIn => 37,
            PictOp::ConjointInReverse => 38,
            PictOp::ConjointOut => 39,
            PictOp::ConjointOutReverse => 40,
            PictOp::ConjointAtop => 41,
            PictOp::ConjointAtopReverse => 42,
            PictOp::ConjointXor => 43,
            PictOp::Multiply => 48,
            PictOp::Screen => 49,
            PictOp::Overlay => 50,
            PictOp::Darken => 51,
            PictOp::Lighten => 52,
            PictOp::ColorDodge => 53,
            PictOp::ColorBurn => 54,
            PictOp::HardLight => 55,
            PictOp::SoftLight => 56,
            PictOp::Difference => 57,
            PictOp::Exclusion => 58,
            PictOp::HSLHue => 59,
            PictOp::HSLSaturation => 60,
            PictOp::HSLColor => 61,
            PictOp::HSLLuminosity => 62,
        }
    }
}
impl From<PictOp> for Option<u8> {
    fn from(input: PictOp) -> Self {
        Some(u8::from(input))
    }
}
impl From<PictOp> for u16 {
    fn from(input: PictOp) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PictOp> for Option<u16> {
    fn from(input: PictOp) -> Self {
        Some(u16::from(input))
    }
}
impl From<PictOp> for u32 {
    fn from(input: PictOp) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PictOp> for Option<u32> {
    fn from(input: PictOp) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PictOp {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PictOp::Clear),
            1 => Ok(PictOp::Src),
            2 => Ok(PictOp::Dst),
            3 => Ok(PictOp::Over),
            4 => Ok(PictOp::OverReverse),
            5 => Ok(PictOp::In),
            6 => Ok(PictOp::InReverse),
            7 => Ok(PictOp::Out),
            8 => Ok(PictOp::OutReverse),
            9 => Ok(PictOp::Atop),
            10 => Ok(PictOp::AtopReverse),
            11 => Ok(PictOp::Xor),
            12 => Ok(PictOp::Add),
            13 => Ok(PictOp::Saturate),
            16 => Ok(PictOp::DisjointClear),
            17 => Ok(PictOp::DisjointSrc),
            18 => Ok(PictOp::DisjointDst),
            19 => Ok(PictOp::DisjointOver),
            20 => Ok(PictOp::DisjointOverReverse),
            21 => Ok(PictOp::DisjointIn),
            22 => Ok(PictOp::DisjointInReverse),
            23 => Ok(PictOp::DisjointOut),
            24 => Ok(PictOp::DisjointOutReverse),
            25 => Ok(PictOp::DisjointAtop),
            26 => Ok(PictOp::DisjointAtopReverse),
            27 => Ok(PictOp::DisjointXor),
            32 => Ok(PictOp::ConjointClear),
            33 => Ok(PictOp::ConjointSrc),
            34 => Ok(PictOp::ConjointDst),
            35 => Ok(PictOp::ConjointOver),
            36 => Ok(PictOp::ConjointOverReverse),
            37 => Ok(PictOp::ConjointIn),
            38 => Ok(PictOp::ConjointInReverse),
            39 => Ok(PictOp::ConjointOut),
            40 => Ok(PictOp::ConjointOutReverse),
            41 => Ok(PictOp::ConjointAtop),
            42 => Ok(PictOp::ConjointAtopReverse),
            43 => Ok(PictOp::ConjointXor),
            48 => Ok(PictOp::Multiply),
            49 => Ok(PictOp::Screen),
            50 => Ok(PictOp::Overlay),
            51 => Ok(PictOp::Darken),
            52 => Ok(PictOp::Lighten),
            53 => Ok(PictOp::ColorDodge),
            54 => Ok(PictOp::ColorBurn),
            55 => Ok(PictOp::HardLight),
            56 => Ok(PictOp::SoftLight),
            57 => Ok(PictOp::Difference),
            58 => Ok(PictOp::Exclusion),
            59 => Ok(PictOp::HSLHue),
            60 => Ok(PictOp::HSLSaturation),
            61 => Ok(PictOp::HSLColor),
            62 => Ok(PictOp::HSLLuminosity),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PictOp {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PictOp {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PolyEdge {
    Sharp = 0,
    Smooth = 1,
}
impl From<PolyEdge> for u8 {
    fn from(input: PolyEdge) -> Self {
        match input {
            PolyEdge::Sharp => 0,
            PolyEdge::Smooth => 1,
        }
    }
}
impl From<PolyEdge> for Option<u8> {
    fn from(input: PolyEdge) -> Self {
        Some(u8::from(input))
    }
}
impl From<PolyEdge> for u16 {
    fn from(input: PolyEdge) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PolyEdge> for Option<u16> {
    fn from(input: PolyEdge) -> Self {
        Some(u16::from(input))
    }
}
impl From<PolyEdge> for u32 {
    fn from(input: PolyEdge) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PolyEdge> for Option<u32> {
    fn from(input: PolyEdge) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PolyEdge {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PolyEdge::Sharp),
            1 => Ok(PolyEdge::Smooth),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PolyEdge {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PolyEdge {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PolyMode {
    Precise = 0,
    Imprecise = 1,
}
impl From<PolyMode> for u8 {
    fn from(input: PolyMode) -> Self {
        match input {
            PolyMode::Precise => 0,
            PolyMode::Imprecise => 1,
        }
    }
}
impl From<PolyMode> for Option<u8> {
    fn from(input: PolyMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<PolyMode> for u16 {
    fn from(input: PolyMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PolyMode> for Option<u16> {
    fn from(input: PolyMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<PolyMode> for u32 {
    fn from(input: PolyMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PolyMode> for Option<u32> {
    fn from(input: PolyMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PolyMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PolyMode::Precise),
            1 => Ok(PolyMode::Imprecise),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PolyMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PolyMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CP {
    Repeat = 1 << 0,
    AlphaMap = 1 << 1,
    AlphaXOrigin = 1 << 2,
    AlphaYOrigin = 1 << 3,
    ClipXOrigin = 1 << 4,
    ClipYOrigin = 1 << 5,
    ClipMask = 1 << 6,
    GraphicsExposure = 1 << 7,
    SubwindowMode = 1 << 8,
    PolyEdge = 1 << 9,
    PolyMode = 1 << 10,
    Dither = 1 << 11,
    ComponentAlpha = 1 << 12,
}
impl From<CP> for u16 {
    fn from(input: CP) -> Self {
        match input {
            CP::Repeat => 1 << 0,
            CP::AlphaMap => 1 << 1,
            CP::AlphaXOrigin => 1 << 2,
            CP::AlphaYOrigin => 1 << 3,
            CP::ClipXOrigin => 1 << 4,
            CP::ClipYOrigin => 1 << 5,
            CP::ClipMask => 1 << 6,
            CP::GraphicsExposure => 1 << 7,
            CP::SubwindowMode => 1 << 8,
            CP::PolyEdge => 1 << 9,
            CP::PolyMode => 1 << 10,
            CP::Dither => 1 << 11,
            CP::ComponentAlpha => 1 << 12,
        }
    }
}
impl From<CP> for Option<u16> {
    fn from(input: CP) -> Self {
        Some(u16::from(input))
    }
}
impl From<CP> for u32 {
    fn from(input: CP) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<CP> for Option<u32> {
    fn from(input: CP) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for CP {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CP::Repeat),
            2 => Ok(CP::AlphaMap),
            4 => Ok(CP::AlphaXOrigin),
            8 => Ok(CP::AlphaYOrigin),
            16 => Ok(CP::ClipXOrigin),
            32 => Ok(CP::ClipYOrigin),
            64 => Ok(CP::ClipMask),
            128 => Ok(CP::GraphicsExposure),
            256 => Ok(CP::SubwindowMode),
            512 => Ok(CP::PolyEdge),
            1024 => Ok(CP::PolyMode),
            2048 => Ok(CP::Dither),
            4096 => Ok(CP::ComponentAlpha),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for CP {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(CP, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SubPixel {
    Unknown = 0,
    HorizontalRGB = 1,
    HorizontalBGR = 2,
    VerticalRGB = 3,
    VerticalBGR = 4,
    None = 5,
}
impl From<SubPixel> for u8 {
    fn from(input: SubPixel) -> Self {
        match input {
            SubPixel::Unknown => 0,
            SubPixel::HorizontalRGB => 1,
            SubPixel::HorizontalBGR => 2,
            SubPixel::VerticalRGB => 3,
            SubPixel::VerticalBGR => 4,
            SubPixel::None => 5,
        }
    }
}
impl From<SubPixel> for Option<u8> {
    fn from(input: SubPixel) -> Self {
        Some(u8::from(input))
    }
}
impl From<SubPixel> for u16 {
    fn from(input: SubPixel) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SubPixel> for Option<u16> {
    fn from(input: SubPixel) -> Self {
        Some(u16::from(input))
    }
}
impl From<SubPixel> for u32 {
    fn from(input: SubPixel) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SubPixel> for Option<u32> {
    fn from(input: SubPixel) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SubPixel {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SubPixel::Unknown),
            1 => Ok(SubPixel::HorizontalRGB),
            2 => Ok(SubPixel::HorizontalBGR),
            3 => Ok(SubPixel::VerticalRGB),
            4 => Ok(SubPixel::VerticalBGR),
            5 => Ok(SubPixel::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SubPixel {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SubPixel {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Repeat {
    None = 0,
    Normal = 1,
    Pad = 2,
    Reflect = 3,
}
impl From<Repeat> for u8 {
    fn from(input: Repeat) -> Self {
        match input {
            Repeat::None => 0,
            Repeat::Normal => 1,
            Repeat::Pad => 2,
            Repeat::Reflect => 3,
        }
    }
}
impl From<Repeat> for Option<u8> {
    fn from(input: Repeat) -> Self {
        Some(u8::from(input))
    }
}
impl From<Repeat> for u16 {
    fn from(input: Repeat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Repeat> for Option<u16> {
    fn from(input: Repeat) -> Self {
        Some(u16::from(input))
    }
}
impl From<Repeat> for u32 {
    fn from(input: Repeat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Repeat> for Option<u32> {
    fn from(input: Repeat) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Repeat {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Repeat::None),
            1 => Ok(Repeat::Normal),
            2 => Ok(Repeat::Pad),
            3 => Ok(Repeat::Reflect),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Repeat {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Repeat {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

pub type Glyph = u32;

pub type Glyphset = u32;

pub type Picture = u32;

pub type Pictformat = u32;

pub type Fixed = i32;

/// Opcode for the PictFormat error
pub const PICT_FORMAT_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PictFormatError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl PictFormatError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = PictFormatError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PictFormatError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for PictFormatError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for PictFormatError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&PictFormatError> for [u8; 32] {
    fn from(input: &PictFormatError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<PictFormatError> for [u8; 32] {
    fn from(input: PictFormatError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Picture error
pub const PICTURE_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PictureError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl PictureError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = PictureError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PictureError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for PictureError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for PictureError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&PictureError> for [u8; 32] {
    fn from(input: &PictureError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<PictureError> for [u8; 32] {
    fn from(input: PictureError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the PictOp error
pub const PICT_OP_ERROR: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PictOpError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl PictOpError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = PictOpError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PictOpError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for PictOpError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for PictOpError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&PictOpError> for [u8; 32] {
    fn from(input: &PictOpError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<PictOpError> for [u8; 32] {
    fn from(input: PictOpError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the GlyphSet error
pub const GLYPH_SET_ERROR: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphSetError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl GlyphSetError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = GlyphSetError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GlyphSetError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for GlyphSetError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for GlyphSetError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&GlyphSetError> for [u8; 32] {
    fn from(input: &GlyphSetError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<GlyphSetError> for [u8; 32] {
    fn from(input: GlyphSetError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Glyph error
pub const GLYPH_ERROR: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl GlyphError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = GlyphError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GlyphError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for GlyphError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for GlyphError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&GlyphError> for [u8; 32] {
    fn from(input: &GlyphError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<GlyphError> for [u8; 32] {
    fn from(input: GlyphError) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Directformat {
    pub red_shift: u16,
    pub red_mask: u16,
    pub green_shift: u16,
    pub green_mask: u16,
    pub blue_shift: u16,
    pub blue_mask: u16,
    pub alpha_shift: u16,
    pub alpha_mask: u16,
}
impl TryParse for Directformat {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (red_shift, remaining) = u16::try_parse(remaining)?;
        let (red_mask, remaining) = u16::try_parse(remaining)?;
        let (green_shift, remaining) = u16::try_parse(remaining)?;
        let (green_mask, remaining) = u16::try_parse(remaining)?;
        let (blue_shift, remaining) = u16::try_parse(remaining)?;
        let (blue_mask, remaining) = u16::try_parse(remaining)?;
        let (alpha_shift, remaining) = u16::try_parse(remaining)?;
        let (alpha_mask, remaining) = u16::try_parse(remaining)?;
        let result = Directformat { red_shift, red_mask, green_shift, green_mask, blue_shift, blue_mask, alpha_shift, alpha_mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Directformat {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Directformat {
    type Bytes = [u8; 16];
    fn serialize(&self) -> Self::Bytes {
        let red_shift_bytes = self.red_shift.serialize();
        let red_mask_bytes = self.red_mask.serialize();
        let green_shift_bytes = self.green_shift.serialize();
        let green_mask_bytes = self.green_mask.serialize();
        let blue_shift_bytes = self.blue_shift.serialize();
        let blue_mask_bytes = self.blue_mask.serialize();
        let alpha_shift_bytes = self.alpha_shift.serialize();
        let alpha_mask_bytes = self.alpha_mask.serialize();
        [
            red_shift_bytes[0],
            red_shift_bytes[1],
            red_mask_bytes[0],
            red_mask_bytes[1],
            green_shift_bytes[0],
            green_shift_bytes[1],
            green_mask_bytes[0],
            green_mask_bytes[1],
            blue_shift_bytes[0],
            blue_shift_bytes[1],
            blue_mask_bytes[0],
            blue_mask_bytes[1],
            alpha_shift_bytes[0],
            alpha_shift_bytes[1],
            alpha_mask_bytes[0],
            alpha_mask_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.red_shift.serialize_into(bytes);
        self.red_mask.serialize_into(bytes);
        self.green_shift.serialize_into(bytes);
        self.green_mask.serialize_into(bytes);
        self.blue_shift.serialize_into(bytes);
        self.blue_mask.serialize_into(bytes);
        self.alpha_shift.serialize_into(bytes);
        self.alpha_mask.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pictforminfo {
    pub id: Pictformat,
    pub type_: PictType,
    pub depth: u8,
    pub direct: Directformat,
    pub colormap: xproto::Colormap,
}
impl TryParse for Pictforminfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (id, remaining) = Pictformat::try_parse(remaining)?;
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (direct, remaining) = Directformat::try_parse(remaining)?;
        let (colormap, remaining) = xproto::Colormap::try_parse(remaining)?;
        let type_ = type_.try_into()?;
        let result = Pictforminfo { id, type_, depth, direct, colormap };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Pictforminfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Pictforminfo {
    type Bytes = [u8; 28];
    fn serialize(&self) -> Self::Bytes {
        let id_bytes = self.id.serialize();
        let type_bytes = u8::from(self.type_).serialize();
        let depth_bytes = self.depth.serialize();
        let direct_bytes = self.direct.serialize();
        let colormap_bytes = self.colormap.serialize();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            type_bytes[0],
            depth_bytes[0],
            0,
            0,
            direct_bytes[0],
            direct_bytes[1],
            direct_bytes[2],
            direct_bytes[3],
            direct_bytes[4],
            direct_bytes[5],
            direct_bytes[6],
            direct_bytes[7],
            direct_bytes[8],
            direct_bytes[9],
            direct_bytes[10],
            direct_bytes[11],
            direct_bytes[12],
            direct_bytes[13],
            direct_bytes[14],
            direct_bytes[15],
            colormap_bytes[0],
            colormap_bytes[1],
            colormap_bytes[2],
            colormap_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(28);
        self.id.serialize_into(bytes);
        u8::from(self.type_).serialize_into(bytes);
        self.depth.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.direct.serialize_into(bytes);
        self.colormap.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pictvisual {
    pub visual: xproto::Visualid,
    pub format: Pictformat,
}
impl TryParse for Pictvisual {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (visual, remaining) = xproto::Visualid::try_parse(remaining)?;
        let (format, remaining) = Pictformat::try_parse(remaining)?;
        let result = Pictvisual { visual, format };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Pictvisual {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Pictvisual {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let visual_bytes = self.visual.serialize();
        let format_bytes = self.format.serialize();
        [
            visual_bytes[0],
            visual_bytes[1],
            visual_bytes[2],
            visual_bytes[3],
            format_bytes[0],
            format_bytes[1],
            format_bytes[2],
            format_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.visual.serialize_into(bytes);
        self.format.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pictdepth {
    pub depth: u8,
    pub visuals: Vec<Pictvisual>,
}
impl TryParse for Pictdepth {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (depth, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (num_visuals, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (visuals, remaining) = crate::x11_utils::parse_list::<Pictvisual>(remaining, num_visuals as usize)?;
        let result = Pictdepth { depth, visuals };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Pictdepth {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Pictdepth {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.depth.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        let num_visuals = self.visuals.len() as u16;
        num_visuals.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.visuals.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pictscreen {
    pub fallback: Pictformat,
    pub depths: Vec<Pictdepth>,
}
impl TryParse for Pictscreen {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (num_depths, remaining) = u32::try_parse(remaining)?;
        let (fallback, remaining) = Pictformat::try_parse(remaining)?;
        let (depths, remaining) = crate::x11_utils::parse_list::<Pictdepth>(remaining, num_depths as usize)?;
        let result = Pictscreen { fallback, depths };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Pictscreen {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Pictscreen {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        let num_depths = self.depths.len() as u32;
        num_depths.serialize_into(bytes);
        self.fallback.serialize_into(bytes);
        self.depths.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Indexvalue {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}
impl TryParse for Indexvalue {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (pixel, remaining) = u32::try_parse(remaining)?;
        let (red, remaining) = u16::try_parse(remaining)?;
        let (green, remaining) = u16::try_parse(remaining)?;
        let (blue, remaining) = u16::try_parse(remaining)?;
        let (alpha, remaining) = u16::try_parse(remaining)?;
        let result = Indexvalue { pixel, red, green, blue, alpha };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Indexvalue {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Indexvalue {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let pixel_bytes = self.pixel.serialize();
        let red_bytes = self.red.serialize();
        let green_bytes = self.green.serialize();
        let blue_bytes = self.blue.serialize();
        let alpha_bytes = self.alpha.serialize();
        [
            pixel_bytes[0],
            pixel_bytes[1],
            pixel_bytes[2],
            pixel_bytes[3],
            red_bytes[0],
            red_bytes[1],
            green_bytes[0],
            green_bytes[1],
            blue_bytes[0],
            blue_bytes[1],
            alpha_bytes[0],
            alpha_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.pixel.serialize_into(bytes);
        self.red.serialize_into(bytes);
        self.green.serialize_into(bytes);
        self.blue.serialize_into(bytes);
        self.alpha.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}
impl TryParse for Color {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (red, remaining) = u16::try_parse(remaining)?;
        let (green, remaining) = u16::try_parse(remaining)?;
        let (blue, remaining) = u16::try_parse(remaining)?;
        let (alpha, remaining) = u16::try_parse(remaining)?;
        let result = Color { red, green, blue, alpha };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Color {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Color {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let red_bytes = self.red.serialize();
        let green_bytes = self.green.serialize();
        let blue_bytes = self.blue.serialize();
        let alpha_bytes = self.alpha.serialize();
        [
            red_bytes[0],
            red_bytes[1],
            green_bytes[0],
            green_bytes[1],
            blue_bytes[0],
            blue_bytes[1],
            alpha_bytes[0],
            alpha_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.red.serialize_into(bytes);
        self.green.serialize_into(bytes);
        self.blue.serialize_into(bytes);
        self.alpha.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointfix {
    pub x: Fixed,
    pub y: Fixed,
}
impl TryParse for Pointfix {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (x, remaining) = Fixed::try_parse(remaining)?;
        let (y, remaining) = Fixed::try_parse(remaining)?;
        let result = Pointfix { x, y };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Pointfix {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Pointfix {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        [
            x_bytes[0],
            x_bytes[1],
            x_bytes[2],
            x_bytes[3],
            y_bytes[0],
            y_bytes[1],
            y_bytes[2],
            y_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Linefix {
    pub p1: Pointfix,
    pub p2: Pointfix,
}
impl TryParse for Linefix {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (p1, remaining) = Pointfix::try_parse(remaining)?;
        let (p2, remaining) = Pointfix::try_parse(remaining)?;
        let result = Linefix { p1, p2 };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Linefix {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Linefix {
    type Bytes = [u8; 16];
    fn serialize(&self) -> Self::Bytes {
        let p1_bytes = self.p1.serialize();
        let p2_bytes = self.p2.serialize();
        [
            p1_bytes[0],
            p1_bytes[1],
            p1_bytes[2],
            p1_bytes[3],
            p1_bytes[4],
            p1_bytes[5],
            p1_bytes[6],
            p1_bytes[7],
            p2_bytes[0],
            p2_bytes[1],
            p2_bytes[2],
            p2_bytes[3],
            p2_bytes[4],
            p2_bytes[5],
            p2_bytes[6],
            p2_bytes[7],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.p1.serialize_into(bytes);
        self.p2.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Triangle {
    pub p1: Pointfix,
    pub p2: Pointfix,
    pub p3: Pointfix,
}
impl TryParse for Triangle {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (p1, remaining) = Pointfix::try_parse(remaining)?;
        let (p2, remaining) = Pointfix::try_parse(remaining)?;
        let (p3, remaining) = Pointfix::try_parse(remaining)?;
        let result = Triangle { p1, p2, p3 };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Triangle {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Triangle {
    type Bytes = [u8; 24];
    fn serialize(&self) -> Self::Bytes {
        let p1_bytes = self.p1.serialize();
        let p2_bytes = self.p2.serialize();
        let p3_bytes = self.p3.serialize();
        [
            p1_bytes[0],
            p1_bytes[1],
            p1_bytes[2],
            p1_bytes[3],
            p1_bytes[4],
            p1_bytes[5],
            p1_bytes[6],
            p1_bytes[7],
            p2_bytes[0],
            p2_bytes[1],
            p2_bytes[2],
            p2_bytes[3],
            p2_bytes[4],
            p2_bytes[5],
            p2_bytes[6],
            p2_bytes[7],
            p3_bytes[0],
            p3_bytes[1],
            p3_bytes[2],
            p3_bytes[3],
            p3_bytes[4],
            p3_bytes[5],
            p3_bytes[6],
            p3_bytes[7],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.p1.serialize_into(bytes);
        self.p2.serialize_into(bytes);
        self.p3.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trapezoid {
    pub top: Fixed,
    pub bottom: Fixed,
    pub left: Linefix,
    pub right: Linefix,
}
impl TryParse for Trapezoid {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (top, remaining) = Fixed::try_parse(remaining)?;
        let (bottom, remaining) = Fixed::try_parse(remaining)?;
        let (left, remaining) = Linefix::try_parse(remaining)?;
        let (right, remaining) = Linefix::try_parse(remaining)?;
        let result = Trapezoid { top, bottom, left, right };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Trapezoid {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Trapezoid {
    type Bytes = [u8; 40];
    fn serialize(&self) -> Self::Bytes {
        let top_bytes = self.top.serialize();
        let bottom_bytes = self.bottom.serialize();
        let left_bytes = self.left.serialize();
        let right_bytes = self.right.serialize();
        [
            top_bytes[0],
            top_bytes[1],
            top_bytes[2],
            top_bytes[3],
            bottom_bytes[0],
            bottom_bytes[1],
            bottom_bytes[2],
            bottom_bytes[3],
            left_bytes[0],
            left_bytes[1],
            left_bytes[2],
            left_bytes[3],
            left_bytes[4],
            left_bytes[5],
            left_bytes[6],
            left_bytes[7],
            left_bytes[8],
            left_bytes[9],
            left_bytes[10],
            left_bytes[11],
            left_bytes[12],
            left_bytes[13],
            left_bytes[14],
            left_bytes[15],
            right_bytes[0],
            right_bytes[1],
            right_bytes[2],
            right_bytes[3],
            right_bytes[4],
            right_bytes[5],
            right_bytes[6],
            right_bytes[7],
            right_bytes[8],
            right_bytes[9],
            right_bytes[10],
            right_bytes[11],
            right_bytes[12],
            right_bytes[13],
            right_bytes[14],
            right_bytes[15],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(40);
        self.top.serialize_into(bytes);
        self.bottom.serialize_into(bytes);
        self.left.serialize_into(bytes);
        self.right.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Glyphinfo {
    pub width: u16,
    pub height: u16,
    pub x: i16,
    pub y: i16,
    pub x_off: i16,
    pub y_off: i16,
}
impl TryParse for Glyphinfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (x_off, remaining) = i16::try_parse(remaining)?;
        let (y_off, remaining) = i16::try_parse(remaining)?;
        let result = Glyphinfo { width, height, x, y, x_off, y_off };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Glyphinfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Glyphinfo {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let x_off_bytes = self.x_off.serialize();
        let y_off_bytes = self.y_off.serialize();
        [
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            x_off_bytes[0],
            x_off_bytes[1],
            y_off_bytes[0],
            y_off_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
        self.x_off.serialize_into(bytes);
        self.y_off.serialize_into(bytes);
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let client_major_version_bytes = client_major_version.serialize();
    let client_minor_version_bytes = client_minor_version.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        0,
        0,
        client_major_version_bytes[0],
        client_major_version_bytes[1],
        client_major_version_bytes[2],
        client_major_version_bytes[3],
        client_minor_version_bytes[0],
        client_minor_version_bytes[1],
        client_minor_version_bytes[2],
        client_minor_version_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
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

/// Opcode for the QueryPictFormats request
pub const QUERY_PICT_FORMATS_REQUEST: u8 = 1;
pub fn query_pict_formats<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryPictFormatsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let mut request0 = [
        extension_information.major_opcode,
        QUERY_PICT_FORMATS_REQUEST,
        0,
        0,
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryPictFormatsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_depths: u32,
    pub num_visuals: u32,
    pub formats: Vec<Pictforminfo>,
    pub screens: Vec<Pictscreen>,
    pub subpixels: Vec<u32>,
}
impl QueryPictFormatsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_formats, remaining) = u32::try_parse(remaining)?;
        let (num_screens, remaining) = u32::try_parse(remaining)?;
        let (num_depths, remaining) = u32::try_parse(remaining)?;
        let (num_visuals, remaining) = u32::try_parse(remaining)?;
        let (num_subpixel, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (formats, remaining) = crate::x11_utils::parse_list::<Pictforminfo>(remaining, num_formats as usize)?;
        let (screens, remaining) = crate::x11_utils::parse_list::<Pictscreen>(remaining, num_screens as usize)?;
        let (subpixels, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_subpixel as usize)?;
        let result = QueryPictFormatsReply { response_type, sequence, length, num_depths, num_visuals, formats, screens, subpixels };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryPictFormatsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryPictIndexValues request
pub const QUERY_PICT_INDEX_VALUES_REQUEST: u8 = 2;
pub fn query_pict_index_values<Conn>(conn: &Conn, format: Pictformat) -> Result<Cookie<'_, Conn, QueryPictIndexValuesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let format_bytes = format.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        QUERY_PICT_INDEX_VALUES_REQUEST,
        0,
        0,
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryPictIndexValuesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub values: Vec<Indexvalue>,
}
impl QueryPictIndexValuesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_values, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (values, remaining) = crate::x11_utils::parse_list::<Indexvalue>(remaining, num_values as usize)?;
        let result = QueryPictIndexValuesReply { response_type, sequence, length, values };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryPictIndexValuesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreatePicture request
pub const CREATE_PICTURE_REQUEST: u8 = 4;
/// Auxiliary and optional information for the create_picture function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CreatePictureAux {
    pub repeat: Option<u32>,
    pub alphamap: Option<Picture>,
    pub alphaxorigin: Option<i32>,
    pub alphayorigin: Option<i32>,
    pub clipxorigin: Option<i32>,
    pub clipyorigin: Option<i32>,
    pub clipmask: Option<xproto::Pixmap>,
    pub graphicsexposure: Option<u32>,
    pub subwindowmode: Option<u32>,
    pub polyedge: Option<u32>,
    pub polymode: Option<u32>,
    pub dither: Option<xproto::Atom>,
    pub componentalpha: Option<u32>,
}
impl CreatePictureAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.repeat.is_some() {
            mask |= u32::from(CP::Repeat);
        }
        if self.alphamap.is_some() {
            mask |= u32::from(CP::AlphaMap);
        }
        if self.alphaxorigin.is_some() {
            mask |= u32::from(CP::AlphaXOrigin);
        }
        if self.alphayorigin.is_some() {
            mask |= u32::from(CP::AlphaYOrigin);
        }
        if self.clipxorigin.is_some() {
            mask |= u32::from(CP::ClipXOrigin);
        }
        if self.clipyorigin.is_some() {
            mask |= u32::from(CP::ClipYOrigin);
        }
        if self.clipmask.is_some() {
            mask |= u32::from(CP::ClipMask);
        }
        if self.graphicsexposure.is_some() {
            mask |= u32::from(CP::GraphicsExposure);
        }
        if self.subwindowmode.is_some() {
            mask |= u32::from(CP::SubwindowMode);
        }
        if self.polyedge.is_some() {
            mask |= u32::from(CP::PolyEdge);
        }
        if self.polymode.is_some() {
            mask |= u32::from(CP::PolyMode);
        }
        if self.dither.is_some() {
            mask |= u32::from(CP::Dither);
        }
        if self.componentalpha.is_some() {
            mask |= u32::from(CP::ComponentAlpha);
        }
        mask
    }
    /// Set the repeat field of this structure.
    pub fn repeat<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.repeat = value.into();
        self
    }
    /// Set the alphamap field of this structure.
    pub fn alphamap<I>(mut self, value: I) -> Self where I: Into<Option<Picture>> {
        self.alphamap = value.into();
        self
    }
    /// Set the alphaxorigin field of this structure.
    pub fn alphaxorigin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.alphaxorigin = value.into();
        self
    }
    /// Set the alphayorigin field of this structure.
    pub fn alphayorigin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.alphayorigin = value.into();
        self
    }
    /// Set the clipxorigin field of this structure.
    pub fn clipxorigin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.clipxorigin = value.into();
        self
    }
    /// Set the clipyorigin field of this structure.
    pub fn clipyorigin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.clipyorigin = value.into();
        self
    }
    /// Set the clipmask field of this structure.
    pub fn clipmask<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Pixmap>> {
        self.clipmask = value.into();
        self
    }
    /// Set the graphicsexposure field of this structure.
    pub fn graphicsexposure<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.graphicsexposure = value.into();
        self
    }
    /// Set the subwindowmode field of this structure.
    pub fn subwindowmode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.subwindowmode = value.into();
        self
    }
    /// Set the polyedge field of this structure.
    pub fn polyedge<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.polyedge = value.into();
        self
    }
    /// Set the polymode field of this structure.
    pub fn polymode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.polymode = value.into();
        self
    }
    /// Set the dither field of this structure.
    pub fn dither<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Atom>> {
        self.dither = value.into();
        self
    }
    /// Set the componentalpha field of this structure.
    pub fn componentalpha<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.componentalpha = value.into();
        self
    }
}
impl Serialize for CreatePictureAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.repeat {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.alphamap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.alphaxorigin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.alphayorigin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clipxorigin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clipyorigin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clipmask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.graphicsexposure {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.subwindowmode {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.polyedge {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.polymode {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.dither {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.componentalpha {
            value.serialize_into(bytes);
        }
    }
}
pub fn create_picture<'c, Conn>(conn: &'c Conn, pid: Picture, drawable: xproto::Drawable, format: Pictformat, value_list: &CreatePictureAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length_so_far = 0;
    let pid_bytes = pid.serialize();
    let drawable_bytes = drawable.serialize();
    let format_bytes = format.serialize();
    let value_mask_bytes = value_mask.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_PICTURE_REQUEST,
        0,
        0,
        pid_bytes[0],
        pid_bytes[1],
        pid_bytes[2],
        pid_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the ChangePicture request
pub const CHANGE_PICTURE_REQUEST: u8 = 5;
/// Auxiliary and optional information for the change_picture function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChangePictureAux {
    pub repeat: Option<u32>,
    pub alphamap: Option<Picture>,
    pub alphaxorigin: Option<i32>,
    pub alphayorigin: Option<i32>,
    pub clipxorigin: Option<i32>,
    pub clipyorigin: Option<i32>,
    pub clipmask: Option<xproto::Pixmap>,
    pub graphicsexposure: Option<u32>,
    pub subwindowmode: Option<u32>,
    pub polyedge: Option<u32>,
    pub polymode: Option<u32>,
    pub dither: Option<xproto::Atom>,
    pub componentalpha: Option<u32>,
}
impl ChangePictureAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.repeat.is_some() {
            mask |= u32::from(CP::Repeat);
        }
        if self.alphamap.is_some() {
            mask |= u32::from(CP::AlphaMap);
        }
        if self.alphaxorigin.is_some() {
            mask |= u32::from(CP::AlphaXOrigin);
        }
        if self.alphayorigin.is_some() {
            mask |= u32::from(CP::AlphaYOrigin);
        }
        if self.clipxorigin.is_some() {
            mask |= u32::from(CP::ClipXOrigin);
        }
        if self.clipyorigin.is_some() {
            mask |= u32::from(CP::ClipYOrigin);
        }
        if self.clipmask.is_some() {
            mask |= u32::from(CP::ClipMask);
        }
        if self.graphicsexposure.is_some() {
            mask |= u32::from(CP::GraphicsExposure);
        }
        if self.subwindowmode.is_some() {
            mask |= u32::from(CP::SubwindowMode);
        }
        if self.polyedge.is_some() {
            mask |= u32::from(CP::PolyEdge);
        }
        if self.polymode.is_some() {
            mask |= u32::from(CP::PolyMode);
        }
        if self.dither.is_some() {
            mask |= u32::from(CP::Dither);
        }
        if self.componentalpha.is_some() {
            mask |= u32::from(CP::ComponentAlpha);
        }
        mask
    }
    /// Set the repeat field of this structure.
    pub fn repeat<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.repeat = value.into();
        self
    }
    /// Set the alphamap field of this structure.
    pub fn alphamap<I>(mut self, value: I) -> Self where I: Into<Option<Picture>> {
        self.alphamap = value.into();
        self
    }
    /// Set the alphaxorigin field of this structure.
    pub fn alphaxorigin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.alphaxorigin = value.into();
        self
    }
    /// Set the alphayorigin field of this structure.
    pub fn alphayorigin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.alphayorigin = value.into();
        self
    }
    /// Set the clipxorigin field of this structure.
    pub fn clipxorigin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.clipxorigin = value.into();
        self
    }
    /// Set the clipyorigin field of this structure.
    pub fn clipyorigin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.clipyorigin = value.into();
        self
    }
    /// Set the clipmask field of this structure.
    pub fn clipmask<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Pixmap>> {
        self.clipmask = value.into();
        self
    }
    /// Set the graphicsexposure field of this structure.
    pub fn graphicsexposure<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.graphicsexposure = value.into();
        self
    }
    /// Set the subwindowmode field of this structure.
    pub fn subwindowmode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.subwindowmode = value.into();
        self
    }
    /// Set the polyedge field of this structure.
    pub fn polyedge<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.polyedge = value.into();
        self
    }
    /// Set the polymode field of this structure.
    pub fn polymode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.polymode = value.into();
        self
    }
    /// Set the dither field of this structure.
    pub fn dither<I>(mut self, value: I) -> Self where I: Into<Option<xproto::Atom>> {
        self.dither = value.into();
        self
    }
    /// Set the componentalpha field of this structure.
    pub fn componentalpha<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.componentalpha = value.into();
        self
    }
}
impl Serialize for ChangePictureAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.repeat {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.alphamap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.alphaxorigin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.alphayorigin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clipxorigin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clipyorigin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clipmask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.graphicsexposure {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.subwindowmode {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.polyedge {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.polymode {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.dither {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.componentalpha {
            value.serialize_into(bytes);
        }
    }
}
pub fn change_picture<'c, Conn>(conn: &'c Conn, picture: Picture, value_list: &ChangePictureAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let value_mask_bytes = value_mask.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CHANGE_PICTURE_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the SetPictureClipRectangles request
pub const SET_PICTURE_CLIP_RECTANGLES_REQUEST: u8 = 6;
pub fn set_picture_clip_rectangles<'c, Conn>(conn: &'c Conn, picture: Picture, clip_x_origin: i16, clip_y_origin: i16, rectangles: &[xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let clip_x_origin_bytes = clip_x_origin.serialize();
    let clip_y_origin_bytes = clip_y_origin.serialize();
    let rectangles_bytes = rectangles.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_PICTURE_CLIP_RECTANGLES_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        clip_x_origin_bytes[0],
        clip_x_origin_bytes[1],
        clip_y_origin_bytes[0],
        clip_y_origin_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&rectangles_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&rectangles_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the FreePicture request
pub const FREE_PICTURE_REQUEST: u8 = 7;
pub fn free_picture<Conn>(conn: &Conn, picture: Picture) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        FREE_PICTURE_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the Composite request
pub const COMPOSITE_REQUEST: u8 = 8;
pub fn composite<Conn, A>(conn: &Conn, op: A, src: Picture, mask: Picture, dst: Picture, src_x: i16, src_y: i16, mask_x: i16, mask_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let src_bytes = src.serialize();
    let mask_bytes = mask.serialize();
    let dst_bytes = dst.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let mask_x_bytes = mask_x.serialize();
    let mask_y_bytes = mask_y.serialize();
    let dst_x_bytes = dst_x.serialize();
    let dst_y_bytes = dst_y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        COMPOSITE_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        mask_bytes[0],
        mask_bytes[1],
        mask_bytes[2],
        mask_bytes[3],
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
        mask_x_bytes[0],
        mask_x_bytes[1],
        mask_y_bytes[0],
        mask_y_bytes[1],
        dst_x_bytes[0],
        dst_x_bytes[1],
        dst_y_bytes[0],
        dst_y_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the Trapezoids request
pub const TRAPEZOIDS_REQUEST: u8 = 10;
pub fn trapezoids<'c, Conn, A>(conn: &'c Conn, op: A, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, traps: &[Trapezoid]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let src_bytes = src.serialize();
    let dst_bytes = dst.serialize();
    let mask_format_bytes = mask_format.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let traps_bytes = traps.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        TRAPEZOIDS_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        mask_format_bytes[0],
        mask_format_bytes[1],
        mask_format_bytes[2],
        mask_format_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&traps_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&traps_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the Triangles request
pub const TRIANGLES_REQUEST: u8 = 11;
pub fn triangles<'c, Conn, A>(conn: &'c Conn, op: A, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, triangles: &[Triangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let src_bytes = src.serialize();
    let dst_bytes = dst.serialize();
    let mask_format_bytes = mask_format.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let triangles_bytes = triangles.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        TRIANGLES_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        mask_format_bytes[0],
        mask_format_bytes[1],
        mask_format_bytes[2],
        mask_format_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&triangles_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&triangles_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the TriStrip request
pub const TRI_STRIP_REQUEST: u8 = 12;
pub fn tri_strip<'c, Conn, A>(conn: &'c Conn, op: A, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, points: &[Pointfix]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let src_bytes = src.serialize();
    let dst_bytes = dst.serialize();
    let mask_format_bytes = mask_format.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let points_bytes = points.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        TRI_STRIP_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        mask_format_bytes[0],
        mask_format_bytes[1],
        mask_format_bytes[2],
        mask_format_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&points_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&points_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the TriFan request
pub const TRI_FAN_REQUEST: u8 = 13;
pub fn tri_fan<'c, Conn, A>(conn: &'c Conn, op: A, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, points: &[Pointfix]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let src_bytes = src.serialize();
    let dst_bytes = dst.serialize();
    let mask_format_bytes = mask_format.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let points_bytes = points.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        TRI_FAN_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        mask_format_bytes[0],
        mask_format_bytes[1],
        mask_format_bytes[2],
        mask_format_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&points_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&points_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the CreateGlyphSet request
pub const CREATE_GLYPH_SET_REQUEST: u8 = 17;
pub fn create_glyph_set<Conn>(conn: &Conn, gsid: Glyphset, format: Pictformat) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let gsid_bytes = gsid.serialize();
    let format_bytes = format.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_GLYPH_SET_REQUEST,
        0,
        0,
        gsid_bytes[0],
        gsid_bytes[1],
        gsid_bytes[2],
        gsid_bytes[3],
        format_bytes[0],
        format_bytes[1],
        format_bytes[2],
        format_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the ReferenceGlyphSet request
pub const REFERENCE_GLYPH_SET_REQUEST: u8 = 18;
pub fn reference_glyph_set<Conn>(conn: &Conn, gsid: Glyphset, existing: Glyphset) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let gsid_bytes = gsid.serialize();
    let existing_bytes = existing.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        REFERENCE_GLYPH_SET_REQUEST,
        0,
        0,
        gsid_bytes[0],
        gsid_bytes[1],
        gsid_bytes[2],
        gsid_bytes[3],
        existing_bytes[0],
        existing_bytes[1],
        existing_bytes[2],
        existing_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the FreeGlyphSet request
pub const FREE_GLYPH_SET_REQUEST: u8 = 19;
pub fn free_glyph_set<Conn>(conn: &Conn, glyphset: Glyphset) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let glyphset_bytes = glyphset.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        FREE_GLYPH_SET_REQUEST,
        0,
        0,
        glyphset_bytes[0],
        glyphset_bytes[1],
        glyphset_bytes[2],
        glyphset_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the AddGlyphs request
pub const ADD_GLYPHS_REQUEST: u8 = 20;
pub fn add_glyphs<'c, Conn>(conn: &'c Conn, glyphset: Glyphset, glyphs_len: u32, glyphids: &[u32], glyphs: &[Glyphinfo], data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let glyphset_bytes = glyphset.serialize();
    let glyphs_len_bytes = glyphs_len.serialize();
    assert_eq!(glyphids.len(), glyphs_len as usize, "Argument glyphids has an incorrect length");
    let glyphids_bytes = glyphids.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        ADD_GLYPHS_REQUEST,
        0,
        0,
        glyphset_bytes[0],
        glyphset_bytes[1],
        glyphset_bytes[2],
        glyphset_bytes[3],
        glyphs_len_bytes[0],
        glyphs_len_bytes[1],
        glyphs_len_bytes[2],
        glyphs_len_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&glyphids_bytes).len();
    assert_eq!(glyphs.len(), glyphs_len as usize, "Argument glyphs has an incorrect length");
    let glyphs_bytes = glyphs.serialize();
    let length_so_far = length_so_far + (&glyphs_bytes).len();
    let length_so_far = length_so_far + (data).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&glyphids_bytes), IoSlice::new(&glyphs_bytes), IoSlice::new(data), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the FreeGlyphs request
pub const FREE_GLYPHS_REQUEST: u8 = 22;
pub fn free_glyphs<'c, Conn>(conn: &'c Conn, glyphset: Glyphset, glyphs: &[Glyph]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let glyphset_bytes = glyphset.serialize();
    let glyphs_bytes = glyphs.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        FREE_GLYPHS_REQUEST,
        0,
        0,
        glyphset_bytes[0],
        glyphset_bytes[1],
        glyphset_bytes[2],
        glyphset_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&glyphs_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&glyphs_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the CompositeGlyphs8 request
pub const COMPOSITE_GLYPHS8_REQUEST: u8 = 23;
pub fn composite_glyphs8<'c, Conn, A>(conn: &'c Conn, op: A, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let src_bytes = src.serialize();
    let dst_bytes = dst.serialize();
    let mask_format_bytes = mask_format.serialize();
    let glyphset_bytes = glyphset.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        COMPOSITE_GLYPHS8_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        mask_format_bytes[0],
        mask_format_bytes[1],
        mask_format_bytes[2],
        mask_format_bytes[3],
        glyphset_bytes[0],
        glyphset_bytes[1],
        glyphset_bytes[2],
        glyphset_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (glyphcmds).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(glyphcmds), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the CompositeGlyphs16 request
pub const COMPOSITE_GLYPHS16_REQUEST: u8 = 24;
pub fn composite_glyphs16<'c, Conn, A>(conn: &'c Conn, op: A, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let src_bytes = src.serialize();
    let dst_bytes = dst.serialize();
    let mask_format_bytes = mask_format.serialize();
    let glyphset_bytes = glyphset.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        COMPOSITE_GLYPHS16_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        mask_format_bytes[0],
        mask_format_bytes[1],
        mask_format_bytes[2],
        mask_format_bytes[3],
        glyphset_bytes[0],
        glyphset_bytes[1],
        glyphset_bytes[2],
        glyphset_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (glyphcmds).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(glyphcmds), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the CompositeGlyphs32 request
pub const COMPOSITE_GLYPHS32_REQUEST: u8 = 25;
pub fn composite_glyphs32<'c, Conn, A>(conn: &'c Conn, op: A, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let src_bytes = src.serialize();
    let dst_bytes = dst.serialize();
    let mask_format_bytes = mask_format.serialize();
    let glyphset_bytes = glyphset.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        COMPOSITE_GLYPHS32_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        src_bytes[0],
        src_bytes[1],
        src_bytes[2],
        src_bytes[3],
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        mask_format_bytes[0],
        mask_format_bytes[1],
        mask_format_bytes[2],
        mask_format_bytes[3],
        glyphset_bytes[0],
        glyphset_bytes[1],
        glyphset_bytes[2],
        glyphset_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (glyphcmds).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(glyphcmds), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the FillRectangles request
pub const FILL_RECTANGLES_REQUEST: u8 = 26;
pub fn fill_rectangles<'c, Conn, A>(conn: &'c Conn, op: A, dst: Picture, color: Color, rects: &[xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let op = op.into();
    let op_bytes = op.serialize();
    let dst_bytes = dst.serialize();
    let color_bytes = color.serialize();
    let rects_bytes = rects.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        FILL_RECTANGLES_REQUEST,
        0,
        0,
        op_bytes[0],
        0,
        0,
        0,
        dst_bytes[0],
        dst_bytes[1],
        dst_bytes[2],
        dst_bytes[3],
        color_bytes[0],
        color_bytes[1],
        color_bytes[2],
        color_bytes[3],
        color_bytes[4],
        color_bytes[5],
        color_bytes[6],
        color_bytes[7],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&rects_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&rects_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the CreateCursor request
pub const CREATE_CURSOR_REQUEST: u8 = 27;
pub fn create_cursor<Conn>(conn: &Conn, cid: xproto::Cursor, source: Picture, x: u16, y: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let cid_bytes = cid.serialize();
    let source_bytes = source.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_CURSOR_REQUEST,
        0,
        0,
        cid_bytes[0],
        cid_bytes[1],
        cid_bytes[2],
        cid_bytes[3],
        source_bytes[0],
        source_bytes[1],
        source_bytes[2],
        source_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transform {
    pub matrix11: Fixed,
    pub matrix12: Fixed,
    pub matrix13: Fixed,
    pub matrix21: Fixed,
    pub matrix22: Fixed,
    pub matrix23: Fixed,
    pub matrix31: Fixed,
    pub matrix32: Fixed,
    pub matrix33: Fixed,
}
impl TryParse for Transform {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (matrix11, remaining) = Fixed::try_parse(remaining)?;
        let (matrix12, remaining) = Fixed::try_parse(remaining)?;
        let (matrix13, remaining) = Fixed::try_parse(remaining)?;
        let (matrix21, remaining) = Fixed::try_parse(remaining)?;
        let (matrix22, remaining) = Fixed::try_parse(remaining)?;
        let (matrix23, remaining) = Fixed::try_parse(remaining)?;
        let (matrix31, remaining) = Fixed::try_parse(remaining)?;
        let (matrix32, remaining) = Fixed::try_parse(remaining)?;
        let (matrix33, remaining) = Fixed::try_parse(remaining)?;
        let result = Transform { matrix11, matrix12, matrix13, matrix21, matrix22, matrix23, matrix31, matrix32, matrix33 };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Transform {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Transform {
    type Bytes = [u8; 36];
    fn serialize(&self) -> Self::Bytes {
        let matrix11_bytes = self.matrix11.serialize();
        let matrix12_bytes = self.matrix12.serialize();
        let matrix13_bytes = self.matrix13.serialize();
        let matrix21_bytes = self.matrix21.serialize();
        let matrix22_bytes = self.matrix22.serialize();
        let matrix23_bytes = self.matrix23.serialize();
        let matrix31_bytes = self.matrix31.serialize();
        let matrix32_bytes = self.matrix32.serialize();
        let matrix33_bytes = self.matrix33.serialize();
        [
            matrix11_bytes[0],
            matrix11_bytes[1],
            matrix11_bytes[2],
            matrix11_bytes[3],
            matrix12_bytes[0],
            matrix12_bytes[1],
            matrix12_bytes[2],
            matrix12_bytes[3],
            matrix13_bytes[0],
            matrix13_bytes[1],
            matrix13_bytes[2],
            matrix13_bytes[3],
            matrix21_bytes[0],
            matrix21_bytes[1],
            matrix21_bytes[2],
            matrix21_bytes[3],
            matrix22_bytes[0],
            matrix22_bytes[1],
            matrix22_bytes[2],
            matrix22_bytes[3],
            matrix23_bytes[0],
            matrix23_bytes[1],
            matrix23_bytes[2],
            matrix23_bytes[3],
            matrix31_bytes[0],
            matrix31_bytes[1],
            matrix31_bytes[2],
            matrix31_bytes[3],
            matrix32_bytes[0],
            matrix32_bytes[1],
            matrix32_bytes[2],
            matrix32_bytes[3],
            matrix33_bytes[0],
            matrix33_bytes[1],
            matrix33_bytes[2],
            matrix33_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(36);
        self.matrix11.serialize_into(bytes);
        self.matrix12.serialize_into(bytes);
        self.matrix13.serialize_into(bytes);
        self.matrix21.serialize_into(bytes);
        self.matrix22.serialize_into(bytes);
        self.matrix23.serialize_into(bytes);
        self.matrix31.serialize_into(bytes);
        self.matrix32.serialize_into(bytes);
        self.matrix33.serialize_into(bytes);
    }
}

/// Opcode for the SetPictureTransform request
pub const SET_PICTURE_TRANSFORM_REQUEST: u8 = 28;
pub fn set_picture_transform<Conn>(conn: &Conn, picture: Picture, transform: Transform) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let transform_bytes = transform.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_PICTURE_TRANSFORM_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        transform_bytes[0],
        transform_bytes[1],
        transform_bytes[2],
        transform_bytes[3],
        transform_bytes[4],
        transform_bytes[5],
        transform_bytes[6],
        transform_bytes[7],
        transform_bytes[8],
        transform_bytes[9],
        transform_bytes[10],
        transform_bytes[11],
        transform_bytes[12],
        transform_bytes[13],
        transform_bytes[14],
        transform_bytes[15],
        transform_bytes[16],
        transform_bytes[17],
        transform_bytes[18],
        transform_bytes[19],
        transform_bytes[20],
        transform_bytes[21],
        transform_bytes[22],
        transform_bytes[23],
        transform_bytes[24],
        transform_bytes[25],
        transform_bytes[26],
        transform_bytes[27],
        transform_bytes[28],
        transform_bytes[29],
        transform_bytes[30],
        transform_bytes[31],
        transform_bytes[32],
        transform_bytes[33],
        transform_bytes[34],
        transform_bytes[35],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the QueryFilters request
pub const QUERY_FILTERS_REQUEST: u8 = 29;
pub fn query_filters<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<Cookie<'_, Conn, QueryFiltersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        QUERY_FILTERS_REQUEST,
        0,
        0,
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryFiltersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub aliases: Vec<u16>,
    pub filters: Vec<xproto::Str>,
}
impl QueryFiltersReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_aliases, remaining) = u32::try_parse(remaining)?;
        let (num_filters, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (aliases, remaining) = crate::x11_utils::parse_list::<u16>(remaining, num_aliases as usize)?;
        let (filters, remaining) = crate::x11_utils::parse_list::<xproto::Str>(remaining, num_filters as usize)?;
        let result = QueryFiltersReply { response_type, sequence, length, aliases, filters };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryFiltersReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetPictureFilter request
pub const SET_PICTURE_FILTER_REQUEST: u8 = 30;
pub fn set_picture_filter<'c, Conn>(conn: &'c Conn, picture: Picture, filter: &[u8], values: &[Fixed]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let filter_len: u16 = filter.len().try_into()?;
    let filter_len_bytes = filter_len.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        SET_PICTURE_FILTER_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        filter_len_bytes[0],
        filter_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (filter).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    let values_bytes = values.serialize();
    let length_so_far = length_so_far + (&values_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(filter), IoSlice::new(&padding0), IoSlice::new(&values_bytes), IoSlice::new(&padding1)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Animcursorelt {
    pub cursor: xproto::Cursor,
    pub delay: u32,
}
impl TryParse for Animcursorelt {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (cursor, remaining) = xproto::Cursor::try_parse(remaining)?;
        let (delay, remaining) = u32::try_parse(remaining)?;
        let result = Animcursorelt { cursor, delay };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Animcursorelt {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Animcursorelt {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let cursor_bytes = self.cursor.serialize();
        let delay_bytes = self.delay.serialize();
        [
            cursor_bytes[0],
            cursor_bytes[1],
            cursor_bytes[2],
            cursor_bytes[3],
            delay_bytes[0],
            delay_bytes[1],
            delay_bytes[2],
            delay_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.cursor.serialize_into(bytes);
        self.delay.serialize_into(bytes);
    }
}

/// Opcode for the CreateAnimCursor request
pub const CREATE_ANIM_CURSOR_REQUEST: u8 = 31;
pub fn create_anim_cursor<'c, Conn>(conn: &'c Conn, cid: xproto::Cursor, cursors: &[Animcursorelt]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let cid_bytes = cid.serialize();
    let cursors_bytes = cursors.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_ANIM_CURSOR_REQUEST,
        0,
        0,
        cid_bytes[0],
        cid_bytes[1],
        cid_bytes[2],
        cid_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&cursors_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&cursors_bytes), IoSlice::new(&padding0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spanfix {
    pub l: Fixed,
    pub r: Fixed,
    pub y: Fixed,
}
impl TryParse for Spanfix {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (l, remaining) = Fixed::try_parse(remaining)?;
        let (r, remaining) = Fixed::try_parse(remaining)?;
        let (y, remaining) = Fixed::try_parse(remaining)?;
        let result = Spanfix { l, r, y };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Spanfix {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Spanfix {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let l_bytes = self.l.serialize();
        let r_bytes = self.r.serialize();
        let y_bytes = self.y.serialize();
        [
            l_bytes[0],
            l_bytes[1],
            l_bytes[2],
            l_bytes[3],
            r_bytes[0],
            r_bytes[1],
            r_bytes[2],
            r_bytes[3],
            y_bytes[0],
            y_bytes[1],
            y_bytes[2],
            y_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.l.serialize_into(bytes);
        self.r.serialize_into(bytes);
        self.y.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trap {
    pub top: Spanfix,
    pub bot: Spanfix,
}
impl TryParse for Trap {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (top, remaining) = Spanfix::try_parse(remaining)?;
        let (bot, remaining) = Spanfix::try_parse(remaining)?;
        let result = Trap { top, bot };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Trap {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Trap {
    type Bytes = [u8; 24];
    fn serialize(&self) -> Self::Bytes {
        let top_bytes = self.top.serialize();
        let bot_bytes = self.bot.serialize();
        [
            top_bytes[0],
            top_bytes[1],
            top_bytes[2],
            top_bytes[3],
            top_bytes[4],
            top_bytes[5],
            top_bytes[6],
            top_bytes[7],
            top_bytes[8],
            top_bytes[9],
            top_bytes[10],
            top_bytes[11],
            bot_bytes[0],
            bot_bytes[1],
            bot_bytes[2],
            bot_bytes[3],
            bot_bytes[4],
            bot_bytes[5],
            bot_bytes[6],
            bot_bytes[7],
            bot_bytes[8],
            bot_bytes[9],
            bot_bytes[10],
            bot_bytes[11],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.top.serialize_into(bytes);
        self.bot.serialize_into(bytes);
    }
}

/// Opcode for the AddTraps request
pub const ADD_TRAPS_REQUEST: u8 = 32;
pub fn add_traps<'c, Conn>(conn: &'c Conn, picture: Picture, x_off: i16, y_off: i16, traps: &[Trap]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let x_off_bytes = x_off.serialize();
    let y_off_bytes = y_off.serialize();
    let traps_bytes = traps.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        ADD_TRAPS_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        x_off_bytes[0],
        x_off_bytes[1],
        y_off_bytes[0],
        y_off_bytes[1],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&traps_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&traps_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the CreateSolidFill request
pub const CREATE_SOLID_FILL_REQUEST: u8 = 33;
pub fn create_solid_fill<Conn>(conn: &Conn, picture: Picture, color: Color) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let color_bytes = color.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_SOLID_FILL_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        color_bytes[0],
        color_bytes[1],
        color_bytes[2],
        color_bytes[3],
        color_bytes[4],
        color_bytes[5],
        color_bytes[6],
        color_bytes[7],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the CreateLinearGradient request
pub const CREATE_LINEAR_GRADIENT_REQUEST: u8 = 34;
pub fn create_linear_gradient<'c, Conn>(conn: &'c Conn, picture: Picture, p1: Pointfix, p2: Pointfix, num_stops: u32, stops: &[Fixed], colors: &[Color]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let p1_bytes = p1.serialize();
    let p2_bytes = p2.serialize();
    let num_stops_bytes = num_stops.serialize();
    assert_eq!(stops.len(), num_stops as usize, "Argument stops has an incorrect length");
    let stops_bytes = stops.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_LINEAR_GRADIENT_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        p1_bytes[0],
        p1_bytes[1],
        p1_bytes[2],
        p1_bytes[3],
        p1_bytes[4],
        p1_bytes[5],
        p1_bytes[6],
        p1_bytes[7],
        p2_bytes[0],
        p2_bytes[1],
        p2_bytes[2],
        p2_bytes[3],
        p2_bytes[4],
        p2_bytes[5],
        p2_bytes[6],
        p2_bytes[7],
        num_stops_bytes[0],
        num_stops_bytes[1],
        num_stops_bytes[2],
        num_stops_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&stops_bytes).len();
    assert_eq!(colors.len(), num_stops as usize, "Argument colors has an incorrect length");
    let colors_bytes = colors.serialize();
    let length_so_far = length_so_far + (&colors_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&stops_bytes), IoSlice::new(&colors_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the CreateRadialGradient request
pub const CREATE_RADIAL_GRADIENT_REQUEST: u8 = 35;
pub fn create_radial_gradient<'c, Conn>(conn: &'c Conn, picture: Picture, inner: Pointfix, outer: Pointfix, inner_radius: Fixed, outer_radius: Fixed, num_stops: u32, stops: &[Fixed], colors: &[Color]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let inner_bytes = inner.serialize();
    let outer_bytes = outer.serialize();
    let inner_radius_bytes = inner_radius.serialize();
    let outer_radius_bytes = outer_radius.serialize();
    let num_stops_bytes = num_stops.serialize();
    assert_eq!(stops.len(), num_stops as usize, "Argument stops has an incorrect length");
    let stops_bytes = stops.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_RADIAL_GRADIENT_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        inner_bytes[0],
        inner_bytes[1],
        inner_bytes[2],
        inner_bytes[3],
        inner_bytes[4],
        inner_bytes[5],
        inner_bytes[6],
        inner_bytes[7],
        outer_bytes[0],
        outer_bytes[1],
        outer_bytes[2],
        outer_bytes[3],
        outer_bytes[4],
        outer_bytes[5],
        outer_bytes[6],
        outer_bytes[7],
        inner_radius_bytes[0],
        inner_radius_bytes[1],
        inner_radius_bytes[2],
        inner_radius_bytes[3],
        outer_radius_bytes[0],
        outer_radius_bytes[1],
        outer_radius_bytes[2],
        outer_radius_bytes[3],
        num_stops_bytes[0],
        num_stops_bytes[1],
        num_stops_bytes[2],
        num_stops_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&stops_bytes).len();
    assert_eq!(colors.len(), num_stops as usize, "Argument colors has an incorrect length");
    let colors_bytes = colors.serialize();
    let length_so_far = length_so_far + (&colors_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&stops_bytes), IoSlice::new(&colors_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the CreateConicalGradient request
pub const CREATE_CONICAL_GRADIENT_REQUEST: u8 = 36;
pub fn create_conical_gradient<'c, Conn>(conn: &'c Conn, picture: Picture, center: Pointfix, angle: Fixed, num_stops: u32, stops: &[Fixed], colors: &[Color]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let picture_bytes = picture.serialize();
    let center_bytes = center.serialize();
    let angle_bytes = angle.serialize();
    let num_stops_bytes = num_stops.serialize();
    assert_eq!(stops.len(), num_stops as usize, "Argument stops has an incorrect length");
    let stops_bytes = stops.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_CONICAL_GRADIENT_REQUEST,
        0,
        0,
        picture_bytes[0],
        picture_bytes[1],
        picture_bytes[2],
        picture_bytes[3],
        center_bytes[0],
        center_bytes[1],
        center_bytes[2],
        center_bytes[3],
        center_bytes[4],
        center_bytes[5],
        center_bytes[6],
        center_bytes[7],
        angle_bytes[0],
        angle_bytes[1],
        angle_bytes[2],
        angle_bytes[3],
        num_stops_bytes[0],
        num_stops_bytes[1],
        num_stops_bytes[2],
        num_stops_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    let length_so_far = length_so_far + (&stops_bytes).len();
    assert_eq!(colors.len(), num_stops as usize, "Argument colors has an incorrect length");
    let colors_bytes = colors.serialize();
    let length_so_far = length_so_far + (&colors_bytes).len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&stops_bytes), IoSlice::new(&colors_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn render_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }

    fn render_query_pict_formats(&self) -> Result<Cookie<'_, Self, QueryPictFormatsReply>, ConnectionError>
    {
        query_pict_formats(self)
    }

    fn render_query_pict_index_values(&self, format: Pictformat) -> Result<Cookie<'_, Self, QueryPictIndexValuesReply>, ConnectionError>
    {
        query_pict_index_values(self, format)
    }

    fn render_create_picture<'c>(&'c self, pid: Picture, drawable: xproto::Drawable, format: Pictformat, value_list: &CreatePictureAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_picture(self, pid, drawable, format, value_list)
    }

    fn render_change_picture<'c>(&'c self, picture: Picture, value_list: &ChangePictureAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_picture(self, picture, value_list)
    }

    fn render_set_picture_clip_rectangles<'c>(&'c self, picture: Picture, clip_x_origin: i16, clip_y_origin: i16, rectangles: &[xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_picture_clip_rectangles(self, picture, clip_x_origin, clip_y_origin, rectangles)
    }

    fn render_free_picture(&self, picture: Picture) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_picture(self, picture)
    }

    fn render_composite<A>(&self, op: A, src: Picture, mask: Picture, dst: Picture, src_x: i16, src_y: i16, mask_x: i16, mask_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        composite(self, op, src, mask, dst, src_x, src_y, mask_x, mask_y, dst_x, dst_y, width, height)
    }

    fn render_trapezoids<'c, A>(&'c self, op: A, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, traps: &[Trapezoid]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        trapezoids(self, op, src, dst, mask_format, src_x, src_y, traps)
    }

    fn render_triangles<'c, A>(&'c self, op: A, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, triangles: &[Triangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        self::triangles(self, op, src, dst, mask_format, src_x, src_y, triangles)
    }

    fn render_tri_strip<'c, A>(&'c self, op: A, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, points: &[Pointfix]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        tri_strip(self, op, src, dst, mask_format, src_x, src_y, points)
    }

    fn render_tri_fan<'c, A>(&'c self, op: A, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, points: &[Pointfix]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        tri_fan(self, op, src, dst, mask_format, src_x, src_y, points)
    }

    fn render_create_glyph_set(&self, gsid: Glyphset, format: Pictformat) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_glyph_set(self, gsid, format)
    }

    fn render_reference_glyph_set(&self, gsid: Glyphset, existing: Glyphset) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        reference_glyph_set(self, gsid, existing)
    }

    fn render_free_glyph_set(&self, glyphset: Glyphset) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_glyph_set(self, glyphset)
    }

    fn render_add_glyphs<'c>(&'c self, glyphset: Glyphset, glyphs_len: u32, glyphids: &[u32], glyphs: &[Glyphinfo], data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        add_glyphs(self, glyphset, glyphs_len, glyphids, glyphs, data)
    }

    fn render_free_glyphs<'c>(&'c self, glyphset: Glyphset, glyphs: &[Glyph]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        free_glyphs(self, glyphset, glyphs)
    }

    fn render_composite_glyphs8<'c, A>(&'c self, op: A, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        composite_glyphs8(self, op, src, dst, mask_format, glyphset, src_x, src_y, glyphcmds)
    }

    fn render_composite_glyphs16<'c, A>(&'c self, op: A, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        composite_glyphs16(self, op, src, dst, mask_format, glyphset, src_x, src_y, glyphcmds)
    }

    fn render_composite_glyphs32<'c, A>(&'c self, op: A, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        composite_glyphs32(self, op, src, dst, mask_format, glyphset, src_x, src_y, glyphcmds)
    }

    fn render_fill_rectangles<'c, A>(&'c self, op: A, dst: Picture, color: Color, rects: &[xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        fill_rectangles(self, op, dst, color, rects)
    }

    fn render_create_cursor(&self, cid: xproto::Cursor, source: Picture, x: u16, y: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_cursor(self, cid, source, x, y)
    }

    fn render_set_picture_transform(&self, picture: Picture, transform: Transform) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_picture_transform(self, picture, transform)
    }

    fn render_query_filters(&self, drawable: xproto::Drawable) -> Result<Cookie<'_, Self, QueryFiltersReply>, ConnectionError>
    {
        query_filters(self, drawable)
    }

    fn render_set_picture_filter<'c>(&'c self, picture: Picture, filter: &[u8], values: &[Fixed]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_picture_filter(self, picture, filter, values)
    }

    fn render_create_anim_cursor<'c>(&'c self, cid: xproto::Cursor, cursors: &[Animcursorelt]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_anim_cursor(self, cid, cursors)
    }

    fn render_add_traps<'c>(&'c self, picture: Picture, x_off: i16, y_off: i16, traps: &[Trap]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        add_traps(self, picture, x_off, y_off, traps)
    }

    fn render_create_solid_fill(&self, picture: Picture, color: Color) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_solid_fill(self, picture, color)
    }

    fn render_create_linear_gradient<'c>(&'c self, picture: Picture, p1: Pointfix, p2: Pointfix, num_stops: u32, stops: &[Fixed], colors: &[Color]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_linear_gradient(self, picture, p1, p2, num_stops, stops, colors)
    }

    fn render_create_radial_gradient<'c>(&'c self, picture: Picture, inner: Pointfix, outer: Pointfix, inner_radius: Fixed, outer_radius: Fixed, num_stops: u32, stops: &[Fixed], colors: &[Color]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_radial_gradient(self, picture, inner, outer, inner_radius, outer_radius, num_stops, stops, colors)
    }

    fn render_create_conical_gradient<'c>(&'c self, picture: Picture, center: Pointfix, angle: Fixed, num_stops: u32, stops: &[Fixed], colors: &[Color]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_conical_gradient(self, picture, center, angle, num_stops, stops, colors)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
