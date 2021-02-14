// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `DRI3` X11 extension.

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
use super::xproto;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub major_version: u32,
    pub minor_version: u32,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
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
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
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
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major_version,
        minor_version,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Opcode for the Open request
pub const OPEN_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenRequest {
    pub drawable: xproto::Drawable,
    pub provider: u32,
}
impl OpenRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let provider_bytes = self.provider.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            OPEN_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<CookieWithFds<'_, Conn, OpenReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply_with_fds(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != OPEN_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (provider, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(OpenRequest {
            drawable,
            provider,
        })
    }
}
impl Request for OpenRequest {
    type Reply = OpenReply;
}
pub fn open<Conn>(conn: &Conn, drawable: xproto::Drawable, provider: u32) -> Result<CookieWithFds<'_, Conn, OpenReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = OpenRequest {
        drawable,
        provider,
    };
    request0.send(conn)
}

#[derive(Debug, PartialEq, Eq)]
pub struct OpenReply {
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub device_fd: RawFdContainer,
}
impl TryParseFd for OpenReply {
    fn try_parse_fd<'a>(initial_value: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::MissingFileDescriptors) }
        let device_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = OpenReply { nfd, sequence, length, device_fd };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the PixmapFromBuffer request
pub const PIXMAP_FROM_BUFFER_REQUEST: u8 = 2;
#[derive(Debug, PartialEq, Eq)]
pub struct PixmapFromBufferRequest {
    pub pixmap: xproto::Pixmap,
    pub drawable: xproto::Drawable,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
    pub pixmap_fd: RawFdContainer,
}
impl PixmapFromBufferRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pixmap_bytes = self.pixmap.serialize();
        let drawable_bytes = self.drawable.serialize();
        let size_bytes = self.size.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let stride_bytes = self.stride.serialize();
        let depth_bytes = self.depth.serialize();
        let bpp_bytes = self.bpp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            PIXMAP_FROM_BUFFER_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![self.pixmap_fd]))
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
    pub fn try_parse_request_fd(header: RequestHeader, value: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<Self, ParseError> {
        if header.minor_opcode != PIXMAP_FROM_BUFFER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (pixmap, remaining) = xproto::Pixmap::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (size, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (stride, remaining) = u16::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bpp, remaining) = u8::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::MissingFileDescriptors) }
        let pixmap_fd = fds.remove(0);
        let _ = remaining;
        Ok(PixmapFromBufferRequest {
            pixmap,
            drawable,
            size,
            width,
            height,
            stride,
            depth,
            bpp,
            pixmap_fd,
        })
    }
}
impl Request for PixmapFromBufferRequest {
    type Reply = ();
}
pub fn pixmap_from_buffer<Conn, A>(conn: &Conn, pixmap: xproto::Pixmap, drawable: xproto::Drawable, size: u32, width: u16, height: u16, stride: u16, depth: u8, bpp: u8, pixmap_fd: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<RawFdContainer>,
{
    let pixmap_fd: RawFdContainer = pixmap_fd.into();
    let request0 = PixmapFromBufferRequest {
        pixmap,
        drawable,
        size,
        width,
        height,
        stride,
        depth,
        bpp,
        pixmap_fd,
    };
    request0.send(conn)
}

/// Opcode for the BufferFromPixmap request
pub const BUFFER_FROM_PIXMAP_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferFromPixmapRequest {
    pub pixmap: xproto::Pixmap,
}
impl BufferFromPixmapRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pixmap_bytes = self.pixmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            BUFFER_FROM_PIXMAP_REQUEST,
            0,
            0,
            pixmap_bytes[0],
            pixmap_bytes[1],
            pixmap_bytes[2],
            pixmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<CookieWithFds<'_, Conn, BufferFromPixmapReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply_with_fds(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != BUFFER_FROM_PIXMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (pixmap, remaining) = xproto::Pixmap::try_parse(value)?;
        let _ = remaining;
        Ok(BufferFromPixmapRequest {
            pixmap,
        })
    }
}
impl Request for BufferFromPixmapRequest {
    type Reply = BufferFromPixmapReply;
}
pub fn buffer_from_pixmap<Conn>(conn: &Conn, pixmap: xproto::Pixmap) -> Result<CookieWithFds<'_, Conn, BufferFromPixmapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = BufferFromPixmapRequest {
        pixmap,
    };
    request0.send(conn)
}

