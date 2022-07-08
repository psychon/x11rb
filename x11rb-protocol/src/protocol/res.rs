// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Res` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use alloc::borrow::Cow;
#[allow(unused_imports)]
use core::convert::TryInto;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
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
pub const X11_EXTENSION_NAME: &str = "X-Resource";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 2);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for Client {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod client {
    #![allow(dead_code, unused_imports)]
    use super::Client;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for Client {
        fn generate(rng: &Rng) -> Self {
            let resource_base: u32 = GenRandom::generate(rng);
            let resource_mask: u32 = GenRandom::generate(rng);
            Self {
                resource_base,
                resource_mask,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(979174072800);
        let value = Client::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for Type {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod type_ {
    #![allow(dead_code, unused_imports)]
    use super::Type;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for Type {
        fn generate(rng: &Rng) -> Self {
            let resource_type = GenRandom::generate(rng);
            let count: u32 = GenRandom::generate(rng);
            Self {
                resource_type,
                count,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(114975168);
        let value = Type::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClientIdMask(u8);
impl ClientIdMask {
    pub const CLIENT_XID: Self = Self(1 << 0);
    pub const LOCAL_CLIENT_PID: Self = Self(1 << 1);
}
impl From<ClientIdMask> for u8 {
    #[inline]
    fn from(input: ClientIdMask) -> Self {
        input.0
    }
}
impl From<ClientIdMask> for Option<u8> {
    #[inline]
    fn from(input: ClientIdMask) -> Self {
        Some(input.0)
    }
}
impl From<ClientIdMask> for u16 {
    #[inline]
    fn from(input: ClientIdMask) -> Self {
        u16::from(input.0)
    }
}
impl From<ClientIdMask> for Option<u16> {
    #[inline]
    fn from(input: ClientIdMask) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ClientIdMask> for u32 {
    #[inline]
    fn from(input: ClientIdMask) -> Self {
        u32::from(input.0)
    }
}
impl From<ClientIdMask> for Option<u32> {
    #[inline]
    fn from(input: ClientIdMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ClientIdMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl core::fmt::Debug for ClientIdMask  {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let variants = [
            (Self::CLIENT_XID.0.into(), "CLIENT_XID", "ClientXID"),
            (Self::LOCAL_CLIENT_PID.0.into(), "LOCAL_CLIENT_PID", "LocalClientPID"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(ClientIdMask, u8);
#[cfg(test)]
impl crate::x11_utils::GenRandom for ClientIdMask {
    fn generate(rng: &fastrand::Rng) -> Self {
        let possible_values = &[
            Self::CLIENT_XID.0,
            Self::LOCAL_CLIENT_PID.0,
        ];
        let index = rng.usize(..possible_values.len());
        Self(possible_values[index])
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for ClientIdSpec {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod client_id_spec {
    #![allow(dead_code, unused_imports)]
    use super::ClientIdSpec;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ClientIdSpec {
        fn generate(rng: &Rng) -> Self {
            let client: u32 = GenRandom::generate(rng);
            let mask = GenRandom::generate(rng);
            Self {
                client,
                mask,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12530355946013206528);
        let value = ClientIdSpec::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClientIdValue {
    pub spec: ClientIdSpec,
    pub value: Vec<u32>,
}
impl TryParse for ClientIdValue {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (spec, remaining) = ClientIdSpec::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (value, remaining) = crate::x11_utils::parse_list::<u32>(remaining, length.checked_div(4u32).ok_or(ParseError::InvalidExpression)?.try_to_usize()?)?;
        let result = ClientIdValue { spec, value };
        Ok((result, remaining))
    }
}
impl Serialize for ClientIdValue {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.spec.serialize_into(bytes);
        let length = u32::try_from(self.value.len()).ok().and_then(|len| len.checked_mul(4)).expect("`value` has too many elements");
        length.serialize_into(bytes);
        self.value.serialize_into(bytes);
    }
}
impl ClientIdValue {
    /// Get the value of the `length` field.
    ///
    /// The `length` field is used as the length field of the `value` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn length(&self) -> u32 {
        self.value.len()
            .checked_mul(4).unwrap()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod client_id_value {
    #![allow(dead_code, unused_imports)]
    use super::ClientIdValue;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ClientIdValue {
        fn generate(rng: &Rng) -> Self {
            let length = u32::from(rng.u8(..16));
            let spec = GenRandom::generate(rng);
            let value: Vec<u32> = gen_random_list(rng, usize::try_from(length.checked_div(4u32).unwrap()).unwrap());
            Self {
                spec,
                value,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(8805440661720562688);
        let value = ClientIdValue::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for ResourceIdSpec {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
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
#[cfg(test)]
mod resource_id_spec {
    #![allow(dead_code, unused_imports)]
    use super::ResourceIdSpec;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ResourceIdSpec {
        fn generate(rng: &Rng) -> Self {
            let resource: u32 = GenRandom::generate(rng);
            let type_: u32 = GenRandom::generate(rng);
            Self {
                resource,
                type_,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(15530378358007093504);
        let value = ResourceIdSpec::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl Serialize for ResourceSizeSpec {
    type Bytes = [u8; 20];
    fn serialize(&self) -> [u8; 20] {
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
#[cfg(test)]
mod resource_size_spec {
    #![allow(dead_code, unused_imports)]
    use super::ResourceSizeSpec;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ResourceSizeSpec {
        fn generate(rng: &Rng) -> Self {
            let spec = GenRandom::generate(rng);
            let bytes: u32 = GenRandom::generate(rng);
            let ref_count: u32 = GenRandom::generate(rng);
            let use_count: u32 = GenRandom::generate(rng);
            Self {
                spec,
                bytes,
                ref_count,
                use_count,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(10417732137083490688);
        let value = ResourceSizeSpec::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourceSizeValue {
    pub size: ResourceSizeSpec,
    pub cross_references: Vec<ResourceSizeSpec>,
}
impl TryParse for ResourceSizeValue {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (size, remaining) = ResourceSizeSpec::try_parse(remaining)?;
        let (num_cross_references, remaining) = u32::try_parse(remaining)?;
        let (cross_references, remaining) = crate::x11_utils::parse_list::<ResourceSizeSpec>(remaining, num_cross_references.try_to_usize()?)?;
        let result = ResourceSizeValue { size, cross_references };
        Ok((result, remaining))
    }
}
impl Serialize for ResourceSizeValue {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.size.serialize_into(bytes);
        let num_cross_references = u32::try_from(self.cross_references.len()).expect("`cross_references` has too many elements");
        num_cross_references.serialize_into(bytes);
        self.cross_references.serialize_into(bytes);
    }
}
impl ResourceSizeValue {
    /// Get the value of the `num_cross_references` field.
    ///
    /// The `num_cross_references` field is used as the length field of the `cross_references` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_cross_references(&self) -> u32 {
        self.cross_references.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod resource_size_value {
    #![allow(dead_code, unused_imports)]
    use super::ResourceSizeValue;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ResourceSizeValue {
        fn generate(rng: &Rng) -> Self {
            let num_cross_references = u32::from(rng.u8(..16));
            let size = GenRandom::generate(rng);
            let cross_references = gen_random_list(rng, usize::try_from(num_cross_references).unwrap());
            Self {
                size,
                cross_references,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(18258518244292863552);
        let value = ResourceSizeValue::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryVersionRequest {
    pub client_major: u8,
    pub client_minor: u8,
}
impl QueryVersionRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let client_major_bytes = self.client_major.serialize();
        let client_minor_bytes = self.client_minor.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_VERSION_REQUEST,
            0,
            0,
            client_major_bytes[0],
            client_minor_bytes[0],
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
        if header.minor_opcode != QUERY_VERSION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (client_major, remaining) = u8::try_parse(value)?;
        let (client_minor, remaining) = u8::try_parse(remaining)?;
        let _ = remaining;
        Ok(QueryVersionRequest {
            client_major,
            client_minor,
        })
    }
}
impl Request for QueryVersionRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryVersionRequest {
    type Reply = QueryVersionReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryVersionReply {
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryVersionReply { sequence, length, server_major, server_minor };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryVersionReply {
    type Bytes = [u8; 12];
    fn serialize(&self) -> [u8; 12] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let server_major_bytes = self.server_major.serialize();
        let server_minor_bytes = self.server_minor.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            server_major_bytes[0],
            server_major_bytes[1],
            server_minor_bytes[0],
            server_minor_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.server_major.serialize_into(bytes);
        self.server_minor.serialize_into(bytes);
    }
}
#[cfg(test)]
mod query_version_reply {
    #![allow(dead_code, unused_imports)]
    use super::QueryVersionReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryVersionReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let server_major: u16 = GenRandom::generate(rng);
            let server_minor: u16 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                server_major,
                server_minor,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(228006558612858880);
        let value = QueryVersionReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryClients request
pub const QUERY_CLIENTS_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryClientsRequest;
impl QueryClientsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            QUERY_CLIENTS_REQUEST,
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
        if header.minor_opcode != QUERY_CLIENTS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(QueryClientsRequest
        )
    }
}
impl Request for QueryClientsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryClientsRequest {
    type Reply = QueryClientsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryClientsReply {
    pub sequence: u16,
    pub length: u32,
    pub clients: Vec<Client>,
}
impl TryParse for QueryClientsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_clients, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (clients, remaining) = crate::x11_utils::parse_list::<Client>(remaining, num_clients.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryClientsReply { sequence, length, clients };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryClientsReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_clients = u32::try_from(self.clients.len()).expect("`clients` has too many elements");
        num_clients.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.clients.serialize_into(bytes);
    }
}
impl QueryClientsReply {
    /// Get the value of the `num_clients` field.
    ///
    /// The `num_clients` field is used as the length field of the `clients` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_clients(&self) -> u32 {
        self.clients.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod query_clients_reply {
    #![allow(dead_code, unused_imports)]
    use super::QueryClientsReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryClientsReply {
        fn generate(rng: &Rng) -> Self {
            let num_clients = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let clients = gen_random_list(rng, usize::try_from(num_clients).unwrap());
            Self {
                sequence,
                length,
                clients,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11341914187745370112);
        let value = QueryClientsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryClientResources request
pub const QUERY_CLIENT_RESOURCES_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryClientResourcesRequest {
    pub xid: u32,
}
impl QueryClientResourcesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let xid_bytes = self.xid.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_CLIENT_RESOURCES_REQUEST,
            0,
            0,
            xid_bytes[0],
            xid_bytes[1],
            xid_bytes[2],
            xid_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_CLIENT_RESOURCES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (xid, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(QueryClientResourcesRequest {
            xid,
        })
    }
}
impl Request for QueryClientResourcesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryClientResourcesRequest {
    type Reply = QueryClientResourcesReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryClientResourcesReply {
    pub sequence: u16,
    pub length: u32,
    pub types: Vec<Type>,
}
impl TryParse for QueryClientResourcesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_types, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (types, remaining) = crate::x11_utils::parse_list::<Type>(remaining, num_types.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryClientResourcesReply { sequence, length, types };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryClientResourcesReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_types = u32::try_from(self.types.len()).expect("`types` has too many elements");
        num_types.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.types.serialize_into(bytes);
    }
}
impl QueryClientResourcesReply {
    /// Get the value of the `num_types` field.
    ///
    /// The `num_types` field is used as the length field of the `types` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_types(&self) -> u32 {
        self.types.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod query_client_resources_reply {
    #![allow(dead_code, unused_imports)]
    use super::QueryClientResourcesReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryClientResourcesReply {
        fn generate(rng: &Rng) -> Self {
            let num_types = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let types = gen_random_list(rng, usize::try_from(num_types).unwrap());
            Self {
                sequence,
                length,
                types,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(12134460664815255552);
        let value = QueryClientResourcesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryClientPixmapBytes request
pub const QUERY_CLIENT_PIXMAP_BYTES_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryClientPixmapBytesRequest {
    pub xid: u32,
}
impl QueryClientPixmapBytesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let xid_bytes = self.xid.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_CLIENT_PIXMAP_BYTES_REQUEST,
            0,
            0,
            xid_bytes[0],
            xid_bytes[1],
            xid_bytes[2],
            xid_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_CLIENT_PIXMAP_BYTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (xid, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(QueryClientPixmapBytesRequest {
            xid,
        })
    }
}
impl Request for QueryClientPixmapBytesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for QueryClientPixmapBytesRequest {
    type Reply = QueryClientPixmapBytesReply;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryClientPixmapBytesReply {
    pub sequence: u16,
    pub length: u32,
    pub bytes: u32,
    pub bytes_overflow: u32,
}
impl TryParse for QueryClientPixmapBytesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (bytes, remaining) = u32::try_parse(remaining)?;
        let (bytes_overflow, remaining) = u32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryClientPixmapBytesReply { sequence, length, bytes, bytes_overflow };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryClientPixmapBytesReply {
    type Bytes = [u8; 16];
    fn serialize(&self) -> [u8; 16] {
        let response_type_bytes = &[1];
        let sequence_bytes = self.sequence.serialize();
        let length_bytes = self.length.serialize();
        let bytes_bytes = self.bytes.serialize();
        let bytes_overflow_bytes = self.bytes_overflow.serialize();
        [
            response_type_bytes[0],
            0,
            sequence_bytes[0],
            sequence_bytes[1],
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
            bytes_bytes[0],
            bytes_bytes[1],
            bytes_bytes[2],
            bytes_bytes[3],
            bytes_overflow_bytes[0],
            bytes_overflow_bytes[1],
            bytes_overflow_bytes[2],
            bytes_overflow_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(16);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.bytes.serialize_into(bytes);
        self.bytes_overflow.serialize_into(bytes);
    }
}
#[cfg(test)]
mod query_client_pixmap_bytes_reply {
    #![allow(dead_code, unused_imports)]
    use super::QueryClientPixmapBytesReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryClientPixmapBytesReply {
        fn generate(rng: &Rng) -> Self {
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let bytes: u32 = GenRandom::generate(rng);
            let bytes_overflow: u32 = GenRandom::generate(rng);
            Self {
                sequence,
                length,
                bytes,
                bytes_overflow,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5089116049379426304);
        let value = QueryClientPixmapBytesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryClientIds request
pub const QUERY_CLIENT_IDS_REQUEST: u8 = 4;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryClientIdsRequest<'input> {
    pub specs: Cow<'input, [ClientIdSpec]>,
}
impl<'input> QueryClientIdsRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let num_specs = u32::try_from(self.specs.len()).expect("`specs` has too many elements");
        let num_specs_bytes = num_specs.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_CLIENT_IDS_REQUEST,
            0,
            0,
            num_specs_bytes[0],
            num_specs_bytes[1],
            num_specs_bytes[2],
            num_specs_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let specs_bytes = self.specs.serialize();
        let length_so_far = length_so_far + specs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), specs_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_CLIENT_IDS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (num_specs, remaining) = u32::try_parse(value)?;
        let (specs, remaining) = crate::x11_utils::parse_list::<ClientIdSpec>(remaining, num_specs.try_to_usize()?)?;
        let _ = remaining;
        Ok(QueryClientIdsRequest {
            specs: Cow::Owned(specs),
        })
    }
    /// Clone all borrowed data in this QueryClientIdsRequest.
    pub fn into_owned(self) -> QueryClientIdsRequest<'static> {
        QueryClientIdsRequest {
            specs: Cow::Owned(self.specs.into_owned()),
        }
    }
}
impl<'input> Request for QueryClientIdsRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for QueryClientIdsRequest<'input> {
    type Reply = QueryClientIdsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryClientIdsReply {
    pub sequence: u16,
    pub length: u32,
    pub ids: Vec<ClientIdValue>,
}
impl TryParse for QueryClientIdsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_ids, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (ids, remaining) = crate::x11_utils::parse_list::<ClientIdValue>(remaining, num_ids.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryClientIdsReply { sequence, length, ids };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryClientIdsReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_ids = u32::try_from(self.ids.len()).expect("`ids` has too many elements");
        num_ids.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.ids.serialize_into(bytes);
    }
}
impl QueryClientIdsReply {
    /// Get the value of the `num_ids` field.
    ///
    /// The `num_ids` field is used as the length field of the `ids` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_ids(&self) -> u32 {
        self.ids.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod query_client_ids_reply {
    #![allow(dead_code, unused_imports)]
    use super::QueryClientIdsReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryClientIdsReply {
        fn generate(rng: &Rng) -> Self {
            let num_ids = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let ids = gen_random_list(rng, usize::try_from(num_ids).unwrap());
            Self {
                sequence,
                length,
                ids,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6986167732734164992);
        let value = QueryClientIdsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the QueryResourceBytes request
pub const QUERY_RESOURCE_BYTES_REQUEST: u8 = 5;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryResourceBytesRequest<'input> {
    pub client: u32,
    pub specs: Cow<'input, [ResourceIdSpec]>,
}
impl<'input> QueryResourceBytesRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let client_bytes = self.client.serialize();
        let num_specs = u32::try_from(self.specs.len()).expect("`specs` has too many elements");
        let num_specs_bytes = num_specs.serialize();
        let mut request0 = vec![
            major_opcode,
            QUERY_RESOURCE_BYTES_REQUEST,
            0,
            0,
            client_bytes[0],
            client_bytes[1],
            client_bytes[2],
            client_bytes[3],
            num_specs_bytes[0],
            num_specs_bytes[1],
            num_specs_bytes[2],
            num_specs_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let specs_bytes = self.specs.serialize();
        let length_so_far = length_so_far + specs_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), specs_bytes.into(), padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_RESOURCE_BYTES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (client, remaining) = u32::try_parse(value)?;
        let (num_specs, remaining) = u32::try_parse(remaining)?;
        let (specs, remaining) = crate::x11_utils::parse_list::<ResourceIdSpec>(remaining, num_specs.try_to_usize()?)?;
        let _ = remaining;
        Ok(QueryResourceBytesRequest {
            client,
            specs: Cow::Owned(specs),
        })
    }
    /// Clone all borrowed data in this QueryResourceBytesRequest.
    pub fn into_owned(self) -> QueryResourceBytesRequest<'static> {
        QueryResourceBytesRequest {
            client: self.client,
            specs: Cow::Owned(self.specs.into_owned()),
        }
    }
}
impl<'input> Request for QueryResourceBytesRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::ReplyRequest for QueryResourceBytesRequest<'input> {
    type Reply = QueryResourceBytesReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryResourceBytesReply {
    pub sequence: u16,
    pub length: u32,
    pub sizes: Vec<ResourceSizeValue>,
}
impl TryParse for QueryResourceBytesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_sizes, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (sizes, remaining) = crate::x11_utils::parse_list::<ResourceSizeValue>(remaining, num_sizes.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryResourceBytesReply { sequence, length, sizes };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for QueryResourceBytesReply {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(32);
        let response_type_bytes = &[1];
        bytes.push(response_type_bytes[0]);
        bytes.extend_from_slice(&[0; 1]);
        self.sequence.serialize_into(bytes);
        self.length.serialize_into(bytes);
        let num_sizes = u32::try_from(self.sizes.len()).expect("`sizes` has too many elements");
        num_sizes.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.sizes.serialize_into(bytes);
    }
}
impl QueryResourceBytesReply {
    /// Get the value of the `num_sizes` field.
    ///
    /// The `num_sizes` field is used as the length field of the `sizes` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_sizes(&self) -> u32 {
        self.sizes.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod query_resource_bytes_reply {
    #![allow(dead_code, unused_imports)]
    use super::QueryResourceBytesReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for QueryResourceBytesReply {
        fn generate(rng: &Rng) -> Self {
            let num_sizes = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let sizes = gen_random_list(rng, usize::try_from(num_sizes).unwrap());
            Self {
                sequence,
                length,
                sizes,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(2190470301737426944);
        let value = QueryResourceBytesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

