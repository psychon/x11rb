// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Composite` X11 extension.

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
use super::xfixes;
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "Composite";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (0, 4);

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Redirect(u8);
impl Redirect {
    pub const AUTOMATIC: Self = Self(0);
    pub const MANUAL: Self = Self(1);
}
impl From<Redirect> for u8 {
    #[inline]
    fn from(input: Redirect) -> Self {
        input.0
    }
}
impl From<Redirect> for Option<u8> {
    #[inline]
    fn from(input: Redirect) -> Self {
        Some(input.0)
    }
}
impl From<Redirect> for u16 {
    #[inline]
    fn from(input: Redirect) -> Self {
        u16::from(input.0)
    }
}
impl From<Redirect> for Option<u16> {
    #[inline]
    fn from(input: Redirect) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Redirect> for u32 {
    #[inline]
    fn from(input: Redirect) -> Self {
        u32::from(input.0)
    }
}
impl From<Redirect> for Option<u32> {
    #[inline]
    fn from(input: Redirect) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Redirect {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Redirect  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::AUTOMATIC.0.into(), "AUTOMATIC", "Automatic"),
            (Self::MANUAL.0.into(), "MANUAL", "Manual"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub client_major_version: u32,
    pub client_minor_version: u32,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let client_major_version_bytes = self.client_major_version.serialize();
        let client_minor_version_bytes = self.client_minor_version.serialize();
        let mut request0 = vec![
            major_opcode,
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
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (client_major_version, remaining) = u32::try_parse(value)?;
        let (client_minor_version, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            client_major_version,
            client_minor_version,
        })
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
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

/// Opcode for the RedirectWindow request
pub const REDIRECT_WINDOW_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedirectWindowRequest {
    pub window: xproto::Window,
    pub update: Redirect,
}
impl RedirectWindowRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let update_bytes = u8::from(self.update).serialize();
        let mut request0 = vec![
            major_opcode,
            REDIRECT_WINDOW_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            update_bytes[0],
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != REDIRECT_WINDOW_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (update, remaining) = u8::try_parse(remaining)?;
        let update = update.into();
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(RedirectWindowRequest {
            window,
            update,
        })
    }
}
impl Request for RedirectWindowRequest {
    type Reply = ();
}
pub fn redirect_window<Conn>(conn: &Conn, window: xproto::Window, update: Redirect) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RedirectWindowRequest {
        window,
        update,
    };
    request0.send(conn)
}

/// Opcode for the RedirectSubwindows request
pub const REDIRECT_SUBWINDOWS_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedirectSubwindowsRequest {
    pub window: xproto::Window,
    pub update: Redirect,
}
impl RedirectSubwindowsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let update_bytes = u8::from(self.update).serialize();
        let mut request0 = vec![
            major_opcode,
            REDIRECT_SUBWINDOWS_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            update_bytes[0],
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != REDIRECT_SUBWINDOWS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (update, remaining) = u8::try_parse(remaining)?;
        let update = update.into();
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(RedirectSubwindowsRequest {
            window,
            update,
        })
    }
}
impl Request for RedirectSubwindowsRequest {
    type Reply = ();
}
pub fn redirect_subwindows<Conn>(conn: &Conn, window: xproto::Window, update: Redirect) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RedirectSubwindowsRequest {
        window,
        update,
    };
    request0.send(conn)
}

/// Opcode for the UnredirectWindow request
pub const UNREDIRECT_WINDOW_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnredirectWindowRequest {
    pub window: xproto::Window,
    pub update: Redirect,
}
impl UnredirectWindowRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let update_bytes = u8::from(self.update).serialize();
        let mut request0 = vec![
            major_opcode,
            UNREDIRECT_WINDOW_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            update_bytes[0],
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != UNREDIRECT_WINDOW_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (update, remaining) = u8::try_parse(remaining)?;
        let update = update.into();
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(UnredirectWindowRequest {
            window,
            update,
        })
    }
}
impl Request for UnredirectWindowRequest {
    type Reply = ();
}
pub fn unredirect_window<Conn>(conn: &Conn, window: xproto::Window, update: Redirect) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UnredirectWindowRequest {
        window,
        update,
    };
    request0.send(conn)
}

/// Opcode for the UnredirectSubwindows request
pub const UNREDIRECT_SUBWINDOWS_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnredirectSubwindowsRequest {
    pub window: xproto::Window,
    pub update: Redirect,
}
impl UnredirectSubwindowsRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let update_bytes = u8::from(self.update).serialize();
        let mut request0 = vec![
            major_opcode,
            UNREDIRECT_SUBWINDOWS_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            update_bytes[0],
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
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != UNREDIRECT_SUBWINDOWS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (update, remaining) = u8::try_parse(remaining)?;
        let update = update.into();
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(UnredirectSubwindowsRequest {
            window,
            update,
        })
    }
}
impl Request for UnredirectSubwindowsRequest {
    type Reply = ();
}
pub fn unredirect_subwindows<Conn>(conn: &Conn, window: xproto::Window, update: Redirect) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UnredirectSubwindowsRequest {
        window,
        update,
    };
    request0.send(conn)
}

