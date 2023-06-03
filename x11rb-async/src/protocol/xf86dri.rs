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
use std::future::Future;
use std::pin::Pin;

pub use x11rb_protocol::protocol::xf86dri::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn query_direct_rendering_capable<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, QueryDirectRenderingCapableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryDirectRenderingCapableRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn open_connection<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, OpenConnectionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = OpenConnectionRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn close_connection<Conn>(conn: &Conn, screen: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CloseConnectionRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_client_driver_name<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetClientDriverNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClientDriverNameRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn create_context<Conn>(conn: &Conn, screen: u32, visual: u32, context: u32) -> Result<Cookie<'_, Conn, CreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        screen,
        visual,
        context,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn destroy_context<Conn>(conn: &Conn, screen: u32, context: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyContextRequest {
        screen,
        context,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_drawable<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<Cookie<'_, Conn, CreateDrawableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateDrawableRequest {
        screen,
        drawable,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn destroy_drawable<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyDrawableRequest {
        screen,
        drawable,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_drawable_info<Conn>(conn: &Conn, screen: u32, drawable: u32) -> Result<Cookie<'_, Conn, GetDrawableInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDrawableInfoRequest {
        screen,
        drawable,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_device_info<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetDeviceInfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceInfoRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn auth_connection<Conn>(conn: &Conn, screen: u32, magic: u32) -> Result<Cookie<'_, Conn, AuthConnectionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AuthConnectionRequest {
        screen,
        magic,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xf86dri_query_version(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self))
    }
    fn xf86dri_query_direct_rendering_capable(&self, screen: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryDirectRenderingCapableReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_direct_rendering_capable(self, screen))
    }
    fn xf86dri_open_connection(&self, screen: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, OpenConnectionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(open_connection(self, screen))
    }
    fn xf86dri_close_connection(&self, screen: u32) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(close_connection(self, screen))
    }
    fn xf86dri_get_client_driver_name(&self, screen: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetClientDriverNameReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_client_driver_name(self, screen))
    }
    fn xf86dri_create_context(&self, screen: u32, visual: u32, context: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, CreateContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_context(self, screen, visual, context))
    }
    fn xf86dri_destroy_context(&self, screen: u32, context: u32) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_context(self, screen, context))
    }
    fn xf86dri_create_drawable(&self, screen: u32, drawable: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, CreateDrawableReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_drawable(self, screen, drawable))
    }
    fn xf86dri_destroy_drawable(&self, screen: u32, drawable: u32) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_drawable(self, screen, drawable))
    }
    fn xf86dri_get_drawable_info(&self, screen: u32, drawable: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetDrawableInfoReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_drawable_info(self, screen, drawable))
    }
    fn xf86dri_get_device_info(&self, screen: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetDeviceInfoReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_device_info(self, screen))
    }
    fn xf86dri_auth_connection(&self, screen: u32, magic: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, AuthConnectionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(auth_connection(self, screen, magic))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
