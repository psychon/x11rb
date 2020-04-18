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
#[allow(unused_imports)]
use super::render;
#[allow(unused_imports)]
use super::shape;
#[allow(unused_imports)]
use super::xfixes;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "Composite";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (0, 4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Redirect {
    Automatic = 0,
    Manual = 1,
}
impl From<Redirect> for u8 {
    fn from(input: Redirect) -> Self {
        match input {
            Redirect::Automatic => 0,
            Redirect::Manual => 1,
        }
    }
}
impl From<Redirect> for Option<u8> {
    fn from(input: Redirect) -> Self {
        Some(u8::from(input))
    }
}
impl From<Redirect> for u16 {
    fn from(input: Redirect) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Redirect> for Option<u16> {
    fn from(input: Redirect) -> Self {
        Some(u16::from(input))
    }
}
impl From<Redirect> for u32 {
    fn from(input: Redirect) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Redirect> for Option<u32> {
    fn from(input: Redirect) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Redirect {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Redirect::Automatic),
            1 => Ok(Redirect::Manual),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Redirect {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Redirect {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
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

/// Opcode for the RedirectWindow request
pub const REDIRECT_WINDOW_REQUEST: u8 = 1;
pub fn redirect_window<Conn, A>(conn: &Conn, window: xproto::Window, update: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let update = update.into();
    let update_bytes = update.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the RedirectSubwindows request
pub const REDIRECT_SUBWINDOWS_REQUEST: u8 = 2;
pub fn redirect_subwindows<Conn, A>(conn: &Conn, window: xproto::Window, update: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let update = update.into();
    let update_bytes = update.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the UnredirectWindow request
pub const UNREDIRECT_WINDOW_REQUEST: u8 = 3;
pub fn unredirect_window<Conn, A>(conn: &Conn, window: xproto::Window, update: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let update = update.into();
    let update_bytes = update.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the UnredirectSubwindows request
pub const UNREDIRECT_SUBWINDOWS_REQUEST: u8 = 4;
pub fn unredirect_subwindows<Conn, A>(conn: &Conn, window: xproto::Window, update: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let update = update.into();
    let update_bytes = update.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the CreateRegionFromBorderClip request
pub const CREATE_REGION_FROM_BORDER_CLIP_REQUEST: u8 = 5;
pub fn create_region_from_border_clip<Conn>(conn: &Conn, region: xfixes::Region, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let region_bytes = region.serialize();
    let window_bytes = window.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the NameWindowPixmap request
pub const NAME_WINDOW_PIXMAP_REQUEST: u8 = 6;
pub fn name_window_pixmap<Conn>(conn: &Conn, window: xproto::Window, pixmap: xproto::Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let pixmap_bytes = pixmap.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the GetOverlayWindow request
pub const GET_OVERLAY_WINDOW_REQUEST: u8 = 7;
pub fn get_overlay_window<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetOverlayWindowReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_OVERLAY_WINDOW_REQUEST,
        0,
        0,
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetOverlayWindowReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub overlay_win: xproto::Window,
}
impl GetOverlayWindowReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (overlay_win, remaining) = xproto::Window::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = GetOverlayWindowReply { response_type, sequence, length, overlay_win };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetOverlayWindowReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ReleaseOverlayWindow request
pub const RELEASE_OVERLAY_WINDOW_REQUEST: u8 = 8;
pub fn release_overlay_window<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        RELEASE_OVERLAY_WINDOW_REQUEST,
        0,
        0,
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = length_so_far + (&request0).len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn composite_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }

    fn composite_redirect_window<A>(&self, window: xproto::Window, update: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        redirect_window(self, window, update)
    }

    fn composite_redirect_subwindows<A>(&self, window: xproto::Window, update: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        redirect_subwindows(self, window, update)
    }

    fn composite_unredirect_window<A>(&self, window: xproto::Window, update: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        unredirect_window(self, window, update)
    }

    fn composite_unredirect_subwindows<A>(&self, window: xproto::Window, update: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u8>,
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
