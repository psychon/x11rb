// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XFixes` X11 extension.

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
use super::render;
#[allow(unused_imports)]
use super::shape;
#[allow(unused_imports)]
use super::xproto;

pub use x11rb_protocol::protocol::xfixes::*;

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
pub fn query_version<Conn>(conn: &Conn, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
    };
    send_query_version(request0, conn)
}

fn send_change_save_set<'c, Conn>(req: ChangeSaveSetRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn change_save_set<Conn>(conn: &Conn, mode: SaveSetMode, target: SaveSetTarget, map: SaveSetMapping, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeSaveSetRequest {
        mode,
        target,
        map,
        window,
    };
    send_change_save_set(request0, conn)
}

fn send_select_selection_input<'c, Conn>(req: SelectSelectionInputRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn select_selection_input<Conn, A>(conn: &Conn, window: xproto::Window, selection: xproto::Atom, event_mask: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectSelectionInputRequest {
        window,
        selection,
        event_mask,
    };
    send_select_selection_input(request0, conn)
}

fn send_select_cursor_input<'c, Conn>(req: SelectCursorInputRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn select_cursor_input<Conn, A>(conn: &Conn, window: xproto::Window, event_mask: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let event_mask: u32 = event_mask.into();
    let request0 = SelectCursorInputRequest {
        window,
        event_mask,
    };
    send_select_cursor_input(request0, conn)
}

fn send_get_cursor_image<'c, Conn>(req: GetCursorImageRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetCursorImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_cursor_image<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetCursorImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCursorImageRequest;
    send_get_cursor_image(request0, conn)
}

fn send_create_region<'c, Conn>(req: CreateRegionRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_region<'c, 'input, Conn>(conn: &'c Conn, region: Region, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionRequest {
        region,
        rectangles: Cow::Borrowed(rectangles),
    };
    send_create_region(request0, conn)
}

fn send_create_region_from_bitmap<'c, Conn>(req: CreateRegionFromBitmapRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_region_from_bitmap<Conn>(conn: &Conn, region: Region, bitmap: xproto::Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromBitmapRequest {
        region,
        bitmap,
    };
    send_create_region_from_bitmap(request0, conn)
}

fn send_create_region_from_window<'c, Conn>(req: CreateRegionFromWindowRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_region_from_window<Conn>(conn: &Conn, region: Region, window: xproto::Window, kind: shape::SK) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromWindowRequest {
        region,
        window,
        kind,
    };
    send_create_region_from_window(request0, conn)
}

fn send_create_region_from_gc<'c, Conn>(req: CreateRegionFromGCRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_region_from_gc<Conn>(conn: &Conn, region: Region, gc: xproto::Gcontext) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromGCRequest {
        region,
        gc,
    };
    send_create_region_from_gc(request0, conn)
}

fn send_create_region_from_picture<'c, Conn>(req: CreateRegionFromPictureRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_region_from_picture<Conn>(conn: &Conn, region: Region, picture: render::Picture) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRegionFromPictureRequest {
        region,
        picture,
    };
    send_create_region_from_picture(request0, conn)
}

fn send_destroy_region<'c, Conn>(req: DestroyRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn destroy_region<Conn>(conn: &Conn, region: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyRegionRequest {
        region,
    };
    send_destroy_region(request0, conn)
}

fn send_set_region<'c, Conn>(req: SetRegionRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_region<'c, 'input, Conn>(conn: &'c Conn, region: Region, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetRegionRequest {
        region,
        rectangles: Cow::Borrowed(rectangles),
    };
    send_set_region(request0, conn)
}

fn send_copy_region<'c, Conn>(req: CopyRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn copy_region<Conn>(conn: &Conn, source: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CopyRegionRequest {
        source,
        destination,
    };
    send_copy_region(request0, conn)
}

fn send_union_region<'c, Conn>(req: UnionRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn union_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UnionRegionRequest {
        source1,
        source2,
        destination,
    };
    send_union_region(request0, conn)
}

fn send_intersect_region<'c, Conn>(req: IntersectRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn intersect_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IntersectRegionRequest {
        source1,
        source2,
        destination,
    };
    send_intersect_region(request0, conn)
}

fn send_subtract_region<'c, Conn>(req: SubtractRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn subtract_region<Conn>(conn: &Conn, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SubtractRegionRequest {
        source1,
        source2,
        destination,
    };
    send_subtract_region(request0, conn)
}

