// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Sync` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::{RawFdContainer, pretty_print_bitmask, pretty_print_enum};
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd, TryIntoUSize};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "SYNC";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (3, 1);

pub type Alarm = u32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ALARMSTATE(u8);
impl ALARMSTATE {
    pub const ACTIVE: Self = Self(0);
    pub const INACTIVE: Self = Self(1);
    pub const DESTROYED: Self = Self(2);
}
impl From<ALARMSTATE> for u8 {
    #[inline]
    fn from(input: ALARMSTATE) -> Self {
        input.0
    }
}
impl From<ALARMSTATE> for Option<u8> {
    #[inline]
    fn from(input: ALARMSTATE) -> Self {
        Some(input.0)
    }
}
impl From<ALARMSTATE> for u16 {
    #[inline]
    fn from(input: ALARMSTATE) -> Self {
        u16::from(input.0)
    }
}
impl From<ALARMSTATE> for Option<u16> {
    #[inline]
    fn from(input: ALARMSTATE) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<ALARMSTATE> for u32 {
    #[inline]
    fn from(input: ALARMSTATE) -> Self {
        u32::from(input.0)
    }
}
impl From<ALARMSTATE> for Option<u32> {
    #[inline]
    fn from(input: ALARMSTATE) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ALARMSTATE {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ALARMSTATE  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::ACTIVE.0.into(), "ACTIVE", "Active"),
            (Self::INACTIVE.0.into(), "INACTIVE", "Inactive"),
            (Self::DESTROYED.0.into(), "DESTROYED", "Destroyed"),
        ];
        pretty_print_enum(fmt, self.0.into(), &variants)
    }
}

pub type Counter = u32;

