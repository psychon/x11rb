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

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "DRI3";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 2);

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
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

/// Opcode for the Open request
pub const OPEN_REQUEST: u8 = 1;
pub fn open<Conn>(conn: &Conn, drawable: DRAWABLE, provider: u32) -> Result<CookieWithFds<'_, Conn, OpenReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let provider_bytes = provider.serialize();
    let request0 = [
        extension_information.major_opcode,
        OPEN_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        provider_bytes[0],
        provider_bytes[1],
        provider_bytes[2],
        provider_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply_with_fds(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, PartialEq, Eq)]
pub struct OpenReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub device_fd: RawFdContainer,
}
impl OpenReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::ParseError) }
        let device_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let result = OpenReply { response_type, nfd, sequence, length, device_fd };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for OpenReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

/// Opcode for the PixmapFromBuffer request
pub const PIXMAP_FROM_BUFFER_REQUEST: u8 = 2;
pub fn pixmap_from_buffer<Conn, A>(conn: &Conn, pixmap: PIXMAP, drawable: DRAWABLE, size: u32, width: u16, height: u16, stride: u16, depth: u8, bpp: u8, pixmap_fd: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<RawFdContainer>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (24) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let pixmap_bytes = pixmap.serialize();
    let drawable_bytes = drawable.serialize();
    let size_bytes = size.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let stride_bytes = stride.serialize();
    let depth_bytes = depth.serialize();
    let bpp_bytes = bpp.serialize();
    let request0 = [
        extension_information.major_opcode,
        PIXMAP_FROM_BUFFER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        pixmap_bytes[0],
        pixmap_bytes[1],
        pixmap_bytes[2],
        pixmap_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        size_bytes[0],
        size_bytes[1],
        size_bytes[2],
        size_bytes[3],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        stride_bytes[0],
        stride_bytes[1],
        depth_bytes[0],
        bpp_bytes[0],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    let fds = vec!(pixmap_fd.into());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], fds)?)
}

/// Opcode for the BufferFromPixmap request
pub const BUFFER_FROM_PIXMAP_REQUEST: u8 = 3;
pub fn buffer_from_pixmap<Conn>(conn: &Conn, pixmap: PIXMAP) -> Result<CookieWithFds<'_, Conn, BufferFromPixmapReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let pixmap_bytes = pixmap.serialize();
    let request0 = [
        extension_information.major_opcode,
        BUFFER_FROM_PIXMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        pixmap_bytes[0],
        pixmap_bytes[1],
        pixmap_bytes[2],
        pixmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply_with_fds(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, PartialEq, Eq)]
pub struct BufferFromPixmapReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
    pub pixmap_fd: RawFdContainer,
}
impl BufferFromPixmapReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (stride, remaining) = u16::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bpp, remaining) = u8::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::ParseError) }
        let pixmap_fd = fds.remove(0);
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let result = BufferFromPixmapReply { response_type, nfd, sequence, length, size, width, height, stride, depth, bpp, pixmap_fd };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for BufferFromPixmapReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

/// Opcode for the FenceFromFD request
pub const FENCE_FROM_FD_REQUEST: u8 = 4;
pub fn fence_from_fd<Conn, A>(conn: &Conn, drawable: DRAWABLE, fence: u32, initially_triggered: bool, fence_fd: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<RawFdContainer>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let fence_bytes = fence.serialize();
    let initially_triggered_bytes = (initially_triggered as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        FENCE_FROM_FD_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        fence_bytes[0],
        fence_bytes[1],
        fence_bytes[2],
        fence_bytes[3],
        initially_triggered_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    let fds = vec!(fence_fd.into());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], fds)?)
}

/// Opcode for the FDFromFence request
pub const FD_FROM_FENCE_REQUEST: u8 = 5;
pub fn fd_from_fence<Conn>(conn: &Conn, drawable: DRAWABLE, fence: u32) -> Result<CookieWithFds<'_, Conn, FDFromFenceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let fence_bytes = fence.serialize();
    let request0 = [
        extension_information.major_opcode,
        FD_FROM_FENCE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        fence_bytes[0],
        fence_bytes[1],
        fence_bytes[2],
        fence_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply_with_fds(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, PartialEq, Eq)]
