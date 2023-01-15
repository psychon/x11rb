// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `DRI2` X11 extension.

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

pub use x11rb_protocol::protocol::dri2::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major_version,
        minor_version,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn connect<Conn>(conn: &Conn, window: xproto::Window, driver_type: DriverType) -> Result<Cookie<'_, Conn, ConnectReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ConnectRequest {
        window,
        driver_type,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn authenticate<Conn>(conn: &Conn, window: xproto::Window, magic: u32) -> Result<Cookie<'_, Conn, AuthenticateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AuthenticateRequest {
        window,
        magic,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn create_drawable<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateDrawableRequest {
        drawable,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn destroy_drawable<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyDrawableRequest {
        drawable,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_buffers<'c, 'input, Conn>(conn: &'c Conn, drawable: xproto::Drawable, count: u32, attachments: &'input [u32]) -> Result<Cookie<'c, Conn, GetBuffersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetBuffersRequest {
        drawable,
        count,
        attachments: Cow::Borrowed(attachments),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn copy_region<Conn>(conn: &Conn, drawable: xproto::Drawable, region: u32, dest: u32, src: u32) -> Result<Cookie<'_, Conn, CopyRegionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CopyRegionRequest {
        drawable,
        region,
        dest,
        src,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_buffers_with_format<'c, 'input, Conn>(conn: &'c Conn, drawable: xproto::Drawable, count: u32, attachments: &'input [AttachFormat]) -> Result<Cookie<'c, Conn, GetBuffersWithFormatReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetBuffersWithFormatRequest {
        drawable,
        count,
        attachments: Cow::Borrowed(attachments),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn swap_buffers<Conn>(conn: &Conn, drawable: xproto::Drawable, target_msc_hi: u32, target_msc_lo: u32, divisor_hi: u32, divisor_lo: u32, remainder_hi: u32, remainder_lo: u32) -> Result<Cookie<'_, Conn, SwapBuffersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SwapBuffersRequest {
        drawable,
        target_msc_hi,
        target_msc_lo,
        divisor_hi,
        divisor_lo,
        remainder_hi,
        remainder_lo,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_msc<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<Cookie<'_, Conn, GetMSCReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMSCRequest {
        drawable,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn wait_msc<Conn>(conn: &Conn, drawable: xproto::Drawable, target_msc_hi: u32, target_msc_lo: u32, divisor_hi: u32, divisor_lo: u32, remainder_hi: u32, remainder_lo: u32) -> Result<Cookie<'_, Conn, WaitMSCReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = WaitMSCRequest {
        drawable,
        target_msc_hi,
        target_msc_lo,
        divisor_hi,
        divisor_lo,
        remainder_hi,
        remainder_lo,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn wait_sbc<Conn>(conn: &Conn, drawable: xproto::Drawable, target_sbc_hi: u32, target_sbc_lo: u32) -> Result<Cookie<'_, Conn, WaitSBCReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = WaitSBCRequest {
        drawable,
        target_sbc_hi,
        target_sbc_lo,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn swap_interval<Conn>(conn: &Conn, drawable: xproto::Drawable, interval: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SwapIntervalRequest {
        drawable,
        interval,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_param<Conn>(conn: &Conn, drawable: xproto::Drawable, param: u32) -> Result<Cookie<'_, Conn, GetParamReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetParamRequest {
        drawable,
        param,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn dri2_query_version(&self, major_version: u32, minor_version: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self, major_version, minor_version))
    }
    fn dri2_connect(&self, window: xproto::Window, driver_type: DriverType) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, ConnectReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(connect(self, window, driver_type))
    }
    fn dri2_authenticate(&self, window: xproto::Window, magic: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, AuthenticateReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(authenticate(self, window, magic))
    }
    fn dri2_create_drawable(&self, drawable: xproto::Drawable) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_drawable(self, drawable))
    }
    fn dri2_destroy_drawable(&self, drawable: xproto::Drawable) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_drawable(self, drawable))
    }
    fn dri2_get_buffers<'c, 'input, 'future>(&'c self, drawable: xproto::Drawable, count: u32, attachments: &'input [u32]) -> Pin<Box<dyn Future<Output = Result<Cookie<'c, Self, GetBuffersReply>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(get_buffers(self, drawable, count, attachments))
    }
    fn dri2_copy_region(&self, drawable: xproto::Drawable, region: u32, dest: u32, src: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, CopyRegionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(copy_region(self, drawable, region, dest, src))
    }
    fn dri2_get_buffers_with_format<'c, 'input, 'future>(&'c self, drawable: xproto::Drawable, count: u32, attachments: &'input [AttachFormat]) -> Pin<Box<dyn Future<Output = Result<Cookie<'c, Self, GetBuffersWithFormatReply>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(get_buffers_with_format(self, drawable, count, attachments))
    }
    fn dri2_swap_buffers(&self, drawable: xproto::Drawable, target_msc_hi: u32, target_msc_lo: u32, divisor_hi: u32, divisor_lo: u32, remainder_hi: u32, remainder_lo: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, SwapBuffersReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(swap_buffers(self, drawable, target_msc_hi, target_msc_lo, divisor_hi, divisor_lo, remainder_hi, remainder_lo))
    }
    fn dri2_get_msc(&self, drawable: xproto::Drawable) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetMSCReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_msc(self, drawable))
    }
    fn dri2_wait_msc(&self, drawable: xproto::Drawable, target_msc_hi: u32, target_msc_lo: u32, divisor_hi: u32, divisor_lo: u32, remainder_hi: u32, remainder_lo: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, WaitMSCReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(wait_msc(self, drawable, target_msc_hi, target_msc_lo, divisor_hi, divisor_lo, remainder_hi, remainder_lo))
    }
    fn dri2_wait_sbc(&self, drawable: xproto::Drawable, target_sbc_hi: u32, target_sbc_lo: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, WaitSBCReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(wait_sbc(self, drawable, target_sbc_hi, target_sbc_lo))
    }
    fn dri2_swap_interval(&self, drawable: xproto::Drawable, interval: u32) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(swap_interval(self, drawable, interval))
    }
    fn dri2_get_param(&self, drawable: xproto::Drawable, param: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetParamReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_param(self, drawable, param))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
