// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Glx` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use alloc::borrow::Cow;
#[allow(unused_imports)]
use core::convert::TryInto;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
use crate::errors::ParseError;
#[allow(unused_imports)]
use crate::x11_utils::TryIntoUSize;
use crate::{BufWithFds, PiecewiseBuf};
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
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


/// Opcode for the BadContext error
pub const BAD_CONTEXT_ERROR: u8 = 0;

/// Opcode for the BadContextState error
pub const BAD_CONTEXT_STATE_ERROR: u8 = 1;

/// Opcode for the BadDrawable error
pub const BAD_DRAWABLE_ERROR: u8 = 2;

/// Opcode for the BadPixmap error
pub const BAD_PIXMAP_ERROR: u8 = 3;

/// Opcode for the BadContextTag error
pub const BAD_CONTEXT_TAG_ERROR: u8 = 4;

/// Opcode for the BadCurrentWindow error
pub const BAD_CURRENT_WINDOW_ERROR: u8 = 5;

/// Opcode for the BadRenderRequest error
pub const BAD_RENDER_REQUEST_ERROR: u8 = 6;

/// Opcode for the BadLargeRequest error
pub const BAD_LARGE_REQUEST_ERROR: u8 = 7;

/// Opcode for the UnsupportedPrivateRequest error
pub const UNSUPPORTED_PRIVATE_REQUEST_ERROR: u8 = 8;

/// Opcode for the BadFBConfig error
pub const BAD_FB_CONFIG_ERROR: u8 = 9;

/// Opcode for the BadPbuffer error
pub const BAD_PBUFFER_ERROR: u8 = 10;

/// Opcode for the BadCurrentDrawable error
pub const BAD_CURRENT_DRAWABLE_ERROR: u8 = 11;

/// Opcode for the BadWindow error
pub const BAD_WINDOW_ERROR: u8 = 12;

/// Opcode for the GLXBadProfileARB error
pub const GLX_BAD_PROFILE_ARB_ERROR: u8 = 13;

/// Opcode for the PbufferClobber event
pub const PBUFFER_CLOBBER_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for PbufferClobberEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
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
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let result = PbufferClobberEvent { response_type, sequence, event_type, draw_type, drawable, b_mask, aux_buffer, x, y, width, height, count };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for PbufferClobberEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let event_type_bytes = self.event_type.serialize();
        let draw_type_bytes = self.draw_type.serialize();
        let drawable_bytes = self.drawable.serialize();
        let b_mask_bytes = self.b_mask.serialize();
        let aux_buffer_bytes = self.aux_buffer.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let count_bytes = self.count.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            event_type_bytes[0],
            event_type_bytes[1],
            draw_type_bytes[0],
            draw_type_bytes[1],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            b_mask_bytes[0],
            b_mask_bytes[1],
            b_mask_bytes[2],
            b_mask_bytes[3],
            aux_buffer_bytes[0],
            aux_buffer_bytes[1],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            count_bytes[0],
            count_bytes[1],
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        self.draw_type.serialize_into(bytes);
        self.drawable.serialize_into(bytes);
        self.b_mask.serialize_into(bytes);
        self.aux_buffer.serialize_into(bytes);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.count.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
    }
}
#[cfg(test)]
mod pbuffer_clobber_event {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for PbufferClobberEvent {
        fn generate(rng: &Rng) -> Self {
            let response_type: u8 = GenRandom::generate(rng);
            let sequence: u16 = GenRandom::generate(rng);
            let event_type: u16 = GenRandom::generate(rng);
            let draw_type: u16 = GenRandom::generate(rng);
            let drawable: Drawable = GenRandom::generate(rng);
            let b_mask: u32 = GenRandom::generate(rng);
            let aux_buffer: u16 = GenRandom::generate(rng);
            let x: u16 = GenRandom::generate(rng);
            let y: u16 = GenRandom::generate(rng);
            let width: u16 = GenRandom::generate(rng);
            let height: u16 = GenRandom::generate(rng);
            let count: u16 = GenRandom::generate(rng);
            Self {
                response_type,
                sequence,
                event_type,
                draw_type,
                drawable,
                b_mask,
                aux_buffer,
                x,
                y,
                width,
                height,
                count,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12924686000395780096);
        let value = PbufferClobberEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&PbufferClobberEvent> for [u8; 32] {
    fn from(input: &PbufferClobberEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let event_type_bytes = input.event_type.serialize();
        let draw_type_bytes = input.draw_type.serialize();
        let drawable_bytes = input.drawable.serialize();
        let b_mask_bytes = input.b_mask.serialize();
        let aux_buffer_bytes = input.aux_buffer.serialize();
        let x_bytes = input.x.serialize();
        let y_bytes = input.y.serialize();
        let width_bytes = input.width.serialize();
        let height_bytes = input.height.serialize();
        let count_bytes = input.count.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            event_type_bytes[0],
            event_type_bytes[1],
            draw_type_bytes[0],
            draw_type_bytes[1],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            b_mask_bytes[0],
            b_mask_bytes[1],
            b_mask_bytes[2],
            b_mask_bytes[3],
            aux_buffer_bytes[0],
            aux_buffer_bytes[1],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            count_bytes[0],
            count_bytes[1],
            0,
            0,
            0,
            0,
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl TryParse for BufferSwapCompleteEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (drawable, remaining) = Drawable::try_parse(remaining)?;
        let (ust_hi, remaining) = u32::try_parse(remaining)?;
        let (ust_lo, remaining) = u32::try_parse(remaining)?;
        let (msc_hi, remaining) = u32::try_parse(remaining)?;
        let (msc_lo, remaining) = u32::try_parse(remaining)?;
        let (sbc, remaining) = u32::try_parse(remaining)?;
        let result = BufferSwapCompleteEvent { response_type, sequence, event_type, drawable, ust_hi, ust_lo, msc_hi, msc_lo, sbc };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for BufferSwapCompleteEvent {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = self.response_type.serialize();
        let sequence_bytes = self.sequence.serialize();
        let event_type_bytes = self.event_type.serialize();
        let drawable_bytes = self.drawable.serialize();
        let ust_hi_bytes = self.ust_hi.serialize();
        let ust_lo_bytes = self.ust_lo.serialize();
        let msc_hi_bytes = self.msc_hi.serialize();
        let msc_lo_bytes = self.msc_lo.serialize();
        let sbc_bytes = self.sbc.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            event_type_bytes[0],
            event_type_bytes[1],
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            ust_hi_bytes[0],
            ust_hi_bytes[1],
            ust_hi_bytes[2],
            ust_hi_bytes[3],
            ust_lo_bytes[0],
            ust_lo_bytes[1],
            ust_lo_bytes[2],
            ust_lo_bytes[3],
            msc_hi_bytes[0],
            msc_hi_bytes[1],
            msc_hi_bytes[2],
            msc_hi_bytes[3],
            msc_lo_bytes[0],
            msc_lo_bytes[1],
            msc_lo_bytes[2],
            msc_lo_bytes[3],
            sbc_bytes[0],
            sbc_bytes[1],
            sbc_bytes[2],
            sbc_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        self.response_type.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.event_type.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.drawable.serialize_into(bytes);
        self.ust_hi.serialize_into(bytes);
        self.ust_lo.serialize_into(bytes);
        self.msc_hi.serialize_into(bytes);
        self.msc_lo.serialize_into(bytes);
        self.sbc.serialize_into(bytes);
    }
}
#[cfg(test)]
mod buffer_swap_complete_event {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for BufferSwapCompleteEvent {
        fn generate(rng: &Rng) -> Self {
            let response_type: u8 = GenRandom::generate(rng);
            let sequence: u16 = GenRandom::generate(rng);
            let event_type: u16 = GenRandom::generate(rng);
            let drawable: Drawable = GenRandom::generate(rng);
            let ust_hi: u32 = GenRandom::generate(rng);
            let ust_lo: u32 = GenRandom::generate(rng);
            let msc_hi: u32 = GenRandom::generate(rng);
            let msc_lo: u32 = GenRandom::generate(rng);
            let sbc: u32 = GenRandom::generate(rng);
            Self {
                response_type,
                sequence,
                event_type,
                drawable,
                ust_hi,
                ust_lo,
                msc_hi,
                msc_lo,
                sbc,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10394221322546708480);
        let value = BufferSwapCompleteEvent::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}
impl From<&BufferSwapCompleteEvent> for [u8; 32] {
    fn from(input: &BufferSwapCompleteEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let event_type_bytes = input.event_type.serialize();
        let drawable_bytes = input.drawable.serialize();
        let ust_hi_bytes = input.ust_hi.serialize();
        let ust_lo_bytes = input.ust_lo.serialize();
        let msc_hi_bytes = input.msc_hi.serialize();
        let msc_lo_bytes = input.msc_lo.serialize();
        let sbc_bytes = input.sbc.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            event_type_bytes[0],
            event_type_bytes[1],
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            ust_hi_bytes[0],
            ust_hi_bytes[1],
            ust_hi_bytes[2],
            ust_hi_bytes[3],
            ust_lo_bytes[0],
            ust_lo_bytes[1],
            ust_lo_bytes[2],
            ust_lo_bytes[3],
            msc_hi_bytes[0],
            msc_hi_bytes[1],
            msc_hi_bytes[2],
            msc_hi_bytes[3],
            msc_lo_bytes[0],
            msc_lo_bytes[1],
            msc_lo_bytes[2],
            msc_lo_bytes[3],
            sbc_bytes[0],
            sbc_bytes[1],
            sbc_bytes[2],
            sbc_bytes[3],
        ]
    }
}
impl From<BufferSwapCompleteEvent> for [u8; 32] {
    fn from(input: BufferSwapCompleteEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PBCET(u16);
impl PBCET {
    pub const DAMAGED: Self = Self(32791);
    pub const SAVED: Self = Self(32792);
}
impl From<PBCET> for u16 {
    #[inline]
    fn from(input: PBCET) -> Self {
        input.0
    }
}
impl From<PBCET> for Option<u16> {
    #[inline]
    fn from(input: PBCET) -> Self {
        Some(input.0)
    }
}
impl From<PBCET> for u32 {
    #[inline]
    fn from(input: PBCET) -> Self {
        u32::from(input.0)
    }
}
impl From<PBCET> for Option<u32> {
    #[inline]
    fn from(input: PBCET) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for PBCET {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for PBCET {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for PBCET  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::DAMAGED.0.into(), "DAMAGED", "Damaged"),
            (Self::SAVED.0.into(), "SAVED", "Saved"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for PBCET {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::DAMAGED.0,
            Self::SAVED.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PBCDT(u16);
impl PBCDT {
    pub const WINDOW: Self = Self(32793);
    pub const PBUFFER: Self = Self(32794);
}
impl From<PBCDT> for u16 {
    #[inline]
    fn from(input: PBCDT) -> Self {
        input.0
    }
}
impl From<PBCDT> for Option<u16> {
    #[inline]
    fn from(input: PBCDT) -> Self {
        Some(input.0)
    }
}
impl From<PBCDT> for u32 {
    #[inline]
    fn from(input: PBCDT) -> Self {
        u32::from(input.0)
    }
}
impl From<PBCDT> for Option<u32> {
    #[inline]
    fn from(input: PBCDT) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for PBCDT {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for PBCDT {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for PBCDT  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::WINDOW.0.into(), "WINDOW", "Window"),
            (Self::PBUFFER.0.into(), "PBUFFER", "Pbuffer"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for PBCDT {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::WINDOW.0,
            Self::PBUFFER.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the Render request
pub const RENDER_REQUEST: u8 = 1;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderRequest<'input> {
    pub context_tag: ContextTag,
    pub data: Cow<'input, [u8]>,
}
impl<'input> RenderRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            RENDER_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.data.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.data, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != RENDER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (data, remaining) = remaining.split_at(remaining.len());
        let _ = remaining;
        Ok(RenderRequest {
            context_tag,
            data: Cow::Borrowed(data),
        })
    }
    /// Clone all borrowed data in this RenderRequest.
    pub fn into_owned(self) -> RenderRequest<'static> {
        RenderRequest {
            context_tag: self.context_tag,
            data: Cow::Owned(self.data.into_owned()),
        }
    }
}
impl<'input> Request for RenderRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for RenderRequest<'input> {
}

/// Opcode for the RenderLarge request
pub const RENDER_LARGE_REQUEST: u8 = 2;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderLargeRequest<'input> {
    pub context_tag: ContextTag,
    pub request_num: u16,
    pub request_total: u16,
    pub data: Cow<'input, [u8]>,
}
impl<'input> RenderLargeRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let request_num_bytes = self.request_num.serialize();
        let request_total_bytes = self.request_total.serialize();
        let data_len = u32::try_from(self.data.len()).expect("`data` has too many elements");
        let data_len_bytes = data_len.serialize();
        let mut request0 = vec![
            major_opcode,
            RENDER_LARGE_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.data.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.data, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != RENDER_LARGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (request_num, remaining) = u16::try_parse(remaining)?;
        let (request_total, remaining) = u16::try_parse(remaining)?;
        let (data_len, remaining) = u32::try_parse(remaining)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, data_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(RenderLargeRequest {
            context_tag,
            request_num,
            request_total,
            data: Cow::Borrowed(data),
        })
    }
    /// Clone all borrowed data in this RenderLargeRequest.
    pub fn into_owned(self) -> RenderLargeRequest<'static> {
        RenderLargeRequest {
            context_tag: self.context_tag,
            request_num: self.request_num,
            request_total: self.request_total,
            data: Cow::Owned(self.data.into_owned()),
        }
    }
}
impl<'input> Request for RenderLargeRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for RenderLargeRequest<'input> {
}

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateContextRequest {
    pub context: Context,
    pub visual: xproto::Visualid,
    pub screen: u32,
    pub share_list: Context,
    pub is_direct: bool,
}
impl CreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let visual_bytes = self.visual.serialize();
        let screen_bytes = self.screen.serialize();
        let share_list_bytes = self.share_list.serialize();
        let is_direct_bytes = self.is_direct.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_CONTEXT_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let (visual, remaining) = xproto::Visualid::try_parse(remaining)?;
        let (screen, remaining) = u32::try_parse(remaining)?;
        let (share_list, remaining) = Context::try_parse(remaining)?;
        let (is_direct, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(CreateContextRequest {
            context,
            visual,
            screen,
            share_list,
            is_direct,
        })
    }
}
impl Request for CreateContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CreateContextRequest {
}

/// Opcode for the DestroyContext request
pub const DESTROY_CONTEXT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DestroyContextRequest {
    pub context: Context,
}
impl DestroyContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            DESTROY_CONTEXT_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyContextRequest {
            context,
        })
    }
}
impl Request for DestroyContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DestroyContextRequest {
}

/// Opcode for the MakeCurrent request
pub const MAKE_CURRENT_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MakeCurrentRequest {
    pub drawable: Drawable,
    pub context: Context,
    pub old_context_tag: ContextTag,
}
impl MakeCurrentRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let context_bytes = self.context.serialize();
        let old_context_tag_bytes = self.old_context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            MAKE_CURRENT_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != MAKE_CURRENT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = Drawable::try_parse(value)?;
        let (context, remaining) = Context::try_parse(remaining)?;
        let (old_context_tag, remaining) = ContextTag::try_parse(remaining)?;
        let _ = remaining;
        Ok(MakeCurrentRequest {
            drawable,
            context,
            old_context_tag,
        })
    }
}
impl Request for MakeCurrentRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for MakeCurrentRequest {
    type Reply = MakeCurrentReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MakeCurrentReply {
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl TryParse for MakeCurrentReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_tag, remaining) = ContextTag::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = MakeCurrentReply { sequence, length, context_tag };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for MakeCurrentReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let context_tag_bytes = self.context_tag.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
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
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.context_tag.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}
#[cfg(test)]
mod make_current_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for MakeCurrentReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context_tag: ContextTag = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                context_tag,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8779459665928466432);
        let value = MakeCurrentReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the IsDirect request
pub const IS_DIRECT_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsDirectRequest {
    pub context: Context,
}
impl IsDirectRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            IS_DIRECT_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != IS_DIRECT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let _ = remaining;
        Ok(IsDirectRequest {
            context,
        })
    }
}
impl Request for IsDirectRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for IsDirectRequest {
    type Reply = IsDirectReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsDirectReply {
    pub sequence: u16,
    pub length: u32,
    pub is_direct: bool,
}
impl TryParse for IsDirectReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (is_direct, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = IsDirectReply { sequence, length, is_direct };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for IsDirectReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let is_direct_bytes = self.is_direct.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            is_direct_bytes[0],
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
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.is_direct.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 23]);
    }
}
#[cfg(test)]
mod is_direct_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for IsDirectReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let is_direct: bool = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                is_direct,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(16479873601393463296);
        let value = IsDirectReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryVersionRequest {
    pub major_version: u32,
    pub minor_version: u32,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            major_version_bytes[0],
            major_version_bytes[1],
            major_version_bytes[2],
            major_version_bytes[3],
            minor_version_bytes[0],
            minor_version_bytes[1],
            minor_version_bytes[2],
            minor_version_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (major_version, remaining) = u32::try_parse(value)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            major_version,
            minor_version,
        })
    }
}
impl Request for QueryVersionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryVersionRequest {
    type Reply = QueryVersionReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u32::try_parse(remaining)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply { sequence, length, major_version, minor_version };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryVersionReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            major_version_bytes[0],
            major_version_bytes[1],
            major_version_bytes[2],
            major_version_bytes[3],
            minor_version_bytes[0],
            minor_version_bytes[1],
            minor_version_bytes[2],
            minor_version_bytes[3],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.major_version.serialize_into(bytes);
        self.minor_version.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
    }
}
#[cfg(test)]
mod query_version_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryVersionReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let major_version: u32 = GenRandom::generate(rng);
            let minor_version: u32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                major_version,
                minor_version,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(228006558612858880);
        let value = QueryVersionReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the WaitGL request
