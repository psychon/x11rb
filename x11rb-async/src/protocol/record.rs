// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Record` X11 extension.

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
use crate::cookie::RecordEnableContextCookie;
use crate::errors::ConnectionError;
#[allow(unused_imports)]
use crate::errors::ReplyOrIdError;
use std::future::Future;
use std::pin::Pin;

pub use x11rb_protocol::protocol::record::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn query_version<Conn>(conn: &Conn, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
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
pub async fn create_context<'c, 'input, Conn>(conn: &'c Conn, context: Context, element_header: ElementHeader, client_specs: &'input [ClientSpec], ranges: &'input [Range]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        context,
        element_header,
        client_specs: Cow::Borrowed(client_specs),
        ranges: Cow::Borrowed(ranges),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn register_clients<'c, 'input, Conn>(conn: &'c Conn, context: Context, element_header: ElementHeader, client_specs: &'input [ClientSpec], ranges: &'input [Range]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RegisterClientsRequest {
        context,
        element_header,
        client_specs: Cow::Borrowed(client_specs),
        ranges: Cow::Borrowed(ranges),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn unregister_clients<'c, 'input, Conn>(conn: &'c Conn, context: Context, client_specs: &'input [ClientSpec]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UnregisterClientsRequest {
        context,
        client_specs: Cow::Borrowed(client_specs),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_context<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, GetContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn enable_context<Conn>(conn: &Conn, context: Context) -> Result<RecordEnableContextCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = EnableContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    Ok(RecordEnableContextCookie::new(conn.send_request_with_reply(&slices, fds).await?))
}
pub async fn disable_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DisableContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn free_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FreeContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn record_query_version(&self, major_version: u16, minor_version: u16) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self, major_version, minor_version))
    }
    fn record_create_context<'c, 'input, 'future>(&'c self, context: Context, element_header: ElementHeader, client_specs: &'input [ClientSpec], ranges: &'input [Range]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(create_context(self, context, element_header, client_specs, ranges))
    }
    fn record_register_clients<'c, 'input, 'future>(&'c self, context: Context, element_header: ElementHeader, client_specs: &'input [ClientSpec], ranges: &'input [Range]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(register_clients(self, context, element_header, client_specs, ranges))
    }
    fn record_unregister_clients<'c, 'input, 'future>(&'c self, context: Context, client_specs: &'input [ClientSpec]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(unregister_clients(self, context, client_specs))
    }
    fn record_get_context(&self, context: Context) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_context(self, context))
    }
    fn record_enable_context(&self, context: Context) -> Pin<Box<dyn Future<Output = Result<RecordEnableContextCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(enable_context(self, context))
    }
    fn record_disable_context(&self, context: Context) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(disable_context(self, context))
    }
    fn record_free_context(&self, context: Context) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(free_context(self, context))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
