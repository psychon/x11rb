// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Shape` X11 extension.

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

pub use x11rb_protocol::protocol::shape::*;

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
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest;
    send_query_version(request0, conn)
}

fn send_rectangles<'c, Conn>(req: RectanglesRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn rectangles<'c, 'input, Conn>(conn: &'c Conn, operation: SO, destination_kind: SK, ordering: xproto::ClipOrdering, destination_window: xproto::Window, x_offset: i16, y_offset: i16, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RectanglesRequest {
        operation,
        destination_kind,
        ordering,
        destination_window,
        x_offset,
        y_offset,
        rectangles: Cow::Borrowed(rectangles),
    };
    send_rectangles(request0, conn)
}

fn send_mask<'c, Conn>(req: MaskRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn mask<Conn, A>(conn: &Conn, operation: SO, destination_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16, source_bitmap: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Pixmap>,
{
    let source_bitmap: xproto::Pixmap = source_bitmap.into();
    let request0 = MaskRequest {
        operation,
        destination_kind,
        destination_window,
        x_offset,
        y_offset,
        source_bitmap,
    };
    send_mask(request0, conn)
}

fn send_combine<'c, Conn>(req: CombineRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn combine<Conn>(conn: &Conn, operation: SO, destination_kind: SK, source_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16, source_window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CombineRequest {
        operation,
        destination_kind,
        source_kind,
        destination_window,
        x_offset,
        y_offset,
        source_window,
    };
    send_combine(request0, conn)
}

fn send_offset<'c, Conn>(req: OffsetRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn offset<Conn>(conn: &Conn, destination_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = OffsetRequest {
        destination_kind,
        destination_window,
        x_offset,
        y_offset,
    };
    send_offset(request0, conn)
}

fn send_query_extents<'c, Conn>(req: QueryExtentsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryExtentsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_extents<Conn>(conn: &Conn, destination_window: xproto::Window) -> Result<Cookie<'_, Conn, QueryExtentsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryExtentsRequest {
        destination_window,
    };
    send_query_extents(request0, conn)
}

fn send_select_input<'c, Conn>(req: SelectInputRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn select_input<Conn>(conn: &Conn, destination_window: xproto::Window, enable: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectInputRequest {
        destination_window,
        enable,
    };
    send_select_input(request0, conn)
}

fn send_input_selected<'c, Conn>(req: InputSelectedRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, InputSelectedReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn input_selected<Conn>(conn: &Conn, destination_window: xproto::Window) -> Result<Cookie<'_, Conn, InputSelectedReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InputSelectedRequest {
        destination_window,
    };
    send_input_selected(request0, conn)
}

fn send_get_rectangles<'c, Conn>(req: GetRectanglesRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetRectanglesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_rectangles<Conn>(conn: &Conn, window: xproto::Window, source_kind: SK) -> Result<Cookie<'_, Conn, GetRectanglesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetRectanglesRequest {
        window,
        source_kind,
    };
    send_get_rectangles(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn shape_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }
    fn shape_rectangles<'c, 'input>(&'c self, operation: SO, destination_kind: SK, ordering: xproto::ClipOrdering, destination_window: xproto::Window, x_offset: i16, y_offset: i16, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        self::rectangles(self, operation, destination_kind, ordering, destination_window, x_offset, y_offset, rectangles)
    }
    fn shape_mask<A>(&self, operation: SO, destination_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16, source_bitmap: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Pixmap>,
    {
        mask(self, operation, destination_kind, destination_window, x_offset, y_offset, source_bitmap)
    }
    fn shape_combine(&self, operation: SO, destination_kind: SK, source_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16, source_window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        combine(self, operation, destination_kind, source_kind, destination_window, x_offset, y_offset, source_window)
    }
    fn shape_offset(&self, destination_kind: SK, destination_window: xproto::Window, x_offset: i16, y_offset: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        offset(self, destination_kind, destination_window, x_offset, y_offset)
    }
    fn shape_query_extents(&self, destination_window: xproto::Window) -> Result<Cookie<'_, Self, QueryExtentsReply>, ConnectionError>
    {
        query_extents(self, destination_window)
    }
    fn shape_select_input(&self, destination_window: xproto::Window, enable: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_input(self, destination_window, enable)
    }
    fn shape_input_selected(&self, destination_window: xproto::Window) -> Result<Cookie<'_, Self, InputSelectedReply>, ConnectionError>
    {
        input_selected(self, destination_window)
    }
    fn shape_get_rectangles(&self, window: xproto::Window, source_kind: SK) -> Result<Cookie<'_, Self, GetRectanglesReply>, ConnectionError>
    {
        get_rectangles(self, window, source_kind)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