pub type Fence = u32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TESTTYPE(u32);
impl TESTTYPE {
    pub const POSITIVE_TRANSITION: Self = Self(0);
    pub const NEGATIVE_TRANSITION: Self = Self(1);
    pub const POSITIVE_COMPARISON: Self = Self(2);
    pub const NEGATIVE_COMPARISON: Self = Self(3);
}
impl From<TESTTYPE> for u32 {
    #[inline]
    fn from(input: TESTTYPE) -> Self {
        input.0
    }
}
impl From<TESTTYPE> for Option<u32> {
    #[inline]
    fn from(input: TESTTYPE) -> Self {
        Some(input.0)
    }
}
impl From<u8> for TESTTYPE {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for TESTTYPE {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for TESTTYPE {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for TESTTYPE  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::POSITIVE_TRANSITION.0, "POSITIVE_TRANSITION", "PositiveTransition"),
            (Self::NEGATIVE_TRANSITION.0, "NEGATIVE_TRANSITION", "NegativeTransition"),
            (Self::POSITIVE_COMPARISON.0, "POSITIVE_COMPARISON", "PositiveComparison"),
            (Self::NEGATIVE_COMPARISON.0, "NEGATIVE_COMPARISON", "NegativeComparison"),
        ];
        pretty_print_enum(fmt, self.0, &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VALUETYPE(u32);
impl VALUETYPE {
    pub const ABSOLUTE: Self = Self(0);
    pub const RELATIVE: Self = Self(1);
}
impl From<VALUETYPE> for u32 {
    #[inline]
    fn from(input: VALUETYPE) -> Self {
        input.0
    }
}
impl From<VALUETYPE> for Option<u32> {
    #[inline]
    fn from(input: VALUETYPE) -> Self {
        Some(input.0)
    }
}
impl From<u8> for VALUETYPE {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for VALUETYPE {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}
impl From<u32> for VALUETYPE {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for VALUETYPE  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::ABSOLUTE.0, "ABSOLUTE", "Absolute"),
            (Self::RELATIVE.0, "RELATIVE", "Relative"),
        ];
        pretty_print_enum(fmt, self.0, &variants)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CA(u8);
impl CA {
    pub const COUNTER: Self = Self(1 << 0);
    pub const VALUE_TYPE: Self = Self(1 << 1);
    pub const VALUE: Self = Self(1 << 2);
    pub const TEST_TYPE: Self = Self(1 << 3);
    pub const DELTA: Self = Self(1 << 4);
    pub const EVENTS: Self = Self(1 << 5);
}
impl From<CA> for u8 {
    #[inline]
    fn from(input: CA) -> Self {
        input.0
    }
}
impl From<CA> for Option<u8> {
    #[inline]
    fn from(input: CA) -> Self {
        Some(input.0)
    }
}
impl From<CA> for u16 {
    #[inline]
    fn from(input: CA) -> Self {
        u16::from(input.0)
    }
}
impl From<CA> for Option<u16> {
    #[inline]
    fn from(input: CA) -> Self {
        Some(u16::from(input.0))
    }
}
impl From<CA> for u32 {
    #[inline]
    fn from(input: CA) -> Self {
        u32::from(input.0)
    }
}
impl From<CA> for Option<u32> {
    #[inline]
    fn from(input: CA) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for CA {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for CA  {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variants = [
            (Self::COUNTER.0.into(), "COUNTER", "Counter"),
            (Self::VALUE_TYPE.0.into(), "VALUE_TYPE", "ValueType"),
            (Self::VALUE.0.into(), "VALUE", "Value"),
            (Self::TEST_TYPE.0.into(), "TEST_TYPE", "TestType"),
            (Self::DELTA.0.into(), "DELTA", "Delta"),
            (Self::EVENTS.0.into(), "EVENTS", "Events"),
        ];
        pretty_print_bitmask(fmt, self.0.into(), &variants)
    }
}
bitmask_binop!(CA, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Int64 {
    pub hi: i32,
    pub lo: u32,
}
impl TryParse for Int64 {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (hi, remaining) = i32::try_parse(remaining)?;
        let (lo, remaining) = u32::try_parse(remaining)?;
        let result = Int64 { hi, lo };
        Ok((result, remaining))
    }
}
impl Serialize for Int64 {
    type Bytes = [u8; 8];
    fn serialize(&self) -> [u8; 8] {
        let hi_bytes = self.hi.serialize();
        let lo_bytes = self.lo.serialize();
        [
            hi_bytes[0],
            hi_bytes[1],
            hi_bytes[2],
            hi_bytes[3],
            lo_bytes[0],
            lo_bytes[1],
            lo_bytes[2],
            lo_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.hi.serialize_into(bytes);
        self.lo.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Systemcounter {
    pub counter: Counter,
    pub resolution: Int64,
    pub name: Vec<u8>,
}
impl TryParse for Systemcounter {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (counter, remaining) = Counter::try_parse(remaining)?;
        let (resolution, remaining) = Int64::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_to_usize()?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::InsufficientData)?;
        let result = Systemcounter { counter, resolution, name };
        Ok((result, remaining))
    }
}
impl Serialize for Systemcounter {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(14);
        self.counter.serialize_into(bytes);
        self.resolution.serialize_into(bytes);
        let name_len = u16::try_from(self.name.len()).expect("`name` has too many elements");
        name_len.serialize_into(bytes);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl Systemcounter {
    /// Get the value of the `name_len` field.
    ///
    /// The `name_len` field is used as the length field of the `name` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn name_len(&self) -> u16 {
        self.name.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trigger {
    pub counter: Counter,
    pub wait_type: VALUETYPE,
    pub wait_value: Int64,
    pub test_type: TESTTYPE,
}
impl TryParse for Trigger {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (counter, remaining) = Counter::try_parse(remaining)?;
        let (wait_type, remaining) = u32::try_parse(remaining)?;
        let (wait_value, remaining) = Int64::try_parse(remaining)?;
        let (test_type, remaining) = u32::try_parse(remaining)?;
        let wait_type = wait_type.into();
        let test_type = test_type.into();
        let result = Trigger { counter, wait_type, wait_value, test_type };
        Ok((result, remaining))
    }
}
impl Serialize for Trigger {
    type Bytes = [u8; 20];
    fn serialize(&self) -> [u8; 20] {
        let counter_bytes = self.counter.serialize();
        let wait_type_bytes = u32::from(self.wait_type).serialize();
        let wait_value_bytes = self.wait_value.serialize();
        let test_type_bytes = u32::from(self.test_type).serialize();
        [
            counter_bytes[0],
            counter_bytes[1],
            counter_bytes[2],
            counter_bytes[3],
            wait_type_bytes[0],
            wait_type_bytes[1],
            wait_type_bytes[2],
            wait_type_bytes[3],
            wait_value_bytes[0],
            wait_value_bytes[1],
            wait_value_bytes[2],
            wait_value_bytes[3],
            wait_value_bytes[4],
            wait_value_bytes[5],
            wait_value_bytes[6],
            wait_value_bytes[7],
            test_type_bytes[0],
            test_type_bytes[1],
            test_type_bytes[2],
            test_type_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(20);
        self.counter.serialize_into(bytes);
        u32::from(self.wait_type).serialize_into(bytes);
        self.wait_value.serialize_into(bytes);
        u32::from(self.test_type).serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Waitcondition {
    pub trigger: Trigger,
    pub event_threshold: Int64,
}
impl TryParse for Waitcondition {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (trigger, remaining) = Trigger::try_parse(remaining)?;
        let (event_threshold, remaining) = Int64::try_parse(remaining)?;
        let result = Waitcondition { trigger, event_threshold };
        Ok((result, remaining))
    }
}
impl Serialize for Waitcondition {
    type Bytes = [u8; 28];
    fn serialize(&self) -> [u8; 28] {
        let trigger_bytes = self.trigger.serialize();
        let event_threshold_bytes = self.event_threshold.serialize();
        [
            trigger_bytes[0],
            trigger_bytes[1],
            trigger_bytes[2],
            trigger_bytes[3],
            trigger_bytes[4],
            trigger_bytes[5],
            trigger_bytes[6],
            trigger_bytes[7],
            trigger_bytes[8],
            trigger_bytes[9],
            trigger_bytes[10],
            trigger_bytes[11],
            trigger_bytes[12],
            trigger_bytes[13],
            trigger_bytes[14],
            trigger_bytes[15],
            trigger_bytes[16],
            trigger_bytes[17],
            trigger_bytes[18],
            trigger_bytes[19],
            event_threshold_bytes[0],
            event_threshold_bytes[1],
            event_threshold_bytes[2],
            event_threshold_bytes[3],
            event_threshold_bytes[4],
            event_threshold_bytes[5],
            event_threshold_bytes[6],
            event_threshold_bytes[7],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(28);
        self.trigger.serialize_into(bytes);
        self.event_threshold.serialize_into(bytes);
    }
}

/// Opcode for the Counter error
pub const COUNTER_ERROR: u8 = 0;

/// Opcode for the Alarm error
pub const ALARM_ERROR: u8 = 1;

/// Opcode for the Initialize request
pub const INITIALIZE_REQUEST: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitializeRequest {
    pub desired_major_version: u8,
    pub desired_minor_version: u8,
}
impl InitializeRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let desired_major_version_bytes = self.desired_major_version.serialize();
        let desired_minor_version_bytes = self.desired_minor_version.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            INITIALIZE_REQUEST,
            0,
            0,
            desired_major_version_bytes[0],
            desired_minor_version_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, InitializeReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != INITIALIZE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (desired_major_version, remaining) = u8::try_parse(value)?;
        let (desired_minor_version, remaining) = u8::try_parse(remaining)?;
        let _ = remaining;
        Ok(InitializeRequest {
            desired_major_version,
            desired_minor_version,
        })
    }
}
impl Request for InitializeRequest {
    type Reply = InitializeReply;
}
pub fn initialize<Conn>(conn: &Conn, desired_major_version: u8, desired_minor_version: u8) -> Result<Cookie<'_, Conn, InitializeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InitializeRequest {
        desired_major_version,
        desired_minor_version,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitializeReply {
    pub sequence: u16,
    pub length: u32,
    pub major_version: u8,
    pub minor_version: u8,
}
impl TryParse for InitializeReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u8::try_parse(remaining)?;
        let (minor_version, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = InitializeReply { sequence, length, major_version, minor_version };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the ListSystemCounters request
pub const LIST_SYSTEM_COUNTERS_REQUEST: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListSystemCountersRequest;
impl ListSystemCountersRequest {
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
            LIST_SYSTEM_COUNTERS_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, ListSystemCountersReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != LIST_SYSTEM_COUNTERS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let _ = value;
        Ok(ListSystemCountersRequest
        )
    }
}
impl Request for ListSystemCountersRequest {
    type Reply = ListSystemCountersReply;
}
pub fn list_system_counters<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSystemCountersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSystemCountersRequest;
    request0.send(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSystemCountersReply {
    pub sequence: u16,
    pub length: u32,
    pub counters: Vec<Systemcounter>,
}
impl TryParse for ListSystemCountersReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (counters_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::InsufficientData)?;
        let (counters, remaining) = crate::x11_utils::parse_list::<Systemcounter>(remaining, counters_len.try_to_usize()?)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = ListSystemCountersReply { sequence, length, counters };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl ListSystemCountersReply {
    /// Get the value of the `counters_len` field.
    ///
    /// The `counters_len` field is used as the length field of the `counters` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn counters_len(&self) -> u32 {
        self.counters.len()
            .try_into().unwrap()
    }
}

/// Opcode for the CreateCounter request
pub const CREATE_COUNTER_REQUEST: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateCounterRequest {
    pub id: Counter,
    pub initial_value: Int64,
}
impl CreateCounterRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let id_bytes = self.id.serialize();
        let initial_value_bytes = self.initial_value.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_COUNTER_REQUEST,
            0,
            0,
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            initial_value_bytes[0],
            initial_value_bytes[1],
            initial_value_bytes[2],
            initial_value_bytes[3],
            initial_value_bytes[4],
            initial_value_bytes[5],
            initial_value_bytes[6],
            initial_value_bytes[7],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_COUNTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (id, remaining) = Counter::try_parse(value)?;
        let (initial_value, remaining) = Int64::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateCounterRequest {
            id,
            initial_value,
        })
    }
}
impl Request for CreateCounterRequest {
    type Reply = ();
}
pub fn create_counter<Conn>(conn: &Conn, id: Counter, initial_value: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateCounterRequest {
        id,
        initial_value,
    };
    request0.send(conn)
}

/// Opcode for the DestroyCounter request
pub const DESTROY_COUNTER_REQUEST: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyCounterRequest {
    pub counter: Counter,
}
impl DestroyCounterRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let counter_bytes = self.counter.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_COUNTER_REQUEST,
            0,
            0,
            counter_bytes[0],
            counter_bytes[1],
            counter_bytes[2],
            counter_bytes[3],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_COUNTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (counter, remaining) = Counter::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyCounterRequest {
            counter,
        })
    }
}
impl Request for DestroyCounterRequest {
    type Reply = ();
}
pub fn destroy_counter<Conn>(conn: &Conn, counter: Counter) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyCounterRequest {
        counter,
    };
    request0.send(conn)
}