pub const WAIT_GL_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WaitGLRequest {
    pub context_tag: ContextTag,
}
impl WaitGLRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            WAIT_GL_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != WAIT_GL_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let _ = remaining;
        Ok(WaitGLRequest {
            context_tag,
        })
    }
}
impl Request for WaitGLRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for WaitGLRequest {
}

/// Opcode for the WaitX request
pub const WAIT_X_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WaitXRequest {
    pub context_tag: ContextTag,
}
impl WaitXRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            WAIT_X_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != WAIT_X_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let _ = remaining;
        Ok(WaitXRequest {
            context_tag,
        })
    }
}
impl Request for WaitXRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for WaitXRequest {
}

/// Opcode for the CopyContext request
pub const COPY_CONTEXT_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CopyContextRequest {
    pub src: Context,
    pub dest: Context,
    pub mask: u32,
    pub src_context_tag: ContextTag,
}
impl CopyContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let src_bytes = self.src.serialize();
        let dest_bytes = self.dest.serialize();
        let mask_bytes = self.mask.serialize();
        let src_context_tag_bytes = self.src_context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            COPY_CONTEXT_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != COPY_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (src, remaining) = Context::try_parse(value)?;
        let (dest, remaining) = Context::try_parse(remaining)?;
        let (mask, remaining) = u32::try_parse(remaining)?;
        let (src_context_tag, remaining) = ContextTag::try_parse(remaining)?;
        let _ = remaining;
        Ok(CopyContextRequest {
            src,
            dest,
            mask,
            src_context_tag,
        })
    }
}
impl Request for CopyContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CopyContextRequest {
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GC(u32);
impl GC {
    pub const GL_CURRENT_BIT: Self = Self(1 << 0);
    pub const GL_POINT_BIT: Self = Self(1 << 1);
    pub const GL_LINE_BIT: Self = Self(1 << 2);
    pub const GL_POLYGON_BIT: Self = Self(1 << 3);
    pub const GL_POLYGON_STIPPLE_BIT: Self = Self(1 << 4);
    pub const GL_PIXEL_MODE_BIT: Self = Self(1 << 5);
    pub const GL_LIGHTING_BIT: Self = Self(1 << 6);
    pub const GL_FOG_BIT: Self = Self(1 << 7);
    pub const GL_DEPTH_BUFFER_BIT: Self = Self(1 << 8);
    pub const GL_ACCUM_BUFFER_BIT: Self = Self(1 << 9);
    pub const GL_STENCIL_BUFFER_BIT: Self = Self(1 << 10);
    pub const GL_VIEWPORT_BIT: Self = Self(1 << 11);
    pub const GL_TRANSFORM_BIT: Self = Self(1 << 12);
    pub const GL_ENABLE_BIT: Self = Self(1 << 13);
    pub const GL_COLOR_BUFFER_BIT: Self = Self(1 << 14);
    pub const GL_HINT_BIT: Self = Self(1 << 15);
    pub const GL_EVAL_BIT: Self = Self(1 << 16);
    pub const GL_LIST_BIT: Self = Self(1 << 17);
    pub const GL_TEXTURE_BIT: Self = Self(1 << 18);
    pub const GL_SCISSOR_BIT: Self = Self(1 << 19);
    pub const GL_ALL_ATTRIB_BITS: Self = Self(16_777_215);
}
impl From<GC> for u32 {
    #[inline]
    fn from(input: GC) -> Self {
        input.0
    }
}
impl From<GC> for Option<u32> {
    #[inline]
    fn from(input: GC) -> Self {
        Some(input.0)
    }
}
impl From<u8> for GC {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for GC {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for GC {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for GC  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::GL_CURRENT_BIT.0, "GL_CURRENT_BIT", "GL_CURRENT_BIT"),
            (Self::GL_POINT_BIT.0, "GL_POINT_BIT", "GL_POINT_BIT"),
            (Self::GL_LINE_BIT.0, "GL_LINE_BIT", "GL_LINE_BIT"),
            (Self::GL_POLYGON_BIT.0, "GL_POLYGON_BIT", "GL_POLYGON_BIT"),
            (Self::GL_POLYGON_STIPPLE_BIT.0, "GL_POLYGON_STIPPLE_BIT", "GL_POLYGON_STIPPLE_BIT"),
            (Self::GL_PIXEL_MODE_BIT.0, "GL_PIXEL_MODE_BIT", "GL_PIXEL_MODE_BIT"),
            (Self::GL_LIGHTING_BIT.0, "GL_LIGHTING_BIT", "GL_LIGHTING_BIT"),
            (Self::GL_FOG_BIT.0, "GL_FOG_BIT", "GL_FOG_BIT"),
            (Self::GL_DEPTH_BUFFER_BIT.0, "GL_DEPTH_BUFFER_BIT", "GL_DEPTH_BUFFER_BIT"),
            (Self::GL_ACCUM_BUFFER_BIT.0, "GL_ACCUM_BUFFER_BIT", "GL_ACCUM_BUFFER_BIT"),
            (Self::GL_STENCIL_BUFFER_BIT.0, "GL_STENCIL_BUFFER_BIT", "GL_STENCIL_BUFFER_BIT"),
            (Self::GL_VIEWPORT_BIT.0, "GL_VIEWPORT_BIT", "GL_VIEWPORT_BIT"),
            (Self::GL_TRANSFORM_BIT.0, "GL_TRANSFORM_BIT", "GL_TRANSFORM_BIT"),
            (Self::GL_ENABLE_BIT.0, "GL_ENABLE_BIT", "GL_ENABLE_BIT"),
            (Self::GL_COLOR_BUFFER_BIT.0, "GL_COLOR_BUFFER_BIT", "GL_COLOR_BUFFER_BIT"),
            (Self::GL_HINT_BIT.0, "GL_HINT_BIT", "GL_HINT_BIT"),
            (Self::GL_EVAL_BIT.0, "GL_EVAL_BIT", "GL_EVAL_BIT"),
            (Self::GL_LIST_BIT.0, "GL_LIST_BIT", "GL_LIST_BIT"),
            (Self::GL_TEXTURE_BIT.0, "GL_TEXTURE_BIT", "GL_TEXTURE_BIT"),
            (Self::GL_SCISSOR_BIT.0, "GL_SCISSOR_BIT", "GL_SCISSOR_BIT"),
            (Self::GL_ALL_ATTRIB_BITS.0, "GL_ALL_ATTRIB_BITS", "GL_ALL_ATTRIB_BITS"),
        ];
        pretty_print_enum(fmt, self.0, &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for GC {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::GL_CURRENT_BIT.0,
            Self::GL_POINT_BIT.0,
            Self::GL_LINE_BIT.0,
            Self::GL_POLYGON_BIT.0,
            Self::GL_POLYGON_STIPPLE_BIT.0,
            Self::GL_PIXEL_MODE_BIT.0,
            Self::GL_LIGHTING_BIT.0,
            Self::GL_FOG_BIT.0,
            Self::GL_DEPTH_BUFFER_BIT.0,
            Self::GL_ACCUM_BUFFER_BIT.0,
            Self::GL_STENCIL_BUFFER_BIT.0,
            Self::GL_VIEWPORT_BIT.0,
            Self::GL_TRANSFORM_BIT.0,
            Self::GL_ENABLE_BIT.0,
            Self::GL_COLOR_BUFFER_BIT.0,
            Self::GL_HINT_BIT.0,
            Self::GL_EVAL_BIT.0,
            Self::GL_LIST_BIT.0,
            Self::GL_TEXTURE_BIT.0,
            Self::GL_SCISSOR_BIT.0,
            Self::GL_ALL_ATTRIB_BITS.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the SwapBuffers request
pub const SWAP_BUFFERS_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapBuffersRequest {
    pub context_tag: ContextTag,
    pub drawable: Drawable,
}
impl SwapBuffersRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            major_opcode,
            SWAP_BUFFERS_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SWAP_BUFFERS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (drawable, remaining) = Drawable::try_parse(remaining)?;
        let _ = remaining;
        Ok(SwapBuffersRequest {
            context_tag,
            drawable,
        })
    }
}
impl Request for SwapBuffersRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SwapBuffersRequest {
}

/// Opcode for the UseXFont request
pub const USE_X_FONT_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UseXFontRequest {
    pub context_tag: ContextTag,
    pub font: xproto::Font,
    pub first: u32,
    pub count: u32,
    pub list_base: u32,
}
impl UseXFontRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let font_bytes = self.font.serialize();
        let first_bytes = self.first.serialize();
        let count_bytes = self.count.serialize();
        let list_base_bytes = self.list_base.serialize();
        let mut request0 = vec![
            major_opcode,
            USE_X_FONT_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != USE_X_FONT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (font, remaining) = xproto::Font::try_parse(remaining)?;
        let (first, remaining) = u32::try_parse(remaining)?;
        let (count, remaining) = u32::try_parse(remaining)?;
        let (list_base, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(UseXFontRequest {
            context_tag,
            font,
            first,
            count,
            list_base,
        })
    }
}
impl Request for UseXFontRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for UseXFontRequest {
}

/// Opcode for the CreateGLXPixmap request
pub const CREATE_GLX_PIXMAP_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateGLXPixmapRequest {
    pub screen: u32,
    pub visual: xproto::Visualid,
    pub pixmap: xproto::Pixmap,
    pub glx_pixmap: Pixmap,
}
impl CreateGLXPixmapRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let visual_bytes = self.visual.serialize();
        let pixmap_bytes = self.pixmap.serialize();
        let glx_pixmap_bytes = self.glx_pixmap.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_GLX_PIXMAP_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_GLX_PIXMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (visual, remaining) = xproto::Visualid::try_parse(remaining)?;
        let (pixmap, remaining) = xproto::Pixmap::try_parse(remaining)?;
        let (glx_pixmap, remaining) = Pixmap::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateGLXPixmapRequest {
            screen,
            visual,
            pixmap,
            glx_pixmap,
        })
    }
}
impl Request for CreateGLXPixmapRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CreateGLXPixmapRequest {
}

/// Opcode for the GetVisualConfigs request
pub const GET_VISUAL_CONFIGS_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetVisualConfigsRequest {
    pub screen: u32,
}
impl GetVisualConfigsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_VISUAL_CONFIGS_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_VISUAL_CONFIGS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(GetVisualConfigsRequest {
            screen,
        })
    }
}
impl Request for GetVisualConfigsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetVisualConfigsRequest {
    type Reply = GetVisualConfigsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetVisualConfigsReply {
    pub sequence: u16,
    pub num_visuals: u32,
    pub num_properties: u32,
    pub property_list: Vec<u32>,
}
impl TryParse for GetVisualConfigsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_visuals, remaining) = u32::try_parse(remaining)?;
        let (num_properties, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (property_list, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetVisualConfigsReply { sequence, num_visuals, num_properties, property_list };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetVisualConfigsReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        let length = u32::try_from(self.property_list.len()).expect("`property_list` has too many elements");
        length.serialize_into(bytes);
        self.num_visuals.serialize_into(bytes);
        self.num_properties.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
        self.property_list.serialize_into(bytes);
    }
}
impl GetVisualConfigsReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `property_list` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.property_list.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_visual_configs_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetVisualConfigsReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let num_visuals: u32 = GenRandom::generate(rng);
            let num_properties: u32 = GenRandom::generate(rng);
            let property_list: Vec<u32> = gen_random_list(rng, usize::try_from(length).unwrap());
            Self {
                sequence,
                num_visuals,
                num_properties,
                property_list,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1832434775087726592);
        let value = GetVisualConfigsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the DestroyGLXPixmap request
