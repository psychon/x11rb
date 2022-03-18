// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XF86Dri` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use std::borrow::Cow;
#[allow(unused_imports)]
use std::convert::TryInto;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use std::io::IoSlice;
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::connection::Connection as X11Connection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::ConnectionError;
#[allow(unused_imports)]
use crate::errors::ReplyOrIdError;

pub use x11rb_protocol::protocol::xf86dri::*;

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

fn send_query_version<'c, Conn>(req: QueryVersionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest;
    send_query_version(request0, conn)
}

fn send_query_direct_rendering_capable<'c, Conn>(req: QueryDirectRenderingCapableRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryDirectRenderingCapableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_direct_rendering_capable<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, QueryDirectRenderingCapableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryDirectRenderingCapableRequest {
        screen,
    };
    send_query_direct_rendering_capable(request0, conn)
}

fn send_open_connection<'c, Conn>(req: OpenConnectionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, OpenConnectionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn open_connection<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, OpenConnectionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = OpenConnectionRequest {
        screen,
    };
    send_open_connection(request0, conn)
}

fn send_close_connection<'c, Conn>(req: CloseConnectionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn close_connection<Conn>(conn: &Conn, screen: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CloseConnectionRequest {
        screen,
    };
    send_close_connection(request0, conn)
}

fn send_get_client_driver_name<'c, Conn>(req: GetClientDriverNameRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetClientDriverNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_client_driver_name<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetClientDriverNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClientDriverNameRequest {
        screen,
    };
    send_get_client_driver_name(request0, conn)
}

fn send_create_context<'c, Conn>(req: CreateContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, CreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn create_context<Conn>(conn: &Conn, screen: u32, visual: u32, context: u32) -> Result<Cookie<'_, Conn, CreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        screen,
        visual,
        context,
    };
    send_create_context(request0, conn)
}

fn send_destroy_context<'c, Conn>(req: DestroyContextRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn destroy_context<Conn>(conn: &Conn, screen: u32, context: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyContextRequest {
        screen,
        context,
    };
    send_destroy_context(request0, conn)
}

fn send_create_drawable<'c, Conn>(req: CreateDrawableRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, CreateDrawableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn create_drawable<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<Cookie<'_, Conn, CreateDrawableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateDrawableRequest {
        screen,
        drawable,
    };
    send_create_drawable(request0, conn)
}

fn send_destroy_drawable<'c, Conn>(req: DestroyDrawableRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn destroy_drawable<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyDrawableRequest {
        screen,
        drawable,
    };
    send_destroy_drawable(request0, conn)
}

fn send_get_drawable_info<'c, Conn>(req: GetDrawableInfoRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDrawableInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_drawable_info<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<Cookie<'_, Conn, GetDrawableInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDrawableInfoRequest {
        screen,
        drawable,
    };
    send_get_drawable_info(request0, conn)
}

fn send_get_device_info<'c, Conn>(req: GetDeviceInfoRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_info<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetDeviceInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceInfoRequest {
        screen,
    };
    send_get_device_info(request0, conn)
}

fn send_auth_connection<'c, Conn>(req: AuthConnectionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, AuthConnectionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn auth_connection<Conn>(conn: &Conn, screen: u32, magic: u32) -> Result<Cookie<'_, Conn, AuthConnectionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AuthConnectionRequest {
        screen,
        magic,
    };
    send_auth_connection(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xf86dri_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }
    fn xf86dri_query_direct_rendering_capable(&self, screen: u32) -> Result<Cookie<'_, Self, QueryDirectRenderingCapableReply>, ConnectionError>
    {
        query_direct_rendering_capable(self, screen)
    }
    fn xf86dri_open_connection(&self, screen: u32) -> Result<Cookie<'_, Self, OpenConnectionReply>, ConnectionError>
    {
        open_connection(self, screen)
    }
    fn xf86dri_close_connection(&self, screen: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        close_connection(self, screen)
    }
    fn xf86dri_get_client_driver_name(&self, screen: u32) -> Result<Cookie<'_, Self, GetClientDriverNameReply>, ConnectionError>
    {
        get_client_driver_name(self, screen)
    }
    fn xf86dri_create_context(&self, screen: u32, visual: u32, context: u32) -> Result<Cookie<'_, Self, CreateContextReply>, ConnectionError>
    {
        create_context(self, screen, visual, context)
    }
    fn xf86dri_destroy_context(&self, screen: u32, context: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_context(self, screen, context)
    }
    fn xf86dri_create_drawable(&self, screen: u32, drawable: u32) -> Result<Cookie<'_, Self, CreateDrawableReply>, ConnectionError>
    {
        create_drawable(self, screen, drawable)
    }
    fn xf86dri_destroy_drawable(&self, screen: u32, drawable: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_drawable(self, screen, drawable)
    }
    fn xf86dri_get_drawable_info(&self, screen: u32, drawable: u32) -> Result<Cookie<'_, Self, GetDrawableInfoReply>, ConnectionError>
    {
        get_drawable_info(self, screen, drawable)
    }
    fn xf86dri_get_device_info(&self, screen: u32) -> Result<Cookie<'_, Self, GetDeviceInfoReply>, ConnectionError>
    {
        get_device_info(self, screen)
    }
    fn xf86dri_auth_connection(&self, screen: u32, magic: u32) -> Result<Cookie<'_, Self, AuthConnectionReply>, ConnectionError>
    {
        auth_connection(self, screen, magic)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
