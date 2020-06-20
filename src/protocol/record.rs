// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Record` X11 extension.

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
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};

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
    fn serialize(&self) -> [u8; 2] {
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
    fn serialize(&self) -> [u8; 4] {
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
    fn serialize(&self) -> [u8; 6] {
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
    fn serialize(&self) -> [u8; 24] {
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for HType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for HType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
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
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for CS {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for CS {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
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
        let (ranges, remaining) = crate::x11_utils::parse_list::<Range>(remaining, num_ranges.try_into().or(Err(ParseError::ConversionFailed))?)?;
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
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.client_resource.serialize_into(bytes);
        let num_ranges = u32::try_from(self.ranges.len()).expect("`ranges` has too many elements");
        num_ranges.serialize_into(bytes);
        self.ranges.serialize_into(bytes);
    }
}
impl ClientInfo {
    /// Get the value of the `num_ranges` field.
    ///
    /// The `num_ranges` field is used as the length field of the `ranges` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_ranges(&self) -> u32 {
        self.ranges.len()
            .try_into().unwrap()
    }
}

/// Opcode for the BadContext error
pub const BAD_CONTEXT_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadContextError {
    pub error_code: u8,
    pub sequence: u16,
    pub invalid_record: u32,
}
impl TryParse for BadContextError {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (invalid_record, remaining) = u32::try_parse(remaining)?;
        if response_type != 0 {
            return Err(ParseError::ParseError);
        }
        let result = BadContextError { error_code, sequence, invalid_record };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
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
        let response_type_bytes = &[0];
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub major_version: u16,
    pub minor_version: u16,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (major_version, remaining) = u16::try_parse(value)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            major_version,
            minor_version,
        })
    }
}
impl Request for QueryVersionRequest {
    type Reply = QueryVersionReply;
}
pub fn query_version<Conn>(conn: &Conn, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major_version,
        minor_version,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = QueryVersionReply { sequence, length, major_version, minor_version };
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

/// Opcode for the CreateContext request
pub const CREATE_CONTEXT_REQUEST: u8 = 1;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateContextRequest<'input> {
    pub context: Context,
    pub element_header: ElementHeader,
    pub client_specs: Cow<'input, [ClientSpec]>,
    pub ranges: Cow<'input, [Range]>,
}
impl<'input> CreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let element_header_bytes = self.element_header.serialize();
        let num_client_specs = u32::try_from(self.client_specs.len()).expect("`client_specs` has too many elements");
        let num_client_specs_bytes = num_client_specs.serialize();
        let num_ranges = u32::try_from(self.ranges.len()).expect("`ranges` has too many elements");
        let num_ranges_bytes = num_ranges.serialize();
        let mut request0 = vec![
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
        let client_specs_bytes = self.client_specs.serialize();
        let length_so_far = length_so_far + client_specs_bytes.len();
        let ranges_bytes = self.ranges.serialize();
        let length_so_far = length_so_far + ranges_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), client_specs_bytes.into(), ranges_bytes.into(), padding0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_CONTEXT_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let (element_header, remaining) = ElementHeader::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (num_client_specs, remaining) = u32::try_parse(remaining)?;
        let (num_ranges, remaining) = u32::try_parse(remaining)?;
        let (client_specs, remaining) = crate::x11_utils::parse_list::<ClientSpec>(remaining, num_client_specs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (ranges, remaining) = crate::x11_utils::parse_list::<Range>(remaining, num_ranges.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(CreateContextRequest {
            context,
            element_header,
            client_specs: Cow::Owned(client_specs),
            ranges: Cow::Owned(ranges),
        })
    }
    /// Clone all borrowed data in this CreateContextRequest.
    pub fn into_owned(self) -> CreateContextRequest<'static> {
        CreateContextRequest {
            context: self.context,
            element_header: self.element_header,
            client_specs: Cow::Owned(self.client_specs.into_owned()),
            ranges: Cow::Owned(self.ranges.into_owned()),
        }
    }
}
impl<'input> Request for CreateContextRequest<'input> {
    type Reply = ();
}
pub fn create_context<'c, 'input, Conn>(conn: &'c Conn, context: Context, element_header: ElementHeader, client_specs: &'input [ClientSpec], ranges: &'input [Range]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateContextRequest {
        context,
        element_header,
        client_specs: Cow::Borrowed(client_specs),
        ranges: Cow::Borrowed(ranges),
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the RegisterClients request
pub const REGISTER_CLIENTS_REQUEST: u8 = 2;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterClientsRequest<'input> {
    pub context: Context,
    pub element_header: ElementHeader,
    pub client_specs: Cow<'input, [ClientSpec]>,
    pub ranges: Cow<'input, [Range]>,
}
impl<'input> RegisterClientsRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let element_header_bytes = self.element_header.serialize();
        let num_client_specs = u32::try_from(self.client_specs.len()).expect("`client_specs` has too many elements");
        let num_client_specs_bytes = num_client_specs.serialize();
        let num_ranges = u32::try_from(self.ranges.len()).expect("`ranges` has too many elements");
        let num_ranges_bytes = num_ranges.serialize();
        let mut request0 = vec![
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
        let client_specs_bytes = self.client_specs.serialize();
        let length_so_far = length_so_far + client_specs_bytes.len();
        let ranges_bytes = self.ranges.serialize();
        let length_so_far = length_so_far + ranges_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), client_specs_bytes.into(), ranges_bytes.into(), padding0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != REGISTER_CLIENTS_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let (element_header, remaining) = ElementHeader::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (num_client_specs, remaining) = u32::try_parse(remaining)?;
        let (num_ranges, remaining) = u32::try_parse(remaining)?;
        let (client_specs, remaining) = crate::x11_utils::parse_list::<ClientSpec>(remaining, num_client_specs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let (ranges, remaining) = crate::x11_utils::parse_list::<Range>(remaining, num_ranges.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(RegisterClientsRequest {
            context,
            element_header,
            client_specs: Cow::Owned(client_specs),
            ranges: Cow::Owned(ranges),
        })
    }
    /// Clone all borrowed data in this RegisterClientsRequest.
    pub fn into_owned(self) -> RegisterClientsRequest<'static> {
        RegisterClientsRequest {
            context: self.context,
            element_header: self.element_header,
            client_specs: Cow::Owned(self.client_specs.into_owned()),
            ranges: Cow::Owned(self.ranges.into_owned()),
        }
    }
}
impl<'input> Request for RegisterClientsRequest<'input> {
    type Reply = ();
}
pub fn register_clients<'c, 'input, Conn>(conn: &'c Conn, context: Context, element_header: ElementHeader, client_specs: &'input [ClientSpec], ranges: &'input [Range]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = RegisterClientsRequest {
        context,
        element_header,
        client_specs: Cow::Borrowed(client_specs),
        ranges: Cow::Borrowed(ranges),
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the UnregisterClients request
pub const UNREGISTER_CLIENTS_REQUEST: u8 = 3;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnregisterClientsRequest<'input> {
    pub context: Context,
    pub client_specs: Cow<'input, [ClientSpec]>,
}
impl<'input> UnregisterClientsRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let num_client_specs = u32::try_from(self.client_specs.len()).expect("`client_specs` has too many elements");
        let num_client_specs_bytes = num_client_specs.serialize();
        let mut request0 = vec![
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
        let client_specs_bytes = self.client_specs.serialize();
        let length_so_far = length_so_far + client_specs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), client_specs_bytes.into(), padding0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != UNREGISTER_CLIENTS_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let (num_client_specs, remaining) = u32::try_parse(remaining)?;
        let (client_specs, remaining) = crate::x11_utils::parse_list::<ClientSpec>(remaining, num_client_specs.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let _ = remaining;
        Ok(UnregisterClientsRequest {
            context,
            client_specs: Cow::Owned(client_specs),
        })
    }
    /// Clone all borrowed data in this UnregisterClientsRequest.
    pub fn into_owned(self) -> UnregisterClientsRequest<'static> {
        UnregisterClientsRequest {
            context: self.context,
            client_specs: Cow::Owned(self.client_specs.into_owned()),
        }
    }
}
impl<'input> Request for UnregisterClientsRequest<'input> {
    type Reply = ();
}
pub fn unregister_clients<'c, 'input, Conn>(conn: &'c Conn, context: Context, client_specs: &'input [ClientSpec]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = UnregisterClientsRequest {
        context,
        client_specs: Cow::Borrowed(client_specs),
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the GetContext request
pub const GET_CONTEXT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContextRequest {
    pub context: Context,
}
impl GetContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CONTEXT_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let _ = remaining;
        Ok(GetContextRequest {
            context,
        })
    }
}
impl Request for GetContextRequest {
    type Reply = GetContextReply;
}
pub fn get_context<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, GetContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetContextReply {
    pub enabled: bool,
    pub sequence: u16,
    pub length: u32,
    pub element_header: ElementHeader,
    pub intercepted_clients: Vec<ClientInfo>,
}
impl TryParse for GetContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (enabled, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (element_header, remaining) = ElementHeader::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let (num_intercepted_clients, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::InsufficientData)?;
        let (intercepted_clients, remaining) = crate::x11_utils::parse_list::<ClientInfo>(remaining, num_intercepted_clients.try_into().or(Err(ParseError::ConversionFailed))?)?;
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = GetContextReply { enabled, sequence, length, element_header, intercepted_clients };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetContextReply {
    /// Get the value of the `num_intercepted_clients` field.
    ///
    /// The `num_intercepted_clients` field is used as the length field of the `intercepted_clients` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_intercepted_clients(&self) -> u32 {
        self.intercepted_clients.len()
            .try_into().unwrap()
    }
}