pub const DESTROY_GLX_PIXMAP_REQUEST: u8 = 15;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DestroyGLXPixmapRequest {
    pub glx_pixmap: Pixmap,
}
impl DestroyGLXPixmapRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let glx_pixmap_bytes = self.glx_pixmap.serialize();
        let mut request0 = vec![
            major_opcode,
            DESTROY_GLX_PIXMAP_REQUEST,
            0,
            0,
            glx_pixmap_bytes[0],
            glx_pixmap_bytes[1],
            glx_pixmap_bytes[2],
            glx_pixmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_GLX_PIXMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (glx_pixmap, remaining) = Pixmap::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyGLXPixmapRequest {
            glx_pixmap,
        })
    }
}
impl Request for DestroyGLXPixmapRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DestroyGLXPixmapRequest {
}

/// Opcode for the VendorPrivate request
pub const VENDOR_PRIVATE_REQUEST: u8 = 16;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VendorPrivateRequest<'input> {
    pub vendor_code: u32,
    pub context_tag: ContextTag,
    pub data: Cow<'input, [u8]>,
}
impl<'input> VendorPrivateRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let vendor_code_bytes = self.vendor_code.serialize();
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            VENDOR_PRIVATE_REQUEST,
            0,
            0,
            vendor_code_bytes[0],
            vendor_code_bytes[1],
            vendor_code_bytes[2],
            vendor_code_bytes[3],
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.data.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.data, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != VENDOR_PRIVATE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (vendor_code, remaining) = u32::try_parse(value)?;
        let (context_tag, remaining) = ContextTag::try_parse(remaining)?;
        let (data, remaining) = remaining.split_at(remaining.len());
        let _ = remaining;
        Ok(VendorPrivateRequest {
            vendor_code,
            context_tag,
            data: Cow::Borrowed(data),
        })
    }
    /// Clone all borrowed data in this VendorPrivateRequest.
    pub fn into_owned(self) -> VendorPrivateRequest<'static> {
        VendorPrivateRequest {
            vendor_code: self.vendor_code,
            context_tag: self.context_tag,
            data: Cow::Owned(self.data.into_owned()),
        }
    }
}
impl<'input> Request for VendorPrivateRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for VendorPrivateRequest<'input> {
}

/// Opcode for the VendorPrivateWithReply request
pub const VENDOR_PRIVATE_WITH_REPLY_REQUEST: u8 = 17;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VendorPrivateWithReplyRequest<'input> {
    pub vendor_code: u32,
    pub context_tag: ContextTag,
    pub data: Cow<'input, [u8]>,
}
impl<'input> VendorPrivateWithReplyRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let vendor_code_bytes = self.vendor_code.serialize();
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            VENDOR_PRIVATE_WITH_REPLY_REQUEST,
            0,
            0,
            vendor_code_bytes[0],
            vendor_code_bytes[1],
            vendor_code_bytes[2],
            vendor_code_bytes[3],
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.data.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.data, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != VENDOR_PRIVATE_WITH_REPLY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (vendor_code, remaining) = u32::try_parse(value)?;
        let (context_tag, remaining) = ContextTag::try_parse(remaining)?;
        let (data, remaining) = remaining.split_at(remaining.len());
        let _ = remaining;
        Ok(VendorPrivateWithReplyRequest {
            vendor_code,
            context_tag,
            data: Cow::Borrowed(data),
        })
    }
    /// Clone all borrowed data in this VendorPrivateWithReplyRequest.
    pub fn into_owned(self) -> VendorPrivateWithReplyRequest<'static> {
        VendorPrivateWithReplyRequest {
            vendor_code: self.vendor_code,
            context_tag: self.context_tag,
            data: Cow::Owned(self.data.into_owned()),
        }
    }
}
impl<'input> Request for VendorPrivateWithReplyRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for VendorPrivateWithReplyRequest<'input> {
    type Reply = VendorPrivateWithReplyReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VendorPrivateWithReplyReply {
    pub sequence: u16,
    pub retval: u32,
    pub data1: [u8; 24],
    pub data2: Vec<u8>,
}
impl TryParse for VendorPrivateWithReplyReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (retval, remaining) = u32::try_parse(remaining)?;
        let (data1, remaining) = crate::x11_utils::parse_u8_list(remaining, 24)?;
        let data1 = <[u8; 24]>::try_from(data1).unwrap();
        let (data2, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data2 = data2.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = VendorPrivateWithReplyReply { sequence, retval, data1, data2 };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for VendorPrivateWithReplyReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(36);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data2.len() % 4, 0, "`data2` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data2.len() / 4).expect("`data2` has too many elements");
        length.serialize_into(bytes);
        self.retval.serialize_into(bytes);
        bytes.extend_from_slice(&self.data1);
        bytes.extend_from_slice(&self.data2);
    }
}
impl VendorPrivateWithReplyReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data2` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data2.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod vendor_private_with_reply_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for VendorPrivateWithReplyReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let retval: u32 = GenRandom::generate(rng);
            let data1: [u8; 24] = gen_random_list(rng, usize::try_from(24u32).unwrap());
            let data2: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                retval,
                data1,
                data2,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5259099213848903680);
        let value = VendorPrivateWithReplyReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryExtensionsString request
pub const QUERY_EXTENSIONS_STRING_REQUEST: u8 = 18;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryExtensionsStringRequest {
    pub screen: u32,
}
impl QueryExtensionsStringRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_EXTENSIONS_STRING_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_EXTENSIONS_STRING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(QueryExtensionsStringRequest {
            screen,
        })
    }
}
impl Request for QueryExtensionsStringRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryExtensionsStringRequest {
    type Reply = QueryExtensionsStringReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryExtensionsStringReply {
    pub sequence: u16,
    pub length: u32,
    pub n: u32,
}
impl TryParse for QueryExtensionsStringReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryExtensionsStringReply { sequence, length, n };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryExtensionsStringReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let n_bytes = self.n.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            0,
            0,
            0,
            0,
            n_bytes[0],
            n_bytes[1],
            n_bytes[2],
            n_bytes[3],
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
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.n.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
    }
}
#[cfg(test)]
mod query_extensions_string_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryExtensionsStringReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let n: u32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                n,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3847882290182488064);
        let value = QueryExtensionsStringReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryServerString request
pub const QUERY_SERVER_STRING_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryServerStringRequest {
    pub screen: u32,
    pub name: u32,
}
impl QueryServerStringRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let name_bytes = self.name.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_SERVER_STRING_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_SERVER_STRING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (name, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryServerStringRequest {
            screen,
            name,
        })
    }
}
impl Request for QueryServerStringRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryServerStringRequest {
    type Reply = QueryServerStringReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryServerStringReply {
    pub sequence: u16,
    pub length: u32,
    pub string: Vec<u8>,
}
impl TryParse for QueryServerStringReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (str_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, str_len.try_to_usize()?)?;
        let string = string.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryServerStringReply { sequence, length, string };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryServerStringReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let str_len = u32::try_from(self.string.len()).expect("`string` has too many elements");
        str_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
        bytes.extend_from_slice(&self.string);
    }
}
impl QueryServerStringReply {
    /// Get the value of the `str_len` field.
    ///
    /// The `str_len` field is used as the length field of the `string` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn str_len(&self) -> u32 {
        self.string.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod query_server_string_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryServerStringReply {
        fn generate(rng: &Rng) -> Self {
            let str_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let string: Vec<u8> = gen_random_list(rng, usize::try_from(str_len).unwrap());
            Self {
                sequence,
                length,
                string,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9003271042815459328);
        let value = QueryServerStringReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ClientInfo request
pub const CLIENT_INFO_REQUEST: u8 = 20;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClientInfoRequest<'input> {
    pub major_version: u32,
    pub minor_version: u32,
    pub string: Cow<'input, [u8]>,
}
impl<'input> ClientInfoRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let str_len = u32::try_from(self.string.len()).expect("`string` has too many elements");
        let str_len_bytes = str_len.serialize();
        let mut request0 = vec![
            major_opcode,
            CLIENT_INFO_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.string.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.string, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CLIENT_INFO_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (major_version, remaining) = u32::try_parse(value)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let (str_len, remaining) = u32::try_parse(remaining)?;
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, str_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(ClientInfoRequest {
            major_version,
            minor_version,
            string: Cow::Borrowed(string),
        })
    }
    /// Clone all borrowed data in this ClientInfoRequest.
    pub fn into_owned(self) -> ClientInfoRequest<'static> {
        ClientInfoRequest {
            major_version: self.major_version,
            minor_version: self.minor_version,
            string: Cow::Owned(self.string.into_owned()),
        }
    }
}
impl<'input> Request for ClientInfoRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for ClientInfoRequest<'input> {
}

/// Opcode for the GetFBConfigs request
pub const GET_FB_CONFIGS_REQUEST: u8 = 21;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetFBConfigsRequest {
    pub screen: u32,
}
impl GetFBConfigsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_FB_CONFIGS_REQUEST,
            0,
            0,
            screen_bytes[0],
            screen_bytes[1],
            screen_bytes[2],
            screen_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_FB_CONFIGS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(GetFBConfigsRequest {
            screen,
        })
    }
}
impl Request for GetFBConfigsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetFBConfigsRequest {
    type Reply = GetFBConfigsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetFBConfigsReply {
    pub sequence: u16,
    pub num_fb_configs: u32,
    pub num_properties: u32,
    pub property_list: Vec<u32>,
}
impl TryParse for GetFBConfigsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_fb_configs, remaining) = u32::try_parse(remaining)?;
        let (num_properties, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (property_list, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetFBConfigsReply { sequence, num_fb_configs, num_properties, property_list };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetFBConfigsReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        let length = u32::try_from(self.property_list.len()).expect("`property_list` has too many elements");
        length.serialize_into(bytes);
        self.num_fb_configs.serialize_into(bytes);
        self.num_properties.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
        self.property_list.serialize_into(bytes);
    }
}
impl GetFBConfigsReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `property_list` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.property_list.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_fb_configs_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetFBConfigsReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let num_fb_configs: u32 = GenRandom::generate(rng);
            let num_properties: u32 = GenRandom::generate(rng);
            let property_list: Vec<u32> = gen_random_list(rng, usize::try_from(length).unwrap());
            Self {
                sequence,
                num_fb_configs,
                num_properties,
                property_list,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12656405130490159104);
        let value = GetFBConfigsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the CreatePixmap request
pub const CREATE_PIXMAP_REQUEST: u8 = 22;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePixmapRequest<'input> {
    pub screen: u32,
    pub fbconfig: Fbconfig,
    pub pixmap: xproto::Pixmap,
    pub glx_pixmap: Pixmap,
    pub attribs: Cow<'input, [u32]>,
}
impl<'input> CreatePixmapRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let pixmap_bytes = self.pixmap.serialize();
        let glx_pixmap_bytes = self.glx_pixmap.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_PIXMAP_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let attribs_bytes = self.attribs.serialize();
        let length_so_far = length_so_far + attribs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), attribs_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_PIXMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (fbconfig, remaining) = Fbconfig::try_parse(remaining)?;
        let (pixmap, remaining) = xproto::Pixmap::try_parse(remaining)?;
        let (glx_pixmap, remaining) = Pixmap::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let _ = remaining;
        Ok(CreatePixmapRequest {
            screen,
            fbconfig,
            pixmap,
            glx_pixmap,
            attribs: Cow::Owned(attribs),
        })
    }
    /// Clone all borrowed data in this CreatePixmapRequest.
    pub fn into_owned(self) -> CreatePixmapRequest<'static> {
        CreatePixmapRequest {
            screen: self.screen,
            fbconfig: self.fbconfig,
            pixmap: self.pixmap,
            glx_pixmap: self.glx_pixmap,
            attribs: Cow::Owned(self.attribs.into_owned()),
        }
    }
}
impl<'input> Request for CreatePixmapRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for CreatePixmapRequest<'input> {
}

/// Opcode for the DestroyPixmap request
pub const DESTROY_PIXMAP_REQUEST: u8 = 23;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DestroyPixmapRequest {
    pub glx_pixmap: Pixmap,
}
impl DestroyPixmapRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let glx_pixmap_bytes = self.glx_pixmap.serialize();
        let mut request0 = vec![
            major_opcode,
            DESTROY_PIXMAP_REQUEST,
            0,
            0,
            glx_pixmap_bytes[0],
            glx_pixmap_bytes[1],
            glx_pixmap_bytes[2],
            glx_pixmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_PIXMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (glx_pixmap, remaining) = Pixmap::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyPixmapRequest {
            glx_pixmap,
        })
    }
}
impl Request for DestroyPixmapRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DestroyPixmapRequest {
}

/// Opcode for the CreateNewContext request
pub const CREATE_NEW_CONTEXT_REQUEST: u8 = 24;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateNewContextRequest {
    pub context: Context,
    pub fbconfig: Fbconfig,
    pub screen: u32,
    pub render_type: u32,
    pub share_list: Context,
    pub is_direct: bool,
}
impl CreateNewContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let screen_bytes = self.screen.serialize();
        let render_type_bytes = self.render_type.serialize();
        let share_list_bytes = self.share_list.serialize();
        let is_direct_bytes = self.is_direct.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_NEW_CONTEXT_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_NEW_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let (fbconfig, remaining) = Fbconfig::try_parse(remaining)?;
        let (screen, remaining) = u32::try_parse(remaining)?;
        let (render_type, remaining) = u32::try_parse(remaining)?;
        let (share_list, remaining) = Context::try_parse(remaining)?;
        let (is_direct, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(CreateNewContextRequest {
            context,
            fbconfig,
            screen,
            render_type,
            share_list,
            is_direct,
        })
    }
}
impl Request for CreateNewContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for CreateNewContextRequest {
}

/// Opcode for the QueryContext request
pub const QUERY_CONTEXT_REQUEST: u8 = 25;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryContextRequest {
    pub context: Context,
}
impl QueryContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_CONTEXT_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let _ = remaining;
        Ok(QueryContextRequest {
            context,
        })
    }
}
impl Request for QueryContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryContextRequest {
    type Reply = QueryContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryContextReply {
    pub sequence: u16,
    pub length: u32,
    pub attribs: Vec<u32>,
}
impl TryParse for QueryContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryContextReply { sequence, length, attribs };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryContextReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        num_attribs.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.attribs.serialize_into(bytes);
    }
}
impl QueryContextReply {
    /// Get the value of the `num_attribs` field.
    ///
    /// The `num_attribs` field is used as the length field of the `attribs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_attribs(&self) -> u32 {
        self.attribs.len()
            .checked_div(2).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod query_context_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryContextReply {
        fn generate(rng: &Rng) -> Self {
            let num_attribs = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let attribs: Vec<u32> = gen_random_list(rng, usize::try_from(num_attribs.checked_mul(2u32).unwrap()).unwrap());
            Self {
                sequence,
                length,
                attribs,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7141503560154939392);
        let value = QueryContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the MakeContextCurrent request
pub const MAKE_CONTEXT_CURRENT_REQUEST: u8 = 26;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MakeContextCurrentRequest {
    pub old_context_tag: ContextTag,
    pub drawable: Drawable,
    pub read_drawable: Drawable,
    pub context: Context,
}
impl MakeContextCurrentRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let old_context_tag_bytes = self.old_context_tag.serialize();
        let drawable_bytes = self.drawable.serialize();
        let read_drawable_bytes = self.read_drawable.serialize();
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            MAKE_CONTEXT_CURRENT_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != MAKE_CONTEXT_CURRENT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (old_context_tag, remaining) = ContextTag::try_parse(value)?;
        let (drawable, remaining) = Drawable::try_parse(remaining)?;
        let (read_drawable, remaining) = Drawable::try_parse(remaining)?;
        let (context, remaining) = Context::try_parse(remaining)?;
        let _ = remaining;
        Ok(MakeContextCurrentRequest {
            old_context_tag,
            drawable,
            read_drawable,
            context,
        })
    }
}
impl Request for MakeContextCurrentRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for MakeContextCurrentRequest {
    type Reply = MakeContextCurrentReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MakeContextCurrentReply {
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl TryParse for MakeContextCurrentReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_tag, remaining) = ContextTag::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = MakeContextCurrentReply { sequence, length, context_tag };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for MakeContextCurrentReply {
    type Bytes = [u8; 32];
    fn serialize(&self) -> [u8; 32] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let context_tag_bytes = self.context_tag.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
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
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.context_tag.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
    }
}
#[cfg(test)]
mod make_context_current_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for MakeContextCurrentReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context_tag: ContextTag = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                context_tag,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9967291892132478976);
        let value = MakeContextCurrentReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the CreatePbuffer request
