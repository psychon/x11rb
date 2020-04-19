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
#[allow(unused_imports)]
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "RECORD";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 13);

pub type Context = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range8 {
    pub first: u8,
    pub last: u8,
}
impl TryParse for Range8 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (first, remaining) = u8::try_parse(remaining)?;
        let (last, remaining) = u8::try_parse(remaining)?;
        let result = Range8 { first, last };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Range8 {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Range8 {
    type Bytes = [u8; 2];
    fn serialize(&self) -> Self::Bytes {
        let first_bytes = self.first.serialize();
        let last_bytes = self.last.serialize();
        [
            first_bytes[0],
            last_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.first.serialize_into(bytes);
        self.last.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range16 {
    pub first: u16,
    pub last: u16,
}
impl TryParse for Range16 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (first, remaining) = u16::try_parse(remaining)?;
        let (last, remaining) = u16::try_parse(remaining)?;
        let result = Range16 { first, last };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Range16 {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Range16 {
    type Bytes = [u8; 4];
    fn serialize(&self) -> Self::Bytes {
        let first_bytes = self.first.serialize();
        let last_bytes = self.last.serialize();
        [
            first_bytes[0],
            first_bytes[1],
            last_bytes[0],
            last_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.first.serialize_into(bytes);
        self.last.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtRange {
    pub major: Range8,
    pub minor: Range16,
}
impl TryParse for ExtRange {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (major, remaining) = Range8::try_parse(remaining)?;
        let (minor, remaining) = Range16::try_parse(remaining)?;
        let result = ExtRange { major, minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ExtRange {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ExtRange {
    type Bytes = [u8; 6];
    fn serialize(&self) -> Self::Bytes {
        let major_bytes = self.major.serialize();
        let minor_bytes = self.minor.serialize();
        [
            major_bytes[0],
            major_bytes[1],
            minor_bytes[0],
            minor_bytes[1],
            minor_bytes[2],
            minor_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(6);
        self.major.serialize_into(bytes);
        self.minor.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub core_requests: Range8,
    pub core_replies: Range8,
    pub ext_requests: ExtRange,
    pub ext_replies: ExtRange,
    pub delivered_events: Range8,
    pub device_events: Range8,
    pub errors: Range8,
    pub client_started: bool,
    pub client_died: bool,
}
impl TryParse for Range {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (core_requests, remaining) = Range8::try_parse(remaining)?;
        let (core_replies, remaining) = Range8::try_parse(remaining)?;
        let (ext_requests, remaining) = ExtRange::try_parse(remaining)?;
        let (ext_replies, remaining) = ExtRange::try_parse(remaining)?;
        let (delivered_events, remaining) = Range8::try_parse(remaining)?;
        let (device_events, remaining) = Range8::try_parse(remaining)?;
        let (errors, remaining) = Range8::try_parse(remaining)?;
        let (client_started, remaining) = bool::try_parse(remaining)?;
        let (client_died, remaining) = bool::try_parse(remaining)?;
        let result = Range { core_requests, core_replies, ext_requests, ext_replies, delivered_events, device_events, errors, client_started, client_died };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Range {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Range {
    type Bytes = [u8; 24];
    fn serialize(&self) -> Self::Bytes {
        let core_requests_bytes = self.core_requests.serialize();
        let core_replies_bytes = self.core_replies.serialize();
        let ext_requests_bytes = self.ext_requests.serialize();
        let ext_replies_bytes = self.ext_replies.serialize();
        let delivered_events_bytes = self.delivered_events.serialize();
        let device_events_bytes = self.device_events.serialize();
        let errors_bytes = self.errors.serialize();
        let client_started_bytes = self.client_started.serialize();
        let client_died_bytes = self.client_died.serialize();
        [
            core_requests_bytes[0],
            core_requests_bytes[1],
            core_replies_bytes[0],
            core_replies_bytes[1],
            ext_requests_bytes[0],
            ext_requests_bytes[1],
            ext_requests_bytes[2],
            ext_requests_bytes[3],
            ext_requests_bytes[4],
            ext_requests_bytes[5],
            ext_replies_bytes[0],
            ext_replies_bytes[1],
            ext_replies_bytes[2],
            ext_replies_bytes[3],
            ext_replies_bytes[4],
            ext_replies_bytes[5],
            delivered_events_bytes[0],
            delivered_events_bytes[1],
            device_events_bytes[0],
            device_events_bytes[1],
            errors_bytes[0],
            errors_bytes[1],
            client_started_bytes[0],
            client_died_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.core_requests.serialize_into(bytes);
        self.core_replies.serialize_into(bytes);
        self.ext_requests.serialize_into(bytes);
        self.ext_replies.serialize_into(bytes);
        self.delivered_events.serialize_into(bytes);
        self.device_events.serialize_into(bytes);
        self.errors.serialize_into(bytes);
        self.client_started.serialize_into(bytes);
        self.client_died.serialize_into(bytes);
    }
}

pub type ElementHeader = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HType {
    FromServerTime = 1 << 0,
    FromClientTime = 1 << 1,
    FromClientSequence = 1 << 2,
}
impl From<HType> for u8 {
    fn from(input: HType) -> Self {
        match input {
            HType::FromServerTime => 1 << 0,
            HType::FromClientTime => 1 << 1,
            HType::FromClientSequence => 1 << 2,
        }
    }
}
impl From<HType> for Option<u8> {
    fn from(input: HType) -> Self {
        Some(u8::from(input))
    }
}
impl From<HType> for u16 {
    fn from(input: HType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<HType> for Option<u16> {
    fn from(input: HType) -> Self {
        Some(u16::from(input))
    }
}
impl From<HType> for u32 {
    fn from(input: HType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<HType> for Option<u32> {
    fn from(input: HType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for HType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HType::FromServerTime),
            2 => Ok(HType::FromClientTime),
            4 => Ok(HType::FromClientSequence),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for HType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for HType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(HType, u8);

pub type ClientSpec = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CS {
    CurrentClients = 1,
    FutureClients = 2,
    AllClients = 3,
}
impl From<CS> for u8 {
    fn from(input: CS) -> Self {
        match input {
            CS::CurrentClients => 1,
            CS::FutureClients => 2,
            CS::AllClients => 3,
        }
    }
}
impl From<CS> for Option<u8> {
    fn from(input: CS) -> Self {
        Some(u8::from(input))
    }
}
impl From<CS> for u16 {
    fn from(input: CS) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CS> for Option<u16> {
    fn from(input: CS) -> Self {
        Some(u16::from(input))
    }
}
impl From<CS> for u32 {
    fn from(input: CS) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CS> for Option<u32> {
    fn from(input: CS) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CS {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CS::CurrentClients),
            2 => Ok(CS::FutureClients),
            3 => Ok(CS::AllClients),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CS {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CS {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientInfo {
    pub client_resource: ClientSpec,
    pub ranges: Vec<Range>,
}
impl TryParse for ClientInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (client_resource, remaining) = ClientSpec::try_parse(remaining)?;
        let (num_ranges, remaining) = u32::try_parse(remaining)?;
        let (ranges, remaining) = crate::x11_utils::parse_list::<Range>(remaining, num_ranges as usize)?;
        let result = ClientInfo { client_resource, ranges };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ClientInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ClientInfo {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.client_resource.serialize_into(bytes);
        let num_ranges = self.ranges.len() as u32;
        num_ranges.serialize_into(bytes);
        self.ranges.serialize_into(bytes);
    }
}

/// Opcode for the BadContext error
pub const BAD_CONTEXT_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadContextError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub invalid_record: u32,
}
impl TryParse for BadContextError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (invalid_record, remaining) = u32::try_parse(remaining)?;
        let result = BadContextError { response_type, error_code, sequence, invalid_record };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadContextError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadContextError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadContextError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadContextError> for [u8; 32] {
    fn from(input: &BadContextError) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let error_code_bytes = input.error_code.serialize();
        let sequence_bytes = input.sequence.serialize();
        let invalid_record_bytes = input.invalid_record.serialize();
        [
            response_type_bytes[0],
            error_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            invalid_record_bytes[0],
            invalid_record_bytes[1],
            invalid_record_bytes[2],
            invalid_record_bytes[3],
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
impl From<BadContextError> for [u8; 32] {
    fn from(input: BadContextError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let major_version_bytes = major_version.serialize();
    let minor_version_bytes = minor_version.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        0,
        0,
        major_version_bytes[0],
        major_version_bytes[1],
        minor_version_bytes[0],
        minor_version_bytes[1],
    ];
    let length_so_far = length_so_far + request0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], vec![])?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, major_version, minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 1;
pub fn create_context<'c, Conn>(conn: &'c Conn, context: Context, element_header: ElementHeader, client_specs: &[ClientSpec], ranges: &[Range]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let element_header_bytes = element_header.serialize();
    let num_client_specs: u32 = client_specs.len().try_into()?;
    let num_client_specs_bytes = num_client_specs.serialize();
    let num_ranges: u32 = ranges.len().try_into()?;
    let num_ranges_bytes = num_ranges.serialize();
    let client_specs_bytes = client_specs.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        CREATE_CONTEXT_REQUEST,
        0,
        0,
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
        element_header_bytes[0],
        0,
        0,
        0,
        num_client_specs_bytes[0],
        num_client_specs_bytes[1],
        num_client_specs_bytes[2],
        num_client_specs_bytes[3],
        num_ranges_bytes[0],
        num_ranges_bytes[1],
        num_ranges_bytes[2],
        num_ranges_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    let length_so_far = length_so_far + client_specs_bytes.len();
    let ranges_bytes = ranges.serialize();
    let length_so_far = length_so_far + ranges_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&client_specs_bytes), IoSlice::new(&ranges_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the RegisterClients request
pub const REGISTER_CLIENTS_REQUEST: u8 = 2;
pub fn register_clients<'c, Conn>(conn: &'c Conn, context: Context, element_header: ElementHeader, client_specs: &[ClientSpec], ranges: &[Range]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let element_header_bytes = element_header.serialize();
    let num_client_specs: u32 = client_specs.len().try_into()?;
    let num_client_specs_bytes = num_client_specs.serialize();
    let num_ranges: u32 = ranges.len().try_into()?;
    let num_ranges_bytes = num_ranges.serialize();
    let client_specs_bytes = client_specs.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        REGISTER_CLIENTS_REQUEST,
        0,
        0,
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
        element_header_bytes[0],
        0,
        0,
        0,
        num_client_specs_bytes[0],
        num_client_specs_bytes[1],
        num_client_specs_bytes[2],
        num_client_specs_bytes[3],
        num_ranges_bytes[0],
        num_ranges_bytes[1],
        num_ranges_bytes[2],
        num_ranges_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    let length_so_far = length_so_far + client_specs_bytes.len();
    let ranges_bytes = ranges.serialize();
    let length_so_far = length_so_far + ranges_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&client_specs_bytes), IoSlice::new(&ranges_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the UnregisterClients request
pub const UNREGISTER_CLIENTS_REQUEST: u8 = 3;
pub fn unregister_clients<'c, Conn>(conn: &'c Conn, context: Context, client_specs: &[ClientSpec]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let num_client_specs: u32 = client_specs.len().try_into()?;
    let num_client_specs_bytes = num_client_specs.serialize();
    let client_specs_bytes = client_specs.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        UNREGISTER_CLIENTS_REQUEST,
        0,
        0,
        context_bytes[0],
        context_bytes[1],
        context_bytes[2],
        context_bytes[3],
        num_client_specs_bytes[0],
        num_client_specs_bytes[1],
        num_client_specs_bytes[2],
        num_client_specs_bytes[3],
    ];
    let length_so_far = length_so_far + request0.len();
    let length_so_far = length_so_far + client_specs_bytes.len();
    let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + padding0.len();
    assert_eq!(length_so_far % 4, 0);
    let length = u16::try_from(length_so_far / 4).unwrap_or(0);
    request0[2..4].copy_from_slice(&length.to_ne_bytes());
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&client_specs_bytes), IoSlice::new(&padding0)], vec![])?)
}

/// Opcode for the GetContext request
pub const GET_CONTEXT_REQUEST: u8 = 4;
pub fn get_context<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, GetContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        GET_CONTEXT_REQUEST,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetContextReply {
    pub response_type: u8,
    pub enabled: bool,
    pub sequence: u16,
    pub length: u32,
    pub element_header: ElementHeader,
    pub intercepted_clients: Vec<ClientInfo>,
}
impl TryParse for GetContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (enabled, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (element_header, remaining) = ElementHeader::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let (num_intercepted_clients, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (intercepted_clients, remaining) = crate::x11_utils::parse_list::<ClientInfo>(remaining, num_intercepted_clients as usize)?;
        let result = GetContextReply { response_type, enabled, sequence, length, element_header, intercepted_clients };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the EnableContext request
pub const ENABLE_CONTEXT_REQUEST: u8 = 5;
pub fn enable_context<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, EnableContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        ENABLE_CONTEXT_REQUEST,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnableContextReply {
    pub response_type: u8,
    pub category: u8,
    pub sequence: u16,
    pub element_header: ElementHeader,
    pub client_swapped: bool,
    pub xid_base: u32,
    pub server_time: u32,
    pub rec_sequence_num: u32,
    pub data: Vec<u8>,
}
impl TryParse for EnableContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (category, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (element_header, remaining) = ElementHeader::try_parse(remaining)?;
        let (client_swapped, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (xid_base, remaining) = u32::try_parse(remaining)?;
        let (server_time, remaining) = u32::try_parse(remaining)?;
        let (rec_sequence_num, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = EnableContextReply { response_type, category, sequence, element_header, client_swapped, xid_base, server_time, rec_sequence_num, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for EnableContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DisableContext request
pub const DISABLE_CONTEXT_REQUEST: u8 = 6;
pub fn disable_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        DISABLE_CONTEXT_REQUEST,
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

/// Opcode for the FreeContext request
pub const FREE_CONTEXT_REQUEST: u8 = 7;
pub fn free_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length_so_far = 0;
    let context_bytes = context.serialize();
    let mut request0 = [
        extension_information.major_opcode,
        FREE_CONTEXT_REQUEST,
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

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn record_query_version(&self, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }

    fn record_create_context<'c>(&'c self, context: Context, element_header: ElementHeader, client_specs: &[ClientSpec], ranges: &[Range]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_context(self, context, element_header, client_specs, ranges)
    }

    fn record_register_clients<'c>(&'c self, context: Context, element_header: ElementHeader, client_specs: &[ClientSpec], ranges: &[Range]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        register_clients(self, context, element_header, client_specs, ranges)
    }

    fn record_unregister_clients<'c>(&'c self, context: Context, client_specs: &[ClientSpec]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        unregister_clients(self, context, client_specs)
    }

    fn record_get_context(&self, context: Context) -> Result<Cookie<'_, Self, GetContextReply>, ConnectionError>
    {
        get_context(self, context)
    }

    fn record_enable_context(&self, context: Context) -> Result<Cookie<'_, Self, EnableContextReply>, ConnectionError>
    {
        enable_context(self, context)
    }

    fn record_disable_context(&self, context: Context) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        disable_context(self, context)
    }

    fn record_free_context(&self, context: Context) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_context(self, context)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
