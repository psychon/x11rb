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
pub const X11_EXTENSION_NAME: &str = "X-Resource";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Client {
    pub resource_base: u32,
    pub resource_mask: u32,
}
impl TryParse for Client {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (resource_base, remaining) = u32::try_parse(remaining)?;
        let (resource_mask, remaining) = u32::try_parse(remaining)?;
        let result = Client { resource_base, resource_mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Client {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Client {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let resource_base_bytes = self.resource_base.serialize();
        let resource_mask_bytes = self.resource_mask.serialize();
        [
            resource_base_bytes[0],
            resource_base_bytes[1],
            resource_base_bytes[2],
            resource_base_bytes[3],
            resource_mask_bytes[0],
            resource_mask_bytes[1],
            resource_mask_bytes[2],
            resource_mask_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.resource_base.serialize_into(bytes);
        self.resource_mask.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Type {
    pub resource_type: xproto::Atom,
    pub count: u32,
}
impl TryParse for Type {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (resource_type, remaining) = xproto::Atom::try_parse(remaining)?;
        let (count, remaining) = u32::try_parse(remaining)?;
        let result = Type { resource_type, count };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Type {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Type {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let resource_type_bytes = self.resource_type.serialize();
        let count_bytes = self.count.serialize();
        [
            resource_type_bytes[0],
            resource_type_bytes[1],
            resource_type_bytes[2],
            resource_type_bytes[3],
            count_bytes[0],
            count_bytes[1],
            count_bytes[2],
            count_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.resource_type.serialize_into(bytes);
        self.count.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClientIdMask {
    ClientXID = 1 << 0,
    LocalClientPID = 1 << 1,
}
impl From<ClientIdMask> for u8 {
    fn from(input: ClientIdMask) -> Self {
        match input {
            ClientIdMask::ClientXID => 1 << 0,
            ClientIdMask::LocalClientPID => 1 << 1,
        }
    }
}
impl From<ClientIdMask> for Option<u8> {
    fn from(input: ClientIdMask) -> Self {
        Some(u8::from(input))
    }
}
impl From<ClientIdMask> for u16 {
    fn from(input: ClientIdMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ClientIdMask> for Option<u16> {
    fn from(input: ClientIdMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<ClientIdMask> for u32 {
    fn from(input: ClientIdMask) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ClientIdMask> for Option<u32> {
    fn from(input: ClientIdMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ClientIdMask {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ClientIdMask::ClientXID),
            2 => Ok(ClientIdMask::LocalClientPID),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ClientIdMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ClientIdMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ClientIdMask, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientIdSpec {
    pub client: u32,
    pub mask: u32,
}
impl TryParse for ClientIdSpec {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (client, remaining) = u32::try_parse(remaining)?;
        let (mask, remaining) = u32::try_parse(remaining)?;
        let result = ClientIdSpec { client, mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ClientIdSpec {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ClientIdSpec {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let client_bytes = self.client.serialize();
        let mask_bytes = self.mask.serialize();
        [
            client_bytes[0],
            client_bytes[1],
            client_bytes[2],
            client_bytes[3],
            mask_bytes[0],
            mask_bytes[1],
            mask_bytes[2],
            mask_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.client.serialize_into(bytes);
        self.mask.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientIdValue {
    pub spec: ClientIdSpec,
    pub value: Vec<u32>,
}
impl TryParse for ClientIdValue {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (spec, remaining) = ClientIdSpec::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (value, remaining) = crate::x11_utils::parse_list::<u32>(remaining, (length as usize) / (4))?;
        let result = ClientIdValue { spec, value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ClientIdValue {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ClientIdValue {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.spec.serialize_into(bytes);
        let length = self.value.len() as u32;
        let length = length * 4;
        length.serialize_into(bytes);
        self.value.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceIdSpec {
    pub resource: u32,
    pub type_: u32,
}
impl TryParse for ResourceIdSpec {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (resource, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = u32::try_parse(remaining)?;
        let result = ResourceIdSpec { resource, type_ };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ResourceIdSpec {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ResourceIdSpec {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let resource_bytes = self.resource.serialize();
        let type_bytes = self.type_.serialize();
        [
            resource_bytes[0],
            resource_bytes[1],
            resource_bytes[2],
            resource_bytes[3],
            type_bytes[0],
            type_bytes[1],
            type_bytes[2],
            type_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.resource.serialize_into(bytes);
        self.type_.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceSizeSpec {
    pub spec: ResourceIdSpec,
    pub bytes: u32,
    pub ref_count: u32,
    pub use_count: u32,
}
impl TryParse for ResourceSizeSpec {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (spec, remaining) = ResourceIdSpec::try_parse(remaining)?;
        let (bytes, remaining) = u32::try_parse(remaining)?;
        let (ref_count, remaining) = u32::try_parse(remaining)?;
        let (use_count, remaining) = u32::try_parse(remaining)?;
        let result = ResourceSizeSpec { spec, bytes, ref_count, use_count };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ResourceSizeSpec {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ResourceSizeSpec {
    type Bytes = [u8; 20];
    fn serialize(&self) -> Self::Bytes {
        let spec_bytes = self.spec.serialize();
        let bytes_bytes = self.bytes.serialize();
        let ref_count_bytes = self.ref_count.serialize();
        let use_count_bytes = self.use_count.serialize();
        [
            spec_bytes[0],
            spec_bytes[1],
            spec_bytes[2],
            spec_bytes[3],
            spec_bytes[4],
            spec_bytes[5],
            spec_bytes[6],
            spec_bytes[7],
            bytes_bytes[0],
            bytes_bytes[1],
            bytes_bytes[2],
            bytes_bytes[3],
            ref_count_bytes[0],
            ref_count_bytes[1],
            ref_count_bytes[2],
            ref_count_bytes[3],
            use_count_bytes[0],
            use_count_bytes[1],
            use_count_bytes[2],
            use_count_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(20);
        self.spec.serialize_into(bytes);
        self.bytes.serialize_into(bytes);
        self.ref_count.serialize_into(bytes);
        self.use_count.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceSizeValue {
    pub size: ResourceSizeSpec,
    pub cross_references: Vec<ResourceSizeSpec>,
}
impl TryParse for ResourceSizeValue {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (size, remaining) = ResourceSizeSpec::try_parse(remaining)?;
        let (num_cross_references, remaining) = u32::try_parse(remaining)?;
        let (cross_references, remaining) = crate::x11_utils::parse_list::<ResourceSizeSpec>(remaining, num_cross_references as usize)?;
        let result = ResourceSizeValue { size, cross_references };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ResourceSizeValue {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ResourceSizeValue {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.size.serialize_into(bytes);
        let num_cross_references = self.cross_references.len() as u32;
        num_cross_references.serialize_into(bytes);
        self.cross_references.serialize_into(bytes);
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let client_major_bytes = client_major.serialize();
    let client_minor_bytes = client_minor.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        client_major_bytes[0],
        client_minor_bytes[0],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, server_major, server_minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryClients request
pub const QUERY_CLIENTS_REQUEST: u8 = 1;
pub fn query_clients<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryClientsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_CLIENTS_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryClientsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub clients: Vec<Client>,
}
impl QueryClientsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_clients, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (clients, remaining) = crate::x11_utils::parse_list::<Client>(remaining, num_clients as usize)?;
        let result = QueryClientsReply { response_type, sequence, length, clients };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryClientsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryClientResources request
pub const QUERY_CLIENT_RESOURCES_REQUEST: u8 = 2;
pub fn query_client_resources<Conn>(conn: &Conn, xid: u32) -> Result<Cookie<'_, Conn, QueryClientResourcesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let xid_bytes = xid.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_CLIENT_RESOURCES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        xid_bytes[0],
        xid_bytes[1],
        xid_bytes[2],
        xid_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryClientResourcesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub types: Vec<Type>,
}
impl QueryClientResourcesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_types, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (types, remaining) = crate::x11_utils::parse_list::<Type>(remaining, num_types as usize)?;
        let result = QueryClientResourcesReply { response_type, sequence, length, types };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryClientResourcesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryClientPixmapBytes request
pub const QUERY_CLIENT_PIXMAP_BYTES_REQUEST: u8 = 3;
pub fn query_client_pixmap_bytes<Conn>(conn: &Conn, xid: u32) -> Result<Cookie<'_, Conn, QueryClientPixmapBytesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let xid_bytes = xid.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_CLIENT_PIXMAP_BYTES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        xid_bytes[0],
        xid_bytes[1],
        xid_bytes[2],
        xid_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryClientPixmapBytesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub bytes: u32,
    pub bytes_overflow: u32,
}
impl QueryClientPixmapBytesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (bytes, remaining) = u32::try_parse(remaining)?;
        let (bytes_overflow, remaining) = u32::try_parse(remaining)?;
        let result = QueryClientPixmapBytesReply { response_type, sequence, length, bytes, bytes_overflow };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryClientPixmapBytesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryClientIds request
pub const QUERY_CLIENT_IDS_REQUEST: u8 = 4;
pub fn query_client_ids<'c, Conn>(conn: &'c Conn, specs: &[ClientIdSpec]) -> Result<Cookie<'c, Conn, QueryClientIdsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 8 * specs.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let num_specs: u32 = specs.len().try_into()?;
    let num_specs_bytes = num_specs.serialize();
    let specs_bytes = specs.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_CLIENT_IDS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        num_specs_bytes[0],
        num_specs_bytes[1],
        num_specs_bytes[2],
        num_specs_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&specs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&specs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryClientIdsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ids: Vec<ClientIdValue>,
}
impl QueryClientIdsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_ids, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (ids, remaining) = crate::x11_utils::parse_list::<ClientIdValue>(remaining, num_ids as usize)?;
        let result = QueryClientIdsReply { response_type, sequence, length, ids };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryClientIdsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryResourceBytes request
pub const QUERY_RESOURCE_BYTES_REQUEST: u8 = 5;
pub fn query_resource_bytes<'c, Conn>(conn: &'c Conn, client: u32, specs: &[ResourceIdSpec]) -> Result<Cookie<'c, Conn, QueryResourceBytesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12 + 8 * specs.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let client_bytes = client.serialize();
    let num_specs: u32 = specs.len().try_into()?;
    let num_specs_bytes = num_specs.serialize();
    let specs_bytes = specs.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_RESOURCE_BYTES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        client_bytes[0],
        client_bytes[1],
        client_bytes[2],
        client_bytes[3],
        num_specs_bytes[0],
        num_specs_bytes[1],
        num_specs_bytes[2],
        num_specs_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&specs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&specs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryResourceBytesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub sizes: Vec<ResourceSizeValue>,
}
impl QueryResourceBytesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_sizes, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (sizes, remaining) = crate::x11_utils::parse_list::<ResourceSizeValue>(remaining, num_sizes as usize)?;
        let result = QueryResourceBytesReply { response_type, sequence, length, sizes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryResourceBytesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn res_query_version(&self, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major, client_minor)
    }

    fn res_query_clients(&self) -> Result<Cookie<'_, Self, QueryClientsReply>, ConnectionError>
    {
        query_clients(self)
    }

    fn res_query_client_resources(&self, xid: u32) -> Result<Cookie<'_, Self, QueryClientResourcesReply>, ConnectionError>
    {
        query_client_resources(self, xid)
    }

    fn res_query_client_pixmap_bytes(&self, xid: u32) -> Result<Cookie<'_, Self, QueryClientPixmapBytesReply>, ConnectionError>
    {
        query_client_pixmap_bytes(self, xid)
    }

    fn res_query_client_ids<'c>(&'c self, specs: &[ClientIdSpec]) -> Result<Cookie<'c, Self, QueryClientIdsReply>, ConnectionError>
    {
        query_client_ids(self, specs)
    }

    fn res_query_resource_bytes<'c>(&'c self, client: u32, specs: &[ResourceIdSpec]) -> Result<Cookie<'c, Self, QueryResourceBytesReply>, ConnectionError>
    {
        query_resource_bytes(self, client, specs)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
