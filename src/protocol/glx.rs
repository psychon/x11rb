// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Glx` X11 extension.

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

pub use x11rb_protocol::protocol::glx::*;

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

fn send_render<'c, Conn>(req: RenderRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn render<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RenderRequest {
        context_tag,
        data: Cow::Borrowed(data),
    };
    send_render(request0, conn)
}

fn send_render_large<'c, Conn>(req: RenderLargeRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn render_large<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, request_num: u16, request_total: u16, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RenderLargeRequest {
        context_tag,
        request_num,
        request_total,
        data: Cow::Borrowed(data),
    };
    send_render_large(request0, conn)
}

fn send_create_context<'c, Conn>(req: CreateContextRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_context<Conn>(conn: &Conn, context: Context, visual: xproto::Visualid, screen: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        context,
        visual,
        screen,
        share_list,
        is_direct,
    };
    send_create_context(request0, conn)
}

fn send_destroy_context<'c, Conn>(req: DestroyContextRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn destroy_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyContextRequest {
        context,
    };
    send_destroy_context(request0, conn)
}

fn send_make_current<'c, Conn>(req: MakeCurrentRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, MakeCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn make_current<Conn>(conn: &Conn, drawable: Drawable, context: Context, old_context_tag: ContextTag) -> Result<Cookie<'_, Conn, MakeCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = MakeCurrentRequest {
        drawable,
        context,
        old_context_tag,
    };
    send_make_current(request0, conn)
}

fn send_is_direct<'c, Conn>(req: IsDirectRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, IsDirectReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn is_direct<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, IsDirectReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsDirectRequest {
        context,
    };
    send_is_direct(request0, conn)
}

fn send_query_version<'c, Conn>(req: QueryVersionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major_version,
        minor_version,
    };
    send_query_version(request0, conn)
}

fn send_wait_gl<'c, Conn>(req: WaitGLRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn wait_gl<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = WaitGLRequest {
        context_tag,
    };
    send_wait_gl(request0, conn)
}

fn send_wait_x<'c, Conn>(req: WaitXRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn wait_x<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = WaitXRequest {
        context_tag,
    };
    send_wait_x(request0, conn)
}

fn send_copy_context<'c, Conn>(req: CopyContextRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn copy_context<Conn>(conn: &Conn, src: Context, dest: Context, mask: u32, src_context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CopyContextRequest {
        src,
        dest,
        mask,
        src_context_tag,
    };
    send_copy_context(request0, conn)
}

fn send_swap_buffers<'c, Conn>(req: SwapBuffersRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn swap_buffers<Conn>(conn: &Conn, context_tag: ContextTag, drawable: Drawable) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SwapBuffersRequest {
        context_tag,
        drawable,
    };
    send_swap_buffers(request0, conn)
}

fn send_use_x_font<'c, Conn>(req: UseXFontRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn use_x_font<Conn>(conn: &Conn, context_tag: ContextTag, font: xproto::Font, first: u32, count: u32, list_base: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UseXFontRequest {
        context_tag,
        font,
        first,
        count,
        list_base,
    };
    send_use_x_font(request0, conn)
}

fn send_create_glx_pixmap<'c, Conn>(req: CreateGLXPixmapRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_glx_pixmap<Conn>(conn: &Conn, screen: u32, visual: xproto::Visualid, pixmap: xproto::Pixmap, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateGLXPixmapRequest {
        screen,
        visual,
        pixmap,
        glx_pixmap,
    };
    send_create_glx_pixmap(request0, conn)
}

fn send_get_visual_configs<'c, Conn>(req: GetVisualConfigsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetVisualConfigsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_visual_configs<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetVisualConfigsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetVisualConfigsRequest {
        screen,
    };
    send_get_visual_configs(request0, conn)
}

fn send_destroy_glx_pixmap<'c, Conn>(req: DestroyGLXPixmapRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn destroy_glx_pixmap<Conn>(conn: &Conn, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyGLXPixmapRequest {
        glx_pixmap,
    };
    send_destroy_glx_pixmap(request0, conn)
}

fn send_vendor_private<'c, Conn>(req: VendorPrivateRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn vendor_private<'c, 'input, Conn>(conn: &'c Conn, vendor_code: u32, context_tag: ContextTag, data: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = VendorPrivateRequest {
        vendor_code,
        context_tag,
        data: Cow::Borrowed(data),
    };
    send_vendor_private(request0, conn)
}

fn send_vendor_private_with_reply<'c, Conn>(req: VendorPrivateWithReplyRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, VendorPrivateWithReplyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn vendor_private_with_reply<'c, 'input, Conn>(conn: &'c Conn, vendor_code: u32, context_tag: ContextTag, data: &'input [u8]) -> Result<Cookie<'c, Conn, VendorPrivateWithReplyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = VendorPrivateWithReplyRequest {
        vendor_code,
        context_tag,
        data: Cow::Borrowed(data),
    };
    send_vendor_private_with_reply(request0, conn)
}

fn send_query_extensions_string<'c, Conn>(req: QueryExtensionsStringRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryExtensionsStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_extensions_string<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, QueryExtensionsStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryExtensionsStringRequest {
        screen,
    };
    send_query_extensions_string(request0, conn)
}

fn send_query_server_string<'c, Conn>(req: QueryServerStringRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryServerStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_server_string<Conn>(conn: &Conn, screen: u32, name: u32) -> Result<Cookie<'_, Conn, QueryServerStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryServerStringRequest {
        screen,
        name,
    };
    send_query_server_string(request0, conn)
}

fn send_client_info<'c, Conn>(req: ClientInfoRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn client_info<'c, 'input, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, string: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ClientInfoRequest {
        major_version,
        minor_version,
        string: Cow::Borrowed(string),
    };
    send_client_info(request0, conn)
}

fn send_get_fb_configs<'c, Conn>(req: GetFBConfigsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetFBConfigsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_fb_configs<Conn>(conn: &Conn, screen: u32) -> Result<Cookie<'_, Conn, GetFBConfigsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetFBConfigsRequest {
        screen,
    };
    send_get_fb_configs(request0, conn)
}