/// Opcode for the QueryCounter request
pub const QUERY_COUNTER_REQUEST: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryCounterRequest {
    pub counter: Counter,
}
impl QueryCounterRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let counter_bytes = self.counter.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_COUNTER_REQUEST,
            0,
            0,
            counter_bytes[0],
            counter_bytes[1],
            counter_bytes[2],
            counter_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryCounterReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_COUNTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (counter, remaining) = Counter::try_parse(value)?;
        let _ = remaining;
        Ok(QueryCounterRequest {
            counter,
        })
    }
}
impl Request for QueryCounterRequest {
    type Reply = QueryCounterReply;
}
pub fn query_counter<Conn>(conn: &Conn, counter: Counter) -> Result<Cookie<'_, Conn, QueryCounterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryCounterRequest {
        counter,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryCounterReply {
    pub sequence: u16,
    pub length: u32,
    pub counter_value: Int64,
}
impl TryParse for QueryCounterReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (counter_value, remaining) = Int64::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryCounterReply { sequence, length, counter_value };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the Await request
pub const AWAIT_REQUEST: u8 = 7;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwaitRequest<'input> {
    pub wait_list: Cow<'input, [Waitcondition]>,
}
impl<'input> AwaitRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            AWAIT_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let wait_list_bytes = self.wait_list.serialize();
        let length_so_far = length_so_far + wait_list_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), wait_list_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != AWAIT_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let mut remaining = value;
        // Length is 'everything left in the input'
        let mut wait_list = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = Waitcondition::try_parse(value)?;
            remaining = new_remaining;
            wait_list.push(v);
        }
        let _ = remaining;
        Ok(AwaitRequest {
            wait_list: Cow::Owned(wait_list),
        })
    }
    /// Clone all borrowed data in this AwaitRequest.
    pub fn into_owned(self) -> AwaitRequest<'static> {
        AwaitRequest {
            wait_list: Cow::Owned(self.wait_list.into_owned()),
        }
    }
}
impl<'input> Request for AwaitRequest<'input> {
    type Reply = ();
}
pub fn await_<'c, 'input, Conn>(conn: &'c Conn, wait_list: &'input [Waitcondition]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AwaitRequest {
        wait_list: Cow::Borrowed(wait_list),
    };
    request0.send(conn)
}

/// Opcode for the ChangeCounter request
pub const CHANGE_COUNTER_REQUEST: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeCounterRequest {
    pub counter: Counter,
    pub amount: Int64,
}
impl ChangeCounterRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let counter_bytes = self.counter.serialize();
        let amount_bytes = self.amount.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CHANGE_COUNTER_REQUEST,
            0,
            0,
            counter_bytes[0],
            counter_bytes[1],
            counter_bytes[2],
            counter_bytes[3],
            amount_bytes[0],
            amount_bytes[1],
            amount_bytes[2],
            amount_bytes[3],
            amount_bytes[4],
            amount_bytes[5],
            amount_bytes[6],
            amount_bytes[7],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_COUNTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (counter, remaining) = Counter::try_parse(value)?;
        let (amount, remaining) = Int64::try_parse(remaining)?;
        let _ = remaining;
        Ok(ChangeCounterRequest {
            counter,
            amount,
        })
    }
}
impl Request for ChangeCounterRequest {
    type Reply = ();
}
pub fn change_counter<Conn>(conn: &Conn, counter: Counter, amount: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeCounterRequest {
        counter,
        amount,
    };
    request0.send(conn)
}

