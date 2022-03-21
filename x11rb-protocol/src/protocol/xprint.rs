// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XPrint` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use std::borrow::Cow;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::convert::TryFrom;
use crate::errors::ParseError;
#[allow(unused_imports)]
use crate::x11_utils::TryIntoUSize;
use crate::{BufWithFds, PiecewiseBuf};
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
#[allow(unused_imports)]
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XpExtension";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 0);

pub type String8 = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Printer {
    pub name: Vec<String8>,
    pub description: Vec<String8>,
}
impl TryParse for Printer {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (name_len, remaining) = u32::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (desc_len, remaining) = u32::try_parse(remaining)?;
        let (description, remaining) = crate::x11_utils::parse_u8_list(remaining, desc_len.try_to_usize()?)?;
        let description = description.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let result = Printer { name, description };
        Ok((result, remaining))
    }
}
impl Serialize for Printer {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        let name_len = u32::try_from(self.name.len()).expect("`name` has too many elements");
        name_len.serialize_into(bytes);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        let desc_len = u32::try_from(self.description.len()).expect("`description` has too many elements");
        desc_len.serialize_into(bytes);
        bytes.extend_from_slice(&self.description);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl Printer {
    /// Get the value of the `nameLen` field.
    ///
    /// The `nameLen` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn name_len(&self) -> u32 {
        self.name.len()
            .try_into().unwrap()
    }
    /// Get the value of the `descLen` field.
    ///
    /// The `descLen` field is used as the length field of the `description` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn desc_len(&self) -> u32 {
        self.description.len()
            .try_into().unwrap()
    }
}