pub const CREATE_PBUFFER_REQUEST: u8 = 27;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePbufferRequest<'input> {
    pub screen: u32,
    pub fbconfig: Fbconfig,
    pub pbuffer: Pbuffer,
    pub attribs: Cow<'input, [u32]>,
}
impl<'input> CreatePbufferRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let pbuffer_bytes = self.pbuffer.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_PBUFFER_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let attribs_bytes = self.attribs.serialize();
        let length_so_far = length_so_far + attribs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), attribs_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_PBUFFER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (fbconfig, remaining) = Fbconfig::try_parse(remaining)?;
        let (pbuffer, remaining) = Pbuffer::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let _ = remaining;
        Ok(CreatePbufferRequest {
            screen,
            fbconfig,
            pbuffer,
            attribs: Cow::Owned(attribs),
        })
    }
    /// Clone all borrowed data in this CreatePbufferRequest.
    pub fn into_owned(self) -> CreatePbufferRequest<'static> {
        CreatePbufferRequest {
            screen: self.screen,
            fbconfig: self.fbconfig,
            pbuffer: self.pbuffer,
            attribs: Cow::Owned(self.attribs.into_owned()),
        }
    }
}
impl<'input> Request for CreatePbufferRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for CreatePbufferRequest<'input> {
}

/// Opcode for the DestroyPbuffer request
pub const DESTROY_PBUFFER_REQUEST: u8 = 28;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DestroyPbufferRequest {
    pub pbuffer: Pbuffer,
}
impl DestroyPbufferRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let pbuffer_bytes = self.pbuffer.serialize();
        let mut request0 = vec![
            major_opcode,
            DESTROY_PBUFFER_REQUEST,
            0,
            0,
            pbuffer_bytes[0],
            pbuffer_bytes[1],
            pbuffer_bytes[2],
            pbuffer_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_PBUFFER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (pbuffer, remaining) = Pbuffer::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyPbufferRequest {
            pbuffer,
        })
    }
}
impl Request for DestroyPbufferRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DestroyPbufferRequest {
}

/// Opcode for the GetDrawableAttributes request
pub const GET_DRAWABLE_ATTRIBUTES_REQUEST: u8 = 29;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDrawableAttributesRequest {
    pub drawable: Drawable,
}
impl GetDrawableAttributesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DRAWABLE_ATTRIBUTES_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DRAWABLE_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = Drawable::try_parse(value)?;
        let _ = remaining;
        Ok(GetDrawableAttributesRequest {
            drawable,
        })
    }
}
impl Request for GetDrawableAttributesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDrawableAttributesRequest {
    type Reply = GetDrawableAttributesReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDrawableAttributesReply {
    pub sequence: u16,
    pub length: u32,
    pub attribs: Vec<u32>,
}
impl TryParse for GetDrawableAttributesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDrawableAttributesReply { sequence, length, attribs };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDrawableAttributesReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        num_attribs.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.attribs.serialize_into(bytes);
    }
}
impl GetDrawableAttributesReply {
    /// Get the value of the `num_attribs` field.
    ///
    /// The `num_attribs` field is used as the length field of the `attribs` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_attribs(&self) -> u32 {
        self.attribs.len()
            .checked_div(2).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_drawable_attributes_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetDrawableAttributesReply {
        fn generate(rng: &Rng) -> Self {
            let num_attribs = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let attribs: Vec<u32> = gen_random_list(rng, usize::try_from(num_attribs.checked_mul(2u32).unwrap()).unwrap());
            Self {
                sequence,
                length,
                attribs,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6160761388919685120);
        let value = GetDrawableAttributesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ChangeDrawableAttributes request
pub const CHANGE_DRAWABLE_ATTRIBUTES_REQUEST: u8 = 30;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeDrawableAttributesRequest<'input> {
    pub drawable: Drawable,
    pub attribs: Cow<'input, [u32]>,
}
impl<'input> ChangeDrawableAttributesRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            major_opcode,
            CHANGE_DRAWABLE_ATTRIBUTES_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            num_attribs_bytes[0],
            num_attribs_bytes[1],
            num_attribs_bytes[2],
            num_attribs_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let attribs_bytes = self.attribs.serialize();
        let length_so_far = length_so_far + attribs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), attribs_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_DRAWABLE_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = Drawable::try_parse(value)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let _ = remaining;
        Ok(ChangeDrawableAttributesRequest {
            drawable,
            attribs: Cow::Owned(attribs),
        })
    }
    /// Clone all borrowed data in this ChangeDrawableAttributesRequest.
    pub fn into_owned(self) -> ChangeDrawableAttributesRequest<'static> {
        ChangeDrawableAttributesRequest {
            drawable: self.drawable,
            attribs: Cow::Owned(self.attribs.into_owned()),
        }
    }
}
impl<'input> Request for ChangeDrawableAttributesRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for ChangeDrawableAttributesRequest<'input> {
}

/// Opcode for the CreateWindow request
pub const CREATE_WINDOW_REQUEST: u8 = 31;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateWindowRequest<'input> {
    pub screen: u32,
    pub fbconfig: Fbconfig,
    pub window: xproto::Window,
    pub glx_window: Window,
    pub attribs: Cow<'input, [u32]>,
}
impl<'input> CreateWindowRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let window_bytes = self.window.serialize();
        let glx_window_bytes = self.glx_window.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_WINDOW_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let attribs_bytes = self.attribs.serialize();
        let length_so_far = length_so_far + attribs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), attribs_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_WINDOW_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (screen, remaining) = u32::try_parse(value)?;
        let (fbconfig, remaining) = Fbconfig::try_parse(remaining)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (glx_window, remaining) = Window::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let _ = remaining;
        Ok(CreateWindowRequest {
            screen,
            fbconfig,
            window,
            glx_window,
            attribs: Cow::Owned(attribs),
        })
    }
    /// Clone all borrowed data in this CreateWindowRequest.
    pub fn into_owned(self) -> CreateWindowRequest<'static> {
        CreateWindowRequest {
            screen: self.screen,
            fbconfig: self.fbconfig,
            window: self.window,
            glx_window: self.glx_window,
            attribs: Cow::Owned(self.attribs.into_owned()),
        }
    }
}
impl<'input> Request for CreateWindowRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for CreateWindowRequest<'input> {
}

/// Opcode for the DeleteWindow request
pub const DELETE_WINDOW_REQUEST: u8 = 32;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteWindowRequest {
    pub glxwindow: Window,
}
impl DeleteWindowRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let glxwindow_bytes = self.glxwindow.serialize();
        let mut request0 = vec![
            major_opcode,
            DELETE_WINDOW_REQUEST,
            0,
            0,
            glxwindow_bytes[0],
            glxwindow_bytes[1],
            glxwindow_bytes[2],
            glxwindow_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DELETE_WINDOW_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (glxwindow, remaining) = Window::try_parse(value)?;
        let _ = remaining;
        Ok(DeleteWindowRequest {
            glxwindow,
        })
    }
}
impl Request for DeleteWindowRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DeleteWindowRequest {
}

/// Opcode for the SetClientInfoARB request
pub const SET_CLIENT_INFO_ARB_REQUEST: u8 = 33;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetClientInfoARBRequest<'input> {
    pub major_version: u32,
    pub minor_version: u32,
    pub gl_versions: Cow<'input, [u32]>,
    pub gl_extension_string: Cow<'input, [u8]>,
    pub glx_extension_string: Cow<'input, [u8]>,
}
impl<'input> SetClientInfoARBRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        assert_eq!(self.gl_versions.len() % 2, 0, "`gl_versions` has an incorrect length, must be a multiple of 2");
        let num_versions = u32::try_from(self.gl_versions.len() / 2).expect("`gl_versions` has too many elements");
        let num_versions_bytes = num_versions.serialize();
        let gl_str_len = u32::try_from(self.gl_extension_string.len()).expect("`gl_extension_string` has too many elements");
        let gl_str_len_bytes = gl_str_len.serialize();
        let glx_str_len = u32::try_from(self.glx_extension_string.len()).expect("`glx_extension_string` has too many elements");
        let glx_str_len_bytes = glx_str_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_CLIENT_INFO_ARB_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let gl_versions_bytes = self.gl_versions.serialize();
        let length_so_far = length_so_far + gl_versions_bytes.len();
        let length_so_far = length_so_far + self.gl_extension_string.len();
        let length_so_far = length_so_far + self.glx_extension_string.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), gl_versions_bytes.into(), self.gl_extension_string, self.glx_extension_string, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_CLIENT_INFO_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (major_version, remaining) = u32::try_parse(value)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let (num_versions, remaining) = u32::try_parse(remaining)?;
        let (gl_str_len, remaining) = u32::try_parse(remaining)?;
        let (glx_str_len, remaining) = u32::try_parse(remaining)?;
        let (gl_versions, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_versions.checked_mul(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let (gl_extension_string, remaining) = crate::x11_utils::parse_u8_list(remaining, gl_str_len.try_to_usize()?)?;
        let (glx_extension_string, remaining) = crate::x11_utils::parse_u8_list(remaining, glx_str_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetClientInfoARBRequest {
            major_version,
            minor_version,
            gl_versions: Cow::Owned(gl_versions),
            gl_extension_string: Cow::Borrowed(gl_extension_string),
            glx_extension_string: Cow::Borrowed(glx_extension_string),
        })
    }
    /// Clone all borrowed data in this SetClientInfoARBRequest.
    pub fn into_owned(self) -> SetClientInfoARBRequest<'static> {
        SetClientInfoARBRequest {
            major_version: self.major_version,
            minor_version: self.minor_version,
            gl_versions: Cow::Owned(self.gl_versions.into_owned()),
            gl_extension_string: Cow::Owned(self.gl_extension_string.into_owned()),
            glx_extension_string: Cow::Owned(self.glx_extension_string.into_owned()),
        }
    }
}
impl<'input> Request for SetClientInfoARBRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetClientInfoARBRequest<'input> {
}

/// Opcode for the CreateContextAttribsARB request
pub const CREATE_CONTEXT_ATTRIBS_ARB_REQUEST: u8 = 34;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateContextAttribsARBRequest<'input> {
    pub context: Context,
    pub fbconfig: Fbconfig,
    pub screen: u32,
    pub share_list: Context,
    pub is_direct: bool,
    pub attribs: Cow<'input, [u32]>,
}
impl<'input> CreateContextAttribsARBRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let screen_bytes = self.screen.serialize();
        let share_list_bytes = self.share_list.serialize();
        let is_direct_bytes = self.is_direct.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_CONTEXT_ATTRIBS_ARB_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let attribs_bytes = self.attribs.serialize();
        let length_so_far = length_so_far + attribs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), attribs_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_CONTEXT_ATTRIBS_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let (fbconfig, remaining) = Fbconfig::try_parse(remaining)?;
        let (screen, remaining) = u32::try_parse(remaining)?;
        let (share_list, remaining) = Context::try_parse(remaining)?;
        let (is_direct, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let _ = remaining;
        Ok(CreateContextAttribsARBRequest {
            context,
            fbconfig,
            screen,
            share_list,
            is_direct,
            attribs: Cow::Owned(attribs),
        })
    }
    /// Clone all borrowed data in this CreateContextAttribsARBRequest.
    pub fn into_owned(self) -> CreateContextAttribsARBRequest<'static> {
        CreateContextAttribsARBRequest {
            context: self.context,
            fbconfig: self.fbconfig,
            screen: self.screen,
            share_list: self.share_list,
            is_direct: self.is_direct,
            attribs: Cow::Owned(self.attribs.into_owned()),
        }
    }
}
impl<'input> Request for CreateContextAttribsARBRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for CreateContextAttribsARBRequest<'input> {
}