/// Opcode for the CreateRegionFromBorderClip request
pub const CREATE_REGION_FROM_BORDER_CLIP_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateRegionFromBorderClipRequest {
    pub region: xfixes::Region,
    pub window: xproto::Window,
}
impl CreateRegionFromBorderClipRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let region_bytes = self.region.serialize();
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_REGION_FROM_BORDER_CLIP_REQUEST,
            0,
            0,
            region_bytes[0],
            region_bytes[1],
            region_bytes[2],
            region_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_REGION_FROM_BORDER_CLIP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (region, remaining) = xfixes::Region::try_parse(value)?;
        let (window, remaining) = xproto::Window::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateRegionFromBorderClipRequest {
            region,
            window,
        })
    }
}
impl Request for CreateRegionFromBorderClipRequest {
    type Reply = ();
}
pub fn create_region_from_border_clip<Conn>(conn: &Conn, region: xfixes::Region, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromBorderClipRequest {
        region,
        window,
    };
    request0.send(conn)
}

/// Opcode for the NameWindowPixmap request
pub const NAME_WINDOW_PIXMAP_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameWindowPixmapRequest {
    pub window: xproto::Window,
    pub pixmap: xproto::Pixmap,
}
impl NameWindowPixmapRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let pixmap_bytes = self.pixmap.serialize();
        let mut request0 = vec![
            major_opcode,
            NAME_WINDOW_PIXMAP_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            pixmap_bytes[0],
            pixmap_bytes[1],
            pixmap_bytes[2],
            pixmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != NAME_WINDOW_PIXMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (pixmap, remaining) = xproto::Pixmap::try_parse(remaining)?;
        let _ = remaining;
        Ok(NameWindowPixmapRequest {
            window,
            pixmap,
        })
    }
}
impl Request for NameWindowPixmapRequest {
    type Reply = ();
}
pub fn name_window_pixmap<Conn>(conn: &Conn, window: xproto::Window, pixmap: xproto::Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = NameWindowPixmapRequest {
        window,
        pixmap,
    };
    request0.send(conn)
}

/// Opcode for the GetOverlayWindow request
pub const GET_OVERLAY_WINDOW_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOverlayWindowRequest {
    pub window: xproto::Window,
}
impl GetOverlayWindowRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_OVERLAY_WINDOW_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetOverlayWindowReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_OVERLAY_WINDOW_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetOverlayWindowRequest {
            window,
        })
    }
}
impl Request for GetOverlayWindowRequest {
    type Reply = GetOverlayWindowReply;
}
pub fn get_overlay_window<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetOverlayWindowReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetOverlayWindowRequest {
        window,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOverlayWindowReply {
    pub sequence: u16,
    pub length: u32,
    pub overlay_win: xproto::Window,
}
impl TryParse for GetOverlayWindowReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (overlay_win, remaining) = xproto::Window::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetOverlayWindowReply { sequence, length, overlay_win };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the ReleaseOverlayWindow request
pub const RELEASE_OVERLAY_WINDOW_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReleaseOverlayWindowRequest {
    pub window: xproto::Window,
}
impl ReleaseOverlayWindowRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            RELEASE_OVERLAY_WINDOW_REQUEST,
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
        (vec![request0.into()], vec![])
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(major_opcode(conn)?);
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != RELEASE_OVERLAY_WINDOW_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(ReleaseOverlayWindowRequest {
            window,
        })
    }
}
impl Request for ReleaseOverlayWindowRequest {
    type Reply = ();
}
pub fn release_overlay_window<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ReleaseOverlayWindowRequest {
        window,
    };
    request0.send(conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn composite_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }
    fn composite_redirect_window(&self, window: xproto::Window, update: Redirect) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        redirect_window(self, window, update)
    }
    fn composite_redirect_subwindows(&self, window: xproto::Window, update: Redirect) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        redirect_subwindows(self, window, update)
    }
    fn composite_unredirect_window(&self, window: xproto::Window, update: Redirect) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        unredirect_window(self, window, update)
    }
    fn composite_unredirect_subwindows(&self, window: xproto::Window, update: Redirect) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        unredirect_subwindows(self, window, update)
    }
    fn composite_create_region_from_border_clip(&self, region: xfixes::Region, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_border_clip(self, region, window)
    }
    fn composite_name_window_pixmap(&self, window: xproto::Window, pixmap: xproto::Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        name_window_pixmap(self, window, pixmap)
    }
    fn composite_get_overlay_window(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetOverlayWindowReply>, ConnectionError>
    {
        get_overlay_window(self, window)
    }
    fn composite_release_overlay_window(&self, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        release_overlay_window(self, window)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
