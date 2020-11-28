// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Shm` X11 extension.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]

#[allow(unused_imports)]
use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "MIT-SHM";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 2);

pub type Seg = u32;

/// Opcode for the Completion event
pub const COMPLETION_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub drawable: xproto::Drawable,
    pub minor_event: u16,
    pub major_event: u8,
    pub shmseg: Seg,
    pub offset: u32,
}
impl TryParse for CompletionEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (minor_event, remaining) = u16::try_parse(remaining)?;
        let (major_event, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (shmseg, remaining) = Seg::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let result = CompletionEvent { response_type, sequence, drawable, minor_event, major_event, shmseg, offset };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CompletionEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&CompletionEvent> for [u8; 32] {
    fn from(input: &CompletionEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let sequence_bytes = input.sequence.serialize();
        let drawable_bytes = input.drawable.serialize();
        let minor_event_bytes = input.minor_event.serialize();
        let major_event_bytes = input.major_event.serialize();
        let shmseg_bytes = input.shmseg.serialize();
        let offset_bytes = input.offset.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            minor_event_bytes[0],
            minor_event_bytes[1],
            major_event_bytes[0],
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
            // trailing padding
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    }
}
impl From<CompletionEvent> for [u8; 32] {
    fn from(input: CompletionEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadSeg error
pub const BAD_SEG_ERROR: u8 = 0;

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest;
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(QueryVersionRequest
        )
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub shared_pixmaps: bool,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub uid: u16,
    pub gid: u16,
    pub pixmap_format: u8,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (shared_pixmaps, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let (uid, remaining) = u16::try_parse(remaining)?;
        let (gid, remaining) = u16::try_parse(remaining)?;
        let (pixmap_format, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply { shared_pixmaps, sequence, length, major_version, minor_version, uid, gid, pixmap_format };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the Attach request
pub const ATTACH_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttachRequest {
    pub shmseg: Seg,
    pub shmid: u32,
    pub read_only: bool,
}
impl AttachRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let shmseg_bytes = self.shmseg.serialize();
        let shmid_bytes = self.shmid.serialize();
        let read_only_bytes = self.read_only.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ATTACH_REQUEST,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            shmid_bytes[0],
            shmid_bytes[1],
            shmid_bytes[2],
            shmid_bytes[3],
            read_only_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != ATTACH_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (shmseg, remaining) = Seg::try_parse(value)?;
        let (shmid, remaining) = u32::try_parse(remaining)?;
        let (read_only, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(AttachRequest {
            shmseg,
            shmid,
            read_only,
        })
    }
}
impl Request for AttachRequest {
    type Reply = ();
}
pub fn attach<Conn>(conn: &Conn, shmseg: Seg, shmid: u32, read_only: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AttachRequest {
        shmseg,
        shmid,
        read_only,
    };
    request0.send(conn)
}

/// Opcode for the Detach request
pub const DETACH_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DetachRequest {
    pub shmseg: Seg,
}
impl DetachRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let shmseg_bytes = self.shmseg.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DETACH_REQUEST,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DETACH_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (shmseg, remaining) = Seg::try_parse(value)?;
        let _ = remaining;
        Ok(DetachRequest {
            shmseg,
        })
    }
}
impl Request for DetachRequest {
    type Reply = ();
}
pub fn detach<Conn>(conn: &Conn, shmseg: Seg) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DetachRequest {
        shmseg,
    };
    request0.send(conn)
}

/// Opcode for the PutImage request
pub const PUT_IMAGE_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PutImageRequest {
    pub drawable: xproto::Drawable,
    pub gc: xproto::Gcontext,
    pub total_width: u16,
    pub total_height: u16,
    pub src_x: u16,
    pub src_y: u16,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub depth: u8,
    pub format: u8,
    pub send_event: bool,
    pub shmseg: Seg,
    pub offset: u32,
}
impl PutImageRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let gc_bytes = self.gc.serialize();
        let total_width_bytes = self.total_width.serialize();
        let total_height_bytes = self.total_height.serialize();
        let src_x_bytes = self.src_x.serialize();
        let src_y_bytes = self.src_y.serialize();
        let src_width_bytes = self.src_width.serialize();
        let src_height_bytes = self.src_height.serialize();
        let dst_x_bytes = self.dst_x.serialize();
        let dst_y_bytes = self.dst_y.serialize();
        let depth_bytes = self.depth.serialize();
        let format_bytes = self.format.serialize();
        let send_event_bytes = self.send_event.serialize();
        let shmseg_bytes = self.shmseg.serialize();
        let offset_bytes = self.offset.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            PUT_IMAGE_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            gc_bytes[0],
            gc_bytes[1],
            gc_bytes[2],
            gc_bytes[3],
            total_width_bytes[0],
            total_width_bytes[1],
            total_height_bytes[0],
            total_height_bytes[1],
            src_x_bytes[0],
            src_x_bytes[1],
            src_y_bytes[0],
            src_y_bytes[1],
            src_width_bytes[0],
            src_width_bytes[1],
            src_height_bytes[0],
            src_height_bytes[1],
            dst_x_bytes[0],
            dst_x_bytes[1],
            dst_y_bytes[0],
            dst_y_bytes[1],
            depth_bytes[0],
            format_bytes[0],
            send_event_bytes[0],
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PUT_IMAGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (gc, remaining) = xproto::Gcontext::try_parse(remaining)?;
        let (total_width, remaining) = u16::try_parse(remaining)?;
        let (total_height, remaining) = u16::try_parse(remaining)?;
        let (src_x, remaining) = u16::try_parse(remaining)?;
        let (src_y, remaining) = u16::try_parse(remaining)?;
        let (src_width, remaining) = u16::try_parse(remaining)?;
        let (src_height, remaining) = u16::try_parse(remaining)?;
        let (dst_x, remaining) = i16::try_parse(remaining)?;
        let (dst_y, remaining) = i16::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (send_event, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (shmseg, remaining) = Seg::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(PutImageRequest {
            drawable,
            gc,
            total_width,
            total_height,
            src_x,
            src_y,
            src_width,
            src_height,
            dst_x,
            dst_y,
            depth,
            format,
            send_event,
            shmseg,
            offset,
        })
    }
}
impl Request for PutImageRequest {
    type Reply = ();
}
pub fn put_image<Conn>(conn: &Conn, drawable: xproto::Drawable, gc: xproto::Gcontext, total_width: u16, total_height: u16, src_x: u16, src_y: u16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8, send_event: bool, shmseg: Seg, offset: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PutImageRequest {
        drawable,
        gc,
        total_width,
        total_height,
        src_x,
        src_y,
        src_width,
        src_height,
        dst_x,
        dst_y,
        depth,
        format,
        send_event,
        shmseg,
        offset,
    };
    request0.send(conn)
}

