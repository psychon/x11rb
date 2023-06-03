// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Render` X11 extension.

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

pub use x11rb_protocol::protocol::render::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn query_version<Conn>(conn: &Conn, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
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
pub async fn query_pict_formats<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryPictFormatsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryPictFormatsRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn query_pict_index_values<Conn>(conn: &Conn, format: Pictformat) -> Result<Cookie<'_, Conn, QueryPictIndexValuesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryPictIndexValuesRequest {
        format,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn create_picture<'c, 'input, Conn>(conn: &'c Conn, pid: Picture, drawable: xproto::Drawable, format: Pictformat, value_list: &'input CreatePictureAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreatePictureRequest {
        pid,
        drawable,
        format,
        value_list: Cow::Borrowed(value_list),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn change_picture<'c, 'input, Conn>(conn: &'c Conn, picture: Picture, value_list: &'input ChangePictureAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangePictureRequest {
        picture,
        value_list: Cow::Borrowed(value_list),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn set_picture_clip_rectangles<'c, 'input, Conn>(conn: &'c Conn, picture: Picture, clip_x_origin: i16, clip_y_origin: i16, rectangles: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPictureClipRectanglesRequest {
        picture,
        clip_x_origin,
        clip_y_origin,
        rectangles: Cow::Borrowed(rectangles),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn free_picture<Conn>(conn: &Conn, picture: Picture) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FreePictureRequest {
        picture,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn composite<Conn, A>(conn: &Conn, op: PictOp, src: Picture, mask: A, dst: Picture, src_x: i16, src_y: i16, mask_x: i16, mask_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Picture> + Send,
{
    let mask: Picture = mask.into();
    let request0 = CompositeRequest {
        op,
        src,
        mask,
        dst,
        src_x,
        src_y,
        mask_x,
        mask_y,
        dst_x,
        dst_y,
        width,
        height,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn trapezoids<'c, 'input, Conn>(conn: &'c Conn, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, traps: &'input [Trapezoid]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TrapezoidsRequest {
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        traps: Cow::Borrowed(traps),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn triangles<'c, 'input, Conn>(conn: &'c Conn, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, triangles: &'input [Triangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TrianglesRequest {
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        triangles: Cow::Borrowed(triangles),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn tri_strip<'c, 'input, Conn>(conn: &'c Conn, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, points: &'input [Pointfix]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TriStripRequest {
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        points: Cow::Borrowed(points),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn tri_fan<'c, 'input, Conn>(conn: &'c Conn, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, points: &'input [Pointfix]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TriFanRequest {
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        points: Cow::Borrowed(points),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_glyph_set<Conn>(conn: &Conn, gsid: Glyphset, format: Pictformat) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateGlyphSetRequest {
        gsid,
        format,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn reference_glyph_set<Conn>(conn: &Conn, gsid: Glyphset, existing: Glyphset) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ReferenceGlyphSetRequest {
        gsid,
        existing,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn free_glyph_set<Conn>(conn: &Conn, glyphset: Glyphset) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FreeGlyphSetRequest {
        glyphset,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn add_glyphs<'c, 'input, Conn>(conn: &'c Conn, glyphset: Glyphset, glyphids: &'input [u32], glyphs: &'input [Glyphinfo], data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AddGlyphsRequest {
        glyphset,
        glyphids: Cow::Borrowed(glyphids),
        glyphs: Cow::Borrowed(glyphs),
        data: Cow::Borrowed(data),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2]), IoSlice::new(&bytes[3]), IoSlice::new(&bytes[4])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn free_glyphs<'c, 'input, Conn>(conn: &'c Conn, glyphset: Glyphset, glyphs: &'input [Glyph]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FreeGlyphsRequest {
        glyphset,
        glyphs: Cow::Borrowed(glyphs),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn composite_glyphs8<'c, 'input, Conn>(conn: &'c Conn, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CompositeGlyphs8Request {
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds: Cow::Borrowed(glyphcmds),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn composite_glyphs16<'c, 'input, Conn>(conn: &'c Conn, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CompositeGlyphs16Request {
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds: Cow::Borrowed(glyphcmds),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn composite_glyphs32<'c, 'input, Conn>(conn: &'c Conn, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CompositeGlyphs32Request {
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds: Cow::Borrowed(glyphcmds),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn fill_rectangles<'c, 'input, Conn>(conn: &'c Conn, op: PictOp, dst: Picture, color: Color, rects: &'input [xproto::Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FillRectanglesRequest {
        op,
        dst,
        color,
        rects: Cow::Borrowed(rects),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_cursor<Conn>(conn: &Conn, cid: xproto::Cursor, source: Picture, x: u16, y: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateCursorRequest {
        cid,
        source,
        x,
        y,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn set_picture_transform<Conn>(conn: &Conn, picture: Picture, transform: Transform) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPictureTransformRequest {
        picture,
        transform,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn query_filters<Conn>(conn: &Conn, drawable: xproto::Drawable) -> Result<Cookie<'_, Conn, QueryFiltersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryFiltersRequest {
        drawable,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_picture_filter<'c, 'input, Conn>(conn: &'c Conn, picture: Picture, filter: &'input [u8], values: &'input [Fixed]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPictureFilterRequest {
        picture,
        filter: Cow::Borrowed(filter),
        values: Cow::Borrowed(values),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2]), IoSlice::new(&bytes[3]), IoSlice::new(&bytes[4])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_anim_cursor<'c, 'input, Conn>(conn: &'c Conn, cid: xproto::Cursor, cursors: &'input [Animcursorelt]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateAnimCursorRequest {
        cid,
        cursors: Cow::Borrowed(cursors),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn add_traps<'c, 'input, Conn>(conn: &'c Conn, picture: Picture, x_off: i16, y_off: i16, traps: &'input [Trap]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AddTrapsRequest {
        picture,
        x_off,
        y_off,
        traps: Cow::Borrowed(traps),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_solid_fill<Conn>(conn: &Conn, picture: Picture, color: Color) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateSolidFillRequest {
        picture,
        color,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_linear_gradient<'c, 'input, Conn>(conn: &'c Conn, picture: Picture, p1: Pointfix, p2: Pointfix, stops: &'input [Fixed], colors: &'input [Color]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateLinearGradientRequest {
        picture,
        p1,
        p2,
        stops: Cow::Borrowed(stops),
        colors: Cow::Borrowed(colors),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2]), IoSlice::new(&bytes[3])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_radial_gradient<'c, 'input, Conn>(conn: &'c Conn, picture: Picture, inner: Pointfix, outer: Pointfix, inner_radius: Fixed, outer_radius: Fixed, stops: &'input [Fixed], colors: &'input [Color]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRadialGradientRequest {
        picture,
        inner,
        outer,
        inner_radius,
        outer_radius,
        stops: Cow::Borrowed(stops),
        colors: Cow::Borrowed(colors),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2]), IoSlice::new(&bytes[3])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_conical_gradient<'c, 'input, Conn>(conn: &'c Conn, picture: Picture, center: Pointfix, angle: Fixed, stops: &'input [Fixed], colors: &'input [Color]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateConicalGradientRequest {
        picture,
        center,
        angle,
        stops: Cow::Borrowed(stops),
        colors: Cow::Borrowed(colors),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2]), IoSlice::new(&bytes[3])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn render_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self, client_major_version, client_minor_version))
    }
    fn render_query_pict_formats(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryPictFormatsReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_pict_formats(self))
    }
    fn render_query_pict_index_values(&self, format: Pictformat) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryPictIndexValuesReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_pict_index_values(self, format))
    }
    fn render_create_picture<'c, 'input, 'future>(&'c self, pid: Picture, drawable: xproto::Drawable, format: Pictformat, value_list: &'input CreatePictureAux) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(create_picture(self, pid, drawable, format, value_list))
    }
    fn render_change_picture<'c, 'input, 'future>(&'c self, picture: Picture, value_list: &'input ChangePictureAux) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(change_picture(self, picture, value_list))
    }
    fn render_set_picture_clip_rectangles<'c, 'input, 'future>(&'c self, picture: Picture, clip_x_origin: i16, clip_y_origin: i16, rectangles: &'input [xproto::Rectangle]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_picture_clip_rectangles(self, picture, clip_x_origin, clip_y_origin, rectangles))
    }
    fn render_free_picture(&self, picture: Picture) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(free_picture(self, picture))
    }
    fn render_composite<A>(&self, op: PictOp, src: Picture, mask: A, dst: Picture, src_x: i16, src_y: i16, mask_x: i16, mask_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    where
        A: Into<Picture> + Send + 'static,
    {
        Box::pin(composite(self, op, src, mask, dst, src_x, src_y, mask_x, mask_y, dst_x, dst_y, width, height))
    }
    fn render_trapezoids<'c, 'input, 'future>(&'c self, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, traps: &'input [Trapezoid]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(trapezoids(self, op, src, dst, mask_format, src_x, src_y, traps))
    }
    fn render_triangles<'c, 'input, 'future>(&'c self, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, triangles: &'input [Triangle]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(self::triangles(self, op, src, dst, mask_format, src_x, src_y, triangles))
    }
    fn render_tri_strip<'c, 'input, 'future>(&'c self, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, points: &'input [Pointfix]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(tri_strip(self, op, src, dst, mask_format, src_x, src_y, points))
    }
    fn render_tri_fan<'c, 'input, 'future>(&'c self, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, src_x: i16, src_y: i16, points: &'input [Pointfix]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(tri_fan(self, op, src, dst, mask_format, src_x, src_y, points))
    }
    fn render_create_glyph_set(&self, gsid: Glyphset, format: Pictformat) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_glyph_set(self, gsid, format))
    }
    fn render_reference_glyph_set(&self, gsid: Glyphset, existing: Glyphset) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(reference_glyph_set(self, gsid, existing))
    }
    fn render_free_glyph_set(&self, glyphset: Glyphset) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(free_glyph_set(self, glyphset))
    }
    fn render_add_glyphs<'c, 'input, 'future>(&'c self, glyphset: Glyphset, glyphids: &'input [u32], glyphs: &'input [Glyphinfo], data: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(add_glyphs(self, glyphset, glyphids, glyphs, data))
    }
    fn render_free_glyphs<'c, 'input, 'future>(&'c self, glyphset: Glyphset, glyphs: &'input [Glyph]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(free_glyphs(self, glyphset, glyphs))
    }
    fn render_composite_glyphs8<'c, 'input, 'future>(&'c self, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(composite_glyphs8(self, op, src, dst, mask_format, glyphset, src_x, src_y, glyphcmds))
    }
    fn render_composite_glyphs16<'c, 'input, 'future>(&'c self, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(composite_glyphs16(self, op, src, dst, mask_format, glyphset, src_x, src_y, glyphcmds))
    }
    fn render_composite_glyphs32<'c, 'input, 'future>(&'c self, op: PictOp, src: Picture, dst: Picture, mask_format: Pictformat, glyphset: Glyphset, src_x: i16, src_y: i16, glyphcmds: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(composite_glyphs32(self, op, src, dst, mask_format, glyphset, src_x, src_y, glyphcmds))
    }
    fn render_fill_rectangles<'c, 'input, 'future>(&'c self, op: PictOp, dst: Picture, color: Color, rects: &'input [xproto::Rectangle]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(fill_rectangles(self, op, dst, color, rects))
    }
    fn render_create_cursor(&self, cid: xproto::Cursor, source: Picture, x: u16, y: u16) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_cursor(self, cid, source, x, y))
    }
    fn render_set_picture_transform(&self, picture: Picture, transform: Transform) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(set_picture_transform(self, picture, transform))
    }
    fn render_query_filters(&self, drawable: xproto::Drawable) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryFiltersReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_filters(self, drawable))
    }
    fn render_set_picture_filter<'c, 'input, 'future>(&'c self, picture: Picture, filter: &'input [u8], values: &'input [Fixed]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_picture_filter(self, picture, filter, values))
    }
    fn render_create_anim_cursor<'c, 'input, 'future>(&'c self, cid: xproto::Cursor, cursors: &'input [Animcursorelt]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(create_anim_cursor(self, cid, cursors))
    }
    fn render_add_traps<'c, 'input, 'future>(&'c self, picture: Picture, x_off: i16, y_off: i16, traps: &'input [Trap]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(add_traps(self, picture, x_off, y_off, traps))
    }
    fn render_create_solid_fill(&self, picture: Picture, color: Color) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_solid_fill(self, picture, color))
    }
    fn render_create_linear_gradient<'c, 'input, 'future>(&'c self, picture: Picture, p1: Pointfix, p2: Pointfix, stops: &'input [Fixed], colors: &'input [Color]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(create_linear_gradient(self, picture, p1, p2, stops, colors))
    }
    fn render_create_radial_gradient<'c, 'input, 'future>(&'c self, picture: Picture, inner: Pointfix, outer: Pointfix, inner_radius: Fixed, outer_radius: Fixed, stops: &'input [Fixed], colors: &'input [Color]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(create_radial_gradient(self, picture, inner, outer, inner_radius, outer_radius, stops, colors))
    }
    fn render_create_conical_gradient<'c, 'input, 'future>(&'c self, picture: Picture, center: Pointfix, angle: Fixed, stops: &'input [Fixed], colors: &'input [Color]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(create_conical_gradient(self, picture, center, angle, stops, colors))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
