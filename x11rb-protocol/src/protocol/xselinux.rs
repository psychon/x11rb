// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `SELinux` X11 extension.

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
pub const X11_EXTENSION_NAME: &str = "SELinux";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 0);

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

/// Opcode for the SetDeviceCreateContext request
pub const SET_DEVICE_CREATE_CONTEXT_REQUEST: u8 = 1;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetDeviceCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.context, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetDeviceCreateContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetDeviceCreateContextRequest.
    pub fn into_owned(self) -> SetDeviceCreateContextRequest<'static> {
        SetDeviceCreateContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetDeviceCreateContextRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetDeviceCreateContextRequest<'input> {
}

/// Opcode for the GetDeviceCreateContext request
pub const GET_DEVICE_CREATE_CONTEXT_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceCreateContextRequest;
impl GetDeviceCreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_CREATE_CONTEXT_REQUEST,
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
        if header.minor_opcode != GET_DEVICE_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetDeviceCreateContextRequest
        )
    }
}
impl Request for GetDeviceCreateContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceCreateContextRequest {
    type Reply = GetDeviceCreateContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetDeviceCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceCreateContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceCreateContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetDeviceCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_device_create_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetDeviceCreateContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetDeviceCreateContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6961231456307773440);
        let value = GetDeviceCreateContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetDeviceContext request
pub const SET_DEVICE_CONTEXT_REQUEST: u8 = 3;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDeviceContextRequest<'input> {
    pub device: u32,
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetDeviceContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let device_bytes = self.device.serialize();
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_DEVICE_CONTEXT_REQUEST,
            0,
            0,
            device_bytes[0],
            device_bytes[1],
            device_bytes[2],
            device_bytes[3],
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.context, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_DEVICE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device, remaining) = u32::try_parse(value)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetDeviceContextRequest {
            device,
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetDeviceContextRequest.
    pub fn into_owned(self) -> SetDeviceContextRequest<'static> {
        SetDeviceContextRequest {
            device: self.device,
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetDeviceContextRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetDeviceContextRequest<'input> {
}

/// Opcode for the GetDeviceContext request
pub const GET_DEVICE_CONTEXT_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceContextRequest {
    pub device: u32,
}
impl GetDeviceContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let device_bytes = self.device.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_DEVICE_CONTEXT_REQUEST,
            0,
            0,
            device_bytes[0],
            device_bytes[1],
            device_bytes[2],
            device_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_DEVICE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (device, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(GetDeviceContextRequest {
            device,
        })
    }
}
impl Request for GetDeviceContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetDeviceContextRequest {
    type Reply = GetDeviceContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetDeviceContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetDeviceContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetDeviceContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetDeviceContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_device_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetDeviceContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetDeviceContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(16457490935931994112);
        let value = GetDeviceContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetWindowCreateContext request
pub const SET_WINDOW_CREATE_CONTEXT_REQUEST: u8 = 5;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetWindowCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetWindowCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_WINDOW_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.context, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_WINDOW_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetWindowCreateContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetWindowCreateContextRequest.
    pub fn into_owned(self) -> SetWindowCreateContextRequest<'static> {
        SetWindowCreateContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetWindowCreateContextRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetWindowCreateContextRequest<'input> {
}

