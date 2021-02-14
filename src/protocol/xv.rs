// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xv` X11 extension.

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
use super::shm;
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XVideo";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (2, 2);

pub type Port = u32;

pub type Encoding = u32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Type(u8);
impl Type {
    pub const INPUT_MASK: Self = Self(1 << 0);
    pub const OUTPUT_MASK: Self = Self(1 << 1);
    pub const VIDEO_MASK: Self = Self(1 << 2);
    pub const STILL_MASK: Self = Self(1 << 3);
    pub const IMAGE_MASK: Self = Self(1 << 4);
}
impl From<Type> for u8 {
    #[inline]
    fn from(input: Type) -> Self {
        input.0
    }
}
impl From<Type> for Option<u8> {
    #[inline]
    fn from(input: Type) -> Self {
        Some(input.0)
    }
}
impl From<Type> for u16 {
    #[inline]
    fn from(input: Type) -> Self {
        u16::from(input.0)
    }
}
impl From<Type> for Option<u16> {
    #[inline]
    fn from(input: Type) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Type> for u32 {
    #[inline]
    fn from(input: Type) -> Self {
        u32::from(input.0)
    }
}
impl From<Type> for Option<u32> {
    #[inline]
    fn from(input: Type) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Type {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Type  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::INPUT_MASK.0.into(), "INPUT_MASK", "InputMask"),
            (Self::OUTPUT_MASK.0.into(), "OUTPUT_MASK", "OutputMask"),
            (Self::VIDEO_MASK.0.into(), "VIDEO_MASK", "VideoMask"),
            (Self::STILL_MASK.0.into(), "STILL_MASK", "StillMask"),
            (Self::IMAGE_MASK.0.into(), "IMAGE_MASK", "ImageMask"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(Type, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ImageFormatInfoType(u8);
impl ImageFormatInfoType {
    pub const RGB: Self = Self(0);
    pub const YUV: Self = Self(1);
}
impl From<ImageFormatInfoType> for u8 {
    #[inline]
    fn from(input: ImageFormatInfoType) -> Self {
        input.0
    }
}
impl From<ImageFormatInfoType> for Option<u8> {
    #[inline]
    fn from(input: ImageFormatInfoType) -> Self {
        Some(input.0)
    }
}
impl From<ImageFormatInfoType> for u16 {
    #[inline]
    fn from(input: ImageFormatInfoType) -> Self {
        u16::from(input.0)
    }
}
impl From<ImageFormatInfoType> for Option<u16> {
    #[inline]
    fn from(input: ImageFormatInfoType) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ImageFormatInfoType> for u32 {
    #[inline]
    fn from(input: ImageFormatInfoType) -> Self {
        u32::from(input.0)
    }
}
impl From<ImageFormatInfoType> for Option<u32> {
    #[inline]
    fn from(input: ImageFormatInfoType) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ImageFormatInfoType {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ImageFormatInfoType  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::RGB.0.into(), "RGB", "RGB"),
            (Self::YUV.0.into(), "YUV", "YUV"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ImageFormatInfoFormat(u8);
impl ImageFormatInfoFormat {
    pub const PACKED: Self = Self(0);
    pub const PLANAR: Self = Self(1);
}
impl From<ImageFormatInfoFormat> for u8 {
    #[inline]
    fn from(input: ImageFormatInfoFormat) -> Self {
        input.0
    }
}
impl From<ImageFormatInfoFormat> for Option<u8> {
    #[inline]
    fn from(input: ImageFormatInfoFormat) -> Self {
        Some(input.0)
    }
}
impl From<ImageFormatInfoFormat> for u16 {
    #[inline]
    fn from(input: ImageFormatInfoFormat) -> Self {
        u16::from(input.0)
    }
}
impl From<ImageFormatInfoFormat> for Option<u16> {
    #[inline]
    fn from(input: ImageFormatInfoFormat) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ImageFormatInfoFormat> for u32 {
    #[inline]
    fn from(input: ImageFormatInfoFormat) -> Self {
        u32::from(input.0)
    }
}
impl From<ImageFormatInfoFormat> for Option<u32> {
    #[inline]
    fn from(input: ImageFormatInfoFormat) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ImageFormatInfoFormat {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ImageFormatInfoFormat  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::PACKED.0.into(), "PACKED", "Packed"),
            (Self::PLANAR.0.into(), "PLANAR", "Planar"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AttributeFlag(u8);
impl AttributeFlag {
    pub const GETTABLE: Self = Self(1 << 0);
    pub const SETTABLE: Self = Self(1 << 1);
}
impl From<AttributeFlag> for u8 {
    #[inline]
    fn from(input: AttributeFlag) -> Self {
        input.0
    }
}
impl From<AttributeFlag> for Option<u8> {
    #[inline]
    fn from(input: AttributeFlag) -> Self {
        Some(input.0)
    }
}
impl From<AttributeFlag> for u16 {
    #[inline]
    fn from(input: AttributeFlag) -> Self {
        u16::from(input.0)
    }
}
impl From<AttributeFlag> for Option<u16> {
    #[inline]
    fn from(input: AttributeFlag) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<AttributeFlag> for u32 {
    #[inline]
    fn from(input: AttributeFlag) -> Self {
        u32::from(input.0)
    }
}
impl From<AttributeFlag> for Option<u32> {
    #[inline]
    fn from(input: AttributeFlag) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for AttributeFlag {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for AttributeFlag  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::GETTABLE.0.into(), "GETTABLE", "Gettable"),
            (Self::SETTABLE.0.into(), "SETTABLE", "Settable"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(AttributeFlag, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VideoNotifyReason(u8);
impl VideoNotifyReason {
    pub const STARTED: Self = Self(0);
    pub const STOPPED: Self = Self(1);
    pub const BUSY: Self = Self(2);
    pub const PREEMPTED: Self = Self(3);
    pub const HARD_ERROR: Self = Self(4);
}
impl From<VideoNotifyReason> for u8 {
    #[inline]
    fn from(input: VideoNotifyReason) -> Self {
        input.0
    }
}
impl From<VideoNotifyReason> for Option<u8> {
    #[inline]
    fn from(input: VideoNotifyReason) -> Self {
        Some(input.0)
    }
}
impl From<VideoNotifyReason> for u16 {
    #[inline]
    fn from(input: VideoNotifyReason) -> Self {
        u16::from(input.0)
    }
}
impl From<VideoNotifyReason> for Option<u16> {
    #[inline]
    fn from(input: VideoNotifyReason) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<VideoNotifyReason> for u32 {
    #[inline]
    fn from(input: VideoNotifyReason) -> Self {
        u32::from(input.0)
    }
}
impl From<VideoNotifyReason> for Option<u32> {
    #[inline]
    fn from(input: VideoNotifyReason) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for VideoNotifyReason {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for VideoNotifyReason  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::STARTED.0.into(), "STARTED", "Started"),
            (Self::STOPPED.0.into(), "STOPPED", "Stopped"),
            (Self::BUSY.0.into(), "BUSY", "Busy"),
            (Self::PREEMPTED.0.into(), "PREEMPTED", "Preempted"),
            (Self::HARD_ERROR.0.into(), "HARD_ERROR", "HardError"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ScanlineOrder(u8);
impl ScanlineOrder {
    pub const TOP_TO_BOTTOM: Self = Self(0);
    pub const BOTTOM_TO_TOP: Self = Self(1);
}
impl From<ScanlineOrder> for u8 {
    #[inline]
    fn from(input: ScanlineOrder) -> Self {
        input.0
    }
}
impl From<ScanlineOrder> for Option<u8> {
    #[inline]
    fn from(input: ScanlineOrder) -> Self {
        Some(input.0)
    }
}
impl From<ScanlineOrder> for u16 {
    #[inline]
    fn from(input: ScanlineOrder) -> Self {
        u16::from(input.0)
    }
}
impl From<ScanlineOrder> for Option<u16> {
    #[inline]
    fn from(input: ScanlineOrder) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ScanlineOrder> for u32 {
    #[inline]
    fn from(input: ScanlineOrder) -> Self {
        u32::from(input.0)
    }
}
impl From<ScanlineOrder> for Option<u32> {
    #[inline]
    fn from(input: ScanlineOrder) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ScanlineOrder {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ScanlineOrder  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::TOP_TO_BOTTOM.0.into(), "TOP_TO_BOTTOM", "TopToBottom"),
            (Self::BOTTOM_TO_TOP.0.into(), "BOTTOM_TO_TOP", "BottomToTop"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GrabPortStatus(u8);
impl GrabPortStatus {
    pub const SUCCESS: Self = Self(0);
    pub const BAD_EXTENSION: Self = Self(1);
    pub const ALREADY_GRABBED: Self = Self(2);
    pub const INVALID_TIME: Self = Self(3);
    pub const BAD_REPLY: Self = Self(4);
    pub const BAD_ALLOC: Self = Self(5);
}
impl From<GrabPortStatus> for u8 {
    #[inline]
    fn from(input: GrabPortStatus) -> Self {
        input.0
    }
}
impl From<GrabPortStatus> for Option<u8> {
    #[inline]
    fn from(input: GrabPortStatus) -> Self {
        Some(input.0)
    }
}
impl From<GrabPortStatus> for u16 {
    #[inline]
    fn from(input: GrabPortStatus) -> Self {
        u16::from(input.0)
    }
}
impl From<GrabPortStatus> for Option<u16> {
    #[inline]
    fn from(input: GrabPortStatus) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<GrabPortStatus> for u32 {
    #[inline]
    fn from(input: GrabPortStatus) -> Self {
        u32::from(input.0)
    }
}
impl From<GrabPortStatus> for Option<u32> {
    #[inline]
    fn from(input: GrabPortStatus) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for GrabPortStatus {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for GrabPortStatus  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::SUCCESS.0.into(), "SUCCESS", "Success"),
            (Self::BAD_EXTENSION.0.into(), "BAD_EXTENSION", "BadExtension"),
            (Self::ALREADY_GRABBED.0.into(), "ALREADY_GRABBED", "AlreadyGrabbed"),
            (Self::INVALID_TIME.0.into(), "INVALID_TIME", "InvalidTime"),
            (Self::BAD_REPLY.0.into(), "BAD_REPLY", "BadReply"),
            (Self::BAD_ALLOC.0.into(), "BAD_ALLOC", "BadAlloc"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    pub numerator: i32,
    pub denominator: i32,
}
impl TryParse for Rational {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (numerator, remaining) = i32::try_parse(remaining)?;
        let (denominator, remaining) = i32::try_parse(remaining)?;
        let result = Rational { numerator, denominator };
        Ok((result, remaining))
    }
}
impl Serialize for Rational {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
    pub visual: xproto::Visualid,
    pub depth: u8,
}
impl TryParse for Format {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (visual, remaining) = xproto::Visualid::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let result = Format { visual, depth };
        Ok((result, remaining))
    }
}
impl Serialize for Format {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
    pub base_id: Port,
    pub num_ports: u16,
    pub type_: u8,
    pub name: Vec<u8>,
    pub formats: Vec<Format>,
}
impl TryParse for AdaptorInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (base_id, remaining) = Port::try_parse(remaining)?;
        let (name_size, remaining) = u16::try_parse(remaining)?;
        let (num_ports, remaining) = u16::try_parse(remaining)?;
        let (num_formats, remaining) = u16::try_parse(remaining)?;
        let (type_, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_size.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (formats, remaining) = crate::x11_utils::parse_list::<Format>(remaining, num_formats.try_to_usize()?)?;
        let result = AdaptorInfo { base_id, num_ports, type_, name, formats };
        Ok((result, remaining))
    }
}
impl Serialize for AdaptorInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.base_id.serialize_into(bytes);
        let name_size = u16::try_from(self.name.len()).expect("`name` has too many elements");
        name_size.serialize_into(bytes);
        self.num_ports.serialize_into(bytes);
        let num_formats = u16::try_from(self.formats.len()).expect("`formats` has too many elements");
        num_formats.serialize_into(bytes);
        self.type_.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.formats.serialize_into(bytes);
    }
}
impl AdaptorInfo {
    /// Get the value of the `name_size` field.
    ///
    /// The `name_size` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn name_size(&self) -> u16 {
        self.name.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_formats` field.
    ///
    /// The `num_formats` field is used as the length field of the `formats` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_formats(&self) -> u16 {
        self.formats.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodingInfo {
    pub encoding: Encoding,
    pub width: u16,
    pub height: u16,
    pub rate: Rational,
    pub name: Vec<u8>,
}
impl TryParse for EncodingInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (encoding, remaining) = Encoding::try_parse(remaining)?;
        let (name_size, remaining) = u16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (rate, remaining) = Rational::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_size.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let result = EncodingInfo { encoding, width, height, rate, name };
        Ok((result, remaining))
    }
}
impl Serialize for EncodingInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(20);
        self.encoding.serialize_into(bytes);
        let name_size = u16::try_from(self.name.len()).expect("`name` has too many elements");
        name_size.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.rate.serialize_into(bytes);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl EncodingInfo {
    /// Get the value of the `name_size` field.
    ///
    /// The `name_size` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn name_size(&self) -> u16 {
        self.name.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub pitches: Vec<u32>,
    pub offsets: Vec<u32>,
    pub data: Vec<u8>,
}
impl TryParse for Image {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (id, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (data_size, remaining) = u32::try_parse(remaining)?;
        let (num_planes, remaining) = u32::try_parse(remaining)?;
        let (pitches, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_planes.try_to_usize()?)?;
        let (offsets, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_planes.try_to_usize()?)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, data_size.try_to_usize()?)?;
        let data = data.to_vec();
        let result = Image { id, width, height, pitches, offsets, data };
        Ok((result, remaining))
    }
}
impl Serialize for Image {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.id.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        let data_size = u32::try_from(self.data.len()).expect("`data` has too many elements");
        data_size.serialize_into(bytes);
        let num_planes = u32::try_from(self.pitches.len()).expect("`pitches` has too many elements");
        num_planes.serialize_into(bytes);
        self.pitches.serialize_into(bytes);
        assert_eq!(self.offsets.len(), usize::try_from(num_planes).unwrap(), "`offsets` has an incorrect length");
        self.offsets.serialize_into(bytes);
        bytes.extend_from_slice(&self.data);
    }
}
impl Image {
    /// Get the value of the `data_size` field.
    ///
    /// The `data_size` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn data_size(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_planes` field.
    ///
    /// The `num_planes` field is used as the length field of the `pitches` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_planes(&self) -> u32 {
        self.pitches.len()
            .try_into().unwrap()
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
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (min, remaining) = i32::try_parse(remaining)?;
        let (max, remaining) = i32::try_parse(remaining)?;
        let (size, remaining) = u32::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, size.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let result = AttributeInfo { flags, min, max, name };
        Ok((result, remaining))
    }
}
impl Serialize for AttributeInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        self.flags.serialize_into(bytes);
        self.min.serialize_into(bytes);
        self.max.serialize_into(bytes);
        let size = u32::try_from(self.name.len()).expect("`name` has too many elements");
        size.serialize_into(bytes);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl AttributeInfo {
    /// Get the value of the `size` field.
    ///
    /// The `size` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn size(&self) -> u32 {
        self.name.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageFormatInfo {
    pub id: u32,
    pub type_: ImageFormatInfoType,
    pub byte_order: xproto::ImageOrder,
    pub guid: [u8; 16],
    pub bpp: u8,
    pub num_planes: u8,
    pub depth: u8,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub format: ImageFormatInfoFormat,
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
    pub vscanline_order: ScanlineOrder,
}
impl TryParse for ImageFormatInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (id, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u8::try_parse(remaining)?;
        let (byte_order, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (guid, remaining) = crate::x11_utils::parse_u8_list(remaining, 16)?;
        let guid = <[u8; 16]>::try_from(guid).unwrap();
        let (bpp, remaining) = u8::try_parse(remaining)?;
        let (num_planes, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (red_mask, remaining) = u32::try_parse(remaining)?;
        let (green_mask, remaining) = u32::try_parse(remaining)?;
        let (blue_mask, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (y_sample_bits, remaining) = u32::try_parse(remaining)?;
        let (u_sample_bits, remaining) = u32::try_parse(remaining)?;
        let (v_sample_bits, remaining) = u32::try_parse(remaining)?;
        let (vhorz_y_period, remaining) = u32::try_parse(remaining)?;
        let (vhorz_u_period, remaining) = u32::try_parse(remaining)?;
        let (vhorz_v_period, remaining) = u32::try_parse(remaining)?;
        let (vvert_y_period, remaining) = u32::try_parse(remaining)?;
        let (vvert_u_period, remaining) = u32::try_parse(remaining)?;
        let (vvert_v_period, remaining) = u32::try_parse(remaining)?;
        let (vcomp_order, remaining) = crate::x11_utils::parse_u8_list(remaining, 32)?;
        let vcomp_order = <[u8; 32]>::try_from(vcomp_order).unwrap();
        let (vscanline_order, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(11..).ok_or(ParseError::InsufficientData)?;
        let type_ = type_.into();
        let byte_order = byte_order.into();
        let format = format.into();
        let vscanline_order = vscanline_order.into();
        let result = ImageFormatInfo { id, type_, byte_order, guid, bpp, num_planes, depth, red_mask, green_mask, blue_mask, format, y_sample_bits, u_sample_bits, v_sample_bits, vhorz_y_period, vhorz_u_period, vhorz_v_period, vvert_y_period, vvert_u_period, vvert_v_period, vcomp_order, vscanline_order };
        Ok((result, remaining))
    }
}
impl Serialize for ImageFormatInfo {
    type Bytes = [u8; 128];
    fn serialize(&self) -> [u8; 128] {
        let id_bytes = self.id.serialize();
        let type_bytes = u8::from(self.type_).serialize();
        let byte_order_bytes = u8::from(self.byte_order).serialize();
        let bpp_bytes = self.bpp.serialize();
        let num_planes_bytes = self.num_planes.serialize();
        let depth_bytes = self.depth.serialize();
        let red_mask_bytes = self.red_mask.serialize();
        let green_mask_bytes = self.green_mask.serialize();
        let blue_mask_bytes = self.blue_mask.serialize();
        let format_bytes = u8::from(self.format).serialize();
        let y_sample_bits_bytes = self.y_sample_bits.serialize();
        let u_sample_bits_bytes = self.u_sample_bits.serialize();
        let v_sample_bits_bytes = self.v_sample_bits.serialize();
        let vhorz_y_period_bytes = self.vhorz_y_period.serialize();
        let vhorz_u_period_bytes = self.vhorz_u_period.serialize();
        let vhorz_v_period_bytes = self.vhorz_v_period.serialize();
        let vvert_y_period_bytes = self.vvert_y_period.serialize();
        let vvert_u_period_bytes = self.vvert_u_period.serialize();
        let vvert_v_period_bytes = self.vvert_v_period.serialize();
        let vscanline_order_bytes = u8::from(self.vscanline_order).serialize();
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
        u8::from(self.type_).serialize_into(bytes);
        u8::from(self.byte_order).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        bytes.extend_from_slice(&self.guid);
        self.bpp.serialize_into(bytes);
        self.num_planes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.depth.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3]);
        self.red_mask.serialize_into(bytes);
        self.green_mask.serialize_into(bytes);
        self.blue_mask.serialize_into(bytes);
        u8::from(self.format).serialize_into(bytes);
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
        bytes.extend_from_slice(&self.vcomp_order);
        u8::from(self.vscanline_order).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 11]);
    }
}

/// Opcode for the BadPort error
pub const BAD_PORT_ERROR: u8 = 0;

/// Opcode for the BadEncoding error
pub const BAD_ENCODING_ERROR: u8 = 1;

/// Opcode for the BadControl error
pub const BAD_CONTROL_ERROR: u8 = 2;

/// Opcode for the VideoNotify event
pub const VIDEO_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoNotifyEvent {
    pub response_type: u8,
    pub reason: VideoNotifyReason,
    pub sequence: u16,
    pub time: xproto::Timestamp,
    pub drawable: xproto::Drawable,
    pub port: Port,
}
impl TryParse for VideoNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (reason, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (port, remaining) = Port::try_parse(remaining)?;
        let reason = reason.into();
        let result = VideoNotifyEvent { response_type, reason, sequence, time, drawable, port };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl From<&VideoNotifyEvent> for [u8; 32] {
    fn from(input: &VideoNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let reason_bytes = u8::from(input.reason).serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let drawable_bytes = input.drawable.serialize();
        let port_bytes = input.port.serialize();
        [
            response_type_bytes[0],
            reason_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
            // trailing padding
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
    pub time: xproto::Timestamp,
    pub port: Port,
    pub attribute: xproto::Atom,
    pub value: i32,
}
impl TryParse for PortNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (port, remaining) = Port::try_parse(remaining)?;
        let (attribute, remaining) = xproto::Atom::try_parse(remaining)?;
        let (value, remaining) = i32::try_parse(remaining)?;
        let result = PortNotifyEvent { response_type, sequence, time, port, attribute, value };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl From<&PortNotifyEvent> for [u8; 32] {
    fn from(input: &PortNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let port_bytes = input.port.serialize();
        let attribute_bytes = input.attribute.serialize();
        let value_bytes = input.value.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
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
            // trailing padding
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
impl From<PortNotifyEvent> for [u8; 32] {
    fn from(input: PortNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the QueryExtension request
pub const QUERY_EXTENSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtensionRequest;
impl QueryExtensionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_EXTENSION_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryExtensionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_EXTENSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(QueryExtensionRequest
        )
    }
}
impl Request for QueryExtensionRequest {
    type Reply = QueryExtensionReply;
}
pub fn query_extension<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryExtensionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryExtensionRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtensionReply {
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}
impl TryParse for QueryExtensionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major, remaining) = u16::try_parse(remaining)?;
        let (minor, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryExtensionReply { sequence, length, major, minor };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the QueryAdaptors request
pub const QUERY_ADAPTORS_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryAdaptorsRequest {
    pub window: xproto::Window,
}
impl QueryAdaptorsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_ADAPTORS_REQUEST,
            0,
            0,
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryAdaptorsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_ADAPTORS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(QueryAdaptorsRequest {
            window,
        })
    }
}
impl Request for QueryAdaptorsRequest {
    type Reply = QueryAdaptorsReply;
}
pub fn query_adaptors<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, QueryAdaptorsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryAdaptorsRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryAdaptorsReply {
    pub sequence: u16,
    pub length: u32,
    pub info: Vec<AdaptorInfo>,
}
impl TryParse for QueryAdaptorsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_adaptors, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (info, remaining) = crate::x11_utils::parse_list::<AdaptorInfo>(remaining, num_adaptors.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryAdaptorsReply { sequence, length, info };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl QueryAdaptorsReply {
    /// Get the value of the `num_adaptors` field.
    ///
    /// The `num_adaptors` field is used as the length field of the `info` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_adaptors(&self) -> u16 {
        self.info.len()
            .try_into().unwrap()
    }
}

/// Opcode for the QueryEncodings request
pub const QUERY_ENCODINGS_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryEncodingsRequest {
    pub port: Port,
}
impl QueryEncodingsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_ENCODINGS_REQUEST,
            0,
            0,
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryEncodingsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_ENCODINGS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let _ = remaining;
        Ok(QueryEncodingsRequest {
            port,
        })
    }
}
impl Request for QueryEncodingsRequest {
    type Reply = QueryEncodingsReply;
}
pub fn query_encodings<Conn>(conn: &Conn, port: Port) -> Result<Cookie<'_, Conn, QueryEncodingsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryEncodingsRequest {
        port,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEncodingsReply {
    pub sequence: u16,
    pub length: u32,
    pub info: Vec<EncodingInfo>,
}
impl TryParse for QueryEncodingsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_encodings, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        let (info, remaining) = crate::x11_utils::parse_list::<EncodingInfo>(remaining, num_encodings.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryEncodingsReply { sequence, length, info };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl QueryEncodingsReply {
    /// Get the value of the `num_encodings` field.
    ///
    /// The `num_encodings` field is used as the length field of the `info` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_encodings(&self) -> u16 {
        self.info.len()
            .try_into().unwrap()
    }
}

/// Opcode for the GrabPort request
pub const GRAB_PORT_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabPortRequest {
    pub port: Port,
    pub time: xproto::Timestamp,
}
impl GrabPortRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let time_bytes = self.time.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GRAB_PORT_REQUEST,
            0,
            0,
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GrabPortReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GRAB_PORT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let _ = remaining;
        Ok(GrabPortRequest {
            port,
            time,
        })
    }
}
impl Request for GrabPortRequest {
    type Reply = GrabPortReply;
}
pub fn grab_port<Conn, A>(conn: &Conn, port: Port, time: A) -> Result<Cookie<'_, Conn, GrabPortReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = GrabPortRequest {
        port,
        time,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabPortReply {
    pub result: GrabPortStatus,
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for GrabPortReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (result, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = result.into();
        let result = GrabPortReply { result, sequence, length };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the UngrabPort request
pub const UNGRAB_PORT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UngrabPortRequest {
    pub port: Port,
    pub time: xproto::Timestamp,
}
impl UngrabPortRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let time_bytes = self.time.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            UNGRAB_PORT_REQUEST,
            0,
            0,
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
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
        if header.minor_opcode != UNGRAB_PORT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (time, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let _ = remaining;
        Ok(UngrabPortRequest {
            port,
            time,
        })
    }
}
impl Request for UngrabPortRequest {
    type Reply = ();
}
pub fn ungrab_port<Conn, A>(conn: &Conn, port: Port, time: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = UngrabPortRequest {
        port,
        time,
    };
    request0.send(conn)
}

/// Opcode for the PutVideo request
pub const PUT_VIDEO_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PutVideoRequest {
    pub port: Port,
    pub drawable: xproto::Drawable,
    pub gc: xproto::Gcontext,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}
impl PutVideoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let drawable_bytes = self.drawable.serialize();
        let gc_bytes = self.gc.serialize();
        let vid_x_bytes = self.vid_x.serialize();
        let vid_y_bytes = self.vid_y.serialize();
        let vid_w_bytes = self.vid_w.serialize();
        let vid_h_bytes = self.vid_h.serialize();
        let drw_x_bytes = self.drw_x.serialize();
        let drw_y_bytes = self.drw_y.serialize();
        let drw_w_bytes = self.drw_w.serialize();
        let drw_h_bytes = self.drw_h.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            PUT_VIDEO_REQUEST,
            0,
            0,
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
        if header.minor_opcode != PUT_VIDEO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (gc, remaining) = xproto::Gcontext::try_parse(remaining)?;
        let (vid_x, remaining) = i16::try_parse(remaining)?;
        let (vid_y, remaining) = i16::try_parse(remaining)?;
        let (vid_w, remaining) = u16::try_parse(remaining)?;
        let (vid_h, remaining) = u16::try_parse(remaining)?;
        let (drw_x, remaining) = i16::try_parse(remaining)?;
        let (drw_y, remaining) = i16::try_parse(remaining)?;
        let (drw_w, remaining) = u16::try_parse(remaining)?;
        let (drw_h, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(PutVideoRequest {
            port,
            drawable,
            gc,
            vid_x,
            vid_y,
            vid_w,
            vid_h,
            drw_x,
            drw_y,
            drw_w,
            drw_h,
        })
    }
}
impl Request for PutVideoRequest {
    type Reply = ();
}
pub fn put_video<Conn>(conn: &Conn, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PutVideoRequest {
        port,
        drawable,
        gc,
        vid_x,
        vid_y,
        vid_w,
        vid_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
    };
    request0.send(conn)
}

/// Opcode for the PutStill request
pub const PUT_STILL_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PutStillRequest {
    pub port: Port,
    pub drawable: xproto::Drawable,
    pub gc: xproto::Gcontext,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}
impl PutStillRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let drawable_bytes = self.drawable.serialize();
        let gc_bytes = self.gc.serialize();
        let vid_x_bytes = self.vid_x.serialize();
        let vid_y_bytes = self.vid_y.serialize();
        let vid_w_bytes = self.vid_w.serialize();
        let vid_h_bytes = self.vid_h.serialize();
        let drw_x_bytes = self.drw_x.serialize();
        let drw_y_bytes = self.drw_y.serialize();
        let drw_w_bytes = self.drw_w.serialize();
        let drw_h_bytes = self.drw_h.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            PUT_STILL_REQUEST,
            0,
            0,
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
        if header.minor_opcode != PUT_STILL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (gc, remaining) = xproto::Gcontext::try_parse(remaining)?;
        let (vid_x, remaining) = i16::try_parse(remaining)?;
        let (vid_y, remaining) = i16::try_parse(remaining)?;
        let (vid_w, remaining) = u16::try_parse(remaining)?;
        let (vid_h, remaining) = u16::try_parse(remaining)?;
        let (drw_x, remaining) = i16::try_parse(remaining)?;
        let (drw_y, remaining) = i16::try_parse(remaining)?;
        let (drw_w, remaining) = u16::try_parse(remaining)?;
        let (drw_h, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(PutStillRequest {
            port,
            drawable,
            gc,
            vid_x,
            vid_y,
            vid_w,
            vid_h,
            drw_x,
            drw_y,
            drw_w,
            drw_h,
        })
    }
}
impl Request for PutStillRequest {
    type Reply = ();
}
pub fn put_still<Conn>(conn: &Conn, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PutStillRequest {
        port,
        drawable,
        gc,
        vid_x,
        vid_y,
        vid_w,
        vid_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
    };
    request0.send(conn)
}

/// Opcode for the GetVideo request
pub const GET_VIDEO_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVideoRequest {
    pub port: Port,
    pub drawable: xproto::Drawable,
    pub gc: xproto::Gcontext,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}
impl GetVideoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let drawable_bytes = self.drawable.serialize();
        let gc_bytes = self.gc.serialize();
        let vid_x_bytes = self.vid_x.serialize();
        let vid_y_bytes = self.vid_y.serialize();
        let vid_w_bytes = self.vid_w.serialize();
        let vid_h_bytes = self.vid_h.serialize();
        let drw_x_bytes = self.drw_x.serialize();
        let drw_y_bytes = self.drw_y.serialize();
        let drw_w_bytes = self.drw_w.serialize();
        let drw_h_bytes = self.drw_h.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_VIDEO_REQUEST,
            0,
            0,
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
        if header.minor_opcode != GET_VIDEO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (gc, remaining) = xproto::Gcontext::try_parse(remaining)?;
        let (vid_x, remaining) = i16::try_parse(remaining)?;
        let (vid_y, remaining) = i16::try_parse(remaining)?;
        let (vid_w, remaining) = u16::try_parse(remaining)?;
        let (vid_h, remaining) = u16::try_parse(remaining)?;
        let (drw_x, remaining) = i16::try_parse(remaining)?;
        let (drw_y, remaining) = i16::try_parse(remaining)?;
        let (drw_w, remaining) = u16::try_parse(remaining)?;
        let (drw_h, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetVideoRequest {
            port,
            drawable,
            gc,
            vid_x,
            vid_y,
            vid_w,
            vid_h,
            drw_x,
            drw_y,
            drw_w,
            drw_h,
        })
    }
}
impl Request for GetVideoRequest {
    type Reply = ();
}
pub fn get_video<Conn>(conn: &Conn, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetVideoRequest {
        port,
        drawable,
        gc,
        vid_x,
        vid_y,
        vid_w,
        vid_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
    };
    request0.send(conn)
}

/// Opcode for the GetStill request
pub const GET_STILL_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStillRequest {
    pub port: Port,
    pub drawable: xproto::Drawable,
    pub gc: xproto::Gcontext,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}
impl GetStillRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let drawable_bytes = self.drawable.serialize();
        let gc_bytes = self.gc.serialize();
        let vid_x_bytes = self.vid_x.serialize();
        let vid_y_bytes = self.vid_y.serialize();
        let vid_w_bytes = self.vid_w.serialize();
        let vid_h_bytes = self.vid_h.serialize();
        let drw_x_bytes = self.drw_x.serialize();
        let drw_y_bytes = self.drw_y.serialize();
        let drw_w_bytes = self.drw_w.serialize();
        let drw_h_bytes = self.drw_h.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_STILL_REQUEST,
            0,
            0,
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
        if header.minor_opcode != GET_STILL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (gc, remaining) = xproto::Gcontext::try_parse(remaining)?;
        let (vid_x, remaining) = i16::try_parse(remaining)?;
        let (vid_y, remaining) = i16::try_parse(remaining)?;
        let (vid_w, remaining) = u16::try_parse(remaining)?;
        let (vid_h, remaining) = u16::try_parse(remaining)?;
        let (drw_x, remaining) = i16::try_parse(remaining)?;
        let (drw_y, remaining) = i16::try_parse(remaining)?;
        let (drw_w, remaining) = u16::try_parse(remaining)?;
        let (drw_h, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetStillRequest {
            port,
            drawable,
            gc,
            vid_x,
            vid_y,
            vid_w,
            vid_h,
            drw_x,
            drw_y,
            drw_w,
            drw_h,
        })
    }
}
impl Request for GetStillRequest {
    type Reply = ();
}
pub fn get_still<Conn>(conn: &Conn, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetStillRequest {
        port,
        drawable,
        gc,
        vid_x,
        vid_y,
        vid_w,
        vid_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
    };
    request0.send(conn)
}

/// Opcode for the StopVideo request
pub const STOP_VIDEO_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StopVideoRequest {
    pub port: Port,
    pub drawable: xproto::Drawable,
}
impl StopVideoRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            STOP_VIDEO_REQUEST,
            0,
            0,
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
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
        if header.minor_opcode != STOP_VIDEO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let _ = remaining;
        Ok(StopVideoRequest {
            port,
            drawable,
        })
    }
}
impl Request for StopVideoRequest {
    type Reply = ();
}
pub fn stop_video<Conn>(conn: &Conn, port: Port, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = StopVideoRequest {
        port,
        drawable,
    };
    request0.send(conn)
}

/// Opcode for the SelectVideoNotify request
pub const SELECT_VIDEO_NOTIFY_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectVideoNotifyRequest {
    pub drawable: xproto::Drawable,
    pub onoff: bool,
}
impl SelectVideoNotifyRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let onoff_bytes = self.onoff.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SELECT_VIDEO_NOTIFY_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            onoff_bytes[0],
            0,
            0,
            0,
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
        if header.minor_opcode != SELECT_VIDEO_NOTIFY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (onoff, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(SelectVideoNotifyRequest {
            drawable,
            onoff,
        })
    }
}
impl Request for SelectVideoNotifyRequest {
    type Reply = ();
}
pub fn select_video_notify<Conn>(conn: &Conn, drawable: xproto::Drawable, onoff: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectVideoNotifyRequest {
        drawable,
        onoff,
    };
    request0.send(conn)
}

/// Opcode for the SelectPortNotify request
pub const SELECT_PORT_NOTIFY_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectPortNotifyRequest {
    pub port: Port,
    pub onoff: bool,
}
impl SelectPortNotifyRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let onoff_bytes = self.onoff.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SELECT_PORT_NOTIFY_REQUEST,
            0,
            0,
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
            onoff_bytes[0],
            0,
            0,
            0,
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
        if header.minor_opcode != SELECT_PORT_NOTIFY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (onoff, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(SelectPortNotifyRequest {
            port,
            onoff,
        })
    }
}
impl Request for SelectPortNotifyRequest {
    type Reply = ();
}
pub fn select_port_notify<Conn>(conn: &Conn, port: Port, onoff: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectPortNotifyRequest {
        port,
        onoff,
    };
    request0.send(conn)
}

/// Opcode for the QueryBestSize request
pub const QUERY_BEST_SIZE_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryBestSizeRequest {
    pub port: Port,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub motion: bool,
}
impl QueryBestSizeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let vid_w_bytes = self.vid_w.serialize();
        let vid_h_bytes = self.vid_h.serialize();
        let drw_w_bytes = self.drw_w.serialize();
        let drw_h_bytes = self.drw_h.serialize();
        let motion_bytes = self.motion.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_BEST_SIZE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryBestSizeReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_BEST_SIZE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (vid_w, remaining) = u16::try_parse(remaining)?;
        let (vid_h, remaining) = u16::try_parse(remaining)?;
        let (drw_w, remaining) = u16::try_parse(remaining)?;
        let (drw_h, remaining) = u16::try_parse(remaining)?;
        let (motion, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(QueryBestSizeRequest {
            port,
            vid_w,
            vid_h,
            drw_w,
            drw_h,
            motion,
        })
    }
}
impl Request for QueryBestSizeRequest {
    type Reply = QueryBestSizeReply;
}
pub fn query_best_size<Conn>(conn: &Conn, port: Port, vid_w: u16, vid_h: u16, drw_w: u16, drw_h: u16, motion: bool) -> Result<Cookie<'_, Conn, QueryBestSizeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryBestSizeRequest {
        port,
        vid_w,
        vid_h,
        drw_w,
        drw_h,
        motion,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryBestSizeReply {
    pub sequence: u16,
    pub length: u32,
    pub actual_width: u16,
    pub actual_height: u16,
}
impl TryParse for QueryBestSizeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (actual_width, remaining) = u16::try_parse(remaining)?;
        let (actual_height, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryBestSizeReply { sequence, length, actual_width, actual_height };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetPortAttribute request
pub const SET_PORT_ATTRIBUTE_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPortAttributeRequest {
    pub port: Port,
    pub attribute: xproto::Atom,
    pub value: i32,
}
impl SetPortAttributeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let attribute_bytes = self.attribute.serialize();
        let value_bytes = self.value.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_PORT_ATTRIBUTE_REQUEST,
            0,
            0,
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
        if header.minor_opcode != SET_PORT_ATTRIBUTE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (attribute, remaining) = xproto::Atom::try_parse(remaining)?;
        let (value, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetPortAttributeRequest {
            port,
            attribute,
            value,
        })
    }
}
impl Request for SetPortAttributeRequest {
    type Reply = ();
}
pub fn set_port_attribute<Conn>(conn: &Conn, port: Port, attribute: xproto::Atom, value: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPortAttributeRequest {
        port,
        attribute,
        value,
    };
    request0.send(conn)
}

/// Opcode for the GetPortAttribute request
pub const GET_PORT_ATTRIBUTE_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPortAttributeRequest {
    pub port: Port,
    pub attribute: xproto::Atom,
}
impl GetPortAttributeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let attribute_bytes = self.attribute.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_PORT_ATTRIBUTE_REQUEST,
            0,
            0,
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
            attribute_bytes[0],
            attribute_bytes[1],
            attribute_bytes[2],
            attribute_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetPortAttributeReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PORT_ATTRIBUTE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (attribute, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPortAttributeRequest {
            port,
            attribute,
        })
    }
}
impl Request for GetPortAttributeRequest {
    type Reply = GetPortAttributeReply;
}
pub fn get_port_attribute<Conn>(conn: &Conn, port: Port, attribute: xproto::Atom) -> Result<Cookie<'_, Conn, GetPortAttributeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPortAttributeRequest {
        port,
        attribute,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPortAttributeReply {
    pub sequence: u16,
    pub length: u32,
    pub value: i32,
}
impl TryParse for GetPortAttributeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (value, remaining) = i32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPortAttributeReply { sequence, length, value };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the QueryPortAttributes request
pub const QUERY_PORT_ATTRIBUTES_REQUEST: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryPortAttributesRequest {
    pub port: Port,
}
impl QueryPortAttributesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_PORT_ATTRIBUTES_REQUEST,
            0,
            0,
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryPortAttributesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_PORT_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let _ = remaining;
        Ok(QueryPortAttributesRequest {
            port,
        })
    }
}
impl Request for QueryPortAttributesRequest {
    type Reply = QueryPortAttributesReply;
}
pub fn query_port_attributes<Conn>(conn: &Conn, port: Port) -> Result<Cookie<'_, Conn, QueryPortAttributesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryPortAttributesRequest {
        port,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryPortAttributesReply {
    pub sequence: u16,
    pub length: u32,
    pub text_size: u32,
    pub attributes: Vec<AttributeInfo>,
}
impl TryParse for QueryPortAttributesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_attributes, remaining) = u32::try_parse(remaining)?;
        let (text_size, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (attributes, remaining) = crate::x11_utils::parse_list::<AttributeInfo>(remaining, num_attributes.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryPortAttributesReply { sequence, length, text_size, attributes };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl QueryPortAttributesReply {
    /// Get the value of the `num_attributes` field.
    ///
    /// The `num_attributes` field is used as the length field of the `attributes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_attributes(&self) -> u32 {
        self.attributes.len()
            .try_into().unwrap()
    }
}

/// Opcode for the ListImageFormats request
pub const LIST_IMAGE_FORMATS_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListImageFormatsRequest {
    pub port: Port,
}
impl ListImageFormatsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            LIST_IMAGE_FORMATS_REQUEST,
            0,
            0,
            port_bytes[0],
            port_bytes[1],
            port_bytes[2],
            port_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListImageFormatsReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_IMAGE_FORMATS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let _ = remaining;
        Ok(ListImageFormatsRequest {
            port,
        })
    }
}
impl Request for ListImageFormatsRequest {
    type Reply = ListImageFormatsReply;
}
pub fn list_image_formats<Conn>(conn: &Conn, port: Port) -> Result<Cookie<'_, Conn, ListImageFormatsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListImageFormatsRequest {
        port,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListImageFormatsReply {
    pub sequence: u16,
    pub length: u32,
    pub format: Vec<ImageFormatInfo>,
}
impl TryParse for ListImageFormatsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_formats, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (format, remaining) = crate::x11_utils::parse_list::<ImageFormatInfo>(remaining, num_formats.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListImageFormatsReply { sequence, length, format };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl ListImageFormatsReply {
    /// Get the value of the `num_formats` field.
    ///
    /// The `num_formats` field is used as the length field of the `format` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_formats(&self) -> u32 {
        self.format.len()
            .try_into().unwrap()
    }
}

