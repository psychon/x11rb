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
#[allow(unused_imports)]
use super::shm;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XVideo";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (2, 2);

pub type PORT = u32;

pub type ENCODING = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    InputMask = 1 << 0,
    OutputMask = 1 << 1,
    VideoMask = 1 << 2,
    StillMask = 1 << 3,
    ImageMask = 1 << 4,
}
impl From<Type> for u8 {
    fn from(input: Type) -> Self {
        match input {
            Type::InputMask => 1 << 0,
            Type::OutputMask => 1 << 1,
            Type::VideoMask => 1 << 2,
            Type::StillMask => 1 << 3,
            Type::ImageMask => 1 << 4,
        }
    }
}
impl From<Type> for Option<u8> {
    fn from(input: Type) -> Self {
        Some(u8::from(input))
    }
}
impl From<Type> for u16 {
    fn from(input: Type) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Type> for Option<u16> {
    fn from(input: Type) -> Self {
        Some(u16::from(input))
    }
}
impl From<Type> for u32 {
    fn from(input: Type) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Type> for Option<u32> {
    fn from(input: Type) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Type {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Type::InputMask),
            2 => Ok(Type::OutputMask),
            4 => Ok(Type::VideoMask),
            8 => Ok(Type::StillMask),
            16 => Ok(Type::ImageMask),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Type {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Type {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(Type, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum ImageFormatInfoType {
    RGB = 0,
    YUV = 1,
}
impl From<ImageFormatInfoType> for u8 {
    fn from(input: ImageFormatInfoType) -> Self {
        match input {
            ImageFormatInfoType::RGB => 0,
            ImageFormatInfoType::YUV => 1,
        }
    }
}
impl From<ImageFormatInfoType> for Option<u8> {
    fn from(input: ImageFormatInfoType) -> Self {
        Some(u8::from(input))
    }
}
impl From<ImageFormatInfoType> for u16 {
    fn from(input: ImageFormatInfoType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ImageFormatInfoType> for Option<u16> {
    fn from(input: ImageFormatInfoType) -> Self {
        Some(u16::from(input))
    }
}
impl From<ImageFormatInfoType> for u32 {
    fn from(input: ImageFormatInfoType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ImageFormatInfoType> for Option<u32> {
    fn from(input: ImageFormatInfoType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ImageFormatInfoType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ImageFormatInfoType::RGB),
            1 => Ok(ImageFormatInfoType::YUV),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ImageFormatInfoType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ImageFormatInfoType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ImageFormatInfoFormat {
    Packed = 0,
    Planar = 1,
}
impl From<ImageFormatInfoFormat> for u8 {
    fn from(input: ImageFormatInfoFormat) -> Self {
        match input {
            ImageFormatInfoFormat::Packed => 0,
            ImageFormatInfoFormat::Planar => 1,
        }
    }
}
impl From<ImageFormatInfoFormat> for Option<u8> {
    fn from(input: ImageFormatInfoFormat) -> Self {
        Some(u8::from(input))
    }
}
impl From<ImageFormatInfoFormat> for u16 {
    fn from(input: ImageFormatInfoFormat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ImageFormatInfoFormat> for Option<u16> {
    fn from(input: ImageFormatInfoFormat) -> Self {
        Some(u16::from(input))
    }
}
impl From<ImageFormatInfoFormat> for u32 {
    fn from(input: ImageFormatInfoFormat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ImageFormatInfoFormat> for Option<u32> {
    fn from(input: ImageFormatInfoFormat) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ImageFormatInfoFormat {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ImageFormatInfoFormat::Packed),
            1 => Ok(ImageFormatInfoFormat::Planar),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ImageFormatInfoFormat {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ImageFormatInfoFormat {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AttributeFlag {
    Gettable = 1 << 0,
    Settable = 1 << 1,
}
impl From<AttributeFlag> for u8 {
    fn from(input: AttributeFlag) -> Self {
        match input {
            AttributeFlag::Gettable => 1 << 0,
            AttributeFlag::Settable => 1 << 1,
        }
    }
}
impl From<AttributeFlag> for Option<u8> {
    fn from(input: AttributeFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<AttributeFlag> for u16 {
    fn from(input: AttributeFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AttributeFlag> for Option<u16> {
    fn from(input: AttributeFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<AttributeFlag> for u32 {
    fn from(input: AttributeFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AttributeFlag> for Option<u32> {
    fn from(input: AttributeFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for AttributeFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AttributeFlag::Gettable),
            2 => Ok(AttributeFlag::Settable),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for AttributeFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for AttributeFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(AttributeFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VideoNotifyReason {
    Started = 0,
    Stopped = 1,
    Busy = 2,
    Preempted = 3,
    HardError = 4,
}
impl From<VideoNotifyReason> for u8 {
    fn from(input: VideoNotifyReason) -> Self {
        match input {
            VideoNotifyReason::Started => 0,
            VideoNotifyReason::Stopped => 1,
            VideoNotifyReason::Busy => 2,
            VideoNotifyReason::Preempted => 3,
            VideoNotifyReason::HardError => 4,
        }
    }
}
impl From<VideoNotifyReason> for Option<u8> {
    fn from(input: VideoNotifyReason) -> Self {
        Some(u8::from(input))
    }
}
impl From<VideoNotifyReason> for u16 {
    fn from(input: VideoNotifyReason) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VideoNotifyReason> for Option<u16> {
    fn from(input: VideoNotifyReason) -> Self {
        Some(u16::from(input))
    }
}
impl From<VideoNotifyReason> for u32 {
    fn from(input: VideoNotifyReason) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VideoNotifyReason> for Option<u32> {
    fn from(input: VideoNotifyReason) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for VideoNotifyReason {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VideoNotifyReason::Started),
            1 => Ok(VideoNotifyReason::Stopped),
            2 => Ok(VideoNotifyReason::Busy),
            3 => Ok(VideoNotifyReason::Preempted),
            4 => Ok(VideoNotifyReason::HardError),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for VideoNotifyReason {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for VideoNotifyReason {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScanlineOrder {
    TopToBottom = 0,
    BottomToTop = 1,
}
impl From<ScanlineOrder> for u8 {
    fn from(input: ScanlineOrder) -> Self {
        match input {
            ScanlineOrder::TopToBottom => 0,
            ScanlineOrder::BottomToTop => 1,
        }
    }
}
impl From<ScanlineOrder> for Option<u8> {
    fn from(input: ScanlineOrder) -> Self {
        Some(u8::from(input))
    }
}
impl From<ScanlineOrder> for u16 {
    fn from(input: ScanlineOrder) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ScanlineOrder> for Option<u16> {
    fn from(input: ScanlineOrder) -> Self {
        Some(u16::from(input))
    }
}
impl From<ScanlineOrder> for u32 {
    fn from(input: ScanlineOrder) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ScanlineOrder> for Option<u32> {
    fn from(input: ScanlineOrder) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ScanlineOrder {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScanlineOrder::TopToBottom),
            1 => Ok(ScanlineOrder::BottomToTop),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ScanlineOrder {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ScanlineOrder {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GrabPortStatus {
    Success = 0,
    BadExtension = 1,
    AlreadyGrabbed = 2,
    InvalidTime = 3,
    BadReply = 4,
    BadAlloc = 5,
}
impl From<GrabPortStatus> for u8 {
    fn from(input: GrabPortStatus) -> Self {
        match input {
            GrabPortStatus::Success => 0,
            GrabPortStatus::BadExtension => 1,
            GrabPortStatus::AlreadyGrabbed => 2,
            GrabPortStatus::InvalidTime => 3,
            GrabPortStatus::BadReply => 4,
            GrabPortStatus::BadAlloc => 5,
        }
    }
}
impl From<GrabPortStatus> for Option<u8> {
    fn from(input: GrabPortStatus) -> Self {
        Some(u8::from(input))
    }
}
impl From<GrabPortStatus> for u16 {
    fn from(input: GrabPortStatus) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabPortStatus> for Option<u16> {
    fn from(input: GrabPortStatus) -> Self {
        Some(u16::from(input))
    }
}
impl From<GrabPortStatus> for u32 {
    fn from(input: GrabPortStatus) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabPortStatus> for Option<u32> {
    fn from(input: GrabPortStatus) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GrabPortStatus {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GrabPortStatus::Success),
            1 => Ok(GrabPortStatus::BadExtension),
            2 => Ok(GrabPortStatus::AlreadyGrabbed),
            3 => Ok(GrabPortStatus::InvalidTime),
            4 => Ok(GrabPortStatus::BadReply),
            5 => Ok(GrabPortStatus::BadAlloc),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for GrabPortStatus {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GrabPortStatus {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    pub numerator: i32,
    pub denominator: i32,
}
impl TryParse for Rational {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (numerator, new_remaining) = i32::try_parse(remaining)?;
        remaining = new_remaining;
        let (denominator, new_remaining) = i32::try_parse(remaining)?;
        remaining = new_remaining;
        let result = Rational { numerator, denominator };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Rational {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Rational {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let numerator_bytes = self.numerator.serialize();
        let denominator_bytes = self.denominator.serialize();
        [
            numerator_bytes[0],
            numerator_bytes[1],
            numerator_bytes[2],
            numerator_bytes[3],
            denominator_bytes[0],
            denominator_bytes[1],
            denominator_bytes[2],
            denominator_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.numerator.serialize_into(bytes);
        self.denominator.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Format {
    pub visual: VISUALID,
    pub depth: u8,
}
impl TryParse for Format {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (visual, new_remaining) = VISUALID::try_parse(remaining)?;
        remaining = new_remaining;
        let (depth, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(3..).ok_or(ParseError::ParseError)?;
        let result = Format { visual, depth };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Format {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Format {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let visual_bytes = self.visual.serialize();
        let depth_bytes = self.depth.serialize();
        [
            visual_bytes[0],
            visual_bytes[1],
            visual_bytes[2],
            visual_bytes[3],
            depth_bytes[0],
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.visual.serialize_into(bytes);
        self.depth.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptorInfo {
    pub base_id: PORT,
    pub num_ports: u16,
    pub type_: u8,
    pub name: Vec<u8>,
    pub formats: Vec<Format>,
}
impl TryParse for AdaptorInfo {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (base_id, new_remaining) = PORT::try_parse(remaining)?;
        remaining = new_remaining;
        let (name_size, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_ports, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_formats, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (type_, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (name, new_remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_size as usize)?;
        remaining = new_remaining;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        remaining = &remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (formats, new_remaining) = crate::x11_utils::parse_list::<Format>(remaining, num_formats as usize)?;
        remaining = new_remaining;
        let result = AdaptorInfo { base_id, num_ports, type_, name, formats };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AdaptorInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for AdaptorInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.base_id.serialize_into(bytes);
        let name_size = self.name.len() as u16;
        name_size.serialize_into(bytes);
        self.num_ports.serialize_into(bytes);
        let num_formats = self.formats.len() as u16;
        num_formats.serialize_into(bytes);
        self.type_.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.formats.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodingInfo {
    pub encoding: ENCODING,
    pub width: u16,
    pub height: u16,
    pub rate: Rational,
    pub name: Vec<u8>,
}
impl TryParse for EncodingInfo {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (encoding, new_remaining) = ENCODING::try_parse(remaining)?;
        remaining = new_remaining;
        let (name_size, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (width, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (height, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (rate, new_remaining) = Rational::try_parse(remaining)?;
        remaining = new_remaining;
        let (name, new_remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_size as usize)?;
        remaining = new_remaining;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        remaining = &remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = EncodingInfo { encoding, width, height, rate, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for EncodingInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for EncodingInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(20);
        self.encoding.serialize_into(bytes);
        let name_size = self.name.len() as u16;
        name_size.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.rate.serialize_into(bytes);
        self.name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub num_planes: u32,
    pub pitches: Vec<u32>,
    pub offsets: Vec<u32>,
    pub data: Vec<u8>,
}
impl TryParse for Image {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (id, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (width, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (height, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (data_size, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_planes, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (pitches, new_remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_planes as usize)?;
        remaining = new_remaining;
        let (offsets, new_remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_planes as usize)?;
        remaining = new_remaining;
        let (data, new_remaining) = crate::x11_utils::parse_list::<u8>(remaining, data_size as usize)?;
        remaining = new_remaining;
        let result = Image { id, width, height, num_planes, pitches, offsets, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Image {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Image {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.id.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        let data_size = self.data.len() as u32;
        data_size.serialize_into(bytes);
        self.num_planes.serialize_into(bytes);
        self.pitches.serialize_into(bytes);
        self.offsets.serialize_into(bytes);
        self.data.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeInfo {
    pub flags: u32,
    pub min: i32,
    pub max: i32,
    pub name: Vec<u8>,
}
impl TryParse for AttributeInfo {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (flags, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (min, new_remaining) = i32::try_parse(remaining)?;
        remaining = new_remaining;
        let (max, new_remaining) = i32::try_parse(remaining)?;
        remaining = new_remaining;
        let (size, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (name, new_remaining) = crate::x11_utils::parse_list::<u8>(remaining, size as usize)?;
        remaining = new_remaining;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        remaining = &remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = AttributeInfo { flags, min, max, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AttributeInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for AttributeInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.flags.serialize_into(bytes);
        self.min.serialize_into(bytes);
        self.max.serialize_into(bytes);
        let size = self.name.len() as u32;
        size.serialize_into(bytes);
        self.name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageFormatInfo {
    pub id: u32,
    pub type_: u8,
    pub byte_order: u8,
    pub guid: [u8; 16],
    pub bpp: u8,
    pub num_planes: u8,
    pub depth: u8,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub format: u8,
    pub y_sample_bits: u32,
    pub u_sample_bits: u32,
    pub v_sample_bits: u32,
    pub vhorz_y_period: u32,
    pub vhorz_u_period: u32,
    pub vhorz_v_period: u32,
    pub vvert_y_period: u32,
    pub vvert_u_period: u32,
    pub vvert_v_period: u32,
    pub vcomp_order: [u8; 32],
    pub vscanline_order: u8,
}
impl TryParse for ImageFormatInfo {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (id, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (type_, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (byte_order, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (guid_0, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_1, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_2, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_3, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_4, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_5, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_6, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_7, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_8, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_9, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_10, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_11, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_12, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_13, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_14, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (guid_15, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let guid = [
            guid_0,
            guid_1,
            guid_2,
            guid_3,
            guid_4,
            guid_5,
            guid_6,
            guid_7,
            guid_8,
            guid_9,
            guid_10,
            guid_11,
            guid_12,
            guid_13,
            guid_14,
            guid_15,
        ];
        let (bpp, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_planes, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (depth, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (red_mask, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (green_mask, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (blue_mask, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (format, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (y_sample_bits, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (u_sample_bits, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (v_sample_bits, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (vhorz_y_period, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (vhorz_u_period, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (vhorz_v_period, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (vvert_y_period, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (vvert_u_period, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (vvert_v_period, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_0, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_1, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_2, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_3, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_4, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_5, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_6, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_7, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_8, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_9, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_10, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_11, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_12, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_13, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_14, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_15, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_16, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_17, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_18, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_19, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_20, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_21, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_22, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_23, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_24, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_25, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_26, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_27, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_28, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_29, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_30, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (vcomp_order_31, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let vcomp_order = [
            vcomp_order_0,
            vcomp_order_1,
            vcomp_order_2,
            vcomp_order_3,
            vcomp_order_4,
            vcomp_order_5,
            vcomp_order_6,
            vcomp_order_7,
            vcomp_order_8,
            vcomp_order_9,
            vcomp_order_10,
            vcomp_order_11,
            vcomp_order_12,
            vcomp_order_13,
            vcomp_order_14,
            vcomp_order_15,
            vcomp_order_16,
            vcomp_order_17,
            vcomp_order_18,
            vcomp_order_19,
            vcomp_order_20,
            vcomp_order_21,
            vcomp_order_22,
            vcomp_order_23,
            vcomp_order_24,
            vcomp_order_25,
            vcomp_order_26,
            vcomp_order_27,
            vcomp_order_28,
            vcomp_order_29,
            vcomp_order_30,
            vcomp_order_31,
        ];
        let (vscanline_order, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(11..).ok_or(ParseError::ParseError)?;
        let result = ImageFormatInfo { id, type_, byte_order, guid, bpp, num_planes, depth, red_mask, green_mask, blue_mask, format, y_sample_bits, u_sample_bits, v_sample_bits, vhorz_y_period, vhorz_u_period, vhorz_v_period, vvert_y_period, vvert_u_period, vvert_v_period, vcomp_order, vscanline_order };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ImageFormatInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ImageFormatInfo {
    type Bytes = [u8; 128];
    fn serialize(&self) -> Self::Bytes {
        let id_bytes = self.id.serialize();
        let type_bytes = self.type_.serialize();
        let byte_order_bytes = self.byte_order.serialize();
        let bpp_bytes = self.bpp.serialize();
        let num_planes_bytes = self.num_planes.serialize();
        let depth_bytes = self.depth.serialize();
        let red_mask_bytes = self.red_mask.serialize();
        let green_mask_bytes = self.green_mask.serialize();
        let blue_mask_bytes = self.blue_mask.serialize();
        let format_bytes = self.format.serialize();
        let y_sample_bits_bytes = self.y_sample_bits.serialize();
        let u_sample_bits_bytes = self.u_sample_bits.serialize();
        let v_sample_bits_bytes = self.v_sample_bits.serialize();
        let vhorz_y_period_bytes = self.vhorz_y_period.serialize();
        let vhorz_u_period_bytes = self.vhorz_u_period.serialize();
        let vhorz_v_period_bytes = self.vhorz_v_period.serialize();
        let vvert_y_period_bytes = self.vvert_y_period.serialize();
        let vvert_u_period_bytes = self.vvert_u_period.serialize();
        let vvert_v_period_bytes = self.vvert_v_period.serialize();
        let vscanline_order_bytes = self.vscanline_order.serialize();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            type_bytes[0],
            byte_order_bytes[0],
            0,
            0,
            self.guid[0],
            self.guid[1],
            self.guid[2],
            self.guid[3],
            self.guid[4],
            self.guid[5],
            self.guid[6],
            self.guid[7],
            self.guid[8],
            self.guid[9],
            self.guid[10],
            self.guid[11],
            self.guid[12],
            self.guid[13],
            self.guid[14],
            self.guid[15],
            bpp_bytes[0],
            num_planes_bytes[0],
            0,
            0,
            depth_bytes[0],
            0,
            0,
            0,
            red_mask_bytes[0],
            red_mask_bytes[1],
            red_mask_bytes[2],
            red_mask_bytes[3],
            green_mask_bytes[0],
            green_mask_bytes[1],
            green_mask_bytes[2],
            green_mask_bytes[3],
            blue_mask_bytes[0],
            blue_mask_bytes[1],
            blue_mask_bytes[2],
            blue_mask_bytes[3],
            format_bytes[0],
            0,
            0,
            0,
            y_sample_bits_bytes[0],
            y_sample_bits_bytes[1],
            y_sample_bits_bytes[2],
            y_sample_bits_bytes[3],
            u_sample_bits_bytes[0],
            u_sample_bits_bytes[1],
            u_sample_bits_bytes[2],
            u_sample_bits_bytes[3],
            v_sample_bits_bytes[0],
            v_sample_bits_bytes[1],
            v_sample_bits_bytes[2],
            v_sample_bits_bytes[3],
            vhorz_y_period_bytes[0],
            vhorz_y_period_bytes[1],
            vhorz_y_period_bytes[2],
            vhorz_y_period_bytes[3],
            vhorz_u_period_bytes[0],
            vhorz_u_period_bytes[1],
            vhorz_u_period_bytes[2],
            vhorz_u_period_bytes[3],
            vhorz_v_period_bytes[0],
            vhorz_v_period_bytes[1],
            vhorz_v_period_bytes[2],
            vhorz_v_period_bytes[3],
            vvert_y_period_bytes[0],
            vvert_y_period_bytes[1],
            vvert_y_period_bytes[2],
            vvert_y_period_bytes[3],
            vvert_u_period_bytes[0],
            vvert_u_period_bytes[1],
            vvert_u_period_bytes[2],
            vvert_u_period_bytes[3],
            vvert_v_period_bytes[0],
            vvert_v_period_bytes[1],
            vvert_v_period_bytes[2],
            vvert_v_period_bytes[3],
            self.vcomp_order[0],
            self.vcomp_order[1],
            self.vcomp_order[2],
            self.vcomp_order[3],
            self.vcomp_order[4],
            self.vcomp_order[5],
            self.vcomp_order[6],
            self.vcomp_order[7],
            self.vcomp_order[8],
            self.vcomp_order[9],
            self.vcomp_order[10],
            self.vcomp_order[11],
            self.vcomp_order[12],
            self.vcomp_order[13],
            self.vcomp_order[14],
            self.vcomp_order[15],
            self.vcomp_order[16],
            self.vcomp_order[17],
            self.vcomp_order[18],
            self.vcomp_order[19],
            self.vcomp_order[20],
            self.vcomp_order[21],
            self.vcomp_order[22],
            self.vcomp_order[23],
            self.vcomp_order[24],
            self.vcomp_order[25],
            self.vcomp_order[26],
            self.vcomp_order[27],
            self.vcomp_order[28],
            self.vcomp_order[29],
            self.vcomp_order[30],
            self.vcomp_order[31],
            vscanline_order_bytes[0],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(128);
        self.id.serialize_into(bytes);
        self.type_.serialize_into(bytes);
        self.byte_order.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.guid.serialize_into(bytes);
        self.bpp.serialize_into(bytes);
        self.num_planes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.depth.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.red_mask.serialize_into(bytes);
        self.green_mask.serialize_into(bytes);
        self.blue_mask.serialize_into(bytes);
        self.format.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.y_sample_bits.serialize_into(bytes);
        self.u_sample_bits.serialize_into(bytes);
        self.v_sample_bits.serialize_into(bytes);
        self.vhorz_y_period.serialize_into(bytes);
        self.vhorz_u_period.serialize_into(bytes);
        self.vhorz_v_period.serialize_into(bytes);
        self.vvert_y_period.serialize_into(bytes);
        self.vvert_u_period.serialize_into(bytes);
        self.vvert_v_period.serialize_into(bytes);
        self.vcomp_order.serialize_into(bytes);
        self.vscanline_order.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 11]);
    }
}

/// Opcode for the BadPort error
pub const BAD_PORT_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadPortError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadPortError {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (error_code, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let result = BadPortError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadPortError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadPortError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadPortError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadPortError> for [u8; 32] {
    fn from(input: &BadPortError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadPortError> for [u8; 32] {
    fn from(input: BadPortError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadEncoding error
pub const BAD_ENCODING_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadEncodingError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadEncodingError {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (error_code, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let result = BadEncodingError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadEncodingError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadEncodingError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadEncodingError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadEncodingError> for [u8; 32] {
    fn from(input: &BadEncodingError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadEncodingError> for [u8; 32] {
    fn from(input: BadEncodingError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadControl error
pub const BAD_CONTROL_ERROR: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadControlError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadControlError {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (error_code, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let result = BadControlError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadControlError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadControlError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadControlError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadControlError> for [u8; 32] {
    fn from(input: &BadControlError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadControlError> for [u8; 32] {
    fn from(input: BadControlError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the VideoNotify event
pub const VIDEO_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoNotifyEvent {
    pub response_type: u8,
    pub reason: u8,
    pub sequence: u16,
    pub time: TIMESTAMP,
    pub drawable: DRAWABLE,
    pub port: PORT,
}
impl VideoNotifyEvent {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (reason, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (time, new_remaining) = TIMESTAMP::try_parse(remaining)?;
        remaining = new_remaining;
        let (drawable, new_remaining) = DRAWABLE::try_parse(remaining)?;
        remaining = new_remaining;
        let (port, new_remaining) = PORT::try_parse(remaining)?;
        remaining = new_remaining;
        let result = VideoNotifyEvent { response_type, reason, sequence, time, drawable, port };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for VideoNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for VideoNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for VideoNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&VideoNotifyEvent> for [u8; 32] {
    fn from(input: &VideoNotifyEvent) -> Self {
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let drawable = input.drawable.serialize();
        let port = input.port.serialize();
        [
            input.response_type, input.reason, sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            drawable[0], drawable[1], drawable[2], drawable[3], port[0], port[1], port[2], port[3],
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<VideoNotifyEvent> for [u8; 32] {
    fn from(input: VideoNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the PortNotify event
pub const PORT_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub time: TIMESTAMP,
    pub port: PORT,
    pub attribute: ATOM,
    pub value: i32,
}
impl PortNotifyEvent {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (time, new_remaining) = TIMESTAMP::try_parse(remaining)?;
        remaining = new_remaining;
        let (port, new_remaining) = PORT::try_parse(remaining)?;
        remaining = new_remaining;
        let (attribute, new_remaining) = ATOM::try_parse(remaining)?;
        remaining = new_remaining;
        let (value, new_remaining) = i32::try_parse(remaining)?;
        remaining = new_remaining;
        let result = PortNotifyEvent { response_type, sequence, time, port, attribute, value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PortNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericEvent<B>> for PortNotifyEvent {
    fn from(value: GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericEvent<B>> for PortNotifyEvent {
    fn from(value: &GenericEvent<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&PortNotifyEvent> for [u8; 32] {
    fn from(input: &PortNotifyEvent) -> Self {
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let port = input.port.serialize();
        let attribute = input.attribute.serialize();
        let value = input.value.serialize();
        [
            input.response_type, 0, sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            port[0], port[1], port[2], port[3], attribute[0], attribute[1], attribute[2], attribute[3],
            value[0], value[1], value[2], value[3], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<PortNotifyEvent> for [u8; 32] {
    fn from(input: PortNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the QueryExtension request
pub const QUERY_EXTENSION_REQUEST: u8 = 0;
pub fn query_extension<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryExtensionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_EXTENSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtensionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}
impl QueryExtensionReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (major, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (minor, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let result = QueryExtensionReply { response_type, sequence, length, major, minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryExtensionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryAdaptors request
pub const QUERY_ADAPTORS_REQUEST: u8 = 1;
pub fn query_adaptors<Conn>(conn: &Conn, window: WINDOW) -> Result<Cookie<'_, Conn, QueryAdaptorsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_ADAPTORS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryAdaptorsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub info: Vec<AdaptorInfo>,
}
impl QueryAdaptorsReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_adaptors, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (info, new_remaining) = crate::x11_utils::parse_list::<AdaptorInfo>(remaining, num_adaptors as usize)?;
        remaining = new_remaining;
        let result = QueryAdaptorsReply { response_type, sequence, length, info };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryAdaptorsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryEncodings request
pub const QUERY_ENCODINGS_REQUEST: u8 = 2;
pub fn query_encodings<Conn>(conn: &Conn, port: PORT) -> Result<Cookie<'_, Conn, QueryEncodingsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_ENCODINGS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEncodingsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub info: Vec<EncodingInfo>,
}
impl QueryEncodingsReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_encodings, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (info, new_remaining) = crate::x11_utils::parse_list::<EncodingInfo>(remaining, num_encodings as usize)?;
        remaining = new_remaining;
        let result = QueryEncodingsReply { response_type, sequence, length, info };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryEncodingsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GrabPort request
pub const GRAB_PORT_REQUEST: u8 = 3;
pub fn grab_port<Conn>(conn: &Conn, port: PORT, time: TIMESTAMP) -> Result<Cookie<'_, Conn, GrabPortReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let time_bytes = time.serialize();
    let request0 = [
        extension_information.major_opcode,
        GRAB_PORT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabPortReply {
    pub response_type: u8,
    pub result: u8,
    pub sequence: u16,
    pub length: u32,
}
impl GrabPortReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (result, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let result = GrabPortReply { response_type, result, sequence, length };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GrabPortReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the UngrabPort request
pub const UNGRAB_PORT_REQUEST: u8 = 4;
pub fn ungrab_port<Conn>(conn: &Conn, port: PORT, time: TIMESTAMP) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let time_bytes = time.serialize();
    let request0 = [
        extension_information.major_opcode,
        UNGRAB_PORT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the PutVideo request
pub const PUT_VIDEO_REQUEST: u8 = 5;
pub fn put_video<Conn>(conn: &Conn, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let vid_x_bytes = vid_x.serialize();
    let vid_y_bytes = vid_y.serialize();
    let vid_w_bytes = vid_w.serialize();
    let vid_h_bytes = vid_h.serialize();
    let drw_x_bytes = drw_x.serialize();
    let drw_y_bytes = drw_y.serialize();
    let drw_w_bytes = drw_w.serialize();
    let drw_h_bytes = drw_h.serialize();
    let request0 = [
        extension_information.major_opcode,
        PUT_VIDEO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        vid_x_bytes[0],
        vid_x_bytes[1],
        vid_y_bytes[0],
        vid_y_bytes[1],
        vid_w_bytes[0],
        vid_w_bytes[1],
        vid_h_bytes[0],
        vid_h_bytes[1],
        drw_x_bytes[0],
        drw_x_bytes[1],
        drw_y_bytes[0],
        drw_y_bytes[1],
        drw_w_bytes[0],
        drw_w_bytes[1],
        drw_h_bytes[0],
        drw_h_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the PutStill request
pub const PUT_STILL_REQUEST: u8 = 6;
pub fn put_still<Conn>(conn: &Conn, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let vid_x_bytes = vid_x.serialize();
    let vid_y_bytes = vid_y.serialize();
    let vid_w_bytes = vid_w.serialize();
    let vid_h_bytes = vid_h.serialize();
    let drw_x_bytes = drw_x.serialize();
    let drw_y_bytes = drw_y.serialize();
    let drw_w_bytes = drw_w.serialize();
    let drw_h_bytes = drw_h.serialize();
    let request0 = [
        extension_information.major_opcode,
        PUT_STILL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        vid_x_bytes[0],
        vid_x_bytes[1],
        vid_y_bytes[0],
        vid_y_bytes[1],
        vid_w_bytes[0],
        vid_w_bytes[1],
        vid_h_bytes[0],
        vid_h_bytes[1],
        drw_x_bytes[0],
        drw_x_bytes[1],
        drw_y_bytes[0],
        drw_y_bytes[1],
        drw_w_bytes[0],
        drw_w_bytes[1],
        drw_h_bytes[0],
        drw_h_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetVideo request
pub const GET_VIDEO_REQUEST: u8 = 7;
pub fn get_video<Conn>(conn: &Conn, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let vid_x_bytes = vid_x.serialize();
    let vid_y_bytes = vid_y.serialize();
    let vid_w_bytes = vid_w.serialize();
    let vid_h_bytes = vid_h.serialize();
    let drw_x_bytes = drw_x.serialize();
    let drw_y_bytes = drw_y.serialize();
    let drw_w_bytes = drw_w.serialize();
    let drw_h_bytes = drw_h.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_VIDEO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        vid_x_bytes[0],
        vid_x_bytes[1],
        vid_y_bytes[0],
        vid_y_bytes[1],
        vid_w_bytes[0],
        vid_w_bytes[1],
        vid_h_bytes[0],
        vid_h_bytes[1],
        drw_x_bytes[0],
        drw_x_bytes[1],
        drw_y_bytes[0],
        drw_y_bytes[1],
        drw_w_bytes[0],
        drw_w_bytes[1],
        drw_h_bytes[0],
        drw_h_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetStill request
pub const GET_STILL_REQUEST: u8 = 8;
pub fn get_still<Conn>(conn: &Conn, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let vid_x_bytes = vid_x.serialize();
    let vid_y_bytes = vid_y.serialize();
    let vid_w_bytes = vid_w.serialize();
    let vid_h_bytes = vid_h.serialize();
    let drw_x_bytes = drw_x.serialize();
    let drw_y_bytes = drw_y.serialize();
    let drw_w_bytes = drw_w.serialize();
    let drw_h_bytes = drw_h.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_STILL_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        vid_x_bytes[0],
        vid_x_bytes[1],
        vid_y_bytes[0],
        vid_y_bytes[1],
        vid_w_bytes[0],
        vid_w_bytes[1],
        vid_h_bytes[0],
        vid_h_bytes[1],
        drw_x_bytes[0],
        drw_x_bytes[1],
        drw_y_bytes[0],
        drw_y_bytes[1],
        drw_w_bytes[0],
        drw_w_bytes[1],
        drw_h_bytes[0],
        drw_h_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the StopVideo request
pub const STOP_VIDEO_REQUEST: u8 = 9;
pub fn stop_video<Conn>(conn: &Conn, port: PORT, drawable: DRAWABLE) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let drawable_bytes = drawable.serialize();
    let request0 = [
        extension_information.major_opcode,
        STOP_VIDEO_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SelectVideoNotify request
pub const SELECT_VIDEO_NOTIFY_REQUEST: u8 = 10;
pub fn select_video_notify<Conn>(conn: &Conn, drawable: DRAWABLE, onoff: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let onoff_bytes = (onoff as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_VIDEO_NOTIFY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        onoff_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SelectPortNotify request
pub const SELECT_PORT_NOTIFY_REQUEST: u8 = 11;
pub fn select_port_notify<Conn>(conn: &Conn, port: PORT, onoff: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let onoff_bytes = (onoff as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        SELECT_PORT_NOTIFY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        onoff_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the QueryBestSize request
pub const QUERY_BEST_SIZE_REQUEST: u8 = 12;
pub fn query_best_size<Conn>(conn: &Conn, port: PORT, vid_w: u16, vid_h: u16, drw_w: u16, drw_h: u16, motion: bool) -> Result<Cookie<'_, Conn, QueryBestSizeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (20) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let vid_w_bytes = vid_w.serialize();
    let vid_h_bytes = vid_h.serialize();
    let drw_w_bytes = drw_w.serialize();
    let drw_h_bytes = drw_h.serialize();
    let motion_bytes = (motion as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_BEST_SIZE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        vid_w_bytes[0],
        vid_w_bytes[1],
        vid_h_bytes[0],
        vid_h_bytes[1],
        drw_w_bytes[0],
        drw_w_bytes[1],
        drw_h_bytes[0],
        drw_h_bytes[1],
        motion_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryBestSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub actual_width: u16,
    pub actual_height: u16,
}
impl QueryBestSizeReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (actual_width, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (actual_height, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let result = QueryBestSizeReply { response_type, sequence, length, actual_width, actual_height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryBestSizeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetPortAttribute request
pub const SET_PORT_ATTRIBUTE_REQUEST: u8 = 13;
pub fn set_port_attribute<Conn>(conn: &Conn, port: PORT, attribute: ATOM, value: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let attribute_bytes = attribute.serialize();
    let value_bytes = value.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_PORT_ATTRIBUTE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        attribute_bytes[0],
        attribute_bytes[1],
        attribute_bytes[2],
        attribute_bytes[3],
        value_bytes[0],
        value_bytes[1],
        value_bytes[2],
        value_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetPortAttribute request
pub const GET_PORT_ATTRIBUTE_REQUEST: u8 = 14;
pub fn get_port_attribute<Conn>(conn: &Conn, port: PORT, attribute: ATOM) -> Result<Cookie<'_, Conn, GetPortAttributeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let attribute_bytes = attribute.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PORT_ATTRIBUTE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        attribute_bytes[0],
        attribute_bytes[1],
        attribute_bytes[2],
        attribute_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPortAttributeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub value: i32,
}
impl GetPortAttributeReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (value, new_remaining) = i32::try_parse(remaining)?;
        remaining = new_remaining;
        let result = GetPortAttributeReply { response_type, sequence, length, value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPortAttributeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryPortAttributes request
pub const QUERY_PORT_ATTRIBUTES_REQUEST: u8 = 15;
pub fn query_port_attributes<Conn>(conn: &Conn, port: PORT) -> Result<Cookie<'_, Conn, QueryPortAttributesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_PORT_ATTRIBUTES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryPortAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub text_size: u32,
    pub attributes: Vec<AttributeInfo>,
}
impl QueryPortAttributesReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_attributes, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (text_size, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (attributes, new_remaining) = crate::x11_utils::parse_list::<AttributeInfo>(remaining, num_attributes as usize)?;
        remaining = new_remaining;
        let result = QueryPortAttributesReply { response_type, sequence, length, text_size, attributes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryPortAttributesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListImageFormats request
pub const LIST_IMAGE_FORMATS_REQUEST: u8 = 16;
pub fn list_image_formats<Conn>(conn: &Conn, port: PORT) -> Result<Cookie<'_, Conn, ListImageFormatsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_IMAGE_FORMATS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListImageFormatsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub format: Vec<ImageFormatInfo>,
}
impl ListImageFormatsReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_formats, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (format, new_remaining) = crate::x11_utils::parse_list::<ImageFormatInfo>(remaining, num_formats as usize)?;
        remaining = new_remaining;
        let result = ListImageFormatsReply { response_type, sequence, length, format };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListImageFormatsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryImageAttributes request
pub const QUERY_IMAGE_ATTRIBUTES_REQUEST: u8 = 17;
pub fn query_image_attributes<Conn>(conn: &Conn, port: PORT, id: u32, width: u16, height: u16) -> Result<Cookie<'_, Conn, QueryImageAttributesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let id_bytes = id.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_IMAGE_ATTRIBUTES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryImageAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_planes: u32,
    pub data_size: u32,
    pub width: u16,
    pub height: u16,
    pub pitches: Vec<u32>,
    pub offsets: Vec<u32>,
}
impl QueryImageAttributesReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (num_planes, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (data_size, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (width, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (height, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (pitches, new_remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_planes as usize)?;
        remaining = new_remaining;
        let (offsets, new_remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_planes as usize)?;
        remaining = new_remaining;
        let result = QueryImageAttributesReply { response_type, sequence, length, num_planes, data_size, width, height, pitches, offsets };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryImageAttributesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PutImage request
pub const PUT_IMAGE_REQUEST: u8 = 18;
pub fn put_image<'c, Conn>(conn: &'c Conn, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, id: u32, src_x: i16, src_y: i16, src_w: u16, src_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16, width: u16, height: u16, data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (40 + 1 * data.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let id_bytes = id.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let src_w_bytes = src_w.serialize();
    let src_h_bytes = src_h.serialize();
    let drw_x_bytes = drw_x.serialize();
    let drw_y_bytes = drw_y.serialize();
    let drw_w_bytes = drw_w.serialize();
    let drw_h_bytes = drw_h.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let request0 = [
        extension_information.major_opcode,
        PUT_IMAGE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
        src_w_bytes[0],
        src_w_bytes[1],
        src_h_bytes[0],
        src_h_bytes[1],
        drw_x_bytes[0],
        drw_x_bytes[1],
        drw_y_bytes[0],
        drw_y_bytes[1],
        drw_w_bytes[0],
        drw_w_bytes[1],
        drw_h_bytes[0],
        drw_h_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ShmPutImage request
pub const SHM_PUT_IMAGE_REQUEST: u8 = 19;
pub fn shm_put_image<Conn>(conn: &Conn, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, shmseg: shm::SEG, id: u32, offset: u32, src_x: i16, src_y: i16, src_w: u16, src_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16, width: u16, height: u16, send_event: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (52) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let port_bytes = port.serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let shmseg_bytes = shmseg.serialize();
    let id_bytes = id.serialize();
    let offset_bytes = offset.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let src_w_bytes = src_w.serialize();
    let src_h_bytes = src_h.serialize();
    let drw_x_bytes = drw_x.serialize();
    let drw_y_bytes = drw_y.serialize();
    let drw_w_bytes = drw_w.serialize();
    let drw_h_bytes = drw_h.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let send_event_bytes = send_event.serialize();
    let request0 = [
        extension_information.major_opcode,
        SHM_PUT_IMAGE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        port_bytes[0],
        port_bytes[1],
        port_bytes[2],
        port_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        shmseg_bytes[0],
        shmseg_bytes[1],
        shmseg_bytes[2],
        shmseg_bytes[3],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        offset_bytes[0],
        offset_bytes[1],
        offset_bytes[2],
        offset_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
        src_w_bytes[0],
        src_w_bytes[1],
        src_h_bytes[0],
        src_h_bytes[1],
        drw_x_bytes[0],
        drw_x_bytes[1],
        drw_y_bytes[0],
        drw_y_bytes[1],
        drw_w_bytes[0],
        drw_w_bytes[1],
        drw_h_bytes[0],
        drw_h_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        send_event_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xv_query_extension(&self) -> Result<Cookie<'_, Self, QueryExtensionReply>, ConnectionError>
    {
        query_extension(self)
    }

    fn xv_query_adaptors(&self, window: WINDOW) -> Result<Cookie<'_, Self, QueryAdaptorsReply>, ConnectionError>
    {
        query_adaptors(self, window)
    }

    fn xv_query_encodings(&self, port: PORT) -> Result<Cookie<'_, Self, QueryEncodingsReply>, ConnectionError>
    {
        query_encodings(self, port)
    }

    fn xv_grab_port(&self, port: PORT, time: TIMESTAMP) -> Result<Cookie<'_, Self, GrabPortReply>, ConnectionError>
    {
        grab_port(self, port, time)
    }

    fn xv_ungrab_port(&self, port: PORT, time: TIMESTAMP) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        ungrab_port(self, port, time)
    }

    fn xv_put_video(&self, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        put_video(self, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h)
    }

    fn xv_put_still(&self, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        put_still(self, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h)
    }

    fn xv_get_video(&self, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        get_video(self, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h)
    }

    fn xv_get_still(&self, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        get_still(self, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h)
    }

    fn xv_stop_video(&self, port: PORT, drawable: DRAWABLE) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        stop_video(self, port, drawable)
    }

    fn xv_select_video_notify(&self, drawable: DRAWABLE, onoff: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_video_notify(self, drawable, onoff)
    }

    fn xv_select_port_notify(&self, port: PORT, onoff: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_port_notify(self, port, onoff)
    }

    fn xv_query_best_size(&self, port: PORT, vid_w: u16, vid_h: u16, drw_w: u16, drw_h: u16, motion: bool) -> Result<Cookie<'_, Self, QueryBestSizeReply>, ConnectionError>
    {
        query_best_size(self, port, vid_w, vid_h, drw_w, drw_h, motion)
    }

    fn xv_set_port_attribute(&self, port: PORT, attribute: ATOM, value: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_port_attribute(self, port, attribute, value)
    }

    fn xv_get_port_attribute(&self, port: PORT, attribute: ATOM) -> Result<Cookie<'_, Self, GetPortAttributeReply>, ConnectionError>
    {
        get_port_attribute(self, port, attribute)
    }

    fn xv_query_port_attributes(&self, port: PORT) -> Result<Cookie<'_, Self, QueryPortAttributesReply>, ConnectionError>
    {
        query_port_attributes(self, port)
    }

    fn xv_list_image_formats(&self, port: PORT) -> Result<Cookie<'_, Self, ListImageFormatsReply>, ConnectionError>
    {
        list_image_formats(self, port)
    }

    fn xv_query_image_attributes(&self, port: PORT, id: u32, width: u16, height: u16) -> Result<Cookie<'_, Self, QueryImageAttributesReply>, ConnectionError>
    {
        query_image_attributes(self, port, id, width, height)
    }

    fn xv_put_image<'c>(&'c self, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, id: u32, src_x: i16, src_y: i16, src_w: u16, src_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16, width: u16, height: u16, data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        put_image(self, port, drawable, gc, id, src_x, src_y, src_w, src_h, drw_x, drw_y, drw_w, drw_h, width, height, data)
    }

    fn xv_shm_put_image(&self, port: PORT, drawable: DRAWABLE, gc: GCONTEXT, shmseg: shm::SEG, id: u32, offset: u32, src_x: i16, src_y: i16, src_w: u16, src_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16, width: u16, height: u16, send_event: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        shm_put_image(self, port, drawable, gc, shmseg, id, offset, src_x, src_y, src_w, src_h, drw_x, drw_y, drw_w, drw_h, width, height, send_event)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