/// Opcode for the SetClientInfo2ARB request
pub const SET_CLIENT_INFO2_ARB_REQUEST: u8 = 35;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetClientInfo2ARBRequest<'input> {
    pub major_version: u32,
    pub minor_version: u32,
    pub gl_versions: Cow<'input, [u32]>,
    pub gl_extension_string: Cow<'input, [u8]>,
    pub glx_extension_string: Cow<'input, [u8]>,
}
impl<'input> SetClientInfo2ARBRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        assert_eq!(self.gl_versions.len() % 3, 0, "`gl_versions` has an incorrect length, must be a multiple of 3");
        let num_versions = u32::try_from(self.gl_versions.len() / 3).expect("`gl_versions` has too many elements");
        let num_versions_bytes = num_versions.serialize();
        let gl_str_len = u32::try_from(self.gl_extension_string.len()).expect("`gl_extension_string` has too many elements");
        let gl_str_len_bytes = gl_str_len.serialize();
        let glx_str_len = u32::try_from(self.glx_extension_string.len()).expect("`glx_extension_string` has too many elements");
        let glx_str_len_bytes = glx_str_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_CLIENT_INFO2_ARB_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        let gl_versions_bytes = self.gl_versions.serialize();
        let length_so_far = length_so_far + gl_versions_bytes.len();
        let length_so_far = length_so_far + self.gl_extension_string.len();
        let length_so_far = length_so_far + self.glx_extension_string.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), gl_versions_bytes.into(), self.gl_extension_string, self.glx_extension_string, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_CLIENT_INFO2_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (major_version, remaining) = u32::try_parse(value)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let (num_versions, remaining) = u32::try_parse(remaining)?;
        let (gl_str_len, remaining) = u32::try_parse(remaining)?;
        let (glx_str_len, remaining) = u32::try_parse(remaining)?;
        let (gl_versions, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_versions.checked_mul(3u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let (gl_extension_string, remaining) = crate::x11_utils::parse_u8_list(remaining, gl_str_len.try_to_usize()?)?;
        let (glx_extension_string, remaining) = crate::x11_utils::parse_u8_list(remaining, glx_str_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetClientInfo2ARBRequest {
            major_version,
            minor_version,
            gl_versions: Cow::Owned(gl_versions),
            gl_extension_string: Cow::Borrowed(gl_extension_string),
            glx_extension_string: Cow::Borrowed(glx_extension_string),
        })
    }
    /// Clone all borrowed data in this SetClientInfo2ARBRequest.
    pub fn into_owned(self) -> SetClientInfo2ARBRequest<'static> {
        SetClientInfo2ARBRequest {
            major_version: self.major_version,
            minor_version: self.minor_version,
            gl_versions: Cow::Owned(self.gl_versions.into_owned()),
            gl_extension_string: Cow::Owned(self.gl_extension_string.into_owned()),
            glx_extension_string: Cow::Owned(self.glx_extension_string.into_owned()),
        }
    }
}
impl<'input> Request for SetClientInfo2ARBRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetClientInfo2ARBRequest<'input> {
}

/// Opcode for the NewList request
pub const NEW_LIST_REQUEST: u8 = 101;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewListRequest {
    pub context_tag: ContextTag,
    pub list: u32,
    pub mode: u32,
}
impl NewListRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let list_bytes = self.list.serialize();
        let mode_bytes = self.mode.serialize();
        let mut request0 = vec![
            major_opcode,
            NEW_LIST_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != NEW_LIST_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (list, remaining) = u32::try_parse(remaining)?;
        let (mode, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(NewListRequest {
            context_tag,
            list,
            mode,
        })
    }
}
impl Request for NewListRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for NewListRequest {
}

/// Opcode for the EndList request
pub const END_LIST_REQUEST: u8 = 102;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndListRequest {
    pub context_tag: ContextTag,
}
impl EndListRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            END_LIST_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != END_LIST_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let _ = remaining;
        Ok(EndListRequest {
            context_tag,
        })
    }
}
impl Request for EndListRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for EndListRequest {
}

/// Opcode for the DeleteLists request
pub const DELETE_LISTS_REQUEST: u8 = 103;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteListsRequest {
    pub context_tag: ContextTag,
    pub list: u32,
    pub range: i32,
}
impl DeleteListsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let list_bytes = self.list.serialize();
        let range_bytes = self.range.serialize();
        let mut request0 = vec![
            major_opcode,
            DELETE_LISTS_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DELETE_LISTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (list, remaining) = u32::try_parse(remaining)?;
        let (range, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(DeleteListsRequest {
            context_tag,
            list,
            range,
        })
    }
}
impl Request for DeleteListsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for DeleteListsRequest {
}

/// Opcode for the GenLists request
pub const GEN_LISTS_REQUEST: u8 = 104;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenListsRequest {
    pub context_tag: ContextTag,
    pub range: i32,
}
impl GenListsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let range_bytes = self.range.serialize();
        let mut request0 = vec![
            major_opcode,
            GEN_LISTS_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            range_bytes[0],
            range_bytes[1],
            range_bytes[2],
            range_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GEN_LISTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (range, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GenListsRequest {
            context_tag,
            range,
        })
    }
}
impl Request for GenListsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GenListsRequest {
    type Reply = GenListsReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenListsReply {
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
}
impl TryParse for GenListsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GenListsReply { sequence, length, ret_val };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GenListsReply {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let ret_val_bytes = self.ret_val.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            ret_val_bytes[0],
            ret_val_bytes[1],
            ret_val_bytes[2],
            ret_val_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.ret_val.serialize_into(bytes);
    }
}
#[cfg(test)]
mod gen_lists_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GenListsReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let ret_val: u32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                ret_val,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6363658479379296256);
        let value = GenListsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the FeedbackBuffer request
pub const FEEDBACK_BUFFER_REQUEST: u8 = 105;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedbackBufferRequest {
    pub context_tag: ContextTag,
    pub size: i32,
    pub type_: i32,
}
impl FeedbackBufferRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let size_bytes = self.size.serialize();
        let type_bytes = self.type_.serialize();
        let mut request0 = vec![
            major_opcode,
            FEEDBACK_BUFFER_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != FEEDBACK_BUFFER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (size, remaining) = i32::try_parse(remaining)?;
        let (type_, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(FeedbackBufferRequest {
            context_tag,
            size,
            type_,
        })
    }
}
impl Request for FeedbackBufferRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for FeedbackBufferRequest {
}

/// Opcode for the SelectBuffer request
pub const SELECT_BUFFER_REQUEST: u8 = 106;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectBufferRequest {
    pub context_tag: ContextTag,
    pub size: i32,
}
impl SelectBufferRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let size_bytes = self.size.serialize();
        let mut request0 = vec![
            major_opcode,
            SELECT_BUFFER_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SELECT_BUFFER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (size, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SelectBufferRequest {
            context_tag,
            size,
        })
    }
}
impl Request for SelectBufferRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for SelectBufferRequest {
}

/// Opcode for the RenderMode request
pub const RENDER_MODE_REQUEST: u8 = 107;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderModeRequest {
    pub context_tag: ContextTag,
    pub mode: u32,
}
impl RenderModeRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mode_bytes = self.mode.serialize();
        let mut request0 = vec![
            major_opcode,
            RENDER_MODE_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            mode_bytes[0],
            mode_bytes[1],
            mode_bytes[2],
            mode_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != RENDER_MODE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (mode, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(RenderModeRequest {
            context_tag,
            mode,
        })
    }
}
impl Request for RenderModeRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for RenderModeRequest {
    type Reply = RenderModeReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderModeReply {
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
    pub new_mode: u32,
    pub data: Vec<u32>,
}
impl TryParse for RenderModeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = u32::try_parse(remaining)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (new_mode, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = RenderModeReply { sequence, length, ret_val, new_mode, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for RenderModeReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.ret_val.serialize_into(bytes);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.new_mode.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl RenderModeReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod render_mode_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for RenderModeReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let ret_val: u32 = GenRandom::generate(rng);
            let new_mode: u32 = GenRandom::generate(rng);
            let data: Vec<u32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                ret_val,
                new_mode,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5941499723003478016);
        let value = RenderModeReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RM(u16);
impl RM {
    pub const GL_RENDER: Self = Self(7168);
    pub const GL_FEEDBACK: Self = Self(7169);
    pub const GL_SELECT: Self = Self(7170);
}
impl From<RM> for u16 {
    #[inline]
    fn from(input: RM) -> Self {
        input.0
    }
}
impl From<RM> for Option<u16> {
    #[inline]
    fn from(input: RM) -> Self {
        Some(input.0)
    }
}
impl From<RM> for u32 {
    #[inline]
    fn from(input: RM) -> Self {
        u32::from(input.0)
    }
}
impl From<RM> for Option<u32> {
    #[inline]
    fn from(input: RM) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for RM {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for RM {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for RM  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::GL_RENDER.0.into(), "GL_RENDER", "GL_RENDER"),
            (Self::GL_FEEDBACK.0.into(), "GL_FEEDBACK", "GL_FEEDBACK"),
            (Self::GL_SELECT.0.into(), "GL_SELECT", "GL_SELECT"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}
#[cfg(test)]
impl crate::x11_utils::GenRandom for RM {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::GL_RENDER.0,
            Self::GL_FEEDBACK.0,
            Self::GL_SELECT.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

/// Opcode for the Finish request
pub const FINISH_REQUEST: u8 = 108;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FinishRequest {
    pub context_tag: ContextTag,
}
impl FinishRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            FINISH_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != FINISH_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let _ = remaining;
        Ok(FinishRequest {
            context_tag,
        })
    }
}
impl Request for FinishRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for FinishRequest {
    type Reply = FinishReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FinishReply {
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for FinishReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = FinishReply { sequence, length };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for FinishReply {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
    }
}
#[cfg(test)]
mod finish_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for FinishReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3324270677610672128);
        let value = FinishReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the PixelStoref request
pub const PIXEL_STOREF_REQUEST: u8 = 109;
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PixelStorefRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
    pub datum: Float32,
}
impl PixelStorefRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let datum_bytes = self.datum.serialize();
        let mut request0 = vec![
            major_opcode,
            PIXEL_STOREF_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PIXEL_STOREF_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let _ = remaining;
        Ok(PixelStorefRequest {
            context_tag,
            pname,
            datum,
        })
    }
}
impl Request for PixelStorefRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PixelStorefRequest {
}

/// Opcode for the PixelStorei request
pub const PIXEL_STOREI_REQUEST: u8 = 110;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PixelStoreiRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
    pub datum: i32,
}
impl PixelStoreiRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let datum_bytes = self.datum.serialize();
        let mut request0 = vec![
            major_opcode,
            PIXEL_STOREI_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PIXEL_STOREI_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(PixelStoreiRequest {
            context_tag,
            pname,
            datum,
        })
    }
}
impl Request for PixelStoreiRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PixelStoreiRequest {
}

/// Opcode for the ReadPixels request
pub const READ_PIXELS_REQUEST: u8 = 111;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadPixelsRequest {
    pub context_tag: ContextTag,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
    pub lsb_first: bool,
}
impl ReadPixelsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let lsb_first_bytes = self.lsb_first.serialize();
        let mut request0 = vec![
            major_opcode,
            READ_PIXELS_REQUEST,
            0,
            0,
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
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != READ_PIXELS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (x, remaining) = i32::try_parse(remaining)?;
        let (y, remaining) = i32::try_parse(remaining)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let (format, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u32::try_parse(remaining)?;
        let (swap_bytes, remaining) = bool::try_parse(remaining)?;
        let (lsb_first, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(ReadPixelsRequest {
            context_tag,
            x,
            y,
            width,
            height,
            format,
            type_,
            swap_bytes,
            lsb_first,
        })
    }
}
impl Request for ReadPixelsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for ReadPixelsRequest {
    type Reply = ReadPixelsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadPixelsReply {
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl TryParse for ReadPixelsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ReadPixelsReply { sequence, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ReadPixelsReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
        bytes.extend_from_slice(&self.data);
    }
}
impl ReadPixelsReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod read_pixels_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ReadPixelsReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let data: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(18126840124226928640);
        let value = ReadPixelsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetBooleanv request
pub const GET_BOOLEANV_REQUEST: u8 = 112;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetBooleanvRequest {
    pub context_tag: ContextTag,
    pub pname: i32,
}
impl GetBooleanvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_BOOLEANV_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            pname_bytes[0],
            pname_bytes[1],
            pname_bytes[2],
            pname_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_BOOLEANV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (pname, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetBooleanvRequest {
            context_tag,
            pname,
        })
    }
}
impl Request for GetBooleanvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetBooleanvRequest {
    type Reply = GetBooleanvReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetBooleanvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: bool,
    pub data: Vec<bool>,
}
impl TryParse for GetBooleanvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<bool>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetBooleanvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetBooleanvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 15]);
        self.data.serialize_into(bytes);
    }
}
impl GetBooleanvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_booleanv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetBooleanvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: bool = GenRandom::generate(rng);
            let data: Vec<bool> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11459871985999233024);
        let value = GetBooleanvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetClipPlane request
pub const GET_CLIP_PLANE_REQUEST: u8 = 113;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetClipPlaneRequest {
    pub context_tag: ContextTag,
    pub plane: i32,
}
impl GetClipPlaneRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let plane_bytes = self.plane.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_CLIP_PLANE_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            plane_bytes[0],
            plane_bytes[1],
            plane_bytes[2],
            plane_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CLIP_PLANE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (plane, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetClipPlaneRequest {
            context_tag,
            plane,
        })
    }
}
impl Request for GetClipPlaneRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetClipPlaneRequest {
    type Reply = GetClipPlaneReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetClipPlaneReply {
    pub sequence: u16,
    pub data: Vec<Float64>,
}
impl TryParse for GetClipPlaneReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, length.checked_div(2u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetClipPlaneReply { sequence, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetClipPlaneReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        let length = u32::try_from(self.data.len()).ok().and_then(|len| len.checked_mul(2)).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
        self.data.serialize_into(bytes);
    }
}
impl GetClipPlaneReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_mul(2).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_clip_plane_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetClipPlaneReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let data: Vec<Float64> = gen_random_list(rng, usize::try_from(length.checked_div(2u32).unwrap()).unwrap());
            Self {
                sequence,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(16826379829401092096);
        let value = GetClipPlaneReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetDoublev request
pub const GET_DOUBLEV_REQUEST: u8 = 114;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDoublevRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
}
impl GetDoublevRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DOUBLEV_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            pname_bytes[0],
            pname_bytes[1],
            pname_bytes[2],
            pname_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DOUBLEV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetDoublevRequest {
            context_tag,
            pname,
        })
    }
}
impl Request for GetDoublevRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDoublevRequest {
    type Reply = GetDoublevReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDoublevReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl TryParse for GetDoublevReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDoublevReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDoublevReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.data.serialize_into(bytes);
    }
}
impl GetDoublevReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_doublev_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetDoublevReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float64 = GenRandom::generate(rng);
            let data: Vec<Float64> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(101928068251418624);
        let value = GetDoublevReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetError request
pub const GET_ERROR_REQUEST: u8 = 115;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetErrorRequest {
    pub context_tag: ContextTag,
}
impl GetErrorRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_ERROR_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_ERROR_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let _ = remaining;
        Ok(GetErrorRequest {
            context_tag,
        })
    }
}
impl Request for GetErrorRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetErrorRequest {
    type Reply = GetErrorReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetErrorReply {
    pub sequence: u16,
    pub length: u32,
    pub error: i32,
}
impl TryParse for GetErrorReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (error, remaining) = i32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetErrorReply { sequence, length, error };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetErrorReply {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let error_bytes = self.error.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            error_bytes[0],
            error_bytes[1],
            error_bytes[2],
            error_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.error.serialize_into(bytes);
    }
}
#[cfg(test)]
mod get_error_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetErrorReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let error: i32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                error,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1160541281073844224);
        let value = GetErrorReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetFloatv request
