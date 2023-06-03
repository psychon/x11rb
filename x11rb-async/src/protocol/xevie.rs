// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xevie` X11 extension.

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

pub use x11rb_protocol::protocol::xevie::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn query_version<Conn>(conn: &Conn, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn start<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, StartReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = StartRequest {
        screen,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn end<Conn>(conn: &Conn, cmap: u32) -> Result<Cookie<'_, Conn, EndReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = EndRequest {
        cmap,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn send<Conn>(conn: &Conn, event: Event, data_type: u32) -> Result<Cookie<'_, Conn, SendReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SendRequest {
        event,
        data_type,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn select_input<Conn>(conn: &Conn, event_mask: u32) -> Result<Cookie<'_, Conn, SelectInputReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectInputRequest {
        event_mask,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xevie_query_version(&self, client_major_version: u16, client_minor_version: u16) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self, client_major_version, client_minor_version))
    }
    fn xevie_start(&self, screen: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, StartReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(start(self, screen))
    }
    fn xevie_end(&self, cmap: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, EndReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(end(self, cmap))
    }
    fn xevie_send(&self, event: Event, data_type: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, SendReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(send(self, event, data_type))
    }
    fn xevie_select_input(&self, event_mask: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, SelectInputReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(select_input(self, event_mask))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