/// Opcode for the SetCounter request
pub const SET_COUNTER_REQUEST: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetCounterRequest {
    pub counter: Counter,
    pub value: Int64,
}
impl SetCounterRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let counter_bytes = self.counter.serialize();
        let value_bytes = self.value.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_COUNTER_REQUEST,
            0,
            0,
            counter_bytes[0],
            counter_bytes[1],
            counter_bytes[2],
            counter_bytes[3],
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
            value_bytes[4],
            value_bytes[5],
            value_bytes[6],
            value_bytes[7],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_COUNTER_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (counter, remaining) = Counter::try_parse(value)?;
        let (value, remaining) = Int64::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetCounterRequest {
            counter,
            value,
        })
    }
}
impl Request for SetCounterRequest {
    type Reply = ();
}
pub fn set_counter<Conn>(conn: &Conn, counter: Counter, value: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetCounterRequest {
        counter,
        value,
    };
    request0.send(conn)
}

/// Auxiliary and optional information for the `create_alarm` function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CreateAlarmAux {
    pub counter: Option<Counter>,
    pub value_type: Option<VALUETYPE>,
    pub value: Option<Int64>,
    pub test_type: Option<TESTTYPE>,
    pub delta: Option<Int64>,
    pub events: Option<u32>,
}
impl CreateAlarmAux {
    fn try_parse(value: &[u8], value_mask: u32) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = value_mask;
        let mut outer_remaining = value;
        let counter = if switch_expr & u32::from(CA::COUNTER) != 0 {
            let remaining = outer_remaining;
            let (counter, remaining) = Counter::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(counter)
        } else {
            None
        };
        let value_type = if switch_expr & u32::from(CA::VALUE_TYPE) != 0 {
            let remaining = outer_remaining;
            let (value_type, remaining) = u32::try_parse(remaining)?;
            let value_type = value_type.into();
            outer_remaining = remaining;
            Some(value_type)
        } else {
            None
        };
        let value = if switch_expr & u32::from(CA::VALUE) != 0 {
            let remaining = outer_remaining;
            let (value, remaining) = Int64::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(value)
        } else {
            None
        };
        let test_type = if switch_expr & u32::from(CA::TEST_TYPE) != 0 {
            let remaining = outer_remaining;
            let (test_type, remaining) = u32::try_parse(remaining)?;
            let test_type = test_type.into();
            outer_remaining = remaining;
            Some(test_type)
        } else {
            None
        };
        let delta = if switch_expr & u32::from(CA::DELTA) != 0 {
            let remaining = outer_remaining;
            let (delta, remaining) = Int64::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(delta)
        } else {
            None
        };
        let events = if switch_expr & u32::from(CA::EVENTS) != 0 {
            let remaining = outer_remaining;
            let (events, remaining) = u32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(events)
        } else {
            None
        };
        let result = CreateAlarmAux { counter, value_type, value, test_type, delta, events };
        Ok((result, outer_remaining))
    }
}
impl CreateAlarmAux {
    #[allow(dead_code)]
    fn serialize(&self, value_mask: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, value_mask);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, value_mask: u32) {
        assert_eq!(self.switch_expr(), value_mask, "switch `value_list` has an inconsistent discriminant");
        if let Some(counter) = self.counter {
            counter.serialize_into(bytes);
        }
        if let Some(value_type) = self.value_type {
            u32::from(value_type).serialize_into(bytes);
        }
        if let Some(ref value) = self.value {
            value.serialize_into(bytes);
        }
        if let Some(test_type) = self.test_type {
            u32::from(test_type).serialize_into(bytes);
        }
        if let Some(ref delta) = self.delta {
            delta.serialize_into(bytes);
        }
        if let Some(events) = self.events {
            events.serialize_into(bytes);
        }
    }
}
impl CreateAlarmAux {
    fn switch_expr(&self) -> u32 {
        let mut expr_value = 0;
        if self.counter.is_some() {
            expr_value |= u32::from(CA::COUNTER);
        }
        if self.value_type.is_some() {
            expr_value |= u32::from(CA::VALUE_TYPE);
        }
        if self.value.is_some() {
            expr_value |= u32::from(CA::VALUE);
        }
        if self.test_type.is_some() {
            expr_value |= u32::from(CA::TEST_TYPE);
        }
        if self.delta.is_some() {
            expr_value |= u32::from(CA::DELTA);
        }
        if self.events.is_some() {
            expr_value |= u32::from(CA::EVENTS);
        }
        expr_value
    }
}
impl CreateAlarmAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the `counter` field of this structure.
    pub fn counter<I>(mut self, value: I) -> Self where I: Into<Option<Counter>> {
        self.counter = value.into();
        self
    }
    /// Set the `value_type` field of this structure.
    pub fn value_type<I>(mut self, value: I) -> Self where I: Into<Option<VALUETYPE>> {
        self.value_type = value.into();
        self
    }
    /// Set the `value` field of this structure.
    pub fn value<I>(mut self, value: I) -> Self where I: Into<Option<Int64>> {
        self.value = value.into();
        self
    }
    /// Set the `test_type` field of this structure.
    pub fn test_type<I>(mut self, value: I) -> Self where I: Into<Option<TESTTYPE>> {
        self.test_type = value.into();
        self
    }
    /// Set the `delta` field of this structure.
    pub fn delta<I>(mut self, value: I) -> Self where I: Into<Option<Int64>> {
        self.delta = value.into();
        self
    }
    /// Set the `events` field of this structure.
    pub fn events<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.events = value.into();
        self
    }
}

