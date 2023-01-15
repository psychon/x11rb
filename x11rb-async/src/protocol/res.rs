// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Res` X11 extension.

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

pub use x11rb_protocol::protocol::res::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn query_version<Conn>(conn: &Conn, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major,
        client_minor,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn query_clients<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryClientsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryClientsRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn query_client_resources<Conn>(conn: &Conn, xid: u32) -> Result<Cookie<'_, Conn, QueryClientResourcesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryClientResourcesRequest {
        xid,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn query_client_pixmap_bytes<Conn>(conn: &Conn, xid: u32) -> Result<Cookie<'_, Conn, QueryClientPixmapBytesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryClientPixmapBytesRequest {
        xid,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn query_client_ids<'c, 'input, Conn>(conn: &'c Conn, specs: &'input [ClientIdSpec]) -> Result<Cookie<'c, Conn, QueryClientIdsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryClientIdsRequest {
        specs: Cow::Borrowed(specs),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn query_resource_bytes<'c, 'input, Conn>(conn: &'c Conn, client: u32, specs: &'input [ResourceIdSpec]) -> Result<Cookie<'c, Conn, QueryResourceBytesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryResourceBytesRequest {
        client,
        specs: Cow::Borrowed(specs),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = bytes.iter().map(|b| IoSlice::new(b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn res_query_version(&self, client_major: u8, client_minor: u8) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self, client_major, client_minor))
    }
    fn res_query_clients(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryClientsReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_clients(self))
    }
    fn res_query_client_resources(&self, xid: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryClientResourcesReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_client_resources(self, xid))
    }
    fn res_query_client_pixmap_bytes(&self, xid: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryClientPixmapBytesReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_client_pixmap_bytes(self, xid))
    }
    fn res_query_client_ids<'c, 'input, 'future>(&'c self, specs: &'input [ClientIdSpec]) -> Pin<Box<dyn Future<Output = Result<Cookie<'c, Self, QueryClientIdsReply>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(query_client_ids(self, specs))
    }
    fn res_query_resource_bytes<'c, 'input, 'future>(&'c self, client: u32, specs: &'input [ResourceIdSpec]) -> Pin<Box<dyn Future<Output = Result<Cookie<'c, Self, QueryResourceBytesReply>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(query_resource_bytes(self, client, specs))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