pub struct FDFromFenceReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub fence_fd: RawFdContainer,
}
impl FDFromFenceReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::ParseError) }
        let fence_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let result = FDFromFenceReply { response_type, nfd, sequence, length, fence_fd };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for FDFromFenceReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

/// Opcode for the GetSupportedModifiers request
pub const GET_SUPPORTED_MODIFIERS_REQUEST: u8 = 6;
pub fn get_supported_modifiers<Conn>(conn: &Conn, window: u32, depth: u8, bpp: u8) -> Result<Cookie<'_, Conn, GetSupportedModifiersReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let depth_bytes = depth.serialize();
    let bpp_bytes = bpp.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_SUPPORTED_MODIFIERS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        depth_bytes[0],
        bpp_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSupportedModifiersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub window_modifiers: Vec<u64>,
    pub screen_modifiers: Vec<u64>,
}
impl GetSupportedModifiersReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_window_modifiers, remaining) = u32::try_parse(remaining)?;
        let (num_screen_modifiers, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (window_modifiers, remaining) = crate::x11_utils::parse_list::<u64>(remaining, num_window_modifiers as usize)?;
        let (screen_modifiers, remaining) = crate::x11_utils::parse_list::<u64>(remaining, num_screen_modifiers as usize)?;
        let result = GetSupportedModifiersReply { response_type, sequence, length, window_modifiers, screen_modifiers };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSupportedModifiersReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PixmapFromBuffers request
pub const PIXMAP_FROM_BUFFERS_REQUEST: u8 = 7;
pub fn pixmap_from_buffers<'c, Conn>(conn: &'c Conn, pixmap: PIXMAP, window: WINDOW, width: u16, height: u16, stride0: u32, offset0: u32, stride1: u32, offset1: u32, stride2: u32, offset2: u32, stride3: u32, offset3: u32, depth: u8, bpp: u8, modifier: u64, buffers: Vec<RawFdContainer>) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (64) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let pixmap_bytes = pixmap.serialize();
    let window_bytes = window.serialize();
    let num_buffers: u8 = buffers.len().try_into()?;
    let num_buffers_bytes = num_buffers.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let stride0_bytes = stride0.serialize();
    let offset0_bytes = offset0.serialize();
    let stride1_bytes = stride1.serialize();
    let offset1_bytes = offset1.serialize();
    let stride2_bytes = stride2.serialize();
    let offset2_bytes = offset2.serialize();
    let stride3_bytes = stride3.serialize();
    let offset3_bytes = offset3.serialize();
    let depth_bytes = depth.serialize();
    let bpp_bytes = bpp.serialize();
    let modifier_bytes = modifier.serialize();
    let request0 = [
        extension_information.major_opcode,
        PIXMAP_FROM_BUFFERS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        pixmap_bytes[0],
        pixmap_bytes[1],
        pixmap_bytes[2],
        pixmap_bytes[3],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        num_buffers_bytes[0],
        0,
        0,
        0,
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        stride0_bytes[0],
        stride0_bytes[1],
        stride0_bytes[2],
        stride0_bytes[3],
        offset0_bytes[0],
        offset0_bytes[1],
        offset0_bytes[2],
        offset0_bytes[3],
        stride1_bytes[0],
        stride1_bytes[1],
        stride1_bytes[2],
        stride1_bytes[3],
        offset1_bytes[0],
        offset1_bytes[1],
        offset1_bytes[2],
        offset1_bytes[3],
        stride2_bytes[0],
        stride2_bytes[1],
        stride2_bytes[2],
        stride2_bytes[3],
        offset2_bytes[0],
        offset2_bytes[1],
        offset2_bytes[2],
        offset2_bytes[3],
        stride3_bytes[0],
        stride3_bytes[1],
        stride3_bytes[2],
        stride3_bytes[3],
        offset3_bytes[0],
        offset3_bytes[1],
        offset3_bytes[2],
        offset3_bytes[3],
        depth_bytes[0],
        bpp_bytes[0],
        0,
        0,
        modifier_bytes[0],
        modifier_bytes[1],
        modifier_bytes[2],
        modifier_bytes[3],
        modifier_bytes[4],
        modifier_bytes[5],
        modifier_bytes[6],
        modifier_bytes[7],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], buffers)?)
}