/// Opcode for the GetWindowCreateContext request
pub const GET_WINDOW_CREATE_CONTEXT_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetWindowCreateContextRequest;
impl GetWindowCreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_WINDOW_CREATE_CONTEXT_REQUEST,
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
        if header.minor_opcode != GET_WINDOW_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetWindowCreateContextRequest
        )
    }
}
impl Request for GetWindowCreateContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetWindowCreateContextRequest {
    type Reply = GetWindowCreateContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetWindowCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetWindowCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetWindowCreateContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetWindowCreateContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetWindowCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_window_create_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetWindowCreateContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetWindowCreateContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(353403079304937472);
        let value = GetWindowCreateContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetWindowContext request
pub const GET_WINDOW_CONTEXT_REQUEST: u8 = 7;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetWindowContextRequest {
    pub window: xproto::Window,
}
impl GetWindowContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_WINDOW_CONTEXT_REQUEST,
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
        if header.minor_opcode != GET_WINDOW_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(GetWindowContextRequest {
            window,
        })
    }
}
impl Request for GetWindowContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetWindowContextRequest {
    type Reply = GetWindowContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetWindowContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetWindowContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetWindowContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetWindowContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetWindowContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_window_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetWindowContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetWindowContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(18054366113297334272);
        let value = GetWindowContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListItem {
    pub name: xproto::Atom,
    pub object_context: Vec<u8>,
    pub data_context: Vec<u8>,
}
impl TryParse for ListItem {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (object_context_len, remaining) = u32::try_parse(remaining)?;
        let (data_context_len, remaining) = u32::try_parse(remaining)?;
        let (object_context, remaining) = crate::x11_utils::parse_u8_list(remaining, object_context_len.try_to_usize()?)?;
        let object_context = object_context.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let (data_context, remaining) = crate::x11_utils::parse_u8_list(remaining, data_context_len.try_to_usize()?)?;
        let data_context = data_context.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let result = ListItem { name, object_context, data_context };
        Ok((result, remaining))
    }
}
impl Serialize for ListItem {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.name.serialize_into(bytes);
        let object_context_len = u32::try_from(self.object_context.len()).expect("`object_context` has too many elements");
        object_context_len.serialize_into(bytes);
        let data_context_len = u32::try_from(self.data_context.len()).expect("`data_context` has too many elements");
        data_context_len.serialize_into(bytes);
        bytes.extend_from_slice(&self.object_context);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        bytes.extend_from_slice(&self.data_context);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl ListItem {
    /// Get the value of the `object_context_len` field.
    ///
    /// The `object_context_len` field is used as the length field of the `object_context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn object_context_len(&self) -> u32 {
        self.object_context.len()
            .try_into().unwrap()
    }
    /// Get the value of the `data_context_len` field.
    ///
    /// The `data_context_len` field is used as the length field of the `data_context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn data_context_len(&self) -> u32 {
        self.data_context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod list_item {
    #![allow(dead_code, unused_imports)]
    use super::ListItem;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ListItem {
        fn generate(rng: &Rng) -> Self {
            let object_context_len = u32::from(rng.u8(..16));
            let data_context_len = u32::from(rng.u8(..16));
            let name = GenRandom::generate(rng);
            let object_context: Vec<u8> = gen_random_list(rng, usize::try_from(object_context_len).unwrap());
            let data_context: Vec<u8> = gen_random_list(rng, usize::try_from(data_context_len).unwrap());
            Self {
                name,
                object_context,
                data_context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9924015684878400);
        let value = ListItem::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetPropertyCreateContext request
pub const SET_PROPERTY_CREATE_CONTEXT_REQUEST: u8 = 8;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPropertyCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetPropertyCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_PROPERTY_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.context, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_PROPERTY_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetPropertyCreateContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetPropertyCreateContextRequest.
    pub fn into_owned(self) -> SetPropertyCreateContextRequest<'static> {
        SetPropertyCreateContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetPropertyCreateContextRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetPropertyCreateContextRequest<'input> {
}

/// Opcode for the GetPropertyCreateContext request
pub const GET_PROPERTY_CREATE_CONTEXT_REQUEST: u8 = 9;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyCreateContextRequest;
impl GetPropertyCreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_PROPERTY_CREATE_CONTEXT_REQUEST,
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
        if header.minor_opcode != GET_PROPERTY_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetPropertyCreateContextRequest
        )
    }
}
impl Request for GetPropertyCreateContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPropertyCreateContextRequest {
    type Reply = GetPropertyCreateContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyCreateContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetPropertyCreateContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetPropertyCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_property_create_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetPropertyCreateContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetPropertyCreateContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4289186566372327424);
        let value = GetPropertyCreateContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetPropertyUseContext request
pub const SET_PROPERTY_USE_CONTEXT_REQUEST: u8 = 10;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPropertyUseContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetPropertyUseContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_PROPERTY_USE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.context, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_PROPERTY_USE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetPropertyUseContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetPropertyUseContextRequest.
    pub fn into_owned(self) -> SetPropertyUseContextRequest<'static> {
        SetPropertyUseContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetPropertyUseContextRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetPropertyUseContextRequest<'input> {
}