/// Opcode for the GetImage request
pub const GET_IMAGE_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetImageRequest {
    pub drawable: xproto::Drawable,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub plane_mask: u32,
    pub format: u8,
    pub shmseg: Seg,
    pub offset: u32,
}
impl GetImageRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let plane_mask_bytes = self.plane_mask.serialize();
        let format_bytes = self.format.serialize();
        let shmseg_bytes = self.shmseg.serialize();
        let offset_bytes = self.offset.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_IMAGE_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            plane_mask_bytes[0],
            plane_mask_bytes[1],
            plane_mask_bytes[2],
            plane_mask_bytes[3],
            format_bytes[0],
            0,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetImageReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_IMAGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (plane_mask, remaining) = u32::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (shmseg, remaining) = Seg::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetImageRequest {
            drawable,
            x,
            y,
            width,
            height,
            plane_mask,
            format,
            shmseg,
            offset,
        })
    }
}
impl Request for GetImageRequest {
    type Reply = GetImageReply;
}
pub fn get_image<Conn>(conn: &Conn, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: u8, shmseg: Seg, offset: u32) -> Result<Cookie<'_, Conn, GetImageReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetImageRequest {
        drawable,
        x,
        y,
        width,
        height,
        plane_mask,
        format,
        shmseg,
        offset,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetImageReply {
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xproto::Visualid,
    pub size: u32,
}
impl TryParse for GetImageReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (visual, remaining) = xproto::Visualid::try_parse(remaining)?;
        let (size, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetImageReply { depth, sequence, length, visual, size };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetImageReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreatePixmap request
pub const CREATE_PIXMAP_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatePixmapRequest {
    pub pid: xproto::Pixmap,
    pub drawable: xproto::Drawable,
    pub width: u16,
    pub height: u16,
    pub depth: u8,
    pub shmseg: Seg,
    pub offset: u32,
}
impl CreatePixmapRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pid_bytes = self.pid.serialize();
        let drawable_bytes = self.drawable.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let depth_bytes = self.depth.serialize();
        let shmseg_bytes = self.shmseg.serialize();
        let offset_bytes = self.offset.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_PIXMAP_REQUEST,
            0,
            0,
            pid_bytes[0],
            pid_bytes[1],
            pid_bytes[2],
            pid_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            depth_bytes[0],
            0,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_PIXMAP_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (pid, remaining) = xproto::Pixmap::try_parse(value)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (shmseg, remaining) = Seg::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreatePixmapRequest {
            pid,
            drawable,
            width,
            height,
            depth,
            shmseg,
            offset,
        })
    }
}
impl Request for CreatePixmapRequest {
    type Reply = ();
}
pub fn create_pixmap<Conn>(conn: &Conn, pid: xproto::Pixmap, drawable: xproto::Drawable, width: u16, height: u16, depth: u8, shmseg: Seg, offset: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreatePixmapRequest {
        pid,
        drawable,
        width,
        height,
        depth,
        shmseg,
        offset,
    };
    request0.send(conn)
}