#[derive(Debug, PartialEq, Eq)]
pub struct BufferFromPixmapReply {
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
impl TryParseFd for BufferFromPixmapReply {
    fn try_parse_fd<'a>(initial_value: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let remaining = initial_value;
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
        if fds.is_empty() { return Err(ParseError::MissingFileDescriptors) }
        let pixmap_fd = fds.remove(0);
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = BufferFromPixmapReply { nfd, sequence, length, size, width, height, stride, depth, bpp, pixmap_fd };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the FenceFromFD request
pub const FENCE_FROM_FD_REQUEST: u8 = 4;
#[derive(Debug, PartialEq, Eq)]
pub struct FenceFromFDRequest {
    pub drawable: xproto::Drawable,
    pub fence: u32,
    pub initially_triggered: bool,
    pub fence_fd: RawFdContainer,
}
impl FenceFromFDRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let fence_bytes = self.fence.serialize();
        let initially_triggered_bytes = self.initially_triggered.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            FENCE_FROM_FD_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![self.fence_fd]))
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
    pub fn try_parse_request_fd(header: RequestHeader, value: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<Self, ParseError> {
        if header.minor_opcode != FENCE_FROM_FD_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (fence, remaining) = u32::try_parse(remaining)?;
        let (initially_triggered, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        if fds.is_empty() { return Err(ParseError::MissingFileDescriptors) }
        let fence_fd = fds.remove(0);
        let _ = remaining;
        Ok(FenceFromFDRequest {
            drawable,
            fence,
            initially_triggered,
            fence_fd,
        })
    }
}
impl Request for FenceFromFDRequest {
    type Reply = ();
}
pub fn fence_from_fd<Conn, A>(conn: &Conn, drawable: xproto::Drawable, fence: u32, initially_triggered: bool, fence_fd: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<RawFdContainer>,
{
    let fence_fd: RawFdContainer = fence_fd.into();
    let request0 = FenceFromFDRequest {
        drawable,
        fence,
        initially_triggered,
        fence_fd,
    };
    request0.send(conn)
}

/// Opcode for the FDFromFence request
pub const FD_FROM_FENCE_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FDFromFenceRequest {
    pub drawable: xproto::Drawable,
    pub fence: u32,
}
impl FDFromFenceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let fence_bytes = self.fence.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            FD_FROM_FENCE_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            fence_bytes[0],
            fence_bytes[1],
            fence_bytes[2],
            fence_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<CookieWithFds<'_, Conn, FDFromFenceReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply_with_fds(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != FD_FROM_FENCE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (fence, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(FDFromFenceRequest {
            drawable,
            fence,
        })
    }
}
impl Request for FDFromFenceRequest {
    type Reply = FDFromFenceReply;
}
pub fn fd_from_fence<Conn>(conn: &Conn, drawable: xproto::Drawable, fence: u32) -> Result<CookieWithFds<'_, Conn, FDFromFenceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FDFromFenceRequest {
        drawable,
        fence,
    };
    request0.send(conn)
}