/// Opcode for the EnableContext request
pub const ENABLE_CONTEXT_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableContextRequest {
    pub context: Context,
}
impl EnableContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != ENABLE_CONTEXT_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let _ = remaining;
        Ok(EnableContextRequest {
            context,
        })
    }
}
impl Request for EnableContextRequest {
    type Reply = EnableContextReply;
}
pub fn enable_context<Conn>(conn: &Conn, context: Context) -> Result<Cookie<'_, Conn, EnableContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = EnableContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnableContextReply {
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
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (category, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (element_header, remaining) = ElementHeader::try_parse(remaining)?;
        let (client_swapped, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (xid_base, remaining) = u32::try_parse(remaining)?;
        let (server_time, remaining) = u32::try_parse(remaining)?;
        let (rec_sequence_num, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::InsufficientData)?;
        let (data, remaining) = crate::x11_utils::parse_u8_list(remaining, length.checked_mul(4u32).ok_or(ParseError::ParseError)?.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let data = data.to_vec();
        if response_type != 1 {
            return Err(ParseError::ParseError);
        }
        let result = EnableContextReply { category, sequence, element_header, client_swapped, xid_base, server_time, rec_sequence_num, data };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for EnableContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl EnableContextReply {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `data` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.data.len()
            .checked_div(4).unwrap()
            .try_into().unwrap()
    }
}

/// Opcode for the DisableContext request
pub const DISABLE_CONTEXT_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableContextRequest {
    pub context: Context,
}
impl DisableContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DISABLE_CONTEXT_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let _ = remaining;
        Ok(DisableContextRequest {
            context,
        })
    }
}
impl Request for DisableContextRequest {
    type Reply = ();
}
pub fn disable_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DisableContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Opcode for the FreeContext request
pub const FREE_CONTEXT_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeContextRequest {
    pub context: Context,
}
impl FreeContextRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_bytes = self.context.serialize();
        let mut request0 = vec![
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
        Ok((vec![request0.into()], vec![]))
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != FREE_CONTEXT_REQUEST {
            return Err(ParseError::ParseError);
        }
        let (context, remaining) = Context::try_parse(value)?;
        let _ = remaining;
        Ok(FreeContextRequest {
            context,
        })
    }
}
impl Request for FreeContextRequest {
    type Reply = ();
}
pub fn free_context<Conn>(conn: &Conn, context: Context) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FreeContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn record_query_version(&self, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }
    fn record_create_context<'c, 'input>(&'c self, context: Context, element_header: ElementHeader, client_specs: &'input [ClientSpec], ranges: &'input [Range]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_context(self, context, element_header, client_specs, ranges)
    }
    fn record_register_clients<'c, 'input>(&'c self, context: Context, element_header: ElementHeader, client_specs: &'input [ClientSpec], ranges: &'input [Range]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        register_clients(self, context, element_header, client_specs, ranges)
    }
    fn record_unregister_clients<'c, 'input>(&'c self, context: Context, client_specs: &'input [ClientSpec]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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