/// Opcode for the CreateAlarm request
pub const CREATE_ALARM_REQUEST: u8 = 8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAlarmRequest<'input> {
    pub id: Alarm,
    pub value_list: Cow<'input, CreateAlarmAux>,
}
impl<'input> CreateAlarmRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let id_bytes = self.id.serialize();
        let value_mask = self.value_list.switch_expr();
        let value_mask_bytes = value_mask.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_ALARM_REQUEST,
            0,
            0,
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            value_mask_bytes[0],
            value_mask_bytes[1],
            value_mask_bytes[2],
            value_mask_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let value_list_bytes = self.value_list.serialize(value_mask);
        let length_so_far = length_so_far + value_list_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), value_list_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_ALARM_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (id, remaining) = Alarm::try_parse(value)?;
        let (value_mask, remaining) = u32::try_parse(remaining)?;
        let (value_list, remaining) = CreateAlarmAux::try_parse(remaining, value_mask)?;
        let _ = remaining;
        Ok(CreateAlarmRequest {
            id,
            value_list: Cow::Owned(value_list),
        })
    }
    /// Clone all borrowed data in this CreateAlarmRequest.
    pub fn into_owned(self) -> CreateAlarmRequest<'static> {
        CreateAlarmRequest {
            id: self.id,
            value_list: Cow::Owned(self.value_list.into_owned()),
        }
    }
}
impl<'input> Request for CreateAlarmRequest<'input> {
    type Reply = ();
}
pub fn create_alarm<'c, 'input, Conn>(conn: &'c Conn, id: Alarm, value_list: &'input CreateAlarmAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateAlarmRequest {
        id,
        value_list: Cow::Borrowed(value_list),
    };
    request0.send(conn)
}

/// Auxiliary and optional information for the `change_alarm` function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChangeAlarmAux {
    pub counter: Option<Counter>,
    pub value_type: Option<VALUETYPE>,
    pub value: Option<Int64>,
    pub test_type: Option<TESTTYPE>,
    pub delta: Option<Int64>,
    pub events: Option<u32>,
}
impl ChangeAlarmAux {
    fn try_parse(value: &[u8], value_mask: u32) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = value_mask;
        let mut outer_remaining = value;
        let counter = if switch_expr & u32::from(CA::COUNTER) != 0 {
            let remaining = outer_remaining;
            let (counter, remaining) = Counter::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(counter)
        } else {
            None
        };
        let value_type = if switch_expr & u32::from(CA::VALUE_TYPE) != 0 {
            let remaining = outer_remaining;
            let (value_type, remaining) = u32::try_parse(remaining)?;
            let value_type = value_type.into();
            outer_remaining = remaining;
            Some(value_type)
        } else {
            None
        };
        let value = if switch_expr & u32::from(CA::VALUE) != 0 {
            let remaining = outer_remaining;
            let (value, remaining) = Int64::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(value)
        } else {
            None
        };
        let test_type = if switch_expr & u32::from(CA::TEST_TYPE) != 0 {
            let remaining = outer_remaining;
            let (test_type, remaining) = u32::try_parse(remaining)?;
            let test_type = test_type.into();
            outer_remaining = remaining;
            Some(test_type)
        } else {
            None
        };
        let delta = if switch_expr & u32::from(CA::DELTA) != 0 {
            let remaining = outer_remaining;
            let (delta, remaining) = Int64::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(delta)
        } else {
            None
        };
        let events = if switch_expr & u32::from(CA::EVENTS) != 0 {
            let remaining = outer_remaining;
            let (events, remaining) = u32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(events)
        } else {
            None
        };
        let result = ChangeAlarmAux { counter, value_type, value, test_type, delta, events };
        Ok((result, outer_remaining))
    }
}
impl ChangeAlarmAux {
    #[allow(dead_code)]
    fn serialize(&self, value_mask: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, value_mask);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, value_mask: u32) {
        assert_eq!(self.switch_expr(), value_mask, "switch `value_list` has an inconsistent discriminant");
        if let Some(counter) = self.counter {
            counter.serialize_into(bytes);
        }
        if let Some(value_type) = self.value_type {
            u32::from(value_type).serialize_into(bytes);
        }
        if let Some(ref value) = self.value {
            value.serialize_into(bytes);
        }
        if let Some(test_type) = self.test_type {
            u32::from(test_type).serialize_into(bytes);
        }
        if let Some(ref delta) = self.delta {
            delta.serialize_into(bytes);
        }
        if let Some(events) = self.events {
            events.serialize_into(bytes);
        }
    }
}
impl ChangeAlarmAux {
    fn switch_expr(&self) -> u32 {
        let mut expr_value = 0;
        if self.counter.is_some() {
            expr_value |= u32::from(CA::COUNTER);
        }
        if self.value_type.is_some() {
            expr_value |= u32::from(CA::VALUE_TYPE);
        }
        if self.value.is_some() {
            expr_value |= u32::from(CA::VALUE);
        }
        if self.test_type.is_some() {
            expr_value |= u32::from(CA::TEST_TYPE);
        }
        if self.delta.is_some() {
            expr_value |= u32::from(CA::DELTA);
        }
        if self.events.is_some() {
            expr_value |= u32::from(CA::EVENTS);
        }
        expr_value
    }
}
impl ChangeAlarmAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the `counter` field of this structure.
    pub fn counter<I>(mut self, value: I) -> Self where I: Into<Option<Counter>> {
        self.counter = value.into();
        self
    }
    /// Set the `value_type` field of this structure.
    pub fn value_type<I>(mut self, value: I) -> Self where I: Into<Option<VALUETYPE>> {
        self.value_type = value.into();
        self
    }
    /// Set the `value` field of this structure.
    pub fn value<I>(mut self, value: I) -> Self where I: Into<Option<Int64>> {
        self.value = value.into();
        self
    }
    /// Set the `test_type` field of this structure.
    pub fn test_type<I>(mut self, value: I) -> Self where I: Into<Option<TESTTYPE>> {
        self.test_type = value.into();
        self
    }
    /// Set the `delta` field of this structure.
    pub fn delta<I>(mut self, value: I) -> Self where I: Into<Option<Int64>> {
        self.delta = value.into();
        self
    }
    /// Set the `events` field of this structure.
    pub fn events<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.events = value.into();
        self
    }
}