#[derive(Debug, PartialEq, Eq)]
pub struct FDFromFenceReply {
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub fence_fd: RawFdContainer,
}
impl TryParseFd for FDFromFenceReply {
    fn try_parse_fd<'a>(initial_value: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::MissingFileDescriptors) }
        let fence_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = FDFromFenceReply { nfd, sequence, length, fence_fd };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the GetSupportedModifiers request
pub const GET_SUPPORTED_MODIFIERS_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSupportedModifiersRequest {
    pub window: u32,
    pub depth: u8,
    pub bpp: u8,
}
impl GetSupportedModifiersRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let depth_bytes = self.depth.serialize();
        let bpp_bytes = self.bpp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_SUPPORTED_MODIFIERS_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            depth_bytes[0],
            bpp_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetSupportedModifiersReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SUPPORTED_MODIFIERS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = u32::try_parse(value)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bpp, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(GetSupportedModifiersRequest {
            window,
            depth,
            bpp,
        })
    }
}
impl Request for GetSupportedModifiersRequest {
    type Reply = GetSupportedModifiersReply;
}
pub fn get_supported_modifiers<Conn>(conn: &Conn, window: u32, depth: u8, bpp: u8) -> Result<Cookie<'_, Conn, GetSupportedModifiersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSupportedModifiersRequest {
        window,
        depth,
        bpp,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSupportedModifiersReply {
    pub sequence: u16,
    pub length: u32,
    pub window_modifiers: Vec<u64>,
    pub screen_modifiers: Vec<u64>,
}
impl TryParse for GetSupportedModifiersReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_window_modifiers, remaining) = u32::try_parse(remaining)?;
        let (num_screen_modifiers, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (window_modifiers, remaining) = crate::x11_utils::parse_list::<u64>(remaining, num_window_modifiers.try_to_usize()?)?;
        let (screen_modifiers, remaining) = crate::x11_utils::parse_list::<u64>(remaining, num_screen_modifiers.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSupportedModifiersReply { sequence, length, window_modifiers, screen_modifiers };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl GetSupportedModifiersReply {
    /// Get the value of the `num_window_modifiers` field.
    ///
    /// The `num_window_modifiers` field is used as the length field of the `window_modifiers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_window_modifiers(&self) -> u32 {
        self.window_modifiers.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_screen_modifiers` field.
    ///
    /// The `num_screen_modifiers` field is used as the length field of the `screen_modifiers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_screen_modifiers(&self) -> u32 {
        self.screen_modifiers.len()
            .try_into().unwrap()
    }
}

/// Opcode for the PixmapFromBuffers request
pub const PIXMAP_FROM_BUFFERS_REQUEST: u8 = 7;
#[derive(Debug, PartialEq, Eq)]
pub struct PixmapFromBuffersRequest {
    pub pixmap: xproto::Pixmap,
    pub window: xproto::Window,
    pub width: u16,
    pub height: u16,
    pub stride0: u32,
    pub offset0: u32,
    pub stride1: u32,
    pub offset1: u32,
    pub stride2: u32,
    pub offset2: u32,
    pub stride3: u32,
    pub offset3: u32,
    pub depth: u8,
    pub bpp: u8,
    pub modifier: u64,
    pub buffers: Vec<RawFdContainer>,
}
impl PixmapFromBuffersRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pixmap_bytes = self.pixmap.serialize();
        let window_bytes = self.window.serialize();
        let num_buffers = u8::try_from(self.buffers.len()).expect("`buffers` has too many elements");
        let num_buffers_bytes = num_buffers.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let stride0_bytes = self.stride0.serialize();
        let offset0_bytes = self.offset0.serialize();
        let stride1_bytes = self.stride1.serialize();
        let offset1_bytes = self.offset1.serialize();
        let stride2_bytes = self.stride2.serialize();
        let offset2_bytes = self.offset2.serialize();
        let stride3_bytes = self.stride3.serialize();
        let offset3_bytes = self.offset3.serialize();
        let depth_bytes = self.depth.serialize();
        let bpp_bytes = self.bpp.serialize();
        let modifier_bytes = self.modifier.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            PIXMAP_FROM_BUFFERS_REQUEST,
            0,
            0,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], self.buffers))
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
    pub fn try_parse_request_fd(header: RequestHeader, value: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<Self, ParseError> {
        if header.minor_opcode != PIXMAP_FROM_BUFFERS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (pixmap, remaining) = xproto::Pixmap::try_parse(value)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let (num_buffers, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (stride0, remaining) = u32::try_parse(remaining)?;
        let (offset0, remaining) = u32::try_parse(remaining)?;
        let (stride1, remaining) = u32::try_parse(remaining)?;
        let (offset1, remaining) = u32::try_parse(remaining)?;
        let (stride2, remaining) = u32::try_parse(remaining)?;
        let (offset2, remaining) = u32::try_parse(remaining)?;
        let (stride3, remaining) = u32::try_parse(remaining)?;
        let (offset3, remaining) = u32::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bpp, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (modifier, remaining) = u64::try_parse(remaining)?;
        let fds_len = num_buffers.try_to_usize()?;
        if fds.len() < fds_len { return Err(ParseError::MissingFileDescriptors) }
        let mut buffers = fds.split_off(fds_len);
        std::mem::swap(fds, &mut buffers);
        let _ = remaining;
        Ok(PixmapFromBuffersRequest {
            pixmap,
            window,
            width,
            height,
            stride0,
            offset0,
            stride1,
            offset1,
            stride2,
            offset2,
            stride3,
            offset3,
            depth,
            bpp,
            modifier,
            buffers,
        })
    }
}
impl Request for PixmapFromBuffersRequest {
    type Reply = ();
}
pub fn pixmap_from_buffers<Conn>(conn: &Conn, pixmap: xproto::Pixmap, window: xproto::Window, width: u16, height: u16, stride0: u32, offset0: u32, stride1: u32, offset1: u32, stride2: u32, offset2: u32, stride3: u32, offset3: u32, depth: u8, bpp: u8, modifier: u64, buffers: Vec<RawFdContainer>) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PixmapFromBuffersRequest {
        pixmap,
        window,
        width,
        height,
        stride0,
        offset0,
        stride1,
        offset1,
        stride2,
        offset2,
        stride3,
        offset3,
        depth,
        bpp,
        modifier,
        buffers,
    };
    request0.send(conn)
}