pub type Pcontext = u32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GetDoc(bool);
impl GetDoc {
    pub const FINISHED: Self = Self(false);
    pub const SECOND_CONSUMER: Self = Self(true);
}
impl From<GetDoc> for bool {
    #[inline]
    fn from(input: GetDoc) -> Self {
        input.0
    }
}
impl From<GetDoc> for Option<bool> {
    #[inline]
    fn from(input: GetDoc) -> Self {
        Some(input.0)
    }
}
impl From<GetDoc> for u8 {
    #[inline]
    fn from(input: GetDoc) -> Self {
        u8::from(input.0)
    }
}
impl From<GetDoc> for Option<u8> {
    #[inline]
    fn from(input: GetDoc) -> Self {
        Some(u8::from(input.0))
    }
}
impl From<GetDoc> for u16 {
    #[inline]
    fn from(input: GetDoc) -> Self {
        u16::from(input.0)
    }
}
impl From<GetDoc> for Option<u16> {
    #[inline]
    fn from(input: GetDoc) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<GetDoc> for u32 {
    #[inline]
    fn from(input: GetDoc) -> Self {
        u32::from(input.0)
    }
}
impl From<GetDoc> for Option<u32> {
    #[inline]
    fn from(input: GetDoc) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<bool> for GetDoc {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for GetDoc  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::FINISHED.0.into(), "FINISHED", "Finished"),
            (Self::SECOND_CONSUMER.0.into(), "SECOND_CONSUMER", "SecondConsumer"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EvMask(u8);
impl EvMask {
    pub const NO_EVENT_MASK: Self = Self(0);
    pub const PRINT_MASK: Self = Self(1 << 0);
    pub const ATTRIBUTE_MASK: Self = Self(1 << 1);
}
impl From<EvMask> for u8 {
    #[inline]
    fn from(input: EvMask) -> Self {
        input.0
    }
}
impl From<EvMask> for Option<u8> {
    #[inline]
    fn from(input: EvMask) -> Self {
        Some(input.0)
    }
}
impl From<EvMask> for u16 {
    #[inline]
    fn from(input: EvMask) -> Self {
        u16::from(input.0)
    }
}
impl From<EvMask> for Option<u16> {
    #[inline]
    fn from(input: EvMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<EvMask> for u32 {
    #[inline]
    fn from(input: EvMask) -> Self {
        u32::from(input.0)
    }
}
impl From<EvMask> for Option<u32> {
    #[inline]
    fn from(input: EvMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for EvMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for EvMask  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::NO_EVENT_MASK.0.into(), "NO_EVENT_MASK", "NoEventMask"),
            (Self::PRINT_MASK.0.into(), "PRINT_MASK", "PrintMask"),
            (Self::ATTRIBUTE_MASK.0.into(), "ATTRIBUTE_MASK", "AttributeMask"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(EvMask, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Detail(u8);
impl Detail {
    pub const START_JOB_NOTIFY: Self = Self(1);
    pub const END_JOB_NOTIFY: Self = Self(2);
    pub const START_DOC_NOTIFY: Self = Self(3);
    pub const END_DOC_NOTIFY: Self = Self(4);
    pub const START_PAGE_NOTIFY: Self = Self(5);
    pub const END_PAGE_NOTIFY: Self = Self(6);
}
impl From<Detail> for u8 {
    #[inline]
    fn from(input: Detail) -> Self {
        input.0
    }
}
impl From<Detail> for Option<u8> {
    #[inline]
    fn from(input: Detail) -> Self {
        Some(input.0)
    }
}
impl From<Detail> for u16 {
    #[inline]
    fn from(input: Detail) -> Self {
        u16::from(input.0)
    }
}
impl From<Detail> for Option<u16> {
    #[inline]
    fn from(input: Detail) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Detail> for u32 {
    #[inline]
    fn from(input: Detail) -> Self {
        u32::from(input.0)
    }
}
impl From<Detail> for Option<u32> {
    #[inline]
    fn from(input: Detail) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Detail {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Detail  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::START_JOB_NOTIFY.0.into(), "START_JOB_NOTIFY", "StartJobNotify"),
            (Self::END_JOB_NOTIFY.0.into(), "END_JOB_NOTIFY", "EndJobNotify"),
            (Self::START_DOC_NOTIFY.0.into(), "START_DOC_NOTIFY", "StartDocNotify"),
            (Self::END_DOC_NOTIFY.0.into(), "END_DOC_NOTIFY", "EndDocNotify"),
            (Self::START_PAGE_NOTIFY.0.into(), "START_PAGE_NOTIFY", "StartPageNotify"),
            (Self::END_PAGE_NOTIFY.0.into(), "END_PAGE_NOTIFY", "EndPageNotify"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Attr(u8);
impl Attr {
    pub const JOB_ATTR: Self = Self(1);
    pub const DOC_ATTR: Self = Self(2);
    pub const PAGE_ATTR: Self = Self(3);
    pub const PRINTER_ATTR: Self = Self(4);
    pub const SERVER_ATTR: Self = Self(5);
    pub const MEDIUM_ATTR: Self = Self(6);
    pub const SPOOLER_ATTR: Self = Self(7);
}
impl From<Attr> for u8 {
    #[inline]
    fn from(input: Attr) -> Self {
        input.0
    }
}
impl From<Attr> for Option<u8> {
    #[inline]
    fn from(input: Attr) -> Self {
        Some(input.0)
    }
}
impl From<Attr> for u16 {
    #[inline]
    fn from(input: Attr) -> Self {
        u16::from(input.0)
    }
}
impl From<Attr> for Option<u16> {
    #[inline]
    fn from(input: Attr) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<Attr> for u32 {
    #[inline]
    fn from(input: Attr) -> Self {
        u32::from(input.0)
    }
}
impl From<Attr> for Option<u32> {
    #[inline]
    fn from(input: Attr) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for Attr {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Attr  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::JOB_ATTR.0.into(), "JOB_ATTR", "JobAttr"),
            (Self::DOC_ATTR.0.into(), "DOC_ATTR", "DocAttr"),
            (Self::PAGE_ATTR.0.into(), "PAGE_ATTR", "PageAttr"),
            (Self::PRINTER_ATTR.0.into(), "PRINTER_ATTR", "PrinterAttr"),
            (Self::SERVER_ATTR.0.into(), "SERVER_ATTR", "ServerAttr"),
            (Self::MEDIUM_ATTR.0.into(), "MEDIUM_ATTR", "MediumAttr"),
            (Self::SPOOLER_ATTR.0.into(), "SPOOLER_ATTR", "SpoolerAttr"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

/// Opcode for the PrintQueryVersion request
pub const PRINT_QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintQueryVersionRequest;
impl PrintQueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            PRINT_QUERY_VERSION_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(PrintQueryVersionRequest
        )
    }
}
impl Request for PrintQueryVersionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintQueryVersionRequest {
    type Reply = PrintQueryVersionReply;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintQueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl TryParse for PrintQueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintQueryVersionReply { sequence, length, major_version, minor_version };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the PrintGetPrinterList request
pub const PRINT_GET_PRINTER_LIST_REQUEST: u8 = 1;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetPrinterListRequest<'input> {
    pub printer_name: Cow<'input, [String8]>,
    pub locale: Cow<'input, [String8]>,
}
impl<'input> PrintGetPrinterListRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let printer_name_len = u32::try_from(self.printer_name.len()).expect("`printer_name` has too many elements");
        let printer_name_len_bytes = printer_name_len.serialize();
        let locale_len = u32::try_from(self.locale.len()).expect("`locale` has too many elements");
        let locale_len_bytes = locale_len.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_GET_PRINTER_LIST_REQUEST,
            0,
            0,
            printer_name_len_bytes[0],
            printer_name_len_bytes[1],
            printer_name_len_bytes[2],
            printer_name_len_bytes[3],
            locale_len_bytes[0],
            locale_len_bytes[1],
            locale_len_bytes[2],
            locale_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.printer_name.len();
        let length_so_far = length_so_far + self.locale.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.printer_name, self.locale, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_GET_PRINTER_LIST_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (printer_name_len, remaining) = u32::try_parse(value)?;
        let (locale_len, remaining) = u32::try_parse(remaining)?;
        let (printer_name, remaining) = crate::x11_utils::parse_u8_list(remaining, printer_name_len.try_to_usize()?)?;
        let (locale, remaining) = crate::x11_utils::parse_u8_list(remaining, locale_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(PrintGetPrinterListRequest {
            printer_name: Cow::Borrowed(printer_name),
            locale: Cow::Borrowed(locale),
        })
    }
    /// Clone all borrowed data in this PrintGetPrinterListRequest.
    pub fn into_owned(self) -> PrintGetPrinterListRequest<'static> {
        PrintGetPrinterListRequest {
            printer_name: Cow::Owned(self.printer_name.into_owned()),
            locale: Cow::Owned(self.locale.into_owned()),
        }
    }
}
impl<'input> Request for PrintGetPrinterListRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for PrintGetPrinterListRequest<'input> {
    type Reply = PrintGetPrinterListReply;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetPrinterListReply {
    pub sequence: u16,
    pub length: u32,
    pub printers: Vec<Printer>,
}
impl TryParse for PrintGetPrinterListReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (list_count, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (printers, remaining) = crate::x11_utils::parse_list::<Printer>(remaining, list_count.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintGetPrinterListReply { sequence, length, printers };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl PrintGetPrinterListReply {
    /// Get the value of the `listCount` field.
    ///
    /// The `listCount` field is used as the length field of the `printers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn list_count(&self) -> u32 {
        self.printers.len()
            .try_into().unwrap()
    }
}

/// Opcode for the PrintRehashPrinterList request
pub const PRINT_REHASH_PRINTER_LIST_REQUEST: u8 = 20;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintRehashPrinterListRequest;
impl PrintRehashPrinterListRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            PRINT_REHASH_PRINTER_LIST_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_REHASH_PRINTER_LIST_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(PrintRehashPrinterListRequest
        )
    }
}
impl Request for PrintRehashPrinterListRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintRehashPrinterListRequest {
}

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 2;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateContextRequest<'input> {
    pub context_id: u32,
    pub printer_name: Cow<'input, [String8]>,
    pub locale: Cow<'input, [String8]>,
}
impl<'input> CreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_id_bytes = self.context_id.serialize();
        let printer_name_len = u32::try_from(self.printer_name.len()).expect("`printer_name` has too many elements");
        let printer_name_len_bytes = printer_name_len.serialize();
        let locale_len = u32::try_from(self.locale.len()).expect("`locale` has too many elements");
        let locale_len_bytes = locale_len.serialize();
        let mut request0 = vec![
            major_opcode,
            CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_id_bytes[0],
            context_id_bytes[1],
            context_id_bytes[2],
            context_id_bytes[3],
            printer_name_len_bytes[0],
            printer_name_len_bytes[1],
            printer_name_len_bytes[2],
            printer_name_len_bytes[3],
            locale_len_bytes[0],
            locale_len_bytes[1],
            locale_len_bytes[2],
            locale_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.printer_name.len();
        let length_so_far = length_so_far + self.locale.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.printer_name, self.locale, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_id, remaining) = u32::try_parse(value)?;
        let (printer_name_len, remaining) = u32::try_parse(remaining)?;
        let (locale_len, remaining) = u32::try_parse(remaining)?;
        let (printer_name, remaining) = crate::x11_utils::parse_u8_list(remaining, printer_name_len.try_to_usize()?)?;
        let (locale, remaining) = crate::x11_utils::parse_u8_list(remaining, locale_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(CreateContextRequest {
            context_id,
            printer_name: Cow::Borrowed(printer_name),
            locale: Cow::Borrowed(locale),
        })
    }
    /// Clone all borrowed data in this CreateContextRequest.
    pub fn into_owned(self) -> CreateContextRequest<'static> {
        CreateContextRequest {
            context_id: self.context_id,
            printer_name: Cow::Owned(self.printer_name.into_owned()),
            locale: Cow::Owned(self.locale.into_owned()),
        }
    }
}
impl<'input> Request for CreateContextRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for CreateContextRequest<'input> {
}

/// Opcode for the PrintSetContext request
pub const PRINT_SET_CONTEXT_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintSetContextRequest {
    pub context: u32,
}
impl PrintSetContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_SET_CONTEXT_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_SET_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(PrintSetContextRequest {
            context,
        })
    }
}
impl Request for PrintSetContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintSetContextRequest {
}

/// Opcode for the PrintGetContext request
pub const PRINT_GET_CONTEXT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetContextRequest;
impl PrintGetContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            PRINT_GET_CONTEXT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_GET_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(PrintGetContextRequest
        )
    }
}
impl Request for PrintGetContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintGetContextRequest {
    type Reply = PrintGetContextReply;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: u32,
}
impl TryParse for PrintGetContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintGetContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the PrintDestroyContext request
pub const PRINT_DESTROY_CONTEXT_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintDestroyContextRequest {
    pub context: u32,
}
impl PrintDestroyContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_DESTROY_CONTEXT_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_DESTROY_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(PrintDestroyContextRequest {
            context,
        })
    }
}
impl Request for PrintDestroyContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintDestroyContextRequest {
}

/// Opcode for the PrintGetScreenOfContext request
pub const PRINT_GET_SCREEN_OF_CONTEXT_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetScreenOfContextRequest;
impl PrintGetScreenOfContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            PRINT_GET_SCREEN_OF_CONTEXT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_GET_SCREEN_OF_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(PrintGetScreenOfContextRequest
        )
    }
}
impl Request for PrintGetScreenOfContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintGetScreenOfContextRequest {
    type Reply = PrintGetScreenOfContextReply;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetScreenOfContextReply {
    pub sequence: u16,
    pub length: u32,
    pub root: xproto::Window,
}
impl TryParse for PrintGetScreenOfContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintGetScreenOfContextReply { sequence, length, root };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the PrintStartJob request
pub const PRINT_START_JOB_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintStartJobRequest {
    pub output_mode: u8,
}
impl PrintStartJobRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let output_mode_bytes = self.output_mode.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_START_JOB_REQUEST,
            0,
            0,
            output_mode_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_START_JOB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (output_mode, remaining) = u8::try_parse(value)?;
        let _ = remaining;
        Ok(PrintStartJobRequest {
            output_mode,
        })
    }
}
impl Request for PrintStartJobRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintStartJobRequest {
}