/// Opcode for the BuffersFromPixmap request
pub const BUFFERS_FROM_PIXMAP_REQUEST: u8 = 8;
pub fn buffers_from_pixmap<Conn>(conn: &Conn, pixmap: PIXMAP) -> Result<CookieWithFds<'_, Conn, BuffersFromPixmapReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let pixmap_bytes = pixmap.serialize();
    let request0 = [
        extension_information.major_opcode,
        BUFFERS_FROM_PIXMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        pixmap_bytes[0],
        pixmap_bytes[1],
        pixmap_bytes[2],
        pixmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply_with_fds(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, PartialEq, Eq)]
pub struct BuffersFromPixmapReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
    pub modifier: u64,
    pub depth: u8,
    pub bpp: u8,
    pub strides: Vec<u32>,
    pub offsets: Vec<u32>,
    pub buffers: Vec<RawFdContainer>,
}
impl BuffersFromPixmapReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (modifier, remaining) = u64::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bpp, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(6..).ok_or(ParseError::ParseError)?;
        let (strides, remaining) = crate::x11_utils::parse_list::<u32>(remaining, nfd as usize)?;
        let (offsets, remaining) = crate::x11_utils::parse_list::<u32>(remaining, nfd as usize)?;
        let fds_len = nfd as usize;
        if fds.len() < fds_len { return Err(ParseError::ParseError) }
        let mut buffers = fds.split_off(fds_len);
        std::mem::swap(fds, &mut buffers);
        let result = BuffersFromPixmapReply { response_type, nfd, sequence, length, width, height, modifier, depth, bpp, strides, offsets, buffers };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for BuffersFromPixmapReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn dri3_query_version(&self, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }

    fn dri3_open(&self, drawable: DRAWABLE, provider: u32) -> Result<CookieWithFds<'_, Self, OpenReply>, ConnectionError>
    {
        open(self, drawable, provider)
    }

    fn dri3_pixmap_from_buffer<A>(&self, pixmap: PIXMAP, drawable: DRAWABLE, size: u32, width: u16, height: u16, stride: u16, depth: u8, bpp: u8, pixmap_fd: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<RawFdContainer>
    {
        pixmap_from_buffer(self, pixmap, drawable, size, width, height, stride, depth, bpp, pixmap_fd)
    }

    fn dri3_buffer_from_pixmap(&self, pixmap: PIXMAP) -> Result<CookieWithFds<'_, Self, BufferFromPixmapReply>, ConnectionError>
    {
        buffer_from_pixmap(self, pixmap)
    }

    fn dri3_fence_from_fd<A>(&self, drawable: DRAWABLE, fence: u32, initially_triggered: bool, fence_fd: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<RawFdContainer>
    {
        fence_from_fd(self, drawable, fence, initially_triggered, fence_fd)
    }

    fn dri3_fd_from_fence(&self, drawable: DRAWABLE, fence: u32) -> Result<CookieWithFds<'_, Self, FDFromFenceReply>, ConnectionError>
    {
        fd_from_fence(self, drawable, fence)
    }

    fn dri3_get_supported_modifiers(&self, window: u32, depth: u8, bpp: u8) -> Result<Cookie<'_, Self, GetSupportedModifiersReply>, ConnectionError>
    {
        get_supported_modifiers(self, window, depth, bpp)
    }

    fn dri3_pixmap_from_buffers<'c>(&'c self, pixmap: PIXMAP, window: WINDOW, width: u16, height: u16, stride0: u32, offset0: u32, stride1: u32, offset1: u32, stride2: u32, offset2: u32, stride3: u32, offset3: u32, depth: u8, bpp: u8, modifier: u64, buffers: Vec<RawFdContainer>) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        pixmap_from_buffers(self, pixmap, window, width, height, stride0, offset0, stride1, offset1, stride2, offset2, stride3, offset3, depth, bpp, modifier, buffers)
    }

    fn dri3_buffers_from_pixmap(&self, pixmap: PIXMAP) -> Result<CookieWithFds<'_, Self, BuffersFromPixmapReply>, ConnectionError>
    {
        buffers_from_pixmap(self, pixmap)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
