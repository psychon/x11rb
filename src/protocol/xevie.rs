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

pub use x11rb_protocol::protocol::xevie::*;

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
pub fn query_version<Conn>(conn: &Conn, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
    };
    send_query_version(request0, conn)
}

fn send_start<'c, Conn>(req: StartRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, StartReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn start<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, StartReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = StartRequest {
        screen,
    };
    send_start(request0, conn)
}

fn send_end<'c, Conn>(req: EndRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, EndReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn end<Conn>(conn: &Conn, cmap: u32) -> Result<Cookie<'_, Conn, EndReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = EndRequest {
        cmap,
    };
    send_end(request0, conn)
}

fn send_send<'c, Conn>(req: SendRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, SendReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn send<Conn>(conn: &Conn, event: Event, data_type: u32) -> Result<Cookie<'_, Conn, SendReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SendRequest {
        event,
        data_type,
    };
    send_send(request0, conn)
}

fn send_select_input<'c, Conn>(req: SelectInputRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, SelectInputReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn select_input<Conn>(conn: &Conn, event_mask: u32) -> Result<Cookie<'_, Conn, SelectInputReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectInputRequest {
        event_mask,
    };
    send_select_input(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xevie_query_version(&self, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }
    fn xevie_start(&self, screen: u32) -> Result<Cookie<'_, Self, StartReply>, ConnectionError>
    {
        start(self, screen)
    }
    fn xevie_end(&self, cmap: u32) -> Result<Cookie<'_, Self, EndReply>, ConnectionError>
    {
        end(self, cmap)
    }
    fn xevie_send(&self, event: Event, data_type: u32) -> Result<Cookie<'_, Self, SendReply>, ConnectionError>
    {
        send(self, event, data_type)
    }
    fn xevie_select_input(&self, event_mask: u32) -> Result<Cookie<'_, Self, SelectInputReply>, ConnectionError>
    {
        select_input(self, event_mask)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