/// Opcode for the PrintEndJob request
pub const PRINT_END_JOB_REQUEST: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintEndJobRequest {
    pub cancel: bool,
}
impl PrintEndJobRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let cancel_bytes = self.cancel.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_END_JOB_REQUEST,
            0,
            0,
            cancel_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_END_JOB_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (cancel, remaining) = bool::try_parse(value)?;
        let _ = remaining;
        Ok(PrintEndJobRequest {
            cancel,
        })
    }
}
impl Request for PrintEndJobRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintEndJobRequest {
}

/// Opcode for the PrintStartDoc request
pub const PRINT_START_DOC_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintStartDocRequest {
    pub driver_mode: u8,
}
impl PrintStartDocRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let driver_mode_bytes = self.driver_mode.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_START_DOC_REQUEST,
            0,
            0,
            driver_mode_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_START_DOC_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (driver_mode, remaining) = u8::try_parse(value)?;
        let _ = remaining;
        Ok(PrintStartDocRequest {
            driver_mode,
        })
    }
}
impl Request for PrintStartDocRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintStartDocRequest {
}

/// Opcode for the PrintEndDoc request
pub const PRINT_END_DOC_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintEndDocRequest {
    pub cancel: bool,
}
impl PrintEndDocRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let cancel_bytes = self.cancel.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_END_DOC_REQUEST,
            0,
            0,
            cancel_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_END_DOC_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (cancel, remaining) = bool::try_parse(value)?;
        let _ = remaining;
        Ok(PrintEndDocRequest {
            cancel,
        })
    }
}
impl Request for PrintEndDocRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintEndDocRequest {
}