fn send_create_pixmap<'c, Conn>(req: CreatePixmapRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_pixmap<'c, 'input, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, pixmap: xproto::Pixmap, glx_pixmap: Pixmap, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreatePixmapRequest {
        screen,
        fbconfig,
        pixmap,
        glx_pixmap,
        attribs: Cow::Borrowed(attribs),
    };
    send_create_pixmap(request0, conn)
}

fn send_destroy_pixmap<'c, Conn>(req: DestroyPixmapRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn destroy_pixmap<Conn>(conn: &Conn, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyPixmapRequest {
        glx_pixmap,
    };
    send_destroy_pixmap(request0, conn)
}

fn send_create_new_context<'c, Conn>(req: CreateNewContextRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_new_context<Conn>(conn: &Conn, context: Context, fbconfig: Fbconfig, screen: u32, render_type: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateNewContextRequest {
        context,
        fbconfig,
        screen,
        render_type,
        share_list,
        is_direct,
    };
    send_create_new_context(request0, conn)
}

fn send_query_context<'c, Conn>(req: QueryContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_context<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, QueryContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryContextRequest {
        context,
    };
    send_query_context(request0, conn)
}

fn send_make_context_current<'c, Conn>(req: MakeContextCurrentRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, MakeContextCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn make_context_current<Conn>(conn: &Conn, old_context_tag: ContextTag, drawable: Drawable, read_drawable: Drawable, context: Context) -> Result<Cookie<'_, Conn, MakeContextCurrentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = MakeContextCurrentRequest {
        old_context_tag,
        drawable,
        read_drawable,
        context,
    };
    send_make_context_current(request0, conn)
}

fn send_create_pbuffer<'c, Conn>(req: CreatePbufferRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_pbuffer<'c, 'input, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, pbuffer: Pbuffer, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreatePbufferRequest {
        screen,
        fbconfig,
        pbuffer,
        attribs: Cow::Borrowed(attribs),
    };
    send_create_pbuffer(request0, conn)
}

fn send_destroy_pbuffer<'c, Conn>(req: DestroyPbufferRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn destroy_pbuffer<Conn>(conn: &Conn, pbuffer: Pbuffer) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyPbufferRequest {
        pbuffer,
    };
    send_destroy_pbuffer(request0, conn)
}

fn send_get_drawable_attributes<'c, Conn>(req: GetDrawableAttributesRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDrawableAttributesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_drawable_attributes<Conn>(conn: &Conn, drawable: Drawable) -> Result<Cookie<'_, Conn, GetDrawableAttributesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDrawableAttributesRequest {
        drawable,
    };
    send_get_drawable_attributes(request0, conn)
}

fn send_change_drawable_attributes<'c, Conn>(req: ChangeDrawableAttributesRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn change_drawable_attributes<'c, 'input, Conn>(conn: &'c Conn, drawable: Drawable, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeDrawableAttributesRequest {
        drawable,
        attribs: Cow::Borrowed(attribs),
    };
    send_change_drawable_attributes(request0, conn)
}

fn send_create_window<'c, Conn>(req: CreateWindowRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_window<'c, 'input, Conn>(conn: &'c Conn, screen: u32, fbconfig: Fbconfig, window: xproto::Window, glx_window: Window, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateWindowRequest {
        screen,
        fbconfig,
        window,
        glx_window,
        attribs: Cow::Borrowed(attribs),
    };
    send_create_window(request0, conn)
}

fn send_delete_window<'c, Conn>(req: DeleteWindowRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn delete_window<Conn>(conn: &Conn, glxwindow: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteWindowRequest {
        glxwindow,
    };
    send_delete_window(request0, conn)
}

fn send_set_client_info_arb<'c, Conn>(req: SetClientInfoARBRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_client_info_arb<'c, 'input, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, gl_versions: &'input [u32], gl_extension_string: &'input [u8], glx_extension_string: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetClientInfoARBRequest {
        major_version,
        minor_version,
        gl_versions: Cow::Borrowed(gl_versions),
        gl_extension_string: Cow::Borrowed(gl_extension_string),
        glx_extension_string: Cow::Borrowed(glx_extension_string),
    };
    send_set_client_info_arb(request0, conn)
}

fn send_create_context_attribs_arb<'c, Conn>(req: CreateContextAttribsARBRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn create_context_attribs_arb<'c, 'input, Conn>(conn: &'c Conn, context: Context, fbconfig: Fbconfig, screen: u32, share_list: Context, is_direct: bool, attribs: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextAttribsARBRequest {
        context,
        fbconfig,
        screen,
        share_list,
        is_direct,
        attribs: Cow::Borrowed(attribs),
    };
    send_create_context_attribs_arb(request0, conn)
}

fn send_set_client_info2_arb<'c, Conn>(req: SetClientInfo2ARBRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_client_info2_arb<'c, 'input, Conn>(conn: &'c Conn, major_version: u32, minor_version: u32, gl_versions: &'input [u32], gl_extension_string: &'input [u8], glx_extension_string: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetClientInfo2ARBRequest {
        major_version,
        minor_version,
        gl_versions: Cow::Borrowed(gl_versions),
        gl_extension_string: Cow::Borrowed(gl_extension_string),
        glx_extension_string: Cow::Borrowed(glx_extension_string),
    };
    send_set_client_info2_arb(request0, conn)
}

fn send_new_list<'c, Conn>(req: NewListRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn new_list<Conn>(conn: &Conn, context_tag: ContextTag, list: u32, mode: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = NewListRequest {
        context_tag,
        list,
        mode,
    };
    send_new_list(request0, conn)
}

fn send_end_list<'c, Conn>(req: EndListRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn end_list<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = EndListRequest {
        context_tag,
    };
    send_end_list(request0, conn)
}

fn send_delete_lists<'c, Conn>(req: DeleteListsRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn delete_lists<Conn>(conn: &Conn, context_tag: ContextTag, list: u32, range: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteListsRequest {
        context_tag,
        list,
        range,
    };
    send_delete_lists(request0, conn)
}

fn send_gen_lists<'c, Conn>(req: GenListsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GenListsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn gen_lists<Conn>(conn: &Conn, context_tag: ContextTag, range: i32) -> Result<Cookie<'_, Conn, GenListsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GenListsRequest {
        context_tag,
        range,
    };
    send_gen_lists(request0, conn)
}