fn send_invert_region<'c, Conn>(req: InvertRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn invert_region<Conn>(conn: &Conn, source: Region, bounds: xproto::Rectangle, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InvertRegionRequest {
        source,
        bounds,
        destination,
    };
    send_invert_region(request0, conn)
}

fn send_translate_region<'c, Conn>(req: TranslateRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn translate_region<Conn>(conn: &Conn, region: Region, dx: i16, dy: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TranslateRegionRequest {
        region,
        dx,
        dy,
    };
    send_translate_region(request0, conn)
}

fn send_region_extents<'c, Conn>(req: RegionExtentsRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn region_extents<Conn>(conn: &Conn, source: Region, destination: Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RegionExtentsRequest {
        source,
        destination,
    };
    send_region_extents(request0, conn)
}

fn send_fetch_region<'c, Conn>(req: FetchRegionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, FetchRegionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn fetch_region<Conn>(conn: &Conn, region: Region) -> Result<Cookie<'_, Conn, FetchRegionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FetchRegionRequest {
        region,
    };
    send_fetch_region(request0, conn)
}

fn send_set_gc_clip_region<'c, Conn>(req: SetGCClipRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_gc_clip_region<Conn, A>(conn: &Conn, gc: xproto::Gcontext, region: A, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetGCClipRegionRequest {
        gc,
        region,
        x_origin,
        y_origin,
    };
    send_set_gc_clip_region(request0, conn)
}

fn send_set_window_shape_region<'c, Conn>(req: SetWindowShapeRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_window_shape_region<Conn, A>(conn: &Conn, dest: xproto::Window, dest_kind: shape::SK, x_offset: i16, y_offset: i16, region: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetWindowShapeRegionRequest {
        dest,
        dest_kind,
        x_offset,
        y_offset,
        region,
    };
    send_set_window_shape_region(request0, conn)
}

fn send_set_picture_clip_region<'c, Conn>(req: SetPictureClipRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_picture_clip_region<Conn, A>(conn: &Conn, picture: render::Picture, region: A, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Region>,
{
    let region: Region = region.into();
    let request0 = SetPictureClipRegionRequest {
        picture,
        region,
        x_origin,
        y_origin,
    };
    send_set_picture_clip_region(request0, conn)
}

fn send_set_cursor_name<'c, Conn>(req: SetCursorNameRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_cursor_name<'c, 'input, Conn>(conn: &'c Conn, cursor: xproto::Cursor, name: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetCursorNameRequest {
        cursor,
        name: Cow::Borrowed(name),
    };
    send_set_cursor_name(request0, conn)
}

fn send_get_cursor_name<'c, Conn>(req: GetCursorNameRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetCursorNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_cursor_name<Conn>(conn: &Conn, cursor: xproto::Cursor) -> Result<Cookie<'_, Conn, GetCursorNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCursorNameRequest {
        cursor,
    };
    send_get_cursor_name(request0, conn)
}

fn send_get_cursor_image_and_name<'c, Conn>(req: GetCursorImageAndNameRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetCursorImageAndNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_cursor_image_and_name<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetCursorImageAndNameReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCursorImageAndNameRequest;
    send_get_cursor_image_and_name(request0, conn)
}

fn send_change_cursor<'c, Conn>(req: ChangeCursorRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn change_cursor<Conn>(conn: &Conn, source: xproto::Cursor, destination: xproto::Cursor) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeCursorRequest {
        source,
        destination,
    };
    send_change_cursor(request0, conn)
}

fn send_change_cursor_by_name<'c, Conn>(req: ChangeCursorByNameRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn change_cursor_by_name<'c, 'input, Conn>(conn: &'c Conn, src: xproto::Cursor, name: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeCursorByNameRequest {
        src,
        name: Cow::Borrowed(name),
    };
    send_change_cursor_by_name(request0, conn)
}

fn send_expand_region<'c, Conn>(req: ExpandRegionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn expand_region<Conn>(conn: &Conn, source: Region, destination: Region, left: u16, right: u16, top: u16, bottom: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ExpandRegionRequest {
        source,
        destination,
        left,
        right,
        top,
        bottom,
    };
    send_expand_region(request0, conn)
}

fn send_hide_cursor<'c, Conn>(req: HideCursorRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn hide_cursor<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = HideCursorRequest {
        window,
    };
    send_hide_cursor(request0, conn)
}

fn send_show_cursor<'c, Conn>(req: ShowCursorRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn show_cursor<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ShowCursorRequest {
        window,
    };
    send_show_cursor(request0, conn)
}