/// Opcode for the ChangeAlarm request
pub const CHANGE_ALARM_REQUEST: u8 = 9;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeAlarmRequest<'input> {
    pub id: Alarm,
    pub value_list: Cow<'input, ChangeAlarmAux>,
}
impl<'input> ChangeAlarmRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let id_bytes = self.id.serialize();
        let value_mask = self.value_list.switch_expr();
        let value_mask_bytes = value_mask.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CHANGE_ALARM_REQUEST,
            0,
            0,
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            value_mask_bytes[0],
            value_mask_bytes[1],
            value_mask_bytes[2],
            value_mask_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let value_list_bytes = self.value_list.serialize(value_mask);
        let length_so_far = length_so_far + value_list_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), value_list_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CHANGE_ALARM_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (id, remaining) = Alarm::try_parse(value)?;
        let (value_mask, remaining) = u32::try_parse(remaining)?;
        let (value_list, remaining) = ChangeAlarmAux::try_parse(remaining, value_mask)?;
        let _ = remaining;
        Ok(ChangeAlarmRequest {
            id,
            value_list: Cow::Owned(value_list),
        })
    }
    /// Clone all borrowed data in this ChangeAlarmRequest.
    pub fn into_owned(self) -> ChangeAlarmRequest<'static> {
        ChangeAlarmRequest {
            id: self.id,
            value_list: Cow::Owned(self.value_list.into_owned()),
        }
    }
}
impl<'input> Request for ChangeAlarmRequest<'input> {
    type Reply = ();
}
pub fn change_alarm<'c, 'input, Conn>(conn: &'c Conn, id: Alarm, value_list: &'input ChangeAlarmAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeAlarmRequest {
        id,
        value_list: Cow::Borrowed(value_list),
    };
    request0.send(conn)
}

/// Opcode for the DestroyAlarm request
pub const DESTROY_ALARM_REQUEST: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyAlarmRequest {
    pub alarm: Alarm,
}
impl DestroyAlarmRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let alarm_bytes = self.alarm.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_ALARM_REQUEST,
            0,
            0,
            alarm_bytes[0],
            alarm_bytes[1],
            alarm_bytes[2],
            alarm_bytes[3],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_ALARM_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (alarm, remaining) = Alarm::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyAlarmRequest {
            alarm,
        })
    }
}
impl Request for DestroyAlarmRequest {
    type Reply = ();
}
pub fn destroy_alarm<Conn>(conn: &Conn, alarm: Alarm) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyAlarmRequest {
        alarm,
    };
    request0.send(conn)
}

/// Opcode for the QueryAlarm request
pub const QUERY_ALARM_REQUEST: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryAlarmRequest {
    pub alarm: Alarm,
}
impl QueryAlarmRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let alarm_bytes = self.alarm.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_ALARM_REQUEST,
            0,
            0,
            alarm_bytes[0],
            alarm_bytes[1],
            alarm_bytes[2],
            alarm_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryAlarmReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_ALARM_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (alarm, remaining) = Alarm::try_parse(value)?;
        let _ = remaining;
        Ok(QueryAlarmRequest {
            alarm,
        })
    }
}
impl Request for QueryAlarmRequest {
    type Reply = QueryAlarmReply;
}
pub fn query_alarm<Conn>(conn: &Conn, alarm: Alarm) -> Result<Cookie<'_, Conn, QueryAlarmReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryAlarmRequest {
        alarm,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryAlarmReply {
    pub sequence: u16,
    pub length: u32,
    pub trigger: Trigger,
    pub delta: Int64,
    pub events: bool,
    pub state: ALARMSTATE,
}
impl TryParse for QueryAlarmReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (trigger, remaining) = Trigger::try_parse(remaining)?;
        let (delta, remaining) = Int64::try_parse(remaining)?;
        let (events, remaining) = bool::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let state = state.into();
        let result = QueryAlarmReply { sequence, length, trigger, delta, events, state };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the SetPriority request
pub const SET_PRIORITY_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPriorityRequest {
    pub id: u32,
    pub priority: i32,
}
impl SetPriorityRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let id_bytes = self.id.serialize();
        let priority_bytes = self.priority.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            SET_PRIORITY_REQUEST,
            0,
            0,
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            priority_bytes[0],
            priority_bytes[1],
            priority_bytes[2],
            priority_bytes[3],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != SET_PRIORITY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (id, remaining) = u32::try_parse(value)?;
        let (priority, remaining) = i32::try_parse(remaining)?;
        let _ = remaining;
        Ok(SetPriorityRequest {
            id,
            priority,
        })
    }
}
impl Request for SetPriorityRequest {
    type Reply = ();
}
pub fn set_priority<Conn>(conn: &Conn, id: u32, priority: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPriorityRequest {
        id,
        priority,
    };
    request0.send(conn)
}

/// Opcode for the GetPriority request
pub const GET_PRIORITY_REQUEST: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPriorityRequest {
    pub id: u32,
}
impl GetPriorityRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let id_bytes = self.id.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            GET_PRIORITY_REQUEST,
            0,
            0,
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetPriorityReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != GET_PRIORITY_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (id, remaining) = u32::try_parse(value)?;
        let _ = remaining;
        Ok(GetPriorityRequest {
            id,
        })
    }
}
impl Request for GetPriorityRequest {
    type Reply = GetPriorityReply;
}
pub fn get_priority<Conn>(conn: &Conn, id: u32) -> Result<Cookie<'_, Conn, GetPriorityReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPriorityRequest {
        id,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPriorityReply {
    pub sequence: u16,
    pub length: u32,
    pub priority: i32,
}
impl TryParse for GetPriorityReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (priority, remaining) = i32::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = GetPriorityReply { sequence, length, priority };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the CreateFence request
pub const CREATE_FENCE_REQUEST: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateFenceRequest {
    pub drawable: xproto::Drawable,
    pub fence: Fence,
    pub initially_triggered: bool,
}
impl CreateFenceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let fence_bytes = self.fence.serialize();
        let initially_triggered_bytes = self.initially_triggered.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            CREATE_FENCE_REQUEST,
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            fence_bytes[0],
            fence_bytes[1],
            fence_bytes[2],
            fence_bytes[3],
            initially_triggered_bytes[0],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != CREATE_FENCE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (drawable, remaining) = xproto::Drawable::try_parse(value)?;
        let (fence, remaining) = Fence::try_parse(remaining)?;
        let (initially_triggered, remaining) = bool::try_parse(remaining)?;
        let _ = remaining;
        Ok(CreateFenceRequest {
            drawable,
            fence,
            initially_triggered,
        })
    }
}
impl Request for CreateFenceRequest {
    type Reply = ();
}
pub fn create_fence<Conn>(conn: &Conn, drawable: xproto::Drawable, fence: Fence, initially_triggered: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateFenceRequest {
        drawable,
        fence,
        initially_triggered,
    };
    request0.send(conn)
}