/// Opcode for the PrintPutDocumentData request
pub const PRINT_PUT_DOCUMENT_DATA_REQUEST: u8 = 11;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintPutDocumentDataRequest<'input> {
    pub drawable: xproto::Drawable,
    pub data: Cow<'input, [u8]>,
    pub doc_format: Cow<'input, [String8]>,
    pub options: Cow<'input, [String8]>,
}
impl<'input> PrintPutDocumentDataRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let len_data = u32::try_from(self.data.len()).expect("`data` has too many elements");
        let len_data_bytes = len_data.serialize();
        let len_fmt = u16::try_from(self.doc_format.len()).expect("`doc_format` has too many elements");
        let len_fmt_bytes = len_fmt.serialize();
        let len_options = u16::try_from(self.options.len()).expect("`options` has too many elements");
        let len_options_bytes = len_options.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_PUT_DOCUMENT_DATA_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            len_data_bytes[0],
            len_data_bytes[1],
            len_data_bytes[2],
            len_data_bytes[3],
            len_fmt_bytes[0],
            len_fmt_bytes[1],
            len_options_bytes[0],
            len_options_bytes[1],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.data.len();
        let length_so_far = length_so_far + self.doc_format.len();
        let length_so_far = length_so_far + self.options.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.data, self.doc_format, self.options, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_PUT_DOCUMENT_DATA_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (len_data, remaining) = u32::try_parse(remaining)?;
        let (len_fmt, remaining) = u16::try_parse(remaining)?;
        let (len_options, remaining) = u16::try_parse(remaining)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, len_data.try_to_usize()?)?;
        let (doc_format, remaining) = crate::x11_utils::parse_u8_list(remaining, len_fmt.try_to_usize()?)?;
        let (options, remaining) = crate::x11_utils::parse_u8_list(remaining, len_options.try_to_usize()?)?;
        let _ = remaining;
        Ok(PrintPutDocumentDataRequest {
            drawable,
            data: Cow::Borrowed(data),
            doc_format: Cow::Borrowed(doc_format),
            options: Cow::Borrowed(options),
        })
    }
    /// Clone all borrowed data in this PrintPutDocumentDataRequest.
    pub fn into_owned(self) -> PrintPutDocumentDataRequest<'static> {
        PrintPutDocumentDataRequest {
            drawable: self.drawable,
            data: Cow::Owned(self.data.into_owned()),
            doc_format: Cow::Owned(self.doc_format.into_owned()),
            options: Cow::Owned(self.options.into_owned()),
        }
    }
}
impl<'input> Request for PrintPutDocumentDataRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for PrintPutDocumentDataRequest<'input> {
}