fn send_create_pointer_barrier<'c, Conn>(req: CreatePointerBarrierRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_pointer_barrier<'c, 'input, Conn, A>(conn: &'c Conn, barrier: Barrier, window: xproto::Window, x1: u16, y1: u16, x2: u16, y2: u16, directions: A, devices: &'input [u16]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let directions: u32 = directions.into();
    let request0 = CreatePointerBarrierRequest {
        barrier,
        window,
        x1,
        y1,
        x2,
        y2,
        directions,
        devices: Cow::Borrowed(devices),
    };
    send_create_pointer_barrier(request0, conn)
}

fn send_delete_pointer_barrier<'c, Conn>(req: DeletePointerBarrierRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn delete_pointer_barrier<Conn>(conn: &Conn, barrier: Barrier) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeletePointerBarrierRequest {
        barrier,
    };
    send_delete_pointer_barrier(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xfixes_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major_version, client_minor_version)
    }
    fn xfixes_change_save_set(&self, mode: SaveSetMode, target: SaveSetTarget, map: SaveSetMapping, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        change_save_set(self, mode, target, map, window)
    }
    fn xfixes_select_selection_input<A>(&self, window: xproto::Window, selection: xproto::Atom, event_mask: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        select_selection_input(self, window, selection, event_mask)
    }
    fn xfixes_select_cursor_input<A>(&self, window: xproto::Window, event_mask: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        select_cursor_input(self, window, event_mask)
    }
    fn xfixes_get_cursor_image(&self) -> Result<Cookie<'_, Self, GetCursorImageReply>, ConnectionError>
    {
        get_cursor_image(self)
    }
    fn xfixes_create_region<'c, 'input>(&'c self, region: Region, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_region(self, region, rectangles)
    }
    fn xfixes_create_region_from_bitmap(&self, region: Region, bitmap: xproto::Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_bitmap(self, region, bitmap)
    }
    fn xfixes_create_region_from_window(&self, region: Region, window: xproto::Window, kind: shape::SK) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_window(self, region, window, kind)
    }
    fn xfixes_create_region_from_gc(&self, region: Region, gc: xproto::Gcontext) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_gc(self, region, gc)
    }
    fn xfixes_create_region_from_picture(&self, region: Region, picture: render::Picture) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_region_from_picture(self, region, picture)
    }
    fn xfixes_destroy_region(&self, region: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_region(self, region)
    }
    fn xfixes_set_region<'c, 'input>(&'c self, region: Region, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_region(self, region, rectangles)
    }
    fn xfixes_copy_region(&self, source: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        copy_region(self, source, destination)
    }
    fn xfixes_union_region(&self, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        union_region(self, source1, source2, destination)
    }
    fn xfixes_intersect_region(&self, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        intersect_region(self, source1, source2, destination)
    }
    fn xfixes_subtract_region(&self, source1: Region, source2: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        subtract_region(self, source1, source2, destination)
    }
    fn xfixes_invert_region(&self, source: Region, bounds: xproto::Rectangle, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        invert_region(self, source, bounds, destination)
    }
    fn xfixes_translate_region(&self, region: Region, dx: i16, dy: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        translate_region(self, region, dx, dy)
    }
    fn xfixes_region_extents(&self, source: Region, destination: Region) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        region_extents(self, source, destination)
    }
    fn xfixes_fetch_region(&self, region: Region) -> Result<Cookie<'_, Self, FetchRegionReply>, ConnectionError>
    {
        fetch_region(self, region)
    }
    fn xfixes_set_gc_clip_region<A>(&self, gc: xproto::Gcontext, region: A, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Region>,
    {
        set_gc_clip_region(self, gc, region, x_origin, y_origin)
    }
    fn xfixes_set_window_shape_region<A>(&self, dest: xproto::Window, dest_kind: shape::SK, x_offset: i16, y_offset: i16, region: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Region>,
    {
        set_window_shape_region(self, dest, dest_kind, x_offset, y_offset, region)
    }
    fn xfixes_set_picture_clip_region<A>(&self, picture: render::Picture, region: A, x_origin: i16, y_origin: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Region>,
    {
        set_picture_clip_region(self, picture, region, x_origin, y_origin)
    }
    fn xfixes_set_cursor_name<'c, 'input>(&'c self, cursor: xproto::Cursor, name: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_cursor_name(self, cursor, name)
    }
    fn xfixes_get_cursor_name(&self, cursor: xproto::Cursor) -> Result<Cookie<'_, Self, GetCursorNameReply>, ConnectionError>
    {
        get_cursor_name(self, cursor)
    }
    fn xfixes_get_cursor_image_and_name(&self) -> Result<Cookie<'_, Self, GetCursorImageAndNameReply>, ConnectionError>
    {
        get_cursor_image_and_name(self)
    }
    fn xfixes_change_cursor(&self, source: xproto::Cursor, destination: xproto::Cursor) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        change_cursor(self, source, destination)
    }
    fn xfixes_change_cursor_by_name<'c, 'input>(&'c self, src: xproto::Cursor, name: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_cursor_by_name(self, src, name)
    }
    fn xfixes_expand_region(&self, source: Region, destination: Region, left: u16, right: u16, top: u16, bottom: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        expand_region(self, source, destination, left, right, top, bottom)
    }
    fn xfixes_hide_cursor(&self, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        hide_cursor(self, window)
    }
    fn xfixes_show_cursor(&self, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        show_cursor(self, window)
    }
    fn xfixes_create_pointer_barrier<'c, 'input, A>(&'c self, barrier: Barrier, window: xproto::Window, x1: u16, y1: u16, x2: u16, y2: u16, directions: A, devices: &'input [u16]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        create_pointer_barrier(self, barrier, window, x1, y1, x2, y2, directions, devices)
    }
    fn xfixes_delete_pointer_barrier(&self, barrier: Barrier) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_pointer_barrier(self, barrier)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}

/// A RAII-like wrapper around a [Region].
///
/// Instances of this struct represent a Region that is freed in `Drop`.
///
/// Any errors during `Drop` are silently ignored. Most likely an error here means that your
/// X11 connection is broken and later requests will also fail.
#[derive(Debug)]
pub struct RegionWrapper<'c, C: RequestConnection>(&'c C, Region);