/// Opcode for the AttachFd request
pub const ATTACH_FD_REQUEST: u8 = 6;
#[derive(Debug, PartialEq, Eq)]
pub struct AttachFdRequest {
    pub shmseg: Seg,
    pub shm_fd: RawFdContainer,
    pub read_only: bool,
}
impl AttachFdRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let shmseg_bytes = self.shmseg.serialize();
        let read_only_bytes = self.read_only.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            ATTACH_FD_REQUEST,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            read_only_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![self.shm_fd]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request_fd(header: RequestHeader, value: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<Self, ParseError> {
        if header.minor_opcode != ATTACH_FD_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (shmseg, remaining) = Seg::try_parse(value)?;
        if fds.is_empty() { return Err(ParseError::MissingFileDescriptors) }
        let shm_fd = fds.remove(0);
        let (read_only, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(AttachFdRequest {
            shmseg,
            shm_fd,
            read_only,
        })
    }
}
impl Request for AttachFdRequest {
    type Reply = ();
}
pub fn attach_fd<Conn, A>(conn: &Conn, shmseg: Seg, shm_fd: A, read_only: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<RawFdContainer>,
{
    let shm_fd: RawFdContainer = shm_fd.into();
    let request0 = AttachFdRequest {
        shmseg,
        shm_fd,
        read_only,
    };
    request0.send(conn)
}

/// Opcode for the CreateSegment request
pub const CREATE_SEGMENT_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateSegmentRequest {
    pub shmseg: Seg,
    pub size: u32,
    pub read_only: bool,
}
impl CreateSegmentRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let shmseg_bytes = self.shmseg.serialize();
        let size_bytes = self.size.serialize();
        let read_only_bytes = self.read_only.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_SEGMENT_REQUEST,
            0,
            0,
            shmseg_bytes[0],
            shmseg_bytes[1],
            shmseg_bytes[2],
            shmseg_bytes[3],
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
            read_only_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<CookieWithFds<'_, Conn, CreateSegmentReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply_with_fds(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_SEGMENT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (shmseg, remaining) = Seg::try_parse(value)?;
        let (size, remaining) = u32::try_parse(remaining)?;
        let (read_only, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(CreateSegmentRequest {
            shmseg,
            size,
            read_only,
        })
    }
}
impl Request for CreateSegmentRequest {
    type Reply = CreateSegmentReply;
}
pub fn create_segment<Conn>(conn: &Conn, shmseg: Seg, size: u32, read_only: bool) -> Result<CookieWithFds<'_, Conn, CreateSegmentReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateSegmentRequest {
        shmseg,
        size,
        read_only,
    };
    request0.send(conn)
}

#[derive(Debug, PartialEq, Eq)]
pub struct CreateSegmentReply {
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub shm_fd: RawFdContainer,
}
impl TryParseFd for CreateSegmentReply {
    fn try_parse_fd<'a>(initial_value: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::MissingFileDescriptors) }
        let shm_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = CreateSegmentReply { nfd, sequence, length, shm_fd };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for CreateSegmentReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn shm_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }
    fn shm_attach(&self, shmseg: Seg, shmid: u32, read_only: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        attach(self, shmseg, shmid, read_only)
    }
    fn shm_detach(&self, shmseg: Seg) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        detach(self, shmseg)
    }
    fn shm_put_image(&self, drawable: xproto::Drawable, gc: xproto::Gcontext, total_width: u16, total_height: u16, src_x: u16, src_y: u16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8, send_event: bool, shmseg: Seg, offset: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        put_image(self, drawable, gc, total_width, total_height, src_x, src_y, src_width, src_height, dst_x, dst_y, depth, format, send_event, shmseg, offset)
    }
    fn shm_get_image(&self, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: u8, shmseg: Seg, offset: u32) -> Result<Cookie<'_, Self, GetImageReply>, ConnectionError>
    {
        get_image(self, drawable, x, y, width, height, plane_mask, format, shmseg, offset)
    }
    fn shm_create_pixmap(&self, pid: xproto::Pixmap, drawable: xproto::Drawable, width: u16, height: u16, depth: u8, shmseg: Seg, offset: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_pixmap(self, pid, drawable, width, height, depth, shmseg, offset)
    }
    fn shm_attach_fd<A>(&self, shmseg: Seg, shm_fd: A, read_only: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<RawFdContainer>,
    {
        attach_fd(self, shmseg, shm_fd, read_only)
    }
    fn shm_create_segment(&self, shmseg: Seg, size: u32, read_only: bool) -> Result<CookieWithFds<'_, Self, CreateSegmentReply>, ConnectionError>
    {
        create_segment(self, shmseg, size, read_only)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