/// Opcode for the PrintGetDocumentData request
pub const PRINT_GET_DOCUMENT_DATA_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetDocumentDataRequest {
    pub context: Pcontext,
    pub max_bytes: u32,
}
impl PrintGetDocumentDataRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let max_bytes_bytes = self.max_bytes.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_GET_DOCUMENT_DATA_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
            max_bytes_bytes[0],
            max_bytes_bytes[1],
            max_bytes_bytes[2],
            max_bytes_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_GET_DOCUMENT_DATA_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let (max_bytes, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(PrintGetDocumentDataRequest {
            context,
            max_bytes,
        })
    }
}
impl Request for PrintGetDocumentDataRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintGetDocumentDataRequest {
    type Reply = PrintGetDocumentDataReply;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetDocumentDataReply {
    pub sequence: u16,
    pub length: u32,
    pub status_code: u32,
    pub finished_flag: u32,
    pub data: Vec<u8>,
}
impl TryParse for PrintGetDocumentDataReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status_code, remaining) = u32::try_parse(remaining)?;
        let (finished_flag, remaining) = u32::try_parse(remaining)?;
        let (data_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, data_len.try_to_usize()?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintGetDocumentDataReply { sequence, length, status_code, finished_flag, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl PrintGetDocumentDataReply {
    /// Get the value of the `dataLen` field.
    ///
    /// The `dataLen` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn data_len(&self) -> u32 {
        self.data.len()
            .try_into().unwrap()
    }
}

/// Opcode for the PrintStartPage request
pub const PRINT_START_PAGE_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintStartPageRequest {
    pub window: xproto::Window,
}
impl PrintStartPageRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_START_PAGE_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_START_PAGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(PrintStartPageRequest {
            window,
        })
    }
}
impl Request for PrintStartPageRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintStartPageRequest {
}