/// Opcode for the TriggerFence request
pub const TRIGGER_FENCE_REQUEST: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggerFenceRequest {
    pub fence: Fence,
}
impl TriggerFenceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let fence_bytes = self.fence.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            TRIGGER_FENCE_REQUEST,
            0,
            0,
            fence_bytes[0],
            fence_bytes[1],
            fence_bytes[2],
            fence_bytes[3],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != TRIGGER_FENCE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (fence, remaining) = Fence::try_parse(value)?;
        let _ = remaining;
        Ok(TriggerFenceRequest {
            fence,
        })
    }
}
impl Request for TriggerFenceRequest {
    type Reply = ();
}
pub fn trigger_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TriggerFenceRequest {
        fence,
    };
    request0.send(conn)
}

/// Opcode for the ResetFence request
pub const RESET_FENCE_REQUEST: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetFenceRequest {
    pub fence: Fence,
}
impl ResetFenceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let fence_bytes = self.fence.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            RESET_FENCE_REQUEST,
            0,
            0,
            fence_bytes[0],
            fence_bytes[1],
            fence_bytes[2],
            fence_bytes[3],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != RESET_FENCE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (fence, remaining) = Fence::try_parse(value)?;
        let _ = remaining;
        Ok(ResetFenceRequest {
            fence,
        })
    }
}
impl Request for ResetFenceRequest {
    type Reply = ();
}
pub fn reset_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ResetFenceRequest {
        fence,
    };
    request0.send(conn)
}

/// Opcode for the DestroyFence request
pub const DESTROY_FENCE_REQUEST: u8 = 17;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyFenceRequest {
    pub fence: Fence,
}
impl DestroyFenceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let fence_bytes = self.fence.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            DESTROY_FENCE_REQUEST,
            0,
            0,
            fence_bytes[0],
            fence_bytes[1],
            fence_bytes[2],
            fence_bytes[3],
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
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != DESTROY_FENCE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (fence, remaining) = Fence::try_parse(value)?;
        let _ = remaining;
        Ok(DestroyFenceRequest {
            fence,
        })
    }
}
impl Request for DestroyFenceRequest {
    type Reply = ();
}
pub fn destroy_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyFenceRequest {
        fence,
    };
    request0.send(conn)
}

/// Opcode for the QueryFence request
pub const QUERY_FENCE_REQUEST: u8 = 18;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryFenceRequest {
    pub fence: Fence,
}
impl QueryFenceRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let fence_bytes = self.fence.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            QUERY_FENCE_REQUEST,
            0,
            0,
            fence_bytes[0],
            fence_bytes[1],
            fence_bytes[2],
            fence_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, QueryFenceReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_with_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != QUERY_FENCE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let (fence, remaining) = Fence::try_parse(value)?;
        let _ = remaining;
        Ok(QueryFenceRequest {
            fence,
        })
    }
}
impl Request for QueryFenceRequest {
    type Reply = QueryFenceReply;
}
pub fn query_fence<Conn>(conn: &Conn, fence: Fence) -> Result<Cookie<'_, Conn, QueryFenceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryFenceRequest {
        fence,
    };
    request0.send(conn)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryFenceReply {
    pub sequence: u16,
    pub length: u32,
    pub triggered: bool,
}
impl TryParse for QueryFenceReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (triggered, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::InsufficientData)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let result = QueryFenceReply { sequence, length, triggered };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}

/// Opcode for the AwaitFence request
pub const AWAIT_FENCE_REQUEST: u8 = 19;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwaitFenceRequest<'input> {
    pub fence_list: Cow<'input, [Fence]>,
}
impl<'input> AwaitFenceRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            AWAIT_FENCE_REQUEST,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let fence_list_bytes = self.fence_list.serialize();
        let length_so_far = length_so_far + fence_list_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), fence_list_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        conn.send_request_without_reply(&slices, fds)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.minor_opcode != AWAIT_FENCE_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let mut remaining = value;
        // Length is 'everything left in the input'
        let mut fence_list = Vec::new();
        while !remaining.is_empty() {
            let (v, new_remaining) = Fence::try_parse(value)?;
            remaining = new_remaining;
            fence_list.push(v);
        }
        let _ = remaining;
        Ok(AwaitFenceRequest {
            fence_list: Cow::Owned(fence_list),
        })
    }
    /// Clone all borrowed data in this AwaitFenceRequest.
    pub fn into_owned(self) -> AwaitFenceRequest<'static> {
        AwaitFenceRequest {
            fence_list: Cow::Owned(self.fence_list.into_owned()),
        }
    }
}
impl<'input> Request for AwaitFenceRequest<'input> {
    type Reply = ();
}
pub fn await_fence<'c, 'input, Conn>(conn: &'c Conn, fence_list: &'input [Fence]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AwaitFenceRequest {
        fence_list: Cow::Borrowed(fence_list),
    };
    request0.send(conn)
}