/// Opcode for the BuffersFromPixmap request
pub const BUFFERS_FROM_PIXMAP_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuffersFromPixmapRequest {
    pub pixmap: xproto::Pixmap,
}
impl BuffersFromPixmapRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pixmap_bytes = self.pixmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            BUFFERS_FROM_PIXMAP_REQUEST,
            0,
            0,
            pixmap_bytes[0],
            pixmap_bytes[1],
            pixmap_bytes[2],
            pixmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<CookieWithFds<'_, Conn, BuffersFromPixmapReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply_with_fds(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != BUFFERS_FROM_PIXMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (pixmap, remaining) = xproto::Pixmap::try_parse(value)?;
        let _ = remaining;
        Ok(BuffersFromPixmapRequest {
            pixmap,
        })
    }
}
impl Request for BuffersFromPixmapRequest {
    type Reply = BuffersFromPixmapReply;
}
pub fn buffers_from_pixmap<Conn>(conn: &Conn, pixmap: xproto::Pixmap) -> Result<CookieWithFds<'_, Conn, BuffersFromPixmapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = BuffersFromPixmapRequest {
        pixmap,
    };
    request0.send(conn)
}

#[derive(Debug, PartialEq, Eq)]
pub struct BuffersFromPixmapReply {
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
impl TryParseFd for BuffersFromPixmapReply {
    fn try_parse_fd<'a>(initial_value: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (modifier, remaining) = u64::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bpp, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(6..).ok_or(ParseError::InsufficientData)?;
        let (strides, remaining) = crate::x11_utils::parse_list::<u32>(remaining, nfd.try_to_usize()?)?;
        let (offsets, remaining) = crate::x11_utils::parse_list::<u32>(remaining, nfd.try_to_usize()?)?;
        let fds_len = nfd.try_to_usize()?;
        if fds.len() < fds_len { return Err(ParseError::MissingFileDescriptors) }
        let mut buffers = fds.split_off(fds_len);
        std::mem::swap(fds, &mut buffers);
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = BuffersFromPixmapReply { sequence, length, width, height, modifier, depth, bpp, strides, offsets, buffers };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl BuffersFromPixmapReply {
    /// Get the value of the `nfd` field.
    ///
    /// The `nfd` field is used as the length field of the `strides` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn nfd(&self) -> u8 {
        self.strides.len()
            .try_into().unwrap()
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn dri3_query_version(&self, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }
    fn dri3_open(&self, drawable: xproto::Drawable, provider: u32) -> Result<CookieWithFds<'_, Self, OpenReply>, ConnectionError>
    {
        open(self, drawable, provider)
    }
    fn dri3_pixmap_from_buffer<A>(&self, pixmap: xproto::Pixmap, drawable: xproto::Drawable, size: u32, width: u16, height: u16, stride: u16, depth: u8, bpp: u8, pixmap_fd: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<RawFdContainer>,
    {
        pixmap_from_buffer(self, pixmap, drawable, size, width, height, stride, depth, bpp, pixmap_fd)
    }
    fn dri3_buffer_from_pixmap(&self, pixmap: xproto::Pixmap) -> Result<CookieWithFds<'_, Self, BufferFromPixmapReply>, ConnectionError>
    {
        buffer_from_pixmap(self, pixmap)
    }
    fn dri3_fence_from_fd<A>(&self, drawable: xproto::Drawable, fence: u32, initially_triggered: bool, fence_fd: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<RawFdContainer>,
    {
        fence_from_fd(self, drawable, fence, initially_triggered, fence_fd)
    }
    fn dri3_fd_from_fence(&self, drawable: xproto::Drawable, fence: u32) -> Result<CookieWithFds<'_, Self, FDFromFenceReply>, ConnectionError>
    {
        fd_from_fence(self, drawable, fence)
    }
    fn dri3_get_supported_modifiers(&self, window: u32, depth: u8, bpp: u8) -> Result<Cookie<'_, Self, GetSupportedModifiersReply>, ConnectionError>
    {
        get_supported_modifiers(self, window, depth, bpp)
    }
    fn dri3_pixmap_from_buffers(&self, pixmap: xproto::Pixmap, window: xproto::Window, width: u16, height: u16, stride0: u32, offset0: u32, stride1: u32, offset1: u32, stride2: u32, offset2: u32, stride3: u32, offset3: u32, depth: u8, bpp: u8, modifier: u64, buffers: Vec<RawFdContainer>) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        pixmap_from_buffers(self, pixmap, window, width, height, stride0, offset0, stride1, offset1, stride2, offset2, stride3, offset3, depth, bpp, modifier, buffers)
    }
    fn dri3_buffers_from_pixmap(&self, pixmap: xproto::Pixmap) -> Result<CookieWithFds<'_, Self, BuffersFromPixmapReply>, ConnectionError>
    {
        buffers_from_pixmap(self, pixmap)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