/// Opcode for the QueryImageAttributes request
pub const QUERY_IMAGE_ATTRIBUTES_REQUEST: u8 = 17;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryImageAttributesRequest {
    pub port: Port,
    pub id: u32,
    pub width: u16,
    pub height: u16,
}
impl QueryImageAttributesRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let id_bytes = self.id.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_IMAGE_ATTRIBUTES_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryImageAttributesReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_IMAGE_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (id, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryImageAttributesRequest {
            port,
            id,
            width,
            height,
        })
    }
}
impl Request for QueryImageAttributesRequest {
    type Reply = QueryImageAttributesReply;
}
pub fn query_image_attributes<Conn>(conn: &Conn, port: Port, id: u32, width: u16, height: u16) -> Result<Cookie<'_, Conn, QueryImageAttributesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryImageAttributesRequest {
        port,
        id,
        width,
        height,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryImageAttributesReply {
    pub sequence: u16,
    pub length: u32,
    pub data_size: u32,
    pub width: u16,
    pub height: u16,
    pub pitches: Vec<u32>,
    pub offsets: Vec<u32>,
}
impl TryParse for QueryImageAttributesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_planes, remaining) = u32::try_parse(remaining)?;
        let (data_size, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (pitches, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_planes.try_to_usize()?)?;
        let (offsets, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_planes.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryImageAttributesReply { sequence, length, data_size, width, height, pitches, offsets };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl QueryImageAttributesReply {
    /// Get the value of the `num_planes` field.
    ///
    /// The `num_planes` field is used as the length field of the `pitches` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_planes(&self) -> u32 {
        self.pitches.len()
            .try_into().unwrap()
    }
}

/// Opcode for the PutImage request
pub const PUT_IMAGE_REQUEST: u8 = 18;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutImageRequest<'input> {
    pub port: Port,
    pub drawable: xproto::Drawable,
    pub gc: xproto::Gcontext,
    pub id: u32,
    pub src_x: i16,
    pub src_y: i16,
    pub src_w: u16,
    pub src_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub width: u16,
    pub height: u16,
    pub data: Cow<'input, [u8]>,
}
impl<'input> PutImageRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let drawable_bytes = self.drawable.serialize();
        let gc_bytes = self.gc.serialize();
        let id_bytes = self.id.serialize();
        let src_x_bytes = self.src_x.serialize();
        let src_y_bytes = self.src_y.serialize();
        let src_w_bytes = self.src_w.serialize();
        let src_h_bytes = self.src_h.serialize();
        let drw_x_bytes = self.drw_x.serialize();
        let drw_y_bytes = self.drw_y.serialize();
        let drw_w_bytes = self.drw_w.serialize();
        let drw_h_bytes = self.drw_h.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            PUT_IMAGE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.data.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), self.data, padding0.into()], vec![]))
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
        if header.minor_opcode != PUT_IMAGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (gc, remaining) = xproto::Gcontext::try_parse(remaining)?;
        let (id, remaining) = u32::try_parse(remaining)?;
        let (src_x, remaining) = i16::try_parse(remaining)?;
        let (src_y, remaining) = i16::try_parse(remaining)?;
        let (src_w, remaining) = u16::try_parse(remaining)?;
        let (src_h, remaining) = u16::try_parse(remaining)?;
        let (drw_x, remaining) = i16::try_parse(remaining)?;
        let (drw_y, remaining) = i16::try_parse(remaining)?;
        let (drw_w, remaining) = u16::try_parse(remaining)?;
        let (drw_h, remaining) = u16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = remaining.split_at(remaining.len());
        let _ = remaining;
        Ok(PutImageRequest {
            port,
            drawable,
            gc,
            id,
            src_x,
            src_y,
            src_w,
            src_h,
            drw_x,
            drw_y,
            drw_w,
            drw_h,
            width,
            height,
            data: Cow::Borrowed(data),
        })
    }
    /// Clone all borrowed data in this PutImageRequest.
    pub fn into_owned(self) -> PutImageRequest<'static> {
        PutImageRequest {
            port: self.port,
            drawable: self.drawable,
            gc: self.gc,
            id: self.id,
            src_x: self.src_x,
            src_y: self.src_y,
            src_w: self.src_w,
            src_h: self.src_h,
            drw_x: self.drw_x,
            drw_y: self.drw_y,
            drw_w: self.drw_w,
            drw_h: self.drw_h,
            width: self.width,
            height: self.height,
            data: Cow::Owned(self.data.into_owned()),
        }
    }
}
impl<'input> Request for PutImageRequest<'input> {
    type Reply = ();
}
pub fn put_image<'c, 'input, Conn>(conn: &'c Conn, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, id: u32, src_x: i16, src_y: i16, src_w: u16, src_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16, width: u16, height: u16, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PutImageRequest {
        port,
        drawable,
        gc,
        id,
        src_x,
        src_y,
        src_w,
        src_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
        width,
        height,
        data: Cow::Borrowed(data),
    };
    request0.send(conn)
}

