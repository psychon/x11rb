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
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
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
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_into().or(Err(ParseError::ParseError))?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (desc_len, remaining) = u32::try_parse(remaining)?;
        let (description, remaining) = crate::x11_utils::parse_u8_list(remaining, desc_len.try_into().or(Err(ParseError::ParseError))?)?;
        let description = description.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = Printer { name, description };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Printer {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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

pub type Pcontext = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GetDoc {
    Finished = 0,
    SecondConsumer = 1,
}
impl From<GetDoc> for bool {
    fn from(input: GetDoc) -> Self {
        match input {
            GetDoc::Finished => false,
            GetDoc::SecondConsumer => true,
        }
    }
}
impl From<GetDoc> for u8 {
    fn from(input: GetDoc) -> Self {
        match input {
            GetDoc::Finished => 0,
            GetDoc::SecondConsumer => 1,
        }
    }
}
impl From<GetDoc> for Option<u8> {
    fn from(input: GetDoc) -> Self {
        Some(u8::from(input))
    }
}
impl From<GetDoc> for u16 {
    fn from(input: GetDoc) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GetDoc> for Option<u16> {
    fn from(input: GetDoc) -> Self {
        Some(u16::from(input))
    }
}
impl From<GetDoc> for u32 {
    fn from(input: GetDoc) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GetDoc> for Option<u32> {
    fn from(input: GetDoc) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GetDoc {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GetDoc::Finished),
            1 => Ok(GetDoc::SecondConsumer),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for GetDoc {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GetDoc {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EvMask {
    NoEventMask = 0,
    PrintMask = 1 << 0,
    AttributeMask = 1 << 1,
}
impl From<EvMask> for u8 {
    fn from(input: EvMask) -> Self {
        match input {
            EvMask::NoEventMask => 0,
            EvMask::PrintMask => 1 << 0,
            EvMask::AttributeMask => 1 << 1,
        }
    }
}
impl From<EvMask> for Option<u8> {
    fn from(input: EvMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<EvMask> for u16 {
    fn from(input: EvMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EvMask> for Option<u16> {
    fn from(input: EvMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<EvMask> for u32 {
    fn from(input: EvMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<EvMask> for Option<u32> {
    fn from(input: EvMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for EvMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EvMask::NoEventMask),
            1 => Ok(EvMask::PrintMask),
            2 => Ok(EvMask::AttributeMask),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for EvMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for EvMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(EvMask, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Detail {
    StartJobNotify = 1,
    EndJobNotify = 2,
    StartDocNotify = 3,
    EndDocNotify = 4,
    StartPageNotify = 5,
    EndPageNotify = 6,
}
impl From<Detail> for u8 {
    fn from(input: Detail) -> Self {
        match input {
            Detail::StartJobNotify => 1,
            Detail::EndJobNotify => 2,
            Detail::StartDocNotify => 3,
            Detail::EndDocNotify => 4,
            Detail::StartPageNotify => 5,
            Detail::EndPageNotify => 6,
        }
    }
}
impl From<Detail> for Option<u8> {
    fn from(input: Detail) -> Self {
        Some(u8::from(input))
    }
}
impl From<Detail> for u16 {
    fn from(input: Detail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Detail> for Option<u16> {
    fn from(input: Detail) -> Self {
        Some(u16::from(input))
    }
}
impl From<Detail> for u32 {
    fn from(input: Detail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Detail> for Option<u32> {
    fn from(input: Detail) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Detail {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Detail::StartJobNotify),
            2 => Ok(Detail::EndJobNotify),
            3 => Ok(Detail::StartDocNotify),
            4 => Ok(Detail::EndDocNotify),
            5 => Ok(Detail::StartPageNotify),
            6 => Ok(Detail::EndPageNotify),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Detail {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Detail {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Attr {
    JobAttr = 1,
    DocAttr = 2,
    PageAttr = 3,
    PrinterAttr = 4,
    ServerAttr = 5,
    MediumAttr = 6,
    SpoolerAttr = 7,
}
impl From<Attr> for u8 {
    fn from(input: Attr) -> Self {
        match input {
            Attr::JobAttr => 1,
            Attr::DocAttr => 2,
            Attr::PageAttr => 3,
            Attr::PrinterAttr => 4,
            Attr::ServerAttr => 5,
            Attr::MediumAttr => 6,
            Attr::SpoolerAttr => 7,
        }
    }
}
impl From<Attr> for Option<u8> {
    fn from(input: Attr) -> Self {
        Some(u8::from(input))
    }
}
impl From<Attr> for u16 {
    fn from(input: Attr) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Attr> for Option<u16> {
    fn from(input: Attr) -> Self {
        Some(u16::from(input))
    }
}
impl From<Attr> for u32 {
    fn from(input: Attr) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Attr> for Option<u32> {
    fn from(input: Attr) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Attr {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Attr::JobAttr),
            2 => Ok(Attr::DocAttr),
            3 => Ok(Attr::PageAttr),
            4 => Ok(Attr::PrinterAttr),
            5 => Ok(Attr::ServerAttr),
            6 => Ok(Attr::MediumAttr),
            7 => Ok(Attr::SpoolerAttr),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Attr {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Attr {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the PrintQueryVersion request
pub const PRINT_QUERY_VERSION_REQUEST: u8 = 0;
pub fn print_query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, PrintQueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let mut request0 = [
        extension_information.major_opcode,
        PRINT_QUERY_VERSION_REQUEST,
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintQueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl TryParse for PrintQueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let result = PrintQueryVersionReply { response_type, sequence, length, major_version, minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintQueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintGetPrinterList request
pub const PRINT_GET_PRINTER_LIST_REQUEST: u8 = 1;
pub fn print_get_printer_list<'c, Conn>(conn: &'c Conn, printer_name: &[String8], locale: &[String8]) -> Result<Cookie<'c, Conn, PrintGetPrinterListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let printer_name_len = u32::try_from(printer_name.len()).expect("`printer_name` has too many elements");
    let printer_name_len_bytes = printer_name_len.serialize();
    let locale_len = u32::try_from(locale.len()).expect("`locale` has too many elements");
    let locale_len_bytes = locale_len.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + printer_name.len();
    let length_so_far = length_so_far + locale.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(printer_name), IoSlice::new(locale), IoSlice::new(&padding0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetPrinterListReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub printers: Vec<Printer>,
}
impl TryParse for PrintGetPrinterListReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (list_count, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (printers, remaining) = crate::x11_utils::parse_list::<Printer>(remaining, list_count.try_into().or(Err(ParseError::ParseError))?)?;
        let result = PrintGetPrinterListReply { response_type, sequence, length, printers };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintGetPrinterListReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintRehashPrinterList request
pub const PRINT_REHASH_PRINTER_LIST_REQUEST: u8 = 20;
pub fn print_rehash_printer_list<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let mut request0 = [
        extension_information.major_opcode,
        PRINT_REHASH_PRINTER_LIST_REQUEST,
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 2;
pub fn create_context<'c, Conn>(conn: &'c Conn, context_id: u32, printer_name: &[String8], locale: &[String8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_id_bytes = context_id.serialize();
    let printer_name_len = u32::try_from(printer_name.len()).expect("`printer_name` has too many elements");
    let printer_name_len_bytes = printer_name_len.serialize();
    let locale_len = u32::try_from(locale.len()).expect("`locale` has too many elements");
    let locale_len_bytes = locale_len.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + printer_name.len();
    let length_so_far = length_so_far + locale.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(printer_name), IoSlice::new(locale), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the PrintSetContext request
pub const PRINT_SET_CONTEXT_REQUEST: u8 = 3;
pub fn print_set_context<Conn>(conn: &Conn, context: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintGetContext request
pub const PRINT_GET_CONTEXT_REQUEST: u8 = 4;
pub fn print_get_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, PrintGetContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let mut request0 = [
        extension_information.major_opcode,
        PRINT_GET_CONTEXT_REQUEST,
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: u32,
}
impl TryParse for PrintGetContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context, remaining) = u32::try_parse(remaining)?;
        let result = PrintGetContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintGetContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintDestroyContext request
pub const PRINT_DESTROY_CONTEXT_REQUEST: u8 = 5;
pub fn print_destroy_context<Conn>(conn: &Conn, context: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintGetScreenOfContext request
pub const PRINT_GET_SCREEN_OF_CONTEXT_REQUEST: u8 = 6;
pub fn print_get_screen_of_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, PrintGetScreenOfContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let mut request0 = [
        extension_information.major_opcode,
        PRINT_GET_SCREEN_OF_CONTEXT_REQUEST,
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetScreenOfContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xproto::Window,
}
impl TryParse for PrintGetScreenOfContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = xproto::Window::try_parse(remaining)?;
        let result = PrintGetScreenOfContextReply { response_type, sequence, length, root };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintGetScreenOfContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintStartJob request
pub const PRINT_START_JOB_REQUEST: u8 = 7;
pub fn print_start_job<Conn>(conn: &Conn, output_mode: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let output_mode_bytes = output_mode.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintEndJob request
pub const PRINT_END_JOB_REQUEST: u8 = 8;
pub fn print_end_job<Conn>(conn: &Conn, cancel: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let cancel_bytes = cancel.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintStartDoc request
pub const PRINT_START_DOC_REQUEST: u8 = 9;
pub fn print_start_doc<Conn>(conn: &Conn, driver_mode: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let driver_mode_bytes = driver_mode.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintEndDoc request
pub const PRINT_END_DOC_REQUEST: u8 = 10;
pub fn print_end_doc<Conn>(conn: &Conn, cancel: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let cancel_bytes = cancel.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintPutDocumentData request
pub const PRINT_PUT_DOCUMENT_DATA_REQUEST: u8 = 11;
pub fn print_put_document_data<'c, Conn>(conn: &'c Conn, drawable: xproto::Drawable, data: &[u8], doc_format: &[String8], options: &[String8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let drawable_bytes = drawable.serialize();
    let len_data = u32::try_from(data.len()).expect("`data` has too many elements");
    let len_data_bytes = len_data.serialize();
    let len_fmt = u16::try_from(doc_format.len()).expect("`doc_format` has too many elements");
    let len_fmt_bytes = len_fmt.serialize();
    let len_options = u16::try_from(options.len()).expect("`options` has too many elements");
    let len_options_bytes = len_options.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + data.len();
    let length_so_far = length_so_far + doc_format.len();
    let length_so_far = length_so_far + options.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(doc_format), IoSlice::new(options), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the PrintGetDocumentData request
pub const PRINT_GET_DOCUMENT_DATA_REQUEST: u8 = 12;
pub fn print_get_document_data<Conn>(conn: &Conn, context: Pcontext, max_bytes: u32) -> Result<Cookie<'_, Conn, PrintGetDocumentDataReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let max_bytes_bytes = max_bytes.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetDocumentDataReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status_code: u32,
    pub finished_flag: u32,
    pub data: Vec<u8>,
}
impl TryParse for PrintGetDocumentDataReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status_code, remaining) = u32::try_parse(remaining)?;
        let (finished_flag, remaining) = u32::try_parse(remaining)?;
        let (data_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, data_len.try_into().or(Err(ParseError::ParseError))?)?;
        let data = data.to_vec();
        let result = PrintGetDocumentDataReply { response_type, sequence, length, status_code, finished_flag, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintGetDocumentDataReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintStartPage request
pub const PRINT_START_PAGE_REQUEST: u8 = 13;
pub fn print_start_page<Conn>(conn: &Conn, window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let window_bytes = window.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintEndPage request
pub const PRINT_END_PAGE_REQUEST: u8 = 14;
pub fn print_end_page<Conn>(conn: &Conn, cancel: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let cancel_bytes = cancel.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintSelectInput request
pub const PRINT_SELECT_INPUT_REQUEST: u8 = 15;
pub fn print_select_input<Conn>(conn: &Conn, context: Pcontext, event_mask: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let event_mask_bytes = event_mask.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], vec![])?)
}

/// Opcode for the PrintInputSelected request
pub const PRINT_INPUT_SELECTED_REQUEST: u8 = 16;
pub fn print_input_selected<Conn>(conn: &Conn, context: Pcontext) -> Result<Cookie<'_, Conn, PrintInputSelectedReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintInputSelectedReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_mask: u32,
    pub all_events_mask: u32,
}
impl TryParse for PrintInputSelectedReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_mask, remaining) = u32::try_parse(remaining)?;
        let (all_events_mask, remaining) = u32::try_parse(remaining)?;
        let result = PrintInputSelectedReply { response_type, sequence, length, event_mask, all_events_mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintInputSelectedReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintGetAttributes request
pub const PRINT_GET_ATTRIBUTES_REQUEST: u8 = 17;
pub fn print_get_attributes<Conn>(conn: &Conn, context: Pcontext, pool: u8) -> Result<Cookie<'_, Conn, PrintGetAttributesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let pool_bytes = pool.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attributes: Vec<String8>,
}
impl TryParse for PrintGetAttributesReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (string_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (attributes, remaining) = crate::x11_utils::parse_u8_list(remaining, string_len.try_into().or(Err(ParseError::ParseError))?)?;
        let attributes = attributes.to_vec();
        let result = PrintGetAttributesReply { response_type, sequence, length, attributes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintGetAttributesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintGetOneAttributes request
pub const PRINT_GET_ONE_ATTRIBUTES_REQUEST: u8 = 19;
pub fn print_get_one_attributes<'c, Conn>(conn: &'c Conn, context: Pcontext, pool: u8, name: &[String8]) -> Result<Cookie<'c, Conn, PrintGetOneAttributesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let name_len = u32::try_from(name.len()).expect("`name` has too many elements");
    let name_len_bytes = name_len.serialize();
    let pool_bytes = pool.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + name.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintGetOneAttributesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub value: Vec<String8>,
}
impl TryParse for PrintGetOneAttributesReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (value_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (value, remaining) = crate::x11_utils::parse_u8_list(remaining, value_len.try_into().or(Err(ParseError::ParseError))?)?;
        let value = value.to_vec();
        let result = PrintGetOneAttributesReply { response_type, sequence, length, value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintGetOneAttributesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintSetAttributes request
pub const PRINT_SET_ATTRIBUTES_REQUEST: u8 = 18;
pub fn print_set_attributes<'c, Conn>(conn: &'c Conn, context: Pcontext, string_len: u32, pool: u8, rule: u8, attributes: &[String8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let string_len_bytes = string_len.serialize();
    let pool_bytes = pool.serialize();
    let rule_bytes = rule.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    let length_so_far = length_so_far + attributes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(attributes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the PrintGetPageDimensions request
pub const PRINT_GET_PAGE_DIMENSIONS_REQUEST: u8 = 21;
pub fn print_get_page_dimensions<Conn>(conn: &Conn, context: Pcontext) -> Result<Cookie<'_, Conn, PrintGetPageDimensionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetPageDimensionsReply {
    pub response_type: u8,
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
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (offset_x, remaining) = u16::try_parse(remaining)?;
        let (offset_y, remaining) = u16::try_parse(remaining)?;
        let (reproducible_width, remaining) = u16::try_parse(remaining)?;
        let (reproducible_height, remaining) = u16::try_parse(remaining)?;
        let result = PrintGetPageDimensionsReply { response_type, sequence, length, width, height, offset_x, offset_y, reproducible_width, reproducible_height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintGetPageDimensionsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintQueryScreens request
pub const PRINT_QUERY_SCREENS_REQUEST: u8 = 22;
pub fn print_query_screens<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, PrintQueryScreensReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let mut request0 = [
        extension_information.major_opcode,
        PRINT_QUERY_SCREENS_REQUEST,
        0,
        0,
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintQueryScreensReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub roots: Vec<xproto::Window>,
}
impl TryParse for PrintQueryScreensReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (list_count, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (roots, remaining) = crate::x11_utils::parse_list::<xproto::Window>(remaining, list_count.try_into().or(Err(ParseError::ParseError))?)?;
        let result = PrintQueryScreensReply { response_type, sequence, length, roots };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintQueryScreensReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintSetImageResolution request
pub const PRINT_SET_IMAGE_RESOLUTION_REQUEST: u8 = 23;
pub fn print_set_image_resolution<Conn>(conn: &Conn, context: Pcontext, image_resolution: u16) -> Result<Cookie<'_, Conn, PrintSetImageResolutionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let image_resolution_bytes = image_resolution.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintSetImageResolutionReply {
    pub response_type: u8,
    pub status: bool,
    pub sequence: u16,
    pub length: u32,
    pub previous_resolutions: u16,
}
impl TryParse for PrintSetImageResolutionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (previous_resolutions, remaining) = u16::try_parse(remaining)?;
        let result = PrintSetImageResolutionReply { response_type, status, sequence, length, previous_resolutions };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintSetImageResolutionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PrintGetImageResolution request
pub const PRINT_GET_IMAGE_RESOLUTION_REQUEST: u8 = 24;
pub fn print_get_image_resolution<Conn>(conn: &Conn, context: Pcontext) -> Result<Cookie<'_, Conn, PrintGetImageResolutionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
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
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintGetImageResolutionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub image_resolution: u16,
}
impl TryParse for PrintGetImageResolutionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (image_resolution, remaining) = u16::try_parse(remaining)?;
        let result = PrintGetImageResolutionReply { response_type, sequence, length, image_resolution };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PrintGetImageResolutionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (context, remaining) = Pcontext::try_parse(remaining)?;
        let (cancel, remaining) = bool::try_parse(remaining)?;
        let result = NotifyEvent { response_type, detail, sequence, context, cancel };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for NotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (context, remaining) = Pcontext::try_parse(remaining)?;
        let result = AttributNotifyEvent { response_type, detail, sequence, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AttributNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadContextError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl TryParse for BadContextError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadContextError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadContextError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&BadContextError> for [u8; 32] {
    fn from(input: &BadContextError) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let error_code_bytes = input.error_code.serialize();
        let sequence_bytes = input.sequence.serialize();
        [
            response_type_bytes[0],
            error_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
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
            0,
            0,
            0,
            0,
        ]
    }
}
impl From<BadContextError> for [u8; 32] {
    fn from(input: BadContextError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadSequence error
pub const BAD_SEQUENCE_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadSequenceError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl TryParse for BadSequenceError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadSequenceError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadSequenceError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&BadSequenceError> for [u8; 32] {
    fn from(input: &BadSequenceError) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let error_code_bytes = input.error_code.serialize();
        let sequence_bytes = input.sequence.serialize();
        [
            response_type_bytes[0],
            error_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
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
            0,
            0,
            0,
            0,
        ]
    }
}
impl From<BadSequenceError> for [u8; 32] {
    fn from(input: BadSequenceError) -> Self {
        Self::from(&input)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xprint_print_query_version(&self) -> Result<Cookie<'_, Self, PrintQueryVersionReply>, ConnectionError>
    {
        print_query_version(self)
    }
    fn xprint_print_get_printer_list<'c>(&'c self, printer_name: &[String8], locale: &[String8]) -> Result<Cookie<'c, Self, PrintGetPrinterListReply>, ConnectionError>
    {
        print_get_printer_list(self, printer_name, locale)
    }
    fn xprint_print_rehash_printer_list(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_rehash_printer_list(self)
    }
    fn xprint_create_context<'c>(&'c self, context_id: u32, printer_name: &[String8], locale: &[String8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_context(self, context_id, printer_name, locale)
    }
    fn xprint_print_set_context(&self, context: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_set_context(self, context)
    }
    fn xprint_print_get_context(&self) -> Result<Cookie<'_, Self, PrintGetContextReply>, ConnectionError>
    {
        print_get_context(self)
    }
    fn xprint_print_destroy_context(&self, context: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_destroy_context(self, context)
    }
    fn xprint_print_get_screen_of_context(&self) -> Result<Cookie<'_, Self, PrintGetScreenOfContextReply>, ConnectionError>
    {
        print_get_screen_of_context(self)
    }
    fn xprint_print_start_job(&self, output_mode: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_start_job(self, output_mode)
    }
    fn xprint_print_end_job(&self, cancel: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_end_job(self, cancel)
    }
    fn xprint_print_start_doc(&self, driver_mode: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_start_doc(self, driver_mode)
    }
    fn xprint_print_end_doc(&self, cancel: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_end_doc(self, cancel)
    }
    fn xprint_print_put_document_data<'c>(&'c self, drawable: xproto::Drawable, data: &[u8], doc_format: &[String8], options: &[String8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        print_put_document_data(self, drawable, data, doc_format, options)
    }
    fn xprint_print_get_document_data(&self, context: Pcontext, max_bytes: u32) -> Result<Cookie<'_, Self, PrintGetDocumentDataReply>, ConnectionError>
    {
        print_get_document_data(self, context, max_bytes)
    }
    fn xprint_print_start_page(&self, window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_start_page(self, window)
    }
    fn xprint_print_end_page(&self, cancel: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_end_page(self, cancel)
    }
    fn xprint_print_select_input(&self, context: Pcontext, event_mask: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        print_select_input(self, context, event_mask)
    }
    fn xprint_print_input_selected(&self, context: Pcontext) -> Result<Cookie<'_, Self, PrintInputSelectedReply>, ConnectionError>
    {
        print_input_selected(self, context)
    }
    fn xprint_print_get_attributes(&self, context: Pcontext, pool: u8) -> Result<Cookie<'_, Self, PrintGetAttributesReply>, ConnectionError>
    {
        print_get_attributes(self, context, pool)
    }
    fn xprint_print_get_one_attributes<'c>(&'c self, context: Pcontext, pool: u8, name: &[String8]) -> Result<Cookie<'c, Self, PrintGetOneAttributesReply>, ConnectionError>
    {
        print_get_one_attributes(self, context, pool, name)
    }
    fn xprint_print_set_attributes<'c>(&'c self, context: Pcontext, string_len: u32, pool: u8, rule: u8, attributes: &[String8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        print_set_attributes(self, context, string_len, pool, rule, attributes)
    }
    fn xprint_print_get_page_dimensions(&self, context: Pcontext) -> Result<Cookie<'_, Self, PrintGetPageDimensionsReply>, ConnectionError>
    {
        print_get_page_dimensions(self, context)
    }
    fn xprint_print_query_screens(&self) -> Result<Cookie<'_, Self, PrintQueryScreensReply>, ConnectionError>
    {
        print_query_screens(self)
    }
    fn xprint_print_set_image_resolution(&self, context: Pcontext, image_resolution: u16) -> Result<Cookie<'_, Self, PrintSetImageResolutionReply>, ConnectionError>
    {
        print_set_image_resolution(self, context, image_resolution)
    }
    fn xprint_print_get_image_resolution(&self, context: Pcontext) -> Result<Cookie<'_, Self, PrintGetImageResolutionReply>, ConnectionError>
    {
        print_get_image_resolution(self, context)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