/// Opcode for the GetPropertyUseContext request
pub const GET_PROPERTY_USE_CONTEXT_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyUseContextRequest;
impl GetPropertyUseContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_PROPERTY_USE_CONTEXT_REQUEST,
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
        if header.minor_opcode != GET_PROPERTY_USE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetPropertyUseContextRequest
        )
    }
}
impl Request for GetPropertyUseContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPropertyUseContextRequest {
    type Reply = GetPropertyUseContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyUseContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyUseContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyUseContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetPropertyUseContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetPropertyUseContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_property_use_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetPropertyUseContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetPropertyUseContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(11972078247006437376);
        let value = GetPropertyUseContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetPropertyContext request
pub const GET_PROPERTY_CONTEXT_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyContextRequest {
    pub window: xproto::Window,
    pub property: xproto::Atom,
}
impl GetPropertyContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_PROPERTY_CONTEXT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROPERTY_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPropertyContextRequest {
            window,
            property,
        })
    }
}
impl Request for GetPropertyContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPropertyContextRequest {
    type Reply = GetPropertyContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetPropertyContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetPropertyContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_property_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetPropertyContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetPropertyContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13589980866431418368);
        let value = GetPropertyContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetPropertyDataContext request
pub const GET_PROPERTY_DATA_CONTEXT_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyDataContextRequest {
    pub window: xproto::Window,
    pub property: xproto::Atom,
}
impl GetPropertyDataContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_PROPERTY_DATA_CONTEXT_REQUEST,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PROPERTY_DATA_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let (property, remaining) = xproto::Atom::try_parse(remaining)?;
        let _ = remaining;
        Ok(GetPropertyDataContextRequest {
            window,
            property,
        })
    }
}
impl Request for GetPropertyDataContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetPropertyDataContextRequest {
    type Reply = GetPropertyDataContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyDataContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyDataContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPropertyDataContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetPropertyDataContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetPropertyDataContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_property_data_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetPropertyDataContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetPropertyDataContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(4776007541572239360);
        let value = GetPropertyDataContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ListProperties request
pub const LIST_PROPERTIES_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListPropertiesRequest {
    pub window: xproto::Window,
}
impl ListPropertiesRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            major_opcode,
            LIST_PROPERTIES_REQUEST,
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
        if header.minor_opcode != LIST_PROPERTIES_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (window, remaining) = xproto::Window::try_parse(value)?;
        let _ = remaining;
        Ok(ListPropertiesRequest {
            window,
        })
    }
}
impl Request for ListPropertiesRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for ListPropertiesRequest {
    type Reply = ListPropertiesReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListPropertiesReply {
    pub sequence: u16,
    pub length: u32,
    pub properties: Vec<ListItem>,
}
impl TryParse for ListPropertiesReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (properties_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (properties, remaining) = crate::x11_utils::parse_list::<ListItem>(remaining, properties_len.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListPropertiesReply { sequence, length, properties };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ListPropertiesReply {
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
        let properties_len = u32::try_from(self.properties.len()).expect("`properties` has too many elements");
        properties_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.properties.serialize_into(bytes);
    }
}
impl ListPropertiesReply {
    /// Get the value of the `properties_len` field.
    ///
    /// The `properties_len` field is used as the length field of the `properties` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn properties_len(&self) -> u32 {
        self.properties.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod list_properties_reply {
    #![allow(dead_code, unused_imports)]
    use super::ListPropertiesReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ListPropertiesReply {
        fn generate(rng: &Rng) -> Self {
            let properties_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let properties = gen_random_list(rng, usize::try_from(properties_len).unwrap());
            Self {
                sequence,
                length,
                properties,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(5363867181899579392);
        let value = ListPropertiesReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetSelectionCreateContext request
pub const SET_SELECTION_CREATE_CONTEXT_REQUEST: u8 = 15;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetSelectionCreateContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetSelectionCreateContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_SELECTION_CREATE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.context, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_SELECTION_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetSelectionCreateContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetSelectionCreateContextRequest.
    pub fn into_owned(self) -> SetSelectionCreateContextRequest<'static> {
        SetSelectionCreateContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetSelectionCreateContextRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetSelectionCreateContextRequest<'input> {
}

/// Opcode for the GetSelectionCreateContext request
pub const GET_SELECTION_CREATE_CONTEXT_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectionCreateContextRequest;
impl GetSelectionCreateContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_SELECTION_CREATE_CONTEXT_REQUEST,
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
        if header.minor_opcode != GET_SELECTION_CREATE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetSelectionCreateContextRequest
        )
    }
}
impl Request for GetSelectionCreateContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetSelectionCreateContextRequest {
    type Reply = GetSelectionCreateContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectionCreateContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionCreateContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionCreateContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetSelectionCreateContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetSelectionCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_selection_create_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetSelectionCreateContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetSelectionCreateContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6930251893801222144);
        let value = GetSelectionCreateContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the SetSelectionUseContext request
pub const SET_SELECTION_USE_CONTEXT_REQUEST: u8 = 17;
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetSelectionUseContextRequest<'input> {
    pub context: Cow<'input, [u8]>,
}
impl<'input> SetSelectionUseContextRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'input>> {
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            major_opcode,
            SET_SELECTION_USE_CONTEXT_REQUEST,
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + self.context.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into(), self.context, padding0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_SELECTION_USE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (context_len, remaining) = u32::try_parse(value)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let _ = remaining;
        Ok(SetSelectionUseContextRequest {
            context: Cow::Borrowed(context),
        })
    }
    /// Clone all borrowed data in this SetSelectionUseContextRequest.
    pub fn into_owned(self) -> SetSelectionUseContextRequest<'static> {
        SetSelectionUseContextRequest {
            context: Cow::Owned(self.context.into_owned()),
        }
    }
}
impl<'input> Request for SetSelectionUseContextRequest<'input> {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl<'input> crate::x11_utils::VoidRequest for SetSelectionUseContextRequest<'input> {
}

/// Opcode for the GetSelectionUseContext request
pub const GET_SELECTION_USE_CONTEXT_REQUEST: u8 = 18;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectionUseContextRequest;
impl GetSelectionUseContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            GET_SELECTION_USE_CONTEXT_REQUEST,
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
        if header.minor_opcode != GET_SELECTION_USE_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(GetSelectionUseContextRequest
        )
    }
}
impl Request for GetSelectionUseContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetSelectionUseContextRequest {
    type Reply = GetSelectionUseContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectionUseContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionUseContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionUseContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetSelectionUseContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetSelectionUseContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_selection_use_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetSelectionUseContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetSelectionUseContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(3484972378570620928);
        let value = GetSelectionUseContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetSelectionContext request
pub const GET_SELECTION_CONTEXT_REQUEST: u8 = 19;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectionContextRequest {
    pub selection: xproto::Atom,
}
impl GetSelectionContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let selection_bytes = self.selection.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_SELECTION_CONTEXT_REQUEST,
            0,
            0,
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SELECTION_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (selection, remaining) = xproto::Atom::try_parse(value)?;
        let _ = remaining;
        Ok(GetSelectionContextRequest {
            selection,
        })
    }
}
impl Request for GetSelectionContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetSelectionContextRequest {
    type Reply = GetSelectionContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectionContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetSelectionContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetSelectionContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_selection_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetSelectionContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetSelectionContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(9293993651094945792);
        let value = GetSelectionContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetSelectionDataContext request
pub const GET_SELECTION_DATA_CONTEXT_REQUEST: u8 = 20;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectionDataContextRequest {
    pub selection: xproto::Atom,
}
impl GetSelectionDataContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let selection_bytes = self.selection.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_SELECTION_DATA_CONTEXT_REQUEST,
            0,
            0,
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_SELECTION_DATA_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (selection, remaining) = xproto::Atom::try_parse(value)?;
        let _ = remaining;
        Ok(GetSelectionDataContextRequest {
            selection,
        })
    }
}
impl Request for GetSelectionDataContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetSelectionDataContextRequest {
    type Reply = GetSelectionDataContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetSelectionDataContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionDataContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetSelectionDataContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetSelectionDataContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetSelectionDataContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_selection_data_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetSelectionDataContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetSelectionDataContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(6004155259117633536);
        let value = GetSelectionDataContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the ListSelections request
pub const LIST_SELECTIONS_REQUEST: u8 = 21;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListSelectionsRequest;
impl ListSelectionsRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let mut request0 = vec![
            major_opcode,
            LIST_SELECTIONS_REQUEST,
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
        if header.minor_opcode != LIST_SELECTIONS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(ListSelectionsRequest
        )
    }
}
impl Request for ListSelectionsRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for ListSelectionsRequest {
    type Reply = ListSelectionsReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListSelectionsReply {
    pub sequence: u16,
    pub length: u32,
    pub selections: Vec<ListItem>,
}
impl TryParse for ListSelectionsReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (selections_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (selections, remaining) = crate::x11_utils::parse_list::<ListItem>(remaining, selections_len.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListSelectionsReply { sequence, length, selections };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for ListSelectionsReply {
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
        let selections_len = u32::try_from(self.selections.len()).expect("`selections` has too many elements");
        selections_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        self.selections.serialize_into(bytes);
    }
}
impl ListSelectionsReply {
    /// Get the value of the `selections_len` field.
    ///
    /// The `selections_len` field is used as the length field of the `selections` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn selections_len(&self) -> u32 {
        self.selections.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod list_selections_reply {
    #![allow(dead_code, unused_imports)]
    use super::ListSelectionsReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for ListSelectionsReply {
        fn generate(rng: &Rng) -> Self {
            let selections_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let selections = gen_random_list(rng, usize::try_from(selections_len).unwrap());
            Self {
                sequence,
                length,
                selections,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(13700242364557754368);
        let value = ListSelectionsReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

/// Opcode for the GetClientContext request
pub const GET_CLIENT_CONTEXT_REQUEST: u8 = 22;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetClientContextRequest {
    pub resource: u32,
}
impl GetClientContextRequest {
    /// Serialize this request into bytes for the provided connection
    pub fn serialize(self, major_opcode: u8) -> BufWithFds<PiecewiseBuf<'static>> {
        let length_so_far = 0;
        let resource_bytes = self.resource.serialize();
        let mut request0 = vec![
            major_opcode,
            GET_CLIENT_CONTEXT_REQUEST,
            0,
            0,
            resource_bytes[0],
            resource_bytes[1],
            resource_bytes[2],
            resource_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        (vec![request0.into()], vec![])
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_CLIENT_CONTEXT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (resource, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(GetClientContextRequest {
            resource,
        })
    }
}
impl Request for GetClientContextRequest {
    const EXTENSION_NAME: Option<&'static str> = Some(X11_EXTENSION_NAME);

    fn serialize(self, major_opcode: u8) -> BufWithFds<Vec<u8>> {
        let (bufs, fds) = self.serialize(major_opcode);
        // Flatten the buffers into a single vector
        let buf = bufs.iter().flat_map(|buf| buf.iter().copied()).collect();
        (buf, fds)
    }
}
impl crate::x11_utils::ReplyRequest for GetClientContextRequest {
    type Reply = GetClientContextReply;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetClientContextReply {
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetClientContextReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_to_usize()?)?;
        let context = context.to_vec();
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetClientContextReply { sequence, length, context };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl Serialize for GetClientContextReply {
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
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        context_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 20]);
        bytes.extend_from_slice(&self.context);
    }
}
impl GetClientContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}
#[cfg(test)]
mod get_client_context_reply {
    #![allow(dead_code, unused_imports)]
    use super::GetClientContextReply;
    use crate::x11_utils::{GenRandom, Serialize, gen_random_list};
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use fastrand::Rng;
    impl GenRandom for GetClientContextReply {
        fn generate(rng: &Rng) -> Self {
            let context_len = u32::from(rng.u8(..16));
            let sequence: u16 = GenRandom::generate(rng);
            let length: u32 = GenRandom::generate(rng);
            let context: Vec<u8> = gen_random_list(rng, usize::try_from(context_len).unwrap());
            Self {
                sequence,
                length,
                context,
            }
        }
    }
    #[test]
    fn check_serialize() {
        let rng = Rng::with_seed(17505549001259220992);
        let value = GetClientContextReply::generate(&rng);
        let left = value.serialize();
        let mut right = alloc::vec![];
        value.serialize_into(&mut right);
        assert_eq!(&left[..], right.as_slice());
    }
}