/// Opcode for the CounterNotify event
pub const COUNTER_NOTIFY_EVENT: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterNotifyEvent {
    pub response_type: u8,
    pub kind: u8,
    pub sequence: u16,
    pub counter: Counter,
    pub wait_value: Int64,
    pub counter_value: Int64,
    pub timestamp: xproto::Timestamp,
    pub count: u16,
    pub destroyed: bool,
}
impl TryParse for CounterNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (counter, remaining) = Counter::try_parse(remaining)?;
        let (wait_value, remaining) = Int64::try_parse(remaining)?;
        let (counter_value, remaining) = Int64::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (count, remaining) = u16::try_parse(remaining)?;
        let (destroyed, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let result = CounterNotifyEvent { response_type, kind, sequence, counter, wait_value, counter_value, timestamp, count, destroyed };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl From<&CounterNotifyEvent> for [u8; 32] {
    fn from(input: &CounterNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let kind_bytes = input.kind.serialize();
        let sequence_bytes = input.sequence.serialize();
        let counter_bytes = input.counter.serialize();
        let wait_value_bytes = input.wait_value.serialize();
        let counter_value_bytes = input.counter_value.serialize();
        let timestamp_bytes = input.timestamp.serialize();
        let count_bytes = input.count.serialize();
        let destroyed_bytes = input.destroyed.serialize();
        [
            response_type_bytes[0],
            kind_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            counter_bytes[0],
            counter_bytes[1],
            counter_bytes[2],
            counter_bytes[3],
            wait_value_bytes[0],
            wait_value_bytes[1],
            wait_value_bytes[2],
            wait_value_bytes[3],
            wait_value_bytes[4],
            wait_value_bytes[5],
            wait_value_bytes[6],
            wait_value_bytes[7],
            counter_value_bytes[0],
            counter_value_bytes[1],
            counter_value_bytes[2],
            counter_value_bytes[3],
            counter_value_bytes[4],
            counter_value_bytes[5],
            counter_value_bytes[6],
            counter_value_bytes[7],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            count_bytes[0],
            count_bytes[1],
            destroyed_bytes[0],
            0,
        ]
    }
}
impl From<CounterNotifyEvent> for [u8; 32] {
    fn from(input: CounterNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the AlarmNotify event
pub const ALARM_NOTIFY_EVENT: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlarmNotifyEvent {
    pub response_type: u8,
    pub kind: u8,
    pub sequence: u16,
    pub alarm: Alarm,
    pub counter_value: Int64,
    pub alarm_value: Int64,
    pub timestamp: xproto::Timestamp,
    pub state: ALARMSTATE,
}
impl TryParse for AlarmNotifyEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (alarm, remaining) = Alarm::try_parse(remaining)?;
        let (counter_value, remaining) = Int64::try_parse(remaining)?;
        let (alarm_value, remaining) = Int64::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::InsufficientData)?;
        let state = state.into();
        let result = AlarmNotifyEvent { response_type, kind, sequence, alarm, counter_value, alarm_value, timestamp, state };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl From<&AlarmNotifyEvent> for [u8; 32] {
    fn from(input: &AlarmNotifyEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let kind_bytes = input.kind.serialize();
        let sequence_bytes = input.sequence.serialize();
        let alarm_bytes = input.alarm.serialize();
        let counter_value_bytes = input.counter_value.serialize();
        let alarm_value_bytes = input.alarm_value.serialize();
        let timestamp_bytes = input.timestamp.serialize();
        let state_bytes = u8::from(input.state).serialize();
        [
            response_type_bytes[0],
            kind_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            alarm_bytes[0],
            alarm_bytes[1],
            alarm_bytes[2],
            alarm_bytes[3],
            counter_value_bytes[0],
            counter_value_bytes[1],
            counter_value_bytes[2],
            counter_value_bytes[3],
            counter_value_bytes[4],
            counter_value_bytes[5],
            counter_value_bytes[6],
            counter_value_bytes[7],
            alarm_value_bytes[0],
            alarm_value_bytes[1],
            alarm_value_bytes[2],
            alarm_value_bytes[3],
            alarm_value_bytes[4],
            alarm_value_bytes[5],
            alarm_value_bytes[6],
            alarm_value_bytes[7],
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
            state_bytes[0],
            0,
            0,
            0,
        ]
    }
}
impl From<AlarmNotifyEvent> for [u8; 32] {
    fn from(input: AlarmNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn sync_initialize(&self, desired_major_version: u8, desired_minor_version: u8) -> Result<Cookie<'_, Self, InitializeReply>, ConnectionError>
    {
        initialize(self, desired_major_version, desired_minor_version)
    }
    fn sync_list_system_counters(&self) -> Result<Cookie<'_, Self, ListSystemCountersReply>, ConnectionError>
    {
        list_system_counters(self)
    }
    fn sync_create_counter(&self, id: Counter, initial_value: Int64) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_counter(self, id, initial_value)
    }
    fn sync_destroy_counter(&self, counter: Counter) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_counter(self, counter)
    }
    fn sync_query_counter(&self, counter: Counter) -> Result<Cookie<'_, Self, QueryCounterReply>, ConnectionError>
    {
        query_counter(self, counter)
    }
    fn sync_await_<'c, 'input>(&'c self, wait_list: &'input [Waitcondition]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        await_(self, wait_list)
    }
    fn sync_change_counter(&self, counter: Counter, amount: Int64) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        change_counter(self, counter, amount)
    }
    fn sync_set_counter(&self, counter: Counter, value: Int64) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_counter(self, counter, value)
    }
    fn sync_create_alarm<'c, 'input>(&'c self, id: Alarm, value_list: &'input CreateAlarmAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_alarm(self, id, value_list)
    }
    fn sync_change_alarm<'c, 'input>(&'c self, id: Alarm, value_list: &'input ChangeAlarmAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_alarm(self, id, value_list)
    }
    fn sync_destroy_alarm(&self, alarm: Alarm) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_alarm(self, alarm)
    }
    fn sync_query_alarm(&self, alarm: Alarm) -> Result<Cookie<'_, Self, QueryAlarmReply>, ConnectionError>
    {
        query_alarm(self, alarm)
    }
    fn sync_set_priority(&self, id: u32, priority: i32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_priority(self, id, priority)
    }
    fn sync_get_priority(&self, id: u32) -> Result<Cookie<'_, Self, GetPriorityReply>, ConnectionError>
    {
        get_priority(self, id)
    }
    fn sync_create_fence(&self, drawable: xproto::Drawable, fence: Fence, initially_triggered: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_fence(self, drawable, fence, initially_triggered)
    }
    fn sync_trigger_fence(&self, fence: Fence) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        trigger_fence(self, fence)
    }
    fn sync_reset_fence(&self, fence: Fence) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        reset_fence(self, fence)
    }
    fn sync_destroy_fence(&self, fence: Fence) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_fence(self, fence)
    }
    fn sync_query_fence(&self, fence: Fence) -> Result<Cookie<'_, Self, QueryFenceReply>, ConnectionError>
    {
        query_fence(self, fence)
    }
    fn sync_await_fence<'c, 'input>(&'c self, fence_list: &'input [Fence]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        await_fence(self, fence_list)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
