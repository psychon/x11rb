// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Shm` X11 extension.

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
use super::xproto;

pub use x11rb_protocol::protocol::shm::*;

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
pub async fn attach<Conn>(conn: &Conn, shmseg: Seg, shmid: u32, read_only: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AttachRequest {
        shmseg,
        shmid,
        read_only,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn detach<Conn>(conn: &Conn, shmseg: Seg) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DetachRequest {
        shmseg,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn put_image<Conn>(conn: &Conn, drawable: xproto::Drawable, gc: xproto::Gcontext, total_width: u16, total_height: u16, src_x: u16, src_y: u16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8, send_event: bool, shmseg: Seg, offset: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PutImageRequest {
        drawable,
        gc,
        total_width,
        total_height,
        src_x,
        src_y,
        src_width,
        src_height,
        dst_x,
        dst_y,
        depth,
        format,
        send_event,
        shmseg,
        offset,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_image<Conn>(conn: &Conn, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: u8, shmseg: Seg, offset: u32) -> Result<Cookie<'_, Conn, GetImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetImageRequest {
        drawable,
        x,
        y,
        width,
        height,
        plane_mask,
        format,
        shmseg,
        offset,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn create_pixmap<Conn>(conn: &Conn, pid: xproto::Pixmap, drawable: xproto::Drawable, width: u16, height: u16, depth: u8, shmseg: Seg, offset: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreatePixmapRequest {
        pid,
        drawable,
        width,
        height,
        depth,
        shmseg,
        offset,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn attach_fd<Conn, A>(conn: &Conn, shmseg: Seg, shm_fd: A, read_only: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<RawFdContainer> + Send,
{
    let shm_fd: RawFdContainer = shm_fd.into();
    let request0 = AttachFdRequest {
        shmseg,
        shm_fd,
        read_only,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_segment<Conn>(conn: &Conn, shmseg: Seg, size: u32, read_only: bool) -> Result<CookieWithFds<'_, Conn, CreateSegmentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateSegmentRequest {
        shmseg,
        size,
        read_only,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply_with_fds(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn shm_query_version(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self))
    }
    fn shm_attach(&self, shmseg: Seg, shmid: u32, read_only: bool) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(attach(self, shmseg, shmid, read_only))
    }
    fn shm_detach(&self, shmseg: Seg) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(detach(self, shmseg))
    }
    fn shm_put_image(&self, drawable: xproto::Drawable, gc: xproto::Gcontext, total_width: u16, total_height: u16, src_x: u16, src_y: u16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8, send_event: bool, shmseg: Seg, offset: u32) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(put_image(self, drawable, gc, total_width, total_height, src_x, src_y, src_width, src_height, dst_x, dst_y, depth, format, send_event, shmseg, offset))
    }
    fn shm_get_image(&self, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: u8, shmseg: Seg, offset: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetImageReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_image(self, drawable, x, y, width, height, plane_mask, format, shmseg, offset))
    }
    fn shm_create_pixmap(&self, pid: xproto::Pixmap, drawable: xproto::Drawable, width: u16, height: u16, depth: u8, shmseg: Seg, offset: u32) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_pixmap(self, pid, drawable, width, height, depth, shmseg, offset))
    }
    fn shm_attach_fd<A>(&self, shmseg: Seg, shm_fd: A, read_only: bool) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    where
        A: Into<RawFdContainer> + Send + 'static,
    {
        Box::pin(attach_fd(self, shmseg, shm_fd, read_only))
    }
    fn shm_create_segment(&self, shmseg: Seg, size: u32, read_only: bool) -> Pin<Box<dyn Future<Output = Result<CookieWithFds<'_, Self, CreateSegmentReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_segment(self, shmseg, size, read_only))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