impl<'c, C: RequestConnection> RegionWrapper<'c, C>
{
    /// Assume ownership of the given resource and destroy it in `Drop`.
    pub fn for_region(conn: &'c C, id: Region) -> Self {
        RegionWrapper(conn, id)
    }

    /// Get the XID of the wrapped resource
    pub fn region(&self) -> Region {
        self.1
    }

    /// Assume ownership of the XID of the wrapped resource
    ///
    /// This function destroys this wrapper without freeing the underlying resource.
    pub fn into_region(self) -> Region {
        let id = self.1;
        std::mem::forget(self);
        id
    }
}

impl<'c, C: X11Connection> RegionWrapper<'c, C>
{

    /// Create a new Region and return a Region wrapper and a cookie.
    ///
    /// This is a thin wrapper around [create_region] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`. This also returns a `VoidCookie` that comes from the call to
    /// [create_region].
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region].
    pub fn create_region_and_get_cookie(conn: &'c C, rectangles: &[xproto::Rectangle]) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError>
    {
        let region = conn.generate_id()?;
        let cookie = create_region(conn, region, rectangles)?;
        Ok((Self::for_region(conn, region), cookie))
    }

    /// Create a new Region and return a Region wrapper
    ///
    /// This is a thin wrapper around [create_region] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`.
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region].
    pub fn create_region(conn: &'c C, rectangles: &[xproto::Rectangle]) -> Result<Self, ReplyOrIdError>
    {
        Ok(Self::create_region_and_get_cookie(conn, rectangles)?.0)
    }

    /// Create a new Region and return a Region wrapper and a cookie.
    ///
    /// This is a thin wrapper around [create_region_from_bitmap] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`. This also returns a `VoidCookie` that comes from the call to
    /// [create_region_from_bitmap].
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region_from_bitmap].
    pub fn create_region_from_bitmap_and_get_cookie(conn: &'c C, bitmap: xproto::Pixmap) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError>
    {
        let region = conn.generate_id()?;
        let cookie = create_region_from_bitmap(conn, region, bitmap)?;
        Ok((Self::for_region(conn, region), cookie))
    }

    /// Create a new Region and return a Region wrapper
    ///
    /// This is a thin wrapper around [create_region_from_bitmap] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`.
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region_from_bitmap].
    pub fn create_region_from_bitmap(conn: &'c C, bitmap: xproto::Pixmap) -> Result<Self, ReplyOrIdError>
    {
        Ok(Self::create_region_from_bitmap_and_get_cookie(conn, bitmap)?.0)
    }

    /// Create a new Region and return a Region wrapper and a cookie.
    ///
    /// This is a thin wrapper around [create_region_from_window] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`. This also returns a `VoidCookie` that comes from the call to
    /// [create_region_from_window].
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region_from_window].
    pub fn create_region_from_window_and_get_cookie(conn: &'c C, window: xproto::Window, kind: shape::SK) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError>
    {
        let region = conn.generate_id()?;
        let cookie = create_region_from_window(conn, region, window, kind)?;
        Ok((Self::for_region(conn, region), cookie))
    }

    /// Create a new Region and return a Region wrapper
    ///
    /// This is a thin wrapper around [create_region_from_window] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`.
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region_from_window].
    pub fn create_region_from_window(conn: &'c C, window: xproto::Window, kind: shape::SK) -> Result<Self, ReplyOrIdError>
    {
        Ok(Self::create_region_from_window_and_get_cookie(conn, window, kind)?.0)
    }

    /// Create a new Region and return a Region wrapper and a cookie.
    ///
    /// This is a thin wrapper around [create_region_from_gc] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`. This also returns a `VoidCookie` that comes from the call to
    /// [create_region_from_gc].
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region_from_gc].
    pub fn create_region_from_gc_and_get_cookie(conn: &'c C, gc: xproto::Gcontext) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError>
    {
        let region = conn.generate_id()?;
        let cookie = create_region_from_gc(conn, region, gc)?;
        Ok((Self::for_region(conn, region), cookie))
    }

    /// Create a new Region and return a Region wrapper
    ///
    /// This is a thin wrapper around [create_region_from_gc] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`.
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region_from_gc].
    pub fn create_region_from_gc(conn: &'c C, gc: xproto::Gcontext) -> Result<Self, ReplyOrIdError>
    {
        Ok(Self::create_region_from_gc_and_get_cookie(conn, gc)?.0)
    }

    /// Create a new Region and return a Region wrapper and a cookie.
    ///
    /// This is a thin wrapper around [create_region_from_picture] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`. This also returns a `VoidCookie` that comes from the call to
    /// [create_region_from_picture].
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region_from_picture].
    pub fn create_region_from_picture_and_get_cookie(conn: &'c C, picture: render::Picture) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError>
    {
        let region = conn.generate_id()?;
        let cookie = create_region_from_picture(conn, region, picture)?;
        Ok((Self::for_region(conn, region), cookie))
    }

    /// Create a new Region and return a Region wrapper
    ///
    /// This is a thin wrapper around [create_region_from_picture] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`.
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [create_region_from_picture].
    pub fn create_region_from_picture(conn: &'c C, picture: render::Picture) -> Result<Self, ReplyOrIdError>
    {
        Ok(Self::create_region_from_picture_and_get_cookie(conn, picture)?.0)
    }

    /// Create a new Region and return a Region wrapper and a cookie.
    ///
    /// This is a thin wrapper around [super::composite::create_region_from_border_clip] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`. This also returns a `VoidCookie` that comes from the call to
    /// [super::composite::create_region_from_border_clip].
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [super::composite::create_region_from_border_clip].
    #[cfg(feature = "composite")]
    pub fn composite_create_region_from_border_clip_and_get_cookie(conn: &'c C, window: xproto::Window) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError>
    {
        let region = conn.generate_id()?;
        let cookie = super::composite::create_region_from_border_clip(conn, region, window)?;
        Ok((Self::for_region(conn, region), cookie))
    }

    /// Create a new Region and return a Region wrapper
    ///
    /// This is a thin wrapper around [super::composite::create_region_from_border_clip] that allocates an id for the Region.
    /// This function returns the resulting `RegionWrapper` that owns the created Region and frees
    /// it in `Drop`.
    ///
    /// Errors can come from the call to [X11Connection::generate_id] or [super::composite::create_region_from_border_clip].
    #[cfg(feature = "composite")]
    pub fn composite_create_region_from_border_clip(conn: &'c C, window: xproto::Window) -> Result<Self, ReplyOrIdError>
    {
        Ok(Self::composite_create_region_from_border_clip_and_get_cookie(conn, window)?.0)
    }
}
#[cfg(feature = "composite")]
#[allow(unused_imports)]
use super::composite;

impl<C: RequestConnection> From<&RegionWrapper<'_, C>> for Region {
    fn from(from: &RegionWrapper<'_, C>) -> Self {
        from.1
    }
}

impl<C: RequestConnection> Drop for RegionWrapper<'_, C> {
    fn drop(&mut self) {
        let _ = destroy_region(self.0, self.1);
    }
}