pub const GET_FLOATV_REQUEST: u8 = 116;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetFloatvRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
}
impl GetFloatvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_FLOATV_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            pname_bytes[0],
            pname_bytes[1],
            pname_bytes[2],
            pname_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_FLOATV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetFloatvRequest {
            context_tag,
            pname,
        })
    }
}
impl Request for GetFloatvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetFloatvRequest {
    type Reply = GetFloatvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetFloatvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetFloatvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetFloatvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetFloatvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetFloatvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_floatv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetFloatvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15829631814565888);
        let value = GetFloatvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetIntegerv request
pub const GET_INTEGERV_REQUEST: u8 = 117;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetIntegervRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
}
impl GetIntegervRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_INTEGERV_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            pname_bytes[0],
            pname_bytes[1],
            pname_bytes[2],
            pname_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_INTEGERV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetIntegervRequest {
            context_tag,
            pname,
        })
    }
}
impl Request for GetIntegervRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetIntegervRequest {
    type Reply = GetIntegervReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetIntegervReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetIntegervReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetIntegervReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetIntegervReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetIntegervReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_integerv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetIntegervReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3682935519726551040);
        let value = GetIntegervReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetLightfv request
pub const GET_LIGHTFV_REQUEST: u8 = 118;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetLightfvRequest {
    pub context_tag: ContextTag,
    pub light: u32,
    pub pname: u32,
}
impl GetLightfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let light_bytes = self.light.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_LIGHTFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_LIGHTFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (light, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetLightfvRequest {
            context_tag,
            light,
            pname,
        })
    }
}
impl Request for GetLightfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetLightfvRequest {
    type Reply = GetLightfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetLightfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetLightfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetLightfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetLightfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetLightfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_lightfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetLightfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17352592232192671744);
        let value = GetLightfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetLightiv request
pub const GET_LIGHTIV_REQUEST: u8 = 119;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetLightivRequest {
    pub context_tag: ContextTag,
    pub light: u32,
    pub pname: u32,
}
impl GetLightivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let light_bytes = self.light.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_LIGHTIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_LIGHTIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (light, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetLightivRequest {
            context_tag,
            light,
            pname,
        })
    }
}
impl Request for GetLightivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetLightivRequest {
    type Reply = GetLightivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetLightivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetLightivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetLightivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetLightivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetLightivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_lightiv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetLightivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11352347036536143872);
        let value = GetLightivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetMapdv request
pub const GET_MAPDV_REQUEST: u8 = 120;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMapdvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub query: u32,
}
impl GetMapdvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let query_bytes = self.query.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MAPDV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MAPDV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (query, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMapdvRequest {
            context_tag,
            target,
            query,
        })
    }
}
impl Request for GetMapdvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMapdvRequest {
    type Reply = GetMapdvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMapdvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl TryParse for GetMapdvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMapdvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetMapdvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.data.serialize_into(bytes);
    }
}
impl GetMapdvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_mapdv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetMapdvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float64 = GenRandom::generate(rng);
            let data: Vec<Float64> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11152702160938401792);
        let value = GetMapdvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetMapfv request
pub const GET_MAPFV_REQUEST: u8 = 121;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMapfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub query: u32,
}
impl GetMapfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let query_bytes = self.query.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MAPFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MAPFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (query, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMapfvRequest {
            context_tag,
            target,
            query,
        })
    }
}
impl Request for GetMapfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMapfvRequest {
    type Reply = GetMapfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMapfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetMapfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMapfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetMapfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetMapfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_mapfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetMapfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6948537626466877440);
        let value = GetMapfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetMapiv request
pub const GET_MAPIV_REQUEST: u8 = 122;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMapivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub query: u32,
}
impl GetMapivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let query_bytes = self.query.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MAPIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MAPIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (query, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMapivRequest {
            context_tag,
            target,
            query,
        })
    }
}
impl Request for GetMapivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMapivRequest {
    type Reply = GetMapivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMapivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetMapivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMapivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetMapivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetMapivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_mapiv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetMapivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9865662861614366720);
        let value = GetMapivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetMaterialfv request
pub const GET_MATERIALFV_REQUEST: u8 = 123;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMaterialfvRequest {
    pub context_tag: ContextTag,
    pub face: u32,
    pub pname: u32,
}
impl GetMaterialfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let face_bytes = self.face.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MATERIALFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MATERIALFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (face, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMaterialfvRequest {
            context_tag,
            face,
            pname,
        })
    }
}
impl Request for GetMaterialfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMaterialfvRequest {
    type Reply = GetMaterialfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMaterialfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetMaterialfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMaterialfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetMaterialfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetMaterialfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_materialfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetMaterialfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14587591306169090048);
        let value = GetMaterialfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetMaterialiv request
pub const GET_MATERIALIV_REQUEST: u8 = 124;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMaterialivRequest {
    pub context_tag: ContextTag,
    pub face: u32,
    pub pname: u32,
}
impl GetMaterialivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let face_bytes = self.face.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MATERIALIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MATERIALIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (face, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMaterialivRequest {
            context_tag,
            face,
            pname,
        })
    }
}
impl Request for GetMaterialivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMaterialivRequest {
    type Reply = GetMaterialivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMaterialivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetMaterialivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMaterialivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetMaterialivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetMaterialivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_materialiv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetMaterialivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14474086813006135296);
        let value = GetMaterialivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetPixelMapfv request
pub const GET_PIXEL_MAPFV_REQUEST: u8 = 125;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPixelMapfvRequest {
    pub context_tag: ContextTag,
    pub map: u32,
}
impl GetPixelMapfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let map_bytes = self.map.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_PIXEL_MAPFV_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            map_bytes[0],
            map_bytes[1],
            map_bytes[2],
            map_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PIXEL_MAPFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (map, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPixelMapfvRequest {
            context_tag,
            map,
        })
    }
}
impl Request for GetPixelMapfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPixelMapfvRequest {
    type Reply = GetPixelMapfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPixelMapfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetPixelMapfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPixelMapfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetPixelMapfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetPixelMapfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_pixel_mapfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetPixelMapfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9186480597281800192);
        let value = GetPixelMapfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetPixelMapuiv request
pub const GET_PIXEL_MAPUIV_REQUEST: u8 = 126;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPixelMapuivRequest {
    pub context_tag: ContextTag,
    pub map: u32,
}
impl GetPixelMapuivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let map_bytes = self.map.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_PIXEL_MAPUIV_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            map_bytes[0],
            map_bytes[1],
            map_bytes[2],
            map_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PIXEL_MAPUIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (map, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPixelMapuivRequest {
            context_tag,
            map,
        })
    }
}
impl Request for GetPixelMapuivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPixelMapuivRequest {
    type Reply = GetPixelMapuivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPixelMapuivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: u32,
    pub data: Vec<u32>,
}
impl TryParse for GetPixelMapuivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPixelMapuivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetPixelMapuivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetPixelMapuivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_pixel_mapuiv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetPixelMapuivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: u32 = GenRandom::generate(rng);
            let data: Vec<u32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8849258375133790208);
        let value = GetPixelMapuivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetPixelMapusv request
pub const GET_PIXEL_MAPUSV_REQUEST: u8 = 127;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPixelMapusvRequest {
    pub context_tag: ContextTag,
    pub map: u32,
}
impl GetPixelMapusvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let map_bytes = self.map.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_PIXEL_MAPUSV_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            map_bytes[0],
            map_bytes[1],
            map_bytes[2],
            map_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PIXEL_MAPUSV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (map, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPixelMapusvRequest {
            context_tag,
            map,
        })
    }
}
impl Request for GetPixelMapusvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPixelMapusvRequest {
    type Reply = GetPixelMapusvReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPixelMapusvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: u16,
    pub data: Vec<u16>,
}
impl TryParse for GetPixelMapusvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u16>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPixelMapusvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetPixelMapusvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(34);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
        self.data.serialize_into(bytes);
    }
}
impl GetPixelMapusvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_pixel_mapusv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetPixelMapusvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: u16 = GenRandom::generate(rng);
            let data: Vec<u16> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3543130195814776832);
        let value = GetPixelMapusvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetPolygonStipple request
pub const GET_POLYGON_STIPPLE_REQUEST: u8 = 128;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPolygonStippleRequest {
    pub context_tag: ContextTag,
    pub lsb_first: bool,
}
impl GetPolygonStippleRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let lsb_first_bytes = self.lsb_first.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_POLYGON_STIPPLE_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            lsb_first_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_POLYGON_STIPPLE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (lsb_first, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPolygonStippleRequest {
            context_tag,
            lsb_first,
        })
    }
}
impl Request for GetPolygonStippleRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPolygonStippleRequest {
    type Reply = GetPolygonStippleReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPolygonStippleReply {
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl TryParse for GetPolygonStippleReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPolygonStippleReply { sequence, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetPolygonStippleReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
        bytes.extend_from_slice(&self.data);
    }
}
impl GetPolygonStippleReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_polygon_stipple_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetPolygonStippleReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let data: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15814776368253632512);
        let value = GetPolygonStippleReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetString request
pub const GET_STRING_REQUEST: u8 = 129;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetStringRequest {
    pub context_tag: ContextTag,
    pub name: u32,
}
impl GetStringRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let name_bytes = self.name.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_STRING_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_STRING_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (name, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetStringRequest {
            context_tag,
            name,
        })
    }
}
impl Request for GetStringRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetStringRequest {
    type Reply = GetStringReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetStringReply {
    pub sequence: u16,
    pub length: u32,
    pub string: Vec<u8>,
}
impl TryParse for GetStringReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, n.try_to_usize()?)?;
        let string = string.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetStringReply { sequence, length, string };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetStringReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.string.len()).expect("`string` has too many elements");
        n.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 16]);
        bytes.extend_from_slice(&self.string);
    }
}
impl GetStringReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `string` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.string.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_string_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetStringReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let string: Vec<u8> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                string,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2605158336947953664);
        let value = GetStringReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexEnvfv request
pub const GET_TEX_ENVFV_REQUEST: u8 = 130;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexEnvfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetTexEnvfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_ENVFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_ENVFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexEnvfvRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetTexEnvfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexEnvfvRequest {
    type Reply = GetTexEnvfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexEnvfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetTexEnvfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexEnvfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexEnvfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexEnvfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_envfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexEnvfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10922631212501827584);
        let value = GetTexEnvfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexEnviv request
pub const GET_TEX_ENVIV_REQUEST: u8 = 131;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexEnvivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetTexEnvivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_ENVIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_ENVIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexEnvivRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetTexEnvivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexEnvivRequest {
    type Reply = GetTexEnvivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexEnvivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetTexEnvivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexEnvivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexEnvivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexEnvivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_enviv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexEnvivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(935410442267131904);
        let value = GetTexEnvivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexGendv request
pub const GET_TEX_GENDV_REQUEST: u8 = 132;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexGendvRequest {
    pub context_tag: ContextTag,
    pub coord: u32,
    pub pname: u32,
}
impl GetTexGendvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let coord_bytes = self.coord.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_GENDV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_GENDV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (coord, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexGendvRequest {
            context_tag,
            coord,
            pname,
        })
    }
}
impl Request for GetTexGendvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexGendvRequest {
    type Reply = GetTexGendvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexGendvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl TryParse for GetTexGendvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexGendvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexGendvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexGendvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_gendv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexGendvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float64 = GenRandom::generate(rng);
            let data: Vec<Float64> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3644709683030917120);
        let value = GetTexGendvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexGenfv request
pub const GET_TEX_GENFV_REQUEST: u8 = 133;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexGenfvRequest {
    pub context_tag: ContextTag,
    pub coord: u32,
    pub pname: u32,
}
impl GetTexGenfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let coord_bytes = self.coord.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_GENFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_GENFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (coord, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexGenfvRequest {
            context_tag,
            coord,
            pname,
        })
    }
}
impl Request for GetTexGenfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexGenfvRequest {
    type Reply = GetTexGenfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexGenfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetTexGenfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexGenfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexGenfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexGenfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_genfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexGenfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5562278284062490624);
        let value = GetTexGenfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexGeniv request
pub const GET_TEX_GENIV_REQUEST: u8 = 134;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexGenivRequest {
    pub context_tag: ContextTag,
    pub coord: u32,
    pub pname: u32,
}
impl GetTexGenivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let coord_bytes = self.coord.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_GENIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_GENIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (coord, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexGenivRequest {
            context_tag,
            coord,
            pname,
        })
    }
}
impl Request for GetTexGenivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexGenivRequest {
    type Reply = GetTexGenivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexGenivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetTexGenivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexGenivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexGenivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexGenivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_geniv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexGenivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17662003222464626688);
        let value = GetTexGenivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexImage request
pub const GET_TEX_IMAGE_REQUEST: u8 = 135;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexImageRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub level: i32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
}
impl GetTexImageRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let level_bytes = self.level.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_IMAGE_REQUEST,
            0,
            0,
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
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_IMAGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (level, remaining) = i32::try_parse(remaining)?;
        let (format, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u32::try_parse(remaining)?;
        let (swap_bytes, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexImageRequest {
            context_tag,
            target,
            level,
            format,
            type_,
            swap_bytes,
        })
    }
}
impl Request for GetTexImageRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexImageRequest {
    type Reply = GetTexImageReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexImageReply {
    pub sequence: u16,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetTexImageReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let (depth, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexImageReply { sequence, width, height, depth, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexImageReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.depth.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        bytes.extend_from_slice(&self.data);
    }
}
impl GetTexImageReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_image_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexImageReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let width: i32 = GenRandom::generate(rng);
            let height: i32 = GenRandom::generate(rng);
            let depth: i32 = GenRandom::generate(rng);
            let data: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                width,
                height,
                depth,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4233292346658897920);
        let value = GetTexImageReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexParameterfv request
pub const GET_TEX_PARAMETERFV_REQUEST: u8 = 136;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetTexParameterfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_PARAMETERFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_PARAMETERFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexParameterfvRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetTexParameterfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexParameterfvRequest {
    type Reply = GetTexParameterfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexParameterfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetTexParameterfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexParameterfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexParameterfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexParameterfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_parameterfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexParameterfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(14868383753658433536);
        let value = GetTexParameterfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexParameteriv request
pub const GET_TEX_PARAMETERIV_REQUEST: u8 = 137;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetTexParameterivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_PARAMETERIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_PARAMETERIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexParameterivRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetTexParameterivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexParameterivRequest {
    type Reply = GetTexParameterivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexParameterivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetTexParameterivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexParameterivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexParameterivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexParameterivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_parameteriv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexParameterivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1741906751038423040);
        let value = GetTexParameterivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexLevelParameterfv request
pub const GET_TEX_LEVEL_PARAMETERFV_REQUEST: u8 = 138;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexLevelParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub level: i32,
    pub pname: u32,
}
impl GetTexLevelParameterfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let level_bytes = self.level.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_LEVEL_PARAMETERFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_LEVEL_PARAMETERFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (level, remaining) = i32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexLevelParameterfvRequest {
            context_tag,
            target,
            level,
            pname,
        })
    }
}
impl Request for GetTexLevelParameterfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexLevelParameterfvRequest {
    type Reply = GetTexLevelParameterfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexLevelParameterfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetTexLevelParameterfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexLevelParameterfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexLevelParameterfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexLevelParameterfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_level_parameterfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexLevelParameterfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17586942387026919424);
        let value = GetTexLevelParameterfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetTexLevelParameteriv request
