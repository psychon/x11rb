// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Glx` X11 extension.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]

use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
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
impl TryParse for GenericError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl From<&GenericError> for [u8; 32] {
    fn from(input: &GenericError) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let error_code_bytes = input.error_code.serialize();
        let sequence_bytes = input.sequence.serialize();
        let bad_value_bytes = input.bad_value.serialize();
        let minor_opcode_bytes = input.minor_opcode.serialize();
        let major_opcode_bytes = input.major_opcode.serialize();
        [
            response_type_bytes[0],
            error_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            bad_value_bytes[0],
            bad_value_bytes[1],
            bad_value_bytes[2],
            bad_value_bytes[3],
            minor_opcode_bytes[0],
            minor_opcode_bytes[1],
            major_opcode_bytes[0],
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
}
impl From<GenericError> for [u8; 32] {
    fn from(input: GenericError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadContext error
pub const BAD_CONTEXT_ERROR: u8 = 0;
pub type BadContextError = GenericError;

/// Opcode for the BadContextState error
pub const BAD_CONTEXT_STATE_ERROR: u8 = 1;
pub type BadContextStateError = GenericError;

/// Opcode for the BadDrawable error
pub const BAD_DRAWABLE_ERROR: u8 = 2;
pub type BadDrawableError = GenericError;

/// Opcode for the BadPixmap error
pub const BAD_PIXMAP_ERROR: u8 = 3;
pub type BadPixmapError = GenericError;

/// Opcode for the BadContextTag error
pub const BAD_CONTEXT_TAG_ERROR: u8 = 4;
pub type BadContextTagError = GenericError;

/// Opcode for the BadCurrentWindow error
pub const BAD_CURRENT_WINDOW_ERROR: u8 = 5;
pub type BadCurrentWindowError = GenericError;

/// Opcode for the BadRenderRequest error
pub const BAD_RENDER_REQUEST_ERROR: u8 = 6;
pub type BadRenderRequestError = GenericError;

/// Opcode for the BadLargeRequest error
pub const BAD_LARGE_REQUEST_ERROR: u8 = 7;
pub type BadLargeRequestError = GenericError;

/// Opcode for the UnsupportedPrivateRequest error
pub const UNSUPPORTED_PRIVATE_REQUEST_ERROR: u8 = 8;
pub type UnsupportedPrivateRequestError = GenericError;

/// Opcode for the BadFBConfig error
pub const BAD_FB_CONFIG_ERROR: u8 = 9;
pub type BadFBConfigError = GenericError;

/// Opcode for the BadPbuffer error
pub const BAD_PBUFFER_ERROR: u8 = 10;
pub type BadPbufferError = GenericError;

/// Opcode for the BadCurrentDrawable error
pub const BAD_CURRENT_DRAWABLE_ERROR: u8 = 11;
pub type BadCurrentDrawableError = GenericError;

/// Opcode for the BadWindow error
pub const BAD_WINDOW_ERROR: u8 = 12;
pub type BadWindowError = GenericError;

/// Opcode for the GLXBadProfileARB error
pub const GLX_BAD_PROFILE_ARB_ERROR: u8 = 13;
pub type GLXBadProfileARBError = GenericError;

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
impl TryParse for PbufferClobberEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl TryParse for BufferSwapCompleteEvent {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderRequest<'input> {
    pub context_tag: ContextTag,
    pub data: &'input [u8],
}
impl<'input> RenderRequest<'input> {
    /// Opcode for the Render request
    pub const fn opcode() -> u8 { 1 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            RenderRequest::opcode(),
            0,
            0,
            context_tag_bytes[0],
            context_tag_bytes[1],
            context_tag_bytes[2],
            context_tag_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.data[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.data[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn render<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RenderRequest {
        context_tag: context_tag,
        data: data,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderLargeRequest<'input> {
    pub context_tag: ContextTag,
    pub request_num: u16,
    pub request_total: u16,
    pub data: &'input [u8],
}
impl<'input> RenderLargeRequest<'input> {
    /// Opcode for the RenderLarge request
    pub const fn opcode() -> u8 { 2 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let request_num_bytes = self.request_num.serialize();
        let request_total_bytes = self.request_total.serialize();
        let data_len = u32::try_from(self.data.len()).expect("`data` has too many elements");
        let data_len_bytes = data_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            RenderLargeRequest::opcode(),
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
        let length_so_far = length_so_far + (&self.data[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.data[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn render_large<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, request_num: u16, request_total: u16, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RenderLargeRequest {
        context_tag: context_tag,
        request_num: request_num,
        request_total: request_total,
        data: data,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateContextRequest {
    pub context: Context,
    pub visual: xproto::Visualid,
    pub screen: u32,
    pub share_list: Context,
    pub is_direct: bool,
}
impl CreateContextRequest {
    /// Opcode for the CreateContext request
    pub const fn opcode() -> u8 { 3 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let visual_bytes = self.visual.serialize();
        let screen_bytes = self.screen.serialize();
        let share_list_bytes = self.share_list.serialize();
        let is_direct_bytes = self.is_direct.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateContextRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_context<Conn>(conn: &Conn, context: Context, visual: xproto::Visualid, screen: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        context: context,
        visual: visual,
        screen: screen,
        share_list: share_list,
        is_direct: is_direct,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyContextRequest {
    pub context: Context,
}
impl DestroyContextRequest {
    /// Opcode for the DestroyContext request
    pub const fn opcode() -> u8 { 4 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DestroyContextRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn destroy_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyContextRequest {
        context: context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeCurrentRequest {
    pub drawable: Drawable,
    pub context: Context,
    pub old_context_tag: ContextTag,
}
impl MakeCurrentRequest {
    /// Opcode for the MakeCurrent request
    pub const fn opcode() -> u8 { 5 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let context_bytes = self.context.serialize();
        let old_context_tag_bytes = self.old_context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            MakeCurrentRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn make_current<Conn>(conn: &Conn, drawable: Drawable, context: Context, old_context_tag: ContextTag) -> Result<Cookie<'_, Conn, MakeCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = MakeCurrentRequest {
        drawable: drawable,
        context: context,
        old_context_tag: old_context_tag,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeCurrentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl TryParse for MakeCurrentReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsDirectRequest {
    pub context: Context,
}
impl IsDirectRequest {
    /// Opcode for the IsDirect request
    pub const fn opcode() -> u8 { 6 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            IsDirectRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn is_direct<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, IsDirectReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsDirectRequest {
        context: context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsDirectReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_direct: bool,
}
impl TryParse for IsDirectReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub major_version: u32,
    pub minor_version: u32,
}
impl QueryVersionRequest {
    /// Opcode for the QueryVersion request
    pub const fn opcode() -> u8 { 7 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QueryVersionRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major_version: major_version,
        minor_version: minor_version,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}
impl TryParse for QueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitGLRequest {
    pub context_tag: ContextTag,
}
impl WaitGLRequest {
    /// Opcode for the WaitGL request
    pub const fn opcode() -> u8 { 8 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            WaitGLRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn wait_gl<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = WaitGLRequest {
        context_tag: context_tag,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitXRequest {
    pub context_tag: ContextTag,
}
impl WaitXRequest {
    /// Opcode for the WaitX request
    pub const fn opcode() -> u8 { 9 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            WaitXRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn wait_x<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = WaitXRequest {
        context_tag: context_tag,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyContextRequest {
    pub src: Context,
    pub dest: Context,
    pub mask: u32,
    pub src_context_tag: ContextTag,
}
impl CopyContextRequest {
    /// Opcode for the CopyContext request
    pub const fn opcode() -> u8 { 10 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let src_bytes = self.src.serialize();
        let dest_bytes = self.dest.serialize();
        let mask_bytes = self.mask.serialize();
        let src_context_tag_bytes = self.src_context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CopyContextRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn copy_context<Conn>(conn: &Conn, src: Context, dest: Context, mask: u32, src_context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CopyContextRequest {
        src: src,
        dest: dest,
        mask: mask,
        src_context_tag: src_context_tag,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
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
            GC::GL_ALL_ATTRIB_BITS => 16_777_215,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwapBuffersRequest {
    pub context_tag: ContextTag,
    pub drawable: Drawable,
}
impl SwapBuffersRequest {
    /// Opcode for the SwapBuffers request
    pub const fn opcode() -> u8 { 11 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SwapBuffersRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn swap_buffers<Conn>(conn: &Conn, context_tag: ContextTag, drawable: Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SwapBuffersRequest {
        context_tag: context_tag,
        drawable: drawable,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseXFontRequest {
    pub context_tag: ContextTag,
    pub font: xproto::Font,
    pub first: u32,
    pub count: u32,
    pub list_base: u32,
}
impl UseXFontRequest {
    /// Opcode for the UseXFont request
    pub const fn opcode() -> u8 { 12 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let font_bytes = self.font.serialize();
        let first_bytes = self.first.serialize();
        let count_bytes = self.count.serialize();
        let list_base_bytes = self.list_base.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            UseXFontRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn use_x_font<Conn>(conn: &Conn, context_tag: ContextTag, font: xproto::Font, first: u32, count: u32, list_base: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UseXFontRequest {
        context_tag: context_tag,
        font: font,
        first: first,
        count: count,
        list_base: list_base,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateGLXPixmapRequest {
    pub screen: u32,
    pub visual: xproto::Visualid,
    pub pixmap: xproto::Pixmap,
    pub glx_pixmap: Pixmap,
}
impl CreateGLXPixmapRequest {
    /// Opcode for the CreateGLXPixmap request
    pub const fn opcode() -> u8 { 13 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let visual_bytes = self.visual.serialize();
        let pixmap_bytes = self.pixmap.serialize();
        let glx_pixmap_bytes = self.glx_pixmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateGLXPixmapRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_glx_pixmap<Conn>(conn: &Conn, screen: u32, visual: xproto::Visualid, pixmap: xproto::Pixmap, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateGLXPixmapRequest {
        screen: screen,
        visual: visual,
        pixmap: pixmap,
        glx_pixmap: glx_pixmap,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVisualConfigsRequest {
    pub screen: u32,
}
impl GetVisualConfigsRequest {
    /// Opcode for the GetVisualConfigs request
    pub const fn opcode() -> u8 { 14 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetVisualConfigsRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_visual_configs<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetVisualConfigsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetVisualConfigsRequest {
        screen: screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetVisualConfigsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub num_visuals: u32,
    pub num_properties: u32,
    pub property_list: Vec<u32>,
}
impl TryParse for GetVisualConfigsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_visuals, remaining) = u32::try_parse(remaining)?;
        let (num_properties, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (property_list, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyGLXPixmapRequest {
    pub glx_pixmap: Pixmap,
}
impl DestroyGLXPixmapRequest {
    /// Opcode for the DestroyGLXPixmap request
    pub const fn opcode() -> u8 { 15 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let glx_pixmap_bytes = self.glx_pixmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DestroyGLXPixmapRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn destroy_glx_pixmap<Conn>(conn: &Conn, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyGLXPixmapRequest {
        glx_pixmap: glx_pixmap,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorPrivateRequest<'input> {
    pub vendor_code: u32,
    pub context_tag: ContextTag,
    pub data: &'input [u8],
}
impl<'input> VendorPrivateRequest<'input> {
    /// Opcode for the VendorPrivate request
    pub const fn opcode() -> u8 { 16 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let vendor_code_bytes = self.vendor_code.serialize();
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            VendorPrivateRequest::opcode(),
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
        let length_so_far = length_so_far + (&self.data[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.data[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn vendor_private<'c, 'input, Conn>(conn: &'c Conn, vendor_code: u32, context_tag: ContextTag, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = VendorPrivateRequest {
        vendor_code: vendor_code,
        context_tag: context_tag,
        data: data,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorPrivateWithReplyRequest<'input> {
    pub vendor_code: u32,
    pub context_tag: ContextTag,
    pub data: &'input [u8],
}
impl<'input> VendorPrivateWithReplyRequest<'input> {
    /// Opcode for the VendorPrivateWithReply request
    pub const fn opcode() -> u8 { 17 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let vendor_code_bytes = self.vendor_code.serialize();
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            VendorPrivateWithReplyRequest::opcode(),
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
        let length_so_far = length_so_far + (&self.data[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.data[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn vendor_private_with_reply<'c, 'input, Conn>(conn: &'c Conn, vendor_code: u32, context_tag: ContextTag, data: &'input [u8]) -> Result<Cookie<'c, Conn, VendorPrivateWithReplyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = VendorPrivateWithReplyRequest {
        vendor_code: vendor_code,
        context_tag: context_tag,
        data: data,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorPrivateWithReplyReply {
    pub response_type: u8,
    pub sequence: u16,
    pub retval: u32,
    pub data1: [u8; 24],
    pub data2: Vec<u8>,
}
impl TryParse for VendorPrivateWithReplyReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (retval, remaining) = u32::try_parse(remaining)?;
        let (data1, remaining) = crate::x11_utils::parse_u8_list(remaining, 24)?;
        let data1 = <[u8; 24]>::try_from(data1).unwrap();
        let (data2, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data2 = data2.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtensionsStringRequest {
    pub screen: u32,
}
impl QueryExtensionsStringRequest {
    /// Opcode for the QueryExtensionsString request
    pub const fn opcode() -> u8 { 18 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QueryExtensionsStringRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn query_extensions_string<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, QueryExtensionsStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryExtensionsStringRequest {
        screen: screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtensionsStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub n: u32,
}
impl TryParse for QueryExtensionsStringReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryServerStringRequest {
    pub screen: u32,
    pub name: u32,
}
impl QueryServerStringRequest {
    /// Opcode for the QueryServerString request
    pub const fn opcode() -> u8 { 19 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let name_bytes = self.name.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QueryServerStringRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn query_server_string<Conn>(conn: &Conn, screen: u32, name: u32) -> Result<Cookie<'_, Conn, QueryServerStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryServerStringRequest {
        screen: screen,
        name: name,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryServerStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub string: Vec<u8>,
}
impl TryParse for QueryServerStringReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (str_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, str_len.try_into().or(Err(ParseError::ParseError))?)?;
        let string = string.to_vec();
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientInfoRequest<'input> {
    pub major_version: u32,
    pub minor_version: u32,
    pub string: &'input [u8],
}
impl<'input> ClientInfoRequest<'input> {
    /// Opcode for the ClientInfo request
    pub const fn opcode() -> u8 { 20 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let str_len = u32::try_from(self.string.len()).expect("`string` has too many elements");
        let str_len_bytes = str_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ClientInfoRequest::opcode(),
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
        let length_so_far = length_so_far + (&self.string[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.string[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn client_info<'c, 'input, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, string: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ClientInfoRequest {
        major_version: major_version,
        minor_version: minor_version,
        string: string,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFBConfigsRequest {
    pub screen: u32,
}
impl GetFBConfigsRequest {
    /// Opcode for the GetFBConfigs request
    pub const fn opcode() -> u8 { 21 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetFBConfigsRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_fb_configs<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetFBConfigsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetFBConfigsRequest {
        screen: screen,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFBConfigsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub num_fb_configs: u32,
    pub num_properties: u32,
    pub property_list: Vec<u32>,
}
impl TryParse for GetFBConfigsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_fb_configs, remaining) = u32::try_parse(remaining)?;
        let (num_properties, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (property_list, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePixmapRequest<'input> {
    pub screen: u32,
    pub fbconfig: Fbconfig,
    pub pixmap: xproto::Pixmap,
    pub glx_pixmap: Pixmap,
    pub attribs: &'input [u32],
}
impl<'input> CreatePixmapRequest<'input> {
    /// Opcode for the CreatePixmap request
    pub const fn opcode() -> u8 { 22 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let pixmap_bytes = self.pixmap.serialize();
        let glx_pixmap_bytes = self.glx_pixmap.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreatePixmapRequest::opcode(),
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
        Ok((vec![request0.into(), attribs_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn create_pixmap<'c, 'input, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, pixmap: xproto::Pixmap, glx_pixmap: Pixmap, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreatePixmapRequest {
        screen: screen,
        fbconfig: fbconfig,
        pixmap: pixmap,
        glx_pixmap: glx_pixmap,
        attribs: attribs,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyPixmapRequest {
    pub glx_pixmap: Pixmap,
}
impl DestroyPixmapRequest {
    /// Opcode for the DestroyPixmap request
    pub const fn opcode() -> u8 { 23 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let glx_pixmap_bytes = self.glx_pixmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DestroyPixmapRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn destroy_pixmap<Conn>(conn: &Conn, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyPixmapRequest {
        glx_pixmap: glx_pixmap,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateNewContextRequest {
    pub context: Context,
    pub fbconfig: Fbconfig,
    pub screen: u32,
    pub render_type: u32,
    pub share_list: Context,
    pub is_direct: bool,
}
impl CreateNewContextRequest {
    /// Opcode for the CreateNewContext request
    pub const fn opcode() -> u8 { 24 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let screen_bytes = self.screen.serialize();
        let render_type_bytes = self.render_type.serialize();
        let share_list_bytes = self.share_list.serialize();
        let is_direct_bytes = self.is_direct.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateNewContextRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn create_new_context<Conn>(conn: &Conn, context: Context, fbconfig: Fbconfig, screen: u32, render_type: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateNewContextRequest {
        context: context,
        fbconfig: fbconfig,
        screen: screen,
        render_type: render_type,
        share_list: share_list,
        is_direct: is_direct,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryContextRequest {
    pub context: Context,
}
impl QueryContextRequest {
    /// Opcode for the QueryContext request
    pub const fn opcode() -> u8 { 25 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QueryContextRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn query_context<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, QueryContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryContextRequest {
        context: context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attribs: Vec<u32>,
}
impl TryParse for QueryContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeContextCurrentRequest {
    pub old_context_tag: ContextTag,
    pub drawable: Drawable,
    pub read_drawable: Drawable,
    pub context: Context,
}
impl MakeContextCurrentRequest {
    /// Opcode for the MakeContextCurrent request
    pub const fn opcode() -> u8 { 26 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let old_context_tag_bytes = self.old_context_tag.serialize();
        let drawable_bytes = self.drawable.serialize();
        let read_drawable_bytes = self.read_drawable.serialize();
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            MakeContextCurrentRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn make_context_current<Conn>(conn: &Conn, old_context_tag: ContextTag, drawable: Drawable, read_drawable: Drawable, context: Context) -> Result<Cookie<'_, Conn, MakeContextCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = MakeContextCurrentRequest {
        old_context_tag: old_context_tag,
        drawable: drawable,
        read_drawable: read_drawable,
        context: context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeContextCurrentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl TryParse for MakeContextCurrentReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePbufferRequest<'input> {
    pub screen: u32,
    pub fbconfig: Fbconfig,
    pub pbuffer: Pbuffer,
    pub attribs: &'input [u32],
}
impl<'input> CreatePbufferRequest<'input> {
    /// Opcode for the CreatePbuffer request
    pub const fn opcode() -> u8 { 27 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let pbuffer_bytes = self.pbuffer.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreatePbufferRequest::opcode(),
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
        Ok((vec![request0.into(), attribs_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn create_pbuffer<'c, 'input, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, pbuffer: Pbuffer, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreatePbufferRequest {
        screen: screen,
        fbconfig: fbconfig,
        pbuffer: pbuffer,
        attribs: attribs,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyPbufferRequest {
    pub pbuffer: Pbuffer,
}
impl DestroyPbufferRequest {
    /// Opcode for the DestroyPbuffer request
    pub const fn opcode() -> u8 { 28 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pbuffer_bytes = self.pbuffer.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DestroyPbufferRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn destroy_pbuffer<Conn>(conn: &Conn, pbuffer: Pbuffer) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyPbufferRequest {
        pbuffer: pbuffer,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDrawableAttributesRequest {
    pub drawable: Drawable,
}
impl GetDrawableAttributesRequest {
    /// Opcode for the GetDrawableAttributes request
    pub const fn opcode() -> u8 { 29 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetDrawableAttributesRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_drawable_attributes<Conn>(conn: &Conn, drawable: Drawable) -> Result<Cookie<'_, Conn, GetDrawableAttributesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDrawableAttributesRequest {
        drawable: drawable,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDrawableAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attribs: Vec<u32>,
}
impl TryParse for GetDrawableAttributesReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_attribs, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (attribs, remaining) = crate::x11_utils::parse_list::<u32>(remaining, num_attribs.checked_mul(2u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeDrawableAttributesRequest<'input> {
    pub drawable: Drawable,
    pub attribs: &'input [u32],
}
impl<'input> ChangeDrawableAttributesRequest<'input> {
    /// Opcode for the ChangeDrawableAttributes request
    pub const fn opcode() -> u8 { 30 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ChangeDrawableAttributesRequest::opcode(),
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
        Ok((vec![request0.into(), attribs_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn change_drawable_attributes<'c, 'input, Conn>(conn: &'c Conn, drawable: Drawable, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeDrawableAttributesRequest {
        drawable: drawable,
        attribs: attribs,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateWindowRequest<'input> {
    pub screen: u32,
    pub fbconfig: Fbconfig,
    pub window: xproto::Window,
    pub glx_window: Window,
    pub attribs: &'input [u32],
}
impl<'input> CreateWindowRequest<'input> {
    /// Opcode for the CreateWindow request
    pub const fn opcode() -> u8 { 31 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let screen_bytes = self.screen.serialize();
        let fbconfig_bytes = self.fbconfig.serialize();
        let window_bytes = self.window.serialize();
        let glx_window_bytes = self.glx_window.serialize();
        assert_eq!(self.attribs.len() % 2, 0, "`attribs` has an incorrect length, must be a multiple of 2");
        let num_attribs = u32::try_from(self.attribs.len() / 2).expect("`attribs` has too many elements");
        let num_attribs_bytes = num_attribs.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CreateWindowRequest::opcode(),
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
        Ok((vec![request0.into(), attribs_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn create_window<'c, 'input, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, window: xproto::Window, glx_window: Window, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateWindowRequest {
        screen: screen,
        fbconfig: fbconfig,
        window: window,
        glx_window: glx_window,
        attribs: attribs,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteWindowRequest {
    pub glxwindow: Window,
}
impl DeleteWindowRequest {
    /// Opcode for the DeleteWindow request
    pub const fn opcode() -> u8 { 32 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let glxwindow_bytes = self.glxwindow.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DeleteWindowRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn delete_window<Conn>(conn: &Conn, glxwindow: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteWindowRequest {
        glxwindow: glxwindow,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientInfoARBRequest<'input> {
    pub major_version: u32,
    pub minor_version: u32,
    pub gl_versions: &'input [u32],
    pub gl_extension_string: &'input [u8],
    pub glx_extension_string: &'input [u8],
}
impl<'input> SetClientInfoARBRequest<'input> {
    /// Opcode for the SetClientInfoARB request
    pub const fn opcode() -> u8 { 33 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
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
            extension_information.major_opcode,
            SetClientInfoARBRequest::opcode(),
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
        let length_so_far = length_so_far + (&self.gl_extension_string[..]).len();
        let length_so_far = length_so_far + (&self.glx_extension_string[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), gl_versions_bytes.into(), (&self.gl_extension_string[..]).into(), (&self.glx_extension_string[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_client_info_arb<'c, 'input, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, gl_versions: &'input [u32], gl_extension_string: &'input [u8], glx_extension_string: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetClientInfoARBRequest {
        major_version: major_version,
        minor_version: minor_version,
        gl_versions: gl_versions,
        gl_extension_string: gl_extension_string,
        glx_extension_string: glx_extension_string,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateContextAttribsARBRequest<'input> {
    pub context: Context,
    pub fbconfig: Fbconfig,
    pub screen: u32,
    pub share_list: Context,
    pub is_direct: bool,
    pub attribs: &'input [u32],
}
impl<'input> CreateContextAttribsARBRequest<'input> {
    /// Opcode for the CreateContextAttribsARB request
    pub const fn opcode() -> u8 { 34 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
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
            extension_information.major_opcode,
            CreateContextAttribsARBRequest::opcode(),
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
        Ok((vec![request0.into(), attribs_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn create_context_attribs_arb<'c, 'input, Conn>(conn: &'c Conn, context: Context, fbconfig: Fbconfig, screen: u32, share_list: Context, is_direct: bool, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextAttribsARBRequest {
        context: context,
        fbconfig: fbconfig,
        screen: screen,
        share_list: share_list,
        is_direct: is_direct,
        attribs: attribs,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClientInfo2ARBRequest<'input> {
    pub major_version: u32,
    pub minor_version: u32,
    pub gl_versions: &'input [u32],
    pub gl_extension_string: &'input [u8],
    pub glx_extension_string: &'input [u8],
}
impl<'input> SetClientInfo2ARBRequest<'input> {
    /// Opcode for the SetClientInfo2ARB request
    pub const fn opcode() -> u8 { 35 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
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
            extension_information.major_opcode,
            SetClientInfo2ARBRequest::opcode(),
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
        let length_so_far = length_so_far + (&self.gl_extension_string[..]).len();
        let length_so_far = length_so_far + (&self.glx_extension_string[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), gl_versions_bytes.into(), (&self.gl_extension_string[..]).into(), (&self.glx_extension_string[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_client_info2_arb<'c, 'input, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, gl_versions: &'input [u32], gl_extension_string: &'input [u8], glx_extension_string: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetClientInfo2ARBRequest {
        major_version: major_version,
        minor_version: minor_version,
        gl_versions: gl_versions,
        gl_extension_string: gl_extension_string,
        glx_extension_string: glx_extension_string,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewListRequest {
    pub context_tag: ContextTag,
    pub list: u32,
    pub mode: u32,
}
impl NewListRequest {
    /// Opcode for the NewList request
    pub const fn opcode() -> u8 { 101 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let list_bytes = self.list.serialize();
        let mode_bytes = self.mode.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            NewListRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn new_list<Conn>(conn: &Conn, context_tag: ContextTag, list: u32, mode: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = NewListRequest {
        context_tag: context_tag,
        list: list,
        mode: mode,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndListRequest {
    pub context_tag: ContextTag,
}
impl EndListRequest {
    /// Opcode for the EndList request
    pub const fn opcode() -> u8 { 102 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            EndListRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn end_list<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = EndListRequest {
        context_tag: context_tag,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteListsRequest {
    pub context_tag: ContextTag,
    pub list: u32,
    pub range: i32,
}
impl DeleteListsRequest {
    /// Opcode for the DeleteLists request
    pub const fn opcode() -> u8 { 103 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let list_bytes = self.list.serialize();
        let range_bytes = self.range.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DeleteListsRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn delete_lists<Conn>(conn: &Conn, context_tag: ContextTag, list: u32, range: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteListsRequest {
        context_tag: context_tag,
        list: list,
        range: range,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenListsRequest {
    pub context_tag: ContextTag,
    pub range: i32,
}
impl GenListsRequest {
    /// Opcode for the GenLists request
    pub const fn opcode() -> u8 { 104 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let range_bytes = self.range.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GenListsRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn gen_lists<Conn>(conn: &Conn, context_tag: ContextTag, range: i32) -> Result<Cookie<'_, Conn, GenListsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GenListsRequest {
        context_tag: context_tag,
        range: range,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenListsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
}
impl TryParse for GenListsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackBufferRequest {
    pub context_tag: ContextTag,
    pub size: i32,
    pub type_: i32,
}
impl FeedbackBufferRequest {
    /// Opcode for the FeedbackBuffer request
    pub const fn opcode() -> u8 { 105 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let size_bytes = self.size.serialize();
        let type_bytes = self.type_.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            FeedbackBufferRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn feedback_buffer<Conn>(conn: &Conn, context_tag: ContextTag, size: i32, type_: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FeedbackBufferRequest {
        context_tag: context_tag,
        size: size,
        type_: type_,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectBufferRequest {
    pub context_tag: ContextTag,
    pub size: i32,
}
impl SelectBufferRequest {
    /// Opcode for the SelectBuffer request
    pub const fn opcode() -> u8 { 106 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let size_bytes = self.size.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SelectBufferRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn select_buffer<Conn>(conn: &Conn, context_tag: ContextTag, size: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectBufferRequest {
        context_tag: context_tag,
        size: size,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderModeRequest {
    pub context_tag: ContextTag,
    pub mode: u32,
}
impl RenderModeRequest {
    /// Opcode for the RenderMode request
    pub const fn opcode() -> u8 { 107 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mode_bytes = self.mode.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            RenderModeRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn render_mode<Conn>(conn: &Conn, context_tag: ContextTag, mode: u32) -> Result<Cookie<'_, Conn, RenderModeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RenderModeRequest {
        context_tag: context_tag,
        mode: mode,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
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
impl TryParse for RenderModeReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = u32::try_parse(remaining)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (new_mode, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinishRequest {
    pub context_tag: ContextTag,
}
impl FinishRequest {
    /// Opcode for the Finish request
    pub const fn opcode() -> u8 { 108 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            FinishRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn finish<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<Cookie<'_, Conn, FinishReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FinishRequest {
        context_tag: context_tag,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinishReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
}
impl TryParse for FinishReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelStorefRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
    pub datum: Float32,
}
impl PixelStorefRequest {
    /// Opcode for the PixelStoref request
    pub const fn opcode() -> u8 { 109 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let datum_bytes = self.datum.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            PixelStorefRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn pixel_storef<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32, datum: Float32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PixelStorefRequest {
        context_tag: context_tag,
        pname: pname,
        datum: datum,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelStoreiRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
    pub datum: i32,
}
impl PixelStoreiRequest {
    /// Opcode for the PixelStorei request
    pub const fn opcode() -> u8 { 110 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let datum_bytes = self.datum.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            PixelStoreiRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn pixel_storei<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32, datum: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PixelStoreiRequest {
        context_tag: context_tag,
        pname: pname,
        datum: datum,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Opcode for the ReadPixels request
    pub const fn opcode() -> u8 { 111 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
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
            extension_information.major_opcode,
            ReadPixelsRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn read_pixels<Conn>(conn: &Conn, context_tag: ContextTag, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, swap_bytes: bool, lsb_first: bool) -> Result<Cookie<'_, Conn, ReadPixelsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ReadPixelsRequest {
        context_tag: context_tag,
        x: x,
        y: y,
        width: width,
        height: height,
        format: format,
        type_: type_,
        swap_bytes: swap_bytes,
        lsb_first: lsb_first,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadPixelsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl TryParse for ReadPixelsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetBooleanvRequest {
    pub context_tag: ContextTag,
    pub pname: i32,
}
impl GetBooleanvRequest {
    /// Opcode for the GetBooleanv request
    pub const fn opcode() -> u8 { 112 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetBooleanvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_booleanv<Conn>(conn: &Conn, context_tag: ContextTag, pname: i32) -> Result<Cookie<'_, Conn, GetBooleanvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetBooleanvRequest {
        context_tag: context_tag,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBooleanvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: bool,
    pub data: Vec<bool>,
}
impl TryParse for GetBooleanvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<bool>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetClipPlaneRequest {
    pub context_tag: ContextTag,
    pub plane: i32,
}
impl GetClipPlaneRequest {
    /// Opcode for the GetClipPlane request
    pub const fn opcode() -> u8 { 113 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let plane_bytes = self.plane.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetClipPlaneRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_clip_plane<Conn>(conn: &Conn, context_tag: ContextTag, plane: i32) -> Result<Cookie<'_, Conn, GetClipPlaneReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClipPlaneRequest {
        context_tag: context_tag,
        plane: plane,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetClipPlaneReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<Float64>,
}
impl TryParse for GetClipPlaneReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, length.checked_div(2u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDoublevRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
}
impl GetDoublevRequest {
    /// Opcode for the GetDoublev request
    pub const fn opcode() -> u8 { 114 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetDoublevRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_doublev<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetDoublevReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDoublevRequest {
        context_tag: context_tag,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetDoublevReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl TryParse for GetDoublevReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetErrorRequest {
    pub context_tag: ContextTag,
}
impl GetErrorRequest {
    /// Opcode for the GetError request
    pub const fn opcode() -> u8 { 115 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetErrorRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_error<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<Cookie<'_, Conn, GetErrorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetErrorRequest {
        context_tag: context_tag,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetErrorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub error: i32,
}
impl TryParse for GetErrorReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFloatvRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
}
impl GetFloatvRequest {
    /// Opcode for the GetFloatv request
    pub const fn opcode() -> u8 { 116 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetFloatvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_floatv<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetFloatvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetFloatvRequest {
        context_tag: context_tag,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetFloatvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetFloatvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetIntegervRequest {
    pub context_tag: ContextTag,
    pub pname: u32,
}
impl GetIntegervRequest {
    /// Opcode for the GetIntegerv request
    pub const fn opcode() -> u8 { 117 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetIntegervRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_integerv<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetIntegervReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetIntegervRequest {
        context_tag: context_tag,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetIntegervReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetIntegervReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLightfvRequest {
    pub context_tag: ContextTag,
    pub light: u32,
    pub pname: u32,
}
impl GetLightfvRequest {
    /// Opcode for the GetLightfv request
    pub const fn opcode() -> u8 { 118 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let light_bytes = self.light.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetLightfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_lightfv<Conn>(conn: &Conn, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Conn, GetLightfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetLightfvRequest {
        context_tag: context_tag,
        light: light,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetLightfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetLightfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLightivRequest {
    pub context_tag: ContextTag,
    pub light: u32,
    pub pname: u32,
}
impl GetLightivRequest {
    /// Opcode for the GetLightiv request
    pub const fn opcode() -> u8 { 119 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let light_bytes = self.light.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetLightivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_lightiv<Conn>(conn: &Conn, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Conn, GetLightivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetLightivRequest {
        context_tag: context_tag,
        light: light,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetLightivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetLightivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMapdvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub query: u32,
}
impl GetMapdvRequest {
    /// Opcode for the GetMapdv request
    pub const fn opcode() -> u8 { 120 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let query_bytes = self.query.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetMapdvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_mapdv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapdvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMapdvRequest {
        context_tag: context_tag,
        target: target,
        query: query,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetMapdvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl TryParse for GetMapdvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMapfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub query: u32,
}
impl GetMapfvRequest {
    /// Opcode for the GetMapfv request
    pub const fn opcode() -> u8 { 121 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let query_bytes = self.query.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetMapfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_mapfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMapfvRequest {
        context_tag: context_tag,
        target: target,
        query: query,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetMapfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetMapfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMapivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub query: u32,
}
impl GetMapivRequest {
    /// Opcode for the GetMapiv request
    pub const fn opcode() -> u8 { 122 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let query_bytes = self.query.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetMapivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_mapiv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMapivRequest {
        context_tag: context_tag,
        target: target,
        query: query,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMapivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetMapivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMaterialfvRequest {
    pub context_tag: ContextTag,
    pub face: u32,
    pub pname: u32,
}
impl GetMaterialfvRequest {
    /// Opcode for the GetMaterialfv request
    pub const fn opcode() -> u8 { 123 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let face_bytes = self.face.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetMaterialfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_materialfv<Conn>(conn: &Conn, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMaterialfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMaterialfvRequest {
        context_tag: context_tag,
        face: face,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetMaterialfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetMaterialfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMaterialivRequest {
    pub context_tag: ContextTag,
    pub face: u32,
    pub pname: u32,
}
impl GetMaterialivRequest {
    /// Opcode for the GetMaterialiv request
    pub const fn opcode() -> u8 { 124 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let face_bytes = self.face.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetMaterialivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_materialiv<Conn>(conn: &Conn, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMaterialivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMaterialivRequest {
        context_tag: context_tag,
        face: face,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMaterialivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetMaterialivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPixelMapfvRequest {
    pub context_tag: ContextTag,
    pub map: u32,
}
impl GetPixelMapfvRequest {
    /// Opcode for the GetPixelMapfv request
    pub const fn opcode() -> u8 { 125 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let map_bytes = self.map.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetPixelMapfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_pixel_mapfv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPixelMapfvRequest {
        context_tag: context_tag,
        map: map,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetPixelMapfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetPixelMapfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPixelMapuivRequest {
    pub context_tag: ContextTag,
    pub map: u32,
}
impl GetPixelMapuivRequest {
    /// Opcode for the GetPixelMapuiv request
    pub const fn opcode() -> u8 { 126 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let map_bytes = self.map.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetPixelMapuivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_pixel_mapuiv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapuivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPixelMapuivRequest {
        context_tag: context_tag,
        map: map,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPixelMapuivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u32,
    pub data: Vec<u32>,
}
impl TryParse for GetPixelMapuivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPixelMapusvRequest {
    pub context_tag: ContextTag,
    pub map: u32,
}
impl GetPixelMapusvRequest {
    /// Opcode for the GetPixelMapusv request
    pub const fn opcode() -> u8 { 127 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let map_bytes = self.map.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetPixelMapusvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_pixel_mapusv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapusvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPixelMapusvRequest {
        context_tag: context_tag,
        map: map,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPixelMapusvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u16,
    pub data: Vec<u16>,
}
impl TryParse for GetPixelMapusvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u16>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPolygonStippleRequest {
    pub context_tag: ContextTag,
    pub lsb_first: bool,
}
impl GetPolygonStippleRequest {
    /// Opcode for the GetPolygonStipple request
    pub const fn opcode() -> u8 { 128 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let lsb_first_bytes = self.lsb_first.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetPolygonStippleRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_polygon_stipple<Conn>(conn: &Conn, context_tag: ContextTag, lsb_first: bool) -> Result<Cookie<'_, Conn, GetPolygonStippleReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPolygonStippleRequest {
        context_tag: context_tag,
        lsb_first: lsb_first,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPolygonStippleReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl TryParse for GetPolygonStippleReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStringRequest {
    pub context_tag: ContextTag,
    pub name: u32,
}
impl GetStringRequest {
    /// Opcode for the GetString request
    pub const fn opcode() -> u8 { 129 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let name_bytes = self.name.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetStringRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_string<Conn>(conn: &Conn, context_tag: ContextTag, name: u32) -> Result<Cookie<'_, Conn, GetStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetStringRequest {
        context_tag: context_tag,
        name: name,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStringReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub string: Vec<u8>,
}
impl TryParse for GetStringReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (string, remaining) = crate::x11_utils::parse_u8_list(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
        let string = string.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexEnvfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetTexEnvfvRequest {
    /// Opcode for the GetTexEnvfv request
    pub const fn opcode() -> u8 { 130 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexEnvfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_envfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexEnvfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexEnvfvRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTexEnvfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetTexEnvfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexEnvivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetTexEnvivRequest {
    /// Opcode for the GetTexEnviv request
    pub const fn opcode() -> u8 { 131 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexEnvivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_enviv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexEnvivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexEnvivRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexEnvivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetTexEnvivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexGendvRequest {
    pub context_tag: ContextTag,
    pub coord: u32,
    pub pname: u32,
}
impl GetTexGendvRequest {
    /// Opcode for the GetTexGendv request
    pub const fn opcode() -> u8 { 132 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let coord_bytes = self.coord.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexGendvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_gendv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGendvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexGendvRequest {
        context_tag: context_tag,
        coord: coord,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTexGendvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl TryParse for GetTexGendvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float64::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float64>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexGenfvRequest {
    pub context_tag: ContextTag,
    pub coord: u32,
    pub pname: u32,
}
impl GetTexGenfvRequest {
    /// Opcode for the GetTexGenfv request
    pub const fn opcode() -> u8 { 133 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let coord_bytes = self.coord.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexGenfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_genfv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGenfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexGenfvRequest {
        context_tag: context_tag,
        coord: coord,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTexGenfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetTexGenfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexGenivRequest {
    pub context_tag: ContextTag,
    pub coord: u32,
    pub pname: u32,
}
impl GetTexGenivRequest {
    /// Opcode for the GetTexGeniv request
    pub const fn opcode() -> u8 { 134 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let coord_bytes = self.coord.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexGenivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_geniv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGenivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexGenivRequest {
        context_tag: context_tag,
        coord: coord,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexGenivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetTexGenivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexImageRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub level: i32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
}
impl GetTexImageRequest {
    /// Opcode for the GetTexImage request
    pub const fn opcode() -> u8 { 135 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let level_bytes = self.level.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexImageRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_image<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetTexImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexImageRequest {
        context_tag: context_tag,
        target: target,
        level: level,
        format: format,
        type_: type_,
        swap_bytes: swap_bytes,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
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
impl TryParse for GetTexImageReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let (depth, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetTexParameterfvRequest {
    /// Opcode for the GetTexParameterfv request
    pub const fn opcode() -> u8 { 136 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexParameterfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexParameterfvRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTexParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetTexParameterfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetTexParameterivRequest {
    /// Opcode for the GetTexParameteriv request
    pub const fn opcode() -> u8 { 137 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexParameterivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexParameterivRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetTexParameterivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexLevelParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub level: i32,
    pub pname: u32,
}
impl GetTexLevelParameterfvRequest {
    /// Opcode for the GetTexLevelParameterfv request
    pub const fn opcode() -> u8 { 138 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let level_bytes = self.level.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexLevelParameterfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_level_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Conn, GetTexLevelParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexLevelParameterfvRequest {
        context_tag: context_tag,
        target: target,
        level: level,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTexLevelParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetTexLevelParameterfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTexLevelParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub level: i32,
    pub pname: u32,
}
impl GetTexLevelParameterivRequest {
    /// Opcode for the GetTexLevelParameteriv request
    pub const fn opcode() -> u8 { 139 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let level_bytes = self.level.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetTexLevelParameterivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_tex_level_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Conn, GetTexLevelParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexLevelParameterivRequest {
        context_tag: context_tag,
        target: target,
        level: level,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTexLevelParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetTexLevelParameterivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsEnabledRequest {
    pub context_tag: ContextTag,
    pub capability: u32,
}
impl IsEnabledRequest {
    /// Opcode for the IsEnabled request
    pub const fn opcode() -> u8 { 140 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let capability_bytes = self.capability.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            IsEnabledRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn is_enabled<Conn>(conn: &Conn, context_tag: ContextTag, capability: u32) -> Result<Cookie<'_, Conn, IsEnabledReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsEnabledRequest {
        context_tag: context_tag,
        capability: capability,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsEnabledReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl TryParse for IsEnabledReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsListRequest {
    pub context_tag: ContextTag,
    pub list: u32,
}
impl IsListRequest {
    /// Opcode for the IsList request
    pub const fn opcode() -> u8 { 141 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let list_bytes = self.list.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            IsListRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn is_list<Conn>(conn: &Conn, context_tag: ContextTag, list: u32) -> Result<Cookie<'_, Conn, IsListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsListRequest {
        context_tag: context_tag,
        list: list,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsListReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl TryParse for IsListReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlushRequest {
    pub context_tag: ContextTag,
}
impl FlushRequest {
    /// Opcode for the Flush request
    pub const fn opcode() -> u8 { 142 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            FlushRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn flush<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FlushRequest {
        context_tag: context_tag,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreTexturesResidentRequest<'input> {
    pub context_tag: ContextTag,
    pub textures: &'input [u32],
}
impl<'input> AreTexturesResidentRequest<'input> {
    /// Opcode for the AreTexturesResident request
    pub const fn opcode() -> u8 { 143 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n = i32::try_from(self.textures.len()).expect("`textures` has too many elements");
        let n_bytes = n.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            AreTexturesResidentRequest::opcode(),
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
        Ok((vec![request0.into(), textures_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn are_textures_resident<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, textures: &'input [u32]) -> Result<Cookie<'c, Conn, AreTexturesResidentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AreTexturesResidentRequest {
        context_tag: context_tag,
        textures: textures,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreTexturesResidentReply {
    pub response_type: u8,
    pub sequence: u16,
    pub ret_val: Bool32,
    pub data: Vec<bool>,
}
impl TryParse for AreTexturesResidentReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (ret_val, remaining) = Bool32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<bool>(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteTexturesRequest<'input> {
    pub context_tag: ContextTag,
    pub textures: &'input [u32],
}
impl<'input> DeleteTexturesRequest<'input> {
    /// Opcode for the DeleteTextures request
    pub const fn opcode() -> u8 { 144 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n = i32::try_from(self.textures.len()).expect("`textures` has too many elements");
        let n_bytes = n.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DeleteTexturesRequest::opcode(),
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
        Ok((vec![request0.into(), textures_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn delete_textures<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, textures: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteTexturesRequest {
        context_tag: context_tag,
        textures: textures,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenTexturesRequest {
    pub context_tag: ContextTag,
    pub n: i32,
}
impl GenTexturesRequest {
    /// Opcode for the GenTextures request
    pub const fn opcode() -> u8 { 145 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n_bytes = self.n.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GenTexturesRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn gen_textures<Conn>(conn: &Conn, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Conn, GenTexturesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GenTexturesRequest {
        context_tag: context_tag,
        n: n,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenTexturesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u32>,
}
impl TryParse for GenTexturesReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsTextureRequest {
    pub context_tag: ContextTag,
    pub texture: u32,
}
impl IsTextureRequest {
    /// Opcode for the IsTexture request
    pub const fn opcode() -> u8 { 146 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let texture_bytes = self.texture.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            IsTextureRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn is_texture<Conn>(conn: &Conn, context_tag: ContextTag, texture: u32) -> Result<Cookie<'_, Conn, IsTextureReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsTextureRequest {
        context_tag: context_tag,
        texture: texture,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsTextureReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl TryParse for IsTextureReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetColorTableRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
}
impl GetColorTableRequest {
    /// Opcode for the GetColorTable request
    pub const fn opcode() -> u8 { 147 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetColorTableRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_color_table<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetColorTableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetColorTableRequest {
        context_tag: context_tag,
        target: target,
        format: format,
        type_: type_,
        swap_bytes: swap_bytes,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetColorTableReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetColorTableReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetColorTableParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetColorTableParameterfvRequest {
    /// Opcode for the GetColorTableParameterfv request
    pub const fn opcode() -> u8 { 148 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetColorTableParameterfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_color_table_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetColorTableParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetColorTableParameterfvRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetColorTableParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetColorTableParameterfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetColorTableParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetColorTableParameterivRequest {
    /// Opcode for the GetColorTableParameteriv request
    pub const fn opcode() -> u8 { 149 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetColorTableParameterivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_color_table_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetColorTableParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetColorTableParameterivRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetColorTableParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetColorTableParameterivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetConvolutionFilterRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
}
impl GetConvolutionFilterRequest {
    /// Opcode for the GetConvolutionFilter request
    pub const fn opcode() -> u8 { 150 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetConvolutionFilterRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_convolution_filter<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetConvolutionFilterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetConvolutionFilterRequest {
        context_tag: context_tag,
        target: target,
        format: format,
        type_: type_,
        swap_bytes: swap_bytes,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetConvolutionFilterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetConvolutionFilterReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetConvolutionParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetConvolutionParameterfvRequest {
    /// Opcode for the GetConvolutionParameterfv request
    pub const fn opcode() -> u8 { 151 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetConvolutionParameterfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_convolution_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetConvolutionParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetConvolutionParameterfvRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetConvolutionParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetConvolutionParameterfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetConvolutionParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetConvolutionParameterivRequest {
    /// Opcode for the GetConvolutionParameteriv request
    pub const fn opcode() -> u8 { 152 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetConvolutionParameterivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_convolution_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetConvolutionParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetConvolutionParameterivRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetConvolutionParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetConvolutionParameterivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSeparableFilterRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
}
impl GetSeparableFilterRequest {
    /// Opcode for the GetSeparableFilter request
    pub const fn opcode() -> u8 { 153 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetSeparableFilterRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_separable_filter<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetSeparableFilterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSeparableFilterRequest {
        context_tag: context_tag,
        target: target,
        format: format,
        type_: type_,
        swap_bytes: swap_bytes,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSeparableFilterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub row_w: i32,
    pub col_h: i32,
    pub rows_and_cols: Vec<u8>,
}
impl TryParse for GetSeparableFilterReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (row_w, remaining) = i32::try_parse(remaining)?;
        let (col_h, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (rows_and_cols, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let rows_and_cols = rows_and_cols.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetHistogramRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
    pub reset: bool,
}
impl GetHistogramRequest {
    /// Opcode for the GetHistogram request
    pub const fn opcode() -> u8 { 154 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let reset_bytes = self.reset.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetHistogramRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_histogram<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Conn, GetHistogramReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetHistogramRequest {
        context_tag: context_tag,
        target: target,
        format: format,
        type_: type_,
        swap_bytes: swap_bytes,
        reset: reset,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetHistogramReply {
    pub response_type: u8,
    pub sequence: u16,
    pub width: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetHistogramReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetHistogramParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetHistogramParameterfvRequest {
    /// Opcode for the GetHistogramParameterfv request
    pub const fn opcode() -> u8 { 155 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetHistogramParameterfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_histogram_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetHistogramParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetHistogramParameterfvRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetHistogramParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetHistogramParameterfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetHistogramParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetHistogramParameterivRequest {
    /// Opcode for the GetHistogramParameteriv request
    pub const fn opcode() -> u8 { 156 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetHistogramParameterivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_histogram_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetHistogramParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetHistogramParameterivRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetHistogramParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetHistogramParameterivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMinmaxRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: bool,
    pub reset: bool,
}
impl GetMinmaxRequest {
    /// Opcode for the GetMinmax request
    pub const fn opcode() -> u8 { 157 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let format_bytes = self.format.serialize();
        let type_bytes = self.type_.serialize();
        let swap_bytes_bytes = self.swap_bytes.serialize();
        let reset_bytes = self.reset.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetMinmaxRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_minmax<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Conn, GetMinmaxReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMinmaxRequest {
        context_tag: context_tag,
        target: target,
        format: format,
        type_: type_,
        swap_bytes: swap_bytes,
        reset: reset,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMinmaxReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u8>,
}
impl TryParse for GetMinmaxReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMinmaxParameterfvRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetMinmaxParameterfvRequest {
    /// Opcode for the GetMinmaxParameterfv request
    pub const fn opcode() -> u8 { 158 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetMinmaxParameterfvRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_minmax_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMinmaxParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMinmaxParameterfvRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetMinmaxParameterfvReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl TryParse for GetMinmaxParameterfvReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = Float32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<Float32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetMinmaxParameterivRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetMinmaxParameterivRequest {
    /// Opcode for the GetMinmaxParameteriv request
    pub const fn opcode() -> u8 { 159 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetMinmaxParameterivRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_minmax_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMinmaxParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMinmaxParameterivRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMinmaxParameterivReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetMinmaxParameterivReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCompressedTexImageARBRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub level: i32,
}
impl GetCompressedTexImageARBRequest {
    /// Opcode for the GetCompressedTexImageARB request
    pub const fn opcode() -> u8 { 160 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let level_bytes = self.level.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetCompressedTexImageARBRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_compressed_tex_image_arb<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32) -> Result<Cookie<'_, Conn, GetCompressedTexImageARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCompressedTexImageARBRequest {
        context_tag: context_tag,
        target: target,
        level: level,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCompressedTexImageARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub size: i32,
    pub data: Vec<u8>,
}
impl TryParse for GetCompressedTexImageARBReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (size, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteQueriesARBRequest<'input> {
    pub context_tag: ContextTag,
    pub ids: &'input [u32],
}
impl<'input> DeleteQueriesARBRequest<'input> {
    /// Opcode for the DeleteQueriesARB request
    pub const fn opcode() -> u8 { 161 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n = i32::try_from(self.ids.len()).expect("`ids` has too many elements");
        let n_bytes = n.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DeleteQueriesARBRequest::opcode(),
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
        Ok((vec![request0.into(), ids_bytes.into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn delete_queries_arb<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, ids: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteQueriesARBRequest {
        context_tag: context_tag,
        ids: ids,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenQueriesARBRequest {
    pub context_tag: ContextTag,
    pub n: i32,
}
impl GenQueriesARBRequest {
    /// Opcode for the GenQueriesARB request
    pub const fn opcode() -> u8 { 162 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let n_bytes = self.n.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GenQueriesARBRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn gen_queries_arb<Conn>(conn: &Conn, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Conn, GenQueriesARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GenQueriesARBRequest {
        context_tag: context_tag,
        n: n,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenQueriesARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub data: Vec<u32>,
}
impl TryParse for GenQueriesARBReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsQueryARBRequest {
    pub context_tag: ContextTag,
    pub id: u32,
}
impl IsQueryARBRequest {
    /// Opcode for the IsQueryARB request
    pub const fn opcode() -> u8 { 163 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let id_bytes = self.id.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            IsQueryARBRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn is_query_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32) -> Result<Cookie<'_, Conn, IsQueryARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsQueryARBRequest {
        context_tag: context_tag,
        id: id,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsQueryARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl TryParse for IsQueryARBReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetQueryivARBRequest {
    pub context_tag: ContextTag,
    pub target: u32,
    pub pname: u32,
}
impl GetQueryivARBRequest {
    /// Opcode for the GetQueryivARB request
    pub const fn opcode() -> u8 { 164 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let target_bytes = self.target.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetQueryivARBRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_queryiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetQueryivARBRequest {
        context_tag: context_tag,
        target: target,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetQueryivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetQueryivARBReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetQueryObjectivARBRequest {
    pub context_tag: ContextTag,
    pub id: u32,
    pub pname: u32,
}
impl GetQueryObjectivARBRequest {
    /// Opcode for the GetQueryObjectivARB request
    pub const fn opcode() -> u8 { 165 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let id_bytes = self.id.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetQueryObjectivARBRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_query_objectiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryObjectivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetQueryObjectivARBRequest {
        context_tag: context_tag,
        id: id,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetQueryObjectivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: i32,
    pub data: Vec<i32>,
}
impl TryParse for GetQueryObjectivARBReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = i32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<i32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetQueryObjectuivARBRequest {
    pub context_tag: ContextTag,
    pub id: u32,
    pub pname: u32,
}
impl GetQueryObjectuivARBRequest {
    /// Opcode for the GetQueryObjectuivARB request
    pub const fn opcode() -> u8 { 166 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_tag_bytes = self.context_tag.serialize();
        let id_bytes = self.id.serialize();
        let pname_bytes = self.pname.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GetQueryObjectuivARBRequest::opcode(),
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
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_query_objectuiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryObjectuivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetQueryObjectuivARBRequest {
        context_tag: context_tag,
        id: id,
        pname: pname,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetQueryObjectuivARBReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: u32,
    pub data: Vec<u32>,
}
impl TryParse for GetQueryObjectuivARBReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (n, remaining) = u32::try_parse(remaining)?;
        let (datum, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u32>(remaining, n.try_into().or(Err(ParseError::ParseError))?)?;
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

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn glx_render<'c, 'input>(&'c self, context_tag: ContextTag, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        render(self, context_tag, data)
    }
    fn glx_render_large<'c, 'input>(&'c self, context_tag: ContextTag, request_num: u16, request_total: u16, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn glx_vendor_private<'c, 'input>(&'c self, vendor_code: u32, context_tag: ContextTag, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        vendor_private(self, vendor_code, context_tag, data)
    }
    fn glx_vendor_private_with_reply<'c, 'input>(&'c self, vendor_code: u32, context_tag: ContextTag, data: &'input [u8]) -> Result<Cookie<'c, Self, VendorPrivateWithReplyReply>, ConnectionError>
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
    fn glx_client_info<'c, 'input>(&'c self, major_version: u32, minor_version: u32, string: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        client_info(self, major_version, minor_version, string)
    }
    fn glx_get_fb_configs(&self, screen: u32) -> Result<Cookie<'_, Self, GetFBConfigsReply>, ConnectionError>
    {
        get_fb_configs(self, screen)
    }
    fn glx_create_pixmap<'c, 'input>(&'c self, screen: u32, fbconfig: Fbconfig, pixmap: xproto::Pixmap, glx_pixmap: Pixmap, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn glx_create_pbuffer<'c, 'input>(&'c self, screen: u32, fbconfig: Fbconfig, pbuffer: Pbuffer, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn glx_change_drawable_attributes<'c, 'input>(&'c self, drawable: Drawable, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_drawable_attributes(self, drawable, attribs)
    }
    fn glx_create_window<'c, 'input>(&'c self, screen: u32, fbconfig: Fbconfig, window: xproto::Window, glx_window: Window, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_window(self, screen, fbconfig, window, glx_window, attribs)
    }
    fn glx_delete_window(&self, glxwindow: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_window(self, glxwindow)
    }
    fn glx_set_client_info_arb<'c, 'input>(&'c self, major_version: u32, minor_version: u32, gl_versions: &'input [u32], gl_extension_string: &'input [u8], glx_extension_string: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_client_info_arb(self, major_version, minor_version, gl_versions, gl_extension_string, glx_extension_string)
    }
    fn glx_create_context_attribs_arb<'c, 'input>(&'c self, context: Context, fbconfig: Fbconfig, screen: u32, share_list: Context, is_direct: bool, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_context_attribs_arb(self, context, fbconfig, screen, share_list, is_direct, attribs)
    }
    fn glx_set_client_info2_arb<'c, 'input>(&'c self, major_version: u32, minor_version: u32, gl_versions: &'input [u32], gl_extension_string: &'input [u8], glx_extension_string: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn glx_are_textures_resident<'c, 'input>(&'c self, context_tag: ContextTag, textures: &'input [u32]) -> Result<Cookie<'c, Self, AreTexturesResidentReply>, ConnectionError>
    {
        are_textures_resident(self, context_tag, textures)
    }
    fn glx_delete_textures<'c, 'input>(&'c self, context_tag: ContextTag, textures: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
    fn glx_delete_queries_arb<'c, 'input>(&'c self, context_tag: ContextTag, ids: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