fn send_feedback_buffer<'c, Conn>(req: FeedbackBufferRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn feedback_buffer<Conn>(conn: &Conn, context_tag: ContextTag, size: i32, type_: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FeedbackBufferRequest {
        context_tag,
        size,
        type_,
    };
    send_feedback_buffer(request0, conn)
}

fn send_select_buffer<'c, Conn>(req: SelectBufferRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn select_buffer<Conn>(conn: &Conn, context_tag: ContextTag, size: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectBufferRequest {
        context_tag,
        size,
    };
    send_select_buffer(request0, conn)
}

fn send_render_mode<'c, Conn>(req: RenderModeRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, RenderModeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn render_mode<Conn>(conn: &Conn, context_tag: ContextTag, mode: u32) -> Result<Cookie<'_, Conn, RenderModeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RenderModeRequest {
        context_tag,
        mode,
    };
    send_render_mode(request0, conn)
}

fn send_finish<'c, Conn>(req: FinishRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, FinishReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn finish<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<Cookie<'_, Conn, FinishReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FinishRequest {
        context_tag,
    };
    send_finish(request0, conn)
}

fn send_pixel_storef<'c, Conn>(req: PixelStorefRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn pixel_storef<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32, datum: Float32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PixelStorefRequest {
        context_tag,
        pname,
        datum,
    };
    send_pixel_storef(request0, conn)
}

fn send_pixel_storei<'c, Conn>(req: PixelStoreiRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn pixel_storei<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32, datum: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PixelStoreiRequest {
        context_tag,
        pname,
        datum,
    };
    send_pixel_storei(request0, conn)
}

fn send_read_pixels<'c, Conn>(req: ReadPixelsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, ReadPixelsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn read_pixels<Conn>(conn: &Conn, context_tag: ContextTag, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, swap_bytes: bool, lsb_first: bool) -> Result<Cookie<'_, Conn, ReadPixelsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ReadPixelsRequest {
        context_tag,
        x,
        y,
        width,
        height,
        format,
        type_,
        swap_bytes,
        lsb_first,
    };
    send_read_pixels(request0, conn)
}

fn send_get_booleanv<'c, Conn>(req: GetBooleanvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetBooleanvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_booleanv<Conn>(conn: &Conn, context_tag: ContextTag, pname: i32) -> Result<Cookie<'_, Conn, GetBooleanvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetBooleanvRequest {
        context_tag,
        pname,
    };
    send_get_booleanv(request0, conn)
}

fn send_get_clip_plane<'c, Conn>(req: GetClipPlaneRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetClipPlaneReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_clip_plane<Conn>(conn: &Conn, context_tag: ContextTag, plane: i32) -> Result<Cookie<'_, Conn, GetClipPlaneReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClipPlaneRequest {
        context_tag,
        plane,
    };
    send_get_clip_plane(request0, conn)
}

fn send_get_doublev<'c, Conn>(req: GetDoublevRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDoublevReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_doublev<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetDoublevReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDoublevRequest {
        context_tag,
        pname,
    };
    send_get_doublev(request0, conn)
}

fn send_get_error<'c, Conn>(req: GetErrorRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetErrorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_error<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<Cookie<'_, Conn, GetErrorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetErrorRequest {
        context_tag,
    };
    send_get_error(request0, conn)
}

fn send_get_floatv<'c, Conn>(req: GetFloatvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetFloatvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_floatv<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetFloatvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetFloatvRequest {
        context_tag,
        pname,
    };
    send_get_floatv(request0, conn)
}

fn send_get_integerv<'c, Conn>(req: GetIntegervRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetIntegervReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_integerv<Conn>(conn: &Conn, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Conn, GetIntegervReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetIntegervRequest {
        context_tag,
        pname,
    };
    send_get_integerv(request0, conn)
}

fn send_get_lightfv<'c, Conn>(req: GetLightfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetLightfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_lightfv<Conn>(conn: &Conn, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Conn, GetLightfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetLightfvRequest {
        context_tag,
        light,
        pname,
    };
    send_get_lightfv(request0, conn)
}

fn send_get_lightiv<'c, Conn>(req: GetLightivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetLightivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_lightiv<Conn>(conn: &Conn, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Conn, GetLightivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetLightivRequest {
        context_tag,
        light,
        pname,
    };
    send_get_lightiv(request0, conn)
}

fn send_get_mapdv<'c, Conn>(req: GetMapdvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMapdvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_mapdv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapdvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMapdvRequest {
        context_tag,
        target,
        query,
    };
    send_get_mapdv(request0, conn)
}

fn send_get_mapfv<'c, Conn>(req: GetMapfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMapfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_mapfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMapfvRequest {
        context_tag,
        target,
        query,
    };
    send_get_mapfv(request0, conn)
}

fn send_get_mapiv<'c, Conn>(req: GetMapivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMapivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_mapiv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Conn, GetMapivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMapivRequest {
        context_tag,
        target,
        query,
    };
    send_get_mapiv(request0, conn)
}

fn send_get_materialfv<'c, Conn>(req: GetMaterialfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMaterialfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_materialfv<Conn>(conn: &Conn, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMaterialfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMaterialfvRequest {
        context_tag,
        face,
        pname,
    };
    send_get_materialfv(request0, conn)
}

fn send_get_materialiv<'c, Conn>(req: GetMaterialivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMaterialivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_materialiv<Conn>(conn: &Conn, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMaterialivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMaterialivRequest {
        context_tag,
        face,
        pname,
    };
    send_get_materialiv(request0, conn)
}

fn send_get_pixel_mapfv<'c, Conn>(req: GetPixelMapfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPixelMapfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_pixel_mapfv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPixelMapfvRequest {
        context_tag,
        map,
    };
    send_get_pixel_mapfv(request0, conn)
}

fn send_get_pixel_mapuiv<'c, Conn>(req: GetPixelMapuivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPixelMapuivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_pixel_mapuiv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapuivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPixelMapuivRequest {
        context_tag,
        map,
    };
    send_get_pixel_mapuiv(request0, conn)
}

