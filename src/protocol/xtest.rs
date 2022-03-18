// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Test` X11 extension.

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
#[allow(unused_imports)]
use super::xproto;

pub use x11rb_protocol::protocol::xtest::*;

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

fn send_get_version<'c, Conn>(req: GetVersionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_version<Conn>(conn: &Conn, major_version: u8, minor_version: u16) -> Result<Cookie<'_, Conn, GetVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetVersionRequest {
        major_version,
        minor_version,
    };
    send_get_version(request0, conn)
}

fn send_compare_cursor<'c, Conn>(req: CompareCursorRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, CompareCursorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn compare_cursor<Conn>(conn: &Conn, window: xproto::Window, cursor: xproto::Cursor) -> Result<Cookie<'_, Conn, CompareCursorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CompareCursorRequest {
        window,
        cursor,
    };
    send_compare_cursor(request0, conn)
}

fn send_fake_input<'c, Conn>(req: FakeInputRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn fake_input<Conn>(conn: &Conn, type_: u8, detail: u8, time: u32, root: xproto::Window, root_x: i16, root_y: i16, deviceid: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FakeInputRequest {
        type_,
        detail,
        time,
        root,
        root_x,
        root_y,
        deviceid,
    };
    send_fake_input(request0, conn)
}

fn send_grab_control<'c, Conn>(req: GrabControlRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn grab_control<Conn>(conn: &Conn, impervious: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GrabControlRequest {
        impervious,
    };
    send_grab_control(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xtest_get_version(&self, major_version: u8, minor_version: u16) -> Result<Cookie<'_, Self, GetVersionReply>, ConnectionError>
    {
        get_version(self, major_version, minor_version)
    }
    fn xtest_compare_cursor(&self, window: xproto::Window, cursor: xproto::Cursor) -> Result<Cookie<'_, Self, CompareCursorReply>, ConnectionError>
    {
        compare_cursor(self, window, cursor)
    }
    fn xtest_fake_input(&self, type_: u8, detail: u8, time: u32, root: xproto::Window, root_x: i16, root_y: i16, deviceid: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        fake_input(self, type_, detail, time, root, root_x, root_y, deviceid)
    }
    fn xtest_grab_control(&self, impervious: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        grab_control(self, impervious)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
