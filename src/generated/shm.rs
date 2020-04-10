// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::Event as _;
use crate::x11_utils::{TryParse, Serialize};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ParseError, ConnectionError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;
#[allow(unused_imports)]
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
impl CompletionEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = xproto::Drawable::try_parse(remaining)?;
        let (minor_event, remaining) = u16::try_parse(remaining)?;
        let (major_event, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (shmseg, remaining) = Seg::try_parse(remaining)?;
        let (offset, remaining) = u32::try_parse(remaining)?;
        let result = CompletionEvent { response_type, sequence, drawable, minor_event, major_event, shmseg, offset };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CompletionEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for CompletionEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for CompletionEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&CompletionEvent> for [u8; 32] {
    fn from(input: &CompletionEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let drawable = input.drawable.serialize();
        let minor_event = input.minor_event.serialize();
        let major_event = input.major_event.serialize();
        let shmseg = input.shmseg.serialize();
        let offset = input.offset.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], drawable[0], drawable[1], drawable[2], drawable[3],
            minor_event[0], minor_event[1], major_event[0], 0, shmseg[0], shmseg[1], shmseg[2], shmseg[3],
            offset[0], offset[1], offset[2], offset[3], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadSegError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl BadSegError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = BadSegError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadSegError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadSegError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadSegError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadSegError> for [u8; 32] {
    fn from(input: &BadSegError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadSegError> for [u8; 32] {
    fn from(input: BadSegError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub shared_pixmaps: bool,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub uid: u16,
    pub gid: u16,
    pub pixmap_format: u8,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (shared_pixmaps, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let (uid, remaining) = u16::try_parse(remaining)?;
        let (gid, remaining) = u16::try_parse(remaining)?;
        let (pixmap_format, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(15..).ok_or(ParseError::ParseError)?;
        let result = QueryVersionReply { response_type, shared_pixmaps, sequence, length, major_version, minor_version, uid, gid, pixmap_format };
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
pub fn attach<Conn>(conn: &Conn, shmseg: Seg, shmid: u32, read_only: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let shmseg_bytes = shmseg.serialize();
    let shmid_bytes = shmid.serialize();
    let read_only_bytes = (read_only as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        ATTACH_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the Detach request
pub const DETACH_REQUEST: u8 = 2;
pub fn detach<Conn>(conn: &Conn, shmseg: Seg) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let shmseg_bytes = shmseg.serialize();
    let request0 = [
        extension_information.major_opcode,
        DETACH_REQUEST,
        length_bytes[0],
        length_bytes[1],
        shmseg_bytes[0],
        shmseg_bytes[1],
        shmseg_bytes[2],
        shmseg_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the PutImage request
pub const PUT_IMAGE_REQUEST: u8 = 3;
pub fn put_image<Conn>(conn: &Conn, drawable: xproto::Drawable, gc: xproto::Gcontext, total_width: u16, total_height: u16, src_x: u16, src_y: u16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8, send_event: bool, shmseg: Seg, offset: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (40) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let total_width_bytes = total_width.serialize();
    let total_height_bytes = total_height.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let src_width_bytes = src_width.serialize();
    let src_height_bytes = src_height.serialize();
    let dst_x_bytes = dst_x.serialize();
    let dst_y_bytes = dst_y.serialize();
    let depth_bytes = depth.serialize();
    let format_bytes = format.serialize();
    let send_event_bytes = (send_event as u8).serialize();
    let shmseg_bytes = shmseg.serialize();
    let offset_bytes = offset.serialize();
    let request0 = [
        extension_information.major_opcode,
        PUT_IMAGE_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetImage request
pub const GET_IMAGE_REQUEST: u8 = 4;
pub fn get_image<Conn>(conn: &Conn, drawable: xproto::Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: u8, shmseg: Seg, offset: u32) -> Result<Cookie<'_, Conn, GetImageReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let plane_mask_bytes = plane_mask.serialize();
    let format_bytes = format.serialize();
    let shmseg_bytes = shmseg.serialize();
    let offset_bytes = offset.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_IMAGE_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetImageReply {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xproto::Visualid,
    pub size: u32,
}
impl GetImageReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (visual, remaining) = xproto::Visualid::try_parse(remaining)?;
        let (size, remaining) = u32::try_parse(remaining)?;
        let result = GetImageReply { response_type, depth, sequence, length, visual, size };
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
pub fn create_pixmap<Conn>(conn: &Conn, pid: xproto::Pixmap, drawable: xproto::Drawable, width: u16, height: u16, depth: u8, shmseg: Seg, offset: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (28) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let pid_bytes = pid.serialize();
    let drawable_bytes = drawable.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let depth_bytes = depth.serialize();
    let shmseg_bytes = shmseg.serialize();
    let offset_bytes = offset.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_PIXMAP_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the AttachFd request
pub const ATTACH_FD_REQUEST: u8 = 6;
pub fn attach_fd<Conn, A>(conn: &Conn, shmseg: Seg, shm_fd: A, read_only: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<RawFdContainer>
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let shmseg_bytes = shmseg.serialize();
    let read_only_bytes = (read_only as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        ATTACH_FD_REQUEST,
        length_bytes[0],
        length_bytes[1],
        shmseg_bytes[0],
        shmseg_bytes[1],
        shmseg_bytes[2],
        shmseg_bytes[3],
        read_only_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    let fds = vec!(shm_fd.into());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], fds)?)
}

/// Opcode for the CreateSegment request
pub const CREATE_SEGMENT_REQUEST: u8 = 7;
pub fn create_segment<Conn>(conn: &Conn, shmseg: Seg, size: u32, read_only: bool) -> Result<CookieWithFds<'_, Conn, CreateSegmentReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let shmseg_bytes = shmseg.serialize();
    let size_bytes = size.serialize();
    let read_only_bytes = (read_only as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_SEGMENT_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply_with_fds(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, PartialEq, Eq)]
pub struct CreateSegmentReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub shm_fd: RawFdContainer,
}
impl CreateSegmentReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::ParseError) }
        let shm_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let result = CreateSegmentReply { response_type, nfd, sequence, length, shm_fd };
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
    where A: Into<RawFdContainer>
    {
        attach_fd(self, shmseg, shm_fd, read_only)
    }

    fn shm_create_segment(&self, shmseg: Seg, size: u32, read_only: bool) -> Result<CookieWithFds<'_, Self, CreateSegmentReply>, ConnectionError>
    {
        create_segment(self, shmseg, size, read_only)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