/// Opcode for the ShmPutImage request
pub const SHM_PUT_IMAGE_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShmPutImageRequest {
    pub port: Port,
    pub drawable: xproto::Drawable,
    pub gc: xproto::Gcontext,
    pub shmseg: shm::Seg,
    pub id: u32,
    pub offset: u32,
    pub src_x: i16,
    pub src_y: i16,
    pub src_w: u16,
    pub src_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub width: u16,
    pub height: u16,
    pub send_event: u8,
}
impl ShmPutImageRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let port_bytes = self.port.serialize();
        let drawable_bytes = self.drawable.serialize();
        let gc_bytes = self.gc.serialize();
        let shmseg_bytes = self.shmseg.serialize();
        let id_bytes = self.id.serialize();
        let offset_bytes = self.offset.serialize();
        let src_x_bytes = self.src_x.serialize();
        let src_y_bytes = self.src_y.serialize();
        let src_w_bytes = self.src_w.serialize();
        let src_h_bytes = self.src_h.serialize();
        let drw_x_bytes = self.drw_x.serialize();
        let drw_y_bytes = self.drw_y.serialize();
        let drw_w_bytes = self.drw_w.serialize();
        let drw_h_bytes = self.drw_h.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let send_event_bytes = self.send_event.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SHM_PUT_IMAGE_REQUEST,
            0,
            0,
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
        if header.minor_opcode != SHM_PUT_IMAGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (port, remaining) = Port::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (gc, remaining) = xproto::Gcontext::try_parse(remaining)?;
        let (shmseg, remaining) = shm::Seg::try_parse(remaining)?;
        let (id, remaining) = u32::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let (src_x, remaining) = i16::try_parse(remaining)?;
        let (src_y, remaining) = i16::try_parse(remaining)?;
        let (src_w, remaining) = u16::try_parse(remaining)?;
        let (src_h, remaining) = u16::try_parse(remaining)?;
        let (drw_x, remaining) = i16::try_parse(remaining)?;
        let (drw_y, remaining) = i16::try_parse(remaining)?;
        let (drw_w, remaining) = u16::try_parse(remaining)?;
        let (drw_h, remaining) = u16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (send_event, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(ShmPutImageRequest {
            port,
            drawable,
            gc,
            shmseg,
            id,
            offset,
            src_x,
            src_y,
            src_w,
            src_h,
            drw_x,
            drw_y,
            drw_w,
            drw_h,
            width,
            height,
            send_event,
        })
    }
}
impl Request for ShmPutImageRequest {
    type Reply = ();
}
pub fn shm_put_image<Conn>(conn: &Conn, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, shmseg: shm::Seg, id: u32, offset: u32, src_x: i16, src_y: i16, src_w: u16, src_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16, width: u16, height: u16, send_event: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ShmPutImageRequest {
        port,
        drawable,
        gc,
        shmseg,
        id,
        offset,
        src_x,
        src_y,
        src_w,
        src_h,
        drw_x,
        drw_y,
        drw_w,
        drw_h,
        width,
        height,
        send_event,
    };
    request0.send(conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xv_query_extension(&self) -> Result<Cookie<'_, Self, QueryExtensionReply>, ConnectionError>
    {
        query_extension(self)
    }
    fn xv_query_adaptors(&self, window: xproto::Window) -> Result<Cookie<'_, Self, QueryAdaptorsReply>, ConnectionError>
    {
        query_adaptors(self, window)
    }
    fn xv_query_encodings(&self, port: Port) -> Result<Cookie<'_, Self, QueryEncodingsReply>, ConnectionError>
    {
        query_encodings(self, port)
    }
    fn xv_grab_port<A>(&self, port: Port, time: A) -> Result<Cookie<'_, Self, GrabPortReply>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
    {
        grab_port(self, port, time)
    }
    fn xv_ungrab_port<A>(&self, port: Port, time: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
    {
        ungrab_port(self, port, time)
    }
    fn xv_put_video(&self, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        put_video(self, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h)
    }
    fn xv_put_still(&self, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        put_still(self, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h)
    }
    fn xv_get_video(&self, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        get_video(self, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h)
    }
    fn xv_get_still(&self, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, vid_x: i16, vid_y: i16, vid_w: u16, vid_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        get_still(self, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h)
    }
    fn xv_stop_video(&self, port: Port, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        stop_video(self, port, drawable)
    }
    fn xv_select_video_notify(&self, drawable: xproto::Drawable, onoff: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_video_notify(self, drawable, onoff)
    }
    fn xv_select_port_notify(&self, port: Port, onoff: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_port_notify(self, port, onoff)
    }
    fn xv_query_best_size(&self, port: Port, vid_w: u16, vid_h: u16, drw_w: u16, drw_h: u16, motion: bool) -> Result<Cookie<'_, Self, QueryBestSizeReply>, ConnectionError>
    {
        query_best_size(self, port, vid_w, vid_h, drw_w, drw_h, motion)
    }
    fn xv_set_port_attribute(&self, port: Port, attribute: xproto::Atom, value: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_port_attribute(self, port, attribute, value)
    }
    fn xv_get_port_attribute(&self, port: Port, attribute: xproto::Atom) -> Result<Cookie<'_, Self, GetPortAttributeReply>, ConnectionError>
    {
        get_port_attribute(self, port, attribute)
    }
    fn xv_query_port_attributes(&self, port: Port) -> Result<Cookie<'_, Self, QueryPortAttributesReply>, ConnectionError>
    {
        query_port_attributes(self, port)
    }
    fn xv_list_image_formats(&self, port: Port) -> Result<Cookie<'_, Self, ListImageFormatsReply>, ConnectionError>
    {
        list_image_formats(self, port)
    }
    fn xv_query_image_attributes(&self, port: Port, id: u32, width: u16, height: u16) -> Result<Cookie<'_, Self, QueryImageAttributesReply>, ConnectionError>
    {
        query_image_attributes(self, port, id, width, height)
    }
    fn xv_put_image<'c, 'input>(&'c self, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, id: u32, src_x: i16, src_y: i16, src_w: u16, src_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16, width: u16, height: u16, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        put_image(self, port, drawable, gc, id, src_x, src_y, src_w, src_h, drw_x, drw_y, drw_w, drw_h, width, height, data)
    }
    fn xv_shm_put_image(&self, port: Port, drawable: xproto::Drawable, gc: xproto::Gcontext, shmseg: shm::Seg, id: u32, offset: u32, src_x: i16, src_y: i16, src_w: u16, src_h: u16, drw_x: i16, drw_y: i16, drw_w: u16, drw_h: u16, width: u16, height: u16, send_event: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        shm_put_image(self, port, drawable, gc, shmseg, id, offset, src_x, src_y, src_w, src_h, drw_x, drw_y, drw_w, drw_h, width, height, send_event)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