pub const GET_TEX_LEVEL_PARAMETERIV_REQUEST: u8 = 139;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexLevelParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub level: i32,
    pub pname: u32,
}
impl GetTexLevelParameterivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let level_bytes = self.level.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_TEX_LEVEL_PARAMETERIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_TEX_LEVEL_PARAMETERIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (level, remaining) = i32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetTexLevelParameterivRequest {
            context_tag,
            target,
            level,
            pname,
        })
    }
}
impl Request for GetTexLevelParameterivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetTexLevelParameterivRequest {
    type Reply = GetTexLevelParameterivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetTexLevelParameterivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetTexLevelParameterivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetTexLevelParameterivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetTexLevelParameterivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetTexLevelParameterivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_tex_level_parameteriv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetTexLevelParameterivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(742563917271662592);
        let value = GetTexLevelParameterivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the IsEnabled request
pub const IS_ENABLED_REQUEST: u8 = 140;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsEnabledRequest {
    pub context_tag: ContextTag,
    pub capability: u32,
}
impl IsEnabledRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let capability_bytes = self.capability.serialize();
        let mut request0 = vec![
            major_opcode,
            IS_ENABLED_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            capability_bytes[0],
            capability_bytes[1],
            capability_bytes[2],
            capability_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != IS_ENABLED_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (capability, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(IsEnabledRequest {
            context_tag,
            capability,
        })
    }
}
impl Request for IsEnabledRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for IsEnabledRequest {
    type Reply = IsEnabledReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsEnabledReply {
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl TryParse for IsEnabledReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = IsEnabledReply { sequence, length, ret_val };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for IsEnabledReply {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let ret_val_bytes = self.ret_val.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            ret_val_bytes[0],
            ret_val_bytes[1],
            ret_val_bytes[2],
            ret_val_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.ret_val.serialize_into(bytes);
    }
}
#[cfg(test)]
mod is_enabled_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for IsEnabledReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let ret_val: Bool32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                ret_val,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13173746225038483456);
        let value = IsEnabledReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the IsList request
pub const IS_LIST_REQUEST: u8 = 141;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsListRequest {
    pub context_tag: ContextTag,
    pub list: u32,
}
impl IsListRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let list_bytes = self.list.serialize();
        let mut request0 = vec![
            major_opcode,
            IS_LIST_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            list_bytes[0],
            list_bytes[1],
            list_bytes[2],
            list_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != IS_LIST_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (list, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(IsListRequest {
            context_tag,
            list,
        })
    }
}
impl Request for IsListRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for IsListRequest {
    type Reply = IsListReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsListReply {
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl TryParse for IsListReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = IsListReply { sequence, length, ret_val };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for IsListReply {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let ret_val_bytes = self.ret_val.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            ret_val_bytes[0],
            ret_val_bytes[1],
            ret_val_bytes[2],
            ret_val_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.ret_val.serialize_into(bytes);
    }
}
#[cfg(test)]
mod is_list_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for IsListReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let ret_val: Bool32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                ret_val,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4587942290356369408);
        let value = IsListReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the Flush request
pub const FLUSH_REQUEST: u8 = 142;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushRequest {
    pub context_tag: ContextTag,
}
impl FlushRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            major_opcode,
            FLUSH_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != FLUSH_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let _ = remaining;
        Ok(FlushRequest {
            context_tag,
        })
    }
}
impl Request for FlushRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for FlushRequest {
}

/// Opcode for the AreTexturesResident request
pub const ARE_TEXTURES_RESIDENT_REQUEST: u8 = 143;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreTexturesResidentRequest<'input> {
    pub context_tag: ContextTag,
    pub textures: Cow<'input, [u32]>,
}
impl<'input> AreTexturesResidentRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n = i32::try_from(self.textures.len()).expect("`textures` has too many elements");
        let n_bytes = n.serialize();
        let mut request0 = vec![
            major_opcode,
            ARE_TEXTURES_RESIDENT_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            n_bytes[0],
            n_bytes[1],
            n_bytes[2],
            n_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let textures_bytes = self.textures.serialize();
        let length_so_far = length_so_far + textures_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), textures_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != ARE_TEXTURES_RESIDENT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (n, remaining) = i32::try_parse(remaining)?;
        let (textures, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_to_usize()?)?;
        let _ = remaining;
        Ok(AreTexturesResidentRequest {
            context_tag,
            textures: Cow::Owned(textures),
        })
    }
    /// Clone all borrowed data in this AreTexturesResidentRequest.
    pub fn into_owned(self) -> AreTexturesResidentRequest<'static> {
        AreTexturesResidentRequest {
            context_tag: self.context_tag,
            textures: Cow::Owned(self.textures.into_owned()),
        }
    }
}
impl<'input> Request for AreTexturesResidentRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for AreTexturesResidentRequest<'input> {
    type Reply = AreTexturesResidentReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreTexturesResidentReply {
    pub sequence: u16,
    pub ret_val: Bool32,
    pub data: Vec<bool>,
}
impl TryParse for AreTexturesResidentReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<bool>(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = AreTexturesResidentReply { sequence, ret_val, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for AreTexturesResidentReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        self.ret_val.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.data.serialize_into(bytes);
    }
}
impl AreTexturesResidentReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod are_textures_resident_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for AreTexturesResidentReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let ret_val: Bool32 = GenRandom::generate(rng);
            let data: Vec<bool> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                ret_val,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7851241948319842304);
        let value = AreTexturesResidentReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the DeleteTextures request
pub const DELETE_TEXTURES_REQUEST: u8 = 144;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteTexturesRequest<'input> {
    pub context_tag: ContextTag,
    pub textures: Cow<'input, [u32]>,
}
impl<'input> DeleteTexturesRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n = i32::try_from(self.textures.len()).expect("`textures` has too many elements");
        let n_bytes = n.serialize();
        let mut request0 = vec![
            major_opcode,
            DELETE_TEXTURES_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            n_bytes[0],
            n_bytes[1],
            n_bytes[2],
            n_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let textures_bytes = self.textures.serialize();
        let length_so_far = length_so_far + textures_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), textures_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DELETE_TEXTURES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (n, remaining) = i32::try_parse(remaining)?;
        let (textures, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_to_usize()?)?;
        let _ = remaining;
        Ok(DeleteTexturesRequest {
            context_tag,
            textures: Cow::Owned(textures),
        })
    }
    /// Clone all borrowed data in this DeleteTexturesRequest.
    pub fn into_owned(self) -> DeleteTexturesRequest<'static> {
        DeleteTexturesRequest {
            context_tag: self.context_tag,
            textures: Cow::Owned(self.textures.into_owned()),
        }
    }
}
impl<'input> Request for DeleteTexturesRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for DeleteTexturesRequest<'input> {
}

/// Opcode for the GenTextures request
pub const GEN_TEXTURES_REQUEST: u8 = 145;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenTexturesRequest {
    pub context_tag: ContextTag,
    pub n: i32,
}
impl GenTexturesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n_bytes = self.n.serialize();
        let mut request0 = vec![
            major_opcode,
            GEN_TEXTURES_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            n_bytes[0],
            n_bytes[1],
            n_bytes[2],
            n_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GEN_TEXTURES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (n, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GenTexturesRequest {
            context_tag,
            n,
        })
    }
}
impl Request for GenTexturesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GenTexturesRequest {
    type Reply = GenTexturesReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenTexturesReply {
    pub sequence: u16,
    pub data: Vec<u32>,
}
impl TryParse for GenTexturesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GenTexturesReply { sequence, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GenTexturesReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        let length = u32::try_from(self.data.len()).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
        self.data.serialize_into(bytes);
    }
}
impl GenTexturesReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod gen_textures_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GenTexturesReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let data: Vec<u32> = gen_random_list(rng, usize::try_from(length).unwrap());
            Self {
                sequence,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10289315535470788608);
        let value = GenTexturesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the IsTexture request
pub const IS_TEXTURE_REQUEST: u8 = 146;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsTextureRequest {
    pub context_tag: ContextTag,
    pub texture: u32,
}
impl IsTextureRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let texture_bytes = self.texture.serialize();
        let mut request0 = vec![
            major_opcode,
            IS_TEXTURE_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            texture_bytes[0],
            texture_bytes[1],
            texture_bytes[2],
            texture_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != IS_TEXTURE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (texture, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(IsTextureRequest {
            context_tag,
            texture,
        })
    }
}
impl Request for IsTextureRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for IsTextureRequest {
    type Reply = IsTextureReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsTextureReply {
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl TryParse for IsTextureReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = IsTextureReply { sequence, length, ret_val };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for IsTextureReply {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let ret_val_bytes = self.ret_val.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            ret_val_bytes[0],
            ret_val_bytes[1],
            ret_val_bytes[2],
            ret_val_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.ret_val.serialize_into(bytes);
    }
}
#[cfg(test)]
mod is_texture_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for IsTextureReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let ret_val: Bool32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                ret_val,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5628353883641511936);
        let value = IsTextureReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetColorTable request
pub const GET_COLOR_TABLE_REQUEST: u8 = 147;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetColorTableRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
}
impl GetColorTableRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_COLOR_TABLE_REQUEST,
            0,
            0,
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
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_COLOR_TABLE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u32::try_parse(remaining)?;
        let (swap_bytes, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetColorTableRequest {
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
        })
    }
}
impl Request for GetColorTableRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetColorTableRequest {
    type Reply = GetColorTableReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetColorTableReply {
    pub sequence: u16,
    pub width: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetColorTableReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetColorTableReply { sequence, width, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetColorTableReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.width.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        bytes.extend_from_slice(&self.data);
    }
}
impl GetColorTableReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_color_table_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetColorTableReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let width: i32 = GenRandom::generate(rng);
            let data: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                width,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12860743954894290944);
        let value = GetColorTableReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetColorTableParameterfv request
pub const GET_COLOR_TABLE_PARAMETERFV_REQUEST: u8 = 148;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetColorTableParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetColorTableParameterfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_COLOR_TABLE_PARAMETERFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_COLOR_TABLE_PARAMETERFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetColorTableParameterfvRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetColorTableParameterfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetColorTableParameterfvRequest {
    type Reply = GetColorTableParameterfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetColorTableParameterfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetColorTableParameterfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetColorTableParameterfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetColorTableParameterfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetColorTableParameterfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_color_table_parameterfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetColorTableParameterfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12676518370661629952);
        let value = GetColorTableParameterfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetColorTableParameteriv request
pub const GET_COLOR_TABLE_PARAMETERIV_REQUEST: u8 = 149;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetColorTableParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetColorTableParameterivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_COLOR_TABLE_PARAMETERIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_COLOR_TABLE_PARAMETERIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetColorTableParameterivRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetColorTableParameterivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetColorTableParameterivRequest {
    type Reply = GetColorTableParameterivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetColorTableParameterivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetColorTableParameterivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetColorTableParameterivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetColorTableParameterivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetColorTableParameterivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_color_table_parameteriv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetColorTableParameterivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2198331220557824000);
        let value = GetColorTableParameterivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetConvolutionFilter request
pub const GET_CONVOLUTION_FILTER_REQUEST: u8 = 150;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConvolutionFilterRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
}
impl GetConvolutionFilterRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_CONVOLUTION_FILTER_REQUEST,
            0,
            0,
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
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CONVOLUTION_FILTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u32::try_parse(remaining)?;
        let (swap_bytes, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetConvolutionFilterRequest {
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
        })
    }
}
impl Request for GetConvolutionFilterRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetConvolutionFilterRequest {
    type Reply = GetConvolutionFilterReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConvolutionFilterReply {
    pub sequence: u16,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetConvolutionFilterReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetConvolutionFilterReply { sequence, width, height, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetConvolutionFilterReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        bytes.extend_from_slice(&self.data);
    }
}
impl GetConvolutionFilterReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_convolution_filter_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetConvolutionFilterReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let width: i32 = GenRandom::generate(rng);
            let height: i32 = GenRandom::generate(rng);
            let data: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                width,
                height,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13075852885286191104);
        let value = GetConvolutionFilterReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetConvolutionParameterfv request
pub const GET_CONVOLUTION_PARAMETERFV_REQUEST: u8 = 151;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConvolutionParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetConvolutionParameterfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_CONVOLUTION_PARAMETERFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CONVOLUTION_PARAMETERFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetConvolutionParameterfvRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetConvolutionParameterfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetConvolutionParameterfvRequest {
    type Reply = GetConvolutionParameterfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConvolutionParameterfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetConvolutionParameterfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetConvolutionParameterfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetConvolutionParameterfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetConvolutionParameterfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_convolution_parameterfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetConvolutionParameterfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7455216085902557184);
        let value = GetConvolutionParameterfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetConvolutionParameteriv request
pub const GET_CONVOLUTION_PARAMETERIV_REQUEST: u8 = 152;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConvolutionParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetConvolutionParameterivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_CONVOLUTION_PARAMETERIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CONVOLUTION_PARAMETERIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetConvolutionParameterivRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetConvolutionParameterivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetConvolutionParameterivRequest {
    type Reply = GetConvolutionParameterivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConvolutionParameterivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetConvolutionParameterivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetConvolutionParameterivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetConvolutionParameterivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetConvolutionParameterivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_convolution_parameteriv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetConvolutionParameterivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11472346221251657728);
        let value = GetConvolutionParameterivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetSeparableFilter request