fn send_get_pixel_mapusv<'c, Conn>(req: GetPixelMapusvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPixelMapusvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_pixel_mapusv<Conn>(conn: &Conn, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Conn, GetPixelMapusvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPixelMapusvRequest {
        context_tag,
        map,
    };
    send_get_pixel_mapusv(request0, conn)
}

fn send_get_polygon_stipple<'c, Conn>(req: GetPolygonStippleRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPolygonStippleReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_polygon_stipple<Conn>(conn: &Conn, context_tag: ContextTag, lsb_first: bool) -> Result<Cookie<'_, Conn, GetPolygonStippleReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPolygonStippleRequest {
        context_tag,
        lsb_first,
    };
    send_get_polygon_stipple(request0, conn)
}

fn send_get_string<'c, Conn>(req: GetStringRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_string<Conn>(conn: &Conn, context_tag: ContextTag, name: u32) -> Result<Cookie<'_, Conn, GetStringReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetStringRequest {
        context_tag,
        name,
    };
    send_get_string(request0, conn)
}

fn send_get_tex_envfv<'c, Conn>(req: GetTexEnvfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexEnvfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_envfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexEnvfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexEnvfvRequest {
        context_tag,
        target,
        pname,
    };
    send_get_tex_envfv(request0, conn)
}

fn send_get_tex_enviv<'c, Conn>(req: GetTexEnvivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexEnvivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_enviv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexEnvivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexEnvivRequest {
        context_tag,
        target,
        pname,
    };
    send_get_tex_enviv(request0, conn)
}

fn send_get_tex_gendv<'c, Conn>(req: GetTexGendvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexGendvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_gendv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGendvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexGendvRequest {
        context_tag,
        coord,
        pname,
    };
    send_get_tex_gendv(request0, conn)
}

fn send_get_tex_genfv<'c, Conn>(req: GetTexGenfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexGenfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_genfv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGenfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexGenfvRequest {
        context_tag,
        coord,
        pname,
    };
    send_get_tex_genfv(request0, conn)
}

fn send_get_tex_geniv<'c, Conn>(req: GetTexGenivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexGenivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_geniv<Conn>(conn: &Conn, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexGenivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexGenivRequest {
        context_tag,
        coord,
        pname,
    };
    send_get_tex_geniv(request0, conn)
}

fn send_get_tex_image<'c, Conn>(req: GetTexImageRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_image<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetTexImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexImageRequest {
        context_tag,
        target,
        level,
        format,
        type_,
        swap_bytes,
    };
    send_get_tex_image(request0, conn)
}

fn send_get_tex_parameterfv<'c, Conn>(req: GetTexParameterfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    send_get_tex_parameterfv(request0, conn)
}

fn send_get_tex_parameteriv<'c, Conn>(req: GetTexParameterivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetTexParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexParameterivRequest {
        context_tag,
        target,
        pname,
    };
    send_get_tex_parameteriv(request0, conn)
}

fn send_get_tex_level_parameterfv<'c, Conn>(req: GetTexLevelParameterfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexLevelParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_level_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Conn, GetTexLevelParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexLevelParameterfvRequest {
        context_tag,
        target,
        level,
        pname,
    };
    send_get_tex_level_parameterfv(request0, conn)
}

fn send_get_tex_level_parameteriv<'c, Conn>(req: GetTexLevelParameterivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTexLevelParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_tex_level_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Conn, GetTexLevelParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTexLevelParameterivRequest {
        context_tag,
        target,
        level,
        pname,
    };
    send_get_tex_level_parameteriv(request0, conn)
}

fn send_is_enabled<'c, Conn>(req: IsEnabledRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, IsEnabledReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn is_enabled<Conn>(conn: &Conn, context_tag: ContextTag, capability: u32) -> Result<Cookie<'_, Conn, IsEnabledReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsEnabledRequest {
        context_tag,
        capability,
    };
    send_is_enabled(request0, conn)
}

fn send_is_list<'c, Conn>(req: IsListRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, IsListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn is_list<Conn>(conn: &Conn, context_tag: ContextTag, list: u32) -> Result<Cookie<'_, Conn, IsListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsListRequest {
        context_tag,
        list,
    };
    send_is_list(request0, conn)
}

fn send_flush<'c, Conn>(req: FlushRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn flush<Conn>(conn: &Conn, context_tag: ContextTag) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FlushRequest {
        context_tag,
    };
    send_flush(request0, conn)
}

fn send_are_textures_resident<'c, Conn>(req: AreTexturesResidentRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, AreTexturesResidentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn are_textures_resident<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, textures: &'input [u32]) -> Result<Cookie<'c, Conn, AreTexturesResidentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AreTexturesResidentRequest {
        context_tag,
        textures: Cow::Borrowed(textures),
    };
    send_are_textures_resident(request0, conn)
}

fn send_delete_textures<'c, Conn>(req: DeleteTexturesRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn delete_textures<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, textures: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteTexturesRequest {
        context_tag,
        textures: Cow::Borrowed(textures),
    };
    send_delete_textures(request0, conn)
}

fn send_gen_textures<'c, Conn>(req: GenTexturesRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GenTexturesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn gen_textures<Conn>(conn: &Conn, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Conn, GenTexturesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GenTexturesRequest {
        context_tag,
        n,
    };
    send_gen_textures(request0, conn)
}

fn send_is_texture<'c, Conn>(req: IsTextureRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, IsTextureReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn is_texture<Conn>(conn: &Conn, context_tag: ContextTag, texture: u32) -> Result<Cookie<'_, Conn, IsTextureReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsTextureRequest {
        context_tag,
        texture,
    };
    send_is_texture(request0, conn)
}

fn send_get_color_table<'c, Conn>(req: GetColorTableRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetColorTableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_color_table<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetColorTableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetColorTableRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
    };
    send_get_color_table(request0, conn)
}

fn send_get_color_table_parameterfv<'c, Conn>(req: GetColorTableParameterfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetColorTableParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_color_table_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetColorTableParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetColorTableParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    send_get_color_table_parameterfv(request0, conn)
}

