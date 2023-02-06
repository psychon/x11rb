// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XCMisc` X11 extension.

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

pub use x11rb_protocol::protocol::xc_misc::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn get_version<Conn>(conn: &Conn, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Conn, GetVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetVersionRequest {
        client_major_version,
        client_minor_version,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_xid_range<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetXIDRangeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetXIDRangeRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_xid_list<Conn>(conn: &Conn, count: u32) -> Result<Cookie<'_, Conn, GetXIDListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetXIDListRequest {
        count,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xc_misc_get_version(&self, client_major_version: u16, client_minor_version: u16) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_version(self, client_major_version, client_minor_version))
    }
    fn xc_misc_get_xid_range(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetXIDRangeReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_xid_range(self))
    }
    fn xc_misc_get_xid_list(&self, count: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetXIDListReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_xid_list(self, count))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