pub const GET_SEPARABLE_FILTER_REQUEST: u8 = 153;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSeparableFilterRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
}
impl GetSeparableFilterRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_SEPARABLE_FILTER_REQUEST,
            0,
            0,
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
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SEPARABLE_FILTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u32::try_parse(remaining)?;
        let (swap_bytes, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetSeparableFilterRequest {
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
        })
    }
}
impl Request for GetSeparableFilterRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetSeparableFilterRequest {
    type Reply = GetSeparableFilterReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSeparableFilterReply {
    pub sequence: u16,
    pub row_w: i32,
    pub col_h: i32,
    pub rows_and_cols: Vec<u8>,
}
impl TryParse for GetSeparableFilterReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (row_w, remaining) = i32::try_parse(remaining)?;
        let (col_h, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (rows_and_cols, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let rows_and_cols = rows_and_cols.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSeparableFilterReply { sequence, row_w, col_h, rows_and_cols };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetSeparableFilterReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.rows_and_cols.len() % 4, 0, "`rows_and_cols` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.rows_and_cols.len() / 4).expect("`rows_and_cols` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.row_w.serialize_into(bytes);
        self.col_h.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        bytes.extend_from_slice(&self.rows_and_cols);
    }
}
impl GetSeparableFilterReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `rows_and_cols` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.rows_and_cols.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_separable_filter_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetSeparableFilterReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let row_w: i32 = GenRandom::generate(rng);
            let col_h: i32 = GenRandom::generate(rng);
            let rows_and_cols: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                row_w,
                col_h,
                rows_and_cols,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(1902977498816184320);
        let value = GetSeparableFilterReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetHistogram request
pub const GET_HISTOGRAM_REQUEST: u8 = 154;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetHistogramRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
    pub reset: bool,
}
impl GetHistogramRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let reset_bytes = self.reset.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_HISTOGRAM_REQUEST,
            0,
            0,
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
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_HISTOGRAM_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u32::try_parse(remaining)?;
        let (swap_bytes, remaining) = bool::try_parse(remaining)?;
        let (reset, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetHistogramRequest {
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
            reset,
        })
    }
}
impl Request for GetHistogramRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetHistogramRequest {
    type Reply = GetHistogramReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetHistogramReply {
    pub sequence: u16,
    pub width: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetHistogramReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetHistogramReply { sequence, width, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetHistogramReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.width.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        bytes.extend_from_slice(&self.data);
    }
}
impl GetHistogramReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_histogram_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetHistogramReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let width: i32 = GenRandom::generate(rng);
            let data: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                width,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7209322977286324224);
        let value = GetHistogramReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetHistogramParameterfv request
pub const GET_HISTOGRAM_PARAMETERFV_REQUEST: u8 = 155;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetHistogramParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetHistogramParameterfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_HISTOGRAM_PARAMETERFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_HISTOGRAM_PARAMETERFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetHistogramParameterfvRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetHistogramParameterfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetHistogramParameterfvRequest {
    type Reply = GetHistogramParameterfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetHistogramParameterfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetHistogramParameterfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetHistogramParameterfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetHistogramParameterfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetHistogramParameterfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_histogram_parameterfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetHistogramParameterfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15312439383227891712);
        let value = GetHistogramParameterfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetHistogramParameteriv request
pub const GET_HISTOGRAM_PARAMETERIV_REQUEST: u8 = 156;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetHistogramParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetHistogramParameterivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_HISTOGRAM_PARAMETERIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_HISTOGRAM_PARAMETERIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetHistogramParameterivRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetHistogramParameterivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetHistogramParameterivRequest {
    type Reply = GetHistogramParameterivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetHistogramParameterivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetHistogramParameterivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetHistogramParameterivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetHistogramParameterivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetHistogramParameterivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_histogram_parameteriv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetHistogramParameterivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9252189692013576192);
        let value = GetHistogramParameterivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetMinmax request
pub const GET_MINMAX_REQUEST: u8 = 157;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMinmaxRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
    pub reset: bool,
}
impl GetMinmaxRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let reset_bytes = self.reset.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MINMAX_REQUEST,
            0,
            0,
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
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MINMAX_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u32::try_parse(remaining)?;
        let (swap_bytes, remaining) = bool::try_parse(remaining)?;
        let (reset, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMinmaxRequest {
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
            reset,
        })
    }
}
impl Request for GetMinmaxRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMinmaxRequest {
    type Reply = GetMinmaxReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMinmaxReply {
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl TryParse for GetMinmaxReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMinmaxReply { sequence, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetMinmaxReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
        bytes.extend_from_slice(&self.data);
    }
}
impl GetMinmaxReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_minmax_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetMinmaxReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let data: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4948184665918005248);
        let value = GetMinmaxReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetMinmaxParameterfv request
pub const GET_MINMAX_PARAMETERFV_REQUEST: u8 = 158;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMinmaxParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetMinmaxParameterfvRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MINMAX_PARAMETERFV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MINMAX_PARAMETERFV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMinmaxParameterfvRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetMinmaxParameterfvRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMinmaxParameterfvRequest {
    type Reply = GetMinmaxParameterfvReply;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMinmaxParameterfvReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetMinmaxParameterfvReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMinmaxParameterfvReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetMinmaxParameterfvReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetMinmaxParameterfvReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_minmax_parameterfv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetMinmaxParameterfvReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: Float32 = GenRandom::generate(rng);
            let data: Vec<Float32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(7591657877379481600);
        let value = GetMinmaxParameterfvReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetMinmaxParameteriv request
pub const GET_MINMAX_PARAMETERIV_REQUEST: u8 = 159;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMinmaxParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetMinmaxParameterivRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_MINMAX_PARAMETERIV_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_MINMAX_PARAMETERIV_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetMinmaxParameterivRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetMinmaxParameterivRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetMinmaxParameterivRequest {
    type Reply = GetMinmaxParameterivReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetMinmaxParameterivReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetMinmaxParameterivReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetMinmaxParameterivReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetMinmaxParameterivReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetMinmaxParameterivReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_minmax_parameteriv_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetMinmaxParameterivReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17580865265736876032);
        let value = GetMinmaxParameterivReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetCompressedTexImageARB request
pub const GET_COMPRESSED_TEX_IMAGE_ARB_REQUEST: u8 = 160;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCompressedTexImageARBRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub level: i32,
}
impl GetCompressedTexImageARBRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let level_bytes = self.level.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_COMPRESSED_TEX_IMAGE_ARB_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_COMPRESSED_TEX_IMAGE_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (level, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetCompressedTexImageARBRequest {
            context_tag,
            target,
            level,
        })
    }
}
impl Request for GetCompressedTexImageARBRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetCompressedTexImageARBRequest {
    type Reply = GetCompressedTexImageARBReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCompressedTexImageARBReply {
    pub sequence: u16,
    pub size: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetCompressedTexImageARBReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (size, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetCompressedTexImageARBReply { sequence, size, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetCompressedTexImageARBReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        assert_eq!(self.data.len() % 4, 0, "`data` has an incorrect length, must be a multiple of 4");
        let length = u32::try_from(self.data.len() / 4).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 8]);
        self.size.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        bytes.extend_from_slice(&self.data);
    }
}
impl GetCompressedTexImageARBReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_compressed_tex_image_arb_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetCompressedTexImageARBReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let size: i32 = GenRandom::generate(rng);
            let data: Vec<u8> = gen_random_list(rng, usize::try_from(length.checked_mul(4u32).unwrap()).unwrap());
            Self {
                sequence,
                size,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12441866542432387072);
        let value = GetCompressedTexImageARBReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the DeleteQueriesARB request
pub const DELETE_QUERIES_ARB_REQUEST: u8 = 161;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteQueriesARBRequest<'input> {
    pub context_tag: ContextTag,
    pub ids: Cow<'input, [u32]>,
}
impl<'input> DeleteQueriesARBRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n = i32::try_from(self.ids.len()).expect("`ids` has too many elements");
        let n_bytes = n.serialize();
        let mut request0 = vec![
            major_opcode,
            DELETE_QUERIES_ARB_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            n_bytes[0],
            n_bytes[1],
            n_bytes[2],
            n_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let ids_bytes = self.ids.serialize();
        let length_so_far = length_so_far + ids_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), ids_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DELETE_QUERIES_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (n, remaining) = i32::try_parse(remaining)?;
        let (ids, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_to_usize()?)?;
        let _ = remaining;
        Ok(DeleteQueriesARBRequest {
            context_tag,
            ids: Cow::Owned(ids),
        })
    }
    /// Clone all borrowed data in this DeleteQueriesARBRequest.
    pub fn into_owned(self) -> DeleteQueriesARBRequest<'static> {
        DeleteQueriesARBRequest {
            context_tag: self.context_tag,
            ids: Cow::Owned(self.ids.into_owned()),
        }
    }
}
impl<'input> Request for DeleteQueriesARBRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for DeleteQueriesARBRequest<'input> {
}

/// Opcode for the GenQueriesARB request
pub const GEN_QUERIES_ARB_REQUEST: u8 = 162;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenQueriesARBRequest {
    pub context_tag: ContextTag,
    pub n: i32,
}
impl GenQueriesARBRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n_bytes = self.n.serialize();
        let mut request0 = vec![
            major_opcode,
            GEN_QUERIES_ARB_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            n_bytes[0],
            n_bytes[1],
            n_bytes[2],
            n_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GEN_QUERIES_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (n, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GenQueriesARBRequest {
            context_tag,
            n,
        })
    }
}
impl Request for GenQueriesARBRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GenQueriesARBRequest {
    type Reply = GenQueriesARBReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenQueriesARBReply {
    pub sequence: u16,
    pub data: Vec<u32>,
}
impl TryParse for GenQueriesARBReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GenQueriesARBReply { sequence, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GenQueriesARBReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        let length = u32::try_from(self.data.len()).expect("`data` has too many elements");
        length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 24]);
        self.data.serialize_into(bytes);
    }
}
impl GenQueriesARBReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod gen_queries_arb_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GenQueriesARBReply {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let data: Vec<u32> = gen_random_list(rng, usize::try_from(length).unwrap());
            Self {
                sequence,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3471532714765801472);
        let value = GenQueriesARBReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the IsQueryARB request
pub const IS_QUERY_ARB_REQUEST: u8 = 163;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsQueryARBRequest {
    pub context_tag: ContextTag,
    pub id: u32,
}
impl IsQueryARBRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let id_bytes = self.id.serialize();
        let mut request0 = vec![
            major_opcode,
            IS_QUERY_ARB_REQUEST,
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != IS_QUERY_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (id, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(IsQueryARBRequest {
            context_tag,
            id,
        })
    }
}
impl Request for IsQueryARBRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for IsQueryARBRequest {
    type Reply = IsQueryARBReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsQueryARBReply {
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl TryParse for IsQueryARBReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = IsQueryARBReply { sequence, length, ret_val };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for IsQueryARBReply {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let ret_val_bytes = self.ret_val.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            ret_val_bytes[0],
            ret_val_bytes[1],
            ret_val_bytes[2],
            ret_val_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.ret_val.serialize_into(bytes);
    }
}
#[cfg(test)]
mod is_query_arb_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for IsQueryARBReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let ret_val: Bool32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                ret_val,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13638606248364518400);
        let value = IsQueryARBReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetQueryivARB request
pub const GET_QUERYIV_ARB_REQUEST: u8 = 164;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetQueryivARBRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetQueryivARBRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_QUERYIV_ARB_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_QUERYIV_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (target, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetQueryivARBRequest {
            context_tag,
            target,
            pname,
        })
    }
}
impl Request for GetQueryivARBRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetQueryivARBRequest {
    type Reply = GetQueryivARBReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetQueryivARBReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetQueryivARBReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetQueryivARBReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetQueryivARBReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetQueryivARBReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_queryiv_arb_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetQueryivARBReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2220234867625336832);
        let value = GetQueryivARBReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetQueryObjectivARB request
pub const GET_QUERY_OBJECTIV_ARB_REQUEST: u8 = 165;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetQueryObjectivARBRequest {
    pub context_tag: ContextTag,
    pub id: u32,
    pub pname: u32,
}
impl GetQueryObjectivARBRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let id_bytes = self.id.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_QUERY_OBJECTIV_ARB_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_QUERY_OBJECTIV_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (id, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetQueryObjectivARBRequest {
            context_tag,
            id,
            pname,
        })
    }
}
impl Request for GetQueryObjectivARBRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetQueryObjectivARBRequest {
    type Reply = GetQueryObjectivARBReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetQueryObjectivARBReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetQueryObjectivARBReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetQueryObjectivARBReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetQueryObjectivARBReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetQueryObjectivARBReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_query_objectiv_arb_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetQueryObjectivARBReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: i32 = GenRandom::generate(rng);
            let data: Vec<i32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9769312490701324288);
        let value = GetQueryObjectivARBReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetQueryObjectuivARB request
pub const GET_QUERY_OBJECTUIV_ARB_REQUEST: u8 = 166;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetQueryObjectuivARBRequest {
    pub context_tag: ContextTag,
    pub id: u32,
    pub pname: u32,
}
impl GetQueryObjectuivARBRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let id_bytes = self.id.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_QUERY_OBJECTUIV_ARB_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_QUERY_OBJECTUIV_ARB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_tag, remaining) = ContextTag::try_parse(value)?;
        let (id, remaining) = u32::try_parse(remaining)?;
        let (pname, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetQueryObjectuivARBRequest {
            context_tag,
            id,
            pname,
        })
    }
}
impl Request for GetQueryObjectuivARBRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetQueryObjectuivARBRequest {
    type Reply = GetQueryObjectuivARBReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetQueryObjectuivARBReply {
    pub sequence: u16,
    pub length: u32,
    pub datum: u32,
    pub data: Vec<u32>,
}
impl TryParse for GetQueryObjectuivARBReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetQueryObjectuivARBReply { sequence, length, datum, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetQueryObjectuivARBReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        let n = u32::try_from(self.data.len()).expect("`data` has too many elements");
        n.serialize_into(bytes);
        self.datum.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.data.serialize_into(bytes);
    }
}
impl GetQueryObjectuivARBReply {
    /// Get the value of the `n` field.
    ///
    /// The `n` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn n(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_query_objectuiv_arb_reply {
    #![allow(dead_code, unused_imports, clippy::useless_conversion)]
    use super::*;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetQueryObjectuivARBReply {
        fn generate(rng: &Rng) -> Self {
            let n = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let datum: u32 = GenRandom::generate(rng);
            let data: Vec<u32> = gen_random_list(rng, usize::try_from(n).unwrap());
            Self {
                sequence,
                length,
                datum,
                data,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17758172915772293120);
        let value = GetQueryObjectuivARBReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