/// Opcode for the PrintEndPage request
pub const PRINT_END_PAGE_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintEndPageRequest {
    pub cancel: bool,
}
impl PrintEndPageRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let cancel_bytes = self.cancel.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_END_PAGE_REQUEST,
            0,
            0,
            cancel_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_END_PAGE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (cancel, remaining) = bool::try_parse(value)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(PrintEndPageRequest {
            cancel,
        })
    }
}
impl Request for PrintEndPageRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintEndPageRequest {
}

/// Opcode for the PrintSelectInput request
pub const PRINT_SELECT_INPUT_REQUEST: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintSelectInputRequest {
    pub context: Pcontext,
    pub event_mask: u32,
}
impl PrintSelectInputRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let event_mask_bytes = self.event_mask.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_SELECT_INPUT_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
            event_mask_bytes[0],
            event_mask_bytes[1],
            event_mask_bytes[2],
            event_mask_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_SELECT_INPUT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let (event_mask, remaining) = u32::try_parse(remaining)?;
        let _ = remaining;
        Ok(PrintSelectInputRequest {
            context,
            event_mask,
        })
    }
}
impl Request for PrintSelectInputRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::VoidRequest for PrintSelectInputRequest {
}

/// Opcode for the PrintInputSelected request
pub const PRINT_INPUT_SELECTED_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintInputSelectedRequest {
    pub context: Pcontext,
}
impl PrintInputSelectedRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_INPUT_SELECTED_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_INPUT_SELECTED_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let _ = remaining;
        Ok(PrintInputSelectedRequest {
            context,
        })
    }
}
impl Request for PrintInputSelectedRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintInputSelectedRequest {
    type Reply = PrintInputSelectedReply;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintInputSelectedReply {
    pub sequence: u16,
    pub length: u32,
    pub event_mask: u32,
    pub all_events_mask: u32,
}
impl TryParse for PrintInputSelectedReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_mask, remaining) = u32::try_parse(remaining)?;
        let (all_events_mask, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintInputSelectedReply { sequence, length, event_mask, all_events_mask };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the PrintGetAttributes request
pub const PRINT_GET_ATTRIBUTES_REQUEST: u8 = 17;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetAttributesRequest {
    pub context: Pcontext,
    pub pool: u8,
}
impl PrintGetAttributesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let pool_bytes = self.pool.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_GET_ATTRIBUTES_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
            pool_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_GET_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let (pool, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        Ok(PrintGetAttributesRequest {
            context,
            pool,
        })
    }
}
impl Request for PrintGetAttributesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintGetAttributesRequest {
    type Reply = PrintGetAttributesReply;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetAttributesReply {
    pub sequence: u16,
    pub length: u32,
    pub attributes: Vec<String8>,
}
impl TryParse for PrintGetAttributesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (string_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (attributes, remaining) = crate::x11_utils::parse_u8_list(remaining, string_len.try_to_usize()?)?;
        let attributes = attributes.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintGetAttributesReply { sequence, length, attributes };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl PrintGetAttributesReply {
    /// Get the value of the `stringLen` field.
    ///
    /// The `stringLen` field is used as the length field of the `attributes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn string_len(&self) -> u32 {
        self.attributes.len()
            .try_into().unwrap()
    }
}

/// Opcode for the PrintGetOneAttributes request
pub const PRINT_GET_ONE_ATTRIBUTES_REQUEST: u8 = 19;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetOneAttributesRequest<'input> {
    pub context: Pcontext,
    pub pool: u8,
    pub name: Cow<'input, [String8]>,
}
impl<'input> PrintGetOneAttributesRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let name_len = u32::try_from(self.name.len()).expect("`name` has too many elements");
        let name_len_bytes = name_len.serialize();
        let pool_bytes = self.pool.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_GET_ONE_ATTRIBUTES_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
            name_len_bytes[0],
            name_len_bytes[1],
            name_len_bytes[2],
            name_len_bytes[3],
            pool_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.name.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.name, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_GET_ONE_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let (name_len, remaining) = u32::try_parse(remaining)?;
        let (pool, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(PrintGetOneAttributesRequest {
            context,
            pool,
            name: Cow::Borrowed(name),
        })
    }
    /// Clone all borrowed data in this PrintGetOneAttributesRequest.
    pub fn into_owned(self) -> PrintGetOneAttributesRequest<'static> {
        PrintGetOneAttributesRequest {
            context: self.context,
            pool: self.pool,
            name: Cow::Owned(self.name.into_owned()),
        }
    }
}
impl<'input> Request for PrintGetOneAttributesRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for PrintGetOneAttributesRequest<'input> {
    type Reply = PrintGetOneAttributesReply;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetOneAttributesReply {
    pub sequence: u16,
    pub length: u32,
    pub value: Vec<String8>,
}
impl TryParse for PrintGetOneAttributesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (value_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (value, remaining) = crate::x11_utils::parse_u8_list(remaining, value_len.try_to_usize()?)?;
        let value = value.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintGetOneAttributesReply { sequence, length, value };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl PrintGetOneAttributesReply {
    /// Get the value of the `valueLen` field.
    ///
    /// The `valueLen` field is used as the length field of the `value` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn value_len(&self) -> u32 {
        self.value.len()
            .try_into().unwrap()
    }
}

/// Opcode for the PrintSetAttributes request
pub const PRINT_SET_ATTRIBUTES_REQUEST: u8 = 18;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintSetAttributesRequest<'input> {
    pub context: Pcontext,
    pub string_len: u32,
    pub pool: u8,
    pub rule: u8,
    pub attributes: Cow<'input, [String8]>,
}
impl<'input> PrintSetAttributesRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let string_len_bytes = self.string_len.serialize();
        let pool_bytes = self.pool.serialize();
        let rule_bytes = self.rule.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_SET_ATTRIBUTES_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
            string_len_bytes[0],
            string_len_bytes[1],
            string_len_bytes[2],
            string_len_bytes[3],
            pool_bytes[0],
            rule_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.attributes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.attributes, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_SET_ATTRIBUTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let (string_len, remaining) = u32::try_parse(remaining)?;
        let (pool, remaining) = u8::try_parse(remaining)?;
        let (rule, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (attributes, remaining) = remaining.split_at(remaining.len());
        let _ = remaining;
        Ok(PrintSetAttributesRequest {
            context,
            string_len,
            pool,
            rule,
            attributes: Cow::Borrowed(attributes),
        })
    }
    /// Clone all borrowed data in this PrintSetAttributesRequest.
    pub fn into_owned(self) -> PrintSetAttributesRequest<'static> {
        PrintSetAttributesRequest {
            context: self.context,
            string_len: self.string_len,
            pool: self.pool,
            rule: self.rule,
            attributes: Cow::Owned(self.attributes.into_owned()),
        }
    }
}
impl<'input> Request for PrintSetAttributesRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for PrintSetAttributesRequest<'input> {
}