fn send_get_color_table_parameteriv<'c, Conn>(req: GetColorTableParameterivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetColorTableParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_color_table_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetColorTableParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetColorTableParameterivRequest {
        context_tag,
        target,
        pname,
    };
    send_get_color_table_parameteriv(request0, conn)
}

fn send_get_convolution_filter<'c, Conn>(req: GetConvolutionFilterRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetConvolutionFilterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_convolution_filter<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetConvolutionFilterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetConvolutionFilterRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
    };
    send_get_convolution_filter(request0, conn)
}

fn send_get_convolution_parameterfv<'c, Conn>(req: GetConvolutionParameterfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetConvolutionParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_convolution_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetConvolutionParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetConvolutionParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    send_get_convolution_parameterfv(request0, conn)
}

fn send_get_convolution_parameteriv<'c, Conn>(req: GetConvolutionParameterivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetConvolutionParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_convolution_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetConvolutionParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetConvolutionParameterivRequest {
        context_tag,
        target,
        pname,
    };
    send_get_convolution_parameteriv(request0, conn)
}

fn send_get_separable_filter<'c, Conn>(req: GetSeparableFilterRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetSeparableFilterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_separable_filter<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Conn, GetSeparableFilterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSeparableFilterRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
    };
    send_get_separable_filter(request0, conn)
}

fn send_get_histogram<'c, Conn>(req: GetHistogramRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetHistogramReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_histogram<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Conn, GetHistogramReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetHistogramRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
        reset,
    };
    send_get_histogram(request0, conn)
}

fn send_get_histogram_parameterfv<'c, Conn>(req: GetHistogramParameterfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetHistogramParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_histogram_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetHistogramParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetHistogramParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    send_get_histogram_parameterfv(request0, conn)
}

fn send_get_histogram_parameteriv<'c, Conn>(req: GetHistogramParameterivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetHistogramParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_histogram_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetHistogramParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetHistogramParameterivRequest {
        context_tag,
        target,
        pname,
    };
    send_get_histogram_parameteriv(request0, conn)
}

fn send_get_minmax<'c, Conn>(req: GetMinmaxRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMinmaxReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_minmax<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Conn, GetMinmaxReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMinmaxRequest {
        context_tag,
        target,
        format,
        type_,
        swap_bytes,
        reset,
    };
    send_get_minmax(request0, conn)
}

fn send_get_minmax_parameterfv<'c, Conn>(req: GetMinmaxParameterfvRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMinmaxParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_minmax_parameterfv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMinmaxParameterfvReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMinmaxParameterfvRequest {
        context_tag,
        target,
        pname,
    };
    send_get_minmax_parameterfv(request0, conn)
}

fn send_get_minmax_parameteriv<'c, Conn>(req: GetMinmaxParameterivRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMinmaxParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_minmax_parameteriv<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetMinmaxParameterivReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMinmaxParameterivRequest {
        context_tag,
        target,
        pname,
    };
    send_get_minmax_parameteriv(request0, conn)
}

fn send_get_compressed_tex_image_arb<'c, Conn>(req: GetCompressedTexImageARBRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetCompressedTexImageARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_compressed_tex_image_arb<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, level: i32) -> Result<Cookie<'_, Conn, GetCompressedTexImageARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetCompressedTexImageARBRequest {
        context_tag,
        target,
        level,
    };
    send_get_compressed_tex_image_arb(request0, conn)
}

fn send_delete_queries_arb<'c, Conn>(req: DeleteQueriesARBRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn delete_queries_arb<'c, 'input, Conn>(conn: &'c Conn, context_tag: ContextTag, ids: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteQueriesARBRequest {
        context_tag,
        ids: Cow::Borrowed(ids),
    };
    send_delete_queries_arb(request0, conn)
}

fn send_gen_queries_arb<'c, Conn>(req: GenQueriesARBRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GenQueriesARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn gen_queries_arb<Conn>(conn: &Conn, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Conn, GenQueriesARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GenQueriesARBRequest {
        context_tag,
        n,
    };
    send_gen_queries_arb(request0, conn)
}

fn send_is_query_arb<'c, Conn>(req: IsQueryARBRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, IsQueryARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn is_query_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32) -> Result<Cookie<'_, Conn, IsQueryARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsQueryARBRequest {
        context_tag,
        id,
    };
    send_is_query_arb(request0, conn)
}

fn send_get_queryiv_arb<'c, Conn>(req: GetQueryivARBRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetQueryivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_queryiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetQueryivARBRequest {
        context_tag,
        target,
        pname,
    };
    send_get_queryiv_arb(request0, conn)
}

fn send_get_query_objectiv_arb<'c, Conn>(req: GetQueryObjectivARBRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetQueryObjectivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_query_objectiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryObjectivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetQueryObjectivARBRequest {
        context_tag,
        id,
        pname,
    };
    send_get_query_objectiv_arb(request0, conn)
}

