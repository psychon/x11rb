// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XvMC` X11 extension.

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
#[allow(unused_imports)]
use super::xv;

pub use x11rb_protocol::protocol::xvmc::*;

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
pub async fn list_surface_types<Conn>(conn: &Conn, port_id: xv::Port) -> Result<Cookie<'_, Conn, ListSurfaceTypesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSurfaceTypesRequest {
        port_id,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn create_context<Conn>(conn: &Conn, context_id: Context, port_id: xv::Port, surface_id: Surface, width: u16, height: u16, flags: u32) -> Result<Cookie<'_, Conn, CreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        context_id,
        port_id,
        surface_id,
        width,
        height,
        flags,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn destroy_context<Conn>(conn: &Conn, context_id: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyContextRequest {
        context_id,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_surface<Conn>(conn: &Conn, surface_id: Surface, context_id: Context) -> Result<Cookie<'_, Conn, CreateSurfaceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateSurfaceRequest {
        surface_id,
        context_id,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn destroy_surface<Conn>(conn: &Conn, surface_id: Surface) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroySurfaceRequest {
        surface_id,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_subpicture<Conn>(conn: &Conn, subpicture_id: Subpicture, context: Context, xvimage_id: u32, width: u16, height: u16) -> Result<Cookie<'_, Conn, CreateSubpictureReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateSubpictureRequest {
        subpicture_id,
        context,
        xvimage_id,
        width,
        height,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn destroy_subpicture<Conn>(conn: &Conn, subpicture_id: Subpicture) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroySubpictureRequest {
        subpicture_id,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn list_subpicture_types<Conn>(conn: &Conn, port_id: xv::Port, surface_id: Surface) -> Result<Cookie<'_, Conn, ListSubpictureTypesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSubpictureTypesRequest {
        port_id,
        surface_id,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xvmc_query_version(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self))
    }
    fn xvmc_list_surface_types(&self, port_id: xv::Port) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, ListSurfaceTypesReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(list_surface_types(self, port_id))
    }
    fn xvmc_create_context(&self, context_id: Context, port_id: xv::Port, surface_id: Surface, width: u16, height: u16, flags: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, CreateContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_context(self, context_id, port_id, surface_id, width, height, flags))
    }
    fn xvmc_destroy_context(&self, context_id: Context) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_context(self, context_id))
    }
    fn xvmc_create_surface(&self, surface_id: Surface, context_id: Context) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, CreateSurfaceReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_surface(self, surface_id, context_id))
    }
    fn xvmc_destroy_surface(&self, surface_id: Surface) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_surface(self, surface_id))
    }
    fn xvmc_create_subpicture(&self, subpicture_id: Subpicture, context: Context, xvimage_id: u32, width: u16, height: u16) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, CreateSubpictureReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_subpicture(self, subpicture_id, context, xvimage_id, width, height))
    }
    fn xvmc_destroy_subpicture(&self, subpicture_id: Subpicture) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_subpicture(self, subpicture_id))
    }
    fn xvmc_list_subpicture_types(&self, port_id: xv::Port, surface_id: Surface) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, ListSubpictureTypesReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(list_subpicture_types(self, port_id, surface_id))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