/// Opcode for the PrintGetPageDimensions request
pub const PRINT_GET_PAGE_DIMENSIONS_REQUEST: u8 = 21;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetPageDimensionsRequest {
    pub context: Pcontext,
}
impl PrintGetPageDimensionsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_GET_PAGE_DIMENSIONS_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_GET_PAGE_DIMENSIONS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let _ = remaining;
        Ok(PrintGetPageDimensionsRequest {
            context,
        })
    }
}
impl Request for PrintGetPageDimensionsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintGetPageDimensionsRequest {
    type Reply = PrintGetPageDimensionsReply;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetPageDimensionsReply {
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
    pub offset_x: u16,
    pub offset_y: u16,
    pub reproducible_width: u16,
    pub reproducible_height: u16,
}
impl TryParse for PrintGetPageDimensionsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (offset_x, remaining) = u16::try_parse(remaining)?;
        let (offset_y, remaining) = u16::try_parse(remaining)?;
        let (reproducible_width, remaining) = u16::try_parse(remaining)?;
        let (reproducible_height, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintGetPageDimensionsReply { sequence, length, width, height, offset_x, offset_y, reproducible_width, reproducible_height };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the PrintQueryScreens request
pub const PRINT_QUERY_SCREENS_REQUEST: u8 = 22;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintQueryScreensRequest;
impl PrintQueryScreensRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            PRINT_QUERY_SCREENS_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_QUERY_SCREENS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(PrintQueryScreensRequest
        )
    }
}
impl Request for PrintQueryScreensRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintQueryScreensRequest {
    type Reply = PrintQueryScreensReply;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintQueryScreensReply {
    pub sequence: u16,
    pub length: u32,
    pub roots: Vec<xproto::Window>,
}
impl TryParse for PrintQueryScreensReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (list_count, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (roots, remaining) = crate::x11_utils::parse_list::<xproto::Window>(remaining, list_count.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintQueryScreensReply { sequence, length, roots };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl PrintQueryScreensReply {
    /// Get the value of the `listCount` field.
    ///
    /// The `listCount` field is used as the length field of the `roots` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn list_count(&self) -> u32 {
        self.roots.len()
            .try_into().unwrap()
    }
}

/// Opcode for the PrintSetImageResolution request
pub const PRINT_SET_IMAGE_RESOLUTION_REQUEST: u8 = 23;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintSetImageResolutionRequest {
    pub context: Pcontext,
    pub image_resolution: u16,
}
impl PrintSetImageResolutionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let image_resolution_bytes = self.image_resolution.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_SET_IMAGE_RESOLUTION_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
            image_resolution_bytes[0],
            image_resolution_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_SET_IMAGE_RESOLUTION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let (image_resolution, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(PrintSetImageResolutionRequest {
            context,
            image_resolution,
        })
    }
}
impl Request for PrintSetImageResolutionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintSetImageResolutionRequest {
    type Reply = PrintSetImageResolutionReply;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintSetImageResolutionReply {
    pub status: bool,
    pub sequence: u16,
    pub length: u32,
    pub previous_resolutions: u16,
}
impl TryParse for PrintSetImageResolutionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (previous_resolutions, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintSetImageResolutionReply { status, sequence, length, previous_resolutions };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the PrintGetImageResolution request
pub const PRINT_GET_IMAGE_RESOLUTION_REQUEST: u8 = 24;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetImageResolutionRequest {
    pub context: Pcontext,
}
impl PrintGetImageResolutionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
            major_opcode,
            PRINT_GET_IMAGE_RESOLUTION_REQUEST,
            0,
            0,
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != PRINT_GET_IMAGE_RESOLUTION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context, remaining) = Pcontext::try_parse(value)?;
        let _ = remaining;
        Ok(PrintGetImageResolutionRequest {
            context,
        })
    }
}
impl Request for PrintGetImageResolutionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for PrintGetImageResolutionRequest {
    type Reply = PrintGetImageResolutionReply;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetImageResolutionReply {
    pub sequence: u16,
    pub length: u32,
    pub image_resolution: u16,
}
impl TryParse for PrintGetImageResolutionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (image_resolution, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = PrintGetImageResolutionReply { sequence, length, image_resolution };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the Notify event
pub const NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotifyEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub context: Pcontext,
    pub cancel: bool,
}
impl TryParse for NotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (context, remaining) = Pcontext::try_parse(remaining)?;
        let (cancel, remaining) = bool::try_parse(remaining)?;
        let result = NotifyEvent { response_type, detail, sequence, context, cancel };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl From<&NotifyEvent> for [u8; 32] {
    fn from(input: &NotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let detail_bytes = input.detail.serialize();
        let sequence_bytes = input.sequence.serialize();
        let context_bytes = input.context.serialize();
        let cancel_bytes = input.cancel.serialize();
        [
            response_type_bytes[0],
            detail_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
            cancel_bytes[0],
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
impl From<NotifyEvent> for [u8; 32] {
    fn from(input: NotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the AttributNotify event
pub const ATTRIBUT_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributNotifyEvent {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub context: Pcontext,
}
impl TryParse for AttributNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (context, remaining) = Pcontext::try_parse(remaining)?;
        let result = AttributNotifyEvent { response_type, detail, sequence, context };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl From<&AttributNotifyEvent> for [u8; 32] {
    fn from(input: &AttributNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let detail_bytes = input.detail.serialize();
        let sequence_bytes = input.sequence.serialize();
        let context_bytes = input.context.serialize();
        [
            response_type_bytes[0],
            detail_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            context_bytes[0],
            context_bytes[1],
            context_bytes[2],
            context_bytes[3],
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
impl From<AttributNotifyEvent> for [u8; 32] {
    fn from(input: AttributNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadContext error
pub const BAD_CONTEXT_ERROR: u8 = 0;

/// Opcode for the BadSequence error
pub const BAD_SEQUENCE_ERROR: u8 = 1;