fn send_get_query_objectuiv_arb<'c, Conn>(req: GetQueryObjectuivARBRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetQueryObjectuivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_query_objectuiv_arb<Conn>(conn: &Conn, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Conn, GetQueryObjectuivARBReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetQueryObjectuivARBRequest {
        context_tag,
        id,
        pname,
    };
    send_get_query_objectuiv_arb(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn glx_render<'c, 'input>(&'c self, context_tag: ContextTag, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        render(self, context_tag, data)
    }
    fn glx_render_large<'c, 'input>(&'c self, context_tag: ContextTag, request_num: u16, request_total: u16, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        render_large(self, context_tag, request_num, request_total, data)
    }
    fn glx_create_context(&self, context: Context, visual: xproto::Visualid, screen: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_context(self, context, visual, screen, share_list, is_direct)
    }
    fn glx_destroy_context(&self, context: Context) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_context(self, context)
    }
    fn glx_make_current(&self, drawable: Drawable, context: Context, old_context_tag: ContextTag) -> Result<Cookie<'_, Self, MakeCurrentReply>, ConnectionError>
    {
        make_current(self, drawable, context, old_context_tag)
    }
    fn glx_is_direct(&self, context: Context) -> Result<Cookie<'_, Self, IsDirectReply>, ConnectionError>
    {
        is_direct(self, context)
    }
    fn glx_query_version(&self, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }
    fn glx_wait_gl(&self, context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        wait_gl(self, context_tag)
    }
    fn glx_wait_x(&self, context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        wait_x(self, context_tag)
    }
    fn glx_copy_context(&self, src: Context, dest: Context, mask: u32, src_context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        copy_context(self, src, dest, mask, src_context_tag)
    }
    fn glx_swap_buffers(&self, context_tag: ContextTag, drawable: Drawable) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        swap_buffers(self, context_tag, drawable)
    }
    fn glx_use_x_font(&self, context_tag: ContextTag, font: xproto::Font, first: u32, count: u32, list_base: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        use_x_font(self, context_tag, font, first, count, list_base)
    }
    fn glx_create_glx_pixmap(&self, screen: u32, visual: xproto::Visualid, pixmap: xproto::Pixmap, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_glx_pixmap(self, screen, visual, pixmap, glx_pixmap)
    }
    fn glx_get_visual_configs(&self, screen: u32) -> Result<Cookie<'_, Self, GetVisualConfigsReply>, ConnectionError>
    {
        get_visual_configs(self, screen)
    }
    fn glx_destroy_glx_pixmap(&self, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_glx_pixmap(self, glx_pixmap)
    }
    fn glx_vendor_private<'c, 'input>(&'c self, vendor_code: u32, context_tag: ContextTag, data: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        vendor_private(self, vendor_code, context_tag, data)
    }
    fn glx_vendor_private_with_reply<'c, 'input>(&'c self, vendor_code: u32, context_tag: ContextTag, data: &'input [u8]) -> Result<Cookie<'c, Self, VendorPrivateWithReplyReply>, ConnectionError>
    {
        vendor_private_with_reply(self, vendor_code, context_tag, data)
    }
    fn glx_query_extensions_string(&self, screen: u32) -> Result<Cookie<'_, Self, QueryExtensionsStringReply>, ConnectionError>
    {
        query_extensions_string(self, screen)
    }
    fn glx_query_server_string(&self, screen: u32, name: u32) -> Result<Cookie<'_, Self, QueryServerStringReply>, ConnectionError>
    {
        query_server_string(self, screen, name)
    }
    fn glx_client_info<'c, 'input>(&'c self, major_version: u32, minor_version: u32, string: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        client_info(self, major_version, minor_version, string)
    }
    fn glx_get_fb_configs(&self, screen: u32) -> Result<Cookie<'_, Self, GetFBConfigsReply>, ConnectionError>
    {
        get_fb_configs(self, screen)
    }
    fn glx_create_pixmap<'c, 'input>(&'c self, screen: u32, fbconfig: Fbconfig, pixmap: xproto::Pixmap, glx_pixmap: Pixmap, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_pixmap(self, screen, fbconfig, pixmap, glx_pixmap, attribs)
    }
    fn glx_destroy_pixmap(&self, glx_pixmap: Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_pixmap(self, glx_pixmap)
    }
    fn glx_create_new_context(&self, context: Context, fbconfig: Fbconfig, screen: u32, render_type: u32, share_list: Context, is_direct: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_new_context(self, context, fbconfig, screen, render_type, share_list, is_direct)
    }
    fn glx_query_context(&self, context: Context) -> Result<Cookie<'_, Self, QueryContextReply>, ConnectionError>
    {
        query_context(self, context)
    }
    fn glx_make_context_current(&self, old_context_tag: ContextTag, drawable: Drawable, read_drawable: Drawable, context: Context) -> Result<Cookie<'_, Self, MakeContextCurrentReply>, ConnectionError>
    {
        make_context_current(self, old_context_tag, drawable, read_drawable, context)
    }
    fn glx_create_pbuffer<'c, 'input>(&'c self, screen: u32, fbconfig: Fbconfig, pbuffer: Pbuffer, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_pbuffer(self, screen, fbconfig, pbuffer, attribs)
    }
    fn glx_destroy_pbuffer(&self, pbuffer: Pbuffer) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_pbuffer(self, pbuffer)
    }
    fn glx_get_drawable_attributes(&self, drawable: Drawable) -> Result<Cookie<'_, Self, GetDrawableAttributesReply>, ConnectionError>
    {
        get_drawable_attributes(self, drawable)
    }
    fn glx_change_drawable_attributes<'c, 'input>(&'c self, drawable: Drawable, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_drawable_attributes(self, drawable, attribs)
    }
    fn glx_create_window<'c, 'input>(&'c self, screen: u32, fbconfig: Fbconfig, window: xproto::Window, glx_window: Window, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_window(self, screen, fbconfig, window, glx_window, attribs)
    }
    fn glx_delete_window(&self, glxwindow: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_window(self, glxwindow)
    }
    fn glx_set_client_info_arb<'c, 'input>(&'c self, major_version: u32, minor_version: u32, gl_versions: &'input [u32], gl_extension_string: &'input [u8], glx_extension_string: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_client_info_arb(self, major_version, minor_version, gl_versions, gl_extension_string, glx_extension_string)
    }
    fn glx_create_context_attribs_arb<'c, 'input>(&'c self, context: Context, fbconfig: Fbconfig, screen: u32, share_list: Context, is_direct: bool, attribs: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_context_attribs_arb(self, context, fbconfig, screen, share_list, is_direct, attribs)
    }
    fn glx_set_client_info2_arb<'c, 'input>(&'c self, major_version: u32, minor_version: u32, gl_versions: &'input [u32], gl_extension_string: &'input [u8], glx_extension_string: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_client_info2_arb(self, major_version, minor_version, gl_versions, gl_extension_string, glx_extension_string)
    }
    fn glx_new_list(&self, context_tag: ContextTag, list: u32, mode: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        new_list(self, context_tag, list, mode)
    }
    fn glx_end_list(&self, context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        end_list(self, context_tag)
    }
    fn glx_delete_lists(&self, context_tag: ContextTag, list: u32, range: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_lists(self, context_tag, list, range)
    }
    fn glx_gen_lists(&self, context_tag: ContextTag, range: i32) -> Result<Cookie<'_, Self, GenListsReply>, ConnectionError>
    {
        gen_lists(self, context_tag, range)
    }
    fn glx_feedback_buffer(&self, context_tag: ContextTag, size: i32, type_: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        feedback_buffer(self, context_tag, size, type_)
    }
    fn glx_select_buffer(&self, context_tag: ContextTag, size: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        select_buffer(self, context_tag, size)
    }
    fn glx_render_mode(&self, context_tag: ContextTag, mode: u32) -> Result<Cookie<'_, Self, RenderModeReply>, ConnectionError>
    {
        render_mode(self, context_tag, mode)
    }
    fn glx_finish(&self, context_tag: ContextTag) -> Result<Cookie<'_, Self, FinishReply>, ConnectionError>
    {
        finish(self, context_tag)
    }
    fn glx_pixel_storef(&self, context_tag: ContextTag, pname: u32, datum: Float32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        pixel_storef(self, context_tag, pname, datum)
    }
    fn glx_pixel_storei(&self, context_tag: ContextTag, pname: u32, datum: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        pixel_storei(self, context_tag, pname, datum)
    }
    fn glx_read_pixels(&self, context_tag: ContextTag, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, swap_bytes: bool, lsb_first: bool) -> Result<Cookie<'_, Self, ReadPixelsReply>, ConnectionError>
    {
        read_pixels(self, context_tag, x, y, width, height, format, type_, swap_bytes, lsb_first)
    }
    fn glx_get_booleanv(&self, context_tag: ContextTag, pname: i32) -> Result<Cookie<'_, Self, GetBooleanvReply>, ConnectionError>
    {
        get_booleanv(self, context_tag, pname)
    }
    fn glx_get_clip_plane(&self, context_tag: ContextTag, plane: i32) -> Result<Cookie<'_, Self, GetClipPlaneReply>, ConnectionError>
    {
        get_clip_plane(self, context_tag, plane)
    }
    fn glx_get_doublev(&self, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Self, GetDoublevReply>, ConnectionError>
    {
        get_doublev(self, context_tag, pname)
    }
    fn glx_get_error(&self, context_tag: ContextTag) -> Result<Cookie<'_, Self, GetErrorReply>, ConnectionError>
    {
        get_error(self, context_tag)
    }
    fn glx_get_floatv(&self, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Self, GetFloatvReply>, ConnectionError>
    {
        get_floatv(self, context_tag, pname)
    }
    fn glx_get_integerv(&self, context_tag: ContextTag, pname: u32) -> Result<Cookie<'_, Self, GetIntegervReply>, ConnectionError>
    {
        get_integerv(self, context_tag, pname)
    }
    fn glx_get_lightfv(&self, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Self, GetLightfvReply>, ConnectionError>
    {
        get_lightfv(self, context_tag, light, pname)
    }
    fn glx_get_lightiv(&self, context_tag: ContextTag, light: u32, pname: u32) -> Result<Cookie<'_, Self, GetLightivReply>, ConnectionError>
    {
        get_lightiv(self, context_tag, light, pname)
    }
    fn glx_get_mapdv(&self, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Self, GetMapdvReply>, ConnectionError>
    {
        get_mapdv(self, context_tag, target, query)
    }
    fn glx_get_mapfv(&self, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Self, GetMapfvReply>, ConnectionError>
    {
        get_mapfv(self, context_tag, target, query)
    }
    fn glx_get_mapiv(&self, context_tag: ContextTag, target: u32, query: u32) -> Result<Cookie<'_, Self, GetMapivReply>, ConnectionError>
    {
        get_mapiv(self, context_tag, target, query)
    }
    fn glx_get_materialfv(&self, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Self, GetMaterialfvReply>, ConnectionError>
    {
        get_materialfv(self, context_tag, face, pname)
    }
    fn glx_get_materialiv(&self, context_tag: ContextTag, face: u32, pname: u32) -> Result<Cookie<'_, Self, GetMaterialivReply>, ConnectionError>
    {
        get_materialiv(self, context_tag, face, pname)
    }
    fn glx_get_pixel_mapfv(&self, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Self, GetPixelMapfvReply>, ConnectionError>
    {
        get_pixel_mapfv(self, context_tag, map)
    }
    fn glx_get_pixel_mapuiv(&self, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Self, GetPixelMapuivReply>, ConnectionError>
    {
        get_pixel_mapuiv(self, context_tag, map)
    }
    fn glx_get_pixel_mapusv(&self, context_tag: ContextTag, map: u32) -> Result<Cookie<'_, Self, GetPixelMapusvReply>, ConnectionError>
    {
        get_pixel_mapusv(self, context_tag, map)
    }
    fn glx_get_polygon_stipple(&self, context_tag: ContextTag, lsb_first: bool) -> Result<Cookie<'_, Self, GetPolygonStippleReply>, ConnectionError>
    {
        get_polygon_stipple(self, context_tag, lsb_first)
    }
    fn glx_get_string(&self, context_tag: ContextTag, name: u32) -> Result<Cookie<'_, Self, GetStringReply>, ConnectionError>
    {
        get_string(self, context_tag, name)
    }
    fn glx_get_tex_envfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexEnvfvReply>, ConnectionError>
    {
        get_tex_envfv(self, context_tag, target, pname)
    }
    fn glx_get_tex_enviv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexEnvivReply>, ConnectionError>
    {
        get_tex_enviv(self, context_tag, target, pname)
    }
    fn glx_get_tex_gendv(&self, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexGendvReply>, ConnectionError>
    {
        get_tex_gendv(self, context_tag, coord, pname)
    }
    fn glx_get_tex_genfv(&self, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexGenfvReply>, ConnectionError>
    {
        get_tex_genfv(self, context_tag, coord, pname)
    }
    fn glx_get_tex_geniv(&self, context_tag: ContextTag, coord: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexGenivReply>, ConnectionError>
    {
        get_tex_geniv(self, context_tag, coord, pname)
    }
    fn glx_get_tex_image(&self, context_tag: ContextTag, target: u32, level: i32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Self, GetTexImageReply>, ConnectionError>
    {
        get_tex_image(self, context_tag, target, level, format, type_, swap_bytes)
    }
    fn glx_get_tex_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexParameterfvReply>, ConnectionError>
    {
        get_tex_parameterfv(self, context_tag, target, pname)
    }
    fn glx_get_tex_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetTexParameterivReply>, ConnectionError>
    {
        get_tex_parameteriv(self, context_tag, target, pname)
    }
    fn glx_get_tex_level_parameterfv(&self, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Self, GetTexLevelParameterfvReply>, ConnectionError>
    {
        get_tex_level_parameterfv(self, context_tag, target, level, pname)
    }
    fn glx_get_tex_level_parameteriv(&self, context_tag: ContextTag, target: u32, level: i32, pname: u32) -> Result<Cookie<'_, Self, GetTexLevelParameterivReply>, ConnectionError>
    {
        get_tex_level_parameteriv(self, context_tag, target, level, pname)
    }
    fn glx_is_enabled(&self, context_tag: ContextTag, capability: u32) -> Result<Cookie<'_, Self, IsEnabledReply>, ConnectionError>
    {
        is_enabled(self, context_tag, capability)
    }
    fn glx_is_list(&self, context_tag: ContextTag, list: u32) -> Result<Cookie<'_, Self, IsListReply>, ConnectionError>
    {
        is_list(self, context_tag, list)
    }
    fn glx_flush(&self, context_tag: ContextTag) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        flush(self, context_tag)
    }
    fn glx_are_textures_resident<'c, 'input>(&'c self, context_tag: ContextTag, textures: &'input [u32]) -> Result<Cookie<'c, Self, AreTexturesResidentReply>, ConnectionError>
    {
        are_textures_resident(self, context_tag, textures)
    }
    fn glx_delete_textures<'c, 'input>(&'c self, context_tag: ContextTag, textures: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        delete_textures(self, context_tag, textures)
    }
    fn glx_gen_textures(&self, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Self, GenTexturesReply>, ConnectionError>
    {
        gen_textures(self, context_tag, n)
    }
    fn glx_is_texture(&self, context_tag: ContextTag, texture: u32) -> Result<Cookie<'_, Self, IsTextureReply>, ConnectionError>
    {
        is_texture(self, context_tag, texture)
    }
    fn glx_get_color_table(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Self, GetColorTableReply>, ConnectionError>
    {
        get_color_table(self, context_tag, target, format, type_, swap_bytes)
    }
    fn glx_get_color_table_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetColorTableParameterfvReply>, ConnectionError>
    {
        get_color_table_parameterfv(self, context_tag, target, pname)
    }
    fn glx_get_color_table_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetColorTableParameterivReply>, ConnectionError>
    {
        get_color_table_parameteriv(self, context_tag, target, pname)
    }
    fn glx_get_convolution_filter(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Self, GetConvolutionFilterReply>, ConnectionError>
    {
        get_convolution_filter(self, context_tag, target, format, type_, swap_bytes)
    }
    fn glx_get_convolution_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetConvolutionParameterfvReply>, ConnectionError>
    {
        get_convolution_parameterfv(self, context_tag, target, pname)
    }
    fn glx_get_convolution_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetConvolutionParameterivReply>, ConnectionError>
    {
        get_convolution_parameteriv(self, context_tag, target, pname)
    }
    fn glx_get_separable_filter(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool) -> Result<Cookie<'_, Self, GetSeparableFilterReply>, ConnectionError>
    {
        get_separable_filter(self, context_tag, target, format, type_, swap_bytes)
    }
    fn glx_get_histogram(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Self, GetHistogramReply>, ConnectionError>
    {
        get_histogram(self, context_tag, target, format, type_, swap_bytes, reset)
    }
    fn glx_get_histogram_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetHistogramParameterfvReply>, ConnectionError>
    {
        get_histogram_parameterfv(self, context_tag, target, pname)
    }
    fn glx_get_histogram_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetHistogramParameterivReply>, ConnectionError>
    {
        get_histogram_parameteriv(self, context_tag, target, pname)
    }
    fn glx_get_minmax(&self, context_tag: ContextTag, target: u32, format: u32, type_: u32, swap_bytes: bool, reset: bool) -> Result<Cookie<'_, Self, GetMinmaxReply>, ConnectionError>
    {
        get_minmax(self, context_tag, target, format, type_, swap_bytes, reset)
    }
    fn glx_get_minmax_parameterfv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetMinmaxParameterfvReply>, ConnectionError>
    {
        get_minmax_parameterfv(self, context_tag, target, pname)
    }
    fn glx_get_minmax_parameteriv(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetMinmaxParameterivReply>, ConnectionError>
    {
        get_minmax_parameteriv(self, context_tag, target, pname)
    }
    fn glx_get_compressed_tex_image_arb(&self, context_tag: ContextTag, target: u32, level: i32) -> Result<Cookie<'_, Self, GetCompressedTexImageARBReply>, ConnectionError>
    {
        get_compressed_tex_image_arb(self, context_tag, target, level)
    }
    fn glx_delete_queries_arb<'c, 'input>(&'c self, context_tag: ContextTag, ids: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        delete_queries_arb(self, context_tag, ids)
    }
    fn glx_gen_queries_arb(&self, context_tag: ContextTag, n: i32) -> Result<Cookie<'_, Self, GenQueriesARBReply>, ConnectionError>
    {
        gen_queries_arb(self, context_tag, n)
    }
    fn glx_is_query_arb(&self, context_tag: ContextTag, id: u32) -> Result<Cookie<'_, Self, IsQueryARBReply>, ConnectionError>
    {
        is_query_arb(self, context_tag, id)
    }
    fn glx_get_queryiv_arb(&self, context_tag: ContextTag, target: u32, pname: u32) -> Result<Cookie<'_, Self, GetQueryivARBReply>, ConnectionError>
    {
        get_queryiv_arb(self, context_tag, target, pname)
    }
    fn glx_get_query_objectiv_arb(&self, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Self, GetQueryObjectivARBReply>, ConnectionError>
    {
        get_query_objectiv_arb(self, context_tag, id, pname)
    }
    fn glx_get_query_objectuiv_arb(&self, context_tag: ContextTag, id: u32, pname: u32) -> Result<Cookie<'_, Self, GetQueryObjectuivARBReply>, ConnectionError>
    {
        get_query_objectuiv_arb(self, context_tag, id, pname)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
